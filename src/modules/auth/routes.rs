use actix_web::web;
use crate::modules::auth::handler::{login_page, login_user, logout_user, register_page, register};

// Configure auth routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")  // Add a scope for auth routes
            .service(
                web::resource("/login")
                    .route(web::get().to(login_page))
                    .route(web::post().to(login_user))
            )
            .service(web::resource("/logout").route(web::get().to(logout_user)))
            .service(
                web::resource("/register")
                    .route(web::get().to(register_page))
                    .route(web::post().to(register))
            )
    );
}
