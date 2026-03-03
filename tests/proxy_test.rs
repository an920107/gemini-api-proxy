use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    test, App, Error, web,
};
use gemini_api_proxy::{middleware::auth::ApiKeyAuth, routes::proxy};
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;

async fn setup_app(
    unique_name: &str,
) -> (
    impl Service<Request, Response = ServiceResponse, Error = Error>,
    MockServer,
    String,
) {
    let pool = common::configure_test_db().await;
    let api_key = common::seed_unique_api_key(&pool, unique_name).await;
    let mock_server = MockServer::start().await;
    let mock_config = common::setup_test_config(mock_server.uri());
    let client = reqwest::Client::new();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client))
            .app_data(web::Data::new(mock_config))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .route("/{tail:.*}", web::to(proxy::proxy_handler)),
            ),
    )
    .await;
    (app, mock_server, api_key)
}

#[actix_web::test]
async fn test_proxy_request_forwarding() {
    let unique_name = Uuid::new_v4().to_string();
    let (app, mock_server, api_key) = setup_app(&unique_name).await;
    Mock::given(method("POST"))
        .and(path("/v1beta/models/gemini-pro:generateContent"))
        .and(header("x-goog-api-key", api_key.as_str()))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "contents": [{
                "parts": [{"text": "Hello"}]
            }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{"text": "Hello world"}]
                }
            }]
        })))
        .mount(&mock_server)
        .await;
    let req = test::TestRequest::post()
        .uri("/v1beta/models/gemini-pro:generateContent")
        .insert_header(("x-goog-api-key", api_key.as_str()))
        .set_json(serde_json::json!({
            "contents": [{
                "parts": [{"text": "Hello"}]
            }]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body["candidates"][0]["content"]["parts"][0]["text"],
        "Hello world"
    );
}

#[actix_web::test]
async fn test_proxy_get_models() {
    let unique_name = Uuid::new_v4().to_string();
    let (app, mock_server, api_key) = setup_app(&unique_name).await;
    Mock::given(method("GET"))
        .and(path("/v1beta/models"))
        .and(header("x-goog-api-key", api_key.as_str()))
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
    let req = test::TestRequest::get()
        .uri("/v1beta/models")
        .insert_header(("x-goog-api-key", api_key.as_str()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["models"][0]["name"], "models/gemini-pro");
}
