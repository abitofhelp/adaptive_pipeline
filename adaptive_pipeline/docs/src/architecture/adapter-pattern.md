# Hexagonal Architecture (Ports and Adapters)

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

Hexagonal Architecture, also known as **Ports and Adapters**, is a pattern that isolates the core business logic (domain) from external concerns. The pipeline system uses this pattern to keep the domain pure and infrastructure replaceable.

![Hexagonal Architecture](../diagrams/hexagonal-architecture.svg)

## The Hexagon Metaphor

Think of your application as a hexagon:

```text
                     ┌─────────────────┐
                     │   Primary       │
                     │   Adapters      │
                     │  (Drivers)      │
                     └────────┬────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        │              ┌──────▼──────┐              │
        │              │             │              │
        │              │   Domain    │              │
        │              │    (Core)   │              │
        │              │             │              │
        │              └──────┬──────┘              │
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                     ┌────────▼────────┐
                     │   Secondary     │
                     │   Adapters      │
                     │  (Driven)       │
                     └─────────────────┘
```

- **The Hexagon (Core)**: Your domain logic - completely independent
- **Ports**: Interfaces that define how to interact with the core
- **Adapters**: Implementations that connect the core to the outside world

## Ports: The Interfaces

**Ports** are interfaces defined by the domain layer. They specify what the domain needs without caring about implementation details.

### Primary Ports (Driving)

Primary ports define **use cases** - what the application can do. External systems drive the application through these ports.

```rust
// Domain layer defines the interface (port)
#[async_trait]
pub trait FileProcessorService: Send + Sync {
    async fn process_file(
        &self,
        pipeline_id: &PipelineId,
        input_path: &FilePath,
        output_path: &FilePath,
    ) -> Result<ProcessingMetrics, PipelineError>;
}
```

**Examples in our system:**
- `FileProcessorService` - File processing operations
- `PipelineService` - Pipeline management operations

### Secondary Ports (Driven)

Secondary ports define **dependencies** - what the domain needs from the outside world. The application drives these external systems.

```rust
// Domain layer defines what it needs (port)
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn create(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError>;
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn delete(&self, id: &PipelineId) -> Result<(), PipelineError>;
}

#[async_trait]
pub trait CompressionService: Send + Sync {
    async fn compress(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
    ) -> Result<Vec<u8>, PipelineError>;

    async fn decompress(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
    ) -> Result<Vec<u8>, PipelineError>;
}
```

**Examples in our system:**
- `PipelineRepository` - Data persistence
- `CompressionService` - Data compression
- `EncryptionService` - Data encryption
- `ChecksumService` - Integrity verification

## Adapters: The Implementations

**Adapters** are concrete implementations of ports. They translate between the domain and external systems.

### Primary Adapters (Driving)

Primary adapters **drive** the application. They take input from the outside world and call the domain.

#### CLI Adapter (main.rs)

```rust
// Primary adapter - drives the application
#[tokio::main]
async fn main() -> std::process::ExitCode {
    // 1. Parse user input
    let cli = bootstrap::bootstrap_cli()?;

    // 2. Set up infrastructure (dependency injection)
    let services = setup_services().await?;

    // 3. Drive the domain through primary port
    match cli.command {
        Commands::Process { input, output, pipeline } => {
            // Call domain through FileProcessorService port
            services.file_processor
                .process_file(&pipeline, &input, &output)
                .await?
        }
        Commands::Create { name, stages } => {
            // Call domain through PipelineService port
            services.pipeline_service
                .create_pipeline(&name, stages)
                .await?
        }
        // ... more commands
    }
}
```

**Key characteristics:**
- Translates user input to domain operations
- Handles presentation concerns (formatting, errors)
- Drives the application core

#### HTTP API Adapter (future)

```rust
// Another primary adapter for HTTP API
async fn handle_process_request(
    req: HttpRequest,
    services: Arc<Services>,
) -> HttpResponse {
    // Parse HTTP request
    let body: ProcessFileRequest = req.json().await?;

    // Drive domain through the same port
    let result = services.file_processor
        .process_file(&body.pipeline_id, &body.input, &body.output)
        .await;

    // Convert result to HTTP response
    match result {
        Ok(metrics) => HttpResponse::Ok().json(metrics),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
```

**Notice:** Both CLI and HTTP adapters use the **same domain ports**. The domain doesn't know or care which adapter is calling it.

### Secondary Adapters (Driven)

Secondary adapters are **driven by** the application. They implement the interfaces the domain needs.

#### SQLite Repository Adapter

```rust
// Infrastructure layer - implements domain port
pub struct SQLitePipelineRepository {
    pool: SqlitePool,
}

#[async_trait]
impl PipelineRepository for SQLitePipelineRepository {
    async fn create(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        // Convert domain entity to database row
        let row = PipelineRow::from_domain(pipeline);

        // Persist to SQLite
        sqlx::query(
            "INSERT INTO pipelines (id, name, archived, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&row.id)
        .bind(&row.name)
        .bind(row.archived)
        .bind(&row.created_at)
        .bind(&row.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| PipelineError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError> {
        // Query SQLite
        let row = sqlx::query_as::<_, PipelineRow>(
            "SELECT * FROM pipelines WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PipelineError::RepositoryError(e.to_string()))?;

        // Convert database row to domain entity
        row.map(|r| Pipeline::from_database(r)).transpose()
    }
}
```

