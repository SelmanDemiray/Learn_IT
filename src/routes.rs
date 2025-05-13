use actix_web::{web, HttpResponse, Result};
use tera::Tera;

// Import from crate::modules instead of declaring local modules
use crate::modules::auth;
use crate::modules::courses;
use crate::modules::lessons;
use crate::modules::users;

// Main index function
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Home");
    
    let rendered = tmpl.render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// About page
async fn about(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "About");
    
    let rendered = tmpl.render("about.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Configure all routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
       .service(web::resource("/about").to(about))
       .configure(auth::routes::configure)
       .configure(courses::routes::configure)
       .configure(lessons::routes::configure)
       .configure(users::routes::configure);
}
