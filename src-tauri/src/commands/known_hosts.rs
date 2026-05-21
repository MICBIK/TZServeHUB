//! IPC commands for known SSH hosts.
//!
use crate::deployer::known_hosts::KnownHost;
use crate::error::AppResult;
use sqlx::SqlitePool;
use tauri::State;

/// List all known SSH hosts sorted by last_seen DESC.
#[tauri::command]
pub async fn list_known_hosts(pool: State<'_, SqlitePool>) -> Result<Vec<KnownHost>, String> {
    list_known_hosts_from_pool(pool.inner())
        .await
        .map_err(|error| error.to_string())
}

pub async fn list_known_hosts_from_pool(pool: &SqlitePool) -> AppResult<Vec<KnownHost>> {
    let rows = sqlx::query_as::<_, KnownHostRow>(
        r#"
        SELECT host, port, fingerprint, algorithm, first_seen, last_seen
        FROM known_hosts
        ORDER BY last_seen DESC, host ASC, port ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(KnownHost::from).collect())
}

/// Remove one known host entry.
#[tauri::command]
pub async fn remove_known_host(
    pool: State<'_, SqlitePool>,
    host: String,
    port: u16,
) -> Result<(), String> {
    remove_known_host_from_pool(pool.inner(), &host, port)
        .await
        .map_err(|error| error.to_string())
}

pub async fn remove_known_host_from_pool(
    pool: &SqlitePool,
    host: &str,
    port: u16,
) -> AppResult<()> {
    sqlx::query("DELETE FROM known_hosts WHERE host = ?1 AND port = ?2")
        .bind(host)
        .bind(i64::from(port))
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct KnownHostRow {
    host: String,
    port: i64,
    fingerprint: String,
    algorithm: String,
    first_seen: i64,
    last_seen: i64,
}

impl From<KnownHostRow> for KnownHost {
    fn from(row: KnownHostRow) -> Self {
        Self {
            host: row.host,
            port: row.port as u16,
            fingerprint: row.fingerprint,
            algorithm: row.algorithm,
            first_seen: row.first_seen,
            last_seen: row.last_seen,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite pool");

        sqlx::query(
            r#"
            CREATE TABLE known_hosts (
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                fingerprint TEXT NOT NULL,
                algorithm TEXT NOT NULL,
                first_seen INTEGER NOT NULL,
                last_seen INTEGER NOT NULL,
                PRIMARY KEY (host, port)
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("known_hosts test table");

        pool
    }

    async fn insert_host(pool: &SqlitePool, host: &str, port: u16, last_seen: i64) {
        sqlx::query(
            r#"
            INSERT INTO known_hosts (host, port, fingerprint, algorithm, first_seen, last_seen)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(host)
        .bind(i64::from(port))
        .bind(format!("fingerprint-{host}"))
        .bind("ssh-ed25519")
        .bind(last_seen - 100)
        .bind(last_seen)
        .execute(pool)
        .await
        .expect("insert known host fixture");
    }

    /// HOST-005: list_known_hosts sorts records by last_seen DESC.
    #[tokio::test]
    async fn list_known_hosts_returns_records_sorted_by_last_seen() {
        let pool = test_pool().await;
        insert_host(&pool, "old.example", 22, 10).await;
        insert_host(&pool, "new.example", 22, 30).await;
        insert_host(&pool, "middle.example", 2222, 20).await;

        let hosts = list_known_hosts_from_pool(&pool)
            .await
            .expect("list known hosts");

        assert_eq!(hosts.len(), 3);
        assert_eq!(hosts[0].host, "new.example");
        assert_eq!(hosts[1].host, "middle.example");
        assert_eq!(hosts[2].host, "old.example");
        assert_eq!(hosts[1].port, 2222);
        assert_eq!(hosts[1].algorithm, "ssh-ed25519");
    }

    /// HOST-006: remove_known_host deletes the row.
    #[tokio::test]
    async fn remove_known_host_deletes_record() {
        let pool = test_pool().await;
        insert_host(&pool, "remove.example", 22, 20).await;
        insert_host(&pool, "keep.example", 22, 10).await;

        remove_known_host_from_pool(&pool, "remove.example", 22)
            .await
            .expect("remove known host");

        let hosts = list_known_hosts_from_pool(&pool)
            .await
            .expect("list known hosts");
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].host, "keep.example");
    }
}
