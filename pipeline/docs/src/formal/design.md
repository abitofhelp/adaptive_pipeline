# Software Design Document (SDD)

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

---

## 1. Introduction

### 1.1 Purpose

This Software Design Document (SDD) describes the architectural and detailed design of the Optimized Adaptive Pipeline system. It provides technical specifications for developers implementing, maintaining, and extending the system.

**Intended Audience:**
- Software developers implementing features
- Technical architects reviewing design decisions
- Code reviewers evaluating pull requests
- New team members onboarding to the codebase

### 1.2 Scope

This document covers the design of a high-performance file processing pipeline implemented in Rust, following Domain-Driven Design (DDD), Clean Architecture, and Hexagonal Architecture principles.

**Covered Topics:**
- System architecture and layer organization
- Component interactions and dependencies
- Data structures and algorithms
- Interface specifications
- Design patterns and rationale
- Technology stack and tooling
- Cross-cutting concerns

**Not Covered:**
- Requirements (see SRS)
- Testing strategies (see Test Plan)
- Deployment procedures
- Operational procedures

### 1.3 Design Philosophy

**Core Principles:**

1. **Domain-Driven Design**: Business logic isolated in domain layer
2. **Clean Architecture**: Dependencies point inward toward domain
3. **Hexagonal Architecture**: Ports and adapters isolate external concerns
4. **SOLID Principles**: Single responsibility, dependency inversion
5. **Rust Idioms**: Zero-cost abstractions, ownership, type safety

**Design Goals:**

- **Correctness**: Type-safe, compiler-verified invariants
- **Performance**: Async I/O, parallel processing, minimal allocations
- **Maintainability**: Clear separation of concerns, testable components
- **Extensibility**: Plugin architecture for custom stages
- **Observability**: Comprehensive metrics and logging

### 1.4 References

- [Software Requirements Specification](../reference/srs.md)
- [Architecture Overview](../architecture/overview.md)
- [Domain Model](../architecture/domain-model.md)
- Clean Architecture: Robert C. Martin, 2017
- Domain-Driven Design: Eric Evans, 2003

---

## 2. System Architecture

### 2.1 Architectural Overview

The system follows a **layered architecture** with strict dependency rules:

```
┌─────────────────────────────────────────────┐
│         Presentation Layer                  │ ← CLI, Future APIs
│  (bootstrap, cli, config, logger)           │
├─────────────────────────────────────────────┤
│         Application Layer                    │ ← Use Cases
│  (commands, services, orchestration)         │
├─────────────────────────────────────────────┤
│         Domain Layer (Core)                  │ ← Business Logic
│  (entities, value objects, services)         │
├─────────────────────────────────────────────┤
│         Infrastructure Layer                 │ ← External Concerns
│  (repositories, adapters, file I/O)          │
└─────────────────────────────────────────────┘

Dependency Rule: Inner layers know nothing about outer layers
```

### 2.2 Layered Architecture Details

#### 2.2.1 Presentation Layer (Bootstrap Crate)

**Responsibilities:**
- CLI argument parsing and validation
- Application configuration
- Logging and console output
- Exit code handling
- Platform-specific concerns

**Key Components:**
- `cli::parser` - Command-line argument parsing
- `cli::validator` - Input validation and sanitization
- `config::AppConfig` - Application configuration
- `logger::Logger` - Logging abstraction
- `platform::Platform` - Platform detection

**Design Decisions:**
- Separate crate for clean separation
- No domain dependencies
- Generic, reusable components

#### 2.2.2 Application Layer

**Responsibilities:**
- Orchestrate use cases
- Coordinate domain services
- Manage transactions
- Handle application-level errors

**Key Components:**
- `use_cases::ProcessFileUseCase` - File processing orchestration
- `use_cases::RestoreFileUseCase` - File restoration orchestration
- `commands::ProcessFileCommand` - Command pattern implementation
- `commands::RestoreFileCommand` - Restoration command

**Design Patterns:**
- **Command Pattern**: Encapsulate requests as objects
- **Use Case Pattern**: Application-specific business rules
- **Dependency Injection**: Services injected via constructors

**Key Abstractions:**
```rust
#[async_trait]
pub trait UseCase<Input, Output> {
    async fn execute(&self, input: Input) -> Result<Output>;
}
```

