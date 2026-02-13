use actix_web::{App, HttpServer, web};
use log::{info, warn};
use std::error::Error;

mod config;
mod routes;

/// Entry point for the Actix Web application.
#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Initialize the logger for structured and colorful logging.
    env_logger::init();
    dotenvy::dotenv().ok();
    info!("Server startup initiated.");

    // 2. Create and get the database connection pool.
    // The application will panic if it cannot establish a connection.
    let pool = config::get_db_pool().await?;
    info!("Database connection pool created.");

    // 3. Perform a quick check to ensure the database connection is active.
    let mut conn = pool.acquire().await?;
    match sqlx::query("SELECT 1").execute(&mut *conn).await {
        Ok(_) => info!("Database connectivity verified."),
        Err(e) => warn!("Database query test failed: {:?}", e),
    }

    // 4. Configure and start the Actix Web server.
    // The `move` keyword is used to transfer ownership of `pool` into the closure.
    HttpServer::new(move || {
        App::new()
            // Register the database pool as application data, making it accessible to handlers.
            .app_data(web::Data::new(pool.clone()))
            // Register the health check service.
            .service(routes::health::health_check)
    })
    // Bind the server to the specified address and port.
    .bind(("0.0.0.0", 8080))?
    // Run the server, awaiting its termination.
    .run()
    .await?; // Add '?' to handle the Result from .run().await

    Ok(())
}
