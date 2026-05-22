use crate::adapters::{
    go_agent::GoAgentAdapter, node_exporter::NodeExporterAdapter, MetricAdapter,
};
use crate::alerts::notifier::AlertNotifier;
use crate::alerts::rules::AlertEngine;
use crate::commands::alerts::persist_alert_event;
use crate::error::AppResult;
use crate::metrics::{derived::DerivedMetricsEngine, rollup::RollupEngine};
use crate::models::{
    alert::AlertStatus,
    metric::RawMetric,
    server::{AccessMethod, AdapterType, AuthType, ServerConfig},
};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};
use tauri::{AppHandle, Runtime};
use tokio::sync::Mutex;
use tokio::time::{interval, sleep, Duration};

const DESKTOP_VANTAGE_POINT: &str = "desktop";

pub async fn start<R: Runtime>(
    pool: SqlitePool,
    alert_engine: Arc<Mutex<AlertEngine>>,
    app_handle: AppHandle<R>,
) -> AppResult<()> {
    RollupEngine::start(pool.clone());
    tokio::spawn(async move {
        let notifier = AlertNotifier::new(app_handle);
        let mut polling_tasks: HashMap<String, tokio::task::JoinHandle<()>> = HashMap::new();
        loop {
            match get_enabled_servers(&pool).await {
                Ok(servers) => reconcile_polling_tasks(
                    &pool,
                    servers,
                    &mut polling_tasks,
                    &alert_engine,
                    &notifier,
                ),
                Err(e) => log::error!("Failed to query servers: {e}"),
            }
            sleep(Duration::from_secs(10)).await;
        }
    });
    Ok(())
}

fn reconcile_polling_tasks(
    pool: &SqlitePool,
    servers: Vec<ServerConfig>,
    tasks: &mut HashMap<String, tokio::task::JoinHandle<()>>,
    alert_engine: &Arc<Mutex<AlertEngine>>,
    notifier: &AlertNotifier,
) {
    let current_ids: HashSet<String> = servers.iter().map(|server| server.id.clone()).collect();
    tasks.retain(|server_id, handle| {
        if current_ids.contains(server_id) {
            true
        } else {
            handle.abort();
            false
        }
    });
    for server in servers {
        if !tasks.contains_key(&server.id) {
            let pool_clone = pool.clone();
            let engine_clone = alert_engine.clone();
            let notifier_clone = notifier.clone();
            let server_id = server.id.clone();
            tasks.insert(
                server_id,
                tokio::spawn(async move {
                    poll_server_loop(pool_clone, server, engine_clone, notifier_clone).await;
                }),
            );
        }
    }
}

