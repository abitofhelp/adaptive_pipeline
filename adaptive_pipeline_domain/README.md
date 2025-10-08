<!--
Adaptive Pipeline
Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
SPDX-License-Identifier: BSD-3-Clause
See LICENSE file in the project root.
-->

# adaptive-pipeline-domain

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![crates.io](https://img.shields.io/crates/v/adaptive-pipeline-domain.svg)](https://crates.io/crates/adaptive-pipeline-domain)
[![Documentation](https://docs.rs/adaptive-pipeline-domain/badge.svg)](https://docs.rs/adaptive-pipeline-domain)

**Pure business logic and domain model for the Adaptive Pipeline** - A reusable, framework-agnostic library following Domain-Driven Design principles.

## 🎯 Overview

This crate contains the **pure domain layer** of the Adaptive Pipeline system - all business logic, entities, value objects, and domain services with **zero infrastructure dependencies**. It's completely synchronous, has no I/O, and can be reused in any Rust application.

### Design Philosophy

- **✨ Pure Rust** - No async, no tokio, no I/O dependencies
- **🎨 Domain-Driven Design** - Entities, value objects, aggregates, domain services
- **🔒 Type Safety** - Leverages Rust's type system for compile-time guarantees
- **♻️ Reusable** - Can be used independently in any Rust project
- **🧪 Testable** - Pure functions and immutable values make testing trivial

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
adaptive-pipeline-domain = "1.0"
```

## 🏗️ Architecture

This crate implements the **Domain Layer** from Clean Architecture:

```
┌─────────────────────────────────────────────┐
│           Domain Layer (This Crate)         │
│  ┌────────────────────────────────────────┐ │
│  │  Entities                              │ │
│  │  - Pipeline                            │ │
│  │  - PipelineStage                       │ │
│  │  - ProcessingContext                   │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Value Objects                         │ │
│  │  - PipelineId (ULID)                   │ │
│  │  - FileChunk                           │ │
│  │  - ChunkSize                           │ │
│  │  - Algorithm                           │ │
│  │  - FilePath (validated)                │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Domain Services (Sync)                │ │
│  │  - CompressionService                  │ │
│  │  - EncryptionService                   │ │
│  │  - ChecksumService                     │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │  Infrastructure Ports (Traits)         │ │
│  │  - FileIOService                       │ │
│  │  - PipelineRepository                  │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

### Key Components

#### Entities (with identity)

- **Pipeline** - Main aggregate root with pipeline configuration
- **PipelineStage** - Individual processing stage in a pipeline
- **ProcessingContext** - Maintains state during file processing

#### Value Objects (immutable, no identity)

- **PipelineId / StageId / SessionId** - ULID-based identifiers
- **FileChunk** - Immutable data chunk with metadata
- **ChunkSize** - Validated chunk size with adaptive sizing
- **FilePath** - Type-safe file path with validation
- **Algorithm** - Compression/encryption algorithm configuration

#### Domain Services (CPU-bound, synchronous)

- **CompressionService** - Brotli, Zstd, LZ4 compression
- **EncryptionService** - AES-GCM, ChaCha20-Poly1305 encryption
- **ChecksumService** - SHA-256 integrity verification

#### Infrastructure Ports (I/O-bound, async traits)

- **FileIOService** - Async trait for file operations
- **PipelineRepository** - Async trait for pipeline persistence
- **FileProcessorService** - Async trait for file processing

## 📚 Usage Examples

### Creating a Pipeline Entity

```rust
use adaptive_pipeline_domain::entities::{Pipeline, PipelineStage, StageType};
use adaptive_pipeline_domain::value_objects::PipelineId;

// Create pipeline with stages
let mut pipeline = Pipeline::new("compress-encrypt".to_string());

// Add compression stage
let compress_stage = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,
    1,  // order
);
pipeline.add_stage(compress_stage);

// Add encryption stage
let encrypt_stage = PipelineStage::new(
    "encryption".to_string(),
    StageType::Encryption,
    2,  // order
);
pipeline.add_stage(encrypt_stage);
```

### Working with Value Objects

```rust
use adaptive_pipeline_domain::value_objects::{ChunkSize, FilePath, FileChunk};

// Type-safe chunk size with validation
let chunk_size = ChunkSize::from_mb(8)?;  // 8 MB chunks

// Validated file path
let input_path = FilePath::new("/data/input.txt")?;

// Immutable file chunk
let chunk = FileChunk::new(
    chunk_data,
    0,  // sequence number
    Some("sha256:abc123...".to_string()),
);
```

### Using Domain Services

```rust
use adaptive_pipeline_domain::services::{
    CompressionService, EncryptionService, ChecksumService
};
use adaptive_pipeline_domain::FileChunk;

// Compression service (sync)
let compression = CompressionService::new("brotli", 6)?;
let compressed_chunk = compression.compress(&chunk)?;

// Encryption service (sync)
let encryption = EncryptionService::new("aes256gcm")?;
let encrypted_chunk = encryption.encrypt(&compressed_chunk, &key)?;

// Checksum service (sync)
let checksum = ChecksumService::new("sha256")?;
let hash = checksum.calculate(&encrypted_chunk)?;
```

### Processing Context

```rust
use adaptive_pipeline_domain::entities::ProcessingContext;

let mut context = ProcessingContext::new();

// Track processing state
context.add_metadata("compression_ratio", "0.65");
context.add_metadata("encryption_algorithm", "aes256gcm");

// Access during processing
let ratio = context.get_metadata("compression_ratio");
```

## 🔧 Design Patterns

### Repository Pattern (Ports)

The domain defines repository traits that infrastructure implements:

```rust
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;
    async fn find_by_id(&self, id: &PipelineId) -> Result<Option<Pipeline>, PipelineError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Pipeline>, PipelineError>;
    async fn delete(&self, id: &PipelineId) -> Result<(), PipelineError>;
}
```

### Service Pattern (Domain Logic)

Domain services encapsulate CPU-bound business logic:

```rust
pub struct CompressionService {
    algorithm: Algorithm,
    level: u8,
}

impl CompressionService {
    pub fn compress(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        // Pure CPU-bound compression logic
        // No I/O, no async
    }
}
```

## 🎯 Key Features

### Type-Safe Identifiers

All identifiers use ULIDs for:
- **Time-ordered** - Sortable by creation time
- **Globally unique** - 128-bit collision-resistant
- **URL-safe** - Base32 encoded strings

```rust
use adaptive_pipeline_domain::value_objects::PipelineId;

let id = PipelineId::new();
println!("Pipeline ID: {}", id);  // 01H2X3Y4Z5W6V7U8T9S0R1Q2P3
```

### Validated Value Objects

All value objects enforce domain invariants:

```rust
use adaptive_pipeline_domain::value_objects::ChunkSize;

// Valid chunk sizes: 1MB to 64MB
let chunk = ChunkSize::from_mb(8)?;  // ✅ Valid

// Invalid sizes are rejected at construction
let invalid = ChunkSize::from_mb(128)?;  // ❌ Error: exceeds maximum
```

### Immutable Domain Events

Domain events capture important business occurrences:

```rust
use adaptive_pipeline_domain::events::{DomainEvent, EventType};

let event = DomainEvent::new(
    EventType::PipelineCreated,
    "Pipeline 'secure-backup' created".to_string(),
);

// Events are immutable and serializable
let json = serde_json::to_string(&event)?;
```

## 🧪 Testing

Domain logic is easy to test because it's pure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_reduces_size() {
        let service = CompressionService::new("brotli", 6).unwrap();
        let input = create_test_chunk(1024 * 1024);  // 1 MB

        let compressed = service.compress(&input).unwrap();

        assert!(compressed.data().len() < input.data().len());
    }
}
```

## 📊 Dependencies

This crate has **minimal dependencies** to keep it pure:

- **serde** - Serialization support
- **uuid / ulid** - Unique identifiers
- **thiserror** - Error handling
- **chrono** - Timestamps (domain concern)
- **sha2** - Checksums (domain logic)
- **regex** - Validation (domain rules)

**No async runtime, no I/O libraries, no database dependencies.**

## 🔗 Related Crates

- **[adaptive-pipeline](../adaptive-pipeline)** - Application layer and CLI
- **[adaptive-pipeline-bootstrap](../adaptive-pipeline-bootstrap)** - Platform abstraction and entry point

## 📄 License

BSD 3-Clause License - see [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

This is a pure domain layer - contributions should:
- ✅ Add business logic or domain concepts
- ✅ Enhance type safety and validation
- ✅ Remain synchronous and I/O-free
- ❌ Not add async/await
- ❌ Not add I/O dependencies
- ❌ Not add framework-specific code

---

**Pure Domain Logic | Framework-Agnostic | Highly Reusable**
