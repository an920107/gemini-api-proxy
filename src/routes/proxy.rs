use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};

use std::collections::HashSet;
use std::sync::LazyLock;

static HOP_BY_HOP_HEADERS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    [
        "host",
        "content-length",
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailer",
        "transfer-encoding",
        "upgrade",
    ]
    .into()
});

pub async fn forward_request(
    req: HttpRequest,
    body: Bytes,
    client: web::Data<reqwest::Client>,
    base_url: web::Data<String>,
) -> HttpResponse {
    // 1. Extract path tail
    let tail = req.match_info().get("tail").unwrap_or_else(|| {
        req.path().trim_start_matches("/v1beta/")
    });

    // 2. Construct upstream URL
    let base = base_url.as_str().trim_end_matches('/');
    let mut upstream_url = format!("{}/v1beta/{}", base, tail);

    // Append original query string, if any, so query parameters are forwarded
    if let Some(query) = req.uri().query() {
        upstream_url.push('?');
        upstream_url.push_str(query);
    }
    info!("Forwarding {} request to: {}", req.method(), upstream_url);

    // 3. Create upstream request using the same method as the incoming request
    let method = match reqwest::Method::from_bytes(req.method().as_str().as_bytes()) {
        Ok(m) => m,
        Err(_) => return HttpResponse::BadRequest().body("Invalid HTTP method"),
    };

    let mut upstream_req = client.request(method, &upstream_url);

    // 4. Forward headers
    for (header_name, header_value) in req.headers().iter() {
        // Skip Hop-by-hop headers and others that might cause issues
        let name_str = header_name.as_str().to_lowercase();
        if HOP_BY_HOP_HEADERS.contains(name_str.as_str()) {
            continue;
        }

        if let Ok(val) = reqwest::header::HeaderValue::from_bytes(header_value.as_bytes()) {
            upstream_req = upstream_req.header(header_name.as_str(), val);
        }
    }

    // 5. Forward body if it's not empty
    if !body.is_empty() {
        upstream_req = upstream_req.body(body);
    }

    // 6. Send request
    match upstream_req.send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let headers = resp.headers().clone();

            // 7. Relay response
            match resp.bytes().await {
                Ok(bytes) => {
                    let mut builder = HttpResponse::build(status);
                    // Forward essential headers back to the client
                    for (header_name, header_value) in headers.iter() {
                        let name_str = header_name.as_str().to_lowercase();
                        if name_str == "content-type" || name_str == "x-goog-api-key" {
                            if let Ok(val) = actix_web::http::header::HeaderValue::from_bytes(
                                header_value.as_bytes(),
                            ) {
                                builder.insert_header((header_name.as_str(), val));
                            }
                        }
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
