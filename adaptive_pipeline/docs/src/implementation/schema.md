# Schema Management

**Version**: 1.0
**Date**: 2025-10-04
**License**: BSD-3-Clause
**Copyright**: (c) 2025 Michael Gardner, A Bit of Help, Inc.
**Authors**: Michael Gardner
**Status**: Active

---

## Overview

The Adaptive Pipeline uses **SQLite** for data persistence with an automated schema management system powered by **sqlx migrations**. This chapter explains the database schema design, migration strategy, and best practices for schema evolution.

### Key Features

- **Automatic Migrations**: Schema automatically initialized and updated on startup
- **Version Tracking**: Migrations tracked in `_sqlx_migrations` table
- **Idempotent**: Safe to run migrations multiple times
- **Normalized Design**: Proper foreign keys and referential integrity
- **Performance Indexed**: Strategic indexes for common queries
- **Test-Friendly**: Support for in-memory databases

---

## Database Schema

### Entity-Relationship Diagram

```text
┌─────────────────────────────────────────────────────────────┐
│                        pipelines                            │
├─────────────────────────────────────────────────────────────┤
│ id (PK)             TEXT                                    │
│ name                TEXT UNIQUE NOT NULL                    │
│ archived            BOOLEAN DEFAULT false                   │
│ created_at          TEXT NOT NULL                           │
│ updated_at          TEXT NOT NULL                           │
└────────────────┬────────────────────────────────────────────┘
                 │
                 │ 1:N
                 │
    ┌────────────┼──────────────────┐
    │            │                  │
    ▼            ▼                  ▼
┌─────────────────┐  ┌───────────────────────┐  ┌──────────────────┐
│ pipeline_stages │  │pipeline_configuration │  │processing_metrics│
├─────────────────┤  ├───────────────────────┤  ├──────────────────┤
│ id (PK)         │  │ pipeline_id (PK,FK)   │  │ pipeline_id (PK,FK)│
│ pipeline_id (FK)│  │ key (PK)              │  │ bytes_processed  │
│ name            │  │ value                 │  │ throughput_*     │
│ stage_type      │  │ archived              │  │ error_count      │
│ algorithm       │  │ created_at            │  │ ...              │
│ enabled         │  │ updated_at            │  └──────────────────┘
│ stage_order     │  └───────────────────────┘
│ ...             │
└────────┬────────┘
         │
         │ 1:N
         │
         ▼
┌──────────────────┐
│ stage_parameters │
├──────────────────┤
│ stage_id (PK,FK) │
│ key (PK)         │
│ value            │
│ archived         │
│ created_at       │
│ updated_at       │
└──────────────────┘
```

### Tables Overview

| Table | Purpose | Relationships |
|-------|---------|---------------|
| **pipelines** | Core pipeline configurations | Parent of stages, config, metrics |
| **pipeline_stages** | Processing stages within pipelines | Child of pipelines, parent of parameters |
| **pipeline_configuration** | Key-value configuration for pipelines | Child of pipelines |
| **stage_parameters** | Key-value parameters for stages | Child of pipeline_stages |
| **processing_metrics** | Execution metrics and statistics | Child of pipelines |

---

## Table Schemas

### pipelines

The root table for pipeline management:

