use std::collections::HashMap;
use crate::models::alert::{AlertRule, AlertCondition, AlertEvent, AlertStatus};
use chrono::Utc;

pub struct AlertEngine {
    rules: HashMap<String, AlertRule>,
    firing_state: HashMap<String, FiringState>,
}

struct FiringState {
    first_breach_at: i64,
    last_check_at: i64,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            firing_state: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.remove(rule_id);
        self.firing_state.remove(rule_id);
    }

    pub fn evaluate(&mut self, metric_key: &str, value: f64) -> Vec<AlertEvent> {
        let now = Utc::now().timestamp();
        let mut events = Vec::new();

        for rule in self.rules.values() {
            if !rule.enabled || rule.metric_key != metric_key {
                continue;
            }

            let breached = match rule.condition {
                AlertCondition::Gt => value > rule.threshold,
                AlertCondition::Lt => value < rule.threshold,
                AlertCondition::Eq => (value - rule.threshold).abs() < 0.001,
            };

            if breached {
                let state = self.firing_state.entry(rule.id.clone()).or_insert(FiringState {
                    first_breach_at: now,
                    last_check_at: now,
                });

                state.last_check_at = now;

                if now - state.first_breach_at >= rule.duration_sec as i64 {
                    events.push(AlertEvent {
                        id: uuid::Uuid::new_v4().to_string().parse().unwrap_or(0),
                        rule_id: rule.id.clone(),
                        server_id: rule.server_id.clone().unwrap_or_default(),
                        metric_key: metric_key.to_string(),
                        value,
                        status: AlertStatus::Firing,
                        fired_at: now,
                        resolved_at: None,
                    });
                }
            } else {
                if self.firing_state.remove(&rule.id).is_some() {
                    events.push(AlertEvent {
                        id: uuid::Uuid::new_v4().to_string().parse().unwrap_or(0),
                        rule_id: rule.id.clone(),
                        server_id: rule.server_id.clone().unwrap_or_default(),
                        metric_key: metric_key.to_string(),
                        value,
                        status: AlertStatus::Resolved,
                        fired_at: now,
                        resolved_at: Some(now),
                    });
                }
            }
        }

        events
    }
}
