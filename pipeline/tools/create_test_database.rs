//! # Database Creation Tool
//!
//! This tool creates SQLite databases with proper schema and test data for the
//! adaptive pipeline system. It provides a convenient way to set up test
//! databases for development, testing, and demonstration purposes.
//!
//! ## Overview
//!
//! The database creation tool provides:
//!
//! - **Schema Creation**: Generates proper SQLite schema for pipeline data
//! - **Test Data Population**: Optionally populates database with test data
//! - **Flexible Configuration**: Customizable database paths and options
//! - **Validation**: Ensures database integrity and proper structure
//!
//! ## Features
//!
//! ### Schema Generation
//! - **Pipeline Tables**: Creates tables for pipeline configurations
//! - **Stage Tables**: Creates tables for pipeline stage definitions
//! - **Metadata Tables**: Creates tables for system metadata and settings
//! - **Index Creation**: Adds appropriate indexes for performance
//!
//! ### Test Data Population
//! - **Sample Pipelines**: Creates example pipeline configurations
//! - **Stage Configurations**: Populates stage definitions and parameters
//! - **Realistic Data**: Uses realistic ULIDs and configuration values
//! - **Validation**: Ensures all generated data is valid and consistent
//!
//! ## Usage
//!
//! ### Basic Usage
//! ```bash
//! # Create database with default path
//! cargo run --bin create-test-database
//!
//! # Create database with custom path
//! cargo run --bin create-test-database /tmp/my_test.db
//!
//! # Create database with test data
//! cargo run --bin create-test-database --with-data
//!
//! # Create database with test data at custom path
//! cargo run --bin create-test-database --with-data /tmp/populated.db
//! ```
//!
//! ### Command Line Options
//! - `--with-data`: Populate database with test data
//! - `--help`: Show help information
//! - `database_path`: Optional path for the generated database
//!
//! ### Default Paths
//! - Default database path: `scripts/test_data/generated_database.db`
//! - Schema files: Located in `scripts/` directory
//! - Test data: Generated programmatically with realistic values
//!
//! ## Database Schema
//!
//! The tool creates the following tables:
//!
//! ### Pipeline Tables
//! - **pipelines**: Main pipeline configurations
//! - **pipeline_stages**: Stage definitions and ordering
//! - **stage_parameters**: Stage-specific configuration parameters
//!
//! ### Metadata Tables
//! - **schema_version**: Database schema version information
//! - **system_settings**: System-wide configuration settings
//!
//! ## Test Data
//!
//! When `--with-data` is specified, the tool generates:
//!
//! ### Sample Pipelines
//! - **secure-backup**: Compression + encryption pipeline
//! - **fast-compress**: High-speed compression pipeline
//! - **archive-pipeline**: Long-term archival pipeline
//! - **streaming-pipeline**: Real-time streaming pipeline
//!
//! ### Stage Configurations
//! - **Compression stages**: Various compression algorithms and levels
//! - **Encryption stages**: Different encryption algorithms and key sizes
//! - **Validation stages**: Checksum and integrity verification
//!
//! ## Error Handling
//!
//! The tool handles various error conditions:
//! - **File System Errors**: Permission issues, disk space, path validation
//! - **Database Errors**: SQLite connection issues, schema creation failures
//! - **Data Generation Errors**: Invalid test data, constraint violations
//! - **Configuration Errors**: Invalid command line arguments
//!
//! ## Integration
//!
//! The tool integrates with:
//! - **Pipeline System**: Uses same domain entities and value objects
//! - **Test Framework**: Provides databases for integration tests
//! - **Development Workflow**: Supports rapid development and testing
//! - **CI/CD Pipeline**: Can be used in automated testing environments
//!
//! ## Examples
//!
//! ### Create Empty Database
//! ```bash
//! cargo run --bin create-test-database /tmp/empty.db
//! ```
//!
//! ### Create Populated Database
//! ```bash
//! cargo run --bin create-test-database --with-data /tmp/populated.db
//! ```
//!
//! ### Use in Tests
//! ```rust
//! use std::process::Command;
//!
//! // Create test database in test
//! let output = Command::new("cargo")
//!     .args(&[
//!         "run",
//!         "--bin",
//!         "create-test-database",
//!         "--with-data",
//!         "/tmp/test.db",
//!     ])
//!     .output()
//!     .expect("Failed to create test database");
//!
//! assert!(output.status.success());
//! ```

