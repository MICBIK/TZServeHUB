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
