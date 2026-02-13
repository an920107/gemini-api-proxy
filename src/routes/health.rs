use actix_web::{HttpResponse, Responder, web};
use serde_json;
use sqlx::PgPool;

pub async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    if let Ok(mut conn) = pool.acquire().await {
        if sqlx::query("SELECT 1").execute(&mut *conn).await.is_ok() {
            HttpResponse::Ok().json(serde_json::json!({ "status": "ok", "db": "connected" }))
        } else {
            HttpResponse::ServiceUnavailable()
                .json(serde_json::json!({ "status": "error", "db": "disconnected" }))
        }
    } else {
        HttpResponse::ServiceUnavailable()
            .json(serde_json::json!({ "status": "error", "db": "disconnected" }))
    }
}
