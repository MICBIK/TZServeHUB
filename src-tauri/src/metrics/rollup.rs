use sqlx::SqlitePool;
use chrono::Utc;
use crate::error::AppResult;

pub struct RollupEngine {
    pool: SqlitePool,
}

impl RollupEngine {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn rollup_to_1m(&self) -> AppResult<()> {
        let now = Utc::now().timestamp();
        let bucket_start = (now / 60) * 60 - 60;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO metrics_1m (server_id, key, min_val, max_val, avg_val, bucket)
            SELECT
                server_id,
                key,
                MIN(value) as min_val,
                MAX(value) as max_val,
                AVG(value) as avg_val,
                ? as bucket
            FROM raw_metrics
            WHERE timestamp >= ? AND timestamp < ?
            GROUP BY server_id, key
            "#
        )
        .bind(bucket_start)
        .bind(bucket_start)
        .bind(bucket_start + 60)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn rollup_to_15m(&self) -> AppResult<()> {
        let now = Utc::now().timestamp();
        let bucket_start = (now / 900) * 900 - 900;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO metrics_15m (server_id, key, min_val, max_val, avg_val, bucket)
            SELECT
                server_id,
                key,
                MIN(min_val) as min_val,
                MAX(max_val) as max_val,
                AVG(avg_val) as avg_val,
                ? as bucket
            FROM metrics_1m
            WHERE bucket >= ? AND bucket < ?
            GROUP BY server_id, key
            "#
        )
        .bind(bucket_start)
        .bind(bucket_start)
        .bind(bucket_start + 900)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
