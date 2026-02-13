# Quickstart: Usage Logging Verification

**Version**: 1.0
**Status**: DRAFT
**Author**: Gemini CLI Agent
**Last Updated**: 2026-02-14

## 1. Overview

This guide explains how to verify that API requests are being correctly logged to the database.

## 2. Prerequisites

*   The application must be running (`cargo run`).
*   Database migrations must be applied (`sqlx migrate run`).
*   A valid API key must be seeded in the `api_keys` table.

## 3. Verification Steps

### Step 1: Send a Request

Use `curl` or a tool like Postman to send a valid request to the proxy.

```bash
curl -X POST http://localhost:8080/v1beta/models/gemini-pro:generateContent 
  -H "x-goog-api-key: YOUR_VALID_KEY" 
  -H "Content-Type: application/json" 
  -d '{
    "contents": [{
      "parts": [{"text": "Hello"}]
    }]
  }'
```

### Step 2: Check the Logs

Connect to the database and query the `request_logs` table.

```bash
psql -d gemini_api_proxy -c "SELECT * FROM request_logs ORDER BY created_at DESC LIMIT 1;"
```

**Expected Output**:
You should see a new row with:
*   `endpoint`: `/v1beta/models/gemini-pro:generateContent`
*   `total_tokens`: A non-zero integer (e.g., 5).
*   `latency_ms`: A non-zero integer.

### Step 3: Troubleshooting

If no log appears:
1.  Check the application logs (`RUST_LOG=info cargo run`) for errors related to "Failed to parse usage metadata" or database errors.
2.  Ensure the upstream API actually returned `usageMetadata`.
