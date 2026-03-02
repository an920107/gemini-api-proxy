# Data Model

## Entities

### RequestLog (Existing)

The existing `request_logs` table will be used to store the usage metadata for streaming requests.

-   **id**: UUID PRIMARY KEY
-   **api_key_id**: UUID NOT NULL (REFERENCES api_keys.id)
-   **endpoint**: TEXT NOT NULL
-   **model_version**: TEXT NOT NULL
-   **prompt_tokens**: INT NOT NULL DEFAULT 0
-   **candidate_tokens**: INT NOT NULL DEFAULT 0
-   **total_tokens**: INT NOT NULL DEFAULT 0
-   **latency_ms**: BIGINT NOT NULL
-   **created_at**: TIMESTAMP NOT NULL DEFAULT NOW()

No schema changes are required for this feature. The existing columns are sufficient to store usage data from streaming requests.
