-- Notification channel definitions. Sensitive values live in SecretStore and are referenced by secret_ref.
CREATE TABLE IF NOT EXISTS notification_channels (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL CHECK (kind IN ('desktop', 'webhook', 'email', 'telegram')),
    name TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    config_json TEXT NOT NULL DEFAULT '{}',
    secret_ref TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_notification_channels_enabled_kind
    ON notification_channels(enabled, kind);

CREATE INDEX IF NOT EXISTS idx_notification_channels_updated_at
    ON notification_channels(updated_at DESC);
