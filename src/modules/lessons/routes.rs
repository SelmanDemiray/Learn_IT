use actix_web::web;
use crate::modules::lessons::handler::{get_lessons, get_lessons_by_level, get_lesson};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lessons")
            .route("", web::get().to(get_lessons))
            .route("/{level}", web::get().to(get_lessons_by_level))
            .route("/{level}/{id}", web::get().to(get_lesson))
    );
}
