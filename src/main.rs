use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
// Update imports
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use tera::Tera;

mod routes;
mod modules;
mod config;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    // Load environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".to_string());
    
    // Secret key for cookies/sessions - must be at least 64 characters long
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| {
        // Fallback to a long, secure key (at least 64 chars)
        "xKYxsEm29T8XnJQXudM7CmzNBpos2q5bLcGFE1JNyAjf6uBAmCe4Z8Pw3t5kSRdDVvHg".to_string()
    });
    
    // Ensure database directory exists
    if let Some(db_path) = database_url.strip_prefix("sqlite:") {
        let path = std::path::Path::new(db_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                log::warn!("Failed to create database directory: {}", e);
            });
        }
    }
    
    // Set up the database connection pool
    let pool = match SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                log::info!("Database connection successful");
                pool
            },
            Err(e) => {
                log::error!("Failed to create database pool: {}", e);
                // Create a dummy in-memory SQLite database as fallback
                SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect("sqlite::memory:")
                    .await
                    .expect("Failed to create in-memory database")
            }
        };
    
    // Set up Tera templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Template parsing error: {}", e);
            Tera::default()
        }
    };
    
    log::info!("Starting server at http://{}:{}", host, port);
    
    // Start the HTTP server
    HttpServer::new(move || {
        // Initialize middleware and templates
        // Create key from the secret - must be at least 32 bytes for cookie-0.16.2
        let key = Key::from(secret_key.as_bytes());
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            // Add session middleware first (important for correct order)
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                key.clone()
            ))
            // Then add identity middleware
            .wrap(IdentityMiddleware::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            // Configure routes
            .configure(crate::routes::configure)
            // Static files
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
