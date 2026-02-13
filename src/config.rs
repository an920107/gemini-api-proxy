use sqlx::{PgPool, postgres::PgPoolOptions};
use std::error::Error;
use std::time::Duration;

pub async fn get_db_pool() -> Result<PgPool, Box<dyn Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pg_pool)
}
