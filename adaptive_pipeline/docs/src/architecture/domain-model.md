# Domain Model

**Version:** 1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The domain model is the heart of the pipeline system. It captures the core business concepts, rules, and behaviors using Domain-Driven Design (DDD) principles. This chapter explains how the domain model is structured and why it's designed this way.

![Domain Model](../diagrams/domain-model.svg)

## Domain-Driven Design Principles

Domain-Driven Design (DDD) is a software development approach that emphasizes:

1. **Focus on the core domain** - The business logic is the most important part
2. **Model-driven design** - The domain model drives the software design
3. **Ubiquitous language** - Shared vocabulary between developers and domain experts
4. **Bounded contexts** - Clear boundaries between different parts of the system

### Why DDD?

For a pipeline processing system, DDD provides:

- **Clear separation** between business logic and infrastructure
- **Testable code** - Domain logic can be tested without databases or files
- **Flexibility** - Easy to change infrastructure without touching business rules
- **Maintainability** - Business rules are explicit and well-organized

## Core Domain Concepts

### Entities

**Entities** are objects with a unique identity that persists through time. Two entities are equal if they have the same ID, even if all their other attributes differ.

#### Pipeline Entity

The central entity representing a file processing workflow.

```rust
pub struct Pipeline {
    id: PipelineId,                    // Unique identity
    name: String,                      // Human-readable name
    stages: Vec<PipelineStage>,        // Ordered processing stages
    configuration: HashMap<String, String>,  // Custom settings
    metrics: ProcessingMetrics,        // Performance data
    archived: bool,                    // Lifecycle state
    created_at: DateTime<Utc>,         // Creation timestamp
    updated_at: DateTime<Utc>,         // Last modification
}
```

**Key characteristics:**
- Has unique `PipelineId`
- Can be modified while maintaining identity
- Enforces business rules (e.g., must have at least one stage)
- Automatically adds integrity verification stages

**Example:**
```rust
use adaptive_pipeline_domain::Pipeline;

// Two pipelines with same ID are equal, even if names differ
let pipeline1 = Pipeline::new("Original Name", stages.clone())?;
let pipeline2 = pipeline1.clone();
pipeline2.set_name("Different Name");

assert_eq!(pipeline1.id(), pipeline2.id());  // Same identity
```

#### PipelineStage Entity

Represents a single processing operation within a pipeline.

```rust
pub struct PipelineStage {
    id: StageId,                       // Unique identity
    name: String,                      // Stage name
    stage_type: StageType,             // Compression, Encryption, etc.
    configuration: StageConfiguration, // Algorithm and parameters
    order: usize,                      // Execution order
}
```

**Stage Types:**
- `Compression` - Data compression
- `Encryption` - Data encryption
- `Integrity` - Checksum verification
- `Custom` - User-defined operations

#### ProcessingContext Entity

Manages the runtime execution state of a pipeline.

```rust
pub struct ProcessingContext {
    id: ProcessingContextId,           // Unique identity
    pipeline_id: PipelineId,           // Associated pipeline
    input_path: FilePath,              // Input file
    output_path: FilePath,             // Output file
    current_stage: usize,              // Current stage index
    status: ProcessingStatus,          // Running, Completed, Failed
    metrics: ProcessingMetrics,        // Runtime metrics
}
```

#### SecurityContext Entity

Manages security and permissions for pipeline operations.

```rust
pub struct SecurityContext {
    id: SecurityContextId,             // Unique identity
    user_id: UserId,                   // User performing operation
    security_level: SecurityLevel,     // Required security level
    permissions: Vec<Permission>,      // Granted permissions
    encryption_key_id: Option<EncryptionKeyId>,  // Key for encryption
}
```

### Value Objects

**Value Objects** are immutable objects defined by their attributes. Two value objects with the same attributes are considered equal.

#### Algorithm Value Object

Type-safe representation of processing algorithms.

