use actix_web::{web, HttpResponse, Result};

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Lessons index"))
}

async fn level(path: web::Path<String>) -> Result<HttpResponse> {
    let level = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Lessons for {} level", level)))
}

async fn show(path: web::Path<(String, String)>) -> Result<HttpResponse> {
    let (level, id) = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("Lesson {} for {} level", id, level)))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lessons")
            .route("", web::get().to(index))
            .route("/{level}", web::get().to(level))
            .route("/{level}/{id}", web::get().to(show))
    );
}
