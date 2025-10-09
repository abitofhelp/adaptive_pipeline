# Data Persistence

**Version:** 0.1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a comprehensive overview of the data persistence architecture in the adaptive pipeline system. Learn how the repository pattern, SQLite database, and schema management work together to provide reliable, efficient data storage.

---

## Table of Contents

- [Overview](#overview)
- [Persistence Architecture](#persistence-architecture)
- [Repository Pattern](#repository-pattern)
- [Database Choice: SQLite](#database-choice-sqlite)
- [Storage Architecture](#storage-architecture)
- [Transaction Management](#transaction-management)
- [Connection Management](#connection-management)
- [Data Mapping](#data-mapping)
- [Performance Optimization](#performance-optimization)
- [Usage Examples](#usage-examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Testing Strategies](#testing-strategies)
- [Next Steps](#next-steps)

---

## Overview

**Data persistence** in the adaptive pipeline system follows Domain-Driven Design principles, separating domain logic from infrastructure concerns through the repository pattern. The system uses SQLite for reliable, zero-configuration data storage with full ACID transaction support.

### Key Features

- **Repository Pattern**: Abstraction layer between domain and infrastructure
- **SQLite Database**: Embedded database with zero configuration
- **Schema Management**: Automated migrations with sqlx
- **ACID Transactions**: Full transactional support for data consistency
- **Connection Pooling**: Efficient connection management
- **Type Safety**: Compile-time query validation

### Persistence Stack

```text
┌──────────────────────────────────────────────────────────┐
│                    Domain Layer                          │
│  ┌────────────────────────────────────────────────┐     │
│  │   PipelineRepository (Trait)                   │     │
│  │   - save(), find_by_id(), list_all()          │     │
│  └────────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────────┘
                         ↓ implements
┌──────────────────────────────────────────────────────────┐
│                Infrastructure Layer                       │
│  ┌────────────────────────────────────────────────┐     │
│  │   SqlitePipelineRepository                     │     │
│  │   - Concrete SQLite implementation             │     │
│  └────────────────────────────────────────────────┘     │
│                         ↓ uses                           │
│  ┌────────────────────────────────────────────────┐     │
│  │   Schema Management                            │     │
│  │   - Migrations, initialization                 │     │
│  └────────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────────┘
                         ↓ persists to
┌──────────────────────────────────────────────────────────┐
│                  SQLite Database                         │
│  ┌────────────┬──────────────┬──────────────────┐      │
│  │ pipelines  │pipeline_stage│pipeline_config   │      │
│  └────────────┴──────────────┴──────────────────┘      │
└──────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**: Domain logic independent of storage technology
2. **Testability**: Easy mocking with in-memory implementations
3. **Flexibility**: Support for different storage backends
4. **Consistency**: ACID transactions ensure data integrity
5. **Performance**: Connection pooling and query optimization

---

## Persistence Architecture

The persistence layer follows a three-tier architecture aligned with Domain-Driven Design.

### Architectural Layers

```text
┌─────────────────────────────────────────────────────────────┐
│ Application Layer                                           │
│  - PipelineService uses repository trait                    │
│  - Business logic remains persistence-agnostic              │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Domain Layer                                                │
│  - PipelineRepository trait (abstract interface)            │
│  - Pipeline, PipelineStage entities                         │
│  - No infrastructure dependencies                           │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Infrastructure Layer                                        │
│  - SqlitePipelineRepository (concrete implementation)       │
│  - Schema management and migrations                         │
│  - Connection pooling and transaction management            │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Storage Layer                                               │
│  - SQLite database file                                     │
│  - Indexes and constraints                                  │
│  - Migration history tracking                               │
└─────────────────────────────────────────────────────────────┘
```

### Component Relationships

**Domain-to-Infrastructure Flow:**

```rust
// Application code depends on domain trait
use adaptive_pipeline_domain::repositories::PipelineRepository;

async fn create_pipeline(
    repo: &dyn PipelineRepository,
    name: String,
) -> Result<Pipeline, PipelineError> {
    let pipeline = Pipeline::new(name)?;
    repo.save(&pipeline).await?;
    Ok(pipeline)
}

// Infrastructure provides concrete implementation
use adaptive_pipeline::infrastructure::repositories::SqlitePipelineRepository;

let repository = SqlitePipelineRepository::new(pool);
let pipeline = create_pipeline(&repository, "my-pipeline".to_string()).await?;
```

### Benefits of This Architecture

| Benefit | Description |
|---------|-------------|
| **Domain Independence** | Business logic doesn't depend on SQLite specifics |
| **Testability** | Easy to mock repositories for unit testing |
| **Flexibility** | Can swap SQLite for PostgreSQL without changing domain |
| **Maintainability** | Clear separation makes code easier to understand |
| **Type Safety** | Compile-time verification of database operations |

---

## Repository Pattern

The repository pattern provides an abstraction layer between domain entities and data storage.

### Repository Pattern Benefits

**1. Separation of Concerns**

Domain logic remains free from persistence details:

```rust
// Domain layer - storage-agnostic
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError>;
    async fn list_all(&self) -> Result<Vec<Pipeline>, PipelineError>;
    async fn delete(&self, id: &PipelineId) -> Result<bool, PipelineError>;
}
```

**2. Implementation Flexibility**

Multiple storage backends can implement the same interface:

```rust
// SQLite implementation
pub struct SqlitePipelineRepository { /* ... */ }

// PostgreSQL implementation
pub struct PostgresPipelineRepository { /* ... */ }

// In-memory testing implementation
pub struct InMemoryPipelineRepository { /* ... */ }

// All implement the same PipelineRepository trait
```

**3. Enhanced Testability**

Mock implementations simplify testing:

```rust
#[cfg(test)]
struct MockPipelineRepository {
    pipelines: Arc<Mutex<HashMap<PipelineId, Pipeline>>>,
}

#[async_trait]
impl PipelineRepository for MockPipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        let mut pipelines = self.pipelines.lock().await;
        pipelines.insert(pipeline.id().clone(), pipeline.clone());
        Ok(())
    }
    // ... implement other methods
}
```

### Repository Interface Design

**Method Categories:**

| Category | Methods | Purpose |
|----------|---------|---------|
| **CRUD** | `save()`, `find_by_id()`, `update()`, `delete()` | Basic operations |
| **Queries** | `find_by_name()`, `find_all()`, `list_paginated()` | Data retrieval |
| **Validation** | `exists()`, `count()` | Existence checks |
| **Lifecycle** | `archive()`, `restore()`, `list_archived()` | Soft deletion |
| **Search** | `find_by_config()` | Advanced queries |

**Complete Interface:**

```rust
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    // CRUD Operations
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError>;
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn delete(&self, id: &PipelineId) -> Result<bool, PipelineError>;

    // Query Operations
    async fn find_by_name(&self, name: &str) -> Result<Option<Pipeline>, PipelineError>;
    async fn find_all(&self) -> Result<Vec<Pipeline>, PipelineError>;
    async fn list_all(&self) -> Result<Vec<Pipeline>, PipelineError>;
    async fn list_paginated(&self, offset: usize, limit: usize)
        -> Result<Vec<Pipeline>, PipelineError>;

    // Validation Operations
    async fn exists(&self, id: &PipelineId) -> Result<bool, PipelineError>;
    async fn count(&self) -> Result<usize, PipelineError>;

    // Lifecycle Operations
    async fn archive(&self, id: &PipelineId) -> Result<bool, PipelineError>;
    async fn restore(&self, id: &PipelineId) -> Result<bool, PipelineError>;
    async fn list_archived(&self) -> Result<Vec<Pipeline>, PipelineError>;

    // Search Operations
    async fn find_by_config(&self, key: &str, value: &str)
        -> Result<Vec<Pipeline>, PipelineError>;
}
```

---

## Database Choice: SQLite

The system uses **SQLite** as the default database for its simplicity, reliability, and zero-configuration deployment.

### Why SQLite?

| Advantage | Description |
|-----------|-------------|
| **Zero Configuration** | No database server to install or configure |
| **Single File** | Entire database stored in one file |
| **ACID Compliant** | Full transactional support |
| **Cross-Platform** | Works on Linux, macOS, Windows |
| **Embedded** | Runs in-process, no network overhead |
| **Reliable** | Battle-tested, used in production worldwide |
| **Fast** | Optimized for local file access |

### SQLite Characteristics

**Performance Profile:**

```text
Operation          | Speed      | Notes
-------------------|------------|--------------------------------
Single INSERT      | ~0.1ms     | Very fast for local file
Batch INSERT       | ~10ms/1000 | Use transactions for batching
Single SELECT      | ~0.05ms    | Fast with proper indexes
Complex JOIN       | ~1-5ms     | Depends on dataset size
Full table scan    | ~10ms/10K  | Avoid without indexes
```

**Limitations:**

- **Concurrent Writes**: Only one writer at a time (readers can be concurrent)
- **Network Access**: Not designed for network file systems
- **Database Size**: Practical limit ~281 TB (theoretical limit)
- **Scalability**: Best for single-server deployments

**When SQLite is Ideal:**

✅ Single-server applications
✅ Embedded systems
✅ Desktop applications
✅ Development and testing
✅ Low-to-medium write concurrency

**When to Consider Alternatives:**

❌ High concurrent write workload
❌ Multi-server deployments
❌ Network file systems
❌ Very large datasets (> 100GB)

### SQLite Configuration

**Connection String:**

```rust
// Local file database
let url = "sqlite://./pipeline.db";

// In-memory database (testing)
let url = "sqlite::memory:";

// Custom connection options
use sqlx::sqlite::SqliteConnectOptions;
let options = SqliteConnectOptions::new()
    .filename("./pipeline.db")
    .create_if_missing(true)
    .foreign_keys(true)
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
```

**Connection Pool Configuration:**

```rust
use sqlx::sqlite::SqlitePoolOptions;

let pool = SqlitePoolOptions::new()
    .max_connections(5)          // Connection pool size
    .min_connections(1)          // Minimum connections
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .connect(&database_url)
    .await?;
```

---

## Storage Architecture

The storage layer uses a normalized relational schema with five core tables.

### Database Schema Overview

```text
┌─────────────┐
│  pipelines  │ (id, name, archived, created_at, updated_at)
└──────┬──────┘
       │ 1:N
       ├──────────────────────────┐
       │                          │
       ↓                          ↓
┌──────────────────┐    ┌──────────────────┐
│ pipeline_stages  │    │pipeline_config   │
│ (id, pipeline_id,│    │(pipeline_id, key,│
│  name, type,     │    │ value)           │
│  algorithm, ...)  │    └──────────────────┘
└──────┬───────────┘
       │ 1:N
       ↓
┌──────────────────┐
│stage_parameters  │
│(stage_id, key,   │
│ value)           │
└──────────────────┘
```

### Table Purposes

| Table | Purpose | Key Fields |
|-------|---------|-----------|
| **pipelines** | Core pipeline entity | id (PK), name (UNIQUE), archived |
| **pipeline_stages** | Stage definitions | id (PK), pipeline_id (FK), stage_order |
| **pipeline_configuration** | Pipeline-level config | (pipeline_id, key) composite PK |
| **stage_parameters** | Stage-level parameters | (stage_id, key) composite PK |
| **processing_metrics** | Execution metrics | pipeline_id (PK/FK), progress, performance |

### Schema Design Principles

**1. Normalization**

Data is normalized to reduce redundancy:

```sql
-- ✅ Normalized: Stages reference pipeline
CREATE TABLE pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,  -- Foreign key
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- ❌ Denormalized: Duplicating pipeline data
-- Each stage would store pipeline name, created_at, etc.
```

**2. Referential Integrity**

Foreign keys enforce data consistency:

```sql
-- CASCADE DELETE: Deleting pipeline removes all stages
FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE

-- Orphaned stages cannot exist
```

**3. Indexing Strategy**

Indexes optimize common queries:

```sql
-- Index on foreign keys for JOIN performance
CREATE INDEX idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);

-- Index on frequently queried fields
CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_archived ON pipelines(archived);
```

**4. Timestamps**

All entities track creation and modification:

```sql
created_at TEXT NOT NULL,  -- RFC 3339 format
updated_at TEXT NOT NULL   -- RFC 3339 format
```

### Schema Initialization

**Automated Migration System:**

```rust
use adaptive_pipeline::infrastructure::repositories::schema;

// High-level initialization (recommended)
let pool = schema::initialize_database("sqlite://./pipeline.db").await?;
// Database created, migrations applied, ready to use!

// Manual initialization
schema::create_database_if_missing("sqlite://./pipeline.db").await?;
let pool = SqlitePool::connect("sqlite://./pipeline.db").await?;
schema::ensure_schema(&pool).await?;
```

**Migration Tracking:**

```sql
-- sqlx automatically creates this table
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
```

For complete schema details, see [Schema Management](schema.md).

---

## Transaction Management

SQLite provides full ACID transaction support for data consistency.

### ACID Properties

| Property | SQLite Implementation |
|----------|----------------------|
| **Atomicity** | All-or-nothing commits via rollback journal |
| **Consistency** | Foreign keys, constraints enforce invariants |
| **Isolation** | Serializable isolation (single writer) |
| **Durability** | WAL mode ensures data persists after commit |

### Transaction Usage

**Explicit Transactions:**

```rust
// Begin transaction
let mut tx = pool.begin().await?;

// Perform multiple operations
sqlx::query("INSERT INTO pipelines (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)")
    .bind(&id)
    .bind(&name)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

sqlx::query("INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, stage_order, algorithm, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
    .bind(&stage_id)
    .bind(&id)
    .bind("compression")
    .bind("compression")
    .bind(0)
    .bind("brotli")
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

// Commit transaction (or rollback on error)
tx.commit().await?;
```

**Automatic Rollback:**

```rust
async fn save_pipeline_with_stages(
    pool: &SqlitePool,
    pipeline: &Pipeline,
) -> Result<(), PipelineError> {
    let mut tx = pool.begin().await?;

    // Insert pipeline
    insert_pipeline(&mut tx, pipeline).await?;

    // Insert all stages
    for stage in pipeline.stages() {
        insert_stage(&mut tx, stage).await?;
    }

    // Commit (or automatic rollback if any operation fails)
    tx.commit().await?;
    Ok(())
}
```

### Transaction Best Practices

**1. Keep Transactions Short**

```rust
// ✅ Good: Short transaction
let mut tx = pool.begin().await?;
sqlx::query("INSERT INTO ...").execute(&mut *tx).await?;
tx.commit().await?;

// ❌ Bad: Long-running transaction
let mut tx = pool.begin().await?;
expensive_computation().await;  // Don't do this inside transaction!
sqlx::query("INSERT INTO ...").execute(&mut *tx).await?;
tx.commit().await?;
```

**2. Handle Errors Gracefully**

```rust
match save_pipeline(&pool, &pipeline).await {
    Ok(()) => info!("Pipeline saved successfully"),
    Err(e) => {
        error!("Failed to save pipeline: {}", e);
        // Transaction automatically rolled back
        return Err(e);
    }
}
```

**3. Use Connection Pool**

```rust
// ✅ Good: Use pool for automatic connection management
async fn save(pool: &SqlitePool, data: &Data) -> Result<(), Error> {
    sqlx::query("INSERT ...").execute(pool).await?;
    Ok(())
}

// ❌ Bad: Creating new connections
async fn save(url: &str, data: &Data) -> Result<(), Error> {
    let pool = SqlitePool::connect(url).await?;  // Expensive!
    sqlx::query("INSERT ...").execute(&pool).await?;
    Ok(())
}
```

---

## Connection Management

Efficient connection management is crucial for performance and resource utilization.

### Connection Pooling

**SqlitePool Benefits:**

- **Connection Reuse**: Avoid overhead of creating new connections
- **Concurrency Control**: Limit concurrent database access
- **Automatic Cleanup**: Close idle connections automatically
- **Health Monitoring**: Detect and recover from connection failures

**Pool Configuration:**

```rust
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

let pool = SqlitePoolOptions::new()
    // Maximum number of connections in pool
    .max_connections(5)

    // Minimum number of idle connections
    .min_connections(1)

    // Timeout for acquiring connection from pool
    .acquire_timeout(Duration::from_secs(30))

    // Close connections idle for this duration
    .idle_timeout(Duration::from_secs(600))

    // Maximum lifetime of a connection
    .max_lifetime(Duration::from_secs(3600))

    // Test connection before returning from pool
    .test_before_acquire(true)

    .connect(&database_url)
    .await?;
```

### Connection Lifecycle

```text
1. Application requests connection
   ↓
2. Pool checks for available connection
   ├─ Available → Reuse existing connection
   └─ Not available → Create new connection (if under max)
   ↓
3. Application uses connection
   ↓
4. Application returns connection to pool
   ↓
5. Pool keeps connection alive (if under idle_timeout)
   ↓
6. Connection eventually closed (after max_lifetime)
```

### Performance Tuning

**Optimal Pool Size:**

```rust
// For CPU-bound workloads
let pool_size = num_cpus::get();

// For I/O-bound workloads
let pool_size = num_cpus::get() * 2;

// For SQLite (single writer)
let pool_size = 5;  // Conservative for write-heavy workloads
```

**Connection Timeout Strategies:**

| Scenario | Timeout | Rationale |
|----------|---------|-----------|
| **Web API** | 5-10 seconds | Fail fast for user requests |
| **Background Job** | 30-60 seconds | More tolerance for delays |
| **Batch Processing** | 2-5 minutes | Long-running operations acceptable |

---

## Data Mapping

Data mapping converts between domain entities and database records.

### Entity-to-Row Mapping

**Domain Entity:**

```rust
pub struct Pipeline {
    id: PipelineId,
    name: String,
    stages: Vec<PipelineStage>,
    archived: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

**Database Row:**

```sql
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT false,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Mapping Logic:**

```rust
// Domain → Database (serialize)
let id_str = pipeline.id().to_string();
let name_str = pipeline.name();
let archived_bool = pipeline.is_archived();
let created_at_str = pipeline.created_at().to_rfc3339();
let updated_at_str = pipeline.updated_at().to_rfc3339();

sqlx::query("INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
    .bind(id_str)
    .bind(name_str)
    .bind(archived_bool)
    .bind(created_at_str)
    .bind(updated_at_str)
    .execute(pool)
    .await?;

// Database → Domain (deserialize)
let row = sqlx::query("SELECT * FROM pipelines WHERE id = ?")
    .bind(id_str)
    .fetch_one(pool)
    .await?;

let id = PipelineId::from(row.get::<String, _>("id"));
let name = row.get::<String, _>("name");
let archived = row.get::<bool, _>("archived");
let created_at = DateTime::parse_from_rfc3339(row.get::<String, _>("created_at"))?;
let updated_at = DateTime::parse_from_rfc3339(row.get::<String, _>("updated_at"))?;
```

### Type Conversions

| Rust Type | SQLite Type | Conversion |
|-----------|-------------|------------|
| `String` | `TEXT` | Direct mapping |
| `i64` | `INTEGER` | Direct mapping |
| `f64` | `REAL` | Direct mapping |
| `bool` | `INTEGER` (0/1) | `sqlx` handles conversion |
| `DateTime<Utc>` | `TEXT` | RFC 3339 string format |
| `PipelineId` | `TEXT` | UUID string representation |
| `Vec<u8>` | `BLOB` | Direct binary mapping |
| `Option<T>` | `NULL` / value | `NULL` for `None` |

### Handling Relationships

**One-to-Many (Pipeline → Stages):**

```rust
async fn load_pipeline_with_stages(
    pool: &SqlitePool,
    id: &PipelineId,
) -> Result<Pipeline, PipelineError> {
    // Load pipeline
    let pipeline_row = sqlx::query("SELECT * FROM pipelines WHERE id = ?")
        .bind(id.to_string())
        .fetch_one(pool)
        .await?;

    // Load related stages
    let stage_rows = sqlx::query("SELECT * FROM pipeline_stages WHERE pipeline_id = ? ORDER BY stage_order")
        .bind(id.to_string())
        .fetch_all(pool)
        .await?;

    // Map to domain entities
    let pipeline = map_pipeline_row(pipeline_row)?;
    let stages = stage_rows.into_iter()
        .map(map_stage_row)
        .collect::<Result<Vec<_>, _>>()?;

    // Combine into aggregate
    pipeline.with_stages(stages)
}
```

For detailed repository implementation, see [Repository Implementation](repositories.md).

---

## Performance Optimization

Several strategies optimize persistence performance.

### Query Optimization

**1. Use Indexes Effectively**

```sql
-- Index on frequently queried columns
CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_archived ON pipelines(archived);

-- Index on foreign keys for JOINs
CREATE INDEX idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
```

**2. Avoid N+1 Queries**

```rust
// ❌ Bad: N+1 query problem
for pipeline_id in pipeline_ids {
    let pipeline = repo.find_by_id(&pipeline_id).await?;
    // Process pipeline...
}

// ✅ Good: Single batch query
let pipelines = repo.find_all().await?;
for pipeline in pipelines {
    // Process pipeline...
}
```

**3. Use Prepared Statements**

```rust
// sqlx automatically uses prepared statements
let pipeline = sqlx::query_as::<_, Pipeline>("SELECT * FROM pipelines WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await?;
```

### Connection Pool Tuning

**Optimal Settings:**

```rust
// For low-concurrency (CLI tools)
.max_connections(2)
.min_connections(1)

// For medium-concurrency (web services)
.max_connections(5)
.min_connections(2)

// For high-concurrency (not recommended for SQLite writes)
.max_connections(10)  // Reading only
.min_connections(5)
```

### Batch Operations

**Batch Inserts:**

```rust
async fn save_multiple_pipelines(
    pool: &SqlitePool,
    pipelines: &[Pipeline],
) -> Result<(), PipelineError> {
    let mut tx = pool.begin().await?;

    for pipeline in pipelines {
        sqlx::query("INSERT INTO pipelines (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)")
            .bind(pipeline.id().to_string())
            .bind(pipeline.name())
            .bind(pipeline.created_at().to_rfc3339())
            .bind(pipeline.updated_at().to_rfc3339())
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}
```

### Performance Benchmarks

| Operation | Latency | Throughput | Notes |
|-----------|---------|------------|-------|
| Single INSERT | ~0.1ms | ~10K/sec | Without transaction |
| Batch INSERT (1000) | ~10ms | ~100K/sec | Within transaction |
| Single SELECT by ID | ~0.05ms | ~20K/sec | With index |
| SELECT with JOIN | ~0.5ms | ~2K/sec | Two-table join |
| Full table scan (10K rows) | ~10ms | ~1K/sec | Without index |

---

## Usage Examples

### Example 1: Basic CRUD Operations

```rust
use adaptive_pipeline::infrastructure::repositories::{schema, SqlitePipelineRepository};
use adaptive_pipeline_domain::Pipeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let pool = schema::initialize_database("sqlite://./pipeline.db").await?;
    let repo = SqlitePipelineRepository::new(pool);

    // Create pipeline
    let pipeline = Pipeline::new("my-pipeline".to_string())?;
    repo.save(&pipeline).await?;
    println!("Saved pipeline: {}", pipeline.id());

    // Read pipeline
    let loaded = repo.find_by_id(pipeline.id()).await?
        .ok_or("Pipeline not found")?;
    println!("Loaded pipeline: {}", loaded.name());

    // Update pipeline
    let mut updated = loaded;
    updated.update_name("renamed-pipeline".to_string())?;
    repo.update(&updated).await?;

    // Delete pipeline
    repo.delete(updated.id()).await?;
    println!("Deleted pipeline");

    Ok(())
}
```

### Example 2: Transaction Management

```rust
use sqlx::SqlitePool;

async fn save_pipeline_atomically(
    pool: &SqlitePool,
    pipeline: &Pipeline,
) -> Result<(), PipelineError> {
    // Begin transaction
    let mut tx = pool.begin().await?;

    // Insert pipeline
    sqlx::query("INSERT INTO pipelines (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)")
        .bind(pipeline.id().to_string())
        .bind(pipeline.name())
        .bind(pipeline.created_at().to_rfc3339())
        .bind(pipeline.updated_at().to_rfc3339())
        .execute(&mut *tx)
        .await?;

    // Insert all stages
    for (i, stage) in pipeline.stages().iter().enumerate() {
        sqlx::query("INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, stage_order, algorithm, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(stage.id().to_string())
            .bind(pipeline.id().to_string())
            .bind(stage.name())
            .bind(stage.stage_type().to_string())
            .bind(i as i64)
            .bind(stage.algorithm())
            .bind(stage.created_at().to_rfc3339())
            .bind(stage.updated_at().to_rfc3339())
            .execute(&mut *tx)
            .await?;
    }

    // Commit transaction (or rollback on error)
    tx.commit().await?;

    Ok(())
}
```

### Example 3: Query with Pagination

```rust
async fn list_pipelines_paginated(
    repo: &dyn PipelineRepository,
    page: usize,
    page_size: usize,
) -> Result<Vec<Pipeline>, PipelineError> {
    let offset = page * page_size;
    repo.list_paginated(offset, page_size).await
}

// Usage
let page_1 = list_pipelines_paginated(&repo, 0, 10).await?;  // First 10
let page_2 = list_pipelines_paginated(&repo, 1, 10).await?;  // Next 10
```

### Example 4: Archive Management

```rust
async fn archive_old_pipelines(
    repo: &dyn PipelineRepository,
    cutoff_date: DateTime<Utc>,
) -> Result<usize, PipelineError> {
    let pipelines = repo.find_all().await?;
    let mut archived_count = 0;

    for pipeline in pipelines {
        if pipeline.created_at() < &cutoff_date {
            repo.archive(pipeline.id()).await?;
            archived_count += 1;
        }
    }

    Ok(archived_count)
}
```

### Example 5: Connection Pool Management

```rust
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

async fn create_optimized_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(3600))
        .connect(database_url)
        .await?;

    Ok(pool)
}
```

---

## Best Practices

### 1. Use Transactions for Multi-Step Operations

```rust
// ✅ Good: Atomic multi-step operation
async fn create_pipeline_with_stages(pool: &SqlitePool, pipeline: &Pipeline) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    insert_pipeline(&mut tx, pipeline).await?;
    insert_stages(&mut tx, pipeline.stages()).await?;
    tx.commit().await?;
    Ok(())
}

// ❌ Bad: Non-atomic operations
async fn create_pipeline_with_stages(pool: &SqlitePool, pipeline: &Pipeline) -> Result<(), Error> {
    insert_pipeline(pool, pipeline).await?;
    insert_stages(pool, pipeline.stages()).await?;  // May fail, leaving orphaned pipeline
    Ok(())
}
```

### 2. Always Use Connection Pooling

```rust
// ✅ Good: Reuse pool
let pool = SqlitePool::connect(&url).await?;
let repo = SqlitePipelineRepository::new(pool.clone());
let service = PipelineService::new(Arc::new(repo));

// ❌ Bad: Create new connections
for _ in 0..100 {
    let pool = SqlitePool::connect(&url).await?;  // Expensive!
    // ...
}
```

### 3. Handle Database Errors Gracefully

```rust
match repo.save(&pipeline).await {
    Ok(()) => info!("Pipeline saved successfully"),
    Err(PipelineError::DatabaseError(msg)) if msg.contains("UNIQUE constraint") => {
        warn!("Pipeline already exists: {}", pipeline.name());
    }
    Err(e) => {
        error!("Failed to save pipeline: {}", e);
        return Err(e);
    }
}
```

### 4. Use Indexes for Frequently Queried Fields

```sql
-- ✅ Good: Index on query columns
CREATE INDEX idx_pipelines_name ON pipelines(name);
SELECT * FROM pipelines WHERE name = ?;  -- Fast!

-- ❌ Bad: No index on query column
-- No index on 'name'
SELECT * FROM pipelines WHERE name = ?;  -- Slow (full table scan)
```

### 5. Keep Transactions Short

```rust
// ✅ Good: Short transaction
let mut tx = pool.begin().await?;
sqlx::query("INSERT ...").execute(&mut *tx).await?;
tx.commit().await?;

// ❌ Bad: Long transaction holding locks
let mut tx = pool.begin().await?;
expensive_computation().await;  // Don't do this!
sqlx::query("INSERT ...").execute(&mut *tx).await?;
tx.commit().await?;
```

### 6. Validate Data Before Persisting

```rust
// ✅ Good: Validate before save
async fn save_pipeline(repo: &dyn PipelineRepository, pipeline: &Pipeline) -> Result<(), Error> {
    pipeline.validate()?;  // Validate first
    repo.save(pipeline).await?;
    Ok(())
}
```

### 7. Use Migrations for Schema Changes

```bash
# ✅ Good: Create migration
sqlx migrate add add_archived_column

# Edit migration file
# migrations/20250101000001_add_archived_column.sql
ALTER TABLE pipelines ADD COLUMN archived BOOLEAN NOT NULL DEFAULT false;

# Apply migration
sqlx migrate run
```

---

## Troubleshooting

### Issue 1: Database Locked Error

**Symptom:**
```text
Error: database is locked
```

**Causes:**
- Long-running transaction blocking other operations
- Multiple writers attempting simultaneous writes
- Connection not released back to pool

**Solutions:**

```rust
// 1. Reduce transaction duration
let mut tx = pool.begin().await?;
// Do minimal work inside transaction
sqlx::query("INSERT ...").execute(&mut *tx).await?;
tx.commit().await?;

// 2. Enable WAL mode for better concurrency
use sqlx::sqlite::SqliteConnectOptions;
let options = SqliteConnectOptions::new()
    .filename("./pipeline.db")
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

// 3. Increase busy timeout
.busy_timeout(Duration::from_secs(5))
```

### Issue 2: Connection Pool Exhausted

**Symptom:**
```text
Error: timed out while waiting for an open connection
```

**Solutions:**

```rust
// 1. Increase pool size
.max_connections(10)

// 2. Increase acquire timeout
.acquire_timeout(Duration::from_secs(60))

// 3. Ensure connections are returned
async fn query_data(pool: &SqlitePool) -> Result<(), Error> {
    let result = sqlx::query("SELECT ...").fetch_all(pool).await?;
    // Connection automatically returned to pool
    Ok(())
}
```

### Issue 3: Foreign Key Constraint Failed

**Symptom:**
```text
Error: FOREIGN KEY constraint failed
```

**Solutions:**

```rust
// 1. Ensure foreign keys are enabled
.foreign_keys(true)

// 2. Verify referenced record exists
let exists = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pipelines WHERE id = ?)")
    .bind(&pipeline_id)
    .fetch_one(pool)
    .await?;

if !exists {
    return Err("Pipeline not found".into());
}

// 3. Use CASCADE DELETE for automatic cleanup
CREATE TABLE pipeline_stages (
    ...
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);
```

### Issue 4: Migration Checksum Mismatch

**Symptom:**
```text
Error: migration checksum mismatch
```

**Solutions:**

```bash
# Option 1: Revert migration
sqlx migrate revert

# Option 2: Reset database (development only!)
rm pipeline.db
sqlx migrate run

# Option 3: Create new migration to fix
sqlx migrate add fix_schema_issue
```

### Issue 5: Query Performance Degradation

**Diagnosis:**

```rust
// Enable query logging
RUST_LOG=sqlx=debug cargo run

// Analyze slow queries
let start = Instant::now();
let result = query.fetch_all(pool).await?;
let duration = start.elapsed();
if duration > Duration::from_millis(100) {
    warn!("Slow query: {:?}", duration);
}
```

**Solutions:**

```sql
-- 1. Add missing indexes
CREATE INDEX idx_pipelines_created_at ON pipelines(created_at);

-- 2. Use EXPLAIN QUERY PLAN
EXPLAIN QUERY PLAN SELECT * FROM pipelines WHERE name = ?;

-- 3. Optimize query
-- Before: Full table scan
SELECT * FROM pipelines WHERE lower(name) = ?;

-- After: Use index
SELECT * FROM pipelines WHERE name = ?;
```

---

## Testing Strategies

### Unit Testing with Mock Repository

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockPipelineRepository {
        pipelines: Arc<Mutex<HashMap<PipelineId, Pipeline>>>,
    }

    #[async_trait]
    impl PipelineRepository for MockPipelineRepository {
        async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
            let mut pipelines = self.pipelines.lock().await;
            pipelines.insert(pipeline.id().clone(), pipeline.clone());
            Ok(())
        }

        async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError> {
            let pipelines = self.pipelines.lock().await;
            Ok(pipelines.get(id).cloned())
        }

        // ... implement other methods
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let repo = MockPipelineRepository {
            pipelines: Arc::new(Mutex::new(HashMap::new())),
        };

        let pipeline = Pipeline::new("test".to_string()).unwrap();
        repo.save(&pipeline).await.unwrap();

        let loaded = repo.find_by_id(pipeline.id()).await.unwrap().unwrap();
        assert_eq!(loaded.name(), "test");
    }
}
```

### Integration Testing with SQLite

```rust
#[tokio::test]
async fn test_sqlite_repository_integration() {
    // Use in-memory database for tests
    let pool = schema::initialize_database("sqlite::memory:").await.unwrap();
    let repo = SqlitePipelineRepository::new(pool);

    // Create pipeline
    let pipeline = Pipeline::new("integration-test".to_string()).unwrap();
    repo.save(&pipeline).await.unwrap();

    // Verify persistence
    let loaded = repo.find_by_id(pipeline.id()).await.unwrap().unwrap();
    assert_eq!(loaded.name(), "integration-test");

    // Verify stages are loaded
    assert_eq!(loaded.stages().len(), pipeline.stages().len());
}
```

### Transaction Testing

```rust
#[tokio::test]
async fn test_transaction_rollback() {
    let pool = schema::initialize_database("sqlite::memory:").await.unwrap();

    let result = async {
        let mut tx = pool.begin().await?;

        sqlx::query("INSERT INTO pipelines (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)")
            .bind("test-id")
            .bind("test")
            .bind("2025-01-01T00:00:00Z")
            .bind("2025-01-01T00:00:00Z")
            .execute(&mut *tx)
            .await?;

        // Simulate error
        return Err::<(), sqlx::Error>(sqlx::Error::RowNotFound);

        // This would commit, but error prevents it
        // tx.commit().await?;
    }.await;

    assert!(result.is_err());

    // Verify rollback - no pipeline should exist
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pipelines")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
```

---

## Next Steps

After understanding data persistence fundamentals, explore specific implementations:

### Detailed Persistence Topics

1. **[Repository Implementation](repositories.md)**: Deep dive into repository pattern implementation
2. **[Schema Management](schema.md)**: Database schema design and migration strategies

### Related Topics

- **[Observability](observability.md)**: Monitoring database operations and performance
- **[Stage Processing](stages.md)**: How stages interact with persistence layer

### Advanced Topics

- **[Performance Optimization](../advanced/performance.md)**: Database query optimization and profiling
- **[Extending the Pipeline](../advanced/extending.md)**: Adding custom persistence backends

---

## Summary

**Key Takeaways:**

1. **Repository Pattern** provides abstraction between domain and infrastructure layers
2. **SQLite** offers zero-configuration, ACID-compliant persistence
3. **Schema Management** uses sqlx migrations for automated database evolution
4. **Connection Pooling** optimizes resource utilization and performance
5. **Transactions** ensure data consistency with ACID guarantees
6. **Data Mapping** converts between domain entities and database records
7. **Performance** optimized through indexing, batching, and query optimization

**Architecture File References:**
- **Repository Interface:** `pipeline-domain/src/repositories/pipeline_repository.rs:138`
- **SQLite Implementation:** `pipeline/src/infrastructure/repositories/sqlite_pipeline_repository.rs:193`
- **Schema Management:** `pipeline/src/infrastructure/repositories/schema.rs:18`
