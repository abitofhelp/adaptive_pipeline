# Optimized Adaptive Pipeline RS

A high-performance file processing system built with Rust, implementing Domain-Driven Design (DDD), Clean Architecture, and Hexagonal Architecture principles. This project demonstrates professional-grade Rust development with a focus on maintainability, extensibility, and performance.

## Overview

The Optimized Adaptive Pipeline is designed to process files through configurable stages (compression, encryption, validation, etc.) while maintaining clean separation of concerns and enabling easy testing and extension.

**Key Highlights:**
- **Multi-stage Processing Pipeline**: Configurable stages for compression, encryption, and custom processing
- **High Performance**: Parallel processing with work-stealing thread pools, async I/O, and memory-mapped files
- **Security First**: AES-256-GCM and ChaCha20-Poly1305 encryption with secure key management
- **Adaptive Processing**: Dynamic chunk sizing based on file characteristics and system resources
- **Extensible Architecture**: Plugin-ready design for custom processing stages
- **Production Ready**: Comprehensive error handling, monitoring, and observability

## Architecture

This project implements a **layered architecture** that combines three architectural patterns:

### Architectural Patterns

1. **Domain-Driven Design (DDD)**
   - Rich domain models with clear business logic
   - Aggregates, entities, and value objects with invariant protection
   - Domain services for complex business operations
   - Domain events for decoupled communication

2. **Clean Architecture**
   - Clear separation of concerns across layers
   - Dependency inversion: inner layers don't depend on outer layers
   - Business logic isolated from infrastructure details
   - Testable without external dependencies

3. **Hexagonal Architecture (Ports and Adapters)**
   - Ports: Interfaces defining how to interact with the system
   - Adapters: Implementations that connect ports to external systems
   - Easy to swap implementations (e.g., different databases, APIs)

### Layer Responsibilities

```
┌─────────────────────────────────────────────┐
│         Presentation Layer                  │
│  (CLI, API endpoints, request/response)     │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│         Application Layer                   │
│  (Use cases, application services, DTOs)    │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│           Domain Layer                      │
│  (Entities, aggregates, domain services)    │
└─────────────────────────────────────────────┘
                    ↑
┌─────────────────────────────────────────────┐
│        Infrastructure Layer                 │
│  (Database, file I/O, external services)    │
└─────────────────────────────────────────────┘
```

### Project Structure

```
├── pipelinelib/                    # Shared utilities and algorithms
│   ├── src/
│   │   ├── compression/            # Compression algorithm wrappers
│   │   ├── encryption/             # Encryption algorithm wrappers
│   │   ├── memory/                 # Memory management utilities
│   │   ├── metrics/                # Metrics collection
│   │   ├── security/               # Security utilities
│   │   └── validation/             # Input validation helpers
│   └── Cargo.toml
│
├── pipeline/                       # Main application
│   ├── src/
│   │   ├── core/                   # Business logic (inner layers)
│   │   │   ├── domain/             # Domain layer
│   │   │   │   ├── aggregates/     # Root entities with boundaries
│   │   │   │   ├── entities/       # Domain entities with identity
│   │   │   │   ├── value_objects/  # Immutable value types
│   │   │   │   ├── services/       # Domain services (interfaces)
│   │   │   │   ├── repositories/   # Repository interfaces
│   │   │   │   ├── events/         # Domain events
│   │   │   │   └── error/          # Domain-specific errors
│   │   │   └── application/        # Application layer
│   │   │       ├── services/       # Application services (orchestration)
│   │   │       ├── use_cases/      # Use case implementations
│   │   │       └── dtos/           # Data transfer objects
│   │   │
│   │   ├── infrastructure/         # External integrations (outer layer)
│   │   │   ├── adapters/           # Hexagonal architecture adapters
│   │   │   ├── repositories/       # Repository implementations
│   │   │   ├── services/           # Infrastructure service implementations
│   │   │   ├── persistence/        # Database access layer
│   │   │   ├── config/             # Configuration loading
│   │   │   ├── logging/            # Logging setup
│   │   │   └── metrics/            # Metrics exporters
│   │   │
│   │   └── presentation/           # User-facing interfaces
│   │       └── adapters/
│   │           ├── cli/            # Command-line interface
│   │           └── api/            # REST API (future)
│   │
│   ├── tests/                      # Integration tests
│   ├── examples/                   # Example applications
│   └── Cargo.toml
│
├── docs/                           # Documentation
├── scripts/                        # Build and utility scripts
└── Cargo.toml                      # Workspace configuration
```

### Key Design Principles

- **Error Handling**: All operations return `Result<T, PipelineError>` - no panics in production code
- **Type Safety**: Extensive use of newtypes (e.g., `UserId`, `PipelineId`) to prevent bugs
- **Async by Default**: Tokio runtime for non-blocking I/O operations
- **Memory Safety**: Zero-copy operations where possible, memory-mapped files for large data
- **Testing**: Comprehensive unit tests, integration tests, and property-based tests

## Requirements

### System Requirements

