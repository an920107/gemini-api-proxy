# Implementation Plan: Request Forwarding (Transparent Proxy)

**Branch**: `003-proxy-requests` | **Date**: 2026-02-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/003-proxy-requests/spec.md`

## Summary

Implement the actual proxying logic to forwarding requests to Google's Gemini API ("Dumb Proxy"). The system will catch all `POST /v1beta/{tail:.*}` requests, forward them to the upstream Gemini API using a shared `reqwest::Client`, and stream/return the response back to the client.

## Technical Context

**Language/Version**: Rust (2021 edition)
**Primary Dependencies**: `actix-web`, `reqwest` (features: `json`, `rustls-tls`), `wiremock` (dev-dependency)
**Storage**: N/A (Stateless proxy)
**Testing**: `cargo test` with `wiremock` for integration testing.
**Target Platform**: Linux server / Container

## Constitution Check

*   [x] **ATDD First**: Integration tests with `wiremock` will be written first.
*   [x] **Architecture**: Pragmatic MVC. Controller (`proxy.rs`) handles request/response logic.
*   [x] **Error Handling**: Standard Actix error handling; upstream errors are relayed.
*   [x] **Database**: N/A for this feature.
*   [x] **Coding Standards**: Idiomatic Rust, `clippy` compliant.

## Project Structure

### Documentation

```text
specs/003-proxy-requests/
├── plan.md              # This file
├── spec.md              # Feature specification
└── checklists/          # Quality checklists
```

### Source Code

```text
src/
├── main.rs              # App state setup (Client, Base URL)
└── routes/
    └── proxy.rs         # Proxy controller logic

tests/
└── proxy_test.rs        # Integration tests with Wiremock
```

**Structure Decision**: Single project structure, adding a new route handler module.

## Implementation Steps (Detailed)

### Step 1: Dependencies
-   Add `reqwest` (features: `json`, `rustls-tls`) to `Cargo.toml`.
-   Add `wiremock` to `[dev-dependencies]`.
-   Update `.env` to include `GEMINI_BASE_URL`.

### Step 2: Application State Setup
-   Initialize `reqwest::Client` in `main.rs`.
-   Load `GEMINI_BASE_URL` from env.
-   Pass client and URL as `web::Data` to the Actix app.

### Step 3: Proxy Controller
-   Create `src/routes/proxy.rs`.
-   Implement `forward_request` handler:
    -   Extract path tail.
    -   Construct upstream URL.
    -   Forward headers (`x-goog-api-key`, `Content-Type`).
    -   Forward body.
    -   Return upstream response.

### Step 4: Route Configuration
-   Configure `/v1beta` scope in `main.rs` to route `{tail:.*}` to `forward_request`.
-   Ensure `ApiKeyMiddleware` wraps this route.

### Step 5: ATDD - Mocking Upstream
-   Create `tests/proxy_test.rs`.
-   Use `wiremock` to simulate Google API.
-   Assert request headers/body and response relay.
