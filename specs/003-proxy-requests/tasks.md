---
description: "Task list template for feature implementation"
---

# Tasks: Request Forwarding (Transparent Proxy)

**Input**: Design documents from `/specs/003-proxy-requests/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

**Tests**: Integration tests using `wiremock` are required for ATDD.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Add `reqwest` dependency (features: `json`, `rustls-tls`) to `Cargo.toml`
- [x] T002 Add `wiremock` to `[dev-dependencies]` in `Cargo.toml`
- [x] T003 Update `.env.example` to include `GEMINI_BASE_URL` (defaulting to `https://generativelanguage.googleapis.com`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Initialize `reqwest::Client` in `src/main.rs` and pass it to `App::new` as `web::Data`
- [x] T005 Load `GEMINI_BASE_URL` from env in `src/main.rs` and pass it to `App::new` as `web::Data`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Proxy Request to Gemini API (Priority: P1) 🎯 MVP

**Goal**: Implement the core proxying logic to forward requests to Google's Gemini API and return the response.

**Independent Test**: Use `wiremock` to simulate the upstream API and verify request forwarding and response relay.

### Tests for User Story 1 (REQUIRED for ATDD) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T006 [US1] Create integration test file `tests/proxy_test.rs` and setup `wiremock::MockServer` test harness
- [ ] T007 [US1] Write failing test case in `tests/proxy_test.rs` for forwarding a request and receiving a response

### Implementation for User Story 1

- [ ] T008 [US1] Create proxy controller module `src/routes/proxy.rs` and add `pub mod proxy;` to `src/routes.rs`
- [ ] T009 [US1] Implement `forward_request` handler signature in `src/routes/proxy.rs` accepting `req`, `body`, `client`, and `base_url`
- [ ] T010 [US1] Implement logic in `src/routes/proxy.rs` to extract path tail and construct upstream URL
- [ ] T011 [US1] Implement logic in `src/routes/proxy.rs` to forward `x-goog-api-key`, `Content-Type`, and body
- [ ] T012 [US1] Implement logic in `src/routes/proxy.rs` to await upstream response and relay status/body back to client
- [ ] T013 [US1] Configure `/v1beta` scope in `src/main.rs` to route `{tail:.*}` to `forward_request` handler
- [ ] T014 [US1] Ensure `ApiKeyMiddleware` wraps the `/v1beta` route in `src/main.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T015 Run `cargo clippy` and fix any lints
- [ ] T016 Run `cargo fmt` to ensure code style compliance
- [ ] T017 Verify implementation with manual `curl` test against real Google API (optional manual step)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Controller module before implementation logic
- Handler implementation before route configuration
- Story complete before moving to next priority

### Parallel Opportunities

- T001, T002, T003 (Phase 1) can run in parallel
- T004, T005 (Phase 2) can run in parallel
- T010, T011, T012 (Phase 3 Implementation) can be implemented in parallel once the handler signature is defined

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Core Proxy Logic)
4. **STOP and VALIDATE**: Run integration tests
5. Deploy/demo

### Incremental Delivery

1. Foundation ready (Client & Config)
2. Proxy logic implemented & tested
3. Polish & Verification
