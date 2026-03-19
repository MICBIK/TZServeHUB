-- Servers table
CREATE TABLE IF NOT EXISTS servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    adapter_type TEXT NOT NULL,
    access_method TEXT NOT NULL,
    polling_interval_sec INTEGER NOT NULL DEFAULT 30,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Raw metrics table (7 days retention)
CREATE TABLE IF NOT EXISTS raw_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value REAL NOT NULL,
    metric_type TEXT NOT NULL,
    vantage_point TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_raw_metrics_server_time
    ON raw_metrics(server_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_raw_metrics_key_time
    ON raw_metrics(key, timestamp DESC);

-- Aggregated metrics (1-minute rollup, 30 days retention)
CREATE TABLE IF NOT EXISTS metrics_1m (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL,
    key TEXT NOT NULL,
    min_val REAL NOT NULL,
    max_val REAL NOT NULL,
    avg_val REAL NOT NULL,
    bucket INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_metrics_1m_server_bucket
    ON metrics_1m(server_id, bucket DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_metrics_1m_unique
    ON metrics_1m(server_id, key, bucket);

-- Aggregated metrics (15-minute rollup, 90 days retention)
CREATE TABLE IF NOT EXISTS metrics_15m (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL,
    key TEXT NOT NULL,
    min_val REAL NOT NULL,
    max_val REAL NOT NULL,
    avg_val REAL NOT NULL,
    bucket INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_metrics_15m_server_bucket
    ON metrics_15m(server_id, bucket DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_metrics_15m_unique
    ON metrics_15m(server_id, key, bucket);

-- Alert rules
CREATE TABLE IF NOT EXISTS alert_rules (
    id TEXT PRIMARY KEY,
    server_id TEXT NOT NULL,
    name TEXT NOT NULL,
    metric_key TEXT NOT NULL,
    condition TEXT NOT NULL,
    threshold REAL NOT NULL,
    duration_sec INTEGER NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

-- Alert events
CREATE TABLE IF NOT EXISTS alert_events (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    server_id TEXT NOT NULL,
    status TEXT NOT NULL,
    message TEXT NOT NULL,
    fired_at INTEGER NOT NULL,
    resolved_at INTEGER,
    FOREIGN KEY (rule_id) REFERENCES alert_rules(id) ON DELETE CASCADE,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_alert_events_server_time
    ON alert_events(server_id, fired_at DESC);
