#![allow(dead_code)]

use crate::error::AppResult;
use chrono::Utc;
use log::{error, info};
use sqlx::SqlitePool;
use tokio::time::{interval, Duration};

const RAW_RETENTION_DAYS: i64 = 7;
const ROLLUP_1M_RETENTION_DAYS: i64 = 30;
const ROLLUP_15M_RETENTION_DAYS: i64 = 90;

pub struct RetentionManager {
    pool: SqlitePool,
}

pub async fn start(pool: SqlitePool) {
    let manager = RetentionManager::new(pool);

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600)); // 1 hour

        loop {
            interval.tick().await;

            match manager.cleanup_old_data().await {
                Ok(()) => {
                    info!("Retention cleanup task completed successfully");
                }
                Err(e) => {
                    error!("Retention cleanup task failed: {}", e);
                }
            }
        }
    });
}

impl RetentionManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn cleanup_old_data(&self) -> AppResult<()> {
        let now = Utc::now().timestamp();

        let raw_cutoff = now - (RAW_RETENTION_DAYS * 86400);
        let raw_result = sqlx::query("DELETE FROM raw_metrics WHERE timestamp < ?")
            .bind(raw_cutoff)
            .execute(&self.pool)
            .await?;

        let rollup_1m_cutoff = now - (ROLLUP_1M_RETENTION_DAYS * 86400);
        let rollup_1m_result = sqlx::query("DELETE FROM metrics_1m WHERE bucket < ?")
            .bind(rollup_1m_cutoff)
            .execute(&self.pool)
            .await?;

        let rollup_15m_cutoff = now - (ROLLUP_15M_RETENTION_DAYS * 86400);
        let rollup_15m_result = sqlx::query("DELETE FROM metrics_15m WHERE bucket < ?")
            .bind(rollup_15m_cutoff)
            .execute(&self.pool)
            .await?;

        info!(
            "Retention cleanup completed: {} raw_metrics, {} metrics_1m, {} metrics_15m deleted",
            raw_result.rows_affected(),
            rollup_1m_result.rows_affected(),
            rollup_15m_result.rows_affected()
        );

        Ok(())
    }

    pub async fn vacuum(&self) -> AppResult<()> {
        sqlx::query("VACUUM").execute(&self.pool).await?;
        Ok(())
    }
}
