# Tasks: Project Skeleton & Database Connectivity

**Input**: Design documents from `specs/001-project-skeleton-and-db-connectivity/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

**Tests**: Per the ATDD First principle in the constitution, test tasks are included and must be implemented first.

**Organization**: Tasks are grouped by phase to ensure a logical build order, culminating in an independently testable feature.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize a new Rust project using `cargo init`
- [X] T002 Add dependencies to `Cargo.toml`: `actix-web`, `tokio`, `serde`, `dotenvy`, `sqlx`, `env_logger`, `log`
- [X] T003 [P] Create `.env` and `.env.example` files with the `DATABASE_URL` variable
- [X] T004 [P] Create the project directory structure: `src/routes/` and `tests/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before the user story can be implemented

- [X] T005 Setup the database using `sqlx-cli`: `sqlx database create`
- [X] T006 Create an initial database migration file in `migrations/0001_init.sql`
- [X] T007 Implement database configuration and connection pool logic in `src/config.rs`
- [X] T008 Setup basic logging and initialize the logger in `src/main.rs`

---

## Phase 3: User Story 1 - Health Check (Priority: P1) 🎯 MVP

**Goal**: Implement a health check endpoint to verify application status and database connectivity.

**Independent Test**: Send a GET request to `/health`. A 200 OK response with `{"status": "ok", "db": "connected"}` indicates success.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T009 [US1] Create the integration test file `tests/health_check.rs`
- [X] T010 [US1] In `tests/health_check.rs`, write a test that spawns the app, sends a GET request to `/health`, and asserts a successful response.

### Implementation for User Story 1

- [X] T011 [US1] Implement the `health_check` handler function in `src/routes/health.rs`
- [X] T012 [US1] In `src/main.rs`, create the Actix `App`, register the `/health` route, and start the HTTP server.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements

- [X] T013 [P] Add comments to `src/main.rs` explaining the application startup sequence.
- [X] T014 Run `cargo fmt` and `cargo clippy` to ensure code quality.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Must be completed first.
- **Foundational (Phase 2)**: Depends on Setup completion.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **Polish (Phase N)**: Depends on User Story 1 completion.

### Within Each User Story

- Tests (T009, T010) MUST be written and FAIL before implementation (T011, T012).

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1.  Complete Phase 1: Setup
2.  Complete Phase 2: Foundational
3.  Complete Phase 3: User Story 1
4.  **STOP and VALIDATE**: Run `cargo test` to ensure the integration test passes.
5.  Run `cargo run` and manually `curl http://127.0.0.1:8080/health` to verify.
