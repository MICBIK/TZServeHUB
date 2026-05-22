use super::NotificationChannel;
use crate::error::{AppError, AppResult};
use crate::models::alert::AlertEvent;
use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNotificationPayload {
    pub title: String,
    pub body: String,
}

impl DesktopNotificationPayload {
    pub fn alert(event: &AlertEvent) -> Self {
        Self {
            title: "ServerHUB Alert".to_string(),
            body: event.message.clone(),
        }
    }

    pub fn recovery(event: &AlertEvent) -> Self {
        Self {
            title: "ServerHUB Recovery".to_string(),
            body: event.message.clone(),
        }
    }
}

#[async_trait::async_trait]
pub trait DesktopNotificationDispatcher: Send + Sync {
    async fn show(&self, payload: DesktopNotificationPayload) -> AppResult<()>;
}

pub struct TauriDesktopNotificationDispatcher<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> TauriDesktopNotificationDispatcher<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl<R: Runtime> DesktopNotificationDispatcher for TauriDesktopNotificationDispatcher<R> {
    async fn show(&self, payload: DesktopNotificationPayload) -> AppResult<()> {
        self.app
            .notification()
            .builder()
            .title(&payload.title)
            .body(&payload.body)
            .show()
            .map_err(|e| AppError::Notification(e.to_string()))?;
        Ok(())
    }
}

pub struct DesktopChannel<D> {
    dispatcher: D,
}

impl<D> DesktopChannel<D> {
    pub fn new(dispatcher: D) -> Self {
        Self { dispatcher }
    }
}

impl<R: Runtime> DesktopChannel<TauriDesktopNotificationDispatcher<R>> {
    pub fn from_app(app: AppHandle<R>) -> Self {
        Self::new(TauriDesktopNotificationDispatcher::new(app))
    }
}

#[async_trait::async_trait]
impl<D> NotificationChannel for DesktopChannel<D>
where
    D: DesktopNotificationDispatcher,
{
    fn name(&self) -> &str {
        "desktop"
    }

    async fn send_alert(&self, event: &AlertEvent) -> AppResult<()> {
        self.dispatcher
            .show(DesktopNotificationPayload::alert(event))
            .await
    }

    async fn send_recovery(&self, event: &AlertEvent) -> AppResult<()> {
        self.dispatcher
            .show(DesktopNotificationPayload::recovery(event))
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::error::AppResult;
    use crate::models::alert::{AlertEvent, AlertStatus};

    use super::{
        DesktopChannel, DesktopNotificationDispatcher, DesktopNotificationPayload,
        NotificationChannel,
    };

    #[derive(Clone, Default)]
    struct RecordingDispatcher {
        payloads: Arc<Mutex<Vec<DesktopNotificationPayload>>>,
    }

    #[async_trait::async_trait]
    impl DesktopNotificationDispatcher for RecordingDispatcher {
        async fn show(&self, payload: DesktopNotificationPayload) -> AppResult<()> {
            self.payloads.lock().expect("payload lock").push(payload);
            Ok(())
        }
    }

    fn sample_event(status: AlertStatus) -> AlertEvent {
        AlertEvent {
            id: "evt-1".to_string(),
            rule_id: "rule-1".to_string(),
            server_id: "srv-1".to_string(),
            status,
            message: "CPU high".to_string(),
            fired_at: 1_700_000_000,
            resolved_at: None,
            delivery_status: None,
        }
    }

    #[tokio::test]
    async fn desktop_channel_sends_via_tauri_plugin() {
        let dispatcher = RecordingDispatcher::default();
        let channel = DesktopChannel::new(dispatcher.clone());

        assert_eq!(channel.name(), "desktop");
        channel
            .send_alert(&sample_event(AlertStatus::Firing))
            .await
            .expect("desktop alert notification should send");

        let payloads = dispatcher.payloads.lock().expect("payload lock");
        assert_eq!(payloads.len(), 1);
        assert_eq!(payloads[0].title, "ServerHUB Alert");
        assert_eq!(payloads[0].body, "CPU high");
    }
}
