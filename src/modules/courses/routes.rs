use actix_web::web;
use crate::modules::courses::handler::{get_courses, get_course};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::get().to(get_courses))
            .route("/{id}", web::get().to(get_course))
    );
}