#### 2.2.3 Domain Layer

**Responsibilities:**
- Core business logic
- Domain rules and invariants
- Domain events
- Pure functions (no I/O)

**Key Components:**

**Entities:**
- `Pipeline` - Aggregate root for pipeline configuration
- `PipelineStage` - Individual processing stage
- `FileMetadata` - File information and state

**Value Objects:**
- `ChunkSize` - Validated chunk size
- `PipelineId` - Unique pipeline identifier
- `StageType` - Type-safe stage classification

**Domain Services:**
- `CompressionService` - Compression/decompression logic
- `EncryptionService` - Encryption/decryption logic
- `ChecksumService` - Integrity verification logic
- `FileIOService` - File operations abstraction

**Invariants Enforced:**
```rust
impl Pipeline {
    pub fn new(name: String, stages: Vec<PipelineStage>) -> Result<Self> {
        // Invariant: Name must be non-empty
        if name.trim().is_empty() {
            return Err(PipelineError::InvalidName);
        }

        // Invariant: At least one stage required
        if stages.is_empty() {
            return Err(PipelineError::NoStages);
        }

        // Invariant: Stages must have unique, sequential order
        validate_stage_order(&stages)?;

        Ok(Pipeline { name, stages })
    }
}
```

#### 2.2.4 Infrastructure Layer

**Responsibilities:**
- External system integration
- Data persistence
- File I/O operations
- Third-party library adapters

**Key Components:**

**Repositories:**
- `SqlitePipelineRepository` - SQLite-based persistence
- `InMemoryPipelineRepository` - Testing/development

**Adapters:**
- `BrotliCompressionAdapter` - Brotli compression
- `ZstdCompressionAdapter` - Zstandard compression
- `AesGcmEncryptionAdapter` - AES-256-GCM encryption
- `Sha256ChecksumAdapter` - SHA-256 hashing

**File I/O:**
- `BinaryFileWriter` - .adapipe format writer
- `BinaryFileReader` - .adapipe format reader
- `ChunkedFileReader` - Streaming file reader
- `ChunkedFileWriter` - Streaming file writer

### 2.3 Hexagonal Architecture (Ports & Adapters)

**Primary Ports** (driving adapters - how the world uses us):
- CLI commands
- Future: REST API, gRPC API

**Secondary Ports** (driven adapters - how we use the world):
```rust
// Domain defines the interface (port)
#[async_trait]
pub trait CompressionPort: Send + Sync {
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
}

// Infrastructure provides implementations (adapters)
pub struct BrotliAdapter {
    quality: u32,
}

#[async_trait]
impl CompressionPort for BrotliAdapter {
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Brotli-specific implementation
    }
}
```

**Benefits:**
- Domain layer testable in isolation
- Easy to swap implementations
- Third-party dependencies isolated
- Technology-agnostic core

---

## 3. Component Design

### 3.1 Pipeline Processing Engine

**Architecture:**

```
┌──────────────┐
│   Pipeline   │ (Aggregate Root)
│              │
│  - name      │
│  - stages[]  │
│  - metadata  │
└──────┬───────┘
       │ has many
       ▼
┌──────────────┐
│PipelineStage │
│              │
│  - name      │
│  - type      │
│  - config    │
│  - order     │
└──────────────┘
```

**Processing Flow:**

```rust
pub struct PipelineProcessor {
    context: Arc<PipelineContext>,
}

impl PipelineProcessor {
    pub async fn process(&self, pipeline: &Pipeline, input: PathBuf)
        -> Result<PathBuf>
    {
        // 1. Validate input file
        let metadata = self.validate_input(&input).await?;

        // 2. Create output file
        let output = self.create_output_file(&pipeline.name).await?;

        // 3. Process in chunks
        let chunks = self.chunk_file(&input, self.context.chunk_size).await?;

        // 4. Execute stages on each chunk (parallel)
        for chunk in chunks {
            let processed = self.execute_stages(pipeline, chunk).await?;
            self.write_chunk(&output, processed).await?;
        }

        // 5. Write metadata and finalize
        self.finalize(&output, metadata).await?;

        Ok(output)
    }

    async fn execute_stages(&self, pipeline: &Pipeline, chunk: Chunk)
        -> Result<Chunk>
    {
        let mut data = chunk.data;

        for stage in &pipeline.stages {
            data = match stage.stage_type {
                StageType::Compression => {
                    self.context.compression.compress(&data).await?
                }
                StageType::Encryption => {
                    self.context.encryption.encrypt(&data).await?
                }
                StageType::Checksum => {
                    let hash = self.context.checksum.hash(&data).await?;
                    self.context.store_hash(chunk.id, hash);
                    data // Checksum doesn't transform data
                }
                StageType::PassThrough => data,
            };
        }

        Ok(Chunk { data, ..chunk })
    }
}
```

