-- SSH known_hosts table for TOFU host key verification.
CREATE TABLE IF NOT EXISTS known_hosts (
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    fingerprint TEXT NOT NULL,
    algorithm TEXT NOT NULL,
    first_seen INTEGER NOT NULL,
    last_seen INTEGER NOT NULL,
    PRIMARY KEY (host, port)
);

CREATE INDEX IF NOT EXISTS idx_known_hosts_last_seen
    ON known_hosts(last_seen DESC);
