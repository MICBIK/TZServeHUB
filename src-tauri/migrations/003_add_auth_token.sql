-- Add auth_token column to servers (nullable for backward compatibility)
ALTER TABLE servers ADD COLUMN auth_token TEXT;