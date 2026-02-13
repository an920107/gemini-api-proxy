#![allow(dead_code)]

use dotenvy;
use env_logger;
use gemini_api_proxy::{config, models::api_key::ApiKey};
use sqlx::PgPool;
use std::sync::Once;

// Pre-computed SHA-256 hash of "VALID_TEST_KEY"
pub const VALID_API_KEY_HASH: &str =
    "2101dfa5e699f3895e93b8051fe9418b508af69a349d828cd70bc65d34b1b79b";
pub const VALID_API_KEY: &str = "VALID_TEST_KEY";

static INIT: Once = Once::new();
pub fn setup() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        env_logger::init(); // Initialize logger for tests
    });
}

// Helper to seed the database with a valid API key for testing
pub async fn seed_api_key(pool: &PgPool) {
    let _ = ApiKey::create(pool, VALID_API_KEY_HASH, true).await;
}

/// Helper to configure the test database and return a connection pool
pub async fn configure_test_db() -> PgPool {
    setup(); // Load environment variables once
    let pool = config::get_db_pool(Some("TEST_DATABASE_URL"))
        .await
        .expect("Failed to get database pool.");
    // Clean slate for each test run
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations on test DB");

    // Truncate tables to remove data from previous runs
    sqlx::query!("TRUNCATE TABLE api_keys, request_logs")
        .execute(&pool)
        .await
        .expect("Failed to truncate tables");

    pool
}
