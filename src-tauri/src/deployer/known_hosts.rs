//! SSH known_hosts persistence + fingerprint validation.

use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownHost {
    pub host: String,
    pub port: u16,
    pub fingerprint: String,
    pub algorithm: String,
    pub first_seen: i64,
    pub last_seen: i64,
}

/// Persistence + lookup for SSH host keys.
pub struct KnownHostsStore {
    pub pool: SqlitePool,
}

impl KnownHostsStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find a known host record by (host, port).
    pub async fn find(&self, host: &str, port: u16) -> AppResult<Option<KnownHost>> {
        let row = sqlx::query_as::<_, KnownHostRow>(
            r#"
            SELECT host, port, fingerprint, algorithm, first_seen, last_seen
            FROM known_hosts
            WHERE host = ?1 AND port = ?2
            "#,
        )
        .bind(host)
        .bind(i64::from(port))
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(KnownHost::from))
    }

    /// Insert a new known host (TOFU). Returns the freshly-inserted record.
    pub async fn insert(
        &self,
        host: &str,
        port: u16,
        fingerprint: &str,
        algorithm: &str,
    ) -> AppResult<KnownHost> {
        let now = now_unix();
        sqlx::query(
            r#"
            INSERT INTO known_hosts (host, port, fingerprint, algorithm, first_seen, last_seen)
            VALUES (?1, ?2, ?3, ?4, ?5, ?5)
            "#,
        )
        .bind(host)
        .bind(i64::from(port))
        .bind(fingerprint)
        .bind(algorithm)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(KnownHost {
            host: host.to_string(),
            port,
            fingerprint: fingerprint.to_string(),
            algorithm: algorithm.to_string(),
            first_seen: now,
            last_seen: now,
        })
    }

    /// Touch the last_seen column when a known host reconnects.
    pub async fn update_last_seen(&self, host: &str, port: u16) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE known_hosts
            SET last_seen = ?3
            WHERE host = ?1 AND port = ?2
            "#,
        )
        .bind(host)
        .bind(i64::from(port))
        .bind(now_unix())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Remove a known host record.
    pub async fn delete(&self, host: &str, port: u16) -> AppResult<()> {
        sqlx::query("DELETE FROM known_hosts WHERE host = ?1 AND port = ?2")
            .bind(host)
            .bind(i64::from(port))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Verify a presented fingerprint against a stored record.
    /// - returns `Ok(true)` on TOFU insert or matching fingerprint
    /// - returns `Err(AppError)` on mismatch
    pub async fn verify_or_insert(
        &self,
        host: &str,
        port: u16,
        fingerprint: &str,
        algorithm: &str,
    ) -> AppResult<bool> {
        match self.find(host, port).await? {
            None => {
                self.insert(host, port, fingerprint, algorithm).await?;
                Ok(true)
            }
            Some(record) if record.fingerprint == fingerprint => {
                self.update_last_seen(host, port).await?;
                Ok(true)
            }
            Some(record) => Err(AppError::Custom(format!(
                "Host key changed for {host}:{port}: stored fingerprint {}, received fingerprint {}",
                record.fingerprint, fingerprint
            ))),
        }
    }
}

/// Compute the SHA-256 hex fingerprint of an SSH public key.
pub fn fingerprint_public_key(key_bytes: &[u8]) -> String {
    let digest = Sha256::digest(key_bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
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

fn now_unix() -> i64 {
    chrono::Utc::now().timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn test_store() -> KnownHostsStore {
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

        KnownHostsStore::new(pool)
    }

    /// HOST-002: first connect inserts a new fingerprint row (TOFU).
    #[tokio::test]
    async fn tofu_inserts_fingerprint_on_first_seen() {
        let store = test_store().await;

        let accepted = store
            .verify_or_insert("example.test", 22, "fingerprint-a", "ssh-ed25519")
            .await
            .expect("TOFU should accept and insert");

        assert!(accepted);
        let record = store
            .find("example.test", 22)
            .await
            .expect("lookup should succeed")
            .expect("known host should be recorded");
        assert_eq!(record.fingerprint, "fingerprint-a");
        assert_eq!(record.algorithm, "ssh-ed25519");
        assert_eq!(record.first_seen, record.last_seen);
    }

    /// HOST-003: matching fingerprint on reconnect just bumps last_seen.
    #[tokio::test]
    async fn matching_fingerprint_updates_last_seen() {
        let store = test_store().await;
        store
            .insert("example.test", 22, "fingerprint-a", "ssh-ed25519")
            .await
            .expect("insert should succeed");
        sqlx::query("UPDATE known_hosts SET last_seen = last_seen - 10 WHERE host = ?1")
            .bind("example.test")
            .execute(&store.pool)
            .await
            .expect("test timestamp adjustment");
        let before = store
            .find("example.test", 22)
            .await
            .expect("lookup")
            .expect("record");

        let accepted = store
            .verify_or_insert("example.test", 22, "fingerprint-a", "ssh-ed25519")
            .await
            .expect("matching fingerprint should be accepted");

        let after = store
            .find("example.test", 22)
            .await
            .expect("lookup")
            .expect("record");
        assert!(accepted);
        assert_eq!(after.first_seen, before.first_seen);
        assert!(after.last_seen >= before.last_seen);
    }

    /// HOST-004: changed fingerprint must be rejected with an error.
    #[tokio::test]
    async fn mismatched_fingerprint_returns_error() {
        let store = test_store().await;
        store
            .insert("example.test", 22, "fingerprint-a", "ssh-ed25519")
            .await
            .expect("insert should succeed");

        let err = store
            .verify_or_insert("example.test", 22, "fingerprint-b", "ssh-ed25519")
            .await
            .expect_err("mismatch must be rejected");

        let message = err.to_string();
        assert!(message.contains("Host key changed for example.test:22"));
        assert!(message.contains("fingerprint-a"));
        assert!(message.contains("fingerprint-b"));
        let record = store
            .find("example.test", 22)
            .await
            .expect("lookup")
            .expect("record");
        assert_eq!(record.fingerprint, "fingerprint-a");
    }

    /// HOST-007: stored fingerprint includes the algorithm name (e.g. ssh-ed25519).
    #[tokio::test]
    async fn fingerprint_includes_algorithm_field() {
        let store = test_store().await;
        let record = store
            .insert("example.test", 22, "fingerprint-a", "ssh-rsa")
            .await
            .expect("insert should succeed");

        assert_eq!(record.algorithm, "ssh-rsa");
        assert_eq!(
            fingerprint_public_key(b"serverhub-test-key"),
            "e663f0bbc9dc30fc12b0a3fec8301279dabb40ca2ec0aeca190c2228c43b49b0"
        );
    }
}
