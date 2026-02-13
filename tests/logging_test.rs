use actix_web::{App, test, web};
use gemini_api_proxy::{middleware::auth::ApiKeyAuth, routes::proxy};
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
mod common;

#[actix_web::test]
async fn test_request_logging() {
    // 1. Setup
    let pool = common::configure_test_db().await;
    common::seed_api_key(&pool).await;

    let mock_server = MockServer::start().await;
    let gemini_base_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-pro:generateContent"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "candidates": [{}],
            "usageMetadata": {
                "promptTokenCount": 10,
                "candidatesTokenCount": 20,
                "totalTokenCount": 30
            }
        })))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client))
            .app_data(web::Data::new(gemini_base_url))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .route("/{tail:.*}", web::to(proxy::forward_request)),
            ),
    )
    .await;

    // 2. Act
    let req = test::TestRequest::post()
        .uri("/v1beta/models/gemini-pro:generateContent")
        .insert_header(("x-goog-api-key", common::VALID_API_KEY))
        .set_json(serde_json::json!({}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // 3. Wait for async logging (tokio::spawn)
    tokio::time::sleep(Duration::from_millis(200)).await;

    // 4. Assert Log
    let logs = sqlx::query!("SELECT * FROM request_logs")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1, "Expected 1 log entry, found {}", logs.len());
    let log = &logs[0];
    assert_eq!(log.endpoint, "models/gemini-pro:generateContent");
    assert_eq!(log.prompt_tokens, 10);
    assert_eq!(log.candidate_tokens, 20);
    assert_eq!(log.total_tokens, 30);
    assert!(log.latency_ms > 0);
}
