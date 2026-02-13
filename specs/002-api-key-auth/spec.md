# Feature: API Key Whitelist Authentication

**Version**: 1.0
**Status**: DRAFT
**Author**: Gemini CLI Agent
**Last Updated**: 2026-02-13

## 1. Overview

This document specifies the requirements for an API key authentication mechanism. The system will validate incoming requests by checking for a provided API key against a whitelist of approved keys stored in the database. This ensures that only authorized clients can access protected endpoints.

## 2. User Scenarios & Testing

### 2.1. Actors

*   **API Client**: An automated system or application that needs to access the service's protected resources.
*   **Administrator**: A privileged user responsible for managing API keys.

### 2.2. User Stories & Acceptance Criteria

#### Story 1: Authenticate with a Valid API Key

*   **As an** API Client,
*   **I want to** provide my secret API key with my request,
*   **So that** the system can verify my identity and grant me access to the requested resource.

**Acceptance Criteria**:

*   **Given** an API Client has a valid and active API key,
*   **When** the client sends a request to a protected endpoint with the `x-goog-api-key` header containing the valid key,
*   **Then** the system grants access and returns a `200 OK` status.

#### Story 2: Fail Authentication with an Invalid API Key

*   **As an** API Client,
*   **I want to** be denied access if I provide an incorrect API key,
*   **So that** unauthorized access to resources is prevented.

**Acceptance Criteria**:

*   **Given** an API Client provides an API key that is not in the whitelist,
*   **When** the client sends a request to a protected endpoint,
*   **Then** the system denies access and returns a `403 Forbidden` status.

#### Story 3: Fail Authentication with a Missing API Key

*   **As an** API Client,
*   **I want to** be denied access if I do not provide an API key,
*   **So that** anonymous access to protected resources is prevented.

**Acceptance Criteria**:

*   **Given** an API Client does not provide an API key in the request header,
*   **When** the client sends a request to a protected endpoint,
*   **Then** the system denies access and returns a `401 Unauthorized` status.

## 3. Functional Requirements

### 3.1. API Key Validation

*   FR-1: The system **MUST** extract an API key from the `x-goog-api-key` HTTP request header.
*   FR-2: The system **MUST** compare the provided API key against a stored list of hashed, whitelisted keys.
*   FR-3: The system **MUST** verify that the matched API key is marked as `is_active = true`.
*   FR-4: If the key is valid and active, the request is allowed to proceed.
*   FR-5: If the key is not found in the whitelist, the request **MUST** be rejected with a `403 Forbidden` response.
*   FR-6: If the key is found but is marked as inactive (`is_active = false`), the request **MUST** be rejected with a `403 Forbidden` response.
*   FR-7: If the `x-goog-api-key` header is missing from the request, the request **MUST** be rejected with a `401 Unauthorized` response.

### 3.2. Test Endpoint

*   FR-8: A temporary endpoint at `POST /v1beta/test-auth` **MUST** be created to allow testing of the authentication mechanism. This endpoint requires valid authentication to access.

## 4. Key Entities & Data Model

### 4.1. `api_keys` Table

This table stores the API keys that are permitted to access the system.

| Field       | Type    | Constraints                  | Description                                |
|-------------|---------|------------------------------|--------------------------------------------|
| `id`        | TEXT    | PRIMARY KEY                  | A unique identifier for the API key entry. |
| `hashed_key`| TEXT    | UNIQUE, NOT NULL             | The securely hashed value of the API key.  |
| `is_active` | BOOLEAN | NOT NULL, DEFAULT `true`     | A flag to enable or disable the key.       |

## 5. Success Criteria

*   SC-1: 100% of requests to protected endpoints with a valid, active API key are successfully authenticated.
*   SC-2: 100% of requests to protected endpoints with an invalid, inactive, or missing API key are denied with the appropriate HTTP status code (401 or 403).
*   SC-3: The authentication check should add no more than 50ms of latency to a request.

## 6. Assumptions

*   A-1: API keys are generated and securely distributed to clients out-of-band. This feature does not cover key generation or management UIs.
*   A-2: The hashing algorithm for the API keys will be a strong, industry-standard algorithm (e.g., SHA-256). The raw key is never stored.
*   A-3: At least one active API key will be seeded into the database for testing purposes.

## 7. Out of Scope

*   User interface for managing API keys (creating, revoking, listing).
*   Rate limiting or throttling based on API keys.
*   Permissions or roles associated with different API keys.
