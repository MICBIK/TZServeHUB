use async_trait::async_trait;
use crate::adapters::traits::MetricAdapter;
use crate::error::AppResult;
use crate::models::metric::{MetricType, RawMetric};
use crate::models::server::ServerConfig;
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
        if let Some((key, value_str)) = line.rsplit_once(' ') {
            if let Ok(value) = value_str.parse::<f64>() {
                let key = key.trim().to_string();
                let metric_type = if key.ends_with("_total") || key.ends_with("_bytes_total") {
                    MetricType::Counter
                } else {
                    MetricType::Gauge
                };
                metrics.push(RawMetric {
                    key,
                    value,
                    metric_type,
                    timestamp: now,
                    labels: HashMap::new(),
                });
            }
        }
    }
    metrics
}
