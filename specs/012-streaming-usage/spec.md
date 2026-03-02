# Feature Specification: Parse Usage Metadata from Streaming Responses

**Feature Branch**: `012-streaming-usage`  
**Created**: 2026-02-15  
**Status**: Draft  
**Input**: User description: "make the proxy can parse usage metadata from not only json response (currently implemented), but also streaming response (not yet implemented), and then save into db."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
-->

### User Story 1 - Capture Usage for Streaming Requests (Priority: P1)

As a system administrator, I want the proxy to capture and store token usage metadata (prompt tokens, completion tokens, total tokens) for streaming API requests, just as it does for standard JSON requests, so that I have accurate usage records for all traffic types.

**Why this priority**: Streaming is a critical feature for LLM interactions. Without tracking usage for streams, a significant portion of API consumption (and potentially cost) is invisible.

**Independent Test**: Can be fully tested by sending a streaming request to the proxy and verifying that the corresponding record in the database contains the correct token counts.

**Acceptance Scenarios**:

1. **Given** the proxy is running and connected to the database, **When** a client makes a successful streaming request that includes usage metadata in the response stream, **Then** the proxy should parse the usage data from the stream and save it to the database record for that request.
2. **Given** a streaming request, **When** the upstream provider does *not* return usage metadata (or the format is unrecognized), **Then** the proxy should still log the request but may record null/zero for usage, without interrupting the stream to the client.

---

### Edge Cases

- **Interrupted Streams**: What happens if the client disconnects before the stream finishes (and before usage data is sent)? The system should log what it can, but likely usage data will be missing. This should be handled gracefully without crashing.
- **Malformed Chunks**: If a chunk in the stream is malformed, the proxy should continue passing data if possible, or log the error, but ensure the database record is closed out.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST detect when an upstream response is a stream (e.g., `Transfer-Encoding: chunked` or `Content-Type: text/event-stream`).
- **FR-002**: The system MUST parse the incoming stream chunks to identify usage metadata.
  - *Assumption*: Usage metadata is typically provided in the final chunk or a specific event type in the Server-Sent Events (SSE) stream.
- **FR-003**: The system MUST extract `prompt_tokens`, `completion_tokens`, and `total_tokens` (or equivalent fields) from the stream.
- **FR-004**: The system MUST persist the extracted usage metadata to the existing request logging storage (Database).
- **FR-005**: The system MUST NOT block or significantly delay the delivery of stream chunks to the client while parsing.
- **FR-006**: The system MUST maintain existing functionality for standard JSON (non-streaming) responses.

### Key Entities *(include if feature involves data)*

- **Request Log**: The existing entity that stores details about API requests. It will be updated to include usage data for streaming requests (likely reusing the columns added for JSON support).
- **Usage Metadata**: A value object containing the counts of tokens used.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of successful streaming requests that contain usage metadata have that data recorded in the database.
- **SC-002**: Streaming latency (time to first token) is not increased by more than 5ms due to the parsing logic.
- **SC-003**: The format of the data stored for streaming requests is identical to that of non-streaming requests, allowing for unified reporting.
