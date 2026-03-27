use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_polling_interval_sec: u32,
    pub data_retention_days: u32,
    pub theme: String,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_polling_interval_sec: 10,
            data_retention_days: 7,
            theme: "dark".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}

fn get_settings_file_path() -> AppResult<PathBuf> {
    let app_data_dir = crate::storage::database::resolve_app_data_dir()?;
    std::fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir.join("settings.json"))
}

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    match get_settings_impl().await {
        Ok(settings) => Ok(settings),
        Err(e) => Err(format!("Failed to get settings: {}", e.to_user_message())),
    }
}

async fn get_settings_impl() -> AppResult<AppSettings> {
    let settings_path = get_settings_file_path()?;

    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    let content = std::fs::read_to_string(&settings_path)?;
    let settings: AppSettings = serde_json::from_str(&content)?;
    Ok(settings)
}

#[tauri::command]
pub async fn update_settings(settings: AppSettings) -> Result<AppSettings, String> {
    match update_settings_impl(settings).await {
        Ok(settings) => Ok(settings),
        Err(e) => Err(format!(
            "Failed to update settings: {}",
            e.to_user_message()
        )),
    }
}

async fn update_settings_impl(settings: AppSettings) -> AppResult<AppSettings> {
    let settings_path = get_settings_file_path()?;
    let content = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, content)?;
    Ok(settings)
}
