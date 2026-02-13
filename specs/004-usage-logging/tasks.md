---
description: "Task list for Usage Logging feature"
---

# Tasks: Usage Logging & Token Statistics

**Input**: Design documents from `/specs/004-usage-logging/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

**Tests**: Integration tests required for US1.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel
- **[Story]**: Which user story this task belongs to (e.g., US1)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create migration for `request_logs` table using `sqlx migrate add create_request_logs_table`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T002 Update `src/middleware/auth.rs` to retrieve `api_key_id` and attach it to `req.extensions()`
- [x] T003 [P] Create `src/models/request_log.rs` with `RequestLog` struct and `create` method (and export in `src/models.rs`)
- [x] T004 [P] Create `src/models/gemini.rs` with `GeminiResponsePartial` and `GeminiUsageMetadata` structs (and export in `src/models.rs`)

**Checkpoint**: Database schema ready, middleware passing ID, models defined.

---

## Phase 3: User Story 1 - Log Token Usage (Priority: P1) 🎯 MVP

**Goal**: Intercept successful Gemini API responses and asynchronously log token usage to the database.

**Independent Test**: Send a proxy request with a mocked upstream response containing usage metadata, then assert a new row in `request_logs`.

### Tests for User Story 1 (REQUIRED for ATDD) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T005 [US1] Create integration test `tests/logging_test.rs` with wiremock setup for usage metadata
- [x] T006 [US1] Write failing test assertions verifying `request_logs` insertion

### Implementation for User Story 1

- [x] T007 [US1] Update `src/routes/proxy.rs` to capture request start time and `api_key_id` from extensions
- [x] T008 [US1] Modify `src/routes/proxy.rs` to read the full upstream response body into `Bytes` (instead of streaming)
- [x] T009 [US1] Implement logic in `src/routes/proxy.rs` to parse `GeminiResponsePartial` from the response body
- [x] T010 [US1] Implement `tokio::spawn` logic in `src/routes/proxy.rs` to asynchronously call `RequestLog::create`
- [x] T011 [US1] Ensure `src/routes/proxy.rs` returns the original response body to the client

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T012 Run `cargo clippy` and fix any lints
- [x] T013 Run `cargo fmt`
- [x] T014 Verify logging manually with `quickstart.md` steps

---

## Phase 5: Enhancement - Model Version Logging

**Purpose**: Log the `modelVersion` returned by the Gemini API.

- [x] T015 Create migration to add `model_version` to `request_logs` using `sqlx migrate add add_model_version_to_request_logs`
- [x] T016 Update `src/models/request_log.rs` to include `model_version` field and update `create` method
- [x] T017 Update `src/models/gemini.rs` to include `modelVersion` field in `GeminiResponsePartial`
- [x] T018 Update `src/routes/proxy.rs` to extract and pass `model_version` to `RequestLog::create`
- [x] T019 Update `tests/logging_test.rs` to verify `model_version` is logged correctly

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup (migration).
- **User Stories (Phase 3+)**: All depend on Foundational phase completion.

### Parallel Opportunities

- T003 and T004 can run in parallel.
- T007, T008, T009 can be implemented incrementally in the same file.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 & 2 (Schema & Models).
2. Complete Phase 3 (Proxy Logic).
3. **STOP and VALIDATE**: Run `tests/logging_test.rs`.
4. Deploy/demo.
