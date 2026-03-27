use crate::adapters::traits::MetricAdapter;
use crate::error::AppResult;
use crate::models::metric::{MetricType, RawMetric};
use crate::models::server::ServerConfig;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct NodeExporterAdapter {
    client: reqwest::Client,
}

impl NodeExporterAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .expect("failed to build HTTP client"),
        }
    }
}

#[async_trait]
impl MetricAdapter for NodeExporterAdapter {
    fn name(&self) -> &str {
        "node_exporter"
    }

    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>> {
        let url = format!("http://{}:{}/metrics", server.host, server.port);
        let resp = self.client.get(&url).send().await?;
        let body = resp.text().await?;
        Ok(parse_prometheus_text(&body))
    }

    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool> {
        let url = format!("http://{}:{}/metrics", server.host, server.port);
        match self.client.get(&url).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

fn parse_prometheus_text(body: &str) -> Vec<RawMetric> {
    let mut metrics = Vec::new();
    let now = chrono::Utc::now().timestamp();

    for line in body.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        // Split by last space to separate value/timestamp
        if let Some((metric_part, value_part)) = line.rsplit_once(' ') {
            // Parse value (may have timestamp after it)
            let value = if let Some((val_str, _timestamp)) = value_part.split_once(' ') {
                val_str.parse::<f64>()
            } else {
                value_part.parse::<f64>()
            };

            if let Ok(value) = value {
                let (metric_name, labels) = parse_metric_name_and_labels(metric_part);

                let metric_type = if metric_name.ends_with("_total")
                    || metric_name.ends_with("_bytes_total")
                    || metric_name.ends_with("_count")
                {
                    MetricType::Counter
                } else {
                    MetricType::Gauge
                };

                metrics.push(RawMetric {
                    key: metric_name,
                    value,
                    metric_type,
                    timestamp: now,
                    labels,
                });
            }
        }
    }
    metrics
}

fn parse_metric_name_and_labels(metric_part: &str) -> (String, HashMap<String, String>) {
    if let Some(brace_start) = metric_part.find('{') {
        let metric_name = metric_part[..brace_start].to_string();
        let labels_str = &metric_part[brace_start + 1..];

        if let Some(brace_end) = labels_str.rfind('}') {
            let labels_content = &labels_str[..brace_end];
            let labels = parse_labels(labels_content);
            (metric_name, labels)
        } else {
            (metric_part.to_string(), HashMap::new())
        }
    } else {
        (metric_part.to_string(), HashMap::new())
    }
}

fn parse_labels(labels_str: &str) -> HashMap<String, String> {
    let mut labels = HashMap::new();
    let mut chars = labels_str.chars().peekable();

    while chars.peek().is_some() {
        // Skip whitespace
        while chars.peek() == Some(&' ') || chars.peek() == Some(&',') {
            chars.next();
        }

        // Parse key
        let mut key = String::new();
        while let Some(&ch) = chars.peek() {
            if ch == '=' {
                break;
            }
            key.push(chars.next().unwrap());
        }

        if chars.next() != Some('=') {
            continue;
        }

        // Parse value (handle quoted strings)
        let mut value = String::new();
        if chars.peek() == Some(&'"') {
            chars.next(); // consume opening quote
            while let Some(ch) = chars.next() {
                if ch == '"' {
                    break;
                } else if ch == '\\' {
                    if let Some(escaped) = chars.next() {
                        value.push(escaped);
                    }
                } else {
                    value.push(ch);
                }
            }
        } else {
            while let Some(&ch) = chars.peek() {
                if ch == ',' || ch == ' ' {
                    break;
                }
                value.push(chars.next().unwrap());
            }
        }

        if !key.trim().is_empty() {
            labels.insert(key.trim().to_string(), value);
        }
    }

    labels
}

#[cfg(test)]
mod tests {
    use super::parse_prometheus_text;

    #[test]
    fn parses_prometheus_labels_into_structured_map() {
        let body = r#"node_network_receive_bytes_total{device="eth0",instance="srv-1"} 42
"#;

        let metrics = parse_prometheus_text(body);
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].key, "node_network_receive_bytes_total");
        assert_eq!(
            metrics[0].labels.get("device").map(String::as_str),
            Some("eth0")
        );
        assert_eq!(
            metrics[0].labels.get("instance").map(String::as_str),
            Some("srv-1")
        );
    }
}
