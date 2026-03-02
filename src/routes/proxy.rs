use crate::errors::StreamError;
use crate::models::{
    gemini::{GeminiResponsePartial, GeminiUsageMetadata, StreamedGeminiResponse},
    request_log::RequestLog,
};
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt;
use log::{error, info, warn};
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

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

pub async fn proxy_handler(
    req: HttpRequest,
    body: Bytes,
    client: web::Data<reqwest::Client>,
    base_url: web::Data<String>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let start = Instant::now();
    let api_key_id = req.extensions().get::<Uuid>().cloned();

    let tail = req
        .match_info()
        .get("tail")
        .unwrap_or_else(|| req.path().trim_start_matches("/v1beta/"));
    let endpoint = tail.to_string();

    let base = base_url.as_str().trim_end_matches('/');
    let mut upstream_url = format!("{}/v1beta/{}", base, tail);
    if let Some(query) = req.uri().query() {
        upstream_url.push('?');
        upstream_url.push_str(query);
    }
    info!("Forwarding {} request to: {}", req.method(), upstream_url);

    let method = match reqwest::Method::from_bytes(req.method().as_str().as_bytes()) {
        Ok(m) => m,
        Err(_) => return HttpResponse::BadRequest().body("Invalid HTTP method"),
    };

    let mut upstream_req = client.request(method, &upstream_url);
    for (header_name, header_value) in req.headers().iter() {
        let name_str = header_name.as_str().to_lowercase();
        if HOP_BY_HOP_HEADERS.contains(name_str.as_str()) {
            continue;
        }
        if let Ok(val) = reqwest::header::HeaderValue::from_bytes(header_value.as_bytes()) {
            upstream_req = upstream_req.header(header_name.as_str(), val);
        }
    }
    if !body.is_empty() {
        upstream_req = upstream_req.body(body);
    }

    let upstream_resp = match upstream_req.send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to forward request: {}", e);
            return HttpResponse::BadGateway().body(format!("Upstream error: {}", e));
        }
    };

    let status = StatusCode::from_u16(upstream_resp.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let headers = upstream_resp.headers().clone();

    let mut builder = HttpResponse::build(status);
    for (header_name, header_value) in headers.iter() {
        let name_str = header_name.as_str().to_lowercase();
        if HOP_BY_HOP_HEADERS.contains(name_str.as_str()) || name_str == "x-goog-api-key" {
            continue;
        }
        if let Ok(val) = actix_web::http::header::HeaderValue::from_bytes(header_value.as_bytes()) {
            builder.insert_header((header_name.as_str(), val));
        }
    }

    if headers
        .get("content-type")
        .map_or(false, |h| h.to_str().unwrap_or("").contains("text/event-stream"))
    {
        let usage_metadata = Arc::new(Mutex::new(None::<GeminiUsageMetadata>));
        let usage_metadata_clone = usage_metadata.clone();

        let mut original_stream = upstream_resp.bytes_stream();
        let (tx, rx) = mpsc::unbounded_channel::<Result<Bytes, StreamError>>();
        let stream_rx = UnboundedReceiverStream::new(rx);

        tokio::spawn(async move {
            while let Some(item) = original_stream.next().await {
                match item {
                    Ok(chunk) => {
                        if let Ok(text) = std::str::from_utf8(&chunk) {
                            for line in text.lines().filter(|l| l.starts_with("data: ")) {
                                let json_str = &line["data: ".len()..];
                                if let Ok(parsed) =
                                    serde_json::from_str::<StreamedGeminiResponse>(json_str)
                                {
                                    if let Some(usage) = parsed.usage_metadata {
                                        *usage_metadata_clone.lock().unwrap() = Some(usage);
                                        break;
                                    }
                                }
                            }
                        }
                        if tx.send(Ok(chunk)).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(StreamError {
                            status: StatusCode::INTERNAL_SERVER_ERROR,
                            message: e.to_string(),
                        }));
                        break;
                    }
                }
            }
        });

        let latency_ms = start.elapsed().as_millis() as i64;
        if let Some(key_id) = api_key_id {
            let pool = pool.get_ref().clone();
            let endpoint_clone = endpoint.clone();
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                let usage = usage_metadata.lock().unwrap().take();
                if let Some(u) = usage {
                    if let Err(e) = RequestLog::create(
                        &pool,
                        key_id,
                        endpoint_clone,
                        "unknown".to_string(),
                        u.prompt_token_count.unwrap_or(0),
                        u.candidates_token_count.unwrap_or(0),
                        u.total_token_count.unwrap_or(0),
                        latency_ms,
                    )
                    .await
                    {
                        error!("Failed to log request usage: {}", e);
                    }
                } else {
                    warn!(
                        "No usage metadata found in stream for endpoint: {}",
                        endpoint_clone
                    );
                }
            });
        }

        return builder.streaming(stream_rx);
    }

    match upstream_resp.bytes().await {
        Ok(bytes) => {
            let latency_ms = start.elapsed().as_millis() as i64;
            if status.is_success() {
                if let Ok(partial) = serde_json::from_slice::<GeminiResponsePartial>(&bytes) {
                    if let Some(usage) = partial.usage_metadata {
                        if let Some(key_id) = api_key_id {
                            let pool = pool.get_ref().clone();
                            let endpoint_clone = endpoint.clone();
                            tokio::spawn(async move {
                                if let Err(e) = RequestLog::create(
                                    &pool,
                                    key_id,
                                    endpoint_clone,
                                    partial.model_version.unwrap_or("unknown".to_string()),
                                    usage.prompt_token_count.unwrap_or(0),
                                    usage.candidates_token_count.unwrap_or(0),
                                    usage.total_token_count.unwrap_or(0),
                                    latency_ms,
                                )
                                .await
                                {
                                    error!("Failed to log request usage: {}", e);
                                }
                            });
                        }
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
