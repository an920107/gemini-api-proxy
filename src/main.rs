use actix_web::{App, HttpServer, web};
use gemini_api_proxy::{
    config::{self, Config},
    middleware::auth::ApiKeyAuth,
    routes::{health, proxy},
};
use log::{info, warn};
use std::error::Error;

/// Entry point for the Actix Web application.
#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Initialize the logger and load environment variables.
    env_logger::init();
    dotenvy::dotenv().ok();
    info!("Server startup initiated.");

    // Load configuration
    let config = Config::from_env()?;
    let app_config = config.clone();

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

    info!("Gemini Base URL: {}", &config.gemini_base_url);
    info!("Payload size limit: {} bytes", &config.payload_size_limit);

    // 6. Configure and start the Actix Web server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .app_data(web::JsonConfig::default().limit(app_config.payload_size_limit))
            .app_data(web::PayloadConfig::default().limit(app_config.payload_size_limit))
            .service(web::resource("/health").route(web::get().to(health::health_check)))
            .service(
                web::scope("/v1beta")
                    .wrap(ApiKeyAuth)
                    .route("/{tail:.*}", web::to(proxy::proxy_handler)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
