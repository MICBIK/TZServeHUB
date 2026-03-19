use serde::{Deserialize, Serialize};

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

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    Ok(AppSettings::default())
}

#[tauri::command]
pub async fn update_settings(settings: AppSettings) -> Result<AppSettings, String> {
    // TODO: persist settings
    Ok(settings)
}
