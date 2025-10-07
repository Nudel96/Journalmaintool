use crate::{error::AppError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub email: String,
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
}

/// Generate JWT token for user
pub fn generate_token(user_id: Uuid, email: &str, secret: &str, expiration_hours: i64) -> Result<String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(expiration_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError("Failed to generate token".to_string()))
}

/// Verify and decode JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

