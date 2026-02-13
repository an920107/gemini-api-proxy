use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};

pub async fn forward_request(
    req: HttpRequest,
    body: Bytes,
    client: web::Data<reqwest::Client>,
    base_url: web::Data<String>,
) -> HttpResponse {
    // 1. Extract path tail
    let tail: String = req.match_info().query("tail").parse().unwrap_or_default();

    // 2. Construct upstream URL
    let base = base_url.as_str().trim_end_matches('/');
    let upstream_url = format!("{}/v1beta/{}", base, tail);

    info!("Forwarding request to: {}", upstream_url);

    // 3. Create upstream request
    let mut upstream_req = client.post(&upstream_url);

    // 4. Forward headers
    if let Some(api_key) = req.headers().get("x-goog-api-key")
        && let Ok(val) = reqwest::header::HeaderValue::from_bytes(api_key.as_bytes())
    {
        upstream_req = upstream_req.header("x-goog-api-key", val);
    }
    if let Some(content_type) = req.headers().get("content-type")
        && let Ok(val) = reqwest::header::HeaderValue::from_bytes(content_type.as_bytes())
    {
        upstream_req = upstream_req.header("content-type", val);
    }

    // 5. Forward body
    upstream_req = upstream_req.body(body);

    // 6. Send request
    match upstream_req.send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let headers = resp.headers().clone(); // Clone headers before consuming body

            // 7. Relay response
            match resp.bytes().await {
                Ok(bytes) => {
                    let mut builder = HttpResponse::build(status);
                    // Forward Content-Type if present
                    if let Some(ct) = headers.get("content-type")
                        && let Ok(val) =
                            actix_web::http::header::HeaderValue::from_bytes(ct.as_bytes())
                    {
                        builder.insert_header(("content-type", val));
                    }
                    builder.body(bytes)
                }
                Err(e) => {
                    error!("Failed to read upstream response body: {}", e);
                    HttpResponse::InternalServerError().body("Upstream error")
                }
            }
        }
        Err(e) => {
            error!("Failed to forward request: {}", e);
            HttpResponse::BadGateway().body(format!("Upstream error: {}", e))
        }
    }
}
