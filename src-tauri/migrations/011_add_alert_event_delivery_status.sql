-- Per-channel delivery result JSON for dispatched alert events.
ALTER TABLE alert_events ADD COLUMN delivery_status TEXT;
