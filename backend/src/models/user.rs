use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User model from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip_serializing)]
    pub password_hash: String,
    
    // Stripe Integration
    pub stripe_customer_id: Option<String>,
    pub subscription_status: String,
    pub subscription_tier: String,
    pub subscription_interval: Option<String>,
    
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// User response (without sensitive data)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub subscription_status: String,
    pub subscription_tier: String,
    pub subscription_interval: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            subscription_status: user.subscription_status,
            subscription_tier: user.subscription_tier,
            subscription_interval: user.subscription_interval,
            created_at: user.created_at,
        }
    }
}

/// Create user request
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Auth response with token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

