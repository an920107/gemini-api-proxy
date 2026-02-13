use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub hashed_key: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

impl ApiKey {
    pub async fn find_by_hashed_key(pool: &PgPool, hashed_key: &str) -> Result<Option<Self>> {
        sqlx::query_as!(
            ApiKey,
            "SELECT id, hashed_key, is_active, created_at FROM api_keys WHERE hashed_key = $1",
            hashed_key
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, hashed_key: &str, is_active: bool) -> Result<Self> {
        sqlx::query_as!(
            ApiKey,
            r#"
            INSERT INTO api_keys (hashed_key, is_active)
            VALUES ($1, $2)
            RETURNING id, hashed_key, is_active, created_at
            "#,
            hashed_key,
            is_active
        )
        .fetch_one(pool)
        .await
    }
}
