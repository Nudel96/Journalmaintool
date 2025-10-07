pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;

pub use config::Config;
pub use error::{AppError, Result};

use sqlx::PgPool;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

