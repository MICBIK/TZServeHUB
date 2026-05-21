//! Encrypted file fallback secret storage.

use super::SecretStore;
use crate::error::AppResult;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

pub struct EncryptedFileStore {
    path: PathBuf,
}

impl EncryptedFileStore {
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            path: data_dir.as_ref().join("secrets.enc"),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[async_trait]
impl SecretStore for EncryptedFileStore {
    async fn put(&self, _key: &str, _value: &str) -> AppResult<()> {
        todo!("KEY-006: implement AES-GCM-SIV encrypted write")
    }

    async fn get(&self, _key: &str) -> AppResult<Option<String>> {
        todo!("KEY-006: implement encrypted read")
    }

    async fn delete(&self, _key: &str) -> AppResult<()> {
        todo!("KEY-006: implement encrypted delete")
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
    async fn encrypted_file_store_roundtrip_with_aes_gcm() {
        let dir = unique_data_dir("key-006-roundtrip");
        let store = EncryptedFileStore::new(&dir);

        assert_eq!(store.path(), dir.join("secrets.enc"));
        store
            .put("serverhub.test.001.token", "secret-value")
            .await
            .expect("put should write encrypted file");

        let encrypted = fs::read_to_string(store.path()).expect("secrets.enc should exist");
        assert!(
            !encrypted.contains("secret-value"),
            "encrypted fallback file must not contain plaintext secret"
        );

        let value = store
            .get("serverhub.test.001.token")
            .await
            .expect("get should read encrypted file");
        assert_eq!(value.as_deref(), Some("secret-value"));

        store
            .delete("serverhub.test.001.token")
            .await
            .expect("delete should update encrypted file");
        let value = store
            .get("serverhub.test.001.token")
            .await
            .expect("deleted value should be absent");
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn encrypted_file_write_is_atomic() {
        let dir = unique_data_dir("key-006-atomic");
        let store = EncryptedFileStore::new(&dir);

        store
            .put("serverhub.test.001.token", "first-value")
            .await
            .expect("initial put should succeed");
        store
            .put("serverhub.test.001.token", "second-value")
            .await
            .expect("replacement put should succeed");

        let temp_files: Vec<_> = fs::read_dir(&dir)
            .expect("data dir should be readable")
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().contains(".tmp"))
            .collect();
        assert!(
            temp_files.is_empty(),
            "atomic writes should rename temp files into place"
        );

        let value = store
            .get("serverhub.test.001.token")
            .await
            .expect("get should read latest value");
        assert_eq!(value.as_deref(), Some("second-value"));
    }
}
