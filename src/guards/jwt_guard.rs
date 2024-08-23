use actix_web::{Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use log::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
#[allow(dead_code)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            match auth_header.to_str() {
                Ok(auth_str) => {
                    let token = auth_str.trim_start_matches("Bearer ");
                    match std::env::var("JWT_SECRET") {
                        Ok(secret) => {
                            match decode::<Claims>(
                                token,
                                &DecodingKey::from_secret(secret.as_ref()),
                                &Validation::default(),
                            ) {
                                Ok(token_data) => {
                                    return ready(Ok(AuthenticatedUser {
                                        user_id: token_data.claims.sub,
                                    }));
                                }
                                Err(e) => {
                                    error!("Token decoding error: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("JWT_SECRET environment variable error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Authorization header to_str error: {:?}", e);
                }
            }
        } else {
            error!("Authorization header missing");
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
    }
}