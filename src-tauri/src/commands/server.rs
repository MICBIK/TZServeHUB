use crate::models::server::{AccessMethod, AdapterType, ServerConfig};

#[tauri::command]
pub async fn list_servers() -> Result<Vec<ServerConfig>, String> {
    // TODO: implement with storage
    Ok(vec![])
}

#[tauri::command]
pub async fn add_server(name: String, host: String, port: u16) -> Result<ServerConfig, String> {
    let server = ServerConfig {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        host,
        port,
        adapter_type: AdapterType::NodeExporter,
        access_method: AccessMethod::Private,
        polling_interval_sec: 10,
        enabled: true,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };
    // TODO: persist to storage
    Ok(server)
}

#[tauri::command]
pub async fn remove_server(id: String) -> Result<(), String> {
    // TODO: implement with storage
    let _ = id;
    Ok(())
}
