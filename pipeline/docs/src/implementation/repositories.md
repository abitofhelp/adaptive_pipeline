# Repository Implementation

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The repository pattern provides an abstraction layer between the domain and data persistence, enabling the application to work with domain entities without knowing about database details. This separation allows for flexible storage implementations and easier testing.

**Key Benefits:**
- **Domain Independence**: Business logic stays free from persistence concerns
- **Testability**: Easy mocking with in-memory implementations
- **Flexibility**: Support for different storage backends (SQLite, PostgreSQL, etc.)
- **Consistency**: Standardized data access patterns

## Repository Interface

### Domain-Defined Contract

The domain layer defines the repository interface:

```rust
use pipeline_domain::repositories::PipelineRepository;
use pipeline_domain::entities::Pipeline;
use pipeline_domain::value_objects::PipelineId;
use pipeline_domain::PipelineError;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineRepository: Send + Sync {
    /// Saves a pipeline
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;

    /// Finds a pipeline by ID
    async fn find_by_id(&self, id: PipelineId)
        -> Result<Option<Pipeline>, PipelineError>;

    /// Finds a pipeline by name
    async fn find_by_name(&self, name: &str)
        -> Result<Option<Pipeline>, PipelineError>;

    /// Lists all pipelines
    async fn list_all(&self) -> Result<Vec<Pipeline>, PipelineError>;

    /// Lists pipelines with pagination
    async fn list_paginated(&self, offset: usize, limit: usize)
        -> Result<Vec<Pipeline>, PipelineError>;

    /// Updates a pipeline
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;

    /// Deletes a pipeline by ID
    async fn delete(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Checks if a pipeline exists
    async fn exists(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Counts total pipelines
    async fn count(&self) -> Result<usize, PipelineError>;

    /// Finds pipelines by configuration parameter
    async fn find_by_config(&self, key: &str, value: &str)
        -> Result<Vec<Pipeline>, PipelineError>;

    /// Archives a pipeline (soft delete)
    async fn archive(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Restores an archived pipeline
    async fn restore(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Lists archived pipelines
    async fn list_archived(&self) -> Result<Vec<Pipeline>, PipelineError>;
}
```

### Thread Safety

All repository implementations must be `Send + Sync` for concurrent access:

```rust
// ✅ CORRECT: Thread-safe repository
pub struct SqlitePipelineRepository {
    pool: SqlitePool, // SqlitePool is Send + Sync
}

// ❌ WRONG: Not thread-safe
pub struct UnsafeRepository {
    conn: Rc<Connection>, // Rc is not Send or Sync
}
```

## SQLite Implementation

### Architecture

The SQLite repository implements the domain interface using sqlx for type-safe queries:

```rust
use pipeline_domain::repositories::PipelineRepository;
use sqlx::SqlitePool;

pub struct SqlitePipelineRepository {
    pool: SqlitePool,
}

impl SqlitePipelineRepository {
    pub async fn new(database_path: &str) -> Result<Self, PipelineError> {
        let database_url = format!("sqlite:{}", database_path);
        let pool = SqlitePool::connect(&database_url)
            .await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to connect: {}", e)
            ))?;

        Ok(Self { pool })
    }
}
```

### Database Schema

The repository uses a normalized relational schema:

#### Pipelines Table

```sql
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    archived BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_archived ON pipelines(archived);
```

#### Pipeline Stages Table

```sql
CREATE TABLE pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    name TEXT NOT NULL,
    stage_type TEXT NOT NULL,
    algorithm TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    order_index INTEGER NOT NULL,
    parallel_processing BOOLEAN NOT NULL DEFAULT 0,
    chunk_size INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

CREATE INDEX idx_stages_pipeline ON pipeline_stages(pipeline_id);
CREATE INDEX idx_stages_order ON pipeline_stages(pipeline_id, order_index);
```

#### Pipeline Configuration Table