### 3.2 Chunk Processing Strategy

**Design Rationale:**
- Large files cannot fit in memory
- Streaming processing enables arbitrary file sizes
- Parallel chunk processing utilizes multiple cores
- Fixed chunk size simplifies memory management

**Implementation:**

```rust
pub const DEFAULT_CHUNK_SIZE: usize = 1_048_576; // 1 MB

pub struct Chunk {
    pub id: usize,
    pub data: Vec<u8>,
    pub is_final: bool,
}

pub struct ChunkedFileReader {
    file: File,
    chunk_size: usize,
    chunk_id: usize,
}

impl ChunkedFileReader {
    pub async fn read_chunk(&mut self) -> Result<Option<Chunk>> {
        let mut buffer = vec![0u8; self.chunk_size];
        let bytes_read = self.file.read(&mut buffer).await?;

        if bytes_read == 0 {
            return Ok(None);
        }

        buffer.truncate(bytes_read);

        let chunk = Chunk {
            id: self.chunk_id,
            data: buffer,
            is_final: bytes_read < self.chunk_size,
        };

        self.chunk_id += 1;
        Ok(Some(chunk))
    }
}
```

**Parallel Processing:**

```rust
use tokio::task::JoinSet;

pub async fn process_chunks_parallel(
    chunks: Vec<Chunk>,
    stage: &dyn ProcessingStage,
) -> Result<Vec<Chunk>> {
    let mut join_set = JoinSet::new();

    for chunk in chunks {
        let stage = Arc::clone(&stage);
        join_set.spawn(async move {
            stage.process(chunk).await
        });
    }

    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        results.push(result??);
    }

    // Sort by chunk ID to maintain order
    results.sort_by_key(|c| c.id);

    Ok(results)
}
```

### 3.3 Repository Pattern Implementation

**Interface (Port):**

```rust
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn save(&self, pipeline: &Pipeline) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Pipeline>>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Pipeline>>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn list_all(&self) -> Result<Vec<Pipeline>>;
}
```

**SQLite Implementation:**

```rust
pub struct SqlitePipelineRepository {
    pool: Arc<SqlitePool>,
}

#[async_trait]
impl PipelineRepository for SqlitePipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Insert pipeline
        sqlx::query!(
            "INSERT INTO pipelines (id, name, created_at) VALUES (?, ?, ?)",
            pipeline.id,
            pipeline.name,
            pipeline.created_at
        )
        .execute(&mut *tx)
        .await?;

        // Insert stages
        for stage in &pipeline.stages {
            sqlx::query!(
                "INSERT INTO pipeline_stages
                 (pipeline_id, name, type, order_num, config)
                 VALUES (?, ?, ?, ?, ?)",
                pipeline.id,
                stage.name,
                stage.stage_type.to_string(),
                stage.order,
                serde_json::to_string(&stage.configuration)?
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
```

### 3.4 Adapter Pattern for Algorithms

**Design:** Each algorithm family has a common interface with multiple implementations.

**Compression Adapters:**

```rust
#[async_trait]
pub trait CompressionAdapter: Send + Sync {
    fn name(&self) -> &str;
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
}

pub struct BrotliAdapter {
    quality: u32,
}

#[async_trait]
impl CompressionAdapter for BrotliAdapter {
    fn name(&self) -> &str { "brotli" }

    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut compressor = brotli::CompressorReader::new(
            data,
            4096,
            self.quality,
            22
        );
        compressor.read_to_end(&mut output)?;
        Ok(output)
    }

    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        let mut decompressor = brotli::Decompressor::new(data, 4096);
        decompressor.read_to_end(&mut output)?;
        Ok(output)
    }
}
```

**Factory Pattern:**

