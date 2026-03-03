ALTER TABLE api_keys
ADD COLUMN name VARCHAR(255);

UPDATE api_keys SET name = id::text;

ALTER TABLE api_keys
ALTER COLUMN name SET NOT NULL;

ALTER TABLE api_keys
ADD CONSTRAINT api_keys_name_unique UNIQUE (name);

CREATE INDEX api_keys_name_idx ON api_keys (name);
