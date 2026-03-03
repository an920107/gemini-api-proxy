use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub hashed_key: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub name: String,
}

impl ApiKey {
    pub async fn find_by_hashed_key(pool: &PgPool, hashed_key: &str) -> Result<Option<Self>> {
        sqlx::query_as!(
            ApiKey,
            "SELECT id, hashed_key, is_active, created_at, name FROM api_keys WHERE hashed_key = $1",
            hashed_key
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, hashed_key: &str, is_active: bool, name: &str) -> Result<Self> {
        sqlx::query_as!(
            ApiKey,
            r#"
            INSERT INTO api_keys (hashed_key, is_active, name)
            VALUES ($1, $2, $3)
            RETURNING id, hashed_key, is_active, created_at, name
            "#,
            hashed_key,
            is_active,
            name
        )
        .fetch_one(pool)
        .await
    }
}
