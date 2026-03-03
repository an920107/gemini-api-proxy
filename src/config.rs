use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::error::Error;
use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub test_database_url: String,
    pub gemini_base_url: String,
    pub payload_size_limit: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            test_database_url: env::var("TEST_DATABASE_URL")?,
            gemini_base_url: env::var("GEMINI_BASE_URL")?,
            payload_size_limit: env::var("PAYLOAD_SIZE_LIMIT")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()?,
        })
    }
}

pub async fn get_db_pool(
    env_key_override: Option<&str>,
) -> Result<PgPool, Box<dyn Error + Send + Sync>> {
    let database_url = std::env::var(env_key_override.unwrap_or("DATABASE_URL"))?;
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pg_pool)
}
