use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    Token,
    Password,
    SshKey,
    None,
}

impl AuthType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Token => "token",
            Self::Password => "password",
            Self::SshKey => "ssh_key",
            Self::None => "none",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "token" => Self::Token,
            "password" => Self::Password,
            "ssh_key" => Self::SshKey,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub adapter_type: AdapterType,
    pub access_method: AccessMethod,
    pub polling_interval_sec: u32,
    pub enabled: bool,
    pub auth_token: Option<String>,
    pub auth_type: AuthType,
    pub ssh_key_path: Option<String>,
    pub ssh_passphrase: Option<String>,
    pub password: Option<String>,
    pub status: String,
    pub last_seen_at: Option<i64>,
    pub last_error: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterType {
    NodeExporter,
    Glances,
    GoAgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMethod {
    Private,
    Tunnel,
    Gateway,
}

impl AdapterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NodeExporter => "node_exporter",
            Self::Glances => "glances",
            Self::GoAgent => "go_agent",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "go_agent" => Self::GoAgent,
            "glances" => Self::Glances,
            _ => Self::NodeExporter,
        }
    }
}

impl AccessMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Tunnel => "tunnel",
            Self::Gateway => "gateway",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "tunnel" => Self::Tunnel,
            "gateway" => Self::Gateway,
            _ => Self::Private,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppResult;
    use crate::storage::secrets::SecretStore;
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct MemorySecretStore {
        entries: HashMap<String, String>,
    }

    impl MemorySecretStore {
        fn with_entry(key: &str, value: &str) -> Self {
            let mut entries = HashMap::new();
            entries.insert(key.to_string(), value.to_string());
            Self { entries }
        }
    }

    #[async_trait]
    impl SecretStore for MemorySecretStore {
        async fn put(&self, _key: &str, _value: &str) -> AppResult<()> {
            Ok(())
        }

        async fn get(&self, key: &str) -> AppResult<Option<String>> {
            Ok(self.entries.get(key).cloned())
        }

        async fn delete(&self, _key: &str) -> AppResult<()> {
            Ok(())
        }
    }

    fn server_with_legacy_plaintext() -> ServerConfig {
        ServerConfig {
            id: "srv-1".to_string(),
            name: "demo".to_string(),
            host: "203.0.113.10".to_string(),
            port: 9100,
            adapter_type: AdapterType::GoAgent,
            access_method: AccessMethod::Private,
            polling_interval_sec: 30,
            enabled: true,
            auth_token: Some("legacy-plaintext-token".to_string()),
            auth_type: AuthType::Token,
            ssh_key_path: None,
            ssh_passphrase: Some("legacy-passphrase".to_string()),
            password: Some("legacy-password".to_string()),
            status: "unknown".to_string(),
            last_seen_at: None,
            last_error: None,
            created_at: 0,
            updated_at: 0,
        }
    }

    /// KEY-008: credential getters must read from SecretStore rather than
    /// returning legacy SQLite plaintext fields.
    #[tokio::test]
    async fn server_get_auth_token_reads_from_secret_store() {
        let server = server_with_legacy_plaintext();
        let store = MemorySecretStore::with_entry(
            "serverhub.server.srv-1.auth_token",
            "secret-store-token",
        );

        let token = server
            .get_auth_token(&store)
            .await
            .expect("auth token getter should read from SecretStore");

        assert_eq!(token.as_deref(), Some("secret-store-token"));
        assert_ne!(token.as_deref(), Some("legacy-plaintext-token"));
    }
}
