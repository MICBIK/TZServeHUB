#![cfg(target_os = "macos")]

use serverhub_lib::storage::secrets::{OsKeychainStore, SecretStore};

fn unique_key(field: &str) -> String {
    format!("serverhub.test.{}.{}", uuid::Uuid::new_v4(), field)
}

/// KEY-002: macOS Keychain put/get/delete roundtrip works.
#[tokio::test]
async fn keychain_put_get_delete_roundtrip_macos() {
    let store = OsKeychainStore::new("serverhub-test");
    let key = unique_key("token");

    store
        .put(&key, "secret-value")
        .await
        .expect("macOS keychain put should succeed");
    let value = store
        .get(&key)
        .await
        .expect("macOS keychain get should succeed");
    assert_eq!(value.as_deref(), Some("secret-value"));

    store
        .delete(&key)
        .await
        .expect("macOS keychain delete should succeed");
    let value = store
        .get(&key)
        .await
        .expect("macOS keychain get after delete should succeed");
    assert_eq!(value, None);
}
