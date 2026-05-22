use serverhub_lib::alerts::channels::desktop::DesktopNotificationPayload;
use serverhub_lib::models::alert::{AlertEvent, AlertStatus};

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

#[test]
fn desktop_notification_behavior_matches_v0_1() {
    let alert = DesktopNotificationPayload::alert(&sample_event(AlertStatus::Firing));
    assert_eq!(alert.title, "ServerHUB Alert");
    assert_eq!(alert.body, "CPU high");

    let recovery = DesktopNotificationPayload::recovery(&sample_event(AlertStatus::Resolved));
    assert_eq!(recovery.title, "ServerHUB Recovery");
    assert_eq!(recovery.body, "CPU high");
}
