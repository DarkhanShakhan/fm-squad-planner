use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::time::Duration;

/// Initialize the database connection pool
pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    log::info!("Connecting to database at: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await?;

    log::info!("Database connection pool established");

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Running database migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    log::info!("Database migrations completed successfully");

    Ok(())
}

/// Initialize database (create pool and run migrations)
pub async fn initialize(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = init_pool(database_url).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}