```rust
pub struct AdapterFactory;

impl AdapterFactory {
    pub fn create_compression(
        algorithm: &str
    ) -> Result<Box<dyn CompressionAdapter>> {
        match algorithm.to_lowercase().as_str() {
            "brotli" => Ok(Box::new(BrotliAdapter::new(11))),
            "zstd" => Ok(Box::new(ZstdAdapter::new(3))),
            "gzip" => Ok(Box::new(GzipAdapter::new(6))),
            "lz4" => Ok(Box::new(Lz4Adapter::new())),
            _ => Err(PipelineError::UnsupportedAlgorithm(
                algorithm.to_string()
            )),
        }
    }
}
```

---

## 4. Data Design

### 4.1 Domain Entities

**Pipeline Entity:**

```rust
pub struct Pipeline {
    id: PipelineId,
    name: String,
    stages: Vec<PipelineStage>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Pipeline {
    // Factory method enforces invariants
    pub fn new(name: String, stages: Vec<PipelineStage>) -> Result<Self> {
        // Validation logic
    }

    // Domain behavior
    pub fn add_stage(&mut self, stage: PipelineStage) -> Result<()> {
        // Ensure stage order is valid
        // Ensure no duplicate stage names
    }

    pub fn execute_order(&self) -> &[PipelineStage] {
        &self.stages
    }

    pub fn restore_order(&self) -> Vec<PipelineStage> {
        self.stages.iter().rev().cloned().collect()
    }
}
```

**PipelineStage Entity:**

```rust
pub struct PipelineStage {
    name: String,
    stage_type: StageType,
    configuration: StageConfiguration,
    order: usize,
}

pub enum StageType {
    Compression,
    Encryption,
    Checksum,
    PassThrough,
}

pub struct StageConfiguration {
    pub algorithm: String,
    pub parameters: HashMap<String, String>,
    pub parallel_processing: bool,
    pub chunk_size: Option<usize>,
}
```

### 4.2 Value Objects

**ChunkSize:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkSize(usize);

impl ChunkSize {
    pub const MIN: usize = 1024;  // 1 KB
    pub const MAX: usize = 100 * 1024 * 1024;  // 100 MB
    pub const DEFAULT: usize = 1_048_576;  // 1 MB

