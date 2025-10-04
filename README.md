# Optimized Adaptive Pipeline RS

A **production-grade**, **high-performance** file processing system built with Rust, featuring advanced concurrency patterns, adaptive performance optimization, and enterprise-level reliability. This project demonstrates professional Rust development with Channel-based Architecture, Domain-Driven Design, and comprehensive error handling.

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 🚀 What Makes This Different

This isn't just another file processor - it's a **showcase of advanced Rust patterns** and **production engineering**:

- **🔄 Channel-Based Concurrency**: Reader → CPU Workers → Direct Writer pattern eliminates bottlenecks
- **⚡ Hybrid Parallelism**: Rayon for CPU-bound ops + Tokio for async I/O = optimal resource utilization
- **🎯 Adaptive Performance**: Dynamic chunk sizing and worker scaling based on file characteristics
- **🛡️ Zero-Panic Production Code**: No unwrap/expect/panic patterns
- **🔐 Security First**: AES-256-GCM, ChaCha20-Poly1305 with Argon2 key derivation
- **📊 Observable**: Prometheus metrics, structured tracing, performance dashboards

## 📋 Table of Contents

- [Architecture](#architecture)
- [Performance](#performance)
- [Quick Start](#quick-start)
- [Command Line Reference](#-command-line-reference)
- [Features](#features)
- [Development](#development)
- [Advanced Usage](#advanced-usage)
- [Contributing](#contributing)

## 🏗️ Architecture

### Workspace Structure

The project uses a **3-crate workspace** for clean separation of concerns:

```
optimized_adaptive_pipeline_rs/
├── pipeline-domain/          # Pure domain logic (no async, no I/O)
│   ├── entities/             # Business entities with identity
│   ├── value_objects/        # Immutable domain values
│   ├── services/             # Core business logic (sync)
│   └── Cargo.toml           # Zero infrastructure deps
│
├── pipeline/                 # Application & Infrastructure
│   ├── application/          # Use cases, orchestration
│   ├── infrastructure/       # I/O, persistence, adapters
│   ├── presentation/         # CLI interface
│   └── Cargo.toml           # Full feature set
│
├── bootstrap/                # Entry point & platform layer
│   ├── config.rs            # DI container, service registry
│   ├── signals.rs           # SIGTERM/SIGINT/SIGHUP handling
│   └── platform/            # Cross-platform abstractions
│
└── Cargo.toml               # Workspace config
```

### Concurrency Model

**Channel-Based Execution Pipeline**:

```
┌─────────────┐    Channel     ┌──────────────┐    Direct Write    ┌────────────┐
│   Reader    │───────────────→│ CPU Workers  │───────────────────→│   Writer   │
│   Task      │  Backpressure  │  (Parallel)  │  Random Access     │  (.adapipe)│
└─────────────┘                └──────────────┘                    └────────────┘
      ↓                              ↓ ↓ ↓                              ↓
  File I/O                      Rayon Threads                    Concurrent Seeks
  (Streaming)                   (CPU-bound)                      (No Mutex!)
```

**Key Design Decisions:**

1. **Reader Task**: Streams file chunks with backpressure - prevents memory overload
2. **CPU Workers**: Rayon thread pool for parallel stage execution (compress/encrypt)
3. **Direct Writes**: Workers write directly to calculated positions - **no writer bottleneck**
4. **Resource Manager**: Global semaphores prevent CPU/IO oversubscription

### Layer Responsibilities

```
┌─────────────────────────────────────────────┐
│         Bootstrap Layer                     │
│  (DI, Platform Detection, Signal Handling)  │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│         Presentation Layer                  │
│  (CLI, API endpoints, DTOs)                 │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│         Application Layer                   │
│  (Use cases, orchestration, async services) │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│           Domain Layer                      │
│  (Pure business logic - SYNC only)          │
└─────────────────────────────────────────────┘
                    ↑
┌─────────────────────────────────────────────┐
│        Infrastructure Layer                 │
│  (Database, File I/O, External Systems)     │
└─────────────────────────────────────────────┘
```

**Architecture Principles:**

- **Domain Layer**: Pure Rust, no async, no I/O - just business logic
- **Infrastructure Layer**: All I/O, all async, all external dependencies
- **Dependency Inversion**: Domain defines interfaces, infrastructure implements
- **Hexagonal Ports**: `FileIOService`, `CompressionService` are domain ports

## ⚡ Performance

### Benchmarks (M1 Pro, 10-core CPU)

| File Size | Throughput | Worker Count | Chunk Size | Memory |
|-----------|-----------|--------------|------------|--------|
| 100 MB    | 520 MB/s  | 8 workers    | 4 MB       | 128 MB |
| 1 GB      | 580 MB/s  | 10 workers   | 8 MB       | 256 MB |
| 10 GB     | 610 MB/s  | 10 workers   | 16 MB      | 512 MB |

### Optimizations Implemented

✅ **Memory Efficiency**
- Streaming I/O (no full-file read)
- Memory-mapped files for large data
- Zero-copy operations where possible
- Adaptive chunk sizing (1MB-64MB)

✅ **CPU Optimization**
- Rayon work-stealing for CPU-bound ops
- SIMD acceleration (where available)
- Lock-free metrics collection
- Parallel chunk processing

✅ **I/O Optimization**
- Async I/O with Tokio
- Direct concurrent writes (no mutex!)
- Read-ahead buffering
- Write coalescing

✅ **Concurrency Patterns**
- Channel backpressure prevents overload
- Resource tokens prevent oversubscription
- Graceful cancellation (CancellationToken)
- Supervision tree for task management

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.70 or later (install via [rustup](https://rustup.rs/))
- **SQLite**: For pipeline persistence
- **OS**: macOS, Linux, or Windows (WSL recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/abitofhelp/optimized_adaptive_pipeline_rs.git
cd optimized_adaptive_pipeline_rs

# Build optimized binary
make build-release

# Run tests to verify
make test

# Binary location
./target/release/pipeline --help
```

### First Run

```bash
# Create a test file
dd if=/dev/urandom of=test.dat bs=1M count=100

# Process with compression + encryption
./target/release/pipeline process \
  --input test.dat \
  --output test.adapipe \
  --compress \
  --encrypt

# Check output
ls -lh test.adapipe

# Restore original
./target/release/pipeline restore \
  --input test.adapipe \
  --output restored.dat

# Verify integrity
diff test.dat restored.dat
```

## 📟 Command Line Reference

### Global Options

```bash
pipeline [OPTIONS] <COMMAND>

Options:
  -v, --verbose              Enable verbose logging
  -c, --config <PATH>        Configuration file path
      --cpu-threads <N>      Override CPU worker thread count (default: num_cpus - 1)
      --io-threads <N>       Override I/O worker thread count (default: auto-detect)
      --storage-type <TYPE>  Storage device type: nvme, ssd, hdd (default: auto)
      --channel-depth <N>    Channel depth for pipeline stages (default: 4)
  -h, --help                 Print help
  -V, --version              Print version
```

### Commands

#### `process` - Process File Through Pipeline

Process a file through a configured pipeline with compression, encryption, or validation.

```bash
pipeline process --input <FILE> --output <FILE> --pipeline <NAME> [OPTIONS]

Options:
  -i, --input <FILE>         Input file path
  -o, --output <FILE>        Output file path (.adapipe)
  -p, --pipeline <NAME>      Pipeline name (e.g., "compress-encrypt")
      --chunk-size-mb <MB>   Chunk size in MB (default: adaptive)
      --workers <N>          Number of parallel workers (default: adaptive)

Examples:
  # Process with default pipeline
  pipeline process -i large.dat -o large.adapipe -p compress-encrypt

  # Process with custom settings
  pipeline process -i data.bin -o data.adapipe -p secure \
    --chunk-size-mb 16 --workers 8

  # Process on NVMe with optimized I/O
  pipeline process -i huge.dat -o huge.adapipe -p fast \
    --storage-type nvme --io-threads 24
```

#### `create` - Create New Pipeline

Create a new processing pipeline with custom stages.

```bash
pipeline create --name <NAME> --stages <STAGES> [OPTIONS]

Options:
  -n, --name <NAME>          Pipeline name (kebab-case)
  -s, --stages <STAGES>      Comma-separated stages: compression,encryption,integrity
  -o, --output <FILE>        Save pipeline definition to file (optional)

Supported Stages:
  compression                Brotli compression (default)
  compression:zstd           Zstandard compression
  compression:lz4            LZ4 compression
  encryption                 AES-256-GCM encryption (default)
  encryption:chacha20        ChaCha20-Poly1305 encryption
  integrity                  SHA-256 checksum
  passthrough                No transformation (testing)

Examples:
  # Create compression-only pipeline
  pipeline create -n compress-only -s compression

  # Create secure pipeline with encryption
  pipeline create -n secure-backup -s compression:zstd,encryption,integrity

  # Create fast pipeline with LZ4
  pipeline create -n fast-compress -s compression:lz4
```

#### `list` - List Available Pipelines

List all configured pipelines in the database.

```bash
pipeline list

Example Output:
  Found 3 pipeline(s):

  Pipeline: compress-encrypt
    ID: 550e8400-e29b-41d4-a716-446655440000
    Status: active
    Stages: 2
    Created: 2025-01-15 10:30:45 UTC
    Updated: 2025-01-15 10:30:45 UTC
```

#### `show` - Show Pipeline Details

Display detailed information about a specific pipeline.

```bash
pipeline show <PIPELINE_NAME>

Arguments:
  <PIPELINE_NAME>  Name of the pipeline to show

Example:
  pipeline show compress-encrypt

Example Output:
  === Pipeline Details ===
  ID: 550e8400-e29b-41d4-a716-446655440000
  Name: compress-encrypt
  Status: active
  Created: 2025-01-15 10:30:45 UTC

  Stages (2):
    1. compression (Compression)
       Algorithm: brotli
       Enabled: true
       Order: 0

    2. encryption (Encryption)
       Algorithm: aes256gcm
       Enabled: true
       Order: 1
```

#### `delete` - Delete Pipeline

Delete a pipeline from the database.

```bash
pipeline delete <PIPELINE_NAME> [OPTIONS]

Arguments:
  <PIPELINE_NAME>  Name of the pipeline to delete

Options:
      --force      Skip confirmation prompt

Examples:
  # Delete with confirmation
  pipeline delete old-pipeline

  # Force delete without confirmation
  pipeline delete old-pipeline --force
```

#### `restore` - Restore Original File

Restore an original file from a processed `.adapipe` file.

```bash
pipeline restore --input <FILE> [OPTIONS]

Options:
  -i, --input <FILE>         .adapipe file to restore from
  -o, --output-dir <DIR>     Output directory (default: use original path)
      --mkdir                Create directories without prompting
      --overwrite            Overwrite existing files without prompting

Examples:
  # Restore to original location
  pipeline restore -i backup.adapipe

  # Restore to specific directory
  pipeline restore -i data.adapipe -o /tmp/restored/ --mkdir

  # Force overwrite existing file
  pipeline restore -i file.adapipe --overwrite
```

#### `validate` - Validate Configuration

Validate a pipeline configuration file (TOML/JSON/YAML).

```bash
pipeline validate <CONFIG_FILE>

Arguments:
  <CONFIG_FILE>  Path to configuration file

Supported Formats:
  - TOML (.toml)
  - JSON (.json)
  - YAML (.yaml, .yml)

Example:
  pipeline validate pipeline-config.toml
```

#### `validate-file` - Validate .adapipe File

Validate the integrity of a processed `.adapipe` file.

```bash
pipeline validate-file --file <FILE> [OPTIONS]

Options:
  -f, --file <FILE>  .adapipe file to validate
      --full         Perform full streaming validation (decrypt/decompress/verify)

Examples:
  # Quick format validation
  pipeline validate-file -f output.adapipe

  # Full integrity check (slower but thorough)
  pipeline validate-file -f output.adapipe --full
```

#### `compare` - Compare Files

Compare an original file against its `.adapipe` processed version.

```bash
pipeline compare --original <FILE> --adapipe <FILE> [OPTIONS]

Options:
  -o, --original <FILE>  Original file to compare
  -a, --adapipe <FILE>   .adapipe file to compare against
      --detailed         Show detailed differences

Example:
  pipeline compare -o original.dat -a processed.adapipe --detailed

Example Output:
  📊 File Comparison:
     Original file: original.dat
     .adapipe file: processed.adapipe

  📏 Size Comparison:
     Current file size: 104857600 bytes
     Expected size: 104857600 bytes
     ✅ Size matches

  🔐 Checksum Comparison:
     Expected: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
     Current:  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
     ✅ Checksums match - files are identical
```

#### `benchmark` - System Benchmarking

Run performance benchmarks to optimize configuration.

```bash
pipeline benchmark [OPTIONS]

Options:
  -f, --file <FILE>          Use existing file for benchmark
      --size-mb <MB>         Test data size in MB (default: 100)
      --iterations <N>       Number of iterations (default: 3)

Examples:
  # Quick benchmark with defaults
  pipeline benchmark

  # Comprehensive benchmark
  pipeline benchmark --size-mb 1000 --iterations 5

  # Benchmark with existing file
  pipeline benchmark -f /path/to/large/file.dat

Output:
  - Generates optimization report: pipeline_optimization_report.md
  - Tests multiple chunk sizes and worker counts
  - Recommends optimal configuration for your system
```

### Exit Codes

The CLI uses standard Unix exit codes (sysexits.h):

| Code | Name         | Description                          |
|------|--------------|--------------------------------------|
| 0    | SUCCESS      | Command completed successfully       |
| 1    | ERROR        | General error                        |
| 65   | EX_DATAERR   | Invalid data or configuration        |
| 66   | EX_NOINPUT   | Input file not found                 |
| 70   | EX_SOFTWARE  | Internal software error              |
| 74   | EX_IOERR     | I/O error (read/write failure)       |

**Usage in scripts:**
```bash
#!/bin/bash
pipeline process -i input.dat -o output.adapipe -p compress-encrypt
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo "Success!"
elif [ $EXIT_CODE -eq 66 ]; then
    echo "Input file not found"
elif [ $EXIT_CODE -eq 74 ]; then
    echo "I/O error - check disk space and permissions"
else
    echo "Error occurred (code: $EXIT_CODE)"
fi
```

### Environment Variables

```bash
# Database location
export ADAPIPE_SQLITE_PATH="./pipeline.db"

# Logging configuration
export RUST_LOG="pipeline=debug,tower_http=warn"

# Performance tuning
export RAYON_NUM_THREADS=8        # CPU thread pool size
export TOKIO_WORKER_THREADS=4     # Async I/O thread pool size
```

### Shell Completion

Generate shell completion scripts for your shell:

```bash
# Bash
pipeline --generate-completion bash > /etc/bash_completion.d/pipeline

# Zsh
pipeline --generate-completion zsh > /usr/local/share/zsh/site-functions/_pipeline

# Fish
pipeline --generate-completion fish > ~/.config/fish/completions/pipeline.fish

# PowerShell
pipeline --generate-completion powershell > pipeline.ps1
```

## ✨ Features

### Core Capabilities

🔄 **Multi-Stage Processing Pipeline**
- Configurable stages: compression, encryption, validation
- Dynamic stage ordering and parallelization
- Plugin architecture for custom stages

🔐 **Enterprise Security**
- **Encryption**: AES-256-GCM (hardware accelerated), ChaCha20-Poly1305
- **Key Derivation**: Argon2id, scrypt, PBKDF2
- **Memory Safety**: Automatic key zeroing, secure memory handling
- **Integrity**: SHA-256 checksums, chunk-level verification

⚡ **Adaptive Performance**
- Auto-detects optimal chunk size (1MB-64MB)
- Dynamic worker count (1-32 based on cores)
- File size-based optimization profiles
- Resource-aware backpressure

📊 **Observability**
- Prometheus metrics endpoint
- Structured logging (tracing)
- Performance dashboards (Grafana)
- Queue depth monitoring

🛡️ **Production Reliability**
- Zero panic in production code
- Comprehensive error handling
- Graceful shutdown (SIGTERM/SIGINT/SIGHUP)
- Supervision tree for task recovery

### Supported Algorithms

**Compression:**
- Brotli (best ratio, good speed)
- Zstd (balanced)
- Gzip (widely compatible)
- LZ4 (fastest)

**Encryption:**
- AES-256-GCM (default, hardware accelerated)
- ChaCha20-Poly1305 (constant-time)
- AES-128/192-GCM variants

**Key Derivation:**
- Argon2id (memory-hard, GPU-resistant)
- scrypt (alternative memory-hard)
- PBKDF2 (legacy compatibility)

## 💻 Development

### Makefile Targets

```bash
# Development
make check              # Fast syntax check
make build             # Debug build
make build-release     # Optimized build

# Quality
make lint              # Development linting
make lint-strict       # Production linting (no unwrap/panic)
make format            # Auto-format code
make test              # Run all tests

# CI/CD
make ci-local          # Full CI pipeline locally
make pre-commit        # Pre-commit checks

# Performance
make bench             # Run benchmarks
make flamegraph        # Generate flamegraph

# Documentation
make doc-open          # Generate & open docs
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Unit tests only (fast)
cargo test --lib

# Integration tests
cargo test --test '*'

# With logging
RUST_LOG=debug cargo test -- --nocapture

# Specific test
cargo test test_channel_pipeline
```

### Code Quality Standards

**Zero Tolerance in Production:**
```rust
// ❌ Never in production code
.unwrap()
.expect("...")
panic!("...")
todo!()
unimplemented!()

// ✅ Always use
.map_err(|e| PipelineError::...)?
.ok_or_else(|| PipelineError::...)?
Result<T, PipelineError>
```

**Enforced by CI:**
```bash
make lint-strict  # Must pass for merge

# Denies:
# - clippy::unwrap_used
# - clippy::expect_used
# - clippy::panic
# - clippy::todo
# - clippy::unimplemented
```

### Project Structure

```
pipeline/src/
├── application/
│   ├── services/
│   │   ├── pipeline_service.rs          # Channel-based orchestration
│   │   ├── file_processor_service.rs    # File processing logic
│   │   └── transactional_chunk_writer.rs # Concurrent writes
│   └── use_cases/
│       └── restore_file.rs               # File restoration
│
├── infrastructure/
│   ├── adapters/
│   │   ├── compression_service_adapter.rs
│   │   ├── encryption_service_adapter.rs
│   │   └── file_io_service_adapter.rs
│   ├── config/
│   │   └── rayon_config.rs               # CPU pool management
│   ├── metrics/
│   │   ├── metrics_service.rs            # Prometheus integration
│   │   └── concurrency_metrics.rs        # Queue depth tracking
│   ├── runtime/
│   │   ├── resource_manager.rs           # CPU/IO tokens
│   │   └── supervisor.rs                 # Task supervision
│   └── repositories/
│       └── sqlite_pipeline_repository.rs # Pipeline persistence
│
├── presentation/
│   └── cli/
│       └── commands.rs                    # CLI interface
│
└── main.rs                                # Entry point

bootstrap/src/
├── config.rs           # DI container
├── signals.rs          # Signal handling
└── platform/           # Platform abstractions

pipeline-domain/src/
├── entities/           # Business entities
├── value_objects/      # Domain values
└── services/           # Domain services (sync)
```

## 🎯 Advanced Usage

### Custom Pipeline Configuration

```rust
use pipeline::PipelineBuilder;

// Create custom pipeline
let pipeline = PipelineBuilder::new("secure-backup")
    .add_compression("zstd", 9)
    .add_encryption("aes256gcm")
    .add_integrity_check()
    .chunk_size_mb(16)
    .workers(8)
    .build()?;

// Execute
pipeline.process("input.dat", "output.adapipe").await?;
```

### Benchmarking

```bash
# Auto-benchmark all file sizes
./target/release/pipeline benchmark

# Specific size with iterations
./target/release/pipeline benchmark \
  --size-mb 1000 \
  --iterations 5 \
  --output-report bench_results.md

# Compare configurations
./target/release/pipeline compare \
  --configs baseline.toml,optimized.toml \
  --size-mb 500
```

### Monitoring

```bash
# Start with metrics
./target/release/pipeline serve --metrics-port 9090

# Query Prometheus
curl http://localhost:9090/metrics

# Key metrics:
# - pipeline_throughput_bytes_per_second
# - pipeline_cpu_queue_depth
# - pipeline_worker_utilization
# - pipeline_chunk_processing_duration_ms
```

### Platform-Specific Builds

```bash
# Cross-compilation (requires cross)
make install-cross-targets

# Linux x86_64
make build-linux-x86_64

# macOS Apple Silicon
make build-macos-aarch64

# Windows x64
make build-windows-x86_64

# All platforms
make build-all-platforms
```

## 🔧 Configuration

### Example: `pipeline.toml`

```toml
[pipeline]
name = "production-pipeline"
chunk_size_mb = 8        # Adaptive default
parallel_workers = 0     # 0 = auto-detect

[compression]
algorithm = "zstd"
level = "balanced"       # fast | balanced | best
parallel_processing = true

[encryption]
algorithm = "aes256gcm"
key_derivation = "argon2id"
memory_cost = 65536     # 64 MB
iterations = 3

[performance]
memory_limit_mb = 2048
use_memory_mapping = true
cpu_throttle = false
simd_enabled = true

[observability]
metrics_enabled = true
metrics_port = 9090
trace_level = "info"    # trace | debug | info | warn | error
```

### Environment Variables

```bash
# Database
export ADAPIPE_SQLITE_PATH="pipeline.db"

# Logging
export RUST_LOG="pipeline=debug,tower_http=warn"

# Performance
export RAYON_NUM_THREADS=8
export TOKIO_WORKER_THREADS=4
```

## 🤝 Contributing

We welcome contributions! This project showcases production-grade Rust patterns.

### Getting Started

```bash
# Fork the repository
git clone https://github.com/YOUR_USERNAME/optimized_adaptive_pipeline_rs.git

# Create feature branch
git checkout -b feature/amazing-feature

# Make changes, add tests
make test

# Ensure quality
make lint-strict
make format

# Commit with conventional commits
git commit -m "feat: add amazing feature"

# Push and create PR
git push origin feature/amazing-feature
```

### Development Guidelines

1. **All production code must be panic-free** (`make lint-strict` must pass)
2. **Domain layer stays pure** (no async, no I/O dependencies)
3. **Test coverage** for new features (unit + integration)
4. **Documentation** for public APIs (rustdoc comments)
5. **Performance** - benchmark before/after for critical paths

### Code Review Checklist

- [ ] No `unwrap/expect/panic` in production code
- [ ] Domain layer remains pure (no async)
- [ ] Tests added and passing
- [ ] Benchmarks show no regression
- [ ] Documentation updated
- [ ] `make lint-strict` passes
- [ ] Conventional commit messages

## 📚 Resources

### Documentation

- [Channel-Based Architecture](docs/EXECUTION_VS_PROCESSING_PIPELINES.md)
- [Database Setup](docs/DATABASE_SETUP.md)
- [Performance Tuning](docs/adaptive-performance-optimization.md)
- [API Documentation](https://docs.rs/optimized-adaptive-pipeline-rs)

## 📄 License

This project is licensed under the **BSD 3-Clause License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Architecture inspired by production systems at scale
- Concurrency patterns from Tokio/Rayon ecosystems
- Security design following NIST Cybersecurity Framework
- Domain-Driven Design principles from Eric Evans
- Built with ❤️ by the Rust community

---

**Built with Rust 🦀 | Production-Ready ✨ | Performance-First ⚡**
