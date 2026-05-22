use crate::alerts::channels::{desktop::DesktopChannel, NotificationChannel};
use crate::error::AppResult;
use crate::models::alert::AlertEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Runtime};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelDeliveryStatus {
    pub channel: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationDeliveryReport {
    pub channels: Vec<ChannelDeliveryStatus>,
}

impl NotificationDeliveryReport {
    pub fn to_delivery_status_json(&self) -> AppResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Debug, Clone, Copy)]
enum DeliveryKind {
    Alert,
    Recovery,
}

impl DeliveryKind {
    fn label(self) -> &'static str {
        match self {
            DeliveryKind::Alert => "alert",
            DeliveryKind::Recovery => "recovery",
        }
    }
}

#[derive(Clone)]
pub struct AlertNotifier {
    channels: Vec<Arc<dyn NotificationChannel>>,
}

impl AlertNotifier {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Self {
        Self {
            channels: vec![Arc::new(DesktopChannel::from_app(app))],
        }
    }

    pub fn with_channels(channels: Vec<Arc<dyn NotificationChannel>>) -> Self {
        Self { channels }
    }

    pub async fn send_alert(&self, event: &AlertEvent) -> NotificationDeliveryReport {
        self.dispatch(event, DeliveryKind::Alert).await
    }

    pub async fn send_recovery(&self, event: &AlertEvent) -> NotificationDeliveryReport {
        self.dispatch(event, DeliveryKind::Recovery).await
    }

    async fn dispatch(&self, event: &AlertEvent, kind: DeliveryKind) -> NotificationDeliveryReport {
        let mut tasks = tokio::task::JoinSet::new();

        for (index, channel) in self.channels.iter().cloned().enumerate() {
            let event = event.clone();
            tasks.spawn(async move {
                let channel_name = channel.name().to_string();
                let result = match kind {
                    DeliveryKind::Alert => channel.send_alert(&event).await,
                    DeliveryKind::Recovery => channel.send_recovery(&event).await,
                };
                let status = ChannelDeliveryStatus {
                    channel: channel_name,
                    success: result.is_ok(),
                    error: result.err().map(|e| e.to_string()),
                };
                (index, status)
            });
        }

        let mut indexed_statuses = Vec::with_capacity(self.channels.len());
        while let Some(join_result) = tasks.join_next().await {
            match join_result {
                Ok((index, status)) => {
                    if !status.success {
                        log::error!(
                            "Notification channel '{}' failed during {} delivery: {}",
                            status.channel,
                            kind.label(),
                            status.error.as_deref().unwrap_or("unknown error")
                        );
                    }
                    indexed_statuses.push((index, status));
                }
                Err(e) => {
                    log::error!(
                        "Notification channel task failed during {} delivery: {e}",
                        kind.label()
                    );
                    indexed_statuses.push((
                        usize::MAX,
                        ChannelDeliveryStatus {
                            channel: "unknown".to_string(),
                            success: false,
                            error: Some(e.to_string()),
                        },
                    ));
                }
            }
        }

        indexed_statuses.sort_by_key(|(index, _)| *index);
        NotificationDeliveryReport {
            channels: indexed_statuses
                .into_iter()
                .map(|(_, status)| status)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use crate::error::{AppError, AppResult};
    use crate::models::alert::{AlertEvent, AlertStatus};

    use super::{AlertNotifier, ChannelDeliveryStatus, NotificationChannel};

    struct RecordingChannel {
        name: &'static str,
        calls: Arc<AtomicUsize>,
        should_fail: bool,
    }

    impl RecordingChannel {
        fn ok(name: &'static str, calls: Arc<AtomicUsize>) -> Self {
            Self {
                name,
                calls,
                should_fail: false,
            }
        }

        fn failing(name: &'static str, calls: Arc<AtomicUsize>) -> Self {
            Self {
                name,
                calls,
                should_fail: true,
            }
        }
    }

    #[async_trait::async_trait]
    impl NotificationChannel for RecordingChannel {
        fn name(&self) -> &str {
            self.name
        }

        async fn send_alert(&self, _event: &AlertEvent) -> AppResult<()> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.should_fail {
                return Err(AppError::Notification(format!("{} failed", self.name)));
            }
            Ok(())
        }

        async fn send_recovery(&self, _event: &AlertEvent) -> AppResult<()> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.should_fail {
                return Err(AppError::Notification(format!("{} failed", self.name)));
            }
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

    fn channel_status<'a>(
        statuses: &'a [ChannelDeliveryStatus],
        channel: &str,
    ) -> &'a ChannelDeliveryStatus {
        statuses
            .iter()
            .find(|status| status.channel == channel)
            .expect("channel status should exist")
    }

