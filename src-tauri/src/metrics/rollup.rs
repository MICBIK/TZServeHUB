use crate::error::AppResult;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use tokio::time::{interval, Duration};

pub struct RollupEngine {
    pool: SqlitePool,
}

impl RollupEngine {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn generate_1m_rollup(
        &self,
        server_id: &str,
        key: &str,
        labels: &str,
        vantage_point: &str,
        from: i64,
        to: i64,
    ) -> AppResult<()> {
        self.generate_rollup(
            "metrics_1m",
            60,
            "raw_metrics",
            server_id,
            key,
            labels,
            vantage_point,
            from,
            to,
        )
        .await
    }

    pub async fn generate_15m_rollup(
        &self,
        server_id: &str,
        key: &str,
        labels: &str,
        vantage_point: &str,
        from: i64,
        to: i64,
    ) -> AppResult<()> {
        self.generate_rollup(
            "metrics_15m",
            900,
            "metrics_1m",
            server_id,
            key,
            labels,
            vantage_point,
            from,
            to,
        )
        .await
    }

    async fn generate_rollup(
        &self,
        table: &str,
        bucket_size: i64,
        source_table: &str,
        server_id: &str,
        key: &str,
        labels: &str,
        vantage_point: &str,
        from: i64,
        to: i64,
    ) -> AppResult<()> {
        let bucket_start = (from / bucket_size) * bucket_size;
        let bucket_end = (to / bucket_size) * bucket_size;
        let value_expr = if source_table == "raw_metrics" {
            "value"
        } else {
            "avg_val"
        };
        let sql = format!(
            "INSERT OR REPLACE INTO {table} (server_id, key, labels, vantage_point, min_val, max_val, avg_val, bucket)
             SELECT ?, ?, ?, ?, MIN({value_expr}), MAX({value_expr}), AVG({value_expr}), ?
             FROM {source_table}
             WHERE server_id = ? AND key = ? AND labels = ? AND vantage_point = ? AND {time_col} >= ? AND {time_col} < ?
             HAVING COUNT(*) > 0",
            time_col = if source_table == "raw_metrics" { "timestamp" } else { "bucket" },
        );

        let mut current_bucket = bucket_start;
        while current_bucket <= bucket_end {
            sqlx::query(&sql)
                .bind(server_id)
                .bind(key)
                .bind(labels)
                .bind(vantage_point)
                .bind(current_bucket)
                .bind(server_id)
                .bind(key)
                .bind(labels)
                .bind(vantage_point)
                .bind(current_bucket)
                .bind(current_bucket + bucket_size)
                .execute(&self.pool)
                .await?;
            current_bucket += bucket_size;
        }

        Ok(())
    }

    pub fn start(pool: SqlitePool) {
        let engine = Self::new(pool);
        tokio::spawn(async move {
            let mut interval_timer = interval(Duration::from_secs(300));
            loop {
                interval_timer.tick().await;
                if let Err(e) = engine.run_rollup_cycle().await {
                    log::error!("Rollup cycle failed: {e}");
                }
            }
        });
    }

    async fn run_rollup_cycle(&self) -> AppResult<()> {
        let now = Utc::now().timestamp();
        let rows = sqlx::query(
            "SELECT DISTINCT server_id, key, COALESCE(labels, '{}') AS labels, vantage_point
             FROM raw_metrics WHERE timestamp >= ?",
        )
        .bind(now - 3600)
        .fetch_all(&self.pool)
        .await?;

        for row in rows {
            let server_id: String = row.get("server_id");
            let key: String = row.get("key");
            let labels: String = row.get("labels");
            let vantage_point: String = row.get("vantage_point");

            self.generate_1m_rollup(&server_id, &key, &labels, &vantage_point, now - 3600, now)
                .await?;
            self.generate_15m_rollup(&server_id, &key, &labels, &vantage_point, now - 14400, now)
                .await?;
        }

        Ok(())
    }
}
