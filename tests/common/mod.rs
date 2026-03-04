#![allow(dead_code)]

use gemini_api_proxy::{config, models::api_key::ApiKey, utils::crypto::hash_api_key};
use sqlx::PgPool;
use std::sync::Once;
use std::time::Duration;

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
pub async fn seed_api_key(pool: &PgPool, unique_name: &str) {
    let _ = ApiKey::create(pool, VALID_API_KEY_HASH, true, unique_name)
        .await
        .expect("Failed to create API key for test");
}

// Helper to seed a unique API key and return it
pub async fn seed_unique_api_key(pool: &PgPool, unique_name: &str) -> String {
    let api_key = format!("KEY_{}", uuid::Uuid::new_v4());
    let hashed_key = hash_api_key(&api_key);
    let _ = ApiKey::create(pool, &hashed_key, true, unique_name)
        .await
        .expect("Failed to create unique API key for test");
    api_key
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
    sqlx::query!("TRUNCATE TABLE api_keys, request_logs CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate tables");

    pool
}

// Helper to create a Config for tests
pub fn setup_test_config(gemini_base_url: String) -> config::Config {
    config::Config {
        database_url: "".to_string(),
        test_database_url: Some("".to_string()),
        gemini_base_url,
        payload_size_limit: 1024, // Set a default for testing
    }
}

// Helper to poll for request logs to appear in the database
pub async fn wait_for_request_log(pool: &PgPool) {
    for _ in 0..100u32 {
        let count = sqlx::query!("SELECT count(*) as count FROM request_logs")
            .fetch_one(pool)
            .await
            .expect("Failed to fetch log count")
            .count
            .unwrap_or(0);

        if count > 0 {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("Timeout waiting for request log to appear in database");
}
