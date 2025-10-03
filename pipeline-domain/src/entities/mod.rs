//! # Domain Entities
//!
//! This module contains the domain entities - objects with distinct identity
//! that can change state over time while maintaining their identity. Entities
//! represent the core business objects in the pipeline processing domain.
//!
//! ## Entity Characteristics
//!
//! Domain entities in this system have the following characteristics:
//!
//! - **Identity**: Each entity has a unique identifier that persists through
//!   changes
//! - **Mutability**: Entities can change state while maintaining their identity
//! - **Lifecycle**: Entities have creation, modification, and potentially
//!   deletion lifecycles
//! - **Business Logic**: Entities encapsulate business rules and invariants
//! - **Equality**: Two entities are equal if they have the same identity,
//!   regardless of state
//!
//! ## Core Entities
//!
//! ### Pipeline
//! The central entity representing a complete file processing workflow.
//! Contains ordered stages and manages the overall processing logic.
//!
//! **Key Features**:
//! - Unique pipeline identifier for tracking and management
//! - Ordered collection of processing stages
//! - Pipeline configuration and requirements
//! - State management throughout execution
//! - Validation of stage ordering and dependencies
//!
//! **Use Cases**:
//! - File compression and encryption workflows
//! - Multi-stage data transformation pipelines
//! - Configurable processing chains
//! - Auditable processing workflows
//!
//!
//! ### PipelineStage
//! Represents an individual processing step within a pipeline.
//! Each stage performs a specific transformation on the data.
//!
//! **Key Features**:
//! - Unique stage identifier and ordering
//! - Stage-specific configuration and parameters
//! - Input/output type specifications
//! - Stage execution status tracking
//! - Error handling and recovery
//!
//! **Stage Types**:
//! - Compression stages (Zlib, Zstd, Brotli)
//! - Encryption stages (AES-256-GCM, ChaCha20-Poly1305)
//! - Validation stages (checksum, integrity)
//! - Custom transformation stages
//!
//!
//! ### ProcessingContext
//! Maintains the runtime state and context for pipeline execution.
//! Tracks progress, metrics, and execution state.
//!
//! **Key Features**:
//! - Unique context identifier for tracing
//! - Current execution state and progress tracking
//! - Performance metrics collection
//! - Error tracking and reporting
//! - Resource usage monitoring
//!
//! **Tracked Information**:
//! - Bytes processed and remaining
//! - Current pipeline stage
//! - Elapsed processing time
//! - Memory and CPU utilization
//! - Error history and recovery attempts
//!
//!
//! ### SecurityContext
//! Manages security-related information and permissions for processing
//! operations. Enforces access control and security policies.
//!
//! **Key Features**:
//! - User identity and authentication
//! - Permission-based access control
//! - Security level enforcement
//! - Audit trail generation
//! - Session management and expiration
//!
//! **Security Levels**:
//! - Public: No authentication required
//! - Authenticated: Valid user authentication
//! - Confidential: Enhanced permissions required
//! - Secret: Highest level access control
//!
//!
//! ### ProcessingMetrics
//! Collects and aggregates performance and operational metrics during
//! processing. Provides insights into system performance and resource
//! utilization.
//!
//! **Key Features**:
//! - Real-time performance metric collection
//! - Throughput measurement (bytes/second)
//! - Latency tracking (operation duration)
//! - Resource utilization monitoring
//! - Error rate calculation
//!
//! **Collected Metrics**:
//! - Total bytes processed
//! - Processing throughput (MB/s)
//! - Average/min/max latencies
//! - Memory consumption
//! - CPU utilization percentage
//! - Stage-specific performance data
//!
//!
//! ### TransactionalChunkWriter
//! Provides transactional semantics for writing file chunks with rollback
//! capabilities. Ensures data consistency during chunk-based file operations.
//!
//! **Key Features**:
//! - ACID transaction support for chunk writes
//! - Automatic rollback on write failures
//! - Staged write-commit pattern
//! - Integrity verification before commit
//! - Cleanup of partial writes on error
//!
//! **Transaction Lifecycle**:
//! 1. Begin transaction
//! 2. Write chunks to staging area
//! 3. Validate chunk integrity
//! 4. Commit or rollback transaction
//! 5. Clean up temporary resources
//!
//!
//! ## Entity Lifecycle Management
//!
//! Entities follow a well-defined lifecycle:
//!
//! 1. **Creation**: Entities are created with valid initial state
//! 2. **Validation**: All state changes are validated against business rules
//! 3. **Modification**: Entities can be modified through controlled methods
//! 4. **Persistence**: Entity state can be persisted and restored
//! 5. **Cleanup**: Resources are properly cleaned up when entities are no
//!    longer needed
//!
//! ## Business Rules and Invariants
//!
//! Entities enforce important business rules:
//!
//! - **Pipeline Rules**: Pipelines must have at least one stage, stages must be
//!   ordered
//! - **Security Rules**: Security contexts must be validated, permissions must
//!   be checked
//! - **Processing Rules**: Processing context must track accurate metrics and
//!   state
//! - **Data Integrity**: All data modifications must maintain consistency
//!
//! ## Testing Entities
//!
//! Entities are designed to be easily testable:
//!
//! ```rust
//! # #![allow(unused)]
//! # use uuid::Uuid;
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct EntityId(Uuid);
//! #[derive(Debug, Clone)]
//! struct Device {
//!     id: EntityId,
//!     name: String,
//! }
//! impl Device {
//!     fn new(name: impl Into<String>) -> Self {
//!         Self { id: EntityId(Uuid::new_v4()), name: name.into() }
//!     }
//!     fn rename(&mut self, name: impl Into<String>) { self.name = name.into(); }
//! }
//! let mut d = Device::new("Sensor-A");
//! d.rename("Sensor-A1");
//! assert!(matches!(d.id, EntityId(_)));
//! ```

pub mod pipeline;
pub mod pipeline_stage;
pub mod processing_context;
pub mod processing_metrics;
pub mod security_context;
// TODO: Move TransactionalChunkWriter to application layer (it's an application service, not a domain entity)
// It uses tokio directly which violates domain purity. Should be in pipeline/src/application/services/
// pub mod transactional_chunk_writer;

// Re-export all entity types for convenient access
pub use pipeline::Pipeline;
pub use pipeline_stage::{PipelineStage, StageConfiguration, StageType};
pub use processing_context::ProcessingContext;
pub use processing_metrics::ProcessingMetrics;
pub use security_context::{SecurityContext, SecurityLevel};
// pub use transactional_chunk_writer::TransactionalChunkWriter;