```rust
pub struct Algorithm(String);

impl Algorithm {
    // Predefined compression algorithms
    pub fn brotli() -> Self { /* ... */ }
    pub fn gzip() -> Self { /* ... */ }
    pub fn zstd() -> Self { /* ... */ }
    pub fn lz4() -> Self { /* ... */ }

    // Predefined encryption algorithms
    pub fn aes_256_gcm() -> Self { /* ... */ }
    pub fn chacha20_poly1305() -> Self { /* ... */ }

    // Predefined hashing algorithms
    pub fn sha256() -> Self { /* ... */ }
    pub fn blake3() -> Self { /* ... */ }
}
```

**Key characteristics:**
- Immutable after creation
- Self-validating (enforces format rules)
- Category detection (is_compression(), is_encryption())
- Type-safe (can't accidentally use wrong algorithm)

#### ChunkSize Value Object

Represents validated chunk sizes for file processing.

```rust
pub struct ChunkSize(usize);

impl ChunkSize {
    pub fn new(bytes: usize) -> Result<Self, PipelineError> {
        // Validates size is within acceptable range
        if bytes < MIN_CHUNK_SIZE || bytes > MAX_CHUNK_SIZE {
            return Err(PipelineError::InvalidConfiguration(/* ... */));
        }
        Ok(Self(bytes))
    }

    pub fn from_megabytes(mb: usize) -> Result<Self, PipelineError> {
        Self::new(mb * 1024 * 1024)
    }
}
```

#### FileChunk Value Object

Immutable representation of a piece of file data.

```rust
pub struct FileChunk {
    id: FileChunkId,                   // Unique chunk identifier
    sequence: usize,                   // Position in file
    data: Vec<u8>,                     // Chunk data
    is_final: bool,                    // Last chunk flag
    checksum: Option<String>,          // Integrity verification
}
```

#### FilePath Value Object

Type-safe, validated file paths.

```rust
pub struct FilePath(PathBuf);

impl FilePath {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, PipelineError> {
        let path = path.into();
        // Validation:
        // - Path traversal prevention
        // - Null byte checks
        // - Length limits
        // - Encoding validation
        Ok(Self(path))
    }
}
```

#### PipelineId, StageId, UserId (Type-Safe IDs)

All identifiers are wrapped in newtype value objects for type safety:

```rust
pub struct PipelineId(Ulid);  // Can't accidentally use StageId as PipelineId
pub struct StageId(Ulid);
pub struct UserId(Ulid);
pub struct ProcessingContextId(Ulid);
pub struct SecurityContextId(Ulid);
```

This prevents common bugs like passing the wrong ID to a function.

### Domain Services

**Domain Services** contain business logic that doesn't naturally fit in an entity or value object. They are stateless and operate on domain objects.

Domain services in our system are defined as traits (interfaces) in the domain layer and implemented in the infrastructure layer.

#### CompressionService

```rust
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

#### EncryptionService

```rust
#[async_trait]
pub trait EncryptionService: Send + Sync {
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError>;

    async fn decrypt(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError>;
}
```

#### ChecksumService

```rust
pub trait ChecksumService: Send + Sync {
    fn calculate(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
    ) -> Result<String, PipelineError>;

    fn verify(
        &self,
        data: &[u8],
        expected: &str,
        algorithm: &Algorithm,
    ) -> Result<bool, PipelineError>;
}
```

### Repositories

**Repositories** abstract data persistence, allowing the domain to work with collections without knowing about storage details.

```rust
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn create(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Pipeline>, PipelineError>;
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn delete(&self, id: &PipelineId) -> Result<(), PipelineError>;
    async fn list_all(&self) -> Result<Vec<Pipeline>, PipelineError>;
}
```

The repository interface is defined in the domain layer, but implementations live in the infrastructure layer. This follows the Dependency Inversion Principle.

### Domain Events

**Domain Events** represent significant business occurrences that other parts of the system might care about.

```rust
pub enum DomainEvent {
    PipelineCreated {
        pipeline_id: PipelineId,
        name: String,
        created_at: DateTime<Utc>,
    },
    ProcessingStarted {
        pipeline_id: PipelineId,
        context_id: ProcessingContextId,
        input_path: FilePath,
    },
    ProcessingCompleted {
        pipeline_id: PipelineId,
        context_id: ProcessingContextId,
        metrics: ProcessingMetrics,
    },
    ProcessingFailed {
        pipeline_id: PipelineId,
        context_id: ProcessingContextId,
        error: String,
    },
}
```

Events enable:
- **Loose coupling** - Components don't need direct references
- **Audit trails** - Track all significant operations
- **Integration** - External systems can react to events
- **Event sourcing** - Reconstruct state from event history

## Business Rules and Invariants

The domain model enforces critical business rules:

### Pipeline Rules

1. **Pipelines must have at least one user-defined stage**
   ```rust
   if user_stages.is_empty() {
       return Err(PipelineError::InvalidConfiguration(
           "Pipeline must have at least one stage".to_string()
       ));
   }
   ```

2. **Stage order must be sequential and valid**
   ```rust
   // Stages are automatically reordered: 0, 1, 2, 3...
   // Input checksum = 0
   // User stages = 1, 2, 3...
   // Output checksum = final
   ```

3. **Pipeline names must be unique** (enforced by repository)

### Chunk Processing Rules

1. **Chunks must have non-zero size**
   ```rust
   if size == 0 {
       return Err(PipelineError::InvalidChunkSize);
   }
   ```

2. **Chunk sequence numbers must be sequential**
   ```rust
   // Chunks are numbered 0, 1, 2, 3...
   // Missing sequences cause processing to fail
   ```

3. **Final chunks must be properly marked**
   ```rust
   if chunk.is_final() {
       // No more chunks should follow
   }
   ```

### Security Rules

1. **Security contexts must be validated**
   ```rust
   security_context.validate()?;
   ```

2. **Encryption keys must meet strength requirements**
   ```rust
   if key.len() < MIN_KEY_LENGTH {
       return Err(PipelineError::WeakEncryptionKey);
   }
   ```

3. **Access permissions must be checked**
   ```rust
   if !security_context.has_permission(Permission::ProcessFile) {
       return Err(PipelineError::PermissionDenied);
   }
   ```

## Ubiquitous Language

The domain model uses consistent terminology shared between developers and domain experts:

| Term | Meaning |
|------|---------|
| **Pipeline** | An ordered sequence of processing stages |
| **Stage** | A single processing operation (compress, encrypt, etc.) |
| **Chunk** | A piece of a file processed in parallel |
| **Algorithm** | A specific processing method (zstd, aes-256-gcm, etc.) |
| **Repository** | Storage abstraction for domain objects |
| **Context** | Runtime execution state |
| **Metrics** | Performance and operational measurements |
| **Integrity** | Data verification through checksums |
| **Security Level** | Required protection level (Public, Confidential, Secret) |

## Testing Domain Logic

Domain objects are designed for easy testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_enforces_minimum_stages() {
        // Domain logic can be tested without any infrastructure
        let result = Pipeline::new("test".to_string(), vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn algorithm_validates_format() {
        // Value objects self-validate
        let result = Algorithm::new("INVALID-NAME".to_string());
        assert!(result.is_err());

        let result = Algorithm::new("valid-name".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn chunk_size_enforces_limits() {
        // Business rules are explicit and testable
        let too_small = ChunkSize::new(1);
        assert!(too_small.is_err());

        let valid = ChunkSize::from_megabytes(10);
        assert!(valid.is_ok());
    }
}
```

## Benefits of This Domain Model

1. **Pure Business Logic** - No infrastructure dependencies
2. **Highly Testable** - Can test without databases, files, or networks
3. **Type Safety** - Strong typing prevents many bugs at compile time
4. **Self-Documenting** - Code structure reflects business concepts
5. **Flexible** - Easy to change infrastructure without touching domain
6. **Maintainable** - Business rules are explicit and centralized

## Next Steps

Now that you understand the domain model:

- [Layered Architecture](layers.md) - How the domain fits into the overall architecture
- [Hexagonal Architecture](adapter-pattern.md) - Ports and adapters pattern
- [Repository Pattern](repository-pattern.md) - Data persistence abstraction