```sql
CREATE TABLE pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

#### Pipeline Metrics Table

```sql
CREATE TABLE pipeline_metrics (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    bytes_processed INTEGER NOT NULL DEFAULT 0,
    bytes_total INTEGER NOT NULL DEFAULT 0,
    chunks_processed INTEGER NOT NULL DEFAULT 0,
    chunks_total INTEGER NOT NULL DEFAULT 0,
    start_time TEXT,
    end_time TEXT,
    throughput_mbps REAL NOT NULL DEFAULT 0.0,
    compression_ratio REAL,
    error_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    recorded_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

CREATE INDEX idx_metrics_pipeline ON pipeline_metrics(pipeline_id);
```

## CRUD Operations

### Create (Save)

Save a complete pipeline with all related data:

```rust
#[async_trait]
impl PipelineRepository for SqlitePipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        // Start transaction for atomicity
        let mut tx = self.pool.begin().await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to start transaction: {}", e)
            ))?;

        // Insert pipeline
        sqlx::query(
            "INSERT INTO pipelines
             (id, name, description, archived, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(pipeline.id().to_string())
        .bind(pipeline.name())
        .bind(pipeline.description())
        .bind(pipeline.archived())
        .bind(pipeline.created_at().to_rfc3339())
        .bind(pipeline.updated_at().to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| PipelineError::database_error(
            format!("Failed to insert pipeline: {}", e)
        ))?;

        // Insert stages
        for (index, stage) in pipeline.stages().iter().enumerate() {
            sqlx::query(
                "INSERT INTO pipeline_stages
                 (id, pipeline_id, name, stage_type, algorithm, enabled,
                  order_index, parallel_processing, chunk_size,
                  created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(stage.id().to_string())
            .bind(pipeline.id().to_string())
            .bind(stage.name())
            .bind(stage.stage_type().to_string())
            .bind(stage.algorithm().name())
            .bind(stage.enabled())
            .bind(index as i64)
            .bind(stage.parallel_processing())
            .bind(stage.chunk_size().map(|cs| cs.as_u64() as i64))
            .bind(stage.created_at().to_rfc3339())
            .bind(stage.updated_at().to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to insert stage: {}", e)
            ))?;
        }

        // Insert configuration
        for (key, value) in pipeline.configuration() {
            sqlx::query(
                "INSERT INTO pipeline_configuration (pipeline_id, key, value)
                 VALUES (?, ?, ?)"
            )
            .bind(pipeline.id().to_string())
            .bind(key)
            .bind(value)
            .execute(&mut *tx)
            .await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to insert config: {}", e)
            ))?;
        }

        // Commit transaction
        tx.commit().await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to commit: {}", e)
            ))?;

        Ok(())
    }
}
```

### Read (Find)

Retrieve pipelines with all related data:

```rust
impl SqlitePipelineRepository {
    async fn find_by_id(&self, id: PipelineId)
        -> Result<Option<Pipeline>, PipelineError> {
        // Fetch pipeline
        let pipeline_row = sqlx::query(
            "SELECT id, name, description, archived, created_at, updated_at
             FROM pipelines WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PipelineError::database_error(
            format!("Failed to fetch pipeline: {}", e)
        ))?;

        let Some(row) = pipeline_row else {
            return Ok(None);
        };

        // Fetch stages
        let stage_rows = sqlx::query(
            "SELECT id, name, stage_type, algorithm, enabled,
                    order_index, parallel_processing, chunk_size,
                    created_at, updated_at
             FROM pipeline_stages
             WHERE pipeline_id = ?
             ORDER BY order_index"
        )
        .bind(id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PipelineError::database_error(
            format!("Failed to fetch stages: {}", e)
        ))?;

        // Fetch configuration
        let config_rows = sqlx::query(
            "SELECT key, value FROM pipeline_configuration
             WHERE pipeline_id = ?"
        )
        .bind(id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PipelineError::database_error(
            format!("Failed to fetch config: {}", e)
        ))?;

        // Map rows to domain entities
        let pipeline = self.map_to_pipeline(row, stage_rows, config_rows)?;

        Ok(Some(pipeline))
    }
}
```

### Update

Update existing pipeline:

```rust
impl SqlitePipelineRepository {
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to start transaction: {}", e)
            ))?;

        // Update pipeline
        sqlx::query(
            "UPDATE pipelines
             SET name = ?, description = ?, archived = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(pipeline.name())
        .bind(pipeline.description())
        .bind(pipeline.archived())
        .bind(pipeline.updated_at().to_rfc3339())
        .bind(pipeline.id().to_string())
        .execute(&mut *tx)
        .await
        .map_err(|e| PipelineError::database_error(
            format!("Failed to update pipeline: {}", e)
        ))?;

        // Delete and re-insert stages (simpler than updating)
        sqlx::query("DELETE FROM pipeline_stages WHERE pipeline_id = ?")
            .bind(pipeline.id().to_string())
            .execute(&mut *tx)
            .await?;

        // Insert updated stages
        for (index, stage) in pipeline.stages().iter().enumerate() {
            // ... (same as save operation)
        }

        tx.commit().await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to commit: {}", e)
            ))?;

        Ok(())
    }
}
```

### Delete

Remove pipeline and all related data:

```rust
impl SqlitePipelineRepository {
    async fn delete(&self, id: PipelineId) -> Result<bool, PipelineError> {
        let result = sqlx::query("DELETE FROM pipelines WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| PipelineError::database_error(
                format!("Failed to delete: {}", e)
            ))?;