fn row_to_server(row: &sqlx::sqlite::SqliteRow) -> ServerConfig {
    ServerConfig {
        id: row.get("id"),
        name: row.get("name"),
        host: row.get("host"),
        port: row.get::<i64, _>("port") as u16,
        adapter_type: AdapterType::from_db(&row.get::<String, _>("adapter_type")),
        access_method: AccessMethod::from_db(&row.get::<String, _>("access_method")),
        polling_interval_sec: row.get::<i64, _>("polling_interval_sec") as u32,
        enabled: row.get::<i64, _>("enabled") != 0,
        auth_token: row.get("auth_token"),
        auth_type: AuthType::from_db(&row.get::<String, _>("auth_type")),
        ssh_key_path: row.get("ssh_key_path"),
        ssh_passphrase: row.get("ssh_passphrase"),
        password: row.get("password"),
        status: row.get("status"),
        last_seen_at: row.get("last_seen_at"),
        last_error: row.get("last_error"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

async fn get_enabled_servers(pool: &SqlitePool) -> AppResult<Vec<ServerConfig>> {
    let rows = sqlx::query(
        "SELECT id, name, host, port, adapter_type, access_method, polling_interval_sec, enabled, auth_token, auth_type, ssh_key_path, ssh_passphrase, password, status, last_seen_at, last_error, created_at, updated_at FROM servers WHERE enabled = 1",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(row_to_server).collect())
}

async fn poll_server_loop(
    pool: SqlitePool,
    server: ServerConfig,
    alert_engine: Arc<Mutex<AlertEngine>>,
    notifier: AlertNotifier,
) {
    let adapter: Arc<dyn MetricAdapter> = match server.adapter_type {
        AdapterType::GoAgent => Arc::new(GoAgentAdapter::new()),
        AdapterType::NodeExporter => Arc::new(NodeExporterAdapter::new()),
        AdapterType::Glances => {
            return log::warn!("Glances adapter not implemented yet for {}", server.name)
        }
    };
    let mut ticker = interval(Duration::from_secs(server.polling_interval_sec as u64));
    let mut consecutive_failures = 0_u32;
    let mut derived_engine = DerivedMetricsEngine::new();
    loop {
        ticker.tick().await;
        match adapter.fetch_host_metrics(&server).await {
            Ok(metrics) => {
                consecutive_failures = 0;
                // Update server status to online
                let now = chrono::Utc::now().timestamp();
                if let Err(e) = sqlx::query(
                    "UPDATE servers SET status = 'online', last_seen_at = ?, last_error = NULL WHERE id = ?",
                )
                .bind(now)
                .bind(&server.id)
                .execute(&pool)
                .await
                {
                    log::error!("Failed to update server status for {}: {e}", server.name);
                }
                if let Err(e) = store_metrics(
                    &pool,
                    &server.id,
                    metrics,
                    &mut derived_engine,
                    &alert_engine,
                    &notifier,
                )
                .await
                {
                    log::error!("Failed to store metrics for {}: {e}", server.name);
                }
            }
            Err(e) => {
                consecutive_failures += 1;
                let error_msg = format!("{e}");
                log::error!(
                    "Failed to fetch metrics from {} (attempt {}): {e}",
                    server.name,
                    consecutive_failures
                );
                // Update server status to error
                if let Err(update_err) = sqlx::query(
                    "UPDATE servers SET status = 'error', last_error = ? WHERE id = ?",
                )
                .bind(&error_msg)
                .bind(&server.id)
                .execute(&pool)
                .await
                {
                    log::error!("Failed to update server error status for {}: {update_err}", server.name);
                }
                if consecutive_failures > 1 {
                    sleep(Duration::from_secs(std::cmp::min(
                        300,
                        2_u64.pow(consecutive_failures - 1),
                    )))
                    .await;
                }
            }
        }
    }
}

fn labels_json(metric: &RawMetric) -> AppResult<String> {
    let ordered: BTreeMap<String, String> = metric.labels.clone().into_iter().collect();
    Ok(serde_json::to_string(&ordered)?)
}

async fn store_metrics(
    pool: &SqlitePool,
    server_id: &str,
    metrics: Vec<RawMetric>,
    derived_engine: &mut DerivedMetricsEngine,
    alert_engine: &Arc<Mutex<AlertEngine>>,
    notifier: &AlertNotifier,
) -> AppResult<()> {
    let mut series = HashSet::new();
    let mut time_range = (i64::MAX, i64::MIN);
    let mut metric_values: Vec<(String, f64)> = Vec::new();

    for metric in derived_engine.process_metrics(metrics) {
        time_range.0 = time_range.0.min(metric.timestamp);
        time_range.1 = time_range.1.max(metric.timestamp);
        let labels = labels_json(&metric)?;
        series.insert((metric.key.clone(), labels.clone()));
        metric_values.push((metric.key.clone(), metric.value));
        sqlx::query("INSERT INTO raw_metrics (server_id, key, value, metric_type, vantage_point, timestamp, labels) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(server_id)
            .bind(&metric.key)
            .bind(metric.value)
            .bind(format!("{:?}", metric.metric_type).to_lowercase())
            .bind(DESKTOP_VANTAGE_POINT)
            .bind(metric.timestamp)
            .bind(labels)
            .execute(pool)
            .await?;
    }

    // Evaluate alert rules against incoming metrics
    {
        let mut engine = alert_engine.lock().await;
        for (key, value) in &metric_values {
            let events = engine.evaluate(key, *value);
            for event in &events {
                let delivery_report = match event.status {
                    AlertStatus::Firing => notifier.send_alert(event).await,
                    AlertStatus::Resolved => notifier.send_recovery(event).await,
                };

                let mut event_to_persist = event.clone();
                match delivery_report.to_delivery_status_json() {
                    Ok(status_json) => event_to_persist.delivery_status = Some(status_json),
                    Err(e) => log::error!("Failed to encode alert delivery status: {e}"),
                }

                if let Err(e) = persist_alert_event(pool, &event_to_persist).await {
                    log::error!("Failed to persist alert event: {e}");
                }
            }
        }
    }

    if time_range.0 == i64::MAX || time_range.1 == i64::MIN {
        return Ok(());
    }

    let rollup_engine = RollupEngine::new(pool.clone());
    for (key, labels) in series {
        rollup_engine
            .generate_1m_rollup(
                server_id,
                &key,
                &labels,
                DESKTOP_VANTAGE_POINT,
                time_range.0,
                time_range.1,
            )
            .await?;
        rollup_engine
            .generate_15m_rollup(
                server_id,
                &key,
                &labels,
                DESKTOP_VANTAGE_POINT,
                time_range.0,
                time_range.1,
            )
            .await?;
    }

    Ok(())
}
