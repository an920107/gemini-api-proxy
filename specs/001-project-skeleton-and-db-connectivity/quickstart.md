# Quickstart: Project Skeleton & Database Connectivity

## Prerequisites

-   Rust (latest stable version)
-   Docker and Docker Compose
-   `sqlx-cli` (`cargo install sqlx-cli`)

## Running the Service

1.  **Start the database:**
    ```bash
    docker-compose up -d
    ```

2.  **Create and migrate the database:**
    ```bash
    # Create the database
    sqlx database create

    # Run migrations
    sqlx migrate run
    ```

3.  **Run the application:**
    ```bash
    cargo run
    ```

4.  **Verify the health check:**
    ```bash
    curl http://127.0.0.1:8080/health
    ```
    You should see:
    ```json
    {"status":"ok","db":"connected"}
    ```
