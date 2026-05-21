//! Secret storage abstraction.

pub mod encrypted_file;
pub mod factory;

use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

/// Abstract backend for sensitive string storage.
///
/// Secret keys should use the `serverhub.<scope>.<id>.<field>` namespace.
#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn put(&self, key: &str, value: &str) -> AppResult<()>;
    async fn get(&self, key: &str) -> AppResult<Option<String>>;
    async fn delete(&self, key: &str) -> AppResult<()>;
}

/// OS-native keychain implementation backed by the `keyring` crate.
pub struct OsKeychainStore {
    service: String,
}

impl OsKeychainStore {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
        }
    }

    fn entry(&self, key: &str) -> AppResult<keyring::Entry> {
        keyring::Entry::new(&self.service, key).map_err(keyring_to_app_error)
    }
}

#[async_trait]
impl SecretStore for OsKeychainStore {
    async fn put(&self, key: &str, value: &str) -> AppResult<()> {
        self.entry(key)?
            .set_password(value)
            .map_err(keyring_to_app_error)
    }

    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        match self.entry(key)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(keyring_to_app_error(error)),
        }
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        match self.entry(key)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(keyring_to_app_error(error)),
        }
    }
}

fn keyring_to_app_error(error: keyring::Error) -> AppError {
    AppError::Custom(format!("Keychain error: {error}"))
}

pub async fn has_legacy_plaintext_credentials(pool: &SqlitePool) -> AppResult<bool> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM servers
        WHERE auth_token IS NOT NULL
           OR ssh_passphrase IS NOT NULL
           OR password IS NOT NULL
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}

/// Migrate legacy plaintext credentials from `servers` into a SecretStore.
///
/// Store writes happen before SQLite is updated. If a store write fails, the
/// plaintext columns remain intact so the migration can be retried.
pub async fn migrate_legacy_plaintext_to_keychain(
    pool: &SqlitePool,
    store: &dyn SecretStore,
) -> AppResult<()> {
    let rows = sqlx::query(
        r#"
        SELECT id, auth_token, ssh_passphrase, password
        FROM servers
        WHERE auth_token IS NOT NULL
           OR ssh_passphrase IS NOT NULL
           OR password IS NOT NULL
        "#,
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let server_id: String = row.get("id");
        let credentials = [
            ("auth_token", row.get::<Option<String>, _>("auth_token")),
            (
                "ssh_passphrase",
                row.get::<Option<String>, _>("ssh_passphrase"),
            ),
            ("password", row.get::<Option<String>, _>("password")),
        ];

        let mut migrated = Vec::new();
        for (field, value) in credentials {
            if let Some(secret) = value.filter(|value| !value.is_empty()) {
                let key = secret_key_for_server_field(&server_id, field);
                store.put(&key, &secret).await?;
                migrated.push((field, key));
            }
        }

        if migrated.is_empty() {
            continue;
        }

        let mut tx = pool.begin().await?;
        for (field, key) in migrated {
            sqlx::query(
                r#"
                INSERT INTO secret_refs (server_id, field, secret_key, created_at, updated_at)
                VALUES (?, ?, ?, strftime('%s','now'), strftime('%s','now'))
                ON CONFLICT(server_id, field) DO UPDATE SET
                    secret_key = excluded.secret_key,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(&server_id)
            .bind(field)
            .bind(key)
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query(
            r#"
            UPDATE servers
            SET auth_token = NULL,
                ssh_passphrase = NULL,
                password = NULL,
                updated_at = strftime('%s','now')
            WHERE id = ?
            "#,
        )
        .bind(&server_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
    }

    Ok(())
}

fn secret_key_for_server_field(server_id: &str, field: &str) -> String {
    format!("serverhub.server.{server_id}.{field}")
}

#[cfg(test)]
mod tests {
    use super::SecretStore;
    use crate::error::AppResult;
    use async_trait::async_trait;

    struct MemoryStore;

    #[async_trait]
    impl SecretStore for MemoryStore {
        async fn put(&self, _key: &str, _value: &str) -> AppResult<()> {
            Ok(())
        }

        async fn get(&self, _key: &str) -> AppResult<Option<String>> {
            Ok(Some("secret-value".to_string()))
        }

        async fn delete(&self, _key: &str) -> AppResult<()> {
            Ok(())
        }
    }

    /// KEY-001: SecretStore trait must expose async put/get/delete methods.
    #[tokio::test]
    async fn trait_provides_put_get_delete() {
        let store = MemoryStore;
        store
            .put("serverhub.test.001.token", "secret-value")
            .await
            .expect("put should return AppResult");
        let value = store
            .get("serverhub.test.001.token")
            .await
            .expect("get should return AppResult");
        assert_eq!(value.as_deref(), Some("secret-value"));
        store
            .delete("serverhub.test.001.token")
            .await
            .expect("delete should return AppResult");
    }
}
