use actix_web::{App, test};
use gemini_api_proxy::config;
use gemini_api_proxy::routes::health;
use serde_json::Value;

#[actix_web::test]
async fn health_check_returns_200_ok_and_db_connected() {
    // Load .env variables
    dotenvy::dotenv().ok();

    let pool = config::get_db_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(health::health_check),
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
