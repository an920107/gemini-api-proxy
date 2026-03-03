# Quickstart: Streaming Usage Feature

This quickstart guide outlines how to get started with the streaming usage feature of the Gemini API Proxy.

## Prerequisites

- Rust (2021 edition or later)
- Docker and Docker Compose (if running with database locally)
- PostgreSQL (if running database directly)
- An API Key for the Gemini API

## Setup

1.  **Clone the repository**:
    ```bash
    git clone <repository_url>
    cd gemini-api-proxy
    ```

2.  **Configure Environment Variables**:
    Create a `.env` file in the project root based on `.env.example` and populate it with your database connection string and Gemini API key.

    ```ini
    DATABASE_URL=postgres://user:password@localhost:5432/gemini_proxy_db
    GEMINI_API_KEY=YOUR_GEMINI_API_KEY
    ```

3.  **Run Database Migrations**:
    Ensure your database is up and running (e.g., via Docker Compose) and run migrations.
    ```bash
    cargo install sqlx-cli # if you don't have it
    sqlx migrate run
    ```

4.  **Build and Run the Proxy**:
    ```bash
    cargo run
    ```
    The proxy will start on `http://127.0.0.1:8080` (or configured port).

## Testing the Streaming Endpoint

To test the streaming endpoint, you can use `curl` or a similar HTTP client. Replace `YOUR_MODEL_ID` with an actual model ID (e.g., `gemini-pro`).

```bash
curl -X POST 
  -H "Content-Type: application/json" 
  -H "x-goog-api-key: YOUR_PROXY_API_KEY" 
  -d '{
    "contents": [
      {
        "parts": [
          {
            "text": "Write a short story about a cat."
          }
        ]
      }
    ]
  }' 
  http://127.0.0.1:8080/v1beta/models/YOUR_MODEL_ID:streamGenerateContent
```

**Expected Output**:

A stream of Server-Sent Events (SSE) will be returned, containing the generated content and potentially usage metadata in the final event.

## Verifying Usage Logging

After making a streaming request, you can inspect the `request_logs` table in your PostgreSQL database to verify that the `prompt_tokens`, `candidate_tokens`, and `total_tokens` have been recorded correctly.

```sql
SELECT id, endpoint, prompt_tokens, candidate_tokens, total_tokens, created_at FROM request_logs ORDER BY created_at DESC LIMIT 1;
```
