use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub server_id: Option<String>,
    pub metric_key: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub duration_sec: u32,
    pub cooldown_sec: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertCondition {
    Gt,
    Lt,
    Eq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: i64,
    pub rule_id: String,
    pub server_id: String,
    pub metric_key: String,
    pub value: f64,
    pub status: AlertStatus,
    pub fired_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Firing,
    Resolved,
}
