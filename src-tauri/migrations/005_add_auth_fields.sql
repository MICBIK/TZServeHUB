-- Add authentication fields to servers table
ALTER TABLE servers ADD COLUMN auth_type TEXT NOT NULL DEFAULT 'token';
ALTER TABLE servers ADD COLUMN ssh_key_path TEXT;
ALTER TABLE servers ADD COLUMN ssh_passphrase TEXT;
ALTER TABLE servers ADD COLUMN password TEXT;
