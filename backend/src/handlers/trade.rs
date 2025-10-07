use crate::{
    error::{AppError, Result},
    middleware::AuthUser,
    models::{CreateTradeRequest, Trade, TradeFilters, UpdateTradeRequest},
    repositories::TradeRepository,
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

/// Create a new trade
pub async fn create_trade(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateTradeRequest>,
) -> Result<Json<Trade>> {
    // Validate direction
    if payload.direction != "long" && payload.direction != "short" {
        return Err(AppError::ValidationError(
            "Direction must be 'long' or 'short'".to_string(),
        ));
    }

    let trade_repo = TradeRepository::new(state.db.clone());
    let trade = trade_repo.create(user_id, payload).await?;

    Ok(Json(trade))
}

/// Get trade by ID
pub async fn get_trade(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(trade_id): Path<Uuid>,
) -> Result<Json<Trade>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    
    let trade = trade_repo
        .get(trade_id, user_id)
        .await?
        .ok_or(AppError::ValidationError("Trade not found".to_string()))?;

    Ok(Json(trade))
}

/// List trades with filters
pub async fn list_trades(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(filters): Query<TradeFilters>,
) -> Result<Json<Vec<Trade>>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    let trades = trade_repo.list(user_id, filters).await?;

    Ok(Json(trades))
}

/// Update trade
pub async fn update_trade(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(trade_id): Path<Uuid>,
    Json(payload): Json<UpdateTradeRequest>,
) -> Result<Json<Trade>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    let trade = trade_repo.update(trade_id, user_id, payload).await?;

    Ok(Json(trade))
}

/// Delete trade
pub async fn delete_trade(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(trade_id): Path<Uuid>,
) -> Result<StatusCode> {
    let trade_repo = TradeRepository::new(state.db.clone());
    trade_repo.delete(trade_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

