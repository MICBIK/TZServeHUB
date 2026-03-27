-- Probe results table for storing periodic probe outcomes
CREATE TABLE IF NOT EXISTS probe_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id TEXT NOT NULL,
    probe_type TEXT NOT NULL,
    target TEXT NOT NULL,
    success INTEGER NOT NULL,
    latency_ms REAL,
    loss_rate REAL,
    error_message TEXT,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_probe_results_server_time
    ON probe_results(server_id, probe_type, timestamp DESC);
