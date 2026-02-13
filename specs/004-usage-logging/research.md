# Research: Usage Logging & Token Statistics

**Version**: 1.0
**Status**: COMPLETE
**Author**: Gemini CLI Agent
**Last Updated**: 2026-02-14

## 1. Overview

This document captures the technical decisions for implementing usage logging. The primary challenge is extracting usage data from the response stream without blocking or delaying the response to the client.

## 2. Key Technical Decisions

| Technology / Pattern | Decision & Rationale | Alternatives Considered |
| :--- | :--- | :--- |
| **Context Passing** | **Actix Web Extensions (`HttpMessage::extensions`)**: Chosen to pass `api_key_id` from the authentication middleware to the proxy handler. This is the idiomatic way to share request-scoped data in Actix. | **Thread Local Storage**: Rejected due to complexity in async runtimes. **Custom Header**: Rejected as it leaks internal IDs to the request headers potentially exposed downstream. |
| **Async Logging** | **`tokio::spawn`**: Chosen to offload the database insertion to a detached background task. This ensures the HTTP response handling is not blocked by DB I/O. | **Sync Blocking**: Rejected as it increases latency directly. **Message Queue**: Rejected as overkill for the current scale; `tokio::spawn` is sufficient for MVP. |
| **Response Parsing** | **Partial Deserialization (Serde)**: Chosen to parse only the `usageMetadata` field from the JSON body. By defining a struct with only the needed fields, `serde` effectively ignores the rest, optimizing parsing performance. | **Full Deserialization**: Rejected to avoid memory overhead of parsing the entire large generation content. |
| **Body Handling** | **Buffering (`Bytes`)**: Chosen to read the full upstream response into memory to allow parsing. While streaming is generally preferred for proxies, the requirement to log usage *from the body* necessitates reading it. | **Stream Duplication**: Considered, but complex to implement correctly without buffering at least partially. Buffering is acceptable given typical text response sizes. |

## 3. Implementation Details

### 3.1. Middleware Update
The `ApiKeyAuth` middleware must select the `id` (UUID) of the API key and insert it into `req.extensions_mut()`.

### 3.2. Proxy Logic
1.  Capture start time.
2.  Forward request.
3.  Read response body to `Bytes`.
4.  Capture end time (latency).
5.  Try to deserialize `usageMetadata`.
6.  Spawn logging task.
7.  Return response with original `Bytes`.
