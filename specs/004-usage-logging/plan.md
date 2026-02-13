# Implementation Plan: Usage Logging & Token Statistics

**Branch**: `004-usage-logging` | **Date**: 2026-02-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/004-usage-logging/spec.md`

## Summary

Implement an "Observer" layer to track token usage by intercepting successful Gemini API responses. The system will asynchronously log `prompt_tokens`, `candidate_tokens`, `total_tokens`, and `latency_ms` to a Postgres database without blocking the client response.

## Technical Context

**Language/Version**: Rust 2021
**Primary Dependencies**: `actix-web`, `tokio`, `serde`, `sqlx`, `reqwest`
**Storage**: PostgreSQL
**Testing**: `cargo test`, `wiremock`
**Target Platform**: Linux server
**Project Type**: single
**Performance Goals**: <5ms overhead for logging mechanism
**Constraints**: Logging must be non-blocking (async/spawned)
**Scale/Scope**: Log every successful generation request

## Constitution Check

*   [x] **ATDD First**: An acceptance test is written and failing before implementation.
*   [x] **Architecture**: The solution adheres to the pragmatic MVC pattern.
*   [x] **Error Handling**: `thiserror` and `ResponseError` are used for centralized error management.
*   [x] **Database**: `sqlx` is used with compile-time checked queries.
*   [x] **Coding Standards**: The code is idiomatic Rust (2021+), clippy compliant, uses `Result`/`Option`, uses `feature.rs` instead of `feature/mod.rs`, and dependencies are managed via cargo commands.

## Project Structure

### Documentation (this feature)

```text
specs/004-usage-logging/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── models/
│   ├── gemini.rs        # Partial response struct for usage metadata
│   └── request_log.rs   # DB model for request_logs table
├── routes/
│   └── proxy.rs         # Updated to handle interception and logging
└── middleware/
    └── auth.rs          # Updated to pass api_key_id

tests/
└── logging_test.rs      # ATDD test for logging verification
```

**Structure Decision**: Single project structure extending existing modules.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
