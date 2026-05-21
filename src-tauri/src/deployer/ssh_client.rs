use super::known_hosts::{fingerprint_public_key, KnownHostsStore};
use crate::error::AppResult;
use russh::client;
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

    pub async fn verify_public_key(
        &self,
        server_public_key: &PublicKey,
    ) -> AppResult<HostKeyDetail> {
        let detail = HostKeyDetail {
            algorithm: server_public_key.name().to_string(),
            fingerprint: fingerprint_public_key(&server_public_key.public_key_bytes()),
        };

        self.store
            .verify_or_insert(&self.host, self.port, &detail.fingerprint, &detail.algorithm)
            .await?;

        Ok(detail)
    }
}

pub struct ClientHandler {
    verifier: HostKeyVerifier,
    last_host_key: Option<HostKeyDetail>,
}

impl ClientHandler {
    pub fn new(verifier: HostKeyVerifier) -> Self {
        Self {
            verifier,
            last_host_key: None,
        }
    }

    pub fn last_host_key_detail(&self) -> Option<&HostKeyDetail> {
        self.last_host_key.as_ref()
    }
}

#[async_trait::async_trait]
impl client::Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        match self.verifier.verify_public_key(server_public_key).await {
            Ok(detail) => {
                self.last_host_key = Some(detail);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
}
