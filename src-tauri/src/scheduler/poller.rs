use crate::adapters::{
    go_agent::GoAgentAdapter, node_exporter::NodeExporterAdapter, MetricAdapter,
};
use crate::error::AppResult;
use crate::metrics::{derived::DerivedMetricsEngine, rollup::RollupEngine};
use crate::models::{
    metric::RawMetric,
    server::{AccessMethod, AdapterType, AuthType, ServerConfig},
};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Arc,
};
use tokio::time::{interval, sleep, Duration};

const DESKTOP_VANTAGE_POINT: &str = "desktop";

pub async fn start(pool: SqlitePool) -> AppResult<()> {
    RollupEngine::start(pool.clone());
    tokio::spawn(async move {
        let mut polling_tasks: HashMap<String, tokio::task::JoinHandle<()>> = HashMap::new();
        loop {
            match get_enabled_servers(&pool).await {
                Ok(servers) => reconcile_polling_tasks(&pool, servers, &mut polling_tasks),
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
            let server_id = server.id.clone();
            tasks.insert(
                server_id,
                tokio::spawn(async move {
                    poll_server_loop(pool_clone, server).await;
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
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

async fn get_enabled_servers(pool: &SqlitePool) -> AppResult<Vec<ServerConfig>> {
    let rows = sqlx::query(
        "SELECT id, name, host, port, adapter_type, access_method, polling_interval_sec, enabled, auth_token, auth_type, ssh_key_path, ssh_passphrase, password, created_at, updated_at FROM servers WHERE enabled = 1",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(row_to_server).collect())
}

async fn poll_server_loop(pool: SqlitePool, server: ServerConfig) {
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
                if let Err(e) = store_metrics(&pool, &server.id, metrics, &mut derived_engine).await
                {
                    log::error!("Failed to store metrics for {}: {e}", server.name);
                }
            }
            Err(e) => {
                consecutive_failures += 1;
                log::error!(
                    "Failed to fetch metrics from {} (attempt {}): {e}",
                    server.name,
                    consecutive_failures
                );
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
) -> AppResult<()> {
    let mut series = HashSet::new();
    let mut time_range = (i64::MAX, i64::MIN);

    for metric in derived_engine.process_metrics(metrics) {
        time_range.0 = time_range.0.min(metric.timestamp);
        time_range.1 = time_range.1.max(metric.timestamp);
        let labels = labels_json(&metric)?;
        series.insert((metric.key.clone(), labels.clone()));
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