    #[tokio::test]
    async fn router_fans_out_to_all_channels() {
        let desktop_calls = Arc::new(AtomicUsize::new(0));
        let webhook_calls = Arc::new(AtomicUsize::new(0));
        let email_calls = Arc::new(AtomicUsize::new(0));
        let notifier = AlertNotifier::with_channels(vec![
            Arc::new(RecordingChannel::ok("desktop", desktop_calls.clone())),
            Arc::new(RecordingChannel::ok("webhook", webhook_calls.clone())),
            Arc::new(RecordingChannel::ok("email", email_calls.clone())),
        ]);

        let report = notifier
            .send_alert(&sample_event(AlertStatus::Firing))
            .await;

        assert_eq!(desktop_calls.load(Ordering::SeqCst), 1);
        assert_eq!(webhook_calls.load(Ordering::SeqCst), 1);
        assert_eq!(email_calls.load(Ordering::SeqCst), 1);
        assert_eq!(report.channels.len(), 3);
        assert!(report.channels.iter().all(|status| status.success));
    }

    #[tokio::test]
    async fn router_continues_when_one_channel_fails() {
        let failing_calls = Arc::new(AtomicUsize::new(0));
        let healthy_calls = Arc::new(AtomicUsize::new(0));
        let notifier = AlertNotifier::with_channels(vec![
            Arc::new(RecordingChannel::failing("webhook", failing_calls.clone())),
            Arc::new(RecordingChannel::ok("email", healthy_calls.clone())),
        ]);

        let report = notifier
            .send_alert(&sample_event(AlertStatus::Firing))
            .await;

        assert_eq!(failing_calls.load(Ordering::SeqCst), 1);
        assert_eq!(healthy_calls.load(Ordering::SeqCst), 1);
        assert_eq!(report.channels.len(), 2);
        assert!(!channel_status(&report.channels, "webhook").success);
        assert!(channel_status(&report.channels, "webhook")
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("webhook failed"));
        assert!(channel_status(&report.channels, "email").success);
    }

    #[tokio::test]
    async fn event_persistence_independent_of_channel_failures() {
        let failing_calls = Arc::new(AtomicUsize::new(0));
        let healthy_calls = Arc::new(AtomicUsize::new(0));
        let notifier = AlertNotifier::with_channels(vec![
            Arc::new(RecordingChannel::failing("webhook", failing_calls.clone())),
            Arc::new(RecordingChannel::ok("desktop", healthy_calls.clone())),
        ]);

        let mut event = sample_event(AlertStatus::Firing);
        let report = notifier.send_alert(&event).await;
        event.delivery_status = Some(
            report
                .to_delivery_status_json()
                .expect("delivery status should serialize"),
        );

        assert_eq!(failing_calls.load(Ordering::SeqCst), 1);
        assert_eq!(healthy_calls.load(Ordering::SeqCst), 1);
        let persisted_status = event
            .delivery_status
            .as_deref()
            .expect("delivery_status should be populated before persistence");
        let decoded: super::NotificationDeliveryReport =
            serde_json::from_str(persisted_status).expect("delivery_status should decode");

        assert_eq!(decoded.channels.len(), 2);
        assert!(!channel_status(&decoded.channels, "webhook").success);
        assert!(channel_status(&decoded.channels, "desktop").success);
    }
}
