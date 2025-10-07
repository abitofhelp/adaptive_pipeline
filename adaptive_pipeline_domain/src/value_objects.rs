// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Domain Value Objects
//!
//! This module contains the domain value objects - immutable objects that
//! represent concepts without identity. Value objects are defined by their
//! attributes and enforce business rules through validation.
//!
//! ## Value Object Characteristics
//!
//! All value objects in this system share these characteristics:
//!
//! - **Immutability**: Cannot be modified after creation
//! - **No Identity**: Equality is based on attributes, not identity
//! - **Self-Validating**: Enforce business rules during construction
//! - **Side-Effect Free**: Operations don't modify state
//! - **Composable**: Can be combined to create more complex concepts
//!
//! ## Core Value Objects
//!
//! ### Identifiers
//! Strongly-typed identifiers that prevent mixing different types of IDs:
//!
//! - [`PipelineId`]: Unique identifier for pipeline instances
//! - [`StageId`]: Identifier for individual pipeline stages
//! - [`FileChunkId`]: Identifier for file chunks in processing
//! - [`EncryptionKeyId`]: Identifier for encryption keys
//! - [`UserId`]: Identifier for user accounts and sessions
//! - [`SessionId`]: Identifier for user sessions
//! - [`ProcessingContextId`]: Identifier for processing contexts
//! - [`SecurityContextId`]: Identifier for security contexts
//! - [`GenericId`]: Generic type-safe identifier system
//!
//!
//! ### File Processing Objects
//! Objects representing file data and processing concepts:
//!
//! - [`FileChunk`]: Immutable file chunk with integrity validation
//! - [`ChunkMetadata`]: Metadata for tracking and managing file chunks
//! - [`ChunkSize`]: Type-safe chunk size with validation and optimization
//! - [`GenericSize`]: Generic size value object with unit conversions
//!
//!
//! ### Algorithm and Configuration Objects
//! Objects representing processing algorithms and their configurations:
//!
//! - [`Algorithm`]: Type-safe algorithm specification with validation
//! - [`ProcessingStepDescriptor`]: Description of processing steps in pipeline
//! - [`StageParameters`]: Type-safe parameter management for pipeline stages
//! - [`StageOrder`]: Ordering and sequencing of pipeline stages
//! - [`WorkerCount`]: Validated worker count for parallel processing
//!
//!
//! ### File System Objects
//! Objects representing file paths and permissions with type safety:
//!
//! - [`FilePath`]: Type-safe file paths with category-specific validation
//! - [`FilePermissions`]: Cross-platform file permission management
//!
//!
//! ### Binary Format Objects
//! Objects representing the structure of processed files:
//!
//! - [`FileHeader`]: Binary file format header with integrity verification
//! - [`ChunkFormat`]: Format specification for chunk serialization
//! - [`ProcessingStepType`]: Type enumeration for processing steps
//!
//!
//! ### Security Objects
//! Objects representing security contexts and permissions:
//!
//! - [`SecurityContextId`]: Identifier for security contexts with expiration
//! - [`EncryptionKeyId`]: Type-safe encryption key identifiers
//! - [`EncryptionBenchmark`]: Performance metrics for encryption algorithms
//!
//!
//! ### Processing Configuration Objects
//! Objects representing processing parameters and requirements:
//!
//! - [`PipelineRequirements`]: Configuration for pipeline performance and
//!   security
//! - [`StageParameters`]: Type-safe parameters for individual stages
//! - [`WorkerCount`]: Validated parallel worker configuration
//!
//!
//! ## Validation and Business Rules
//!
//! Value objects enforce business rules through validation:
//!
//!
//! ### Validation Benefits
//!
//! - **Early Error Detection**: Invalid values caught at construction
//! - **No Invalid States**: Impossible to create invalid value objects
//! - **Clear Error Messages**: Descriptive validation errors
//! - **Type Safety**: Compiler enforces validation requirements
//!
//!
//! ## Composition and Transformation
//!
//! Value objects can be composed and transformed safely:
//!
//!
//! ### Transformation Patterns
//!
//! - **Immutable Transformations**: Create new value objects instead of
//!   mutating
//! - **Builder Pattern**: Fluent construction of complex value objects
//! - **Composition**: Combine simple value objects into complex ones
//! - **Type Conversion**: Safe conversion between related types
//!
//!
//! ## Serialization and Persistence
//!
//! Value objects support serialization for persistence and communication:
//!
//!
//! ### Serialization Features
//!
//! - **JSON Support**: All value objects implement `Serialize` and
//!   `Deserialize`
//! - **Type Safety**: Deserialization validates constraints
//! - **Cross-Platform**: Consistent representation across languages
//! - **Version Compatibility**: Serialization format stability
//!
//!
//! ## Testing Value Objects
//!
//! Value objects are easily testable due to their immutability:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test_value_object_equality() {
//!         let id1 = String::new("test-pipeline").unwrap();
//!         let id2 = String::new("test-pipeline").unwrap();
//!         let id3 = String::new("other-pipeline").unwrap();
//!
//!         // Equality based on value, not identity
//!         assert_eq!(id1, id2);
//!         assert_ne!(id1, id3);
//!     }
//!
//!     #[test]
//!     fn test_value_object_immutability() {
//!         let size = String::new(1024).unwrap();
//!         let doubled = size.multiply(2).unwrap();
//!
//!         // Original value unchanged
//!         assert_eq!(size.as_bytes(), 1024);
//!         assert_eq!(doubled.as_bytes(), 2048);
//!     }
//!
//!     #[test]
//!     fn test_value_object_validation() {
//!         // Valid values succeed
//!         assert!(String::new(1024).is_ok());
//!
//!         // Invalid values fail
//!         assert!(String::new(0).is_err());
//!     }
//! }
//! ```

pub mod algorithm;
pub mod binary_file_format;
pub mod chunk_metadata;
pub mod chunk_size;
pub mod encryption_benchmark;
pub mod encryption_key_id;
pub mod file_chunk;
pub mod file_chunk_id;
pub mod file_path;
pub mod file_permissions;
pub mod generic_id;
pub mod generic_size;
pub mod pipeline_id;
pub mod pipeline_requirements;
pub mod processing_context_id;
pub mod processing_step_descriptor;
pub mod security_context_id;
pub mod session_id;
pub mod stage_id;
pub mod stage_order;
pub mod stage_parameters;
pub mod user_id;
pub mod worker_count;

// Re-export all value object types for convenient access
pub use algorithm::Algorithm;
pub use binary_file_format::{ChunkFormat, FileHeader, ProcessingStepType};
pub use chunk_metadata::ChunkMetadata;
pub use chunk_size::ChunkSize;
pub use encryption_benchmark::EncryptionBenchmark;
pub use encryption_key_id::EncryptionKeyId;
pub use file_chunk::FileChunk;
pub use file_chunk_id::FileChunkId;
pub use file_path::FilePath;
pub use file_permissions::FilePermissions;
pub use generic_id::GenericId;
pub use generic_size::GenericSize;
pub use pipeline_id::PipelineId;
pub use pipeline_requirements::PipelineRequirements;
pub use processing_context_id::ProcessingContextId;
pub use processing_step_descriptor::ProcessingStepDescriptor;
pub use security_context_id::SecurityContextId;
pub use session_id::SessionId;
pub use stage_id::StageId;
pub use stage_order::StageOrder;
pub use stage_parameters::StageParameters;
pub use user_id::UserId;
pub use worker_count::WorkerCount;
