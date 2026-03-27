use crate::alerts::rules::AlertEngine;
use crate::error::{AppError, AppResult};
use crate::models::alert::{AlertCondition, AlertEvent, AlertRule, AlertStatus};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

fn row_to_alert_rule(row: &sqlx::sqlite::SqliteRow) -> AlertRule {
    AlertRule {
        id: row.get("id"),
        server_id: row.get("server_id"),
        name: row.get("name"),
        metric_key: row.get("metric_key"),
        condition: AlertCondition::from_db(&row.get::<String, _>("condition")),
        threshold: row.get("threshold"),
        duration_sec: row.get::<i64, _>("duration_sec") as u32,
        enabled: row.get::<i64, _>("enabled") != 0,
        created_at: row.get("created_at"),
    }
}

#[tauri::command]
pub async fn list_alert_rules(pool: State<'_, SqlitePool>) -> Result<Vec<AlertRule>, String> {
    let result: AppResult<Vec<AlertRule>> = async {
        let rows = sqlx::query(
            "SELECT id, server_id, name, metric_key, condition, threshold, duration_sec, enabled, created_at FROM alert_rules ORDER BY created_at DESC",
        )
        .fetch_all(pool.inner())
        .await?;

        Ok(rows.iter().map(row_to_alert_rule).collect())
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_alert_rule(
    pool: State<'_, SqlitePool>,
    engine: State<'_, Arc<Mutex<AlertEngine>>>,
    server_id: String,
    name: String,
    metric_key: String,
    condition: AlertCondition,
    threshold: f64,
    duration_sec: u32,
) -> Result<AlertRule, String> {
    let result: AppResult<AlertRule> = async {
        if name.trim().is_empty() {
            return Err(AppError::Custom("Rule name cannot be empty".to_string()));
        }
        if metric_key.trim().is_empty() {
            return Err(AppError::Custom(
                "Metric key cannot be empty".to_string(),
            ));
        }

        let rule = AlertRule {
            id: uuid::Uuid::new_v4().to_string(),
            server_id,
            name: name.trim().to_string(),
            metric_key: metric_key.trim().to_string(),
            condition,
            threshold,
            duration_sec,
            enabled: true,
            created_at: chrono::Utc::now().timestamp(),
        };

        sqlx::query(
            "INSERT INTO alert_rules (id, server_id, name, metric_key, condition, threshold, duration_sec, enabled, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&rule.id)
        .bind(&rule.server_id)
        .bind(&rule.name)
        .bind(&rule.metric_key)
        .bind(rule.condition.as_str())
        .bind(rule.threshold)
        .bind(rule.duration_sec as i64)
        .bind(1_i64)
        .bind(rule.created_at)
        .execute(pool.inner())
        .await?;

        engine.lock().await.add_rule(rule.clone());

        Ok(rule)
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_alert_rule(
    pool: State<'_, SqlitePool>,
    engine: State<'_, Arc<Mutex<AlertEngine>>>,
    id: String,
) -> Result<(), String> {
    let result: AppResult<()> = async {
        if id.trim().is_empty() {
            return Err(AppError::Custom("Rule ID cannot be empty".to_string()));
        }

        sqlx::query("DELETE FROM alert_rules WHERE id = ?")
            .bind(&id)
            .execute(pool.inner())
            .await?;

        engine.lock().await.remove_rule(&id);

        Ok(())
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_alert_events(
    pool: State<'_, SqlitePool>,
    server_id: Option<String>,
    limit: Option<i64>,
) -> Result<Vec<AlertEvent>, String> {
    let result: AppResult<Vec<AlertEvent>> = async {
        let limit = limit.unwrap_or(100);

        let rows = if let Some(sid) = server_id {
            sqlx::query(
                "SELECT id, rule_id, server_id, status, message, fired_at, resolved_at FROM alert_events WHERE server_id = ? ORDER BY fired_at DESC LIMIT ?",
            )
            .bind(sid)
            .bind(limit)
            .fetch_all(pool.inner())
            .await?
        } else {
            sqlx::query(
                "SELECT id, rule_id, server_id, status, message, fired_at, resolved_at FROM alert_events ORDER BY fired_at DESC LIMIT ?",
            )
            .bind(limit)
            .fetch_all(pool.inner())
            .await?
        };

        Ok(rows
            .iter()
            .map(|row| AlertEvent {
                id: row.get("id"),
                rule_id: row.get("rule_id"),
                server_id: row.get("server_id"),
                status: AlertStatus::from_db(&row.get::<String, _>("status")),
                message: row.get("message"),
                fired_at: row.get("fired_at"),
                resolved_at: row.get("resolved_at"),
            })
            .collect())
    }
    .await;

    result.map_err(|e| e.to_string())
}

/// Load all enabled alert rules from DB into the engine at startup.
pub async fn load_rules_into_engine(
    pool: &SqlitePool,
    engine: &Arc<Mutex<AlertEngine>>,
) -> AppResult<()> {
    let rows = sqlx::query(
        "SELECT id, server_id, name, metric_key, condition, threshold, duration_sec, enabled, created_at FROM alert_rules WHERE enabled = 1",
    )
    .fetch_all(pool)
    .await?;

    let mut eng = engine.lock().await;
    for row in &rows {
        eng.add_rule(row_to_alert_rule(row));
    }
    log::info!("Loaded {} alert rules into engine", rows.len());
    Ok(())
}

/// Persist an alert event to the database.
pub async fn persist_alert_event(pool: &SqlitePool, event: &AlertEvent) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO alert_events (id, rule_id, server_id, status, message, fired_at, resolved_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&event.id)
    .bind(&event.rule_id)
    .bind(&event.server_id)
    .bind(event.status.as_str())
    .bind(&event.message)
    .bind(event.fired_at)
    .bind(event.resolved_at)
    .execute(pool)
    .await?;
    Ok(())
}
