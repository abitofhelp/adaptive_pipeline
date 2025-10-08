# adaptive-pipeline

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![crates.io](https://img.shields.io/crates/v/adaptive-pipeline.svg)](https://crates.io/crates/adaptive-pipeline)
[![Documentation](https://docs.rs/adaptive-pipeline/badge.svg)](https://docs.rs/adaptive-pipeline)

**High-performance adaptive file processing pipeline** with configurable stages, binary format support, and cross-platform compatibility.

## 🎯 Overview

This crate provides the **application and infrastructure layers** for the Adaptive Pipeline system - including use cases, services, adapters, repositories, and a production-ready CLI.

### Key Features

- **⚡ Channel-Based Concurrency** - Reader → CPU Workers → Direct Writer pattern
- **🎯 Adaptive Performance** - Dynamic chunk sizing and worker scaling
- **🔐 Enterprise Security** - AES-256-GCM, ChaCha20-Poly1305, Argon2 KDF
- **📊 Observable** - Prometheus metrics, structured tracing
- **🛡️ Zero-Panic** - No unwrap/expect/panic in production
- **🌐 Cross-Platform** - macOS, Linux, Windows support

## 📦 Installation

### As a Library

```toml
[dependencies]
adaptive-pipeline = "1.0"
```

### As a CLI Tool

```bash
cargo install adaptive-pipeline
```

## 🏗️ Architecture

This crate implements the Application and Infrastructure layers:

```
┌───────────────────────────────────────────┐
│         APPLICATION LAYER                 │
│  ┌─────────────────────────────────────┐ │
│  │  Use Cases                          │ │
│  │  - ProcessFile                      │ │
│  │  - RestoreFile                      │ │
│  │  - CreatePipeline                   │ │
│  │  - ValidateFile                     │ │
│  └─────────────────────────────────────┘ │
│  ┌─────────────────────────────────────┐ │
│  │  Application Services               │ │
│  │  - ConcurrentPipeline (orchestrator)│ │
│  │  - StreamingFileProcessor           │ │
│  └─────────────────────────────────────┘ │
└───────────────────────────────────────────┘
                   │
                   ▼
┌───────────────────────────────────────────┐
│        INFRASTRUCTURE LAYER               │
│  ┌─────────────────────────────────────┐ │
│  │  Adapters                           │ │
│  │  - TokioFileIO                      │ │
│  │  - AsyncCompression                 │ │
│  │  - AsyncEncryption                  │ │
│  └─────────────────────────────────────┘ │
│  ┌─────────────────────────────────────┐ │
│  │  Repositories                       │ │
│  │  - SqlitePipelineRepository         │ │
│  └─────────────────────────────────────┘ │
│  ┌─────────────────────────────────────┐ │
│  │  Runtime Management                 │ │
│  │  - ResourceManager (global tokens)  │ │
│  │  - StageExecutor                    │ │
│  │  - Supervisor (task management)     │ │
│  └─────────────────────────────────────┘ │
└───────────────────────────────────────────┘
```

## 📚 Library Usage

### Processing Files

```rust
use adaptive_pipeline::application::use_cases::ProcessFileUseCase;
use adaptive_pipeline::application::services::ConcurrentPipeline;
use adaptive_pipeline_domain::value_objects::PipelineId;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create pipeline service
    let pipeline_service = ConcurrentPipeline::new(
        file_io_service,
        pipeline_repository,
        stage_registry,
    );

    // Process file through pipeline
    let config = ProcessFileConfig {
        input: Path::new("input.dat").to_path_buf(),
        output: Path::new("output.adapipe").to_path_buf(),
        pipeline: "compress-encrypt".to_string(),
        chunk_size_mb: Some(8),
        workers: None,  // Auto-detect
        channel_depth: Some(4),
    };

    let result = ProcessFileUseCase::execute(config, pipeline_service).await?;

    println!("Processed {} bytes in {:.2}s",
        result.bytes_processed,
        result.duration.as_secs_f64()
    );

    Ok(())
}
```

### Creating Custom Pipelines

```rust
use adaptive_pipeline::application::use_cases::CreatePipelineUseCase;
use adaptive_pipeline_domain::entities::{Pipeline, PipelineStage, StageType};

// Define pipeline stages
let stages = vec![
    PipelineStage::new("compress".to_string(), StageType::Compression, 1),
    PipelineStage::new("encrypt".to_string(), StageType::Encryption, 2),
    PipelineStage::new("checksum".to_string(), StageType::Checksum, 3),
];

// Create and save pipeline
let pipeline = CreatePipelineUseCase::execute(
    "secure-backup".to_string(),
    stages,
    pipeline_repository,
).await?;

println!("Created pipeline: {}", pipeline.id());
```

### Restoring Files

```rust
use adaptive_pipeline::application::use_cases::RestoreFileUseCase;
use std::path::Path;

// Restore from .adapipe format
let result = RestoreFileUseCase::execute(
    Path::new("backup.adapipe"),
    Some(Path::new("/restore/directory")),
    false,  // mkdir
    false,  // overwrite
    pipeline_service,
).await?;

println!("Restored to: {}", result.output_path.display());
```

## 🖥️ CLI Usage

### Process Files

```bash
# Basic processing
adaptive-pipeline process \
  --input data.bin \
  --output data.adapipe \
  --pipeline compress-encrypt

# With custom settings
adaptive-pipeline process \
  -i large.dat \
  -o large.adapipe \
  -p secure \
  --chunk-size-mb 16 \
  --workers 8 \
  --channel-depth 8
```

### Create Pipelines

```bash
# Compression only
adaptive-pipeline create \
  --name fast-compress \
  --stages compression:lz4

# Full security pipeline
adaptive-pipeline create \
  --name secure-backup \
  --stages compression:zstd,encryption:aes256gcm,integrity
```

### Restore Files

```bash
# Restore to original location
adaptive-pipeline restore --input backup.adapipe

# Restore to specific directory
adaptive-pipeline restore \
  --input data.adapipe \
  --output-dir /tmp/restored \
  --mkdir \
  --overwrite
```

### Validate Files

```bash
# Quick format validation
adaptive-pipeline validate-file --file output.adapipe

# Full integrity check
adaptive-pipeline validate-file --file output.adapipe --full
```

### System Benchmarking

```bash
# Quick benchmark
adaptive-pipeline benchmark

# Comprehensive test
adaptive-pipeline benchmark \
  --size-mb 1000 \
  --iterations 5
```

For complete CLI documentation, see the [root README](../README.md#-command-line-reference).

## ⚡ Performance

### Concurrency Model

**Channel-Based Execution:**

```
┌─────────────┐   Channel    ┌──────────────┐   Direct     ┌────────────┐
│   Reader    │─────────────→│ CPU Workers  │─────────────→│   Writer   │
│   Task      │ Backpressure │  (Parallel)  │ Random Access│ (.adapipe) │
└─────────────┘              └──────────────┘              └────────────┘
     ↓                            ↓ ↓ ↓                         ↓
File I/O                    Rayon Threads               Concurrent Seeks
(Streaming)                 (CPU-bound)                 (No Mutex!)
```

**Key Optimizations:**
- Reader streams with backpressure
- Rayon work-stealing for CPU ops
- Direct concurrent writes (no bottleneck)
- Global resource semaphores

### Benchmarks (Mac Pro 2019, Intel Xeon W-3235 @ 3.3GHz, 12-core/24-thread, 48GB RAM, NVMe SSD)

Measured with `adaptive_pipeline benchmark` command (2025-10-07):

| File Size | Best Throughput | Optimal Config | Adaptive Config |
|-----------|----------------|----------------|-----------------|
| 100 MB    | **811 MB/s**   | 16MB chunks, 7 workers | 502 MB/s (16MB, 8 workers) |
| 1 GB      | **822 MB/s**   | 64MB chunks, 5 workers | 660 MB/s (64MB, 10 workers) |

**Performance Insights:**
- Consistent **800+ MB/s** throughput shows excellent scalability
- Lower worker counts (5-7) often outperform higher counts due to reduced context switching
- Larger chunks (16-64MB) maximize sequential I/O performance
- Adaptive configuration provides good baseline; fine-tuning can improve by 20-60%

Run your own benchmarks: `adaptive_pipeline benchmark --file <path>`

## 🔧 Configuration

### Environment Variables

```bash
# Database
export ADAPIPE_SQLITE_PATH="./pipeline.db"

# Logging
export RUST_LOG="adaptive_pipeline=debug,tower_http=warn"

# Performance
export RAYON_NUM_THREADS=8
export TOKIO_WORKER_THREADS=4
```

### Configuration File

```toml
# pipeline.toml
[pipeline]
chunk_size_mb = 8
parallel_workers = 0  # Auto-detect

[compression]
algorithm = "zstd"
level = "balanced"

[encryption]
algorithm = "aes256gcm"
key_derivation = "argon2id"
```

## 📊 Observability

### Prometheus Metrics

```bash
# Start with metrics endpoint
adaptive-pipeline process \
  --input data.bin \
  --output data.adapipe \
  --pipeline test

# Query metrics (default port: 9090)
curl http://localhost:9090/metrics
```

**Key Metrics:**
- `pipeline_throughput_bytes_per_second`
- `pipeline_cpu_queue_depth`
- `pipeline_worker_utilization`
- `pipeline_chunk_processing_duration_ms`

### Structured Logging

```bash
# Enable debug logging
RUST_LOG=adaptive_pipeline=debug adaptive-pipeline process ...

# Log to file
adaptive-pipeline process ... 2>&1 | tee pipeline.log
```

## 🎯 Advanced Features

### Custom Stages

Implement the `StageService` trait:

```rust
use adaptive_pipeline_domain::services::StageService;
use adaptive_pipeline_domain::entities::{FileChunk, ProcessingContext};

pub struct MyCustomStage {
    // Stage configuration
}

impl StageService for MyCustomStage {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // Custom processing logic
        Ok(chunk)
    }

    fn reverse_chunk(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // Reverse transformation
        Ok(chunk)
    }
}
```

### Resource Management

```rust
use adaptive_pipeline::infrastructure::runtime::ResourceManager;

// Global resource manager
let rm = ResourceManager::global();

// Acquire CPU token (respects core count)
let cpu_token = rm.acquire_cpu_token().await?;

// Acquire I/O token (respects device type)
let io_token = rm.acquire_io_token().await?;

// Tokens auto-release on drop
```

### Binary Format

The `.adapipe` binary format includes:
- **Header** - Metadata, algorithms, original file info
- **Chunks** - Processed data with checksums
- **Footer** - Final statistics and verification data

```
┌────────────────────────────────────┐
│  Header (1024 bytes)               │
│  - Magic bytes: ADAPIPE\0          │
│  - Version: 1                      │
│  - Original filename & checksum    │
│  - Pipeline ID and stages          │
│  - Compression/encryption config   │
└────────────────────────────────────┘
┌────────────────────────────────────┐
│  Chunk 0 (variable size)           │
│  - Sequence number                 │
│  - Compressed size                 │
│  - Data                            │
│  - Checksum                        │
└────────────────────────────────────┘
│  ... more chunks ...               │
┌────────────────────────────────────┐
│  Footer (1024 bytes)               │
│  - Total chunks                    │
│  - Output checksum                 │
│  - Processing timestamp            │
└────────────────────────────────────┘
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

## 📊 Dependencies

### Application Layer
- **adaptive-pipeline-domain** - Business logic
- **adaptive-pipeline-bootstrap** - Platform abstraction
- **tokio** - Async runtime
- **rayon** - CPU parallelism

### Infrastructure Layer
- **sqlx** - Database (SQLite)
- **prometheus** - Metrics
- **tracing** - Structured logging
- **brotli / zstd / lz4 / flate2** - Compression
- **aes-gcm / chacha20poly1305** - Encryption
- **argon2 / scrypt** - Key derivation

## 🔗 Related Crates

- **[adaptive-pipeline-domain](../adaptive-pipeline-domain)** - Pure business logic
- **[adaptive-pipeline-bootstrap](../adaptive-pipeline-bootstrap)** - Platform abstraction

## 📄 License

BSD 3-Clause License - see [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

Contributions welcome! Focus areas:
- ✅ New pipeline stages
- ✅ Performance optimizations
- ✅ Additional compression/encryption algorithms
- ✅ Enhanced observability
- ✅ Bug fixes and tests

---

**High Performance | Production Ready | Enterprise Security**
