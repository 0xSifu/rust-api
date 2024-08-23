mod app;
mod common;
mod config;
mod decorators;
mod interceptors;
mod guards;
mod modules;
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use log::{info, error};
use crate::modules::auth::init;
use crate::config::config::init_db;
// use crate::models::migration::run_migrations;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    info!("Initializing server...");

    // Initialize database connection
    match init_db().await {
        Ok(db_client) => {
            info!("Successfully connected to MongoDB");

            // Run database migrations
            // if let Err(e) = run_migrations(&db_client).await {
            //     error!("Failed to run migrations: {:?}", e);
            //     return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to run migrations"));
            // }

            info!("Starting server on 127.0.0.1:8080...");

            // Start the HTTP server
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db_client.database("social_app"))) // Use the MongoDB database instance
                    .configure(init)  // Use the init function to configure routes
                    .wrap(Logger::default())
                    .default_service(web::route().to(|| HttpResponse::NotFound()))
            })
                .bind("127.0.0.1:8080")?
                .run()
                .await
        }
        Err(e) => {
            error!("Failed to connect to MongoDB: {:?}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to connect to MongoDB"))
        }
    }
}
