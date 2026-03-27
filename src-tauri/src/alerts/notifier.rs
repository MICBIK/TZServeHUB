#![allow(dead_code)]

use crate::error::AppResult;
use crate::models::alert::AlertEvent;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub struct AlertNotifier {
    app: AppHandle,
}

impl AlertNotifier {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn send_alert(&self, event: &AlertEvent) -> AppResult<()> {
        self.app
            .notification()
            .builder()
            .title("ServerHUB Alert")
            .body(&event.message)
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;
        Ok(())
    }

    pub fn send_recovery(&self, event: &AlertEvent) -> AppResult<()> {
        self.app
            .notification()
            .builder()
            .title("ServerHUB Recovery")
            .body(&event.message)
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;
        Ok(())
    }
}
