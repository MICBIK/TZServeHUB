use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMetric {
    pub key: String,
    pub value: f64,
    pub metric_type: MetricType,
    pub timestamp: i64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetricType {
    Counter,
    Gauge,
    State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPoint {
    pub server_id: String,
    pub key: String,
    pub value: f64,
    pub metric_type: MetricType,
    pub vantage_point: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetric {
    pub server_id: String,
    pub key: String,
    pub min_val: f64,
    pub max_val: f64,
    pub avg_val: f64,
    pub bucket: i64,
}
