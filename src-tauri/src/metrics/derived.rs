use crate::models::metric::{MetricType, RawMetric};
use std::collections::{BTreeMap, HashMap};

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

    pub fn process_metrics(&mut self, metrics: Vec<RawMetric>) -> Vec<RawMetric> {
        let mut result = Vec::new();
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Clean up stale entries (older than 5 minutes)
        let cutoff_timestamp = current_time - 300; // 5 minutes in seconds
        self.cleanup_stale(cutoff_timestamp);

        for metric in metrics {
            // Always include the original metric
            result.push(metric.clone());

            // Generate rate metric for counters
            if let Some(rate) = self.derive_rate(&metric) {
                let rate_metric = RawMetric {
                    key: format!("{}_rate", metric.key),
                    value: rate,
                    timestamp: metric.timestamp,
                    labels: metric.labels.clone(),
                    metric_type: MetricType::Gauge,
                };
                result.push(rate_metric);
            }
        }

        result
    }

    pub fn derive_rate(&mut self, metric: &RawMetric) -> Option<f64> {
        if metric.metric_type != MetricType::Counter {
            return None;
        }

        let labels: BTreeMap<String, String> = metric.labels.clone().into_iter().collect();
        let key = format!("{}:{:?}", metric.key, labels);

        if let Some(state) = self.counter_state.get(&key) {
            let time_delta = (metric.timestamp - state.last_timestamp) as f64;
            if time_delta <= 0.0 {
                return None;
            }

            let value_delta = metric.value - state.last_value;

            // Handle counter reset
            if value_delta < 0.0 {
                self.counter_state.insert(
                    key,
                    CounterState {
                        last_value: metric.value,
                        last_timestamp: metric.timestamp,
                    },
                );
                return None;
            }

            let rate = value_delta / time_delta;

            self.counter_state.insert(
                key,
                CounterState {
                    last_value: metric.value,
                    last_timestamp: metric.timestamp,
                },
            );

            Some(rate)
        } else {
            self.counter_state.insert(
                key,
                CounterState {
                    last_value: metric.value,
                    last_timestamp: metric.timestamp,
                },
            );
            None
        }
    }

    pub fn cleanup_stale(&mut self, cutoff_timestamp: i64) {
        self.counter_state
            .retain(|_, state| state.last_timestamp > cutoff_timestamp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn counter_metric(value: f64, timestamp: i64) -> RawMetric {
        RawMetric {
            key: "network_receive_bytes_total".to_string(),
            value,
            metric_type: MetricType::Counter,
            timestamp,
            labels: HashMap::from([("interface".to_string(), "eth0".to_string())]),
        }
    }

    fn counter_metric_for_interface(value: f64, timestamp: i64, interface: &str) -> RawMetric {
        RawMetric {
            key: "network_receive_bytes_total".to_string(),
            value,
            metric_type: MetricType::Counter,
            timestamp,
            labels: HashMap::from([("interface".to_string(), interface.to_string())]),
        }
    }

    fn extract_rate(metrics: &[RawMetric], interface: &str) -> Option<f64> {
        metrics
            .iter()
            .find(|metric| {
                metric.key == "network_receive_bytes_total_rate"
                    && metric.labels.get("interface").map(String::as_str) == Some(interface)
            })
            .map(|metric| metric.value)
    }

    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    #[test]
    fn reset_does_not_emit_negative_rate() {
        let mut engine = DerivedMetricsEngine::new();
        assert_eq!(engine.derive_rate(&counter_metric(100.0, 10)), None);
        assert_eq!(engine.derive_rate(&counter_metric(50.0, 20)), None);
    }

    #[test]
    fn process_metrics_emits_rate_across_consecutive_polls() {
        let mut engine = DerivedMetricsEngine::new();
        let timestamp = current_timestamp();

        let first = engine.process_metrics(vec![counter_metric(100.0, timestamp)]);
        let second = engine.process_metrics(vec![counter_metric(160.0, timestamp + 10)]);

        assert_eq!(extract_rate(&first, "eth0"), None);
        assert_eq!(extract_rate(&second, "eth0"), Some(6.0));
    }

    #[test]
    fn process_metrics_keeps_series_identity_separate() {
        let mut engine = DerivedMetricsEngine::new();
        let timestamp = current_timestamp();

        engine.process_metrics(vec![
            counter_metric_for_interface(100.0, timestamp, "eth0"),
            counter_metric_for_interface(200.0, timestamp, "eth1"),
        ]);

        let next = engine.process_metrics(vec![
            counter_metric_for_interface(140.0, timestamp + 10, "eth0"),
            counter_metric_for_interface(260.0, timestamp + 10, "eth1"),
        ]);

        assert_eq!(extract_rate(&next, "eth0"), Some(4.0));
        assert_eq!(extract_rate(&next, "eth1"), Some(6.0));
    }

    #[test]
    fn process_metrics_suppresses_rate_after_counter_reset() {
        let mut engine = DerivedMetricsEngine::new();
        let timestamp = current_timestamp();

        engine.process_metrics(vec![counter_metric(100.0, timestamp)]);
        let after_reset = engine.process_metrics(vec![counter_metric(20.0, timestamp + 10)]);
        let recovered = engine.process_metrics(vec![counter_metric(50.0, timestamp + 20)]);

        assert_eq!(extract_rate(&after_reset, "eth0"), None);
        assert_eq!(extract_rate(&recovered, "eth0"), Some(3.0));
    }
}
