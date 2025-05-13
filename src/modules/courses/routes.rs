use actix_web::{web, HttpResponse, Result};

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Courses index"))
}

async fn show(path: web::Path<String>) -> Result<HttpResponse> {
    let id = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Course details for {}", id)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::get().to(index))
            .route("/{id}", web::get().to(show))
    );
}
