// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Infrastructure Repository Layer
//!
//! This module provides concrete implementations of domain repository
//! interfaces, following Domain-Driven Design (DDD) and Clean Architecture
//! principles. It implements the Repository pattern to provide data persistence
//! abstractions while maintaining strict architectural boundaries.
//!
//! ## Architecture Overview
//!
//! The repository layer follows these architectural principles:
//!
//! - **Dependency Inversion**: Implements domain-defined repository interfaces
//! - **Single Responsibility**: Each repository handles one aggregate root
//! - **Interface Segregation**: Focused interfaces for specific data operations
//! - **Clean Architecture**: Strict layer separation and dependency rules
//!
//! ## Module Structure
//!
//!
//! ## Visibility and Access Control
//!
//! This module follows strict visibility constraints to maintain architectural
//! boundaries:
//!
//! ### Public Exports (Domain Integration)
//! - **Domain-specific repositories**: For dependency injection in application
//!   layer
//! - **Repository implementations**: Concrete implementations of domain
//!   interfaces
//!
//! ### Crate-visible Utilities (Infrastructure Sharing)
//! - **Generic repository utilities**: Shared within infrastructure layer
//! - **Database adapters**: Common database interaction patterns
//! - **Implementation helpers**: Utilities for repository implementations
//!
//! ### Private Implementation Details
//! - **Internal data structures**: Hidden from other layers
//! - **Database schemas**: Implementation-specific details
//! - **Connection management**: Internal resource handling
//!
//! ## Repository Implementations
//!
//! ### SQLite Pipeline Repository
//!
//! Primary repository for pipeline data persistence:
//!
//!
//! ### Generic Repository Base
//!
//! Provides common repository functionality:
//!
//!
//! ## Database Integration
//!
//! ### SQLite Integration
//!
//! The repository layer uses SQLite for data persistence:
//!
//! **Features:**
//! - **ACID Transactions**: Ensures data consistency
//! - **Connection Pooling**: Efficient resource management
//! - **Schema Migration**: Automatic database schema updates
//! - **Query Optimization**: Efficient data retrieval patterns
//!
//! **Configuration:**
//!
//!
//! ### Transaction Management
//!
//! Repositories support transactional operations:
//!
//!
//! ## Error Handling
//!
//! Repository operations use domain-specific error types:
//!
//!
//! ## Performance Considerations
//!
//! ### Connection Pooling
//! - **Pool Size**: Configurable connection pool for concurrent access
//! - **Connection Reuse**: Efficient resource utilization
//! - **Timeout Handling**: Proper timeout management for operations
//!
//! ### Query Optimization
//! - **Prepared Statements**: Reusable compiled queries
//! - **Indexing Strategy**: Optimized database indexes
//! - **Batch Operations**: Efficient bulk data operations
//!
//! ### Caching Strategy
//! - **Entity Caching**: In-memory caching for frequently accessed entities
//! - **Query Result Caching**: Cache results of expensive queries
//! - **Cache Invalidation**: Proper cache invalidation on updates
//!
//! ## Testing Support
//!
//! ### Mock Repositories
//!
//! ### Integration Testing
//! - **Test Database**: Separate database for testing
//! - **Data Fixtures**: Predefined test data sets
//! - **Transaction Rollback**: Clean state between tests
//!
//! ## Security Considerations
//!
//! - **SQL Injection Prevention**: Parameterized queries only
//! - **Access Control**: Repository-level permission checks
//! - **Data Encryption**: Sensitive data encryption at rest
//! - **Audit Logging**: Track all data access and modifications
//!
//! ## Migration and Schema Management
//!
//! - **Schema Versioning**: Track database schema versions
//! - **Migration Scripts**: Automated schema updates
//! - **Backward Compatibility**: Support for schema evolution
//! - **Data Migration**: Safe data transformation during updates
// DOMAIN-SPECIFIC REPOSITORIES (PUBLIC - for dependency injection)
pub mod sqlite_pipeline_repository;

// INFRASTRUCTURE UTILITIES (CRATE-VISIBLE - internal infrastructure sharing)
pub(crate) mod generic_repository;
pub(crate) mod sqlite_repository;
pub(crate) mod sqlite_repository_adapter;
pub(crate) mod stage_executor;

// CLEAN ARCHITECTURE EXPORTS - Only domain-specific implementations
// Following DIP: Export concrete implementations for dependency injection

// INFRASTRUCTURE UTILITIES - Crate-visible for internal use
// Note: Repository utilities removed to eliminate unused import warnings
// These can be re-added when actually needed