**Key characteristics:**
- Implements domain-defined interface
- Handles database-specific operations
- Translates between domain models and database rows
- Can be swapped without changing domain

#### Compression Service Adapter

```rust
// Infrastructure layer - implements domain port
pub struct CompressionServiceAdapter {
    // Internal state for compression libraries
}

#[async_trait]
impl CompressionService for CompressionServiceAdapter {
    async fn compress(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
    ) -> Result<Vec<u8>, PipelineError> {
        // Route to appropriate compression library
        match algorithm.name() {
            "brotli" => {
                let mut compressed = Vec::new();
                brotli::BrotliCompress(
                    &mut Cursor::new(data),
                    &mut compressed,
                    &Default::default(),
                )?;
                Ok(compressed)
            }
            "zstd" => {
                let compressed = zstd::encode_all(data, 3)?;
                Ok(compressed)
            }
            "lz4" => {
                let compressed = lz4::block::compress(data, None, false)?;
                Ok(compressed)
            }
            _ => Err(PipelineError::UnsupportedAlgorithm(
                algorithm.name().to_string()
            )),
        }
    }

    async fn decompress(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
    ) -> Result<Vec<u8>, PipelineError> {
        // Similar implementation for decompression
        // ...
    }
}
```

**Key characteristics:**
- Wraps external libraries (brotli, zstd, lz4)
- Implements domain interface
- Handles library-specific details
- Can be swapped for different implementations

## Benefits of Hexagonal Architecture

### 1. Testability

You can test the domain in isolation using mock adapters:

```rust
// Mock adapter for testing
struct MockPipelineRepository {
    pipelines: Mutex<HashMap<PipelineId, Pipeline>>,
}

#[async_trait]
impl PipelineRepository for MockPipelineRepository {
    async fn create(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        self.pipelines.lock().unwrap()
            .insert(pipeline.id().clone(), pipeline.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError> {
        Ok(self.pipelines.lock().unwrap().get(id).cloned())
    }
}

#[tokio::test]
async fn test_file_processor_service() {
    // Use mock adapter instead of real database
    let repo = Arc::new(MockPipelineRepository::new());
    let service = FileProcessorService::new(repo);

    // Test domain logic without database
    let result = service.process_file(/* ... */).await;
    assert!(result.is_ok());
}
```

### 2. Flexibility

Swap implementations without changing the domain:

```rust
// Start with SQLite
let repo: Arc<dyn PipelineRepository> =
    Arc::new(SQLitePipelineRepository::new(pool));

// Later, switch to PostgreSQL
let repo: Arc<dyn PipelineRepository> =
    Arc::new(PostgresPipelineRepository::new(pool));

// Domain doesn't change - same interface!
let service = FileProcessorService::new(repo);
```

### 3. Multiple Interfaces

Support multiple input sources using the same domain:

```rust
// CLI adapter
async fn cli_handler(cli: Cli, services: Arc<Services>) {
    services.file_processor.process_file(/* ... */).await?;
}

// HTTP adapter
async fn http_handler(req: HttpRequest, services: Arc<Services>) {
    services.file_processor.process_file(/* ... */).await?;
}

// gRPC adapter
async fn grpc_handler(req: GrpcRequest, services: Arc<Services>) {
    services.file_processor.process_file(/* ... */).await?;
}
```

All three adapters use the **same domain logic** through the **same port**.

### 4. Technology Independence

The domain doesn't depend on specific technologies:

```rust
// Domain doesn't know about:
// - SQLite, PostgreSQL, or MongoDB
// - HTTP, gRPC, or CLI
// - Brotli, Zstd, or LZ4
// - Any specific framework or library

// It only knows about:
// - Business concepts (Pipeline, Stage, Chunk)
// - Business rules (validation, ordering)
// - Interfaces it needs (Repository, CompressionService)
```

## Dependency Inversion

Hexagonal Architecture relies on **Dependency Inversion Principle**:

```text
Traditional:                    Hexagonal:

┌──────────┐                   ┌──────────┐
│   CLI    │                   │   CLI    │
└────┬─────┘                   └────┬─────┘
     │ depends on                   │ depends on
     ▼                              ▼
┌──────────┐                   ┌──────────┐
│ Domain   │                   │  Port    │ ← Interface
└────┬─────┘                   │ (trait)  │
     │ depends on               └────△─────┘
     ▼                               │ implements
┌──────────┐                   ┌────┴─────┐
│ Database │                   │  Domain  │
└──────────┘                   └──────────┘
                                     △
                                     │ implements
                               ┌─────┴─────┐
                               │ Database  │
                               │ Adapter   │
                               └───────────┘
```

