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
    
    // Create templates directory if it doesn't exist
    std::fs::create_dir_all("templates").ok();
    
    // Set up Tera templates with better error handling
    let tera = {
        let template_paths = vec![
            "templates/**/*.html",
            "/app/templates/**/*.html", 
            "./templates/**/*.html",
        ];
        
        let mut tera_instance = None;
        
        // First check if templates directory exists
        let template_dirs = vec!["templates", "/app/templates", "./templates"];
        for template_dir in &template_dirs {
            if std::path::Path::new(template_dir).exists() {
                log::info!("Found template directory: {}", template_dir);
                if let Ok(entries) = std::fs::read_dir(template_dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            log::info!("Template file: {:?}", entry.path());
                        }
                    }
                }
                break;
            }
        }
        
        // Try to load templates with error handling
        for path in template_paths {
            match Tera::new(path) {
                Ok(t) => {
                    log::info!("Templates loaded successfully from: {}", path);
                    let template_names: Vec<_> = t.get_template_names().collect();
                    log::info!("Found {} templates: {:?}", template_names.len(), template_names);
                    tera_instance = Some(t);
                    break;
                },
                Err(e) => {
                    log::warn!("Failed to load templates from {}: {}", path, e);
                }
            }
        }
        
        tera_instance.unwrap_or_else(|| {
            log::error!("No templates found in any path, creating minimal template set");
            let mut t = Tera::default();
            
            // Add essential templates inline as fallback
            let base_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}IT Learning Platform{% endblock %}</title>
    <link rel="stylesheet" href="/static/css/style.css">
</head>
<body>
    <header>
        <nav>
            <div class="logo"><a href="/">IT Learning</a></div>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/courses">Courses</a></li>
                <li><a href="/lessons">Lessons</a></li>
                <li><a href="/about">About</a></li>
            </ul>
        </nav>
    </header>
    <main>{% block content %}{% endblock %}</main>
    <footer><p>&copy; 2024 IT Learning Platform</p></footer>
</body>
</html>"#;

            let index_template = r#"{% extends "base.html" %}
{% block title %}Home - IT Learning Platform{% endblock %}
{% block content %}
<div style="text-align: center; padding: 50px;">
    <h1>Learn IT from Beginner to Expert</h1>
    <p>A comprehensive platform for mastering Information Technology skills</p>
    <div style="margin-top: 30px;">
        <a href="/courses" style="padding: 12px 24px; background: #3498db; color: white; text-decoration: none; border-radius: 5px; margin: 0 10px;">Browse Courses</a>
        <a href="/lessons" style="padding: 12px 24px; background: #6c757d; color: white; text-decoration: none; border-radius: 5px; margin: 0 10px;">View All Lessons</a>
    </div>
</div>
{% endblock %}"#;

            let about_template = r#"{% extends "base.html" %}
{% block title %}About - IT Learning Platform{% endblock %}
{% block content %}
<div style="max-width: 800px; margin: 0 auto; padding: 40px 20px;">
    <h1>About IT Learning Platform</h1>
    <p>This is a comprehensive learning platform for Information Technology skills.</p>
    <h2>Features</h2>
    <ul>
        <li>Structured learning paths</li>
        <li>Multiple difficulty levels</li>
        <li>Hands-on exercises</li>
        <li>Progress tracking</li>
    </ul>
</div>
{% endblock %}"#;

            let roadmap_template = r#"{% extends "base.html" %}
{% block title %}Learning Roadmap - IT Learning Platform{% endblock %}
{% block content %}
<div style="max-width: 1200px; margin: 0 auto; padding: 40px 20px;">
    <h1>IT Learning Roadmap</h1>
    <p>Your personalized journey from beginner to expert</p>
    <div>
        {% for path in roadmap.paths %}
        <div style="margin: 30px 0; padding: 20px; border: 1px solid #ddd; border-radius: 8px;">
            <h2>{{ path.name }}</h2>
            <p>{{ path.description }}</p>
            <p><strong>Duration:</strong> {{ path.duration }}</p>
            <div>
                {% for step in path.steps %}
                <div style="margin: 15px 0; padding: 15px; background: #f8f9fa; border-radius: 5px;">
                    <h3>{{ step.title }}</h3>
                    <p>{{ step.description }}</p>
                    <p><small>Duration: {{ step.duration }}</small></p>
                </div>
                {% endfor %}
            </div>
        </div>
        {% endfor %}
    </div>
</div>
{% endblock %}"#;

            if let Err(e) = t.add_raw_template("base.html", base_template) {
                log::error!("Failed to add base template: {}", e);
            }
            if let Err(e) = t.add_raw_template("index.html", index_template) {
                log::error!("Failed to add index template: {}", e);
            }
            if let Err(e) = t.add_raw_template("about.html", about_template) {
                log::error!("Failed to add about template: {}", e);
            }
            if let Err(e) = t.add_raw_template("roadmap.html", roadmap_template) {
                log::error!("Failed to add roadmap template: {}", e);
            }
            
            log::info!("Created minimal template set with embedded templates");
            t
        })
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
