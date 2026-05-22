use crate::models::alert::{AlertCondition, AlertEvent, AlertRule, AlertStatus};
use chrono::Utc;
use std::collections::HashMap;

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
                let state = self
                    .firing_state
                    .entry(rule.id.clone())
                    .or_insert(FiringState {
                        first_breach_at: now,
                        last_check_at: now,
                    });
                state.last_check_at = now;

                if now - state.first_breach_at >= rule.duration_sec as i64 {
                    events.push(AlertEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        rule_id: rule.id.clone(),
                        server_id: rule.server_id.clone(),
                        status: AlertStatus::Firing,
                        message: format!("{metric_key} breached threshold with value {value}"),
                        fired_at: now,
                        resolved_at: None,
                        delivery_status: None,
                    });
                }
            } else if self.firing_state.remove(&rule.id).is_some() {
                events.push(AlertEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.id.clone(),
                    server_id: rule.server_id.clone(),
                    status: AlertStatus::Resolved,
                    message: format!("{metric_key} recovered with value {value}"),
                    fired_at: now,
                    resolved_at: Some(now),
                    delivery_status: None,
                });
            }
        }

        events
    }
}
