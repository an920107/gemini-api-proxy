# Feature Specification: Usage Logging & Token Statistics

**Feature Branch**: `004-usage-logging`
**Created**: 2026-02-14
**Status**: Draft
**Input**: Implement usage logging by intercepting Gemini API responses and storing token statistics in the database.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Log Token Usage (Priority: P1)

As a System Administrator, I want to track token usage for each API request so that I can monitor costs and usage patterns.

**Why this priority**: Essential for cost management and analytics.

**Independent Test**:
1.  Mock the Gemini API to return a response with known `usageMetadata`.
2.  Send a request through the proxy.
3.  Verify the client receives the response.
4.  Query the `request_logs` database table and assert that a record exists with the correct token counts.

**Acceptance Scenarios**:

1.  **Given** a valid API key and a proxy request,
    **When** the upstream Gemini API returns a successful JSON response with `usageMetadata`,
    **Then** the system logs a new record in `request_logs` containing `prompt_tokens`, `candidate_tokens`, `total_tokens`, and `latency_ms`.

2.  **Given** the logging process takes time,
    **When** a request is processed,
    **Then** the response is returned to the client immediately without waiting for the database insertion (async logging).

## Requirements *(mandatory)*

### Functional Requirements

-   **FR-001**: The system MUST intercept the HTTP response from the upstream Gemini API.
-   **FR-002**: The system MUST parse the JSON response body to extract the `usageMetadata` object.
-   **FR-003**: The system MUST persist a new record in the `request_logs` table for every successful request containing usage data.
-   **FR-004**: The log record MUST include: `api_key_id`, `endpoint`, `prompt_tokens`, `candidate_tokens`, `total_tokens`, `latency_ms`, and `created_at`.
-   **FR-005**: The logging operation MUST be performed asynchronously to avoid blocking the HTTP response to the client.
-   **FR-006**: The system MUST return the original byte-exact response body to the client.

### Key Entities

-   **RequestLog**:
    -   `id`: UUID (Primary Key)
    -   `api_key_id`: UUID (Foreign Key to api_keys)
    -   `endpoint`: String (The API path accessed)
    -   `prompt_tokens`: Integer
    -   `candidate_tokens`: Integer
    -   `total_tokens`: Integer
    -   `latency_ms`: Integer
    -   `created_at`: Timestamp

## Success Criteria *(mandatory)*

### Measurable Outcomes

-   **SC-001**: 100% of successful proxy requests with usage metadata result in a database log entry.
-   **SC-002**: Token counts stored in the database exactly match the values returned by the upstream API.
-   **SC-003**: The logging mechanism adds less than 5ms overhead to the request latency perceived by the client.

## Assumptions

-   **A-001**: The upstream API always returns `usageMetadata` in the JSON body for successful generation requests.
-   **A-002**: The database is available and writable.
-   **A-003**: `api_key_id` can be retrieved from the request context (via middleware).
