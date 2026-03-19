use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use crate::error::AppResult;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> AppResult<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}?mode=rwc", db_path))
            .await?;

        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&pool)
            .await?;

        sqlx::query("PRAGMA synchronous = NORMAL")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}
