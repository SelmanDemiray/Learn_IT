use actix_web::{web, HttpResponse, Result};

async fn profile(path: web::Path<String>) -> Result<HttpResponse> {
    let username = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Profile for {}", username)))
}

async fn dashboard() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("User dashboard"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/dashboard", web::get().to(dashboard))
            .route("/{username}", web::get().to(profile))
    );
}
