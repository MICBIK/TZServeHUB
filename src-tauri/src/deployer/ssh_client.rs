use super::known_hosts::{fingerprint_public_key, KnownHostsStore};
use crate::error::AppResult;
use russh::keys::key::PublicKey;
use russh_keys::PublicKeyBase64;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostKeyDetail {
    pub algorithm: String,
    pub fingerprint: String,
}

impl HostKeyDetail {
    pub fn progress_detail(&self) -> String {
        format!("{} {}", self.algorithm, self.fingerprint)
    }
}

pub struct HostKeyVerifier {
    host: String,
    port: u16,
    store: Arc<KnownHostsStore>,
}

impl HostKeyVerifier {
    pub fn new(host: String, port: u16, store: Arc<KnownHostsStore>) -> Self {
        Self { host, port, store }
    }

    pub async fn verify_public_key(&self, server_public_key: &PublicKey) -> AppResult<HostKeyDetail> {
        let detail = HostKeyDetail {
            algorithm: server_public_key.name().to_string(),
            fingerprint: fingerprint_public_key(&server_public_key.public_key_bytes()),
        };

        let _ = (&self.host, self.port, &self.store);
        Ok(detail)
    }
}
