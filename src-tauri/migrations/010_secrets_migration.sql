-- Secret references for credentials migrated out of plaintext server columns.
CREATE TABLE IF NOT EXISTS secret_refs (
    server_id TEXT NOT NULL,
    field TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (server_id, field),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_secret_refs_server
    ON secret_refs(server_id);
