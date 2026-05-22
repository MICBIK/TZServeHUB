#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::error::AppResult;
    use crate::models::alert::{AlertEvent, AlertStatus};

    use super::NotificationChannel;

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
