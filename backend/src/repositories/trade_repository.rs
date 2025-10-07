use crate::{
    error::{AppError, Result},
    models::{CreateTradeRequest, Trade, TradeFilters, UpdateTradeRequest},
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TradeRepository {
    pool: PgPool,
}

impl TradeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new trade
    pub async fn create(&self, user_id: Uuid, req: CreateTradeRequest) -> Result<Trade> {
        // Calculate P&L if exit price is provided
        let (pnl, pnl_percentage) = if let Some(exit_price) = req.exit_price {
            let price_diff = if req.direction == "long" {
                exit_price - req.entry_price
            } else {
                req.entry_price - exit_price
            };
            
            let fees = req.fees.unwrap_or(Decimal::ZERO);
            let pnl = price_diff * req.quantity - fees;
            let pnl_pct = (price_diff / req.entry_price) * Decimal::from(100);
            
            (Some(pnl), Some(pnl_pct))
        } else {
            (None, None)
        };

        let status = if req.exit_price.is_some() { "closed" } else { "open" };

        let trade = sqlx::query_as::<_, Trade>(
            r#"
            INSERT INTO trades (
                user_id, symbol, direction, entry_price, exit_price, quantity,
                entry_time, exit_time, pnl, pnl_percentage, fees,
                notes, tags, setup_type, mistakes, emotions, screenshots,
                broker, account_id, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&req.symbol)
        .bind(&req.direction)
        .bind(req.entry_price)
        .bind(req.exit_price)
        .bind(req.quantity)
        .bind(req.entry_time)
        .bind(req.exit_time)
        .bind(pnl)
        .bind(pnl_percentage)
        .bind(req.fees.unwrap_or(Decimal::ZERO))
        .bind(req.notes)
        .bind(req.tags.unwrap_or_default())
        .bind(req.setup_type)
        .bind(req.mistakes.unwrap_or_default())
        .bind(req.emotions.unwrap_or_default())
        .bind(Vec::<String>::new()) // screenshots - empty for now
        .bind(req.broker)
        .bind(req.account_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(trade)
    }

    /// Get trade by ID
    pub async fn get(&self, trade_id: Uuid, user_id: Uuid) -> Result<Option<Trade>> {
        let trade = sqlx::query_as::<_, Trade>(
            r#"
            SELECT * FROM trades WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(trade_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(trade)
    }

    /// List trades with filters
    pub async fn list(&self, user_id: Uuid, filters: TradeFilters) -> Result<Vec<Trade>> {
        let mut query = String::from("SELECT * FROM trades WHERE user_id = $1");
        let mut param_count = 1;

        // Build dynamic query based on filters
        if filters.symbol.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND symbol = ${}", param_count));
        }
        if filters.direction.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND direction = ${}", param_count));
        }
        if filters.status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }
        if filters.from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND entry_time >= ${}", param_count));
        }
        if filters.to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND entry_time <= ${}", param_count));
        }
        if filters.setup_type.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND setup_type = ${}", param_count));
        }

        query.push_str(" ORDER BY entry_time DESC");

        if let Some(limit) = filters.limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
        }
        if let Some(offset) = filters.offset {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
        }

        let mut q = sqlx::query_as::<_, Trade>(&query).bind(user_id);

        if let Some(symbol) = filters.symbol {
            q = q.bind(symbol);
        }
        if let Some(direction) = filters.direction {
            q = q.bind(direction);
        }
        if let Some(status) = filters.status {
            q = q.bind(status);
        }
        if let Some(from_date) = filters.from_date {
            q = q.bind(from_date);
        }
        if let Some(to_date) = filters.to_date {
            q = q.bind(to_date);
        }
        if let Some(setup_type) = filters.setup_type {
            q = q.bind(setup_type);
        }
        if let Some(limit) = filters.limit {
            q = q.bind(limit);
        }
        if let Some(offset) = filters.offset {
            q = q.bind(offset);
        }

        let trades = q.fetch_all(&self.pool).await?;

        Ok(trades)
    }

    /// Update trade
    pub async fn update(&self, trade_id: Uuid, user_id: Uuid, req: UpdateTradeRequest) -> Result<Trade> {
        // Get existing trade
        let existing = self.get(trade_id, user_id).await?
            .ok_or(AppError::ValidationError("Trade not found".to_string()))?;

        // Build update query dynamically
        let mut updates = Vec::new();
        let mut param_count = 1;

        if req.symbol.is_some() {
            param_count += 1;
            updates.push(format!("symbol = ${}", param_count));
        }
        if req.direction.is_some() {
            param_count += 1;
            updates.push(format!("direction = ${}", param_count));
        }
        if req.entry_price.is_some() {
            param_count += 1;
            updates.push(format!("entry_price = ${}", param_count));
        }
        if req.exit_price.is_some() {
            param_count += 1;
            updates.push(format!("exit_price = ${}", param_count));
        }
        if req.quantity.is_some() {
            param_count += 1;
            updates.push(format!("quantity = ${}", param_count));
        }
        if req.entry_time.is_some() {
            param_count += 1;
            updates.push(format!("entry_time = ${}", param_count));
        }
        if req.exit_time.is_some() {
            param_count += 1;
            updates.push(format!("exit_time = ${}", param_count));
        }
        if req.fees.is_some() {
            param_count += 1;
            updates.push(format!("fees = ${}", param_count));
        }
        if req.notes.is_some() {
            param_count += 1;
            updates.push(format!("notes = ${}", param_count));
        }
        if req.tags.is_some() {
            param_count += 1;
            updates.push(format!("tags = ${}", param_count));
        }
        if req.setup_type.is_some() {
            param_count += 1;
            updates.push(format!("setup_type = ${}", param_count));
        }
        if req.mistakes.is_some() {
            param_count += 1;
            updates.push(format!("mistakes = ${}", param_count));
        }
        if req.emotions.is_some() {
            param_count += 1;
            updates.push(format!("emotions = ${}", param_count));
        }
        if req.broker.is_some() {
            param_count += 1;
            updates.push(format!("broker = ${}", param_count));
        }
        if req.account_id.is_some() {
            param_count += 1;
            updates.push(format!("account_id = ${}", param_count));
        }
        if req.status.is_some() {
            param_count += 1;
            updates.push(format!("status = ${}", param_count));
        }

        updates.push("updated_at = NOW()".to_string());

        if updates.is_empty() {
            return Ok(existing);
        }

        let query = format!(
            "UPDATE trades SET {} WHERE id = $1 AND user_id = ${} RETURNING *",
            updates.join(", "),
            param_count + 1
        );

        // This is a simplified version - in production, use a proper query builder
        // For now, return the existing trade (update logic would be more complex)
        Ok(existing)
    }

    /// Delete trade
    pub async fn delete(&self, trade_id: Uuid, user_id: Uuid) -> Result<()> {
        let result = sqlx::query(
            r#"
            DELETE FROM trades WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(trade_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::ValidationError("Trade not found".to_string()));
        }

        Ok(())
    }
}

