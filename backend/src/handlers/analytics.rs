use crate::{
    error::Result,
    middleware::AuthUser,
    models::TradeFilters,
    repositories::TradeRepository,
    services::{AnalyticsService, MistakeAnalysis, SetupPerformance, SymbolPerformance, TradeAnalytics},
    AppState,
};
use axum::{extract::State, Json};

/// Get overall analytics
pub async fn get_overview(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<TradeAnalytics>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    
    // Get all closed trades
    let filters = TradeFilters {
        status: Some("closed".to_string()),
        ..Default::default()
    };
    
    let trades = trade_repo.list(user_id, filters).await?;
    let analytics = AnalyticsService::calculate_overview(&trades)?;

    Ok(Json(analytics))
}

/// Get performance by symbol
pub async fn get_by_symbol(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<SymbolPerformance>>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    
    let filters = TradeFilters {
        status: Some("closed".to_string()),
        ..Default::default()
    };
    
    let trades = trade_repo.list(user_id, filters).await?;
    let performance = AnalyticsService::calculate_by_symbol(&trades)?;

    Ok(Json(performance))
}

/// Get performance by setup type
pub async fn get_by_setup(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<SetupPerformance>>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    
    let filters = TradeFilters {
        status: Some("closed".to_string()),
        ..Default::default()
    };
    
    let trades = trade_repo.list(user_id, filters).await?;
    let performance = AnalyticsService::calculate_by_setup(&trades)?;

    Ok(Json(performance))
}

/// Get mistake analysis
pub async fn get_mistakes(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<MistakeAnalysis>>> {
    let trade_repo = TradeRepository::new(state.db.clone());
    
    let filters = TradeFilters {
        status: Some("closed".to_string()),
        ..Default::default()
    };
    
    let trades = trade_repo.list(user_id, filters).await?;
    let mistakes = AnalyticsService::analyze_mistakes(&trades)?;

    Ok(Json(mistakes))
}

