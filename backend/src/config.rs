use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "JWT_SECRET must be set".to_string())?;

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .map_err(|_| "JWT_EXPIRATION_HOURS must be a valid number".to_string())?;

        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "SERVER_PORT must be a valid number".to_string())?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiration_hours,
            server_host,
            server_port,
        })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.jwt_secret.len() < 32 {
            return Err("JWT_SECRET must be at least 32 characters long".to_string());
        }

        if self.jwt_expiration_hours < 1 {
            return Err("JWT_EXPIRATION_HOURS must be at least 1".to_string());
        }

        Ok(())
    }
}

