use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Create a PostgreSQL connection pool
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    tracing::info!("✅ Database connection established");

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Running database migrations...");

    // Read and execute migration files
    let migrations_dir = std::path::Path::new("migrations");
    
    if !migrations_dir.exists() {
        tracing::warn!("Migrations directory not found, skipping migrations");
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)
        .map_err(|e| sqlx::Error::Io(e))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "sql")
                .unwrap_or(false)
        })
        .collect();

    // Sort by filename to ensure correct order
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        tracing::info!("Running migration: {}", filename);
        
        let sql = std::fs::read_to_string(&path)
            .map_err(|e| sqlx::Error::Io(e))?;
        
        sqlx::raw_sql(&sql).execute(pool).await?;
        
        tracing::info!("✅ Migration completed: {}", filename);
    }

    tracing::info!("✅ All migrations completed");

    Ok(())
}

