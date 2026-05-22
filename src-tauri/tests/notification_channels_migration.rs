//! NOTIF-008 — migration 009 must create the `notification_channels` table.

use sqlx::Row;
use std::{env, fs};

#[tokio::test]
async fn migration_009_creates_notification_channels_table() {
    let dir = env::temp_dir().join(format!(
        "serverhub-notification-channels-{}",
        uuid::Uuid::new_v4()
    ));
    fs::create_dir_all(&dir).expect("test data dir");
    env::set_var("SERVERHUB_DATA_DIR", &dir);

    let pool = serverhub_lib::storage::database::init()
        .await
        .expect("database init should succeed");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notification_channels")
        .fetch_one(&pool)
        .await
        .expect("notification_channels table must exist (migration 009)");
    assert_eq!(count, 0);

    let rows = sqlx::query("PRAGMA table_info(notification_channels)")
        .fetch_all(&pool)
        .await
        .expect("PRAGMA table_info should succeed");
    let cols: Vec<String> = rows.iter().map(|row| row.get::<String, _>("name")).collect();

    for required in [
        "id",
        "kind",
        "name",
        "enabled",
        "config_json",
        "secret_ref",
        "created_at",
        "updated_at",
    ] {
        assert!(
            cols.iter().any(|col| col == required),
            "notification_channels must have column {required} (got {cols:?})"
        );
    }

    sqlx::query(
        r#"
        INSERT INTO notification_channels
            (id, kind, name, enabled, config_json, secret_ref, created_at, updated_at)
        VALUES
            ('bad-kind', 'sms', 'SMS', 1, '{}', NULL, 0, 0)
        "#,
    )
    .execute(&pool)
    .await
    .expect_err("kind must be restricted to desktop/webhook/email/telegram");

    fs::remove_dir_all(dir).ok();
}
