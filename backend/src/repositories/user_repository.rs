use crate::{
    error::{AppError, Result},
    models::User,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(&self, name: &str, email: &str, password: &str) -> Result<User> {
        // Hash password
        let password_hash = self.hash_password(password)?;

        // Insert user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(email)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::UserAlreadyExists
            }
            _ => AppError::DatabaseError(e),
        })?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Update last login timestamp
    pub async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users SET last_login = NOW() WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Verify password
    pub fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|_| AppError::InvalidCredentials)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Hash password using Argon2
    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))?
            .to_string();

        Ok(password_hash)
    }
}

