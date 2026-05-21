//! Encrypted file fallback secret storage.

use super::SecretStore;
use crate::error::{AppError, AppResult};
use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use async_trait::async_trait;
use pbkdf2::pbkdf2_hmac;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: u8 = 1;
const KEY_LEN: usize = 32;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const PBKDF2_ROUNDS: u32 = 120_000;

pub struct EncryptedFileStore {
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedSecretsFile {
    version: u8,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
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

    fn load_entries(&self) -> AppResult<BTreeMap<String, String>> {
        if !self.path.exists() {
            return Ok(BTreeMap::new());
        }

        let file_bytes = fs::read(&self.path)?;
        let encrypted: EncryptedSecretsFile = serde_json::from_slice(&file_bytes)?;
        if encrypted.version != VERSION {
            return Err(AppError::Custom(format!(
                "Unsupported encrypted secrets file version: {}",
                encrypted.version
            )));
        }
        if encrypted.salt.len() != SALT_LEN || encrypted.nonce.len() != NONCE_LEN {
            return Err(AppError::Custom(
                "Invalid encrypted secrets file metadata".to_string(),
            ));
        }

        let cipher = cipher_from_salt(&encrypted.salt);
        let plaintext = cipher
            .decrypt(Nonce::from_slice(&encrypted.nonce), encrypted.ciphertext.as_ref())
            .map_err(|_| AppError::Custom("Failed to decrypt encrypted secrets file".to_string()))?;
        serde_json::from_slice(&plaintext).map_err(AppError::from)
    }

    fn write_entries(&self, entries: &BTreeMap<String, String>) -> AppResult<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut salt = [0u8; SALT_LEN];
        let mut nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);

        let cipher = cipher_from_salt(&salt);
        let plaintext = serde_json::to_vec(entries)?;
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .map_err(|_| AppError::Custom("Failed to encrypt secrets".to_string()))?;

        let encrypted = EncryptedSecretsFile {
            version: VERSION,
            salt: salt.to_vec(),
            nonce: nonce.to_vec(),
            ciphertext,
        };
        let bytes = serde_json::to_vec(&encrypted)?;
        let tmp_path = self.temp_path();

        fs::write(&tmp_path, bytes)?;
        fs::rename(&tmp_path, &self.path)?;
        Ok(())
    }

    fn temp_path(&self) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        self.path
            .with_file_name(format!("secrets.enc.tmp.{}.{}", std::process::id(), suffix))
    }
}

#[async_trait]
impl SecretStore for EncryptedFileStore {
    async fn put(&self, key: &str, value: &str) -> AppResult<()> {
        let mut entries = self.load_entries()?;
        entries.insert(key.to_string(), value.to_string());
        self.write_entries(&entries)
    }

    async fn get(&self, key: &str) -> AppResult<Option<String>> {
        let entries = self.load_entries()?;
        Ok(entries.get(key).cloned())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        let mut entries = self.load_entries()?;
        entries.remove(key);
        self.write_entries(&entries)
    }
}

fn cipher_from_salt(salt: &[u8]) -> Aes256GcmSiv {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(
        os_user_identifier().as_bytes(),
        salt,
        PBKDF2_ROUNDS,
        &mut key,
    );
    Aes256GcmSiv::new_from_slice(&key).expect("AES-256-GCM-SIV key length is fixed")
}

fn os_user_identifier() -> String {
    std::env::var("SERVERHUB_SECRET_STORE_USER")
        .or_else(|_| std::env::var("USER"))
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "serverhub-local-user".to_string())
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
