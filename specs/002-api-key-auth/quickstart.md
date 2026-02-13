# Quickstart: API Key Authentication

**Version**: 1.0
**Status**: DRAFT
**Author**: Gemini CLI Agent
**Last Updated**: 2026-02-13

## 1. Overview

This guide provides the essential steps to run the database migration, start the server, and test the API key authentication feature.

## 2. Prerequisites

*   Rust and Cargo installed.
*   `sqlx-cli` installed (`cargo install sqlx-cli`).
*   A running PostgreSQL database.
*   A `.env` file configured with the correct `DATABASE_URL`.

## 3. Setup and Execution

### Step 1: Run the Database Migration

This will create the `api_keys` table and seed it with a test key.

```bash
# Ensure your DATABASE_URL is correctly set in .env
sqlx migrate run
```

### Step 2: Start the Application

Compile and run the server.

```bash
cargo run
```

The server will start on `127.0.0.1:8080`.

## 4. Testing the Endpoint

You can use a tool like `curl` to test the authentication endpoint.

### Scenario 1: Valid API Key

*   **Expected Result**: `200 OK`

```bash
curl -v -X GET -H "x-goog-api-key: VALID_TEST_KEY" http://127.0.0.1:8080/v1beta/models
```

### Scenario 2: Invalid API Key

*   **Expected Result**: `403 Forbidden`

```bash
curl -v -X GET -H "x-goog-api-key: INVALID_KEY" http://127.0.0.1:8080/v1beta/models
```

### Scenario 3: Missing API Key

*   **Expected Result**: `401 Unauthorized`

```bash
curl -v -X GET http://127.0.0.1:8080/v1beta/models
```

## 5. Running Automated Tests

To run the integration tests that verify the scenarios above:

```bash
cargo test --test auth_check
```
