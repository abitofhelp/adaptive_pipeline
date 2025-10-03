//! # Pipeline Domain
//!
//! The pipeline domain represents the core business logic and rules of the
//! pipeline system. It implements Domain-Driven Design (DDD) patterns and is
//! completely independent of external concerns like databases, file systems, or
//! user interfaces.
//!
//! ## Module Structure
//!
//!
//! ## Domain-Driven Design Concepts
//!
//! ### Entities
//! Entities are objects that have a distinct identity that runs through time
//! and different representations. They can change state while maintaining
//! their identity.
//!
//! **Key Characteristics:**
//! - Have unique identifiers
//! - Can be mutated (state changes)
//! - Identity persists through changes
//! - Equality based on identity, not attributes
//!
//! **Examples in this domain:**
//! - `Pipeline`: A processing workflow with stages
//! - `PipelineStage`: An individual processing step
//! - `ProcessingContext`: Runtime execution context
//! - `SecurityContext`: Security and permission management
//! - `ProcessingMetrics`: Performance and operational metrics
//!
//!
//!
//! ### Value Objects
//! Value objects are immutable objects that represent concepts without
//! identity. They are defined by their attributes and two value objects with
//! the same attributes are considered equal.
//!
//! **Key Characteristics:**
//! - Immutable (cannot be changed after creation)
//! - No identity (equality based on attributes)
//! - Self-validating (enforce business rules)
//! - Side-effect free operations
//!
//! **Examples in this domain:**
//! - `ChunkSize`: Represents the size of data chunks
//! - `FileChunk`: Represents a piece of file data
//! - `Algorithm`: Represents compression/encryption algorithms
//! - `PipelineId`: Type-safe pipeline identifier
//! - `StageOrder`: Stage ordering within pipelines
//! - `WorkerCount`: Validated parallel worker count
//!
//!
//!
//! ### Domain Services
//! Domain services contain business logic that doesn't naturally fit within
//! an entity or value object. They are stateless and operate on domain objects.
//!
//! **Key Characteristics:**
//! - Stateless operations
//! - Express domain concepts
//! - Coordinate between domain objects
//! - Implement complex business rules
//!
//! **Examples in this domain:**
//! - `CompressionService`: Handles data compression logic
//! - `EncryptionService`: Manages data encryption/decryption
//! - `ChecksumService`: Calculates and verifies data integrity
//! - `PipelineService`: Orchestrates pipeline execution
//! - `FileProcessorService`: High-level file processing
//!
//!
//!
//! ### Repositories
//! Repositories provide an abstraction over data persistence, allowing the
//! domain to work with collections of objects without knowing about storage
//! details.
//!
//! **Key Characteristics:**
//! - Abstract data access
//! - Collection-oriented interface
//! - Hide persistence technology
//! - Support domain queries

//!
//! ### Domain Events
//! Domain events represent significant occurrences within the domain that
//! other parts of the system might be interested in.
//!
//! **Key Characteristics:**
//! - Represent past occurrences
//! - Immutable
//! - Carry relevant data
//! - Enable loose coupling
//!
//! **Examples in this domain:**
//! - `PipelineCreated`: Emitted when a new pipeline is created
//! - `ProcessingStarted`: Emitted when file processing begins
//! - `ProcessingCompleted`: Emitted when processing finishes
//! - `SecurityContextCreated`: Emitted for new security contexts
//!
//!
//!
//! ## Business Rules and Invariants
//!
//! The domain layer enforces important business rules:
//!
//! ### Pipeline Rules
//! - Pipelines must have at least one stage
//! - Stage order must be sequential and valid
//! - Pipeline names must be unique within a context
//!
//! ### Chunk Processing Rules
//! - Chunks must have non-zero size
//! - Chunk sequence numbers must be sequential
//! - Final chunks must be properly marked
//!
//! ### Security Rules
//! - Security contexts must be validated
//! - Encryption keys must meet strength requirements
//! - Access permissions must be checked
//!
//! ## Error Handling
//!
//! The domain uses a comprehensive error system that categorizes different
//! types of failures:
//!
//!
//!
//! ## Testing Domain Logic
//!
//! Domain objects are designed to be easily testable:
//!

pub mod aggregates;
pub mod entities;
pub mod error;
pub mod events;
pub mod repositories;
pub mod services;
pub mod value_objects;

// Re-export commonly used types for convenient access
// These exports provide a clean API surface for consumers of the domain layer
pub use entities::{
    Pipeline, PipelineStage, ProcessingContext, ProcessingMetrics, SecurityContext, SecurityLevel,
};
pub use error::PipelineError;
pub use events::*;
pub use value_objects::{ChunkSize, FileChunk};
