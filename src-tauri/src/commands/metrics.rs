use crate::error::{AppError, AppResult};
use crate::models::metric::{AggregatedMetric, MetricHistoryResponse, MetricPoint, MetricType};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{BTreeMap, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::State;

const DESKTOP_VANTAGE_POINT: &str = "desktop";

fn parse_metric_type(value: &str) -> MetricType {
    match value {
        "counter" => MetricType::Counter,
        "state" => MetricType::State,
        _ => MetricType::Gauge,
    }
}

fn labels_json(labels: Option<HashMap<String, String>>) -> AppResult<String> {
    let ordered: BTreeMap<String, String> = labels.unwrap_or_default().into_iter().collect();
    Ok(serde_json::to_string(&ordered)?)
}

fn parse_labels(raw: Option<String>) -> AppResult<HashMap<String, String>> {
    Ok(match raw {
        Some(value) if !value.is_empty() => serde_json::from_str(&value)?,
        _ => HashMap::new(),
    })
}

fn pick_resolution(requested: Option<String>, from: i64) -> String {
    if let Some(value) = requested {
        return value;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    if from >= now - (7 * 86400) {
        "raw"
    } else if from >= now - (30 * 86400) {
        "1m"
    } else {
        "15m"
    }
    .to_string()
}

#[tauri::command]
pub async fn get_metrics(
    pool: State<'_, SqlitePool>,
    server_id: String,
) -> Result<Vec<MetricPoint>, String> {
    let result: AppResult<Vec<MetricPoint>> = async {
        if server_id.trim().is_empty() {
            return Err(AppError::Custom("Server ID cannot be empty".to_string()));
        }
        let rows = sqlx::query(
            "SELECT server_id, key, value, metric_type, vantage_point, labels, timestamp
             FROM raw_metrics WHERE server_id = ? AND timestamp = (
                SELECT MAX(timestamp) FROM raw_metrics WHERE server_id = ?
             ) ORDER BY key",
        )
        .bind(&server_id)
        .bind(&server_id)
        .fetch_all(pool.inner())
        .await?;
        rows.into_iter()
            .map(|row| {
                Ok(MetricPoint {
                    server_id: row.get("server_id"),
                    key: row.get("key"),
                    value: row.get("value"),
                    metric_type: parse_metric_type(&row.get::<String, _>("metric_type")),
                    vantage_point: row.get("vantage_point"),
                    labels: parse_labels(row.get("labels"))?,
                    timestamp: row.get("timestamp"),
                })
            })
            .collect()
    }
    .await;
    result.map_err(|e| {
        format!(
            "Failed to get metrics for server '{server_id}': {}",
            e.to_user_message()
        )
    })
}

#[tauri::command]
pub async fn get_metric_history(
    pool: State<'_, SqlitePool>,
    server_id: String,
    key: String,
    from: i64,
    to: i64,
    labels: Option<HashMap<String, String>>,
    resolution: Option<String>,
) -> Result<MetricHistoryResponse, String> {
    let result: AppResult<MetricHistoryResponse> = async {
        if server_id.trim().is_empty() || key.trim().is_empty() || from >= to {
            return Err(AppError::Custom("Invalid metric history query".to_string()));
        }
        let labels = labels_json(labels)?;
        let resolution = pick_resolution(resolution, from);
        match resolution.as_str() {
            "raw" => {
                let rows = sqlx::query(
                    "SELECT server_id, key, value, metric_type, vantage_point, labels, timestamp
                     FROM raw_metrics WHERE server_id = ? AND key = ? AND vantage_point = ? AND COALESCE(labels, '{}') = ? AND timestamp >= ? AND timestamp <= ? ORDER BY timestamp ASC",
                )
                .bind(&server_id).bind(&key).bind(DESKTOP_VANTAGE_POINT).bind(&labels).bind(from).bind(to)
                .fetch_all(pool.inner()).await?;
                let points = rows.into_iter().map(|row| Ok(MetricPoint {
                    server_id: row.get("server_id"),
                    key: row.get("key"),
                    value: row.get("value"),
                    metric_type: parse_metric_type(&row.get::<String, _>("metric_type")),
                    vantage_point: row.get("vantage_point"),
                    labels: parse_labels(row.get("labels"))?,
                    timestamp: row.get("timestamp"),
                })).collect::<AppResult<Vec<_>>>()?;
                Ok(MetricHistoryResponse::Raw { resolution, points })
            }
            "1m" | "15m" => {
                let table = if resolution == "1m" { "metrics_1m" } else { "metrics_15m" };
                let sql = format!(
                    "SELECT server_id, key, labels, vantage_point, min_val, max_val, avg_val, bucket
                     FROM {table} WHERE server_id = ? AND key = ? AND vantage_point = ? AND labels = ? AND bucket >= ? AND bucket <= ? ORDER BY bucket ASC",
                );
                let rows = sqlx::query(&sql)
                    .bind(&server_id).bind(&key).bind(DESKTOP_VANTAGE_POINT).bind(&labels).bind(from).bind(to)
                    .fetch_all(pool.inner()).await?;
                let buckets = rows.into_iter().map(|row| Ok(AggregatedMetric {
                    server_id: row.get("server_id"),
                    key: row.get("key"),
                    labels: parse_labels(Some(row.get("labels")))?,
                    vantage_point: row.get("vantage_point"),
                    resolution: resolution.clone(),
                    min_val: row.get("min_val"),
                    max_val: row.get("max_val"),
                    avg_val: row.get("avg_val"),
                    bucket: row.get("bucket"),
                })).collect::<AppResult<Vec<_>>>()?;
                Ok(MetricHistoryResponse::Rollup { resolution, buckets })
            }
            _ => Err(AppError::Custom("Invalid resolution".to_string())),
        }
    }.await;
    result.map_err(|e| {
        format!(
            "Failed to get metric history for '{key}' on server '{server_id}': {}",
            e.to_user_message()
        )
    })
}
