use actix_web::{App, HttpServer, web};
use gemini_api_proxy::{
    config,
    middleware::auth::ApiKeyAuth,
    routes::{health, models},
};
use log::{info, warn};
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

    // 5. Configure and start the Actix Web server.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/health").route(web::get().to(health::health_check)))
            .service(
                web::resource("/v1beta/models")
                    .wrap(ApiKeyAuth)
                    .route(web::get().to(models::list_models)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
