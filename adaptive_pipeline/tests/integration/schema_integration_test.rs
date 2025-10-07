// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! Integration tests for schema initialization with SQLite repositories.
//!
//! These tests verify that the schema module correctly initializes databases
//! and runs migrations for repository implementations.

use pipeline::infrastructure::repositories::schema;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_schema_creates_database_automatically() {
    // Use temporary file that will be auto-cleaned
    let temp = NamedTempFile::new().unwrap();
    let db_path = temp.path().to_str().unwrap().to_string();
    drop(temp); // Remove file so we can test creation

    // Schema module should create database and run migrations
    let pool = schema::initialize_database(&format!("sqlite://{}", db_path)).await.unwrap();

    // Verify database is usable
    let count: i64 = sqlx
        ::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
        .fetch_one(&pool).await
        .unwrap();

    assert!(count > 0, "Database should have tables after initialization");

    // Cleanup
    let _ = std::fs::remove_file(&db_path);
}

#[tokio::test]
async fn test_schema_with_in_memory_database() {
    // In-memory database should work without file system
    let pool = schema::initialize_database("sqlite::memory:").await.unwrap();

    // Verify pipelines table exists
    let result: i64 = sqlx
        ::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='pipelines'")
        .fetch_one(&pool).await
        .unwrap();

    assert_eq!(result, 1, "Pipelines table should exist");
}

#[tokio::test]
async fn test_schema_migrations_run_automatically() {
    // Use temporary file that will be auto-cleaned
    let temp = NamedTempFile::new().unwrap();
    let db_path = temp.path().to_str().unwrap().to_string();
    drop(temp); // Remove file so we can test creation

    let pool = schema::initialize_database(&format!("sqlite://{}", db_path)).await.unwrap();

    // Verify _sqlx_migrations table exists (proves migrations ran)
    let result: i64 = sqlx
        ::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='_sqlx_migrations'"
        )
        .fetch_one(&pool).await
        .unwrap();

    assert_eq!(result, 1, "Migrations table should exist");

    // Verify at least one migration was applied
    let migration_count: i64 = sqlx
        ::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(&pool).await
        .unwrap();

    assert!(migration_count > 0, "At least one migration should be recorded");

    // Cleanup
    let _ = std::fs::remove_file(&db_path);
}

#[tokio::test]
async fn test_schema_idempotent_initialization() {
    // Use temporary file that will be auto-cleaned
    let temp = NamedTempFile::new().unwrap();
    let db_path = temp.path().to_str().unwrap().to_string();
    drop(temp);

    let db_url = format!("sqlite://{}", db_path);

    // Initialize twice - should not error
    let _pool1 = schema::initialize_database(&db_url).await.unwrap();
    let _pool2 = schema::initialize_database(&db_url).await.unwrap();

    // Cleanup
    let _ = std::fs::remove_file(&db_path);
}
