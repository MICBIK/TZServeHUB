#![allow(dead_code)]

use crate::error::AppResult;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn init() -> AppResult<SqlitePool> {
        let app_data_dir = resolve_app_data_dir()?;

        std::fs::create_dir_all(&app_data_dir)?;
        let db_path = app_data_dir.join("data.db");
        let connection_string = format!("sqlite://{}?mode=rwc", db_path.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await?;

        sqlx::query("PRAGMA journal_mode=WAL")
            .execute(&pool)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        if crate::storage::secrets::has_legacy_plaintext_credentials(&pool).await? {
            let created_store =
                crate::storage::secrets::factory::SecretStoreFactory::create(
                    crate::storage::secrets::factory::SecretStoreFactoryConfig::new(
                        "serverhub",
                        app_data_dir.clone(),
                    ),
                )
                .await?;
            crate::storage::secrets::migrate_legacy_plaintext_to_keychain(
                &pool,
                created_store.store().as_ref(),
            )
            .await?;
        }

        Ok(pool)
    }

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
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}

pub fn resolve_app_data_dir() -> AppResult<PathBuf> {
    if let Ok(path) = std::env::var("SERVERHUB_DATA_DIR") {
        return Ok(PathBuf::from(path));
    }
    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(".serverhub"));
    }
    Ok(std::env::current_dir()?.join(".serverhub"))
}

pub async fn init() -> AppResult<SqlitePool> {
    Database::init().await
}
