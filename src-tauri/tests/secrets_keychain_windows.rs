#![cfg(target_os = "windows")]

use serverhub_lib::storage::secrets::{OsKeychainStore, SecretStore};

fn unique_key(field: &str) -> String {
    format!("serverhub.test.{}.{}", uuid::Uuid::new_v4(), field)
}

/// KEY-004: Windows Credential Manager put/get/delete roundtrip works.
#[tokio::test]
async fn keychain_put_get_delete_roundtrip_windows() {
    let store = OsKeychainStore::new("serverhub-test");
    let key = unique_key("token");

    store
        .put(&key, "secret-value")
        .await
        .expect("Windows credential manager put should succeed");
    let value = store
        .get(&key)
        .await
        .expect("Windows credential manager get should succeed");
    assert_eq!(value.as_deref(), Some("secret-value"));

    store
        .delete(&key)
        .await
        .expect("Windows credential manager delete should succeed");
    let value = store
        .get(&key)
        .await
        .expect("Windows credential manager get after delete should succeed");
    assert_eq!(value, None);
}
