# Repository Pattern

**Version:** 0.1.0
**Date:** October 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

The Repository pattern for data persistence.

## Pattern Overview

The Repository pattern provides an abstraction layer between the domain and data mapping layers. It acts like an in-memory collection of domain objects, hiding the complexities of database operations.

**Key Idea**: Your business logic shouldn't know whether data comes from SQLite, PostgreSQL, or a file. It just uses a `Repository` trait.

## Architecture

![Repository Pattern](../diagrams/repository-pattern.svg)

### Components

**Repository Trait** (Domain Layer)
```rust
trait PipelineRepository {
    fn create(&self, pipeline: &Pipeline) -> Result<()>;
    fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>>;
    fn update(&self, pipeline: &Pipeline) -> Result<()>;
    fn delete(&self, id: &PipelineId) -> Result<()>;
}
```

**Repository Adapter** (Infrastructure Layer)
```rust
struct PipelineRepositoryAdapter {
    repository: SQLitePipelineRepository,
}

impl PipelineRepository for PipelineRepositoryAdapter {
    // Implements trait methods
}
```

**Concrete Repository** (Infrastructure Layer)
```rust
struct SQLitePipelineRepository {
    pool: SqlitePool,
    mapper: PipelineMapper,
}
```

## Layer Responsibilities

### Domain Layer
Defines **what** operations are needed:
```rust
// Domain defines the interface
pub trait PipelineRepository: Send + Sync {
    async fn create(&self, pipeline: &Pipeline) -> Result<()>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>>;
    // ... more methods
}
```

Domain knows:
- What operations it needs
- What domain entities look like
- Business rules and validations

Domain **doesn't know**:
- SQL syntax
- Database technology
- Connection pooling

### Infrastructure Layer
Implements **how** to persist data:
```rust
impl PipelineRepository for PipelineRepositoryAdapter {
    async fn create(&self, pipeline: &Pipeline) -> Result<()> {
        // Convert domain entity to database row
        let row = self.mapper.to_persistence(pipeline);

        // Execute SQL
        sqlx::query("INSERT INTO pipelines ...")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
```

Infrastructure knows:
- SQL syntax and queries
- Database schema
- Connection management
- Error handling

## Data Mapping

The **Mapper** separates domain models from database schema:

```rust
struct PipelineMapper;

impl PipelineMapper {
    // Domain → Database
    fn to_persistence(&self, pipeline: &Pipeline) -> PipelineRow {
        PipelineRow {
            id: pipeline.id().to_string(),
            input_path: pipeline.input_path().to_string(),
            // ... map all fields
        }
    }

    // Database → Domain
    fn to_domain(&self, row: SqliteRow) -> Result<Pipeline> {
        Pipeline::new(
            PipelineId::from_string(&row.id)?,
            FilePath::new(&row.input_path)?,
            FilePath::new(&row.output_path)?,
        )
    }
}
```

**Why mapping?**
- Domain entities stay pure (no database annotations)
- Database schema can change independently
- Different databases can use different schemas
- Validation happens in domain layer

## Benefits

### 1. Testability
Business logic can be tested without a database:

```rust
#[cfg(test)]
mod tests {
    use mockall::mock;

    mock! {
        PipelineRepo {}

        impl PipelineRepository for PipelineRepo {
            async fn create(&self, pipeline: &Pipeline) -> Result<()>;
            // ... mock other methods
        }
    }

    #[tokio::test]
    async fn test_pipeline_service() {
        let mut mock_repo = MockPipelineRepo::new();
        mock_repo.expect_create()
            .returning(|_| Ok(()));

        let service = PipelineService::new(Arc::new(mock_repo));
        // Test business logic without database
    }
}
```

### 2. Flexibility
Swap implementations without changing business logic:

```rust
// Start with SQLite
let repo = SQLitePipelineRepositoryAdapter::new(pool);
let service = PipelineService::new(Arc::new(repo));

// Later, switch to PostgreSQL
let repo = PostgresPipelineRepositoryAdapter::new(pool);
let service = PipelineService::new(Arc::new(repo));
// Business logic unchanged!
```

### 3. Centralized Data Access
All database queries in one place:
- Easier to optimize
- Easier to audit
- Easier to cache
- Easier to add logging

### 4. Domain Purity
Domain layer stays technology-agnostic:
```rust
// Domain doesn't import sqlx, postgres, etc.
// Only depends on standard Rust types
pub struct Pipeline {
    id: PipelineId,           // Not i64 or UUID from database
    input_path: FilePath,     // Not String from database
    status: PipelineStatus,   // Not database enum
}
```

## Usage Example

### Application Layer
```rust
pub struct PipelineService {
    repository: Arc<dyn PipelineRepository>,
}

impl PipelineService {
    pub async fn create_pipeline(
        &self,
        input: FilePath,
        output: FilePath,
    ) -> Result<Pipeline> {
        // Create domain entity
        let pipeline = Pipeline::new(
            PipelineId::new(),
            input,
            output,
        )?;

        // Persist using repository
        self.repository.create(&pipeline).await?;

        Ok(pipeline)
    }

    pub async fn get_pipeline(
        &self,
        id: PipelineId,
    ) -> Result<Option<Pipeline>> {
        self.repository.find_by_id(&id).await
    }
}
```

The service doesn't know or care:
- Which database is used
- How data is stored
- What the SQL looks like

It just uses the `Repository` trait!

## Implementation in Pipeline

Our pipeline uses this pattern for:

**PipelineRepository** - Stores pipeline metadata
- `pipeline/domain/src/repositories/pipeline_repository.rs` (trait)
- `pipeline/src/infrastructure/repositories/sqlite_pipeline_repository.rs` (impl)

**FileChunkRepository** - Stores chunk metadata
- `pipeline/domain/src/repositories/file_chunk_repository.rs` (trait)
- `pipeline/src/infrastructure/repositories/sqlite_file_chunk_repository.rs` (impl)

## Next Steps

Continue to:
- [Service Pattern](service-pattern.md) - Business logic organization
- [Adapter Pattern](adapter-pattern.md) - Infrastructure integration
- [Implementation: Repositories](../implementation/repositories.md) - Concrete implementations
