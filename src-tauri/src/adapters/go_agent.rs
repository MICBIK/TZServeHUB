use crate::adapters::traits::MetricAdapter;
use crate::error::AppResult;
use crate::models::metric::{MetricType, RawMetric};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct GoAgentResponse {
    timestamp: i64,
    cpu: CpuMetrics,
    memory: MemoryMetrics,
    disks: Vec<DiskMetrics>,
    disk_io: Vec<DiskIO>,
    network: Vec<NetMetrics>,
}

#[derive(Deserialize)]
struct CpuMetrics {
    total_percent: f64,
    per_core: Vec<f64>,
}

#[derive(Deserialize)]
struct MemoryMetrics {
    total: u64,
    used: u64,
    #[serde(default)]
    free: u64,
    #[serde(default)]
    cached: u64,
    available: u64,
    #[allow(dead_code)]
    used_percent: f64,
}

#[derive(Deserialize)]
struct DiskMetrics {
    mount: String,
    device: String,
    total: u64,
    used: u64,
    free: u64,
    #[allow(dead_code)]
    used_percent: f64,
}

#[derive(Deserialize)]
struct DiskIO {
    device: String,
    read_bytes: u64,
    write_bytes: u64,
}

#[derive(Deserialize)]
struct NetMetrics {
    interface: String,
    rx_bytes: u64,
    tx_bytes: u64,
}

pub struct GoAgentAdapter {
    client: Client,
}

impl GoAgentAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl MetricAdapter for GoAgentAdapter {
    fn name(&self) -> &str {
        "go_agent"
    }

    async fn fetch_host_metrics(
        &self,
        server: &crate::models::server::ServerConfig,
    ) -> AppResult<Vec<RawMetric>> {
        let url = format!("http://{}:{}/api/metrics", server.host, server.port);
        let mut request = self.client.get(&url);
        if let Some(token) = server
            .auth_token
            .as_deref()
            .filter(|token| !token.is_empty())
        {
            request = request.header("Authorization", format!("Bearer {token}"));
        }
        let response = request.send().await?;

        let data: GoAgentResponse = response.json().await?;
        let mut metrics = Vec::new();

        // CPU metrics
        metrics.push(RawMetric {
            key: "cpu_usage_percent".to_string(),
            value: data.cpu.total_percent,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });

        for (i, percent) in data.cpu.per_core.iter().enumerate() {
            let mut labels = HashMap::new();
            labels.insert("core".to_string(), i.to_string());
            metrics.push(RawMetric {
                key: "cpu_core_usage_percent".to_string(),
                value: *percent,
                metric_type: MetricType::Gauge,
                labels,
                timestamp: data.timestamp,
            });
        }

        // Memory metrics
        metrics.push(RawMetric {
            key: "memory_total_bytes".to_string(),
            value: data.memory.total as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });
        metrics.push(RawMetric {
            key: "memory_used_bytes".to_string(),
            value: data.memory.used as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });
        metrics.push(RawMetric {
            key: "memory_free_bytes".to_string(),
            value: data.memory.free as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });
        metrics.push(RawMetric {
            key: "memory_cached_bytes".to_string(),
            value: data.memory.cached as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });
        metrics.push(RawMetric {
            key: "memory_available_bytes".to_string(),
            value: data.memory.available as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });
        metrics.push(RawMetric {
            key: "memory_used_percent".to_string(),
            value: data.memory.used_percent,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });

        // Disk metrics
        for disk in data.disks {
            let mut labels = HashMap::new();
            labels.insert("device".to_string(), disk.device);
            labels.insert("mount".to_string(), disk.mount);

            metrics.push(RawMetric {
                key: "disk_total_bytes".to_string(),
                value: disk.total as f64,
                metric_type: MetricType::Gauge,
                labels: labels.clone(),
                timestamp: data.timestamp,
            });
            metrics.push(RawMetric {
                key: "disk_used_bytes".to_string(),
                value: disk.used as f64,
                metric_type: MetricType::Gauge,
                labels: labels.clone(),
                timestamp: data.timestamp,
            });
            metrics.push(RawMetric {
                key: "disk_free_bytes".to_string(),
                value: disk.free as f64,
                metric_type: MetricType::Gauge,
                labels,
                timestamp: data.timestamp,
            });
        }

        // Disk I/O counters
        for io in data.disk_io {
            let mut labels = HashMap::new();
            labels.insert("device".to_string(), io.device);

            metrics.push(RawMetric {
                key: "disk_read_bytes_total".to_string(),
                value: io.read_bytes as f64,
                metric_type: MetricType::Counter,
                labels: labels.clone(),
                timestamp: data.timestamp,
            });
            metrics.push(RawMetric {
                key: "disk_write_bytes_total".to_string(),
                value: io.write_bytes as f64,
                metric_type: MetricType::Counter,
                labels,
                timestamp: data.timestamp,
            });
        }

        // Network counters
        for net in data.network {
            let mut labels = HashMap::new();
            labels.insert("interface".to_string(), net.interface);

            metrics.push(RawMetric {
                key: "network_transmit_bytes_total".to_string(),
                value: net.tx_bytes as f64,
                metric_type: MetricType::Counter,
                labels: labels.clone(),
                timestamp: data.timestamp,
            });
            metrics.push(RawMetric {
                key: "network_receive_bytes_total".to_string(),
                value: net.rx_bytes as f64,
                metric_type: MetricType::Counter,
                labels,
                timestamp: data.timestamp,
            });
        }

        Ok(metrics)
    }

    async fn health_check(&self, server: &crate::models::server::ServerConfig) -> AppResult<bool> {
        let url = format!("http://{}:{}/api/health", server.host, server.port);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::GoAgentResponse;

    #[test]
    fn deserializes_current_go_agent_payload() {
        let payload = r#"{
          "timestamp": 1742371200,
          "cpu": {"total_percent": 25.0, "per_core": [20.0, 30.0]},
          "memory": {"total": 1024, "used": 512, "free": 256, "cached": 256, "available": 512, "used_percent": 50.0},
          "disks": [{"mount": "/", "device": "/dev/vda1", "total": 1000, "used": 400, "free": 600, "used_percent": 40.0}],
          "disk_io": [{"device": "vda", "read_bytes": 10, "write_bytes": 20}],
          "network": [{"interface": "eth0", "rx_bytes": 30, "tx_bytes": 40}]
        }"#;

        let parsed: GoAgentResponse =
            serde_json::from_str(payload).expect("payload should deserialize");
        assert_eq!(parsed.cpu.per_core.len(), 2);
        assert_eq!(parsed.disks[0].mount, "/");
        assert_eq!(parsed.disk_io[0].device, "vda");
        assert_eq!(parsed.network[0].interface, "eth0");
    }
}
