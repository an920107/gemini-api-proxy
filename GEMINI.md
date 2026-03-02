# gemini-api-proxy Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-02-13

## Active Technologies
- Rust 2021 + `actix-web`, `tokio`, `serde`, `sqlx`, `reqwest` (004-usage-logging)
- Rust (2021 edition or later) + `actix-web`, `tokio`, `serde`, `sqlx`, `reqwest` (012-streaming-usage)

- Rust (2021 edition or later) + `actix-web`, `tokio`, `serde`, `dotenvy`, `sqlx`, `env_logger`, `log` (001-project-skeleton-and-db-connectivity)

## Project Structure

```text
backend/
frontend/
tests/
```

## Commands

cargo test
cargo clippy

## Code Style

Rust (2021 edition or later): Follow standard conventions

## Recent Changes
- 012-streaming-usage: Added Rust (2021 edition or later) + `actix-web`, `tokio`, `serde`, `sqlx`, `reqwest`
- 004-usage-logging: Added Rust 2021 + `actix-web`, `tokio`, `serde`, `sqlx`, `reqwest`
- 002-api-key-auth: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]


<!-- MANUAL ADDITIONS START -->
- 012-streaming-usage: Added parsing for streaming usage metadata, updated API contracts, and added quickstart guide.
<!-- MANUAL ADDITIONS END -->
