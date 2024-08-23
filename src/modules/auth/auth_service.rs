use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};
use log::{info, error};
use mongodb::Database; // Ensure you have the mongodb crate dependency
use crate::models::schema::employee::Employee; // Ensure this path is correct for your project structure
use bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn register_user(email: &str, password: &str, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    info!("Registering user with email: {}", email);

    // Check if user already exists
    let collection = db.collection::<Employee>("employees");
    let existing_user = collection.find_one(doc! { "email": email }, None).await?;

    if existing_user.is_some() {
        error!("User already exists with email: {}", email);
        return Err("User already exists".into());
    }

    // Hash the password
    let hashed_password = hash(password, DEFAULT_COST)?;

    // Create a new employee instance
    let new_employee = Employee {
        id: None,
        name: String::new(), // Or collect this from the registration form
        email: email.to_string(),
        password: hashed_password,
        bio: None,
        created_at: Some(Utc::now()),
    };

    // Insert the new employee into the database
    collection.insert_one(new_employee, None).await?;
    info!("User registered successfully with email: {}", email);

    Ok(())
}

pub async fn authenticate_user(email: &str, password: &str, db: &Database) -> Result<String, Box<dyn std::error::Error>> {
    let collection = db.collection::<Employee>("employees");

    // Find user by email
    let user = collection.find_one(doc! { "email": email }, None).await?;

    if let Some(user) = user {
        // Verify password
        if verify(password, &user.password)? {
            let claims = Claims {
                sub: email.to_string(),
                exp: (Utc::now() + Duration::hours(1)).timestamp() as usize, // 1 hour expiration
            };

            let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;

            info!("User authenticated successfully: {}", email);
            Ok(token)
        } else {
            info!("Authentication failed for user: {}", email);
            Err("Authentication failed".into())
        }
    } else {
        info!("User not found: {}", email);
        Err("User not found".into())
    }
}