        // CASCADE will automatically delete related records
        Ok(result.rows_affected() > 0)
    }
}
```

## Advanced Queries

### Pagination

Efficiently paginate large result sets:

```rust
impl SqlitePipelineRepository {
    async fn list_paginated(&self, offset: usize, limit: usize)
        -> Result<Vec<Pipeline>, PipelineError> {
        let rows = sqlx::query(
            "SELECT id, name, description, archived, created_at, updated_at
             FROM pipelines
             ORDER BY created_at DESC
             LIMIT ? OFFSET ?"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        // Load stages and config for each pipeline
        let mut pipelines = Vec::new();
        for row in rows {
            let id = PipelineId::parse(&row.get::<String, _>("id"))?;
            if let Some(pipeline) = self.find_by_id(id).await? {
                pipelines.push(pipeline);
            }
        }

        Ok(pipelines)
    }
}
```

### Configuration Search

Find pipelines by configuration:

```rust
impl SqlitePipelineRepository {
    async fn find_by_config(&self, key: &str, value: &str)
        -> Result<Vec<Pipeline>, PipelineError> {
        let rows = sqlx::query(
            "SELECT DISTINCT p.id
             FROM pipelines p
             JOIN pipeline_configuration pc ON p.id = pc.pipeline_id
             WHERE pc.key = ? AND pc.value = ?"
        )
        .bind(key)
        .bind(value)
        .fetch_all(&self.pool)
        .await?;

        let mut pipelines = Vec::new();
        for row in rows {
            let id = PipelineId::parse(&row.get::<String, _>("id"))?;
            if let Some(pipeline) = self.find_by_id(id).await? {
                pipelines.push(pipeline);
            }
        }

        Ok(pipelines)
    }
}
```

### Archive Operations

Soft delete with archive/restore:

```rust
impl SqlitePipelineRepository {
    async fn archive(&self, id: PipelineId) -> Result<bool, PipelineError> {
        let result = sqlx::query(
            "UPDATE pipelines SET archived = 1, updated_at = ?
             WHERE id = ?"
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn restore(&self, id: PipelineId) -> Result<bool, PipelineError> {
        let result = sqlx::query(
            "UPDATE pipelines SET archived = 0, updated_at = ?
             WHERE id = ?"
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn list_archived(&self) -> Result<Vec<Pipeline>, PipelineError> {
        let rows = sqlx::query(
            "SELECT id, name, description, archived, created_at, updated_at
             FROM pipelines WHERE archived = 1"
        )
        .fetch_all(&self.pool)
        .await?;

        // Load full pipelines
        let mut pipelines = Vec::new();
        for row in rows {
            let id = PipelineId::parse(&row.get::<String, _>("id"))?;
            if let Some(pipeline) = self.find_by_id(id).await? {
                pipelines.push(pipeline);
            }
        }

        Ok(pipelines)
    }
}
```

## Transaction Management

### ACID Guarantees

Ensure data consistency with transactions:

```rust
impl SqlitePipelineRepository {
    /// Execute multiple operations atomically
    async fn save_multiple(&self, pipelines: &[Pipeline])
        -> Result<(), PipelineError> {
        let mut tx = self.pool.begin().await?;

        for pipeline in pipelines {
            // All operations use the same transaction
            self.save_in_transaction(&mut tx, pipeline).await?;
        }

        // Commit all or rollback all
        tx.commit().await
            .map_err(|e| PipelineError::database_error(
                format!("Transaction commit failed: {}", e)
            ))?;

        Ok(())
    }

    async fn save_in_transaction(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        pipeline: &Pipeline
    ) -> Result<(), PipelineError> {
        // Insert using transaction
        sqlx::query("INSERT INTO pipelines ...")
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
```

### Rollback on Error

Automatic rollback ensures consistency:

```rust
async fn complex_operation(&self, pipeline: &Pipeline)
    -> Result<(), PipelineError> {
    let mut tx = self.pool.begin().await?;

    // Step 1: Insert pipeline
    sqlx::query("INSERT INTO pipelines ...")
        .execute(&mut *tx)
        .await?;

    // Step 2: Insert stages
    for stage in pipeline.stages() {
        sqlx::query("INSERT INTO pipeline_stages ...")
            .execute(&mut *tx)
            .await?;
        // If this fails, Step 1 is automatically rolled back
    }

    // Commit only if all steps succeed
    tx.commit().await?;
    Ok(())
}
```

## Error Handling

### Database Errors

Handle various database error types:

```rust
impl SqlitePipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        match sqlx::query("INSERT INTO pipelines ...").execute(&self.pool).await {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) => {
                if db_err.is_unique_violation() {
                    Err(PipelineError::AlreadyExists(pipeline.id().to_string()))
                } else if db_err.is_foreign_key_violation() {
                    Err(PipelineError::InvalidReference(
                        "Invalid foreign key".to_string()
                    ))
                } else {
                    Err(PipelineError::database_error(db_err.to_string()))
                }
            }
            Err(e) => Err(PipelineError::database_error(e.to_string())),
        }
    }
}
```

### Connection Failures

Handle connection issues gracefully:

```rust
impl SqlitePipelineRepository {
    async fn with_retry<F, T>(&self, mut operation: F) -> Result<T, PipelineError>
    where
        F: FnMut() -> BoxFuture<'_, Result<T, PipelineError>>,
    {
        let max_retries = 3;
        let mut attempts = 0;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(PipelineError::DatabaseError(_)) if attempts < max_retries => {
                    attempts += 1;
                    tokio::time::sleep(
                        Duration::from_millis(100 * 2_u64.pow(attempts))
                    ).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

## Performance Optimizations

### Connection Pooling

Configure optimal pool settings:

```rust
use sqlx::sqlite::SqlitePoolOptions;

impl SqlitePipelineRepository {
    pub async fn new_with_pool_config(
        database_path: &str,
        max_connections: u32,
    ) -> Result<Self, PipelineError> {
        let database_url = format!("sqlite:{}", database_path);

        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }
}
```

### Batch Operations

Optimize bulk inserts:

```rust
impl SqlitePipelineRepository {
    async fn save_batch(&self, pipelines: &[Pipeline])
        -> Result<(), PipelineError> {
        let mut tx = self.pool.begin().await?;

        // Build batch insert query
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO pipelines
             (id, name, description, archived, created_at, updated_at)"
        );

        query_builder.push_values(pipelines, |mut b, pipeline| {
            b.push_bind(pipeline.id().to_string())
             .push_bind(pipeline.name())
             .push_bind(pipeline.description())
             .push_bind(pipeline.archived())
             .push_bind(pipeline.created_at().to_rfc3339())
             .push_bind(pipeline.updated_at().to_rfc3339());
        });

        query_builder.build()
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
```

### Query Optimization

Use indexes and optimized queries:

```rust
// ✅ GOOD: Uses index on pipeline_id
sqlx::query(
    "SELECT * FROM pipeline_stages
     WHERE pipeline_id = ?
     ORDER BY order_index"
)
.bind(id)
.fetch_all(&pool)
.await?;

// ❌ BAD: Full table scan
sqlx::query(
    "SELECT * FROM pipeline_stages
     WHERE name LIKE '%test%'"
)
.fetch_all(&pool)
.await?;

// ✅ BETTER: Use full-text search or specific index
sqlx::query(
    "SELECT * FROM pipeline_stages
     WHERE name = ?"
)
.bind("test")
.fetch_all(&pool)
.await?;
```

## Testing Strategies

### In-Memory Repository

Create test implementation:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InMemoryPipelineRepository {
    pipelines: Arc<Mutex<HashMap<PipelineId, Pipeline>>>,
}

impl InMemoryPipelineRepository {
    pub fn new() -> Self {
        Self {
            pipelines: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl PipelineRepository for InMemoryPipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        let mut pipelines = self.pipelines.lock().unwrap();

        if pipelines.contains_key(pipeline.id()) {
            return Err(PipelineError::AlreadyExists(
                pipeline.id().to_string()
            ));
        }

        pipelines.insert(pipeline.id().clone(), pipeline.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: PipelineId)
        -> Result<Option<Pipeline>, PipelineError> {
        let pipelines = self.pipelines.lock().unwrap();
        Ok(pipelines.get(&id).cloned())
    }

    // ... implement other methods
}
```

### Unit Tests

Test repository operations:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_find() {
        let repo = InMemoryPipelineRepository::new();
        let pipeline = Pipeline::new("test".to_string(), vec![])?;

        // Save
        repo.save(&pipeline).await.unwrap();

        // Find
        let found = repo.find_by_id(pipeline.id().clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.id(), pipeline.id());
        assert_eq!(found.name(), pipeline.name());
    }

    #[tokio::test]
    async fn test_duplicate_save_fails() {
        let repo = InMemoryPipelineRepository::new();
        let pipeline = Pipeline::new("test".to_string(), vec![])?;

        repo.save(&pipeline).await.unwrap();

        let result = repo.save(&pipeline).await;
        assert!(matches!(result, Err(PipelineError::AlreadyExists(_))));
    }
}
```

### Integration Tests

Test with real database:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    async fn create_test_db() -> SqlitePipelineRepository {
        SqlitePipelineRepository::new(":memory:").await.unwrap()
    }

    #[tokio::test]
    async fn test_transaction_rollback() {
        let repo = create_test_db().await;
        let pipeline = Pipeline::new("test".to_string(), vec![])?;

        // Start transaction
        let mut tx = repo.pool.begin().await.unwrap();

        // Insert pipeline
        sqlx::query("INSERT INTO pipelines ...")
            .execute(&mut *tx)
            .await
            .unwrap();

        // Rollback
        tx.rollback().await.unwrap();

        // Verify pipeline was not saved
        let found = repo.find_by_id(pipeline.id().clone()).await.unwrap();
        assert!(found.is_none());
    }
}
```

## Best Practices

### Use Parameterized Queries

Prevent SQL injection:

```rust
// ✅ GOOD: Parameterized query
sqlx::query("SELECT * FROM pipelines WHERE name = ?")
    .bind(name)
    .fetch_one(&pool)
    .await?;

// ❌ BAD: String concatenation (SQL injection risk!)
let query = format!("SELECT * FROM pipelines WHERE name = '{}'", name);
sqlx::query(&query).fetch_one(&pool).await?;
```

### Handle NULL Values

Properly handle nullable columns:

```rust
let description: Option<String> = row.try_get("description")?;
let chunk_size: Option<i64> = row.try_get("chunk_size")?;

let pipeline = Pipeline {
    description: description.unwrap_or_default(),
    chunk_size: chunk_size.map(|cs| ChunkSize::new(cs as u64)?),
    // ...
};
```

### Use Foreign Keys

Maintain referential integrity:

```sql
CREATE TABLE pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    -- ... other columns
    FOREIGN KEY (pipeline_id)
        REFERENCES pipelines(id)
        ON DELETE CASCADE
);
```

### Index Strategic Columns

Optimize query performance:

```sql
-- Primary lookups
CREATE INDEX idx_pipelines_id ON pipelines(id);
CREATE INDEX idx_pipelines_name ON pipelines(name);

-- Filtering
CREATE INDEX idx_pipelines_archived ON pipelines(archived);

-- Foreign keys
CREATE INDEX idx_stages_pipeline ON pipeline_stages(pipeline_id);

-- Sorting
CREATE INDEX idx_stages_order
    ON pipeline_stages(pipeline_id, order_index);
```

## Next Steps

Now that you understand repository implementation:

- [Schema Management](schema.md) - Database migrations and versioning
- [Binary Format](binary-format.md) - File persistence patterns
- [Observability](observability.md) - Monitoring and metrics
- [Testing](../advanced/testing.md) - Comprehensive testing strategies
