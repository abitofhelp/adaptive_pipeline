# Optimized Adaptive Pipeline RS

A **production-grade**, **high-performance** file processing system built with Rust, featuring advanced concurrency patterns, adaptive performance optimization, and enterprise-level reliability. This project demonstrates professional Rust development with Channel-based Architecture, Domain-Driven Design, and comprehensive error handling.

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 🚀 What Makes This Different

This isn't just another file processor - it's a **showcase of advanced Rust patterns** and **production engineering**:

- **🔄 Channel-Based Concurrency**: Reader → CPU Workers → Direct Writer pattern eliminates bottlenecks
- **⚡ Hybrid Parallelism**: Rayon for CPU-bound ops + Tokio for async I/O = optimal resource utilization
- **🎯 Adaptive Performance**: Dynamic chunk sizing and worker scaling based on file characteristics
- **🛡️ Zero-Panic Production Code**: 297 unwrap/expect/panic patterns eliminated through systematic remediation
- **🔐 Security First**: AES-256-GCM, ChaCha20-Poly1305 with Argon2 key derivation
- **📊 Observable**: Prometheus metrics, structured tracing, performance dashboards

## 📋 Table of Contents

- [Architecture](#architecture)
- [Performance](#performance)
- [Quick Start](#quick-start)
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
│   ├── signals.rs           # SIGTERM/SIGINT handling
│   └── platform/            # Cross-platform abstractions
│
└── Cargo.toml               # Workspace config
```

### Concurrency Model

**Channel-Based Execution Pipeline** (Week 2 Architecture):

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
- Zero panic in production code (297 patterns eliminated)
- Comprehensive error handling
- Graceful shutdown (SIGTERM/SIGINT)
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

### Recent Improvements

- ✅ **Week 2 Concurrency**: Channel-based pipeline with direct writes
- ✅ **Error Remediation**: 297 unwrap/expect/panic eliminated
- ✅ **Rayon Integration**: Parallel CPU-bound processing
- ✅ **Resource Manager**: Global CPU/IO token management
- ✅ **Signal Handling**: Graceful shutdown on SIGTERM/SIGINT
- ✅ **Streaming I/O**: Memory-efficient file processing
- ✅ **License Change**: MIT → BSD 3-Clause

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
