ALTER TABLE servers ADD COLUMN status TEXT NOT NULL DEFAULT 'unknown';
ALTER TABLE servers ADD COLUMN last_seen_at INTEGER;
ALTER TABLE servers ADD COLUMN last_error TEXT;
