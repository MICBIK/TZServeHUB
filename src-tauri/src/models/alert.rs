use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub server_id: String,
    pub name: String,
    pub metric_key: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub duration_sec: u32,
    pub enabled: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertCondition {
    Gt,
    Lt,
    Eq,
}

impl AlertCondition {
    pub fn from_db(value: &str) -> Self {
        match value {
            "gt" => AlertCondition::Gt,
            "lt" => AlertCondition::Lt,
            "eq" => AlertCondition::Eq,
            _ => AlertCondition::Gt,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AlertCondition::Gt => "gt",
            AlertCondition::Lt => "lt",
            AlertCondition::Eq => "eq",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub id: String,
    pub rule_id: String,
    pub server_id: String,
    pub status: AlertStatus,
    pub message: String,
    pub fired_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertStatus {
    Firing,
    Resolved,
}

impl AlertStatus {
    pub fn from_db(value: &str) -> Self {
        match value {
            "firing" => AlertStatus::Firing,
            "resolved" => AlertStatus::Resolved,
            _ => AlertStatus::Firing,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AlertStatus::Firing => "firing",
            AlertStatus::Resolved => "resolved",
        }
    }
}
