use actix_web::web;
use super::handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/profile", web::get().to(handler::get_profile))
            .route("/progress", web::get().to(handler::get_progress))
    )
    .service(
        web::scope("/api")
            .route("/progress", web::post().to(handler::update_progress))
    );
}
