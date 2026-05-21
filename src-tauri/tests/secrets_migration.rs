//! KEY-007 — migrate plaintext server credentials into SecretStore.

use serverhub_lib::storage::secrets::{encrypted_file::EncryptedFileStore, SecretStore};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
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

async fn seed_plaintext_server(pool: &SqlitePool, id: &str) {
    sqlx::query(
        r#"
        INSERT INTO servers (
            id, name, host, port, adapter_type, access_method,
            polling_interval_sec, enabled, auth_token, auth_type,
            ssh_key_path, ssh_passphrase, password, created_at, updated_at
        )
        VALUES (
            ?, 'demo', '203.0.113.10', 9100, 'go_agent', 'private',
            30, 1, 'plain-token', 'password',
            NULL, 'plain-passphrase', 'plain-password', 0, 0
        )
        "#,
    )
    .bind(id)
    .execute(pool)
    .await
    .expect("plaintext server fixture should insert");
}

#[tokio::test]
async fn migration_moves_plaintext_to_keychain() {
    let pool = migrated_test_pool("key-007-move").await;
    seed_plaintext_server(&pool, "test-1").await;
    let store = EncryptedFileStore::new(unique_data_dir("key-007-store"));

    serverhub_lib::storage::secrets::migrate_legacy_plaintext_to_keychain(&pool, &store)
        .await
        .expect("migration should move plaintext credentials");

    assert_eq!(
        store
            .get("serverhub.server.test-1.auth_token")
            .await
            .expect("auth_token lookup should succeed")
            .as_deref(),
        Some("plain-token")
    );
    assert_eq!(
        store
            .get("serverhub.server.test-1.ssh_passphrase")
            .await
            .expect("ssh_passphrase lookup should succeed")
            .as_deref(),
        Some("plain-passphrase")
    );
    assert_eq!(
        store
            .get("serverhub.server.test-1.password")
            .await
            .expect("password lookup should succeed")
            .as_deref(),
        Some("plain-password")
    );

    let row = sqlx::query(
        "SELECT auth_token, ssh_passphrase, password FROM servers WHERE id = 'test-1'",
    )
    .fetch_one(&pool)
    .await
    .expect("migrated server row should exist");
    assert_eq!(row.get::<Option<String>, _>("auth_token"), None);
    assert_eq!(row.get::<Option<String>, _>("ssh_passphrase"), None);
    assert_eq!(row.get::<Option<String>, _>("password"), None);

    let secret_refs: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM secret_refs WHERE server_id = 'test-1'")
            .fetch_one(&pool)
            .await
            .expect("secret_refs should be queryable");
    assert_eq!(secret_refs, 3);
}

#[tokio::test]
async fn migration_is_idempotent() {
    let pool = migrated_test_pool("key-007-idempotent").await;
    seed_plaintext_server(&pool, "test-2").await;
    let store = EncryptedFileStore::new(unique_data_dir("key-007-idempotent-store"));

    serverhub_lib::storage::secrets::migrate_legacy_plaintext_to_keychain(&pool, &store)
        .await
        .expect("first migration should succeed");
    serverhub_lib::storage::secrets::migrate_legacy_plaintext_to_keychain(&pool, &store)
        .await
        .expect("second migration should be a no-op");

    let secret_refs: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM secret_refs WHERE server_id = 'test-2'")
            .fetch_one(&pool)
            .await
            .expect("secret_refs should be queryable");
    assert_eq!(secret_refs, 3);
}
