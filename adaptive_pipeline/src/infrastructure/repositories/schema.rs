// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! SQLite schema helpers shared by repository implementations.
//!
//! Applies migrations on start-up so integration tests and services see a
//! consistent database.

use sqlx::migrate::MigrateDatabase;
use sqlx::SqlitePool;
use tracing::{ debug, info };

/// Runs pending migrations against the provided SQLite pool.
pub async fn ensure_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    debug!("Ensuring database schema is up to date");

    // Run migrations - sqlx will automatically track what's been applied
    sqlx::migrate!("../migrations").run(pool).await?;

    info!("Database schema is up to date");
    Ok(())
}

/// Creates a new SQLite database file if it doesn't exist
///
/// This function is useful for ensuring the database file exists before
/// attempting to connect. SQLitePool::connect() will fail if the file
/// doesn't exist unless using SqliteConnectOptions with create_if_missing.
///
/// # Arguments
///
/// * `database_url` - SQLite connection URL (e.g., "sqlite://path/to/db.db")
///
/// # Returns
///
/// * `Ok(())` - Database exists or was created successfully
/// * `Err(sqlx::Error)` - Failed to create database
///
/// # Example
///
/// ```rust,no_run
/// # use pipeline::infrastructure::repositories::schema;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// schema::create_database_if_missing("sqlite://./pipeline.db").await?;
/// # Ok(())
/// # }
/// ```
pub async fn create_database_if_missing(database_url: &str) -> Result<(), sqlx::Error> {
    if !sqlx::Sqlite::database_exists(database_url).await? {
        debug!("Database does not exist, creating: {}", database_url);
        sqlx::Sqlite::create_database(database_url).await?;
        info!("Created new SQLite database: {}", database_url);
    } else {
        debug!("Database already exists: {}", database_url);
    }
    Ok(())
}

/// Initializes a new database with schema (convenience function)
///
/// This is a high-level function that combines database creation and
/// schema migration in one call. Perfect for application startup.
///
/// # Arguments
///
/// * `database_url` - SQLite connection URL
///
/// # Returns
///
/// * `Ok(SqlitePool)` - Connected pool with schema initialized
/// * `Err(sqlx::Error)` - Initialization failed
///
/// # Example
///
/// ```rust,no_run
/// # use pipeline::infrastructure::repositories::schema;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let pool = schema::initialize_database("sqlite://./pipeline.db").await?;
/// // Database is ready to use!
/// # Ok(())
/// # }
/// ```
pub async fn initialize_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Create database if it doesn't exist
    create_database_if_missing(database_url).await?;

    // Connect to database
    let pool = SqlitePool::connect(database_url).await?;

    // Run migrations
    ensure_schema(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_create_database_if_missing() {
        let temp = NamedTempFile::new().unwrap();
        let db_path = temp.path().to_str().unwrap();
        let db_url = format!("sqlite://{}", db_path);

        // Remove temp file so we can test creation
        drop(temp);

        // Should create the database
        create_database_if_missing(&db_url).await.unwrap();

        // Should succeed if already exists
        create_database_if_missing(&db_url).await.unwrap();
    }

    #[tokio::test]
    async fn test_initialize_database() {
        let temp = NamedTempFile::new().unwrap();
        let db_path = temp.path().to_str().unwrap();
        let db_url = format!("sqlite://{}", db_path);
        drop(temp);

        // Initialize database with schema
        let pool = initialize_database(&db_url).await.unwrap();

        // Verify tables were created by checking for pipelines table
        let result: i32 = sqlx
            ::query_scalar(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='pipelines'"
            )
            .fetch_one(&pool).await
            .unwrap();

        assert_eq!(result, 1, "Pipelines table should exist");
    }

    #[tokio::test]
    async fn test_ensure_schema_idempotent() {
        let temp = NamedTempFile::new().unwrap();
        let db_path = temp.path().to_str().unwrap();
        let db_url = format!("sqlite://{}", db_path);
        drop(temp);

        // Create database first so migrations can be tracked
        create_database_if_missing(&db_url).await.unwrap();
        let pool = SqlitePool::connect(&db_url).await.unwrap();

        // Run migrations twice - should be idempotent
        ensure_schema(&pool).await.unwrap();
        ensure_schema(&pool).await.unwrap();
    }
}
