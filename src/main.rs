use actix_web::{App, HttpServer, web};
use gemini_api_proxy::{
    config,
    middleware::auth::ApiKeyAuth,
    routes::{health, models, proxy},
};
use log::{info, warn};
use std::env;
use std::error::Error;

/// Entry point for the Actix Web application.
#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Initialize the logger for structured and colorful logging.
    env_logger::init();
    dotenvy::dotenv().ok();
    info!("Server startup initiated.");

    // 2. Create and get the database connection pool.
    let pool = config::get_db_pool(None).await?;
    info!("Database connection pool created.");

    // 3. Run database migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Database migrations applied.");

    // 4. Perform a quick check to ensure the database connection is active.
    let mut conn = pool.acquire().await?;
    match sqlx::query("SELECT 1").execute(&mut *conn).await {
        Ok(_) => info!("Database connectivity verified."),
        Err(e) => warn!("Database query test failed: {:?}", e),
    }

    // 5. Initialize shared HTTP client
    let client = reqwest::Client::new();
    let gemini_base_url = env::var("GEMINI_BASE_URL")
        .unwrap_or_else(|_| "https://generativelanguage.googleapis.com".to_string());

    info!("Gemini Base URL: {}", gemini_base_url);

    // 6. Configure and start the Actix Web server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(gemini_base_url.clone()))
            .service(web::resource("/health").route(web::get().to(health::health_check)))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .service(web::resource("/models").route(web::get().to(models::list_models)))
                    .route("/{tail:.*}", web::post().to(proxy::forward_request)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