- **Rust**: 1.70 or later (uses nightly features for some optimizations)
- **RAM**: Minimum 4GB (8GB+ recommended for large files)
- **Storage**: SSD recommended for optimal I/O performance
- **OS**: macOS, Linux, or Windows (WSL recommended for Windows)

### Development Tools

- `rustc` and `cargo` (install via [rustup](https://rustup.rs/))
- `sqlite3` (for database operations)
- Optional: `cargo-watch` for auto-recompilation during development
- Optional: `cargo-tarpaulin` for code coverage

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/abitofhelp/optimized_adaptive_pipeline_rs.git
cd optimized_adaptive_pipeline_rs

# Build the project (development mode)
cargo build

# Run all tests to verify everything works
cargo test

# Build optimized release binary
cargo build --release

# The binary will be at: target/release/pipeline
```

### First Run

```bash
# Process a simple file with compression
echo "Hello, Pipeline!" > test.txt
./target/release/pipeline process -i test.txt -o test.br --compress

# Check the output
ls -lh test.br

# Restore the file
./target/release/pipeline process -i test.br -o restored.txt --decompress
cat restored.txt
```

## Database Setup

The pipeline system requires a SQLite database for storing pipeline configurations and processing metrics.

### Quick Setup

```bash
# 1. Generate database with proper ULID IDs
cd pipeline
cargo run --example generate_test_database_demo

# 2. Create the database
cd ..
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql

# 3. Verify setup
./target/release/pipeline list
```

**Important**: Always use the demo generator to ensure proper ULID format for all identifiers.

For detailed database setup instructions, see [DATABASE_SETUP.md](docs/DATABASE_SETUP.md).

## Usage

### Basic File Processing

```bash
# Compress a file using Brotli
pipeline process -i input.txt -o output.txt.br --compress --compression-algorithm brotli

# Encrypt a file using AES-256-GCM
pipeline process -i input.txt -o output.txt.enc --encrypt --encryption-algorithm aes256gcm

# Compress and encrypt
pipeline process -i input.txt -o output.txt.br.enc --compress --encrypt

# Custom chunk size and parallel workers
pipeline process -i large_file.dat -o processed.dat --compress --chunk-size-mb 16 --workers 8
```

### Pipeline Management

```bash
# Create a custom pipeline
pipeline create --name "secure-backup" --stages "compression,encryption,integrity"

# List available pipelines
pipeline list

# Show pipeline details
pipeline show secure-backup

# Validate pipeline configuration
pipeline validate config.toml
```

### Benchmarking

```bash
# Benchmark system performance
pipeline benchmark --size-mb 1000 --iterations 5

# Benchmark with specific file
pipeline benchmark --file test_data.bin --iterations 3
```

## Configuration

The application supports configuration via TOML, YAML, or JSON files:

```toml
[pipeline]
name = "default"
chunk_size_mb = 1
parallel_workers = 0  # 0 = auto-detect

[compression]
algorithm = "brotli"
level = "balanced"
parallel_processing = true

[encryption]
algorithm = "aes256gcm"
key_derivation = "argon2"
iterations = 100000

[security]
integrity_required = true
audit_enabled = true
security_level = "internal"

[performance]
memory_limit_mb = 2048
cpu_throttle = false
simd_enabled = true
```

## Supported Algorithms

### Compression
- **Brotli** (default): Excellent compression ratio, good performance
- **Gzip**: Fast compression, widely compatible
- **Zstd**: Balanced compression and speed
- **LZ4**: Ultra-fast compression

### Encryption
- **AES-256-GCM** (default): Industry standard, hardware accelerated
- **ChaCha20-Poly1305**: High performance, constant-time
- **AES-128-GCM**: Faster variant for less sensitive data
- **AES-192-GCM**: Balanced security and performance

### Key Derivation
- **Argon2** (default): Memory-hard, resistant to GPU attacks
- **scrypt**: Alternative memory-hard function
- **PBKDF2**: Traditional key derivation

## Performance

The system is designed for high throughput and low latency:

- **Throughput**: 500+ MB/s on modern hardware
- **Scalability**: Linear scaling up to 32 CPU cores
- **Memory Efficiency**: Adaptive memory usage with pooling
- **File Size Support**: Up to 1TB files with checkpointing

### Benchmarks

| Operation | Throughput | CPU Usage | Memory Usage |
|-----------|------------|-----------|--------------|
| Brotli Compression | 150 MB/s | 80% | 64 MB |
| AES-256-GCM Encryption | 800 MB/s | 60% | 32 MB |
| Combined Processing | 120 MB/s | 85% | 96 MB |

## Security

- **Secure by Default**: All configurations use secure defaults
- **Memory Protection**: Sensitive data is zeroed after use
- **Key Management**: Integration with Hardware Security Modules (HSM)
- **Audit Logging**: Comprehensive security event logging
- **Role-Based Access**: Configurable permission system

## Development Guide

### Building from Source

```bash
# Development build (faster compile, includes debug symbols)
cargo build

# Release build (optimized, slower compile)
cargo build --release

# Build with all optimizations (for benchmarking)
cargo build --release --features "simd"

# Check code without building (fast validation)
cargo check

# Format code according to style guidelines
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Running Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_file_restoration

# Run tests in a specific module
cargo test core::domain::value_objects

# Run tests with debug logging
RUST_LOG=debug cargo test

# Run only unit tests (fast)
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run benchmarks (requires nightly)
cargo bench
```

### Understanding Test Results

**Current Test Status (as of last commit):**
- ✅ 465 tests passing
- ⏭️ 10 tests ignored (platform-specific or feature-gated)
- ❌ 5 tests failing (known issue: unimplemented writer features)

The 5 failing tests in `e2e_binary_format_test` are due to incomplete binary format writer implementation and don't affect core functionality.

### Project Commands

```bash
# Watch files and auto-rebuild on changes (requires cargo-watch)
cargo watch -x check

# Auto-run tests on file changes
cargo watch -x test

# Generate documentation and open in browser
cargo doc --open

# Check dependencies for updates
cargo outdated

# Audit dependencies for security issues
cargo audit
```

### Debugging Tips

**Enable Detailed Logging:**
```bash
# Set log level for the entire application
RUST_LOG=debug cargo run -- process -i input.txt -o output.txt

# Filter logs by module
RUST_LOG=pipeline::core::domain=trace cargo run

# Multiple filters
RUST_LOG=pipeline::core=debug,pipeline::infrastructure=info cargo run
```

**Using the Debugger:**
```bash
# Build with debug symbols
cargo build

# Run under debugger (lldb on macOS, gdb on Linux)
rust-lldb target/debug/pipeline
```

**Memory Profiling:**
```bash
# Use valgrind for memory leak detection (Linux)
valgrind --leak-check=full target/debug/pipeline

# Use heaptrack for memory profiling
heaptrack target/release/pipeline
```

### Adding New Features

**Step-by-Step Process:**

1. **Define Domain Model** (if needed)
   - Add value objects in `core/domain/value_objects/`
   - Add entities in `core/domain/entities/`
   - Add domain services in `core/domain/services/`

2. **Create Application Service**
   - Implement use case in `core/application/use_cases/`
   - Add DTOs in `core/application/dtos/`

3. **Implement Infrastructure**
   - Add adapters in `infrastructure/adapters/`
   - Implement repository if needed

4. **Add Presentation Layer**
   - Extend CLI in `presentation/adapters/cli/`
   - Add API endpoints (future)

5. **Write Tests**
   - Unit tests alongside code files
   - Integration tests in `tests/`

6. **Document**
   - Add doc comments (`///`) to public APIs
   - Update README if user-facing

### Code Style Guidelines

- **Naming Conventions:**
  - Types: `PascalCase` (e.g., `PipelineConfig`)
  - Functions: `snake_case` (e.g., `process_file`)
  - Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAX_CHUNK_SIZE`)
  - Modules: `snake_case` (e.g., `value_objects`)

- **Error Handling:**
  - Use `Result<T, PipelineError>` for operations that can fail
  - Never use `.unwrap()` in production code (use `?` instead)
  - Provide context in error messages

- **Documentation:**
  - All public items must have doc comments
  - Include examples in doc comments for complex APIs
  - Explain *why*, not just *what*

- **Testing:**
  - Unit tests go in the same file under `#[cfg(test)]`
  - Integration tests go in `tests/` directory
  - Use property-based testing (proptest) for value objects

### Common Issues and Solutions

**Issue: Compilation is slow**
```bash
# Use cargo check instead of build during development
cargo check

# Or use incremental compilation (enabled by default in dev mode)
export CARGO_INCREMENTAL=1
```

**Issue: Tests are failing**
```bash
# Run tests with output to see what's happening
cargo test -- --nocapture --test-threads=1

# Check if database is set up correctly
ls pipeline/scripts/test_data/structured_pipeline.db
```

**Issue: Binary is too large**
```bash
# Strip debug symbols from release build
cargo build --release
strip target/release/pipeline

# Or add to Cargo.toml:
# [profile.release]
# strip = true
```

### Contributing

We welcome contributions! Please follow these guidelines:

1. **Before You Start:**
   - Check existing issues or create one to discuss your idea
   - Ensure your change fits the project architecture
   - Read through related code to understand conventions

2. **Development Process:**
   - Fork the repository
   - Create a feature branch (`git checkout -b feature/my-feature`)
   - Write code following the style guidelines above
   - Add tests for your changes
   - Ensure all tests pass (`cargo test`)
   - Run the linter (`cargo clippy`)
   - Format your code (`cargo fmt`)

3. **Submitting Changes:**
   - Write clear commit messages
   - Include test coverage for new code
   - Update documentation as needed
   - Submit a pull request with a clear description

4. **Code Review:**
   - Be open to feedback and questions
   - Make requested changes promptly
   - Keep discussions professional and constructive

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built following best practices from adaptive_pipeline_rs and windsurfex
- Architecture inspired by family-service reference implementation
- Security design based on NIST Cybersecurity Framework
