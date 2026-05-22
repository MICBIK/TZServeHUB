use crate::error::AppResult;
use crate::models::alert::AlertEvent;
use crate::storage::secrets::SecretStore;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannelKind {
    Desktop,
    Webhook,
    Email,
    Telegram,
}

impl NotificationChannelKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Webhook => "webhook",
            Self::Email => "email",
            Self::Telegram => "telegram",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannelConfig {
    pub id: String,
    pub kind: NotificationChannelKind,
    pub name: String,
    pub enabled: bool,
    pub config_json: Value,
    pub secret_ref: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
pub trait NotificationChannelTestDispatcher: Send + Sync {
    async fn test_channel(
        &self,
        channel: &NotificationChannelConfig,
        event: &AlertEvent,
    ) -> AppResult<()>;
}

pub async fn list_notification_channels_from_pool(
    _pool: &SqlitePool,
) -> AppResult<Vec<NotificationChannelConfig>> {
    Ok(Vec::new())
}

pub async fn add_notification_channel_with_store(
    _pool: &SqlitePool,
    _store: &dyn SecretStore,
    kind: NotificationChannelKind,
    name: String,
    config_json: Value,
) -> AppResult<NotificationChannelConfig> {
    Ok(NotificationChannelConfig {
        id: "red-placeholder".to_string(),
        kind,
        name,
        enabled: true,
        config_json,
        secret_ref: None,
        created_at: 0,
        updated_at: 0,
    })
}

pub async fn remove_notification_channel_with_store(
    _pool: &SqlitePool,
    _store: &dyn SecretStore,
    _id: &str,
) -> AppResult<()> {
    Ok(())
}

pub async fn update_notification_channel_with_store(
    _pool: &SqlitePool,
    _store: &dyn SecretStore,
    _id: &str,
    _name: String,
    _enabled: bool,
    _config_json: Value,
) -> AppResult<NotificationChannelConfig> {
    Ok(NotificationChannelConfig {
        id: "red-placeholder".to_string(),
        kind: NotificationChannelKind::Desktop,
        name: "red".to_string(),
        enabled: true,
        config_json: serde_json::json!({}),
        secret_ref: None,
        created_at: 0,
        updated_at: 0,
    })
}

pub async fn test_notification_channel_with_dispatcher(
    _pool: &SqlitePool,
    _dispatcher: &dyn NotificationChannelTestDispatcher,
    _id: &str,
) -> AppResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppResult;
    use crate::storage::secrets::SecretStore;
    use sqlx::{sqlite::SqlitePoolOptions, Row};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[derive(Default)]
    struct MemorySecretStore {
        entries: Arc<Mutex<HashMap<String, String>>>,
        deletes: Arc<Mutex<Vec<String>>>,
    }

    impl MemorySecretStore {
        fn value_for(&self, key: &str) -> Option<String> {
            self.entries.lock().expect("entries lock").get(key).cloned()
        }

        fn deleted_keys(&self) -> Vec<String> {
            self.deletes.lock().expect("deletes lock").clone()
        }
    }

    #[async_trait]
    impl SecretStore for MemorySecretStore {
        async fn put(&self, key: &str, value: &str) -> AppResult<()> {
            self.entries
                .lock()
                .expect("entries lock")
                .insert(key.to_string(), value.to_string());
            Ok(())
        }

        async fn get(&self, key: &str) -> AppResult<Option<String>> {
            Ok(self.value_for(key))
        }

        async fn delete(&self, key: &str) -> AppResult<()> {
            self.deletes
                .lock()
                .expect("deletes lock")
                .push(key.to_string());
            self.entries.lock().expect("entries lock").remove(key);
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingDispatcher {
        calls: Arc<Mutex<Vec<(String, AlertEvent)>>>,
    }

    #[async_trait]
    impl NotificationChannelTestDispatcher for RecordingDispatcher {
        async fn test_channel(
            &self,
            channel: &NotificationChannelConfig,
            event: &AlertEvent,
        ) -> AppResult<()> {
            self.calls
                .lock()
                .expect("calls lock")
                .push((channel.id.clone(), event.clone()));
            Ok(())
        }
    }

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite pool");

        sqlx::query(
            r#"
            CREATE TABLE notification_channels (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                config_json TEXT NOT NULL DEFAULT '{}',
                secret_ref TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await
        .expect("notification_channels test table");

        pool
    }

    async fn insert_channel(
        pool: &SqlitePool,
        id: &str,
        kind: &str,
        name: &str,
        secret_ref: Option<&str>,
        updated_at: i64,
    ) {
        sqlx::query(
            r#"
            INSERT INTO notification_channels
                (id, kind, name, enabled, config_json, secret_ref, created_at, updated_at)
            VALUES (?, ?, ?, 1, '{"url":"https://example.test/hook"}', ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(kind)
        .bind(name)
        .bind(secret_ref)
        .bind(updated_at - 10)
        .bind(updated_at)
        .execute(pool)
        .await
        .expect("insert channel fixture");
    }

    #[tokio::test]
    async fn list_channels_returns_persisted_rows() {
        let pool = test_pool().await;
        insert_channel(&pool, "old", "webhook", "Old Hook", None, 10).await;
        insert_channel(&pool, "new", "desktop", "Desktop", None, 20).await;

        let channels = list_notification_channels_from_pool(&pool)
            .await
            .expect("list notification channels");

        assert_eq!(channels.len(), 2);
        assert_eq!(channels[0].id, "new");
        assert_eq!(channels[0].kind, NotificationChannelKind::Desktop);
        assert_eq!(channels[1].id, "old");
    }

    #[tokio::test]
    async fn add_channel_persists_and_writes_secrets_to_keychain() {
        let pool = test_pool().await;
        let store = MemorySecretStore::default();

        let channel = add_notification_channel_with_store(
            &pool,
            &store,
            NotificationChannelKind::Webhook,
            "Deploy Hook".to_string(),
            serde_json::json!({
                "url": "https://example.test/hook",
                "secret": "super-secret-token"
            }),
        )
        .await
        .expect("add notification channel");

        let row = sqlx::query(
            "SELECT kind, name, config_json, secret_ref FROM notification_channels WHERE id = ?",
        )
        .bind(&channel.id)
        .fetch_one(&pool)
        .await
        .expect("channel row should be inserted");
        assert_eq!(row.get::<String, _>("kind"), "webhook");
        assert_eq!(row.get::<String, _>("name"), "Deploy Hook");

        let config_json: String = row.get("config_json");
        assert!(config_json.contains("https://example.test/hook"));
        assert!(!config_json.contains("super-secret-token"));

        let secret_ref: String = row
            .get::<Option<String>, _>("secret_ref")
            .expect("secret_ref should be stored");
        assert_eq!(
            store.value_for(&secret_ref).as_deref(),
            Some("super-secret-token")
        );
    }

    #[tokio::test]
    async fn remove_channel_cleans_keychain_entries() {
        let pool = test_pool().await;
        let store = MemorySecretStore::default();
        store
            .put("serverhub.notification_channel.chan-1.secret", "secret-value")
            .await
            .expect("seed secret");
        insert_channel(
            &pool,
            "chan-1",
            "webhook",
            "Webhook",
            Some("serverhub.notification_channel.chan-1.secret"),
            10,
        )
        .await;

        remove_notification_channel_with_store(&pool, &store, "chan-1")
            .await
            .expect("remove notification channel");

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM notification_channels WHERE id = 'chan-1'")
                .fetch_one(&pool)
                .await
                .expect("count channel rows");
        assert_eq!(count, 0);
        assert_eq!(
            store.deleted_keys(),
            vec!["serverhub.notification_channel.chan-1.secret".to_string()]
        );
    }

    #[tokio::test]
    async fn test_channel_constructs_mock_event_and_dispatches() {
        let pool = test_pool().await;
        let dispatcher = RecordingDispatcher::default();
        insert_channel(&pool, "chan-1", "desktop", "Desktop", None, 10).await;

        test_notification_channel_with_dispatcher(&pool, &dispatcher, "chan-1")
            .await
            .expect("test notification channel");

        let calls = dispatcher.calls.lock().expect("calls lock");
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "chan-1");
        assert_eq!(calls[0].1.status.as_str(), "firing");
        assert_eq!(calls[0].1.message, "ServerHUB test notification");
    }
}
