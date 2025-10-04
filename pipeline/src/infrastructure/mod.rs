// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Infrastructure Layer
//!
//! The infrastructure layer provides concrete implementations of domain
//! abstractions and handles all external system interactions. This layer
//! implements the ports defined by the domain layer and provides adapters for
//! external systems.
//!
//! ## Architecture
//!
//! The infrastructure layer follows the Hexagonal Architecture pattern and
//! provides:
//!
//! - **Repositories**: Concrete implementations of domain repository interfaces
//! - **Services**: Implementations of domain service interfaces
//! - **Adapters**: Adapters for external systems and APIs
//! - **Configuration**: System configuration and dependency injection
//!
//! ## Module Structure
//!
//! ```text
//! infrastructure/
//! ├── adapters/        # Adapters for external systems
//! ├── repositories/    # Data persistence implementations
//! └── services/        # Domain service implementations
//! ```
//!
//! ## Design Principles
//!
//! ### Dependency Inversion
//! The infrastructure layer implements interfaces defined by the domain layer.
//! It depends on domain abstractions, never the reverse.
//!
//! ### Separation of Concerns
//! Each module handles a specific type of external interaction:
//! - Repositories handle data persistence
//! - Services handle business logic implementation
//! - Adapters handle external system integration
//!
//! ### Configuration Management
//! Infrastructure components are configured through dependency injection
//! and configuration objects, making them testable and flexible.
//!
//! ## Repositories
//!
//! Repository implementations provide data persistence for domain entities:
//!
//!
//!
//! ## Services
//!
//! Service implementations provide concrete business logic:
//!
//!
//!
//! ## Adapters
//!
//! Adapters integrate with external systems and APIs:
//!
//!
//!
//! ## Error Handling
//!
//! Infrastructure components translate external errors into domain errors:
//!
//! ```rust
//! use pipeline_domain::PipelineError;
//! use std::io;
//!
//! // Infrastructure error translation
//! fn handle_io_operation() -> Result<String, PipelineError> {
//!     // External operation that might fail
//!     let result = std::fs::read_to_string("config.toml")
//!         .map_err(|e| PipelineError::IoError(format!("Failed to read config: {}", e)))?;
//!
//!     Ok(result)
//! }
//!
//! // Database error translation
//! fn handle_db_operation() -> Result<(), PipelineError> {
//!     // Translate database errors to domain errors
//!     perform_db_query().map_err(|e| {
//!         PipelineError::DatabaseError(format!("Database operation failed: {}", e))
//!     })?;
//!
//!     Ok(())
//! }
//!
//! # fn perform_db_query() -> Result<(), String> { Ok(()) }
//! ```
//!
//!
//! ## Configuration
//!
//! Infrastructure components are configured through structured configuration:
//!
//!
//!
//! ## Testing Strategy
//!
//! Infrastructure components are tested with:
//!
//! - **Unit Tests**: Test individual components with mocked external
//!   dependencies
//! - **Integration Tests**: Test with real external systems (databases, file
//!   systems)
//! - **Contract Tests**: Verify implementations match domain interfaces
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use tempfile::StringempDir;
//!
//!     #[test]
//!     fn test_sqlite_repository() {
//!         // Arrange: Set up test database
//!         let temp_dir = StringempDir::new()?;
//!         let db_path = temp_dir.path().join("test.db");
//!         let repository =
//!             SqlitePipelineRepository::new(format!("sqlite:///{}", db_path.display()))?;
//!
//!         // Act: Stringest repository operations
//!         let pipeline = create_test_pipeline();
//!         repository.save(&pipeline).await?;
//!
//!         // Assert: Verify persistence
//!         let loaded = repository.find_by_id(pipeline.id())?;
//!         assert!(loaded.is_some());
//!         println!("Repository test passed successfully");
//!     }
//! }
//! ```
//!
//! ## Performance Considerations
//!
//! Infrastructure components are optimized for:
//!
//! - **Connection Pooling**: Database connections are pooled and reused
//! - **Caching**: Frequently accessed data is cached appropriately
//! - **Batch Operations**: Multiple operations are batched when possible
//! - **Resource Management**: Resources are properly cleaned up
//!
//! ## Security
//!
//! Infrastructure components implement security best practices:
//!
//! - **Input Validation**: All external inputs are validated and sanitized
//! - **SQL Injection Prevention**: Parameterized queries used exclusively
//! - **Secure Key Storage**: Keys are encrypted at rest and zeroized in memory
//! - **Access Control**: Role-based access control enforced at all layers
//! - **Audit Logging**: All security-relevant operations are logged
//! - **TLS/SSL**: Encrypted connections to external systems
//! - **Rate Limiting**: Protection against abuse and DoS attacks
pub mod adapters;
pub mod config;
pub mod logging;
pub mod metrics;
pub mod repositories;
pub mod runtime;
pub mod services;

// Re-export concrete implementations for dependency injection
// These are the primary implementations that applications will use

// Note: Re-exports are limited to commonly used implementations to maintain
// clean API boundaries and avoid exposing internal infrastructure details.
