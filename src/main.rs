mod config;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{middleware, App, HttpServer};
use config::ServerConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = ServerConfig::from_env();
    let bind_address = config.bind_address();

    // Display server info
    config.display_info();

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(routes::configure)
    })
    .bind(&bind_address)?
    .run()
    .await
}