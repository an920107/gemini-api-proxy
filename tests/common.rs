use dotenvy;
use env_logger;
use gemini_api_proxy::config;
use sqlx::PgPool;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();
        env_logger::init(); // Initialize logger for tests
    });
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
    pool
}
