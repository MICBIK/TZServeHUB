use crate::alerts::notifier::AlertNotifier;
use crate::error::{AppError, AppResult};
use crate::models::alert::{AlertEvent, AlertStatus};
use crate::storage::{
    database::resolve_app_data_dir,
    secrets::{
        factory::{SecretStoreFactory, SecretStoreFactoryConfig},
        SecretStore,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use tauri::{AppHandle, Runtime, State};

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

    pub fn from_db(value: &str) -> AppResult<Self> {
        match value {
            "desktop" => Ok(Self::Desktop),
            "webhook" => Ok(Self::Webhook),
            "email" => Ok(Self::Email),
            "telegram" => Ok(Self::Telegram),
            _ => Err(AppError::Custom(format!(
                "Unknown notification channel kind: {value}"
            ))),
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

pub struct TauriNotificationChannelTestDispatcher<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> TauriNotificationChannelTestDispatcher<R> {
    fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl<R: Runtime> NotificationChannelTestDispatcher for TauriNotificationChannelTestDispatcher<R> {
    async fn test_channel(
        &self,
        channel: &NotificationChannelConfig,
        event: &AlertEvent,
    ) -> AppResult<()> {
        match channel.kind {
            NotificationChannelKind::Desktop => {
                let report = AlertNotifier::new(self.app.clone()).send_alert(event).await;
                if let Some(failure) = report.channels.iter().find(|status| !status.success) {
                    return Err(AppError::Notification(
                        failure
                            .error
                            .clone()
                            .unwrap_or_else(|| "desktop notification failed".to_string()),
                    ));
                }
                Ok(())
            }
            _ => Err(AppError::Custom(format!(
                "{} notification channel testing is not implemented yet",
                channel.kind.as_str()
            ))),
        }
    }
}

#[tauri::command]
pub async fn list_notification_channels(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<NotificationChannelConfig>, String> {
    list_notification_channels_from_pool(pool.inner())
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn add_notification_channel(
    pool: State<'_, SqlitePool>,
    kind: NotificationChannelKind,
    name: String,
    config: Value,
) -> Result<NotificationChannelConfig, String> {
    let result: AppResult<NotificationChannelConfig> = async {
        let created_store = SecretStoreFactory::create(SecretStoreFactoryConfig::new(
            "serverhub",
            resolve_app_data_dir()?,
        ))
        .await?;

        add_notification_channel_with_store(
            pool.inner(),
            created_store.store().as_ref(),
            kind,
            name,
            config,
        )
        .await
    }
    .await;

    result.map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn remove_notification_channel(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    let result: AppResult<()> = async {
        let created_store = SecretStoreFactory::create(SecretStoreFactoryConfig::new(
            "serverhub",
            resolve_app_data_dir()?,
        ))
        .await?;
        remove_notification_channel_with_store(pool.inner(), created_store.store().as_ref(), &id)
            .await
    }
    .await;

    result.map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_notification_channel(
    pool: State<'_, SqlitePool>,
    id: String,
    name: String,
    enabled: bool,
    config: Value,
) -> Result<NotificationChannelConfig, String> {
    let result: AppResult<NotificationChannelConfig> = async {
        let created_store = SecretStoreFactory::create(SecretStoreFactoryConfig::new(
            "serverhub",
            resolve_app_data_dir()?,
        ))
        .await?;
        update_notification_channel_with_store(
            pool.inner(),
            created_store.store().as_ref(),
            &id,
            name,
            enabled,
            config,
        )
        .await
    }
    .await;

    result.map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn test_notification_channel<R: Runtime>(
    pool: State<'_, SqlitePool>,
    app: AppHandle<R>,
    id: String,
) -> Result<(), String> {
    let dispatcher = TauriNotificationChannelTestDispatcher::new(app);
    test_notification_channel_with_dispatcher(pool.inner(), &dispatcher, &id)
        .await
        .map_err(|error| error.to_string())
}

pub async fn list_notification_channels_from_pool(
    pool: &SqlitePool,
) -> AppResult<Vec<NotificationChannelConfig>> {
    let rows = sqlx::query(
        r#"
        SELECT id, kind, name, enabled, config_json, secret_ref, created_at, updated_at
        FROM notification_channels
        ORDER BY updated_at DESC, name ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.iter().map(row_to_notification_channel).collect()
}

pub async fn add_notification_channel_with_store(
    pool: &SqlitePool,
    store: &dyn SecretStore,
    kind: NotificationChannelKind,
    name: String,
    mut config_json: Value,
) -> AppResult<NotificationChannelConfig> {
    let name = normalize_name(name)?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();
    let secret_ref = persist_channel_secret(store, &id, kind, &mut config_json).await?;
    let config_string = serde_json::to_string(&config_json)?;

    sqlx::query(
        r#"
        INSERT INTO notification_channels
            (id, kind, name, enabled, config_json, secret_ref, created_at, updated_at)
        VALUES (?, ?, ?, 1, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(kind.as_str())
    .bind(&name)
    .bind(config_string)
    .bind(&secret_ref)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(NotificationChannelConfig {
        id,
        kind,
        name,
        enabled: true,
        config_json,
        secret_ref,
        created_at: now,
        updated_at: now,
    })
}

pub async fn remove_notification_channel_with_store(
    pool: &SqlitePool,
    store: &dyn SecretStore,
    id: &str,
) -> AppResult<()> {
    let id = normalize_id(id)?;
    if let Some(secret_ref) = notification_channel_secret_ref(pool, &id).await? {
        store.delete(&secret_ref).await?;
    }

    sqlx::query("DELETE FROM notification_channels WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_notification_channel_with_store(
    pool: &SqlitePool,
    store: &dyn SecretStore,
    id: &str,
    name: String,
    enabled: bool,
    mut config_json: Value,
) -> AppResult<NotificationChannelConfig> {
    let id = normalize_id(id)?;
    let name = normalize_name(name)?;
    let existing = get_notification_channel_from_pool(pool, &id).await?;
    let new_secret_ref =
        persist_channel_secret(store, &id, existing.kind, &mut config_json).await?;
    let secret_ref = new_secret_ref.or(existing.secret_ref);
    let now = chrono::Utc::now().timestamp();
    let config_string = serde_json::to_string(&config_json)?;

    sqlx::query(
        r#"
        UPDATE notification_channels
        SET name = ?, enabled = ?, config_json = ?, secret_ref = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(&name)
    .bind(if enabled { 1_i64 } else { 0_i64 })
    .bind(config_string)
    .bind(&secret_ref)
    .bind(now)
    .bind(&id)
    .execute(pool)
    .await?;

    Ok(NotificationChannelConfig {
        id,
        name,
        enabled,
        config_json,
        secret_ref,
        updated_at: now,
        ..existing
    })
}

pub async fn test_notification_channel_with_dispatcher(
    pool: &SqlitePool,
    dispatcher: &dyn NotificationChannelTestDispatcher,
    id: &str,
) -> AppResult<()> {
    let channel = get_notification_channel_from_pool(pool, id).await?;
    let event = AlertEvent {
        id: uuid::Uuid::new_v4().to_string(),
        rule_id: "notification-channel-test".to_string(),
        server_id: channel.id.clone(),
        status: AlertStatus::Firing,
        message: "ServerHUB test notification".to_string(),
        fired_at: chrono::Utc::now().timestamp(),
        resolved_at: None,
        delivery_status: None,
    };

    dispatcher.test_channel(&channel, &event).await
}

async fn get_notification_channel_from_pool(
    pool: &SqlitePool,
    id: &str,
) -> AppResult<NotificationChannelConfig> {
    let id = normalize_id(id)?;
    let row = sqlx::query(
        r#"
        SELECT id, kind, name, enabled, config_json, secret_ref, created_at, updated_at
        FROM notification_channels
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::Custom("Notification channel not found".to_string()))?;

    row_to_notification_channel(&row)
}

fn row_to_notification_channel(
    row: &sqlx::sqlite::SqliteRow,
) -> AppResult<NotificationChannelConfig> {
    let config_json: String = row.get("config_json");
    Ok(NotificationChannelConfig {
        id: row.get("id"),
        kind: NotificationChannelKind::from_db(&row.get::<String, _>("kind"))?,
        name: row.get("name"),
        enabled: row.get::<i64, _>("enabled") != 0,
        config_json: serde_json::from_str(&config_json)?,
        secret_ref: row.get("secret_ref"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

async fn notification_channel_secret_ref(pool: &SqlitePool, id: &str) -> AppResult<Option<String>> {
    let secret_ref =
        sqlx::query_scalar("SELECT secret_ref FROM notification_channels WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(secret_ref.flatten())
}

async fn persist_channel_secret(
    store: &dyn SecretStore,
    id: &str,
    kind: NotificationChannelKind,
    config_json: &mut Value,
) -> AppResult<Option<String>> {
    let Some(secret) = extract_secret(kind, config_json) else {
        return Ok(None);
    };

    let key = secret_key_for_channel(id);
    store.put(&key, &secret).await?;
    Ok(Some(key))
}

fn extract_secret(kind: NotificationChannelKind, config_json: &mut Value) -> Option<String> {
    let object = config_json.as_object_mut()?;
    let keys: &[&str] = match kind {
        NotificationChannelKind::Desktop => &[],
        NotificationChannelKind::Webhook => &["secret", "webhook_secret"],
        NotificationChannelKind::Email => &["smtp_password", "password"],
        NotificationChannelKind::Telegram => &["bot_token"],
    };

    for key in keys {
        if let Some(value) = object.remove(*key) {
            if let Some(secret) = value.as_str().map(str::trim).filter(|s| !s.is_empty()) {
                return Some(secret.to_string());
            }
        }
    }

    None
}

fn secret_key_for_channel(id: &str) -> String {
    format!("serverhub.notification_channel.{id}.secret")
}

fn normalize_name(name: String) -> AppResult<String> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Custom(
            "Notification channel name cannot be empty".to_string(),
        ));
    }
    Ok(name)
}

fn normalize_id(id: &str) -> AppResult<String> {
    let id = id.trim().to_string();
    if id.is_empty() {
        return Err(AppError::Custom(
            "Notification channel ID cannot be empty".to_string(),
        ));
    }
    Ok(id)
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
            .put(
                "serverhub.notification_channel.chan-1.secret",
                "secret-value",
            )
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
