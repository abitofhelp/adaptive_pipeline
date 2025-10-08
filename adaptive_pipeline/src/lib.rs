// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

// Production code safety enforced via CI and `make lint-strict`
// (lib/bins checked separately from tests - tests may use unwrap/expect)

//! # Adaptive Pipeline
//!
//! A high-performance, secure file processing pipeline system built with Rust.
//! This crate provides a comprehensive framework for processing files through
//! configurable stages including compression, encryption, and validation.
//!
//! ## Architecture Overview
//!
//! The pipeline follows Clean Architecture and Domain-Driven Design principles:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Interface Layer                          │
//! │  (CLI, Web API, Configuration Management)                   │
//! └─────────────────────────────────────────────────────────────┘
//!                                │
//! ┌─────────────────────────────────────────────────────────────┐
//! │                  Application Layer                          │
//! │  (Use Cases, Command Handlers, Application Services)        │
//! └─────────────────────────────────────────────────────────────┘
//!                                │
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Domain Layer                             │
//! │  (Entities, Value Objects, Domain Services, Events)         │
//! └─────────────────────────────────────────────────────────────┘
//!                                │
//! ┌─────────────────────────────────────────────────────────────┐
//! │                Infrastructure Layer                         │
//! │  (Database, File System, External APIs, Implementations)    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Core Concepts
//!
//! ### Pipeline
//! A pipeline represents a complete file processing workflow consisting of
//! multiple stages. Each pipeline has:
//! - A unique identifier
//! - Ordered processing stages
//! - Configuration parameters
//! - Security context
//!
//! ### Stages
//! Processing stages are individual operations that transform file data:
//! - **Compression**: Reduces file size using various algorithms
//! - **Encryption**: Secures data using cryptographic methods
//! - **Validation**: Ensures data integrity through checksums
//!
//! ### File Chunks
//! Large files are processed in chunks for memory efficiency:
//! - Configurable chunk sizes
//! - Parallel processing support
//! - Integrity verification per chunk
//!
//! ## Quick Start
//!
//!
//! ## Features
//!
//! ### Compression Algorithms
//! - **Brotli**: High compression ratio, good for text files
//! - **LZ4**: Fast compression/decompression, good for real-time processing
//! - **Zstandard**: Balanced compression ratio and speed
//!
//! ### Encryption Algorithms
//! - **AES-256-GCM**: Authenticated encryption with associated data
//! - **ChaCha20-Poly1305**: Modern stream cipher with authentication
//! - **XChaCha20-Poly1305**: Extended nonce variant of ChaCha20-Poly1305
//!
//! ### Security Features
//! - Multiple security levels (Public, Internal, Confidential, Secret,
//!   TopSecret)
//! - Key derivation using Argon2
//! - Secure key management
//! - Integrity verification
//!
//! ## File Format
//!
//! The pipeline produces `.adapipe` files with the following structure:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    File Header                              │
//! │  (Metadata, Algorithm Info, Security Context)              │
//! ├─────────────────────────────────────────────────────────────┤
//! │                   Processed Data                           │
//! │  (Compressed and/or Encrypted File Content)                │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    File Footer                              │
//! │  (Checksums, Processing Summary, Verification Data)         │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Error Handling
//!
//! The pipeline uses a comprehensive error system that categorizes failures:
//!
//!
//! ## Performance Considerations
//!
//! ### Memory Management
//! - Streaming processing for large files
//! - Configurable chunk sizes based on available memory
//! - Automatic memory pressure detection
//!
//! ### Parallel Processing
//! - Multi-threaded chunk processing
//! - Configurable worker thread counts
//! - NUMA-aware thread placement
//!
//! ### Monitoring
//! - Built-in metrics collection
//! - Performance tracking per stage
//! - Resource utilization monitoring
//!
//! ## Testing
//!
//! The crate includes comprehensive test coverage:
//!
//! ```bash
//! # Run all tests
//! cargo test
//!
//! # Run unit tests only
//! make test-unit
//!
//! # Run integration tests
//! make test-integration
//!
//! # Generate documentation
//! cargo doc --open
//! ```
//!
//! ## Examples
//!
//! See the `examples/` directory for complete usage examples:
//! - Basic file processing
//! - Batch processing
//! - Custom pipeline configurations
//! - Error handling patterns
//! - Performance optimization
//!
//! ## Safety and Security
//!
//! This crate prioritizes safety and security:
//! - Memory-safe Rust implementation
//! - Cryptographically secure random number generation
//! - Constant-time cryptographic operations
//! - Secure key zeroization
//! - Input validation and sanitization
//!
//! # Adaptive Pipeline Processing Library
//!
//! A high-performance, secure file processing pipeline system built in Rust
//! that provides a flexible framework for creating custom file processing
//! workflows with built-in compression, encryption, and integrity verification
//! capabilities.
//!
//! ## Overview
//!
//! This library implements a sophisticated pipeline processing system designed
//! for enterprise-grade file processing operations. It combines performance,
//! security, and flexibility to handle complex data transformation workflows.
//!
//! ## Key Features
//!
//! - **Modular Pipeline Architecture**: Compose processing stages for custom
//!   workflows
//! - **High Performance**: Optimized for large file processing with concurrent
//!   operations
//! - **Security First**: Built-in encryption, integrity checks, and access
//!   controls
//! - **Extensible Design**: Plugin architecture for custom processing stages
//! - **Robust Error Handling**: Comprehensive error types and recovery
//!   mechanisms
//! - **Observability**: Built-in metrics, logging, and monitoring capabilities
//! - **ACID Transactions**: Transactional chunk writing with rollback support
//! - **Event-Driven Architecture**: Domain events for system integration
//!
//! ## Quick Start
//!
//!
//! ### Basic File Processing
//!
//!
//!
//! ## Architecture Overview
//!
//! The library follows Clean Architecture and Domain-Driven Design principles:
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │            Interface Layer          │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │
//! │  │   API   │ │   CLI   │ │ Config  │ │
//! │  └─────────┘ └─────────┘ └─────────┘ │
//! └─────────────────┬───────────────────┘
//! ┌─────────────────┴───────────────────┐
//! │         Application Layer           │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │
//! │  │Commands │ │Handlers │ │Services │ │
//! │  └─────────┘ └─────────┘ └─────────┘ │
//! └─────────────────┬───────────────────┘
//! ┌─────────────────┴───────────────────┐
//! │            Domain Layer             │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │
//! │  │Entities │ │Services │ │ Events  │ │
//! │  └─────────┘ └─────────┘ └─────────┘ │
//! └─────────────────┬───────────────────┘
//! ┌─────────────────┴───────────────────┐
//! │        Infrastructure Layer         │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐ │
//! │  │Database │ │File I/O │ │External │ │
//! │  └─────────┘ └─────────┘ └─────────┘ │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Core Components
//!
//! ### Domain Layer
//! - **Entities**: Core business objects with identity
//! - **Value Objects**: Immutable objects defined by attributes
//! - **Aggregates**: Consistency boundaries and transaction roots
//! - **Domain Services**: Stateless business operations
//! - **Domain Events**: Business occurrences for decoupled communication
//!
//! ### Application Layer
//! - **Commands**: Operations that change system state
//! - **Queries**: Read operations for data retrieval
//! - **Handlers**: Execute commands and queries
//! - **Application Services**: Complex workflow coordination
//!
//! ### Infrastructure Layer
//! - **Repositories**: Data persistence implementations
//! - **External Services**: Integration with external systems
//! - **File I/O**: File system operations
//! - **Encryption/Compression**: Cryptographic and compression services
//!
//! ### Interface Layer
//! - **API**: RESTful HTTP endpoints
//! - **CLI**: Command-line interface
//! - **Configuration**: System settings and configuration
//!
//! ## Usage Examples
//!
//! ### Advanced Pipeline Configuration
//!
//!
//!
//! ### Transactional Processing
//!
//!
//!
//! ### Event-Driven Processing
//!
//!
//!
//! ## Performance Characteristics
//!
//! - **Streaming Processing**: Handles files of any size with constant memory
//!   usage
//! - **Concurrent Operations**: Parallel chunk processing for improved
//!   throughput
//! - **Zero-Copy Operations**: Minimizes data copying for optimal performance
//! - **Adaptive Algorithms**: Automatically selects optimal processing
//!   parameters
//!
//! ## Security Features
//!
//! - **Strong Encryption**: AES-256, ChaCha20Poly1305 support
//! - **Integrity Verification**: SHA-256, BLAKE3 checksums
//! - **Access Control**: Role-based permissions and security levels
//! - **Audit Trail**: Comprehensive logging of all operations
//! - **Secure Defaults**: Security-first configuration
//!
//! ## Error Handling
//!
//! Comprehensive error handling with specific error types:
//!
//!
//!
//! ## Testing
//!
//! The library includes comprehensive test coverage:
//!
//! - Unit tests for all components
//! - Integration tests for complete workflows
//! - Property-based testing for edge cases
//! - Performance benchmarks
//!
//! ## Contributing
//!
//! Contributions are welcome! Please see CONTRIBUTING.md for guidelines.
//!
//! ## License
//!
//! This project is licensed under the BSD 3-Clause License - see LICENSE file
//! for details.

pub mod application;
pub mod infrastructure;
pub mod presentation;

// Tests are organized as:
// - Unit tests: #[cfg(test)] modules within each source file
// - Integration tests: separate files in tests/ directory

// Re-export domain types for convenient access
pub use adaptive_pipeline_domain::{
    ChunkSize, FileChunk, Pipeline, PipelineError, PipelineStage, ProcessingContext, ProcessingMetrics,
    SecurityContext, SecurityLevel,
};

// Re-export restoration functions for testing
pub use crate::application::use_cases::restore_file::create_restoration_pipeline;
