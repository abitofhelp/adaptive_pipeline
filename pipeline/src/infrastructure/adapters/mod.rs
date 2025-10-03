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
//! ├── compression_service_adapter.rs # Compression service implementations
//! ├── encryption_service_adapter.rs  # Encryption service implementations
//! └── file_io_service_adapter.rs    # File I/O service implementations
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
pub mod compression_service_adapter;

/// Async compression adapter (wraps sync domain trait for async contexts)
pub mod async_compression_adapter;

/// Async encryption adapter (wraps sync domain trait for async contexts)
pub mod async_encryption_adapter;

/// Async checksum adapter (wraps sync domain trait for async contexts)
pub mod async_checksum_adapter;

/// Encryption service adapter
pub mod encryption_service_adapter;

/// File I/O service adapter
pub mod file_io_service_adapter;

/// Repository adapters
pub mod repositories;

// Re-export for easy access
pub use async_checksum_adapter::*;
pub use async_compression_adapter::*;
pub use async_encryption_adapter::*;
pub use compression_service_adapter::*;
pub use encryption_service_adapter::*;
