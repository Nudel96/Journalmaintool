use crate::{error::Result, models::Trade};
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct TradeAnalytics {
    // Basic Metrics
    pub total_trades: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub win_rate: f64,
    
    // P&L Metrics
    pub total_pnl: Decimal,
    pub total_pnl_percentage: f64,
    pub average_win: Decimal,
    pub average_loss: Decimal,
    pub largest_win: Decimal,
    pub largest_loss: Decimal,
    
    // Risk/Reward
    pub profit_factor: f64,
    pub risk_reward_ratio: f64,
    
    // Streaks
    pub current_streak: i32,
    pub longest_win_streak: i32,
    pub longest_loss_streak: i32,
}

#[derive(Debug, Serialize)]
pub struct SymbolPerformance {
    pub symbol: String,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub win_rate: f64,
    pub total_pnl: Decimal,
    pub average_pnl: Decimal,
}

#[derive(Debug, Serialize)]
pub struct SetupPerformance {
    pub setup_type: String,
    pub total_trades: i32,
    pub winning_trades: i32,
    pub win_rate: f64,
    pub total_pnl: Decimal,
    pub average_pnl: Decimal,
}

#[derive(Debug, Serialize)]
pub struct MistakeAnalysis {
    pub mistake: String,
    pub count: i32,
    pub average_pnl: Decimal,
    pub total_pnl: Decimal,
}

pub struct AnalyticsService;

