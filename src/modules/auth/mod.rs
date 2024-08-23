mod auth_service;
pub(crate) mod auth_controller;
use actix_web::{web};
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        .route("/login", web::post().to(auth_controller::authenticate_user))
        .route("/register", web::post().to(auth_controller::register_user)));
}
