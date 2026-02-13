use actix_web::{App, test, web};
use gemini_api_proxy::{middleware::auth::ApiKeyAuth, routes::models};
use sqlx::PgPool;
mod common;

// Pre-computed SHA-256 hash of "VALID_TEST_KEY"
const VALID_API_KEY_HASH: &str = "2101dfa5e699f3895e93b8051fe9418b508af69a349d828cd70bc65d34b1b79b";
const VALID_API_KEY: &str = "VALID_TEST_KEY";
const INVALID_API_KEY: &str = "INVALID_KEY";

// Helper to seed the database with a valid API key for testing
pub async fn seed_api_key(pool: &PgPool) {
    sqlx::query!(
        "INSERT INTO api_keys (hashed_key, is_active) VALUES ($1, $2) ON CONFLICT (hashed_key) DO NOTHING",
        VALID_API_KEY_HASH,
        true
    )
    .execute(pool)
    .await
    .expect("Failed to seed API key");
}

#[actix_web::test]
async fn valid_api_key_returns_200_ok() {
    let pool = common::configure_test_db().await;
    seed_api_key(&pool).await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/v1beta/models")
                .wrap(ApiKeyAuth)
                .route(web::get().to(models::list_models)),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1beta/models")
        .insert_header(("x-goog-api-key", VALID_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn invalid_api_key_returns_403_forbidden() {
    let pool = common::configure_test_db().await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/v1beta/models")
                .wrap(ApiKeyAuth)
                .route(web::get().to(models::list_models)),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1beta/models")
        .insert_header(("x-goog-api-key", INVALID_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn missing_api_key_returns_401_unauthorized() {
    let pool = common::configure_test_db().await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/v1beta/models")
                .wrap(ApiKeyAuth)
                .route(web::get().to(models::list_models)),
        ),
    )
    .await;

    let req = test::TestRequest::get().uri("/v1beta/models").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}
