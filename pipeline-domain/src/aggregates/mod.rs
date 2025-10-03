// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Domain Aggregates
//!
//! This module contains the domain aggregates - complex domain objects that
//! serve as consistency boundaries and transaction roots in the Domain-Driven
//! Design (DDD) architecture. Aggregates encapsulate business logic, maintain
//! invariants, and coordinate changes across multiple related entities.
//!
//! ## Overview
//!
//! Aggregates in this pipeline processing system provide:
//!
//! - **Consistency Boundaries**: Ensure all business rules are enforced within
//!   the aggregate
//! - **Transaction Roots**: Serve as the entry point for all operations within
//!   the boundary
//! - **Event Sourcing**: Generate and apply domain events for state changes
//! - **Concurrency Control**: Manage optimistic locking through version
//!   tracking
//! - **Business Logic Encapsulation**: Contain complex domain operations and
//!   validations
//!
//! ## Domain-Driven Design Principles
//!
//! ### Aggregate Root Pattern
//!
//! Each aggregate has a single root entity that:
//!
//! - **Controls Access**: All external access goes through the aggregate root
//! - **Maintains Identity**: Provides unique identification for the entire
//!   aggregate
//! - **Enforces Invariants**: Validates business rules before allowing state
//!   changes
//! - **Publishes Events**: Generates domain events for significant state
//!   changes
//! - **Manages Lifecycle**: Controls creation, modification, and deletion of
//!   contained entities
//!
//! ### Consistency Boundaries
//!
//! Aggregates define consistency boundaries by:
//!
//! - **Immediate Consistency**: All changes within an aggregate are immediately
//!   consistent
//! - **Eventual Consistency**: Changes across aggregates achieve consistency
//!   eventually
//! - **Transaction Scope**: Each aggregate represents a single transaction
//!   boundary
//! - **Invariant Protection**: Business rules are enforced at the aggregate
//!   level
//!
//! ### Event Sourcing Integration
//!
//! Aggregates support event sourcing patterns through:
//!
//! - **Event Generation**: Create domain events for all state changes
//! - **Event Application**: Apply historical events to reconstruct state
//! - **State Reconstruction**: Rebuild aggregate state from event streams
//! - **Optimistic Concurrency**: Use version numbers to detect concurrent
//!   modifications
//!
//! ## Available Aggregates
//!
//! ### PipelineAggregate
//!
//! The `PipelineAggregate` is the primary aggregate root that manages:
//!
//! - **Pipeline Configuration**: Core pipeline settings and stage definitions
//! - **Processing Coordination**: Active file processing operations and their
//!   contexts
//! - **Event Management**: Domain events for pipeline lifecycle and processing
//!   operations
//! - **State Validation**: Business rule enforcement for all pipeline
//!   operations
//!
//! ## Usage Patterns
//!
//! ### Creating New Aggregates
//!
//!
//!
//! ### Event Sourcing Reconstruction
//!
//!
//!
//! ### Repository Integration
//!
//!
//!
//! ## Best Practices
//!
//! ### Aggregate Design
//!
//! - **Keep Aggregates Small**: Include only entities that must be consistent
//!   together
//! - **Single Responsibility**: Each aggregate should have a focused business
//!   purpose
//! - **Avoid Deep Hierarchies**: Limit the depth of entity relationships within
//!   aggregates
//! - **Event-Driven Communication**: Use domain events for inter-aggregate
//!   communication
//!
//! ### Transaction Management
//!
//! - **One Aggregate Per Transaction**: Modify only one aggregate per
//!   transaction
//! - **Eventual Consistency**: Accept eventual consistency between aggregates
//! - **Optimistic Concurrency**: Use version numbers to detect concurrent
//!   modifications
//! - **Event Persistence**: Persist events atomically with aggregate state
//!
//! ### Performance Considerations
//!
//! - **Lazy Loading**: Load aggregate data only when needed
//! - **Event Snapshots**: Use snapshots to avoid replaying long event streams
//! - **Caching Strategy**: Cache frequently accessed aggregates appropriately
//! - **Batch Operations**: Group related operations to minimize transaction
//!   overhead
//!
//! ## Error Handling
//!
//! Aggregates provide comprehensive error handling:
//!
//! - **Validation Errors**: Business rule violations are caught and reported
//! - **Concurrency Conflicts**: Version conflicts are detected and handled
//! - **State Consistency**: Invalid state transitions are prevented
//! - **Event Application**: Event replay failures are handled gracefully
//!
//! ## Testing Strategies
//!
//! ### Unit Testing
//!
//! Test aggregates in isolation:
//!
//!
//! ### Integration Testing
//!
//! Test aggregates with repositories and event stores:
//!

pub mod pipeline_aggregate;

pub use pipeline_aggregate::PipelineAggregate;
