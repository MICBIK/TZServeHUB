//! Secret storage abstraction.

use crate::error::{AppError, AppResult};
use async_trait::async_trait;

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
