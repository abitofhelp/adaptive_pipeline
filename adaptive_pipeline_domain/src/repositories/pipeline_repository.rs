// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Pipeline Repository Interface
//!
//! This module defines the repository pattern interface for pipeline
//! persistence, providing an abstraction layer between the domain and
//! infrastructure layers.
//!
//! ## Overview
//!
//! The `PipelineRepository` trait defines the contract for pipeline data
//! persistence operations. This abstraction enables:
//!
//! - **Separation of Concerns**: Domain logic independent of storage technology
//! - **Testability**: Easy mocking and testing with in-memory implementations
//! - **Flexibility**: Support for different storage backends (SQL, NoSQL, etc.)
//! - **Consistency**: Standardized data access patterns across the application
//!
//! ## Repository Pattern Benefits
//!
//! ### Domain Independence
//! The repository pattern keeps domain logic free from infrastructure concerns:
//! - Domain entities don't know about database schemas
//! - Business rules are not coupled to persistence technology
//! - Clean separation enables better testing and maintenance
//!
//! ### Implementation Flexibility
//! Different storage technologies can be used:
//! - SQL databases (PostgreSQL, MySQL, SQLite)
//! - NoSQL databases (MongoDB, DynamoDB)
//! - In-memory storage for testing
//! - File-based storage for simple deployments
//!
//! ## Usage Examples
//!
//! ### Basic CRUD Operations
//!
//!
//! ### Querying and Listing
//!
//!
//! ### Archive Management
//!
//!
//! ## Implementation Guidelines
//!
//! ### Error Handling
//! Repository implementations should:
//! - Return `PipelineError` for all error conditions
//! - Handle database connection failures gracefully
//! - Provide meaningful error messages for debugging
//! - Log errors appropriately for monitoring
//!
//! ### Transaction Support
//! Implementations should consider:
//! - Atomic operations for data consistency
//! - Transaction rollback on failures
//! - Isolation levels for concurrent access
//! - Deadlock detection and retry logic
//!
//! ### Performance Considerations
//! - **Indexing**: Ensure proper database indexes for queries
//! - **Caching**: Implement caching for frequently accessed data
//! - **Connection Pooling**: Use connection pools for database efficiency
//! - **Batch Operations**: Support batch saves/updates when possible
//!
//! ### Security
//! Repository implementations must:
//! - Validate all input parameters
//! - Use parameterized queries to prevent SQL injection
//! - Implement proper access controls
//! - Audit sensitive operations
//!
//! ## Testing Strategies
//!
//! ### Unit Testing
// # use async_trait::async_trait;
// In-memory implementation for testing
// struct InMemoryPipelineRepository {
//     pipelines: std::sync::Mutex<HashMap<String, String>>,
// }
//
// #[async_trait]
// impl PipelineRepository for InMemoryPipelineRepository {
//     fn save(&self, pipeline: &String) -> Result<(), String> {
//         let mut pipelines = self.pipelines.lock().await.unwrap();
//         pipelines.insert(pipeline.id().clone(), pipeline.clone());
//     }
//
//     fn find_by_id(&self, id: String) -> Result<Option<String>, String> {
//         let pipelines = self.pipelines.lock().await.unwrap();
//         Ok(pipelines.get(&id).cloned())
//     }
//
//     // ... implement other methods ...
// #   fn find_by_name(&self, _name: &str) -> Result<Option<String>, String> {
// Ok(None) } #   fn list_all(&self) -> Result<Vec<String>, String> { Ok(vec![])
// } #   fn find_all(&self) -> Result<Vec<String>, String> { Ok(vec![]) }
// #   fn list_paginated(&self, _offset: usize, _limit: usize) ->
// Result<Vec<String>, String> { Ok(vec![]) } #   fn update(&self, _pipeline:
// &String) -> Result<(), String> { Ok(()) } #   fn delete(&self, _id: String)
// -> Result<bool, String> { Ok(true) } #   fn exists(&self, _id: String) ->
// Result<bool, String> { Ok(false) } #   fn count(&self) -> Result<usize,
// String> { Ok(0) } #   fn find_by_config(&self, _key: &str, _value: &str) ->
// Result<Vec<String>, String> { Ok(vec![]) } #   fn archive(&self, _id: String)
// -> Result<bool, String> { Ok(true) } #   fn restore(&self, _id: String) ->
// Result<bool, String> { Ok(true) } #   fn list_archived(&self) ->
// Result<Vec<String>, String> { Ok(vec![]) } }
// ```
//
// ### Integration Testing
// Test with real database implementations:
// - Verify data persistence across application restarts
// - Test concurrent access scenarios
// - Validate transaction behavior
// - Performance testing with large datasets
//
// ## Concrete Implementations
//
// The infrastructure layer provides concrete implementations:
// - `SqlitePipelineRepository`: SQLite-based implementation
// - `PostgresPipelineRepository`: PostgreSQL implementation
// - `InMemoryPipelineRepository`: Testing implementation
//
// Each implementation handles storage-specific concerns while
// maintaining the same interface contract.

use crate::entities::Pipeline;
use crate::value_objects::PipelineId;
use crate::PipelineError;
use async_trait::async_trait;

/// Repository interface for pipeline persistence operations
///
/// This trait defines the contract for pipeline data access operations,
/// providing an abstraction layer between domain logic and storage technology.
/// All methods are asynchronous to support non-blocking I/O operations.
///
/// # Design Principles
///
/// - **Async-First**: All operations are asynchronous for scalability
/// - **Error Handling**: Comprehensive error handling with `PipelineError`
/// - **Type Safety**: Strong typing with `PipelineId` and `Pipeline` entities
/// - **Flexibility**: Support for different storage implementations
///
/// # Thread Safety
///
/// Implementations must be thread-safe (`Send + Sync`) to support
/// concurrent access in multi-threaded environments.
#[async_trait]
pub trait PipelineRepository: Send + Sync {
    /// Saves a pipeline
    async fn save(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;

    /// Finds a pipeline by ID
    async fn find_by_id(&self, id: PipelineId) -> Result<Option<Pipeline>, PipelineError>;

    /// Finds a pipeline by name
    async fn find_by_name(&self, name: &str) -> Result<Option<Pipeline>, PipelineError>;

    /// Lists all pipelines
    async fn list_all(&self) -> Result<Vec<Pipeline>, PipelineError>;

    /// Finds all pipelines (alias for list_all)
    async fn find_all(&self) -> Result<Vec<Pipeline>, PipelineError>;

    /// Lists pipelines with pagination
    async fn list_paginated(
        &self,
        offset: usize,
        limit: usize
    ) -> Result<Vec<Pipeline>, PipelineError>;

    /// Updates a pipeline
    async fn update(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;

    /// Deletes a pipeline by ID
    async fn delete(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Checks if a pipeline exists
    async fn exists(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Counts total pipelines
    async fn count(&self) -> Result<usize, PipelineError>;

    /// Finds pipelines by configuration parameter
    async fn find_by_config(&self, key: &str, value: &str) -> Result<Vec<Pipeline>, PipelineError>;

    /// Archives a pipeline (soft delete)
    async fn archive(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Restores an archived pipeline
    async fn restore(&self, id: PipelineId) -> Result<bool, PipelineError>;

    /// Lists archived pipelines
    async fn list_archived(&self) -> Result<Vec<Pipeline>, PipelineError>;
}