    pub fn new(size: usize) -> Result<Self> {
        if size < Self::MIN || size > Self::MAX {
            return Err(PipelineError::InvalidChunkSize {
                size,
                min: Self::MIN,
                max: Self::MAX
            });
        }
        Ok(ChunkSize(size))
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
```

### 4.3 Database Schema

**Schema Management:** Using `sqlx` with migrations

```sql
-- migrations/001_initial_schema.sql

CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_pipelines_name ON pipelines(name);

CREATE TABLE IF NOT EXISTS pipeline_stages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pipeline_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    order_num INTEGER NOT NULL,
    config TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE,
    UNIQUE (pipeline_id, name),
    UNIQUE (pipeline_id, order_num)
);

CREATE INDEX idx_stages_pipeline ON pipeline_stages(pipeline_id);

CREATE TABLE IF NOT EXISTS file_metadata (
    id TEXT PRIMARY KEY,
    original_path TEXT NOT NULL,
    processed_path TEXT NOT NULL,
    pipeline_id TEXT NOT NULL,
    original_size INTEGER NOT NULL,
    processed_size INTEGER NOT NULL,
    checksum TEXT,
    processed_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE INDEX idx_metadata_pipeline ON file_metadata(pipeline_id);
```

### 4.4 Binary File Format (.adapipe)

**Format Specification:**

```
.adapipe File Structure:
┌────────────────────────────────────┐
│ Magic Number (4 bytes): "ADPI"    │
│ Version (2 bytes): Major.Minor     │
│ Header Length (4 bytes)            │
├────────────────────────────────────┤
│ Header (JSON):                     │
│  - pipeline_name                   │
│  - stage_configs[]                 │
│  - original_filename               │
│  - original_size                   │
│  - chunk_count                     │
│  - checksum_algorithm              │
├────────────────────────────────────┤
│ Chunk 0:                           │
│  - Length (4 bytes)                │
│  - Checksum (32 bytes)             │
│  - Data (variable)                 │
├────────────────────────────────────┤
│ Chunk 1: ...                       │
├────────────────────────────────────┤
│ ...                                │
├────────────────────────────────────┤
│ Footer (JSON):                     │
│  - total_checksum                  │
│  - created_at                      │
└────────────────────────────────────┘
```

**Implementation:**

```rust
pub struct AdapipeHeader {
    pub pipeline_name: String,
    pub stages: Vec<StageConfig>,
    pub original_filename: String,
    pub original_size: u64,
    pub chunk_count: usize,
    pub checksum_algorithm: String,
}

pub struct BinaryFileWriter {
    file: File,
}

impl BinaryFileWriter {
    pub async fn write_header(&mut self, header: &AdapipeHeader)
        -> Result<()>
    {
        // Magic number
        self.file.write_all(b"ADPI").await?;

        // Version
        self.file.write_u8(1).await?;  // Major
        self.file.write_u8(0).await?;  // Minor

        // Header JSON
        let header_json = serde_json::to_vec(header)?;
        self.file.write_u32(header_json.len() as u32).await?;
        self.file.write_all(&header_json).await?;

        Ok(())
    }

    pub async fn write_chunk(&mut self, chunk: &ProcessedChunk)
        -> Result<()>
    {
        self.file.write_u32(chunk.data.len() as u32).await?;
        self.file.write_all(&chunk.checksum).await?;
        self.file.write_all(&chunk.data).await?;
        Ok(())
    }
}
```

---

## 5. Interface Design

### 5.1 Public API

**Command-Line Interface:**

```bash
# Process a file
pipeline process \
    --input ./document.pdf \
    --output ./document.adapipe \
    --pipeline secure-archive \
    --compress zstd \
    --encrypt aes256gcm \
    --key-file ./key.txt

# Restore a file
pipeline restore \
    --input ./document.adapipe \
    --output ./document.pdf \
    --key-file ./key.txt

# List pipelines
pipeline list

# Create pipeline configuration
pipeline create \
    --name my-pipeline \
    --stages compress:zstd,encrypt:aes256gcm
```

**Future: Library API:**

```rust
use pipeline::{Pipeline, PipelineStage, StageType};

#[tokio::main]
async fn main() -> Result<()> {
    // Create pipeline
    let pipeline = Pipeline::builder()
        .name("my-pipeline")
        .add_stage(
            PipelineStage::compression("compress", "zstd", 1)?
        )
        .add_stage(
            PipelineStage::encryption("encrypt", "aes256gcm", 2)?
        )
        .build()?;

    // Process file
    let processor = PipelineProcessor::new()?;
    let output = processor.process(&pipeline, "input.txt").await?;

    println!("Processed: {:?}", output);
    Ok(())
}
```

### 5.2 Internal APIs

**Domain Service Interfaces:**

```rust
#[async_trait]
pub trait CompressionService: Send + Sync {
    async fn compress(
        &self,
        algorithm: &str,
        data: &[u8]
    ) -> Result<Vec<u8>>;

    async fn decompress(
        &self,
        algorithm: &str,
        data: &[u8]
    ) -> Result<Vec<u8>>;
}

#[async_trait]
pub trait EncryptionService: Send + Sync {
    async fn encrypt(
        &self,
        algorithm: &str,
        data: &[u8],
        key: &[u8]
    ) -> Result<Vec<u8>>;

    async fn decrypt(
        &self,
        algorithm: &str,
        data: &[u8],
        key: &[u8]
    ) -> Result<Vec<u8>>;
}
```

---

## 6. Technology Stack

### 6.1 Core Technologies

| Technology | Purpose | Version |
|------------|---------|---------|
| Rust | Primary language | 1.75+ |
| Tokio | Async runtime | 1.35+ |
| SQLx | Database access | 0.7+ |
| Serde | Serialization | 1.0+ |
| Anyhow | Error handling | 1.0+ |
| Clap | CLI parsing | 4.5+ |

### 6.2 Algorithms & Libraries

**Compression:**
- Brotli (`brotli` crate)
- Zstandard (`zstd` crate)
- Gzip (`flate2` crate)
- LZ4 (`lz4` crate)

**Encryption:**
- AES-GCM (`aes-gcm` crate)
- ChaCha20-Poly1305 (`chacha20poly1305` crate)

**Hashing:**
- SHA-256/SHA-512 (`sha2` crate)
- BLAKE3 (`blake3` crate)

**Testing:**
- Criterion (benchmarking)
- Proptest (property testing)
- Mockall (mocking)

### 6.3 Development Tools

- **Cargo**: Build system and package manager
- **Clippy**: Linter
- **Rustfmt**: Code formatter
- **Cargo-audit**: Security auditing
- **Cargo-deny**: Dependency validation
- **mdBook**: Documentation generation

---

## 7. Design Patterns

### 7.1 Repository Pattern

**Purpose:** Abstract data persistence logic from domain logic.

**Implementation:** See Section 3.3

**Benefits:**
- Domain layer independent of storage mechanism
- Easy to swap SQLite for PostgreSQL or other storage
- Testable with in-memory implementation

### 7.2 Adapter Pattern

**Purpose:** Integrate third-party algorithms without coupling.

**Implementation:** See Section 3.4

**Benefits:**
- Easy to add new algorithms
- Algorithm selection at runtime
- Consistent interface across all adapters

### 7.3 Strategy Pattern

**Purpose:** Select algorithm implementation dynamically.

**Example:**

```rust
pub struct PipelineProcessor {
    compression_strategy: Box<dyn CompressionAdapter>,
    encryption_strategy: Box<dyn EncryptionAdapter>,
}

impl PipelineProcessor {
    pub fn new(
        compression: &str,
        encryption: &str
    ) -> Result<Self> {
        Ok(Self {
            compression_strategy: AdapterFactory::create_compression(
                compression
            )?,
            encryption_strategy: AdapterFactory::create_encryption(
                encryption
            )?,
        })
    }
}
```

### 7.4 Builder Pattern

**Purpose:** Construct complex objects step by step.

**Example:**

```rust
let pipeline = Pipeline::builder()
    .name("my-pipeline")
    .add_stage(compression_stage)
    .add_stage(encryption_stage)
    .chunk_size(2_097_152)
    .build()?;
```

### 7.5 Command Pattern

**Purpose:** Encapsulate requests as objects for undo/redo, queueing.

**Example:**

```rust
#[async_trait]
pub trait Command: Send + Sync {
    async fn execute(&self) -> Result<()>;
}

pub struct ProcessFileCommand {
    pipeline: Pipeline,
    input: PathBuf,
    output: PathBuf,
}

#[async_trait]
impl Command for ProcessFileCommand {
    async fn execute(&self) -> Result<()> {
        // Processing logic
    }
}
```

---

## 8. Cross-Cutting Concerns

### 8.1 Error Handling

**Strategy:** Use `anyhow::Result` for application errors, custom error types for domain errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Invalid pipeline name: {0}")]
    InvalidName(String),

    #[error("No stages defined")]
    NoStages,

    #[error("Invalid chunk size: {size} (must be {min}-{max})")]
    InvalidChunkSize { size: usize, min: usize, max: usize },

    #[error("Stage order conflict at position {0}")]
    StageOrderConflict(usize),

    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

### 8.2 Logging

**Strategy:** Structured logging with tracing.

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(self, data))]
async fn process_chunk(&self, chunk_id: usize, data: &[u8])
    -> Result<Vec<u8>>
{
    debug!(chunk_id, size = data.len(), "Processing chunk");

    let start = Instant::now();
    let result = self.compress(data).await?;

    info!(
        chunk_id,
        original_size = data.len(),
        compressed_size = result.len(),
        duration_ms = start.elapsed().as_millis(),
        "Chunk compressed"
    );

    Ok(result)
}
```

### 8.3 Metrics Collection

**Strategy:** Prometheus-style metrics.

```rust
pub struct PipelineMetrics {
    pub chunks_processed: AtomicUsize,
    pub bytes_processed: AtomicU64,
    pub compression_ratio: AtomicU64,
    pub processing_duration: Duration,
}

impl PipelineMetrics {
    pub fn record_chunk(&self, original_size: usize, compressed_size: usize) {
        self.chunks_processed.fetch_add(1, Ordering::Relaxed);
        self.bytes_processed.fetch_add(
            original_size as u64,
            Ordering::Relaxed
        );

        let ratio = (compressed_size as f64 / original_size as f64 * 100.0)
            as u64;
        self.compression_ratio.store(ratio, Ordering::Relaxed);
    }
}
```

### 8.4 Security

**Key Management:**
- Keys never logged or persisted unencrypted
- Argon2 key derivation from passwords
- Secure memory wiping for key material

**Input Validation:**
- CLI inputs sanitized against injection
- File paths validated
- Chunk sizes bounded

**Algorithm Selection:**
- Only vetted, well-known algorithms
- Default to secure settings
- AEAD ciphers for authenticated encryption

---

## 9. Performance Considerations

### 9.1 Asynchronous I/O

**Rationale:** File I/O is the primary bottleneck. Async I/O allows overlap of CPU and I/O operations.

**Implementation:**
- Tokio async runtime
- `tokio::fs` for file operations
- `tokio::io::AsyncRead` and `AsyncWrite` traits

### 9.2 Parallel Processing

**Chunk-level Parallelism:**

```rust
use rayon::prelude::*;

pub fn process_chunks_parallel(
    chunks: Vec<Chunk>,
    processor: &ChunkProcessor,
) -> Result<Vec<ProcessedChunk>> {
    chunks
        .par_iter()
        .map(|chunk| processor.process(chunk))
        .collect()
}
```

### 9.3 Memory Management

**Strategies:**
- Fixed chunk size limits peak memory
- Streaming processing avoids loading entire files
- Buffer pooling for frequently allocated buffers
- Zero-copy where possible

### 9.4 Algorithm Selection

**Performance Profiles:**

| Algorithm | Speed | Ratio | Memory |
|-----------|-------|-------|--------|
| LZ4 | ★★★★★ | ★★☆☆☆ | ★★★★★ |
| Zstd | ★★★★☆ | ★★★★☆ | ★★★★☆ |
| Gzip | ★★★☆☆ | ★★★☆☆ | ★★★★☆ |
| Brotli | ★★☆☆☆ | ★★★★★ | ★★★☆☆ |

---

## 10. Testing Strategy (Overview)

### 10.1 Test Organization

- **Unit Tests**: `#[cfg(test)]` modules in source files
- **Integration Tests**: `tests/integration/`
- **E2E Tests**: `tests/e2e/`
- **Benchmarks**: `benches/`

### 10.2 Test Coverage Goals

- Domain layer: 90%+ coverage
- Application layer: 80%+ coverage
- Infrastructure: 70%+ coverage (mocked external deps)

### 10.3 Testing Tools

- Mockall for mocking
- Proptest for property-based testing
- Criterion for benchmarking
- Cargo-tarpaulin for coverage

---

## 11. Future Enhancements

### 11.1 Planned Features

1. **Distributed Processing**: Process files across multiple machines
2. **Cloud Integration**: S3, Azure Blob, GCS support
3. **REST API**: HTTP API for remote processing
4. **Plugin System**: Dynamic loading of custom stages
5. **Web UI**: Browser-based configuration and monitoring

### 11.2 Architectural Evolution

**Phase 1 (Current):** Single-machine CLI application

**Phase 2:** Library + CLI + REST API

**Phase 3:** Distributed processing with coordinator nodes

**Phase 4:** Cloud-native deployment with Kubernetes

---

## 12. Conclusion

This Software Design Document describes a robust, extensible file processing pipeline built on solid architectural principles. The design prioritizes:

- **Correctness** through type safety and domain-driven design
- **Performance** through async I/O and parallel processing
- **Maintainability** through clean architecture and separation of concerns
- **Extensibility** through ports & adapters and plugin architecture

The implementation follows Rust best practices and leverages the language's strengths in safety, concurrency, and zero-cost abstractions.

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| Aggregate | Domain entity that serves as consistency boundary |
| AEAD | Authenticated Encryption with Associated Data |
| Chunk | Fixed-size portion of file for streaming processing |
| DDD | Domain-Driven Design architectural approach |
| Port | Interface defined by application core |
| Adapter | Implementation of port for external system |
| Use Case | Application-specific business rule |
| Value Object | Immutable object defined by its attributes |

---

## Appendix B: Diagrams

See the following chapters for detailed diagrams:
- [Layered Architecture](../architecture/layers.md)
- [Domain Model](../architecture/domain-model.md)
- [Pipeline Flow](../diagrams/pipeline-flow.svg)
- [Hexagonal Architecture](../architecture/overview.md)
