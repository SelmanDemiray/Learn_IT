use actix_web::web;
use super::handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::get().to(handler::get_courses))
            .route("/", web::get().to(handler::get_courses))
    );
}