```sql
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT false,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Columns**:
- `id`: UUID or unique identifier (e.g., "pipeline-123")
- `name`: Human-readable name (unique constraint)
- `archived`: Soft delete flag (false = active, true = archived)
- `created_at`: RFC3339 timestamp of creation
- `updated_at`: RFC3339 timestamp of last modification

**Constraints**:
- Primary key on `id`
- Unique constraint on `name`
- Indexed on `name WHERE archived = false` for active pipeline lookups

### pipeline_stages

Defines the ordered stages within a pipeline:

```sql
CREATE TABLE IF NOT EXISTS pipeline_stages (
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
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

**Columns**:
- `id`: Unique stage identifier
- `pipeline_id`: Foreign key to owning pipeline
- `name`: Stage name (e.g., "compression", "encryption")
- `stage_type`: Type of stage (enum: compression, encryption, checksum)
- `enabled`: Whether stage is active
- `stage_order`: Execution order (0-based)
- `algorithm`: Specific algorithm (e.g., "zstd", "aes-256-gcm")
- `parallel_processing`: Whether stage can process chunks in parallel
- `chunk_size`: Optional chunk size override for this stage
- `archived`: Soft delete flag
- `created_at`, `updated_at`: Timestamps

**Constraints**:
- Primary key on `id`
- Foreign key to `pipelines(id)` with CASCADE delete
- Indexed on `(pipeline_id, stage_order)` for ordered retrieval
- Indexed on `pipeline_id` for pipeline lookups

### pipeline_configuration

Key-value configuration storage for pipelines:

```sql
CREATE TABLE IF NOT EXISTS pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

**Columns**:
- `pipeline_id`: Foreign key to pipeline
- `key`: Configuration key (e.g., "max_workers", "buffer_size")
- `value`: Configuration value (stored as TEXT, parsed by application)
- `archived`, `created_at`, `updated_at`: Standard metadata

**Constraints**:
- Composite primary key on `(pipeline_id, key)`
- Foreign key to `pipelines(id)` with CASCADE delete
- Indexed on `pipeline_id`

**Usage Example**:
```
pipeline_id                          | key           | value
-------------------------------------|---------------|-------
pipeline-abc-123                     | max_workers   | 4
pipeline-abc-123                     | buffer_size   | 1048576
```

### stage_parameters

Key-value parameters for individual stages:

```sql
CREATE TABLE IF NOT EXISTS stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id) ON DELETE CASCADE
);
```

**Columns**:
- `stage_id`: Foreign key to stage
- `key`: Parameter key (e.g., "compression_level", "key_size")
- `value`: Parameter value (TEXT, parsed by stage)
- `archived`, `created_at`, `updated_at`: Standard metadata

**Constraints**:
- Composite primary key on `(stage_id, key)`
- Foreign key to `pipeline_stages(id)` with CASCADE delete
- Indexed on `stage_id`

**Usage Example**:
```
stage_id                | key                | value
------------------------|--------------------|---------
stage-comp-456          | compression_level  | 9
stage-enc-789           | key_size           | 256
```

### processing_metrics

Tracks execution metrics for pipeline runs:

```sql
CREATE TABLE IF NOT EXISTS processing_metrics (
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
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

**Columns**:
- `pipeline_id`: Foreign key to pipeline (also primary key - one metric per pipeline)
- Progress tracking: `bytes_processed`, `bytes_total`, `chunks_processed`, `chunks_total`
- Timing: `start_time_rfc3339`, `end_time_rfc3339`, `processing_duration_ms`
- Performance: `throughput_bytes_per_second`, `compression_ratio`
- Status: `error_count`, `warning_count`
- File info: `input_file_size_bytes`, `output_file_size_bytes`
- Integrity: `input_file_checksum`, `output_file_checksum`

**Constraints**:
- Primary key on `pipeline_id`
- Foreign key to `pipelines(id)` with CASCADE delete

---

## Migrations with sqlx

### Migration Files

Migrations live in the `/migrations` directory at the project root:

```text
migrations/
└── 20250101000000_initial_schema.sql
```

**Naming Convention**: `{timestamp}_{description}.sql`
- Timestamp: `YYYYMMDDHHMMSS` format
- Description: Snake_case description of changes

### Migration Structure

Each migration file contains:

```sql
-- Migration: 20250101000000_initial_schema.sql
-- Description: Initial database schema for pipeline management

-- Table creation
CREATE TABLE IF NOT EXISTS pipelines (...);
CREATE TABLE IF NOT EXISTS pipeline_stages (...);
-- ... more tables ...

-- Index creation
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
-- ... more indexes ...
```

### sqlx Migration Macro

The `sqlx::migrate!()` macro embeds migrations at compile time:

```rust
// In schema.rs
pub async fn ensure_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    debug!("Ensuring database schema is up to date");

    // Run migrations - sqlx will automatically track what's been applied
    sqlx::migrate!("../migrations").run(pool).await?;

    info!("Database schema is up to date");
    Ok(())
}
```

**How it works**:
1. `sqlx::migrate!("../migrations")` scans directory at compile time
2. Embeds migration SQL into binary
3. `run(pool)` executes pending migrations at runtime
4. Tracks applied migrations in `_sqlx_migrations` table

---

## Schema Initialization

### Automatic Initialization

The schema module provides convenience functions for database setup:

```rust
/// High-level initialization function
pub async fn initialize_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // 1. Create database if it doesn't exist
    create_database_if_missing(database_url).await?;

    // 2. Connect to database
    let pool = SqlitePool::connect(database_url).await?;

    // 3. Run migrations
    ensure_schema(&pool).await?;

    Ok(pool)
}
```

**Usage in application startup**:

```rust
use pipeline::infrastructure::repositories::schema;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database with schema
    let pool = schema::initialize_database("sqlite://./pipeline.db").await?;

    // Database is ready to use!
    let repository = SqlitePipelineRepository::new(pool);

    Ok(())
}
```

### Create Database if Missing

For file-based SQLite databases:

```rust
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
```

**Handles**:
- New database creation
- Existing database detection
- File system permissions

### In-Memory Databases

For testing, use in-memory databases:

```rust
#[tokio::test]
async fn test_with_in_memory_db() {
    // No file system needed
    let pool = schema::initialize_database("sqlite::memory:")
        .await
        .unwrap();

    // Database is fully initialized in memory
    // ... run tests ...
}
```

---

## Migration Tracking

### _sqlx_migrations Table

sqlx automatically creates a tracking table:

```sql
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
```

**Columns**:
- `version`: Migration timestamp (e.g., 20250101000000)
- `description`: Migration description
- `installed_on`: When migration was applied
- `success`: Whether migration succeeded
- `checksum`: SHA256 of migration SQL
- `execution_time`: Duration in milliseconds

### Querying Applied Migrations

```rust
let migrations: Vec<(i64, String)> = sqlx::query_as(
    "SELECT version, description FROM _sqlx_migrations ORDER BY version"
)
.fetch_all(&pool)
.await?;

for (version, description) in migrations {
    println!("Applied migration: {} - {}", version, description);
}
```

---

## Adding New Migrations

### Step 1: Create Migration File

Create a new file in `/migrations`:

```bash
# Generate timestamp
TIMESTAMP=$(date +%Y%m%d%H%M%S)

# Create migration file
touch migrations/${TIMESTAMP}_add_pipeline_tags.sql
```

### Step 2: Write Migration SQL

```sql
-- migrations/20250204120000_add_pipeline_tags.sql
-- Add tagging support for pipelines

CREATE TABLE IF NOT EXISTS pipeline_tags (
    pipeline_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, tag),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pipeline_tags_tag ON pipeline_tags(tag);
```

### Step 3: Test Migration

```rust
#[tokio::test]
async fn test_new_migration() {
    let pool = schema::initialize_database("sqlite::memory:")
        .await
        .unwrap();

    // Verify new table exists
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='pipeline_tags'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(count, 1);
}
```

### Step 4: Rebuild

```bash
# sqlx macro embeds migrations at compile time
cargo build
```

The next application start will automatically apply the new migration.

---

## Indexes and Performance

### Current Indexes

```sql
-- Ordered stage retrieval
CREATE INDEX idx_pipeline_stages_order
ON pipeline_stages(pipeline_id, stage_order);

-- Stage lookup by pipeline
CREATE INDEX idx_pipeline_stages_pipeline_id
ON pipeline_stages(pipeline_id);

-- Configuration lookup
CREATE INDEX idx_pipeline_configuration_pipeline_id
ON pipeline_configuration(pipeline_id);

-- Parameter lookup
CREATE INDEX idx_stage_parameters_stage_id
ON stage_parameters(stage_id);

-- Active pipelines only
CREATE INDEX idx_pipelines_name
ON pipelines(name) WHERE archived = false;
```

### Index Strategy

**When to add indexes**:
- ✅ Foreign key columns (for JOIN performance)
- ✅ Columns in WHERE clauses (for filtering)
- ✅ Columns in ORDER BY (for sorting)
- ✅ Partial indexes for common filters (e.g., `WHERE archived = false`)

**When NOT to index**:
- ❌ Small tables (< 1000 rows)
- ❌ Columns with low cardinality (few distinct values)
- ❌ Columns rarely used in queries
- ❌ Write-heavy columns (indexes slow INSERTs/UPDATEs)

---

## Best Practices

### ✅ DO

**Use idempotent migrations**
```sql
-- Safe to run multiple times
CREATE TABLE IF NOT EXISTS new_table (...);
CREATE INDEX IF NOT EXISTS idx_name ON table(column);
```

**Include rollback comments**
```sql
-- Migration: Add user_id column
-- Rollback: DROP COLUMN is not supported in SQLite, recreate table

ALTER TABLE pipelines ADD COLUMN user_id TEXT;
```

**Use transactions for multi-statement migrations**
```sql
BEGIN TRANSACTION;

CREATE TABLE new_table (...);
INSERT INTO new_table SELECT ...;
DROP TABLE old_table;

COMMIT;
```

**Test migrations with production-like data**
```rust
#[tokio::test]
async fn test_migration_with_data() {
    let pool = schema::initialize_database("sqlite::memory:").await.unwrap();

    // Insert test data
    sqlx::query("INSERT INTO pipelines (...) VALUES (...)")
        .execute(&pool)
        .await
        .unwrap();

    // Run migration
    schema::ensure_schema(&pool).await.unwrap();

    // Verify data integrity
    // ...
}
```

### ❌ DON'T

**Don't modify existing migrations**
```sql
-- BAD: Editing 20250101000000_initial_schema.sql after deployment
-- This will cause checksum mismatch!

-- GOOD: Create a new migration to alter the schema
-- migrations/20250204000000_modify_pipeline_name.sql
```

**Don't use database-specific features unnecessarily**
```sql
-- BAD: SQLite-only (limits portability)
CREATE TABLE pipelines (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

-- GOOD: Portable approach
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY  -- Application generates UUIDs
);
```

**Don't forget foreign key constraints**
```sql
-- BAD: No referential integrity
CREATE TABLE pipeline_stages (
    pipeline_id TEXT NOT NULL
);

-- GOOD: Enforced relationships
CREATE TABLE pipeline_stages (
    pipeline_id TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

---

## Testing Schema Changes

### Unit Tests

From `schema.rs`:

```rust
#[tokio::test]
async fn test_create_database_if_missing() {
    let temp = NamedTempFile::new().unwrap();
    let db_path = temp.path().to_str().unwrap();
    let db_url = format!("sqlite://{}", db_path);
    drop(temp); // Remove file

    // Should create the database
    create_database_if_missing(&db_url).await.unwrap();

    // Should succeed if already exists
    create_database_if_missing(&db_url).await.unwrap();
}
```

### Integration Tests

From `schema_integration_test.rs`:

```rust
#[tokio::test]
async fn test_schema_migrations_run_automatically() {
    let pool = schema::initialize_database("sqlite::memory:")
        .await
        .unwrap();

    // Verify _sqlx_migrations table exists
    let result: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM _sqlx_migrations"
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(result > 0, "At least one migration should be applied");
}
```

### Idempotency Tests

```rust
#[tokio::test]
async fn test_schema_idempotent_initialization() {
    let db_url = "sqlite::memory:";

    // Initialize twice - should not error
    let _pool1 = schema::initialize_database(db_url).await.unwrap();
    let _pool2 = schema::initialize_database(db_url).await.unwrap();
}
```

---

## Troubleshooting

### Issue: Migration checksum mismatch

**Symptom**: Error: "migration checksum mismatch"

**Cause**: Existing migration file was modified after being applied

**Solution**:
```bash
# NEVER modify applied migrations!
# Instead, create a new migration to make changes

# If in development and migration hasn't been deployed:
# 1. Drop database
rm pipeline.db

# 2. Recreate with modified migration
cargo run
```

### Issue: Database file locked

**Symptom**: Error: "database is locked"

**Cause**: Another process has an exclusive lock

**Solution**:
```rust
// Use connection pool with proper configuration
let pool = SqlitePool::connect_with(
    SqliteConnectOptions::from_str("sqlite://./pipeline.db")?
        .busy_timeout(Duration::from_secs(30))  // Wait for lock
        .journal_mode(SqliteJournalMode::Wal)    // Use WAL mode
)
.await?;
```

### Issue: Foreign key constraint failed

**Symptom**: Error: "FOREIGN KEY constraint failed"

**Cause**: Trying to insert/update with invalid foreign key

**Solution**:
```sql
-- Enable foreign key enforcement (SQLite default is OFF)
PRAGMA foreign_keys = ON;

-- Then verify referenced row exists before insert
SELECT id FROM pipelines WHERE id = ?;
```

---

## Next Steps

- **[Repository Implementation](repositories.md)**: Using the schema in repositories
- **[Data Persistence](persistence.md)**: Persistence patterns and strategies
- **[Testing](../testing/integration-tests.md)**: Integration testing with databases

---

## References

- [sqlx Migrations Documentation](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#create-and-run-migrations)
- [SQLite Data Types](https://www.sqlite.org/datatype3.html)
- [SQLite Foreign Keys](https://www.sqlite.org/foreignkeys.html)
- [SQLite Indexes](https://www.sqlite.org/queryplanner.html)
- Source: `pipeline/src/infrastructure/repositories/schema.rs` (lines 1-157)
- Source: `migrations/20250101000000_initial_schema.sql` (lines 1-81)
- Source: `pipeline/tests/schema_integration_test.rs` (lines 1-110)
