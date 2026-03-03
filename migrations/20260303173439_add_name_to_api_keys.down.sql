DROP INDEX IF EXISTS api_keys_name_idx;

ALTER TABLE api_keys
DROP CONSTRAINT IF EXISTS api_keys_name_unique;

ALTER TABLE api_keys
DROP COLUMN IF EXISTS name;
