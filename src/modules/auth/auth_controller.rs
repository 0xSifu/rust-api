// src/modules/auth/auth_controller.rs
use actix_web::{web, HttpResponse, Responder};
use mongodb::Database;
use crate::modules::auth::auth_service;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    email: String,
    password: String,
}
pub async fn register_user(
    user: web::Json<User>,
    db: web::Data<Database>
) -> impl Responder {
    match auth_service::register_user(&user.email, &user.password, db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register user"),
    }
}

pub async fn authenticate_user(
    user: web::Json<User>,
    db: web::Data<Database>
) -> impl Responder {
    match auth_service::authenticate_user(&user.email, &user.password, db.get_ref()).await {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
