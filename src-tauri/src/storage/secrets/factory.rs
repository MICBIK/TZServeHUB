//! Secret store factory with OS keychain fallback behavior.

use super::SecretStore;
use crate::error::AppResult;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretStoreBackend {
    OsKeychain,
    EncryptedFile,
}

pub struct SecretStoreFactoryConfig {
    pub service: String,
    pub data_dir: PathBuf,
    pub force_keychain_unavailable: bool,
}

impl SecretStoreFactoryConfig {
    pub fn new(service: impl Into<String>, data_dir: impl Into<PathBuf>) -> Self {
        Self {
            service: service.into(),
            data_dir: data_dir.into(),
            force_keychain_unavailable: false,
        }
    }

    #[cfg(test)]
    fn with_unavailable_keychain(mut self) -> Self {
        self.force_keychain_unavailable = true;
        self
    }
}

pub struct CreatedSecretStore {
    store: Arc<dyn SecretStore>,
    backend: SecretStoreBackend,
    warning: Option<String>,
}

impl CreatedSecretStore {
    pub fn store(&self) -> Arc<dyn SecretStore> {
        Arc::clone(&self.store)
    }

    pub fn backend(&self) -> SecretStoreBackend {
        self.backend
    }

    pub fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }
}

pub struct SecretStoreFactory;

impl SecretStoreFactory {
    pub async fn create(_config: SecretStoreFactoryConfig) -> AppResult<CreatedSecretStore> {
        todo!("KEY-005: implement fallback factory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_data_dir(spec: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "serverhub-{spec}-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("test data dir should be created");
        dir
    }

    #[tokio::test]
    async fn factory_falls_back_to_encrypted_file_when_keychain_unavailable() {
        let config =
            SecretStoreFactoryConfig::new("serverhub-test", unique_data_dir("key-005-fallback"))
                .with_unavailable_keychain();

        let created = SecretStoreFactory::create(config)
            .await
            .expect("factory should create encrypted fallback when keychain is unavailable");

        assert_eq!(created.backend(), SecretStoreBackend::EncryptedFile);
        assert_eq!(
            created.warning(),
            Some("OS keychain unavailable, using encrypted file fallback")
        );

        let store = created.store();
        store
            .put("serverhub.test.factory.token", "secret-value")
            .await
            .expect("fallback store should persist secrets");
        let value = store
            .get("serverhub.test.factory.token")
            .await
            .expect("fallback store should read secrets");
        assert_eq!(value.as_deref(), Some("secret-value"));
    }
}
