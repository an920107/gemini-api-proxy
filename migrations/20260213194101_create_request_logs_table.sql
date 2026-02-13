CREATE TABLE request_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    api_key_id UUID NOT NULL REFERENCES api_keys(id),
    endpoint TEXT NOT NULL,
    model_version TEXT NOT NULL,
    prompt_tokens INT NOT NULL DEFAULT 0,
    candidate_tokens INT NOT NULL DEFAULT 0,
    total_tokens INT NOT NULL DEFAULT 0,
    latency_ms BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_request_logs_api_key_id ON request_logs (api_key_id);
CREATE INDEX idx_request_logs_created_at ON request_logs (created_at);
