<!--
Sync Impact Report:
- Version change: 1.1.0 → 1.2.0
- Modified principles:
    - Coding Standards (Added module organization rule)
- Added sections: None
- Removed sections: None
- Templates requiring updates:
    - ✅ .specify/templates/plan-template.md
- Follow-up TODOs: None
-->
# High-performance API Proxy Constitution

## Core Principles

### ATDD First
We strictly follow Acceptance Test-Driven Development. No feature implementation begins without a failing acceptance test (integration test).

### Architecture
Use a pragmatic MVC (Model-View-Controller) pattern.
-   **Model:** `sqlx` structs and database logic.
-   **Controller:** `actix-web` handlers containing business logic.
-   **View:** JSON responses (using `serde`), or original response from api source.
-   *Note:* Keep it simple, but ensure handlers are not overly bloated. Extract complex logic into helper functions if necessary.

### Error Handling
Use `thiserror` for library-level errors and implement `ResponseError` for `actix-web` to map errors to HTTP status codes centrally.

### Database
Use `sqlx` (Async) with PostgreSQ (strictly typed). Ensure all SQL queries are checked at compile time if possible (`sqlx::query_as!`).

## Coding Standards
-   **Language:** Rust (2021 edition or later).
-   **Style:** Follow idiomatic Rust (clippy compliant). Prefer `Result` and `Option` over unwrap.
-   **Testing:** Use `actix-web`'s integration test utilities for acceptance tests.
-   **Dependencies:** Use `cargo add` / `cargo remove` commands to manage dependencies. Do not edit `Cargo.toml` manually unless necessary.
-   **Modules:** Use `feature.rs` style for defining modules instead of `feature/mod.rs` (Rust 2018 edition style).

## Governance
This Constitution supersedes all other practices. Amendments require documentation, approval, and a migration plan. All pull requests and reviews must verify compliance with this constitution.

**Version**: 1.2.0 | **Ratified**: 2026-02-13 | **Last Amended**: 2026-02-13
