use actix_web::{App, test, web};
use gemini_api_proxy::{middleware::auth::ApiKeyAuth, routes::proxy};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
mod common;

#[actix_web::test]
async fn test_proxy_request_forwarding() {
    // 1. Setup Test DB and Seed Key
    let pool = common::configure_test_db().await;
    common::seed_api_key(&pool).await;

    // 2. Setup Mock Server
    let mock_server = MockServer::start().await;
    let gemini_base_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-pro:generateContent"))
        .and(header("x-goog-api-key", common::VALID_API_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{"text": "Hello world"}]
                }
            }]
        })))
        .mount(&mock_server)
        .await;

    // 3. Initialize Client
    let client = reqwest::Client::new();

    // 4. Initialize App
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client))
            .app_data(web::Data::new(gemini_base_url))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .route("/{tail:.*}", web::post().to(proxy::forward_request)),
            ),
    )
    .await;

    // 5. Send Request
    let req = test::TestRequest::post()
        .uri("/v1beta/models/gemini-pro:generateContent")
        .insert_header(("x-goog-api-key", common::VALID_API_KEY))
        .set_json(serde_json::json!({
            "contents": [{
                "parts": [{"text": "Hello"}]
            }]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // 6. Assert Response
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body["candidates"][0]["content"]["parts"][0]["text"],
        "Hello world"
    );
}

#[actix_web::test]
async fn test_proxy_get_models() {
    // 1. Setup Test DB and Seed Key
    let pool = common::configure_test_db().await;
    common::seed_api_key(&pool).await;

    // 2. Setup Mock Server
    let mock_server = MockServer::start().await;
    let gemini_base_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/v1beta/models"))
        .and(header("x-goog-api-key", common::VALID_API_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [
                {
                    "name": "models/gemini-pro",
                    "version": "001",
                    "displayName": "Gemini Pro"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    // 3. Initialize Client
    let client = reqwest::Client::new();

    // 4. Initialize App
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

    // 5. Send Request
    let req = test::TestRequest::get()
        .uri("/v1beta/models")
        .insert_header(("x-goog-api-key", common::VALID_API_KEY))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // 6. Assert Response
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["models"][0]["name"], "models/gemini-pro");
}
