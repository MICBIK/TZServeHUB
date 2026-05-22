pub mod desktop;

use crate::error::AppResult;
use crate::models::alert::AlertEvent;

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait NotificationChannel: Send + Sync {
    fn name(&self) -> &str;
    async fn send_alert(&self, event: &AlertEvent) -> AppResult<()>;
    async fn send_recovery(&self, event: &AlertEvent) -> AppResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::error::AppResult;
    use crate::models::alert::{AlertEvent, AlertStatus};

    use super::NotificationChannel;

    struct RecordingChannel;

    #[async_trait::async_trait]
    impl NotificationChannel for RecordingChannel {
        fn name(&self) -> &str {
            "recording"
        }

        async fn send_alert(&self, _event: &AlertEvent) -> AppResult<()> {
            Ok(())
        }

        async fn send_recovery(&self, _event: &AlertEvent) -> AppResult<()> {
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

    fn assert_send_sync<T: Send + Sync>(_: &T) {}

    #[tokio::test]
    async fn trait_provides_name_send_alert_send_recovery() {
        let channel = RecordingChannel;
        assert_send_sync(&channel);

        assert_eq!(channel.name(), "recording");
        channel
            .send_alert(&sample_event(AlertStatus::Firing))
            .await
            .expect("send_alert should use AppResult");
        channel
            .send_recovery(&sample_event(AlertStatus::Resolved))
            .await
            .expect("send_recovery should use AppResult");
    }
}
