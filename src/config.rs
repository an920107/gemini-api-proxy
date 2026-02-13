use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn get_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool.")
}
