ALTER TABLE metrics_1m ADD COLUMN labels TEXT NOT NULL DEFAULT '{}';
ALTER TABLE metrics_1m ADD COLUMN vantage_point TEXT NOT NULL DEFAULT 'desktop';

DROP INDEX IF EXISTS idx_metrics_1m_unique;
CREATE UNIQUE INDEX idx_metrics_1m_unique
    ON metrics_1m(server_id, key, labels, vantage_point, bucket);

ALTER TABLE metrics_15m ADD COLUMN labels TEXT NOT NULL DEFAULT '{}';
ALTER TABLE metrics_15m ADD COLUMN vantage_point TEXT NOT NULL DEFAULT 'desktop';

DROP INDEX IF EXISTS idx_metrics_15m_unique;
CREATE UNIQUE INDEX idx_metrics_15m_unique
    ON metrics_15m(server_id, key, labels, vantage_point, bucket);
