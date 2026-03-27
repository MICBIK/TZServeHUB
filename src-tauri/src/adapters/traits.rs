use crate::error::AppResult;
use crate::models::metric::RawMetric;
use crate::models::server::ServerConfig;
use async_trait::async_trait;

#[async_trait]
pub trait MetricAdapter: Send + Sync {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    async fn fetch_host_metrics(&self, server: &ServerConfig) -> AppResult<Vec<RawMetric>>;
    #[allow(dead_code)]
    async fn health_check(&self, server: &ServerConfig) -> AppResult<bool>;
}
