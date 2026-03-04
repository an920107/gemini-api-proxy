use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    http::StatusCode,
    test, App, Error, HttpResponse, web,
};
use gemini_api_proxy::middleware::auth::ApiKeyAuth;
use sqlx::PgPool;
use uuid::Uuid;

mod common;

const INVALID_API_KEY: &str = "INVALID_KEY";
async fn dummy_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn setup_app() -> (
    impl Service<Request, Response = ServiceResponse, Error = Error>,
    PgPool,
) {
    let pool = common::configure_test_db().await;
    let mock_config = common::setup_test_config("http://localhost".to_string());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(mock_config))
            .service(
                web::resource("/v1beta/test")
                    .wrap(ApiKeyAuth)
                    .route(web::get().to(dummy_handler)),
            ),
    )
    .await;
    (app, pool)
}

#[actix_web::test]
async fn valid_api_key_returns_200_ok() {
    let (app, pool) = setup_app().await;
    let api_key = common::seed_unique_api_key(&pool, &Uuid::new_v4().to_string()).await;

    let req = test::TestRequest::get()
        .uri("/v1beta/test")
        .insert_header(("x-goog-api-key", api_key.as_str()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn invalid_api_key_returns_403_forbidden() {
    let (app, _) = setup_app().await;

    let req = test::TestRequest::get()
        .uri("/v1beta/test")
        .insert_header(("x-goog-api-key", INVALID_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn missing_api_key_returns_401_unauthorized() {
    let (app, _) = setup_app().await;

    let req = test::TestRequest::get().uri("/v1beta/test").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