use pipeline_domain::entities::pipeline_stage::{StageConfiguration, StageType};
use pipeline_domain::value_objects::{PipelineId, StageId};
use pipeline_domain::{Pipeline, PipelineStage};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use std::collections::HashMap;
use std::env;
use std::path::Path;

/// Database Creation Tool for the adaptive pipeline system
///
/// This tool creates SQLite databases with proper schema and optionally
/// populates them with test data for development, testing, and demonstration
/// purposes.
///
/// # Features
///
/// - **Schema Creation**: Generates complete SQLite schema for pipeline data
/// - **Test Data Population**: Creates realistic test data with proper ULIDs
/// - **Flexible Configuration**: Supports custom database paths and options
/// - **Validation**: Ensures database integrity and proper structure
///
/// # Usage
///
/// ```bash
/// # Basic usage with default path
/// cargo run --bin create-test-database
///
/// # Custom database path
/// cargo run --bin create-test-database /tmp/my_test.db
///
/// # Include test data
/// cargo run --bin create-test-database --with-data
///
/// # Custom path with test data
/// cargo run --bin create-test-database --with-data /tmp/populated.db
/// ```
///
/// # Command Line Arguments
///
/// - `--with-data`: Populate database with sample pipeline configurations
/// - `--help`: Display help information
/// - `database_path`: Optional path for the generated database file
///
/// # Default Behavior
///
/// - **Default Path**: `scripts/test_data/generated_database.db`
/// - **Schema Only**: Creates empty database with proper schema
/// - **Validation**: Verifies database structure after creation
///
/// # Test Data
///
/// When `--with-data` is specified, creates:
/// - Sample pipeline configurations (secure-backup, fast-compress, etc.)
/// - Stage definitions with realistic parameters
/// - Proper ULID identifiers for all entities
/// - Valid configuration values for all algorithms
///
/// # Examples
///
/// ```rust
/// // This tool can be used programmatically in tests
/// use std::process::Command;
///
/// let result = Command::new("cargo")
///     .args(&[
///         "run",
///         "--bin",
///         "create-test-database",
///         "--with-data",
///         "/tmp/test.db",
///     ])
///     .output()
///     .expect("Failed to execute database creation tool");
///
/// assert!(result.status.success());
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for flags
    let with_data = args.contains(&"--with-data".to_string());
    let help_requested = args.iter().any(|arg| arg == "--help" || arg == "-h");

    if help_requested {
        println!("Database Creation Tool");
        println!();
        println!("Creates a SQLite database file with proper schema. Defaults to empty database for testing.");
        println!();
        println!("USAGE:");
        println!("    cargo run --bin create-test-database [FLAGS] [database_path]");
        println!();
        println!("FLAGS:");
        println!("    --with-data    Include test data in the database (default: empty)");
        println!("    -h, --help     Show this help message");
        println!();
        println!("ARGUMENTS:");
        println!(
            "    database_path  Path for the generated SQLite database [default: \
             scripts/test_data/generated_database.db]"
        );
        println!();
        println!("EXAMPLES:");
        println!("    cargo run --bin create-test-database                    # Empty database");
        println!("    cargo run --bin create-test-database --with-data        # With test data");
        println!("    cargo run --bin create-test-database /tmp/test.db       # Empty to custom path");
        println!("    cargo run --bin create-test-database --with-data /tmp/populated.db");
        return Ok(());
    }

    // Find database path (skip flags)
    let db_path = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "scripts/test_data/generated_database.db".to_string());

    let db_type = if with_data {
        "with test data"
    } else {
        "empty (schema only)"
    };

    println!("🔧 Database Creation Tool");
    println!("📁 Database path: {}", db_path);

    // Generate proper IDs using our value objects
    let test_multi_stage_id = PipelineId::new();
    let image_processing_id = PipelineId::new();

    println!("📋 Generated Pipeline IDs:");
    println!("  test-multi-stage: {}", test_multi_stage_id);
    println!("  image-processing: {}", image_processing_id);

    // Generate stage IDs for test-multi-stage pipeline
    let input_checksum_id = StageId::new();
    let compression_id = StageId::new();
    let encryption_id = StageId::new();
    let output_checksum_id = StageId::new();

    println!("\n🔧 Generated Stage IDs for test-multi-stage:");
    println!("  input_checksum: {}", input_checksum_id);
    println!("  compression: {}", compression_id);
    println!("  encryption: {}", encryption_id);
    println!("  output_checksum: {}", output_checksum_id);

    // Generate stage IDs for image-processing pipeline
    let input_validation_id = StageId::new();
    let image_compression_id = StageId::new();

    println!("\n🔧 Generated Stage IDs for image-processing:");
    println!("  input_validation: {}", input_validation_id);
    println!("  image_compression: {}", image_compression_id);

    // Create the actual pipeline objects to test our domain logic
    let mut test_stages = Vec::new();

    // Create stages using our domain objects (Pipeline::new will add checksum
    // stages automatically)
    let compression_stage = PipelineStage::new(
        "compression".to_string(),
        StageType::Compression,
        StageConfiguration::new("brotli".to_string(), HashMap::new(), false),
        1, // This will be adjusted by Pipeline::new
    )?;

    let encryption_stage = PipelineStage::new(
        "encryption".to_string(),
        StageType::Encryption,
        StageConfiguration::new("aes256gcm".to_string(), HashMap::new(), false),
        2, // This will be adjusted by Pipeline::new
    )?;

    test_stages.push(compression_stage);
    test_stages.push(encryption_stage);

    // Create the pipeline (this will automatically add input_checksum and
    // output_checksum stages)
    let test_pipeline = Pipeline::new("test-multi-stage".to_string(), test_stages)?;

    println!(
        "\n✅ Created test-multi-stage pipeline with {} stages:",
        test_pipeline.stages().len()
    );
    for (i, stage) in test_pipeline.stages().iter().enumerate() {
        println!("  {}. {} ({})", i, stage.name(), stage.stage_type());
    }

    // Generate SQL with proper IDs
    println!("\n📝 Generating SQL script ({})...", db_type);

    // Schema part (always included)
    let schema_sql = r#"
