// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Domain Events
//!
//! This module contains the domain events system that implements event-driven
//! architecture patterns for the pipeline processing domain. Events represent
//! significant business occurrences that have already happened and can trigger
//! reactions in other parts of the system or external systems.
//!
//! ## Overview
//!
//! The domain events system provides:
//!
//! - **Event-Driven Architecture**: Decoupled communication between domain
//!   components
//! - **Event Sourcing Support**: Events can be used to reconstruct aggregate
//!   state
//! - **Integration Events**: Communication with external systems and bounded
//!   contexts
//! - **Audit Trail**: Complete history of all significant domain occurrences
//! - **Eventual Consistency**: Coordination of state changes across aggregate
//!   boundaries
//!
//! ## Event Architecture
//!
//! ### Event Types
//!
//! The system defines several categories of domain events:
//!
//! - **Pipeline Events**: Related to pipeline lifecycle and configuration
//!   changes
//! - **Processing Events**: File processing operations and their outcomes
//! - **Security Events**: Authentication, authorization, and audit events
//! - **System Events**: Infrastructure and operational events
//!
//! ### Event Structure
//!
//! All domain events follow a consistent structure:
//!
//! - **Event ID**: Unique identifier for the event instance
//! - **Event Type**: Categorization of the event for routing and handling
//! - **Timestamp**: When the event occurred (immutable)
//! - **Payload**: Event-specific data and context
//! - **Metadata**: Additional information for processing and routing
//!
//! ### Event Sourcing Integration
//!
//! Events support event sourcing patterns through:
//!
//! - **Immutable Events**: Once created, events cannot be modified
//! - **Ordered Streams**: Events maintain chronological order for replay
//! - **State Reconstruction**: Aggregates can be rebuilt from event history
//! - **Snapshot Support**: Periodic snapshots optimize event replay performance
//!
//! ## Core Components
//!
//! ### Generic Event Framework
//!
//! The `generic_event` module provides:
//!
//! - **DomainEvent Trait**: Common interface for all domain events
//! - **Event Categories**: Classification system for event types
//! - **Event Payload**: Structured data containers for event information
//! - **Generic Event Types**: Reusable event structures for common patterns
//!
//! ### Pipeline-Specific Events
//!
//! The `pipeline_events` module contains:
//!
//! - **Pipeline Lifecycle Events**: Creation, updates, and deletion
//! - **Processing Events**: Start, progress, completion, and failure events
//! - **Configuration Events**: Stage modifications and parameter changes
//! - **Metrics Events**: Performance and monitoring data
//!
//! ## Usage Patterns
//!
//! ### Publishing Domain Events
//!
//!
//!
//! ### Event Handling and Reactions
//!
//!
//!
//! ### Event Sourcing Reconstruction
//!
//!
//!
//! ### Event Store Integration
//!
//!
//!
//! ## Event Categories
//!
//! ### Pipeline Events
//!
//! Events related to pipeline lifecycle:
//!
//! - `PipelineCreated`: New pipeline created with configuration
//! - `PipelineUpdated`: Pipeline configuration or stages modified
//! - `PipelineDeleted`: Pipeline removed from system
//! - `PipelineValidated`: Pipeline validation completed
//!
//! ### Processing Events
//!
//! Events related to file processing operations:
//!
//! - `ProcessingStarted`: File processing operation initiated
//! - `ProcessingCompleted`: Processing finished successfully
//! - `ProcessingFailed`: Processing encountered errors
//! - `ProcessingProgressUpdated`: Progress information updated
//!
//! ### Security Events
//!
//! Events related to security and access control:
//!
//! - `SecurityContextCreated`: New security context established
//! - `PermissionGranted`: Access permission granted
//! - `PermissionDenied`: Access permission denied
//! - `AuditEventRecorded`: Security audit event logged
//!
//! ### System Events
//!
//! Events related to system operations:
//!
//! - `SystemStarted`: System initialization completed
//! - `SystemShutdown`: System shutdown initiated
//! - `ConfigurationChanged`: System configuration updated
//! - `HealthCheckCompleted`: System health check performed
//!
//! ## Best Practices
//!
//! ### Event Design
//!
//! - **Immutable Events**: Events should never be modified after creation
//! - **Rich Information**: Include all relevant context in event payload
//! - **Backward Compatibility**: Design events to support schema evolution
//! - **Clear Naming**: Use descriptive names that indicate what happened
//!
//! ### Event Handling
//!
//! - **Idempotent Handlers**: Event handlers should be safe to run multiple
//!   times
//! - **Error Handling**: Handle event processing failures gracefully
//! - **Ordering**: Consider event ordering requirements for handlers
//! - **Performance**: Design handlers for high-throughput event processing
//!
//! ### Event Storage
//!
//! - **Append-Only**: Events should only be appended, never modified
//! - **Partitioning**: Partition event streams for scalability
//! - **Retention**: Define appropriate event retention policies
//! - **Backup**: Ensure event stores are properly backed up
//!
//! ## Performance Considerations
//!
//! - **Event Size**: Keep event payloads reasonably sized
//! - **Batch Processing**: Process events in batches when possible
//! - **Async Handling**: Use asynchronous event processing for scalability
//! - **Caching**: Cache frequently accessed event data appropriately
//!
//! ## Testing Strategies
//!
//! ### Unit Testing Events
//!
//!
//! ### Integration Testing
//!
//! Test event handling with real event stores and handlers:
//!

pub mod generic_event;
pub mod pipeline_events;

// Re-export generic event types for convenience

pub use pipeline_events::*;
