//! HOST-008 — deploy SSH connection must verify host keys through KnownHostsStore.

use serverhub_lib::deployer::known_hosts::KnownHostsStore;
use serverhub_lib::deployer::ssh_client::HostKeyVerifier;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

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

#[tokio::test]
async fn deploy_emits_fingerprint_in_progress_detail() {
    let store = Arc::new(test_store().await);
    let verifier = HostKeyVerifier::new("203.0.113.10".to_string(), 22, store.clone());
    let public_key = russh_keys::parse_public_key_base64(
        "AAAAC3NzaC1lZDI1NTE5AAAAILagOJFgwaMNhBWQINinKOXmqS4Gh5NgxgriXwdOoINJ",
    )
    .expect("fixture public key");

    let detail = verifier
        .verify_public_key(&public_key)
        .await
        .expect("first host key should pass TOFU");

    assert_eq!(detail.algorithm, "ssh-ed25519");
    assert!(
        detail.progress_detail().contains(&detail.fingerprint),
        "deploy progress detail must include fingerprint"
    );

    let stored = store
        .find("203.0.113.10", 22)
        .await
        .expect("known_hosts lookup should succeed");
    assert!(
        stored.is_some(),
        "HostKeyVerifier must record first-seen host keys through KnownHostsStore"
    );
}
