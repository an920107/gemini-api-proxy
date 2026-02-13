use chrono::NaiveDateTime;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct RequestLog {
    pub id: Uuid,
    pub api_key_id: Uuid,
    pub endpoint: String,
    pub model_version: String,
    pub prompt_tokens: i32,
    pub candidate_tokens: i32,
    pub total_tokens: i32,
    pub latency_ms: i64,
    pub created_at: NaiveDateTime,
}

impl RequestLog {
    pub async fn create(
        pool: &PgPool,
        api_key_id: Uuid,
        endpoint: String,
        model_version: String,
        prompt_tokens: i32,
        candidate_tokens: i32,
        total_tokens: i32,
        latency_ms: i64,
    ) -> Result<Self> {
        sqlx::query_as!(
            RequestLog,
            r#"
            INSERT INTO request_logs (
                api_key_id, endpoint, model_version, prompt_tokens, candidate_tokens, total_tokens, latency_ms
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, api_key_id, endpoint, model_version, prompt_tokens, candidate_tokens, total_tokens, latency_ms, created_at
            "#,
            api_key_id,
            endpoint,
            model_version,
            prompt_tokens,
            candidate_tokens,
            total_tokens,
            latency_ms
        )
        .fetch_one(pool)
        .await
    }
}
