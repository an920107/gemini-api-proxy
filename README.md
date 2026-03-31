# gemini-api-proxy

A lightweight Rust proxy for the Google Gemini API with:
- API key authentication (`x-goog-api-key`) backed by PostgreSQL
- Transparent request/response forwarding under `/v1beta/*`
- Usage logging (tokens, latency, endpoint, model) for normal and streaming responses

Built with `actix-web`, `reqwest`, and `sqlx`.

## Why This Project

`gemini-api-proxy` gives teams a single controlled entrypoint for Gemini traffic so they can:
- centrally manage client keys
- enforce payload limits
- capture per-request usage data for cost and observability

## Features

- `GET /health` with DB connectivity status
- Auth middleware for `/v1beta/*` routes
- Generic proxying to Gemini endpoints (any method/path under `/v1beta`)
- Streaming (`text/event-stream`) support with usage extraction from SSE chunks
- Automatic request logging to PostgreSQL:
  - `endpoint`
  - `model_version`
  - `prompt_tokens`
  - `candidate_tokens`
  - `total_tokens`
  - `latency_ms`
- Automatic DB migrations on startup

## Tech Stack

- Rust (edition `2024`)
- `actix-web`
- `reqwest` (`rustls`, `stream`)
- `sqlx` + PostgreSQL
- `tokio`

## Project Layout

```text
src/
  main.rs              # server startup, config loading, migration, route wiring
  middleware/auth.rs   # API key auth middleware
  routes/health.rs     # health endpoint
  routes/proxy.rs      # proxy + usage logging
  models/              # database models
migrations/            # SQL migrations
tests/                 # integration tests
```

## Requirements

- Rust toolchain (stable)
- PostgreSQL (local or remote)

## Quick Start

1. Clone and enter the repo:

```bash
git clone <your-fork-or-repo-url>
cd gemini-api-proxy
```

2. Create environment file:

```bash
cp .env.example .env
```

3. Create databases referenced by `.env`:

```bash
createdb gemini_api_proxy
createdb test_gemini_api_proxy
```

4. Start the server (migrations run automatically at startup):

```bash
cargo run
```

Server listens on `0.0.0.0:8080`.

## Configuration

Environment variables:

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | Yes | - | PostgreSQL URL for runtime DB |
| `TEST_DATABASE_URL` | No | - | PostgreSQL URL used by tests |
| `GEMINI_BASE_URL` | No | `https://generativelanguage.googleapis.com` | Upstream Gemini base URL |
| `PAYLOAD_SIZE_LIMIT` | No | `10485760` | Max request payload size in bytes |

## API Key Management

The proxy validates `x-goog-api-key` by hashing it with SHA-256 and comparing it to `api_keys.hashed_key`.

There is no admin HTTP endpoint for key creation in this repository yet. Add keys directly in DB:

```sql
INSERT INTO api_keys (hashed_key, is_active, name)
VALUES (encode(digest('YOUR_PLAIN_API_KEY', 'sha256'), 'hex'), true, 'local-dev-key');
```

Notes:
- `name` must be unique
- inactive keys return `403 Forbidden`
- missing key returns `401 Unauthorized`

## Usage

Health check:

```bash
curl http://localhost:8080/health
```

Proxy Gemini request:

```bash
curl -X POST "http://localhost:8080/v1beta/models/gemini-pro:generateContent" \
  -H "Content-Type: application/json" \
  -H "x-goog-api-key: YOUR_PLAIN_API_KEY" \
  -d '{
    "contents": [{
      "parts": [{"text": "Hello"}]
    }]
  }'
```

Any path under `/v1beta/*` is forwarded to:

`{GEMINI_BASE_URL}/v1beta/*`

## Data Model

Main tables:
- `api_keys`: stored hashed keys and activation status
- `request_logs`: per-request usage metrics and latency

## Testing

Run all tests:

```bash
cargo test
```

Integration tests use `TEST_DATABASE_URL` and run migrations automatically.

## Docker

Build image:

```bash
docker build -t gemini-api-proxy:local .
```

Run container:

```bash
docker run --rm -p 8080:8080 --env-file .env gemini-api-proxy:local
```

## Development Notes

- `RUST_LOG=info` enables structured runtime logs.
- Proxy strips hop-by-hop headers before forwarding.
- Request usage logging is performed asynchronously.

## Contributing

Contributions are welcome.

Recommended workflow:
1. Fork the repository
2. Create a feature branch
3. Add/adjust tests for your change
4. Open a pull request

## License

Licensed under the Apache License 2.0. See `LICENSE`.

