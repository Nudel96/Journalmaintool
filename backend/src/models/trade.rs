use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Trade model from database
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Trade {
    pub id: Uuid,
    pub user_id: Uuid,
    
    // Trade Details
    pub symbol: String,
    pub direction: String, // "long" or "short"
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub quantity: Decimal,
    
    // Timestamps
    pub entry_time: DateTime<Utc>,
    pub exit_time: Option<DateTime<Utc>>,
    
    // P&L
    pub pnl: Option<Decimal>,
    pub pnl_percentage: Option<Decimal>,
    pub fees: Decimal,
    
    // Metadata
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub setup_type: Option<String>,
    pub mistakes: Vec<String>,
    pub emotions: Vec<String>,
    pub screenshots: Vec<String>,
    
    // Additional Fields
    pub broker: Option<String>,
    pub account_id: Option<String>,
    pub status: String, // "open", "closed", "pending"
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create trade request
#[derive(Debug, Deserialize)]
pub struct CreateTradeRequest {
    pub symbol: String,
    pub direction: String,
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub quantity: Decimal,
    pub entry_time: DateTime<Utc>,
    pub exit_time: Option<DateTime<Utc>>,
    pub fees: Option<Decimal>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    pub setup_type: Option<String>,
    pub mistakes: Option<Vec<String>>,
    pub emotions: Option<Vec<String>>,
    pub broker: Option<String>,
    pub account_id: Option<String>,
}

/// Update trade request
#[derive(Debug, Deserialize)]
pub struct UpdateTradeRequest {
    pub symbol: Option<String>,
    pub direction: Option<String>,
    pub entry_price: Option<Decimal>,
    pub exit_price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub entry_time: Option<DateTime<Utc>>,
    pub exit_time: Option<DateTime<Utc>>,
    pub fees: Option<Decimal>,
    pub notes: Option<String>,
    pub tags: Option<Vec<String>>,
    pub setup_type: Option<String>,
    pub mistakes: Option<Vec<String>>,
    pub emotions: Option<Vec<String>>,
    pub broker: Option<String>,
    pub account_id: Option<String>,
    pub status: Option<String>,
}

/// Trade list filters
#[derive(Debug, Deserialize, Default)]
pub struct TradeFilters {
    pub symbol: Option<String>,
    pub direction: Option<String>,
    pub status: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub setup_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Trade {
    /// Calculate P&L for a trade
    pub fn calculate_pnl(&self) -> Option<(Decimal, Decimal)> {
        if let Some(exit_price) = self.exit_price {
            let price_diff = if self.direction == "long" {
                exit_price - self.entry_price
            } else {
                self.entry_price - exit_price
            };
            
            let pnl = price_diff * self.quantity - self.fees;
            let pnl_percentage = (price_diff / self.entry_price) * Decimal::from(100);
            
            Some((pnl, pnl_percentage))
        } else {
            None
        }
    }
}