**Traditional:** Domain depends on Database (tight coupling)
**Hexagonal:** Database depends on Domain interface (loose coupling)

## Our Adapter Structure

```text
pipeline/src/
├── infrastructure/
│   └── adapters/
│       ├── compression_service_adapter.rs    # Implements CompressionService
│       ├── encryption_service_adapter.rs     # Implements EncryptionService
│       ├── async_compression_adapter.rs      # Async wrapper
│       ├── async_encryption_adapter.rs       # Async wrapper
│       └── repositories/
│           ├── sqlite_repository_adapter.rs  # Implements PipelineRepository
│           └── sqlite_base_repository.rs     # Base repository utilities
```

## Adapter Responsibilities

### What Adapters Should Do

✅ **Translate** between domain and external systems
✅ **Handle** technology-specific details
✅ **Implement** domain-defined interfaces
✅ **Convert** data formats (domain ↔ database, domain ↔ API)
✅ **Manage** external resources (connections, files, etc.)

### What Adapters Should NOT Do

❌ **Contain business logic** - belongs in domain
❌ **Make business decisions** - belongs in domain
❌ **Validate business rules** - belongs in domain
❌ **Know about other adapters** - should be independent
❌ **Expose infrastructure details** to domain

## Example: Complete Flow

Let's trace a complete request through the hexagonal architecture:

```text
1. Primary Adapter (CLI)
   ↓ User types: pipeline process --input file.txt --output file.bin

2. Parse and validate input
   ↓ Create FilePath("/path/to/file.txt")

3. Call Primary Port (FileProcessorService)
   ↓ process_file(pipeline_id, input_path, output_path)

4. Domain Logic
   ├─ Fetch Pipeline (via PipelineRepository port)
   │  └─ Secondary Adapter queries SQLite
   ├─ Process each stage
   │  ├─ Compress (via CompressionService port)
   │  │  └─ Secondary Adapter uses brotli library
   │  ├─ Encrypt (via EncryptionService port)
   │  │  └─ Secondary Adapter uses aes-gcm library
   │  └─ Calculate checksum (via ChecksumService port)
   │     └─ Secondary Adapter uses sha2 library
   └─ Return ProcessingMetrics

5. Primary Adapter formats output
   ↓ Display metrics to user
```

## Common Adapter Patterns

### Repository Adapter Pattern

```rust
// 1. Domain defines interface (port)
pub trait PipelineRepository: Send + Sync {
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>>;
}

// 2. Infrastructure implements adapter
pub struct SQLitePipelineRepository { /* ... */ }

impl PipelineRepository for SQLitePipelineRepository {
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>> {
        // Database-specific implementation
    }
}

// 3. Application uses through interface
pub struct FileProcessorService {
    repository: Arc<dyn PipelineRepository>,  // Uses interface, not concrete type
}
```

### Service Adapter Pattern

```rust
// 1. Domain defines interface
pub trait CompressionService: Send + Sync {
    async fn compress(&self, data: &[u8], algo: &Algorithm) -> Result<Vec<u8>>;
}

// 2. Infrastructure implements adapter
pub struct CompressionServiceAdapter { /* ... */ }

impl CompressionService for CompressionServiceAdapter {
    async fn compress(&self, data: &[u8], algo: &Algorithm) -> Result<Vec<u8>> {
        // Library-specific implementation
    }
}

// 3. Application uses through interface
pub struct StageExecutor {
    compression: Arc<dyn CompressionService>,  // Uses interface
}
```

## Testing with Adapters

### Unit Tests (Domain Layer)

```rust
// Test domain logic without any adapters
#[test]
fn test_pipeline_validation() {
    // Pure domain logic - no infrastructure needed
    let result = Pipeline::new("test", vec![]);
    assert!(result.is_err());
}
```

### Integration Tests (With Mock Adapters)

```rust
#[tokio::test]
async fn test_file_processing() {
    // Use mock adapters
    let mock_repo = Arc::new(MockPipelineRepository::new());
    let mock_compression = Arc::new(MockCompressionService::new());

    let service = FileProcessorService::new(mock_repo, mock_compression);

    // Test without real database or compression
    let result = service.process_file(/* ... */).await;
    assert!(result.is_ok());
}
```

### End-to-End Tests (With Real Adapters)

```rust
#[tokio::test]
async fn test_real_file_processing() {
    // Use real adapters
    let db_pool = create_test_database().await;
    let real_repo = Arc::new(SQLitePipelineRepository::new(db_pool));
    let real_compression = Arc::new(CompressionServiceAdapter::new());

    let service = FileProcessorService::new(real_repo, real_compression);

    // Test with real infrastructure
    let result = service.process_file(/* ... */).await;
    assert!(result.is_ok());
}
```

## Next Steps

Now that you understand hexagonal architecture:

- [Dependency Inversion](dependencies.md) - Managing dependencies properly
- [Layered Architecture](layers.md) - How layers relate to ports/adapters
- [Repository Pattern](repository-pattern.md) - Detailed repository implementation
- [Domain Model](domain-model.md) - Understanding the core domain
