# Tasks for Feature: Parse Usage Metadata from Streaming Responses

**Branch**: `012-streaming-usage` | **Feature Spec**: [./spec.md](./spec.md)

This document outlines the tasks required to implement the streaming usage metadata parsing feature. The tasks are organized into phases, with each phase representing a logical step in the development process. The primary goal is to deliver User Story 1 (US1) as the Minimum Viable Product (MVP).

---

## Phase 1: Setup

*Goal: Prepare the development environment and create necessary placeholders.*

- [x] T001 Create a new test file for streaming usage logging in `tests/streaming_test.rs`

---

## Phase 2: User Story 1 - Capture Usage for Streaming Requests

*Goal: Implement the core functionality to capture and store token usage metadata for streaming API requests.*
*Independent Test: A streaming request to the proxy results in a database record with correct token counts.*

- [x] T002 [US1] Write a failing acceptance test in `tests/streaming_test.rs` that sends a streaming request to the proxy and asserts that the `request_logs` table contains a record with the correct token usage metadata.
- [x] T003 [P] [US1] In `src/routes/proxy.rs`, modify the proxy handler to detect streaming responses (e.g., by checking `Content-Type: text/event-stream` or `Transfer-Encoding: chunked`).
- [x] T004 [US1] In `src/routes/proxy.rs`, implement logic to intercept and parse the response stream. This will involve creating a new stream that wraps the original, inspects the chunks for usage metadata, and forwards the chunks to the client.
- [x] T005 [P] [US1] In `src/models/gemini.rs`, define a struct to represent the usage metadata that may be present in the stream.
- [x] T006 [US1] In `src/routes/proxy.rs`, when usage metadata is found in the stream, extract the `prompt_tokens`, `completion_tokens`, and `total_tokens`.
- [x] T007 [US1] In `src/routes/proxy.rs`, update the corresponding `request_logs` record in the database with the extracted usage metadata. This should happen after the stream has completed.
- [x] T008 [US1] Ensure the acceptance test created in T002 now passes.
- [x] T009 [US1] Manually verify that non-streaming requests in `tests/proxy_test.rs` still log usage correctly and are unaffected by the changes.
---

## Phase 3: Polish & Cross-Cutting Concerns

*Goal: Ensure the implementation is robust and handles edge cases.*

- [x] T010 Review and refactor the stream parsing logic in `src/routes/proxy.rs` for clarity, performance, and adherence to Rust best practices.
- [x] T011 Add logging to the stream parsing logic to aid in debugging potential issues with malformed chunks or missing usage data.
- [x] T012 Verify that the performance impact of the stream parsing logic is within the 5ms latency goal defined in `research.md`.

---

## Dependencies

```mermaid
graph TD
    subgraph Phase 1
        T001
    end

    subgraph Phase 2 (US1)
        T002 --> T003
        T003 --> T004
        T004 --> T006
        T005 --> T006
        T006 --> T007
        T007 --> T008
        T008 --> T009
    end

    subgraph Phase 3
        T009 --> T010
        T010 --> T011
        T011 --> T012
    end

    Phase1 --> Phase2
    Phase2 --> Phase3
```

## Parallel Execution Examples

- **Within US1**:
  - `T003` (detecting stream response) and `T005` (defining usage metadata struct) can be developed in parallel as they are in different files and don't have a direct code dependency.

## Implementation Strategy

The implementation will focus on delivering User Story 1 (US1) as a complete, independently testable feature. This constitutes the MVP for this feature. The tasks are ordered to follow an ATDD (Acceptance Test-Driven Development) approach, starting with a failing test and progressively implementing the functionality to make it pass.
