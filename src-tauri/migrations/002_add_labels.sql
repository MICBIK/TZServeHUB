-- Add labels column to raw_metrics (stores JSON)
ALTER TABLE raw_metrics ADD COLUMN labels TEXT;

-- Update index to include labels for series identity
DROP INDEX IF EXISTS idx_raw_metrics_key_time;
CREATE INDEX idx_raw_metrics_series_time ON raw_metrics(server_id, key, labels, timestamp DESC);