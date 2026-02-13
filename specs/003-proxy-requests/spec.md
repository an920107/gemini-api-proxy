# Feature Specification: Request Forwarding (Transparent Proxy)

**Feature Branch**: `003-proxy-requests`
**Created**: 2026-02-13
**Status**: Draft
**Input**: Implement request forwarding logic to proxy requests to Google's Gemini API.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Proxy Request to Gemini API (Priority: P1)

As an API Client, I want my requests to be forwarded to the Google Gemini API so that I can utilize the LLM capabilities through this proxy.

**Why this priority**: Core functionality of the proxy service.

**Independent Test**: Mock the upstream Google API endpoint. Send a request to the proxy's `/v1beta/...` endpoint. Verify the mock receives the request with correct headers and body, and the client receives the mock's response.

**Acceptance Scenarios**:

1.  **Given** a running proxy service and a valid API key,
    **When** I send a `POST` request to `/v1beta/models/gemini-pro:generateContent` with a valid JSON body,
    **Then** the proxy forwards the request to `https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent`, including the `x-goog-api-key` header and the JSON body, and returns the upstream response to me.

2.  **Given** the upstream API returns an error (e.g., 400 Bad Request),
    **When** I send a request to the proxy,
    **Then** the proxy returns the same error status code and body to me.

## Requirements *(mandatory)*

### Functional Requirements

-   **FR-001**: The system MUST match all `POST` requests starting with `/v1beta/` and capture the remaining path segment.
-   **FR-002**: The system MUST forward the matched request to `https://generativelanguage.googleapis.com/v1beta/{captured_path}`.
-   **FR-003**: The system MUST forward the `x-goog-api-key` header from the incoming request to the upstream request.
-   **FR-004**: The system MUST forward the JSON body of the incoming request strictly as-is to the upstream URL.
-   **FR-005**: The system MUST return the HTTP status code, headers, and body received from the upstream API back to the client.
-   **FR-006**: The system SHOULD support non-streaming responses for the MVP (streaming support is optional for this iteration).

### Key Entities

-   **Proxy Request**: Represents the incoming request from the client.
-   **Upstream Request**: The request sent by the proxy to the Google Gemini API.
-   **Upstream Response**: The response received from Google, to be relayed to the client.

## Success Criteria *(mandatory)*

### Measurable Outcomes

-   **SC-001**: 100% of requests to `/v1beta/*` are forwarded to the correct corresponding upstream URL.
-   **SC-002**: The `x-goog-api-key` header is present in 100% of forwarded requests.
-   **SC-003**: The response body returned to the client is byte-for-byte identical to the upstream response body (for non-streaming).
-   **SC-004**: The system handles upstream errors (4xx, 5xx) by relaying the status code and body to the client.

## Assumptions

-   **A-001**: The client provides a valid `x-goog-api-key` header.
-   **A-002**: The upstream API uses HTTPS.
-   **A-003**: The local network allows outbound connections to `generativelanguage.googleapis.com`.
