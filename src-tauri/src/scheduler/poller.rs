use tokio::time::{interval, Duration};
use std::sync::Arc;
use crate::adapters::MetricAdapter;
use crate::models::server::ServerConfig;
use crate::error::AppResult;

pub struct Poller {
    servers: Vec<ServerConfig>,
    adapters: Vec<Arc<dyn MetricAdapter>>,
}

impl Poller {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            adapters: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server: ServerConfig, adapter: Arc<dyn MetricAdapter>) {
        self.servers.push(server);
        self.adapters.push(adapter);
    }

    pub async fn start(&self) -> AppResult<()> {
        let mut ticker = interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;
            self.poll_all().await?;
        }
    }

    async fn poll_all(&self) -> AppResult<()> {
        for (server, adapter) in self.servers.iter().zip(self.adapters.iter()) {
            if !server.enabled {
                continue;
            }

            match adapter.fetch_host_metrics(server).await {
                Ok(metrics) => {
                    log::info!("Fetched {} metrics from {}", metrics.len(), server.name);
                    // TODO: Store metrics in database
                }
                Err(e) => {
                    log::error!("Failed to fetch metrics from {}: {:?}", server.name, e);
                }
            }
        }

        Ok(())
    }
}
