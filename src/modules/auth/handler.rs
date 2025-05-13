use actix_web::{web, HttpResponse, Result, HttpRequest, Error};
use actix_identity::Identity;
use sqlx::SqlitePool;
use tera::Tera;
use serde::Deserialize;
use uuid::Uuid;
use actix_web::HttpMessage;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

// Function names must match exactly what's being imported in routes.rs
pub async fn login_page(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Login");
    
    let rendered = tmpl.render("auth/login.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn login_user(
    req: HttpRequest,
    form: web::Form<LoginForm>,
    _db: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    // In a real app, you would verify credentials against the database
    // For demo purposes, allow login with any credentials
    
    if form.username.is_empty() || form.password.is_empty() {
        return Ok(HttpResponse::Found()
            .append_header(("LOCATION", "/auth/login?error=invalid_credentials"))
            .finish());
    }
    
    let user_id = Uuid::new_v4().to_string();
    
    Identity::login(&req.extensions(), user_id.to_string())?;
    
    Ok(HttpResponse::Found().append_header(("LOCATION", "/")).finish())
}

pub async fn register_page(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Register");
    
    let rendered = tmpl.render("auth/register.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn register(
    form: web::Form<RegisterForm>,
    _db: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    // Validate inputs
    if form.username.is_empty() || form.email.is_empty() || 
       form.password.is_empty() || form.confirm_password.is_empty() {
        return Ok(HttpResponse::Found()
            .append_header(("LOCATION", "/auth/register?error=missing_fields"))
            .finish());
    }
    
    // Check if passwords match
    if form.password != form.confirm_password {
        return Ok(HttpResponse::Found()
            .append_header(("LOCATION", "/auth/register?error=password_mismatch"))
            .finish());
    }
    
    // For demo purposes, just redirect to login
    Ok(HttpResponse::Found()
        .append_header(("LOCATION", "/auth/login?registered=true"))
        .finish())
}

pub async fn logout_user(
    id: Identity,
) -> Result<HttpResponse, Error> {
    id.logout();
    
    Ok(HttpResponse::Found().append_header(("LOCATION", "/")).finish())
}
