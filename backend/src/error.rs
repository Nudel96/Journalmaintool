use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// Custom error type for the application
#[derive(Debug)]
pub enum AppError {
    // Database errors
    DatabaseError(sqlx::Error),
    
    // Authentication errors
    InvalidCredentials,
    InvalidToken,
    TokenExpired,
    Unauthorized,
    
    // Validation errors
    ValidationError(String),
    
    // User errors
    UserAlreadyExists,
    UserNotFound,
    
    // Internal errors
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::InvalidCredentials => write!(f, "Invalid email or password"),
            AppError::InvalidToken => write!(f, "Invalid authentication token"),
            AppError::TokenExpired => write!(f, "Authentication token has expired"),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::UserAlreadyExists => write!(f, "User with this email already exists"),
            AppError::UserNotFound => write!(f, "User not found"),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_string(),
                Some("Email or password is incorrect".to_string()),
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
                None,
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Token expired".to_string(),
                None,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
                None,
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                Some(msg),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "User already exists".to_string(),
                Some("A user with this email already exists".to_string()),
            ),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "User not found".to_string(),
                None,
            ),
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
        };

        let body = Json(ErrorResponse {
            error: error_message,
            details,
        });

        (status, body).into_response()
    }
}

// Implement From traits for automatic conversion
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AppError::InvalidCredentials
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AppError>;

