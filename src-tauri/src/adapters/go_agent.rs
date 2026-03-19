use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use crate::adapters::traits::MetricAdapter;
use crate::models::metric::{RawMetric, MetricType};
use crate::error::AppResult;

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
    available: u64,
    percent: f64,
}

#[derive(Deserialize)]
struct DiskMetrics {
    device: String,
    mountpoint: String,
    fstype: String,
    total: u64,
    used: u64,
    free: u64,
    percent: f64,
}

#[derive(Deserialize)]
struct DiskIO {
    name: String,
    read_bytes: u64,
    write_bytes: u64,
}

#[derive(Deserialize)]
struct NetMetrics {
    name: String,
    bytes_sent: u64,
    bytes_recv: u64,
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

    async fn fetch_host_metrics(&self, server: &crate::models::server::ServerConfig) -> AppResult<Vec<RawMetric>> {
        let url = format!("http://{}:{}/api/metrics", server.host, server.port);
        let response = self.client
            .get(&url)
            .send()
            .await?;

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
            key: "memory_available_bytes".to_string(),
            value: data.memory.available as f64,
            metric_type: MetricType::Gauge,
            labels: HashMap::new(),
            timestamp: data.timestamp,
        });

        // Disk metrics
        for disk in data.disks {
            let mut labels = HashMap::new();
            labels.insert("device".to_string(), disk.device);
            labels.insert("mountpoint".to_string(), disk.mountpoint);
            labels.insert("fstype".to_string(), disk.fstype);

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
            labels.insert("device".to_string(), io.name);

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
            labels.insert("interface".to_string(), net.name);

            metrics.push(RawMetric {
                key: "network_transmit_bytes_total".to_string(),
                value: net.bytes_sent as f64,
                metric_type: MetricType::Counter,
                labels: labels.clone(),
                timestamp: data.timestamp,
            });
            metrics.push(RawMetric {
                key: "network_receive_bytes_total".to_string(),
                value: net.bytes_recv as f64,
                metric_type: MetricType::Counter,
                labels,
                timestamp: data.timestamp,
            });
        }

        Ok(metrics)
    }

    async fn health_check(&self, server: &crate::models::server::ServerConfig) -> AppResult<bool> {
        let url = format!("http://{}:{}/api/health", server.host, server.port);
        let response = self.client
            .get(&url)
            .send()
            .await?;
        Ok(response.status().is_success())
    }
}
