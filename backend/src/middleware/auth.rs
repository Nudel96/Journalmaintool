use crate::{auth::verify_token, error::AppError, AppState};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// Extract user ID from JWT token in Authorization header
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Get Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    // Extract token from "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::InvalidToken)?;

    // Verify token
    let claims = verify_token(token, &state.config.jwt_secret)?;

    // Parse user ID from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidToken)?;

    // Insert user ID into request extensions
    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}

/// Extractor for authenticated user ID
pub struct AuthUser(pub Uuid);

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Uuid>()
            .copied()
            .map(AuthUser)
            .ok_or(AppError::Unauthorized)
    }
}

