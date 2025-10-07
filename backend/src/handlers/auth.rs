use crate::{
    auth::generate_token,
    error::{AppError, Result},
    middleware::AuthUser,
    models::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse},
    repositories::UserRepository,
    AppState,
};
use axum::{extract::State, Json};

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    if payload.name.trim().is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".to_string()));
    }

    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err(AppError::ValidationError("Invalid email address".to_string()));
    }

    if payload.password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Create user repository
    let user_repo = UserRepository::new(state.db.clone());

    // Create user
    let user = user_repo
        .create(&payload.name, &payload.email, &payload.password)
        .await?;

    // Generate JWT token
    let token = generate_token(
        user.id,
        &user.email,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Login user
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    // Validate input
    if payload.email.trim().is_empty() {
        return Err(AppError::ValidationError("Email cannot be empty".to_string()));
    }

    if payload.password.is_empty() {
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }

    // Create user repository
    let user_repo = UserRepository::new(state.db.clone());

    // Find user by email
    let user = user_repo
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let is_valid = user_repo.verify_password(&payload.password, &user.password_hash)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Update last login
    user_repo.update_last_login(user.id).await?;

    // Generate JWT token
    let token = generate_token(
        user.id,
        &user.email,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Get current user (requires authentication)
pub async fn me(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>> {
    let user_repo = UserRepository::new(state.db.clone());

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    Ok(Json(user.into()))
}

