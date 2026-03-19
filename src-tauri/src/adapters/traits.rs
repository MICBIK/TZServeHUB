use async_trait::async_trait;
use crate::error::AppResult;
use crate::models::metric::RawMetric;
use crate::models::server::ServerConfig;

#[async_trait]
pub trait MetricAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