impl AnalyticsService {
    /// Calculate overall trade analytics
    pub fn calculate_overview(trades: &[Trade]) -> Result<TradeAnalytics> {
        let total_trades = trades.len() as i32;
        
        if total_trades == 0 {
            return Ok(TradeAnalytics {
                total_trades: 0,
                winning_trades: 0,
                losing_trades: 0,
                win_rate: 0.0,
                total_pnl: Decimal::ZERO,
                total_pnl_percentage: 0.0,
                average_win: Decimal::ZERO,
                average_loss: Decimal::ZERO,
                largest_win: Decimal::ZERO,
                largest_loss: Decimal::ZERO,
                profit_factor: 0.0,
                risk_reward_ratio: 0.0,
                current_streak: 0,
                longest_win_streak: 0,
                longest_loss_streak: 0,
            });
        }

        let mut winning_trades = 0;
        let mut losing_trades = 0;
        let mut total_pnl = Decimal::ZERO;
        let mut total_wins = Decimal::ZERO;
        let mut total_losses = Decimal::ZERO;
        let mut largest_win = Decimal::ZERO;
        let mut largest_loss = Decimal::ZERO;
        
        let mut current_streak = 0;
        let mut longest_win_streak = 0;
        let mut longest_loss_streak = 0;
        let mut temp_win_streak = 0;
        let mut temp_loss_streak = 0;

        for trade in trades {
            if let Some(pnl) = trade.pnl {
                total_pnl += pnl;
                
                if pnl > Decimal::ZERO {
                    winning_trades += 1;
                    total_wins += pnl;
                    if pnl > largest_win {
                        largest_win = pnl;
                    }
                    
                    temp_win_streak += 1;
                    temp_loss_streak = 0;
                    if temp_win_streak > longest_win_streak {
                        longest_win_streak = temp_win_streak;
                    }
                } else if pnl < Decimal::ZERO {
                    losing_trades += 1;
                    total_losses += pnl.abs();
                    if pnl < largest_loss {
                        largest_loss = pnl;
                    }
                    
                    temp_loss_streak += 1;
                    temp_win_streak = 0;
                    if temp_loss_streak > longest_loss_streak {
                        longest_loss_streak = temp_loss_streak;
                    }
                }
            }
        }

        current_streak = if temp_win_streak > 0 {
            temp_win_streak
        } else {
            -temp_loss_streak
        };

        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        let average_win = if winning_trades > 0 {
            total_wins / Decimal::from(winning_trades)
        } else {
            Decimal::ZERO
        };

        let average_loss = if losing_trades > 0 {
            total_losses / Decimal::from(losing_trades)
        } else {
            Decimal::ZERO
        };

        let profit_factor = if total_losses > Decimal::ZERO {
            (total_wins / total_losses).to_string().parse::<f64>().unwrap_or(0.0)
        } else if total_wins > Decimal::ZERO {
            999.99 // Infinite profit factor
        } else {
            0.0
        };

        let risk_reward_ratio = if average_loss > Decimal::ZERO {
            (average_win / average_loss).to_string().parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        Ok(TradeAnalytics {
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            total_pnl,
            total_pnl_percentage: 0.0, // TODO: Calculate based on account size
            average_win,
            average_loss,
            largest_win,
            largest_loss,
            profit_factor,
            risk_reward_ratio,
            current_streak,
            longest_win_streak,
            longest_loss_streak,
        })
    }

    /// Calculate performance by symbol
    pub fn calculate_by_symbol(trades: &[Trade]) -> Result<Vec<SymbolPerformance>> {
        let mut symbol_map: HashMap<String, Vec<&Trade>> = HashMap::new();

        for trade in trades {
            symbol_map.entry(trade.symbol.clone()).or_insert_with(Vec::new).push(trade);
        }

        let mut results: Vec<SymbolPerformance> = symbol_map
            .into_iter()
            .map(|(symbol, trades)| {
                let total_trades = trades.len() as i32;
                let mut winning_trades = 0;
                let mut total_pnl = Decimal::ZERO;

                for trade in &trades {
                    if let Some(pnl) = trade.pnl {
                        total_pnl += pnl;
                        if pnl > Decimal::ZERO {
                            winning_trades += 1;
                        }
                    }
                }

                let win_rate = if total_trades > 0 {
                    (winning_trades as f64 / total_trades as f64) * 100.0
                } else {
                    0.0
                };

                let average_pnl = if total_trades > 0 {
                    total_pnl / Decimal::from(total_trades)
                } else {
                    Decimal::ZERO
                };

                SymbolPerformance {
                    symbol,
                    total_trades,
                    winning_trades,
                    win_rate,
                    total_pnl,
                    average_pnl,
                }
            })
            .collect();

        results.sort_by(|a, b| b.win_rate.partial_cmp(&a.win_rate).unwrap());

        Ok(results)
    }

    /// Calculate performance by setup type
    pub fn calculate_by_setup(trades: &[Trade]) -> Result<Vec<SetupPerformance>> {
        let mut setup_map: HashMap<String, Vec<&Trade>> = HashMap::new();

        for trade in trades {
            if let Some(setup) = &trade.setup_type {
                setup_map.entry(setup.clone()).or_insert_with(Vec::new).push(trade);
            }
        }

        let mut results: Vec<SetupPerformance> = setup_map
            .into_iter()
            .map(|(setup_type, trades)| {
                let total_trades = trades.len() as i32;
                let mut winning_trades = 0;
                let mut total_pnl = Decimal::ZERO;

                for trade in &trades {
                    if let Some(pnl) = trade.pnl {
                        total_pnl += pnl;
                        if pnl > Decimal::ZERO {
                            winning_trades += 1;
                        }
                    }
                }

                let win_rate = if total_trades > 0 {
                    (winning_trades as f64 / total_trades as f64) * 100.0
                } else {
                    0.0
                };

                let average_pnl = if total_trades > 0 {
                    total_pnl / Decimal::from(total_trades)
                } else {
                    Decimal::ZERO
                };

                SetupPerformance {
                    setup_type,
                    total_trades,
                    winning_trades,
                    win_rate,
                    total_pnl,
                    average_pnl,
                }
            })
            .collect();

        results.sort_by(|a, b| b.win_rate.partial_cmp(&a.win_rate).unwrap());

        Ok(results)
    }

    /// Analyze mistakes and their impact
    pub fn analyze_mistakes(trades: &[Trade]) -> Result<Vec<MistakeAnalysis>> {
        let mut mistake_map: HashMap<String, Vec<Decimal>> = HashMap::new();

        for trade in trades {
            for mistake in &trade.mistakes {
                if let Some(pnl) = trade.pnl {
                    mistake_map.entry(mistake.clone()).or_insert_with(Vec::new).push(pnl);
                }
            }
        }

        let mut results: Vec<MistakeAnalysis> = mistake_map
            .into_iter()
            .map(|(mistake, pnls)| {
                let count = pnls.len() as i32;
                let total_pnl: Decimal = pnls.iter().sum();
                let average_pnl = if count > 0 {
                    total_pnl / Decimal::from(count)
                } else {
                    Decimal::ZERO
                };

                MistakeAnalysis {
                    mistake,
                    count,
                    average_pnl,
                    total_pnl,
                }
            })
            .collect();

        results.sort_by(|a, b| b.count.cmp(&a.count));

        Ok(results)
    }
}

