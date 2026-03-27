use crate::error::{AppError, AppResult};
use crate::models::server::{AccessMethod, AdapterType, AuthType, ServerConfig};
use serde::Serialize;
use sqlx::{Row, SqlitePool};
use tauri::State;

fn row_to_server(row: &sqlx::sqlite::SqliteRow) -> ServerConfig {
    ServerConfig {
        id: row.get("id"),
        name: row.get("name"),
        host: row.get("host"),
        port: row.get::<i64, _>("port") as u16,
        adapter_type: AdapterType::from_db(&row.get::<String, _>("adapter_type")),
        access_method: AccessMethod::from_db(&row.get::<String, _>("access_method")),
        polling_interval_sec: row.get::<i64, _>("polling_interval_sec") as u32,
        enabled: row.get::<i64, _>("enabled") != 0,
        auth_token: row.get("auth_token"),
        auth_type: AuthType::from_db(&row.get::<String, _>("auth_type")),
        ssh_key_path: row.get("ssh_key_path"),
        ssh_passphrase: row.get("ssh_passphrase"),
        password: row.get("password"),
        status: row.get("status"),
        last_seen_at: row.get("last_seen_at"),
        last_error: row.get("last_error"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[tauri::command]
pub async fn list_servers(pool: State<'_, SqlitePool>) -> Result<Vec<ServerConfig>, String> {
    let result: AppResult<Vec<ServerConfig>> = async {
        let rows = sqlx::query(
            "SELECT id, name, host, port, adapter_type, access_method, polling_interval_sec, enabled, auth_token, auth_type, ssh_key_path, ssh_passphrase, password, status, last_seen_at, last_error, created_at, updated_at FROM servers ORDER BY created_at DESC",
        )
        .fetch_all(pool.inner())
        .await?;

        Ok(rows.iter().map(row_to_server).collect())
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_server(
    pool: State<'_, SqlitePool>,
    name: String,
    host: String,
    port: u16,
    adapter_type: Option<AdapterType>,
    access_method: Option<AccessMethod>,
    polling_interval_sec: Option<u32>,
    auth_token: Option<String>,
    auth_type: Option<AuthType>,
    ssh_key_path: Option<String>,
    ssh_passphrase: Option<String>,
    password: Option<String>,
) -> Result<ServerConfig, String> {
    let result: AppResult<ServerConfig> = async {
        if name.trim().is_empty() {
            return Err(AppError::Custom("Server name cannot be empty".to_string()));
        }
        if host.trim().is_empty() {
            return Err(AppError::Custom("Server host cannot be empty".to_string()));
        }
        if port == 0 {
            return Err(AppError::Custom("Server port must be greater than 0".to_string()));
        }

        let server = ServerConfig {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.trim().to_string(),
            host: host.trim().to_string(),
            port,
            adapter_type: adapter_type.unwrap_or(AdapterType::NodeExporter),
            access_method: access_method.unwrap_or(AccessMethod::Private),
            polling_interval_sec: polling_interval_sec.unwrap_or(10),
            enabled: true,
            auth_token: auth_token.filter(|token| !token.trim().is_empty()),
            auth_type: auth_type.unwrap_or(AuthType::Token),
            ssh_key_path: ssh_key_path.filter(|s| !s.trim().is_empty()),
            ssh_passphrase: ssh_passphrase.filter(|s| !s.trim().is_empty()),
            password: password.filter(|s| !s.trim().is_empty()),
            status: "unknown".to_string(),
            last_seen_at: None,
            last_error: None,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        sqlx::query(
            "INSERT INTO servers (id, name, host, port, adapter_type, access_method, polling_interval_sec, enabled, auth_token, auth_type, ssh_key_path, ssh_passphrase, password, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&server.id)
        .bind(&server.name)
        .bind(&server.host)
        .bind(server.port as i64)
        .bind(server.adapter_type.as_str())
        .bind(server.access_method.as_str())
        .bind(server.polling_interval_sec as i64)
        .bind(1_i64)
        .bind(&server.auth_token)
        .bind(server.auth_type.as_str())
        .bind(&server.ssh_key_path)
        .bind(&server.ssh_passphrase)
        .bind(&server.password)
        .bind(server.created_at)
        .bind(server.updated_at)
        .execute(pool.inner())
        .await?;

        Ok(server)
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_server(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    let result: AppResult<()> = async {
        if id.trim().is_empty() {
            return Err(AppError::Custom("Server ID cannot be empty".to_string()));
        }

        sqlx::query("DELETE FROM servers WHERE id = ?")
            .bind(id)
            .execute(pool.inner())
            .await?;

        Ok(())
    }
    .await;

    result.map_err(|e| e.to_string())
}

#[derive(Debug, Serialize)]
pub struct HealthSummary {
    pub online: u32,
    pub offline: u32,
    pub error: u32,
    pub unknown: u32,
}

#[tauri::command]
pub async fn get_server_health_summary(
    pool: State<'_, SqlitePool>,
) -> Result<HealthSummary, String> {
    let result: AppResult<HealthSummary> = async {
        let rows = sqlx::query(
            "SELECT status, last_seen_at, polling_interval_sec FROM servers WHERE enabled = 1",
        )
        .fetch_all(pool.inner())
        .await?;

        let now = chrono::Utc::now().timestamp();
        let mut summary = HealthSummary {
            online: 0,
            offline: 0,
            error: 0,
            unknown: 0,
        };

        for row in &rows {
            let status: String = row.get("status");
            let last_seen_at: Option<i64> = row.get("last_seen_at");
            let polling_interval_sec = row.get::<i64, _>("polling_interval_sec") as i64;

            match status.as_str() {
                "online" => {
                    if let Some(seen) = last_seen_at {
                        if now - seen > polling_interval_sec * 3 {
                            summary.offline += 1;
                        } else {
                            summary.online += 1;
                        }
                    } else {
                        summary.online += 1;
                    }
                }
                "error" => summary.error += 1,
                _ => summary.unknown += 1,
            }
        }

        Ok(summary)
    }
    .await;

    result.map_err(|e| e.to_string())
}
