// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Infrastructure Adapters Module
//!
//! This module contains concrete implementations of domain interfaces (ports),
//! following the Hexagonal Architecture pattern. These adapters handle:
//!
//! - External service integrations
//! - File I/O operations
//! - Compression algorithms
//! - Encryption services
//! - Data transformation
//!
//! ## Design Principles
//!
//! 1. **Dependency Inversion**: Adapters depend on domain interfaces, not vice
//!    versa
//! 2. **Single Responsibility**: Each adapter has one clear purpose
//! 3. **Testability**: Adapters can be easily mocked or replaced
//! 4. **Configuration**: Adapters are configured through dependency injection
//!
//! ## Module Structure
//!
//! ```text
//! adapters/
//! ├── chunk_processor_adapters.rs  # Chunk processing implementations
//! ├── compression.rs               # Compression service implementations
//! ├── encryption.rs                # Encryption service implementations
//! ├── file_io.rs                   # File I/O service implementations
//! ├── async_compression.rs         # Async compression adapter
//! ├── async_encryption.rs          # Async encryption adapter
//! └── async_checksum.rs            # Async checksum adapter
//!     requires_security_context: false,
//! };
//!
//! // Create adapter
//! let adapter = ServiceChunkAdapter::new(
//!     service,
//!     "my_service".to_string(),
//!     config
//! );
//! ```
//!
//! ## Architecture Benefits
//!
//! - **Separation of Concerns**: Services focus on business logic, adapters
//!   handle integration
//! - **Reusability**: Services can be used in multiple contexts through
//!   different adapters
//! - **Testability**: Easy to test services independently of their usage
//!   context
//! - **Flexibility**: Runtime configuration of adapter behavior

/// Chunk processor adapters for service integration
pub mod chunk_processor_adapters;

/// Compression service adapter
pub mod compression;

/// Async compression adapter (wraps sync domain trait for async contexts)
pub mod async_compression;

/// Async encryption adapter (wraps sync domain trait for async contexts)
pub mod async_encryption;

/// Async checksum adapter (wraps sync domain trait for async contexts)
pub mod async_checksum;

/// Encryption service adapter
pub mod encryption;

/// File I/O service adapter
pub mod file_io;

// Re-export for easy access
pub use async_checksum::*;
pub use async_compression::*;
pub use async_encryption::*;
pub use compression::*;
pub use encryption::*;
