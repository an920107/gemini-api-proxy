use actix_web::{App, test, web};
use gemini_api_proxy::routes::health;
use serde_json::Value;
mod common;

#[actix_web::test]
async fn health_check_returns_200_ok_and_db_connected() {
    // Use the shared helper to connect to the test database
    let pool = common::configure_test_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/health").route(web::get().to(health::health_check))),
    )
    .await;

    // Act
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let response_body: Value = test::read_body_json(resp).await;
    let expected_body: Value = serde_json::json!({ "status": "ok", "db": "connected" });
    assert_eq!(response_body, expected_body);
}
