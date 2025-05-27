use actix_web::web;
use super::handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/profile", web::get().to(handler::get_profile))
            .route("/progress", web::get().to(handler::get_progress))
            .route("/roadmap", web::get().to(handler::get_roadmap))
            .route("/garden", web::get().to(handler::get_flower_garden))
    )
    .service(
        web::scope("/api")
            .route("/progress", web::post().to(handler::update_progress))
            .route("/user-progress", web::get().to(handler::get_user_progress_api))
    );
}
