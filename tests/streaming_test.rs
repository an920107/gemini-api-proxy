//! tests/streaming_test.rs

use actix_web::{test, web, App};
use gemini_api_proxy::{middleware::auth::ApiKeyAuth, routes::proxy::proxy_handler, models::request_log::RequestLog};
use sqlx::PgPool;

mod common;

#[actix_web::test]
async fn test_streaming_usage_logging() {
    let pool: PgPool = common::configure_test_db().await;
    common::seed_api_key(&pool).await;

    // Mock the upstream Gemini API
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/v1beta/models/gemini-pro:streamGenerateContent")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        // Simulate a stream with a usage metadata chunk at the end
        .with_body(
            "data: {\"candidates\": [{\"content\": {\"parts\": [{\"text\": \"Hello, \"}]}}]}\r\n\r\n\
             data: {\"candidates\": [{\"content\": {\"parts\": [{\"text\": \"world!\"}]}}]}\r\n\r\n\
             data: {\"usageMetadata\": {\"promptTokenCount\": 10, \"candidatesTokenCount\": 20, \"totalTokenCount\": 30}}\r\n\r\n",
        )
        .create_async()
        .await;

    let client = reqwest::Client::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client))
            .app_data(web::Data::new(server.url()))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .route("/{tail:.*}", web::to(proxy_handler))
            ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/v1beta/models/gemini-pro:streamGenerateContent")
        .insert_header(("x-goog-api-key", common::VALID_API_KEY))
        .set_payload(r#"{"contents":[{"parts":[{"text":"test"}]}]}"#)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    mock.assert_async().await;
    
    // Wait for the request log to be written to the database
    common::wait_for_request_log(&pool).await;

    // Check the database for the usage log
    let log = sqlx::query_as::<_, RequestLog>("SELECT * FROM request_logs")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch request log");

    assert_eq!(log.prompt_tokens, 10);
    assert_eq!(log.candidate_tokens, 20);
    assert_eq!(log.total_tokens, 30);
}
