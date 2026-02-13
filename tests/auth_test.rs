use actix_web::{App, HttpResponse, test, web};
use gemini_api_proxy::middleware::auth::ApiKeyAuth;
mod common;

const INVALID_API_KEY: &str = "INVALID_KEY";

async fn dummy_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::test]
async fn valid_api_key_returns_200_ok() {
    let pool = common::configure_test_db().await;
    common::seed_api_key(&pool).await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/v1beta/test")
                .wrap(ApiKeyAuth)
                .route(web::get().to(dummy_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1beta/test")
        .insert_header(("x-goog-api-key", common::VALID_API_KEY))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn invalid_api_key_returns_403_forbidden() {
    let pool = common::configure_test_db().await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/v1beta/test")
                .wrap(ApiKeyAuth)
                .route(web::get().to(dummy_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1beta/test")
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
            web::resource("/v1beta/test")
                .wrap(ApiKeyAuth)
                .route(web::get().to(dummy_handler)),
        ),
    )
    .await;

    let req = test::TestRequest::get().uri("/v1beta/test").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}
