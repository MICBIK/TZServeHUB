//! KEY-009 — removing a server clears associated SecretStore entries.

use serverhub_lib::storage::secrets::{encrypted_file::EncryptedFileStore, SecretStore};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;
use std::path::PathBuf;

fn unique_data_dir(spec: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("serverhub-{spec}-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("test data dir should be created");
    dir
}

async fn migrated_test_pool(spec: &str) -> SqlitePool {
    let data_dir = unique_data_dir(spec);
    let db_path = data_dir.join("data.db");
    let connection_string = format!("sqlite://{}?mode=rwc", db_path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&connection_string)
        .await
        .expect("test sqlite pool should connect");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("project migrations should run");

    pool
}

async fn seed_server_with_secret_refs(pool: &SqlitePool, server_id: &str) {
    sqlx::query(
        r#"
        INSERT INTO servers (
            id, name, host, port, adapter_type, access_method,
            polling_interval_sec, enabled, auth_type, created_at, updated_at
        )
        VALUES (?, 'demo', '203.0.113.10', 9100, 'go_agent', 'private', 30, 1, 'token', 0, 0)
        "#,
    )
    .bind(server_id)
    .execute(pool)
    .await
    .expect("server fixture should insert");

    for field in ["auth_token", "ssh_passphrase", "password"] {
        let key = format!("serverhub.server.{server_id}.{field}");
        sqlx::query(
            r#"
            INSERT INTO secret_refs (server_id, field, secret_key, created_at, updated_at)
            VALUES (?, ?, ?, 0, 0)
            "#,
        )
        .bind(server_id)
        .bind(field)
        .bind(key)
        .execute(pool)
        .await
        .expect("secret_ref fixture should insert");
    }
}

#[tokio::test]
async fn remove_server_deletes_all_associated_secret_entries() {
    let pool = migrated_test_pool("key-009-remove").await;
    seed_server_with_secret_refs(&pool, "srv-remove").await;
    let store = EncryptedFileStore::new(unique_data_dir("key-009-store"));

    for field in ["auth_token", "ssh_passphrase", "password"] {
        let key = format!("serverhub.server.srv-remove.{field}");
        store
            .put(&key, &format!("secret-{field}"))
            .await
            .expect("fixture secret should be stored");
    }

    serverhub_lib::storage::secrets::remove_server_secrets(&pool, &store, "srv-remove")
        .await
        .expect("server secret cleanup should succeed");

    for field in ["auth_token", "ssh_passphrase", "password"] {
        let key = format!("serverhub.server.srv-remove.{field}");
        let value = store
            .get(&key)
            .await
            .expect("secret lookup should succeed after cleanup");
        assert_eq!(value, None, "{field} should be deleted from SecretStore");
    }

    let remaining_refs: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM secret_refs WHERE server_id = 'srv-remove'")
            .fetch_one(&pool)
            .await
            .expect("secret_refs count should query");
    assert_eq!(remaining_refs, 0);
}
