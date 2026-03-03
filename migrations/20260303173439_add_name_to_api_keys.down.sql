DROP INDEX api_keys_name_idx;

ALTER TABLE api_keys
DROP CONSTRAINT api_keys_name_unique;

ALTER TABLE api_keys
DROP COLUMN name;
