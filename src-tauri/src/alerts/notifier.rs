use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use crate::models::alert::AlertEvent;
use crate::error::AppResult;

pub struct AlertNotifier {
    app: AppHandle,
}

impl AlertNotifier {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn send_alert(&self, event: &AlertEvent) -> AppResult<()> {
        let message = format!(
            "Alert on server {}: {} = {}",
            event.server_id, event.metric_key, event.value
        );
        self.app
            .notification()
            .builder()
            .title("ServerHUB Alert")
            .body(&message)
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;
        Ok(())
    }

    pub fn send_recovery(&self, event: &AlertEvent) -> AppResult<()> {
        let message = format!(
            "Recovered on server {}: {}",
            event.server_id, event.metric_key
        );
        self.app
            .notification()
            .builder()
            .title("ServerHUB Recovery")
            .body(&message)
            .show()
            .map_err(|e| crate::error::AppError::Notification(e.to_string()))?;
        Ok(())
    }
}
