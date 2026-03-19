use std::collections::HashMap;
use crate::models::metric::{RawMetric, MetricType};

pub struct DerivedMetricsEngine {
    counter_state: HashMap<String, CounterState>,
}

struct CounterState {
    last_value: f64,
    last_timestamp: i64,
}

impl DerivedMetricsEngine {
    pub fn new() -> Self {
        Self {
            counter_state: HashMap::new(),
        }
    }

    pub fn derive_rate(&mut self, metric: &RawMetric) -> Option<f64> {
        if metric.metric_type != MetricType::Counter {
            return None;
        }

        let key = format!("{}:{:?}", metric.key, metric.labels);

        if let Some(state) = self.counter_state.get(&key) {
            let time_delta = (metric.timestamp - state.last_timestamp) as f64;
            if time_delta <= 0.0 {
                return None;
            }

            let value_delta = metric.value - state.last_value;

            // Handle counter reset
            let rate = if value_delta < 0.0 {
                metric.value / time_delta
            } else {
                value_delta / time_delta
            };

            self.counter_state.insert(key, CounterState {
                last_value: metric.value,
                last_timestamp: metric.timestamp,
            });

            Some(rate)
        } else {
            self.counter_state.insert(key, CounterState {
                last_value: metric.value,
                last_timestamp: metric.timestamp,
            });
            None
        }
    }

    pub fn cleanup_stale(&mut self, cutoff_timestamp: i64) {
        self.counter_state.retain(|_, state| {
            state.last_timestamp > cutoff_timestamp
        });
    }
}
