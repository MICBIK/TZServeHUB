use sqlx::SqlitePool;
use chrono::Utc;
use crate::error::AppResult;

const RAW_RETENTION_DAYS: i64 = 7;
const ROLLUP_1M_RETENTION_DAYS: i64 = 30;
const ROLLUP_15M_RETENTION_DAYS: i64 = 90;

pub struct RetentionManager {
    pool: SqlitePool,
}

impl RetentionManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn cleanup_old_data(&self) -> AppResult<()> {
        let now = Utc::now().timestamp();

        let raw_cutoff = now - (RAW_RETENTION_DAYS * 86400);
        sqlx::query("DELETE FROM raw_metrics WHERE timestamp < ?")
            .bind(raw_cutoff)
            .execute(&self.pool)
            .await?;

        let rollup_1m_cutoff = now - (ROLLUP_1M_RETENTION_DAYS * 86400);
        sqlx::query("DELETE FROM metrics_1m WHERE bucket < ?")
            .bind(rollup_1m_cutoff)
            .execute(&self.pool)
            .await?;

        let rollup_15m_cutoff = now - (ROLLUP_15M_RETENTION_DAYS * 86400);
        sqlx::query("DELETE FROM metrics_15m WHERE bucket < ?")
            .bind(rollup_15m_cutoff)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn vacuum(&self) -> AppResult<()> {
        sqlx::query("VACUUM")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
