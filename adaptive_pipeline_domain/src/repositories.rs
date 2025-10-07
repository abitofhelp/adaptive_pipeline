// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Domain Repositories
//!
//! This module contains repository interfaces that define the contracts for
//! data persistence and stage execution within the pipeline processing domain.
//!
//! ## Overview
//!
//! The repositories module provides:
//!
//! - **Data Persistence Interfaces**: Abstract contracts for data storage
//! - **Stage Execution Interfaces**: Contracts for processing pipeline stages
//! - **Domain Abstraction**: Clean separation between domain and infrastructure
//! - **Testability**: Interfaces that can be easily mocked for testing
//!
//! ## Repository Pattern
//!
//! The repository pattern provides several benefits:
//!
//! ### Separation of Concerns
//! - Domain logic remains independent of storage technology
//! - Business rules are not coupled to database schemas
//! - Clean architecture boundaries are maintained
//!
//! ### Flexibility
//! - Multiple storage implementations (SQL, NoSQL, in-memory)
//! - Easy switching between different persistence technologies
//! - Support for different deployment scenarios
//!
//! ### Testability
//! - Easy mocking for unit tests
//! - In-memory implementations for integration tests
//! - Isolated testing of domain logic
//!
//! ## Repository Interfaces
//!
//! ### Pipeline Repository
//! Handles persistence of pipeline configurations:
//!
//!
//! ### Stage Executor
//! Handles execution of pipeline stages:
//!
//!
//! ## Implementation Strategy
//!
//! ### Domain Layer (This Module)
//! - Defines repository interfaces
//! - Specifies contracts and behavior
//! - Remains technology-agnostic
//!
//! ### Infrastructure Layer
//! - Provides concrete implementations
//! - Handles specific storage technologies
//! - Manages connection pooling and optimization
//!
//!
//! ## Usage Patterns
//!
//! ### Dependency Injection
//!
//! ### Testing with Mocks
//!
//! ## Best Practices
//!
//! ### Interface Design
//! - Keep interfaces focused and cohesive
//! - Use async methods for I/O operations
//! - Return domain-specific error types
//! - Include comprehensive documentation
//!
//! ### Error Handling
//! - Use specific error types for different failure modes
//! - Provide meaningful error messages
//! - Handle transient failures appropriately
//! - Log errors for debugging and monitoring
//!
//! ### Performance
//! - Design for async/await patterns
//! - Consider batch operations for efficiency
//! - Support pagination for large result sets
//! - Enable connection pooling in implementations
//!
//! ### Security
//! - Validate all input parameters
//! - Implement proper access controls
//! - Audit sensitive operations
//! - Use parameterized queries in implementations

pub mod pipeline_repository;
pub mod stage_executor;

pub use pipeline_repository::PipelineRepository;
pub use stage_executor::StageExecutor;