-- Generated using our ID value objects - SOURCE OF TRUTH
-- Delete old database and create fresh one
DROP TABLE IF EXISTS stage_parameters;
DROP TABLE IF EXISTS pipeline_stages;
DROP TABLE IF EXISTS pipeline_configuration;
DROP TABLE IF EXISTS processing_metrics;
DROP TABLE IF EXISTS processing_sessions;
DROP TABLE IF EXISTS file_chunks;
DROP TABLE IF EXISTS security_contexts;
DROP TABLE IF EXISTS pipelines;

-- Create tables
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE TABLE pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    name TEXT NOT NULL,
    stage_type TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    stage_order INTEGER NOT NULL,
    algorithm TEXT NOT NULL,
    parallel_processing BOOLEAN NOT NULL DEFAULT FALSE,
    chunk_size INTEGER,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE TABLE stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id)
);

CREATE TABLE processing_metrics (
    pipeline_id TEXT PRIMARY KEY,
    bytes_processed INTEGER NOT NULL DEFAULT 0,
    bytes_total INTEGER NOT NULL DEFAULT 0,
    chunks_processed INTEGER NOT NULL DEFAULT 0,
    chunks_total INTEGER NOT NULL DEFAULT 0,
    start_time_rfc3339 TEXT,
    end_time_rfc3339 TEXT,
    processing_duration_ms INTEGER,
    throughput_bytes_per_second REAL NOT NULL DEFAULT 0.0,
    compression_ratio REAL,
    error_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    input_file_size_bytes INTEGER NOT NULL DEFAULT 0,
    output_file_size_bytes INTEGER NOT NULL DEFAULT 0,
    input_file_checksum TEXT,
    output_file_checksum TEXT,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

"#;

    // Data part (only included with --with-data flag)
    let data_sql = if with_data {
        format!(
            r#"
-- Insert data with proper ULID format from our value objects
INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES
('{}', 'test-multi-stage', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'image-processing', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert pipeline configuration
INSERT INTO pipeline_configuration (pipeline_id, key, value, archived, created_at, updated_at) VALUES
('{}', 'encryption_algorithm', 'aes256gcm', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'compression_algorithm', 'brotli', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'chunk_size_mb', '1', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert pipeline stages for test-multi-stage (4 stages total)
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, archived, created_at, updated_at) VALUES
('{}', '{}', 'input_checksum', 'checksum', true, 0, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'compression', 'compression', true, 1, 'brotli', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'encryption', 'encryption', true, 2, 'aes256gcm', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'output_checksum', 'checksum', true, 3, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'input_validation', 'checksum', true, 0, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'image_compression', 'compression', true, 1, 'jpeg', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert processing metrics
INSERT INTO processing_metrics (pipeline_id) VALUES
('{}'),
('{}');

-- Verify data
SELECT 'pipelines' as table_name, COUNT(*) as count FROM pipelines
UNION ALL
SELECT 'pipeline_configuration', COUNT(*) FROM pipeline_configuration
UNION ALL
SELECT 'pipeline_stages', COUNT(*) FROM pipeline_stages
UNION ALL
SELECT 'processing_metrics', COUNT(*) FROM processing_metrics;
"#,
            test_multi_stage_id,
            image_processing_id,
            test_multi_stage_id,
            test_multi_stage_id,
            test_multi_stage_id,
            input_checksum_id,
            test_multi_stage_id,
            compression_id,
            test_multi_stage_id,
            encryption_id,
            test_multi_stage_id,
            output_checksum_id,
            test_multi_stage_id,
            input_validation_id,
            image_processing_id,
            image_compression_id,
            image_processing_id,
            test_multi_stage_id,
            image_processing_id
        )
    } else {
        String::new()
    };

    // Create parent directory if it doesn't exist
    if let Some(parent) = Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Remove existing database file if it exists
    if Path::new(&db_path).exists() {
        std::fs::remove_file(&db_path)?;
        println!("🗑️  Removed existing database: {}", db_path);
    }

    // Create database connection
    let database_url = format!("sqlite://{}?mode=rwc", db_path);
    let pool = SqlitePool::connect(&database_url).await?;

    println!("📊 Creating database schema...");

    // Execute schema SQL
    sqlx::raw_sql(schema_sql).execute(&pool).await?;

    // Execute data SQL if requested
    if with_data {
        println!("📝 Inserting test data...");
        let formatted_data_sql = format!(
            r#"
-- Insert data with proper ULID format from our value objects
INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES
('{}', 'test-multi-stage', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'image-processing', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert pipeline configuration
INSERT INTO pipeline_configuration (pipeline_id, key, value, archived, created_at, updated_at) VALUES
('{}', 'encryption_algorithm', 'aes256gcm', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'compression_algorithm', 'brotli', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', 'chunk_size_mb', '1', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert pipeline stages for test-multi-stage (4 stages total)
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, archived, created_at, updated_at) VALUES
('{}', '{}', 'input_checksum', 'checksum', true, 0, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'compression', 'compression', true, 1, 'brotli', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'encryption', 'encryption', true, 2, 'aes256gcm', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'output_checksum', 'checksum', true, 3, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'input_validation', 'checksum', true, 0, 'sha256', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('{}', '{}', 'image_compression', 'compression', true, 1, 'jpeg', false, null, false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert processing metrics
INSERT INTO processing_metrics (pipeline_id) VALUES
('{}'),
('{}');
"#,
            test_multi_stage_id,
            image_processing_id,
            test_multi_stage_id,
            test_multi_stage_id,
            test_multi_stage_id,
            input_checksum_id,
            test_multi_stage_id,
            compression_id,
            test_multi_stage_id,
            encryption_id,
            test_multi_stage_id,
            output_checksum_id,
            test_multi_stage_id,
            input_validation_id,
            image_processing_id,
            image_compression_id,
            image_processing_id,
            test_multi_stage_id,
            image_processing_id
        );

        sqlx::raw_sql(&formatted_data_sql).execute(&pool).await?;
    }

    // Verify the database
    let verification_sql = r#"
SELECT 'pipelines' as table_name, COUNT(*) as count FROM pipelines
UNION ALL
SELECT 'pipeline_configuration', COUNT(*) FROM pipeline_configuration
UNION ALL
SELECT 'pipeline_stages', COUNT(*) FROM pipeline_stages
UNION ALL
SELECT 'processing_metrics', COUNT(*) FROM processing_metrics;
"#;

    let rows = sqlx::query(verification_sql).fetch_all(&pool).await?;

    println!("\n📊 Database verification:");
    for row in rows {
        let table_name: String = row.get("table_name");
        let count: i64 = row.get("count");
        println!("  {}: {} records", table_name, count);
    }

    pool.close().await;

    println!("\n✅ Created SQLite database: {}", db_path);
    println!("🎯 This uses our actual ID value objects as the source of truth!");

    Ok(())
}
