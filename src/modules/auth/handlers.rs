use actix_web::{web, HttpResponse, Result};
use actix_identity::Identity;
use sqlx::SqlitePool;
use tera::Tera;
use argon2::{self, Config};
use rand::Rng;
use crate::modules::users::models::User;

// Show login page
pub async fn login_page(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Login");
    
    let rendered = tmpl.render("auth/login.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Process login
pub async fn login(
    form: web::Form<LoginForm>,
    db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // In a real app, we would:
    // 1. Validate the form input
    // 2. Check the user credentials against the database
    // 3. Create a session if valid
    
    // Simple implementation for now
    let user_id = "user123"; // Would normally come from database
    
    // This is the updated method for Identity in newer versions
    id.remember(user_id.to_string());
    
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

// Show registration page
pub async fn register_page(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Register");
    
    let rendered = tmpl.render("auth/register.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Process registration
pub async fn register(
    form: web::Form<RegisterForm>,
    db: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    // In a real app, we would:
    // 1. Validate the form data
    // 2. Check if the user already exists
    // 3. Hash the password
    // 4. Insert the new user into the database
    
    Ok(HttpResponse::Found().header("Location", "/auth/login").finish())
}

// Logout
pub async fn logout(id: Identity) -> Result<HttpResponse> {
    id.forget();
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

// Form structures
#[derive(serde::Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}
