use crate::error::AppResult;
use crate::models::alert::AlertEvent;
use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

pub struct AlertNotifier<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> Clone for AlertNotifier<R> {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
        }
    }
}

impl<R: Runtime> AlertNotifier<R> {
    pub fn new(app: AppHandle<R>) -> Self {
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
