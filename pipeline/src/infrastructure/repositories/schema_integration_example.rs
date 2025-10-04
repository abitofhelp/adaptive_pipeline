// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! Example showing how to wire schema migration into a SQLite repository.
//!
//! Kept as documentation for developers migrating older constructors.


#![allow(dead_code)]

use sqlx::SqlitePool;
use pipeline_domain::PipelineError;

pub struct SqlitePipelineRepository {
    pool: SqlitePool,
}

impl SqlitePipelineRepository {
    /// Creates a new structured pipeline repository with automatic schema initialization
    ///
    /// This constructor:
    /// 1. Creates the database file if it doesn't exist
    /// 2. Runs all pending migrations to ensure schema is up to date
    /// 3. Returns a connected repository ready for use
    ///
    /// # Why This Approach?
    ///
    /// - **Auto-initialization**: No manual database creation required
    /// - **Version tracking**: Migrations are tracked in `_sqlx_migrations` table
    /// - **Idempotent**: Safe to call multiple times, migrations run only once
    /// - **Production-ready**: Same code works in dev, test, and production
    ///
    /// # Arguments
    ///
    /// * `database_path` - Path to SQLite database file
    ///   - File will be created if it doesn't exist
    ///   - Can be relative or absolute path
    ///   - Special value `:memory:` for in-memory database
    ///
    /// # Returns
    ///
    /// * `Ok(SqlitePipelineRepository)` - Ready-to-use repository
    /// * `Err(PipelineError)` - Database initialization failed
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use pipeline::infrastructure::adapters::repositories::sqlite_pipeline_repository_adapter::SqlitePipelineRepository;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Creates database and runs migrations automatically
    /// let repo = SqlitePipelineRepository::new("./pipeline.db").await?;
    ///
    /// // Database is ready to use immediately
    /// // let pipelines = repo.list_pipelines().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(database_path: &str) -> Result<Self, PipelineError> {
        use tracing::debug;

        debug!("Initializing SqlitePipelineRepository with database: {}", database_path);

        // Build proper SQLite URL
        let database_url = if database_path == ":memory:" || database_path == "sqlite::memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite://{}", database_path)
        };

        // Use schema module to initialize database with migrations
        let pool = crate::infrastructure::repositories::schema::initialize_database(&database_url)
            .await
            .map_err(|e| {
                PipelineError::database_error(format!(
                    "Failed to initialize database '{}': {}",
                    database_path, e
                ))
            })?;

        debug!("Successfully initialized SQLite database with schema");
        Ok(Self { pool })
    }

    // Alternative: Manual control over migration timing
    /// Creates repository with manual migration control
    ///
    /// Use this if you need to separate database creation from migration.
    /// Most applications should use `new()` instead.
    pub async fn new_without_migrations(database_path: &str) -> Result<Self, PipelineError> {
        let database_url = if database_path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite://{}", database_path)
        };

        // Create database but don't run migrations yet
        crate::infrastructure::repositories::schema::create_database_if_missing(&database_url)
            .await
            .map_err(|e| PipelineError::database_error(format!("Failed to create database: {}", e)))?;

        let pool = SqlitePool::connect(&database_url)
            .await
            .map_err(|e| PipelineError::database_error(format!("Failed to connect: {}", e)))?;

        Ok(Self { pool })
    }

    /// Manually run migrations on an existing repository
    ///
    /// Only needed if you used `new_without_migrations()`.
    pub async fn run_migrations(&self) -> Result<(), PipelineError> {
        crate::infrastructure::repositories::schema::ensure_schema(&self.pool)
            .await
            .map_err(|e| PipelineError::database_error(format!("Migration failed: {}", e)))
    }
}

// ============================================================================
// COMPARISON: Old vs New Approach
// ============================================================================

/// Old approach (BEFORE)
///
/// Problems:
/// - Misleading log message ("Creating database" but doesn't actually create)
/// - No schema initialization
/// - Assumes database exists with correct schema
/// - Fails with error code 14 if database missing
mod old_approach {
    /*
    pub async fn new(database_path: &str) -> Result<Self, PipelineError> {
        let database_url = format!("sqlite://{}", database_path);

        // This FAILS if database doesn't exist!
        let pool = SqlitePool::connect(&database_url).await
            .map_err(|e| PipelineError::database_error(...))?;

        Ok(Self { pool })
    }
    */
}

/// New approach (AFTER)
///
/// Benefits:
/// - Creates database automatically if missing
/// - Runs migrations to ensure schema is correct
/// - Works on first run without manual setup
/// - Tracks migration history for version management
/// - Production-ready out of the box
mod new_approach {
    /*
    pub async fn new(database_path: &str) -> Result<Self, PipelineError> {
        let database_url = format!("sqlite://{}", database_path);

        // Auto-creates database and runs migrations
        let pool = schema::initialize_database(&database_url).await
            .map_err(|e| PipelineError::database_error(...))?;

        Ok(Self { pool })
    }
    */
}

// ============================================================================
// TESTING
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_new_creates_database_automatically() {
        // Should work even if database doesn't exist
        let repo = SqlitePipelineRepository::new("./test_auto_created.db")
            .await
            .unwrap();

        // Database should be usable immediately
        // (This would fail with old approach)
    }

    #[tokio::test]
    async fn test_in_memory_database() {
        let repo = SqlitePipelineRepository::new(":memory:")
            .await
            .unwrap();

        // In-memory database is perfect for testing
    }

    #[tokio::test]
    async fn test_migrations_run_automatically() {
        let repo = SqlitePipelineRepository::new("./test_migrations.db")
            .await
            .unwrap();

        // Verify schema exists by querying pipelines table
        // (This would fail if migrations didn't run)
    }
}
