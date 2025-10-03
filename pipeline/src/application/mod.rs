// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Application Layer
//!
//! The application layer orchestrates business workflows and coordinates
//! between the domain layer and external systems. It implements the use cases
//! of the system and serves as the entry point for all business operations.
//!
//! ## Architecture
//!
//! The application layer follows the Clean Architecture pattern and acts as a
//! coordination layer that:
//!
//! - Receives requests from the interface layer
//! - Coordinates domain services and entities
//! - Manages transactions and cross-cutting concerns
//! - Transforms data between layers
//! - Handles application-specific business rules
//!
//! ## Module Structure
//!
//! ```text
//! application/
//! ├── commands/     # Command objects representing user intentions
//! ├── handlers/     # Command and query handlers
//! ├── queries/      # Query objects for data retrieval
//! └── services/     # Application services coordinating workflows
//! ```
//!
//! ## Commands
//!
//! Commands represent user intentions and system operations. They are immutable
//! data structures that encapsulate all the information needed to perform an
//! action.
//!
//! **Characteristics:**
//! - Immutable data structures
//! - Self-validating
//! - Express user intent
//! - Contain all necessary parameters
//!
//! **Example:**
//!
//!
//! ## Queries
//!
//! Queries represent requests for data retrieval. They specify what data is
//! needed and any filtering or sorting criteria.
//!
//! **Characteristics:**
//! - Read-only operations
//! - Specify data requirements
//! - Support filtering and pagination
//! - Return DTOs or view models
//!
//! **Example:**
//!
//!
//! ## Handlers
//!
//! Handlers contain the application logic for processing commands and queries.
//! They coordinate between domain services and manage the flow of operations.
//!
//! **Characteristics:**
//! - Stateless operations
//! - Coordinate domain services
//! - Manage transactions
//! - Handle cross-cutting concerns
//!
//! **Example:**
//!
//!
//! ## Application Services
//!
//! Application services implement complex workflows that span multiple domain
//! services or require coordination with external systems.
//!
//! **Characteristics:**
//! - Orchestrate complex workflows
//! - Manage external dependencies
//! - Handle application-specific logic
//! - Provide transaction boundaries
//!
//! **Example:**
//!
//!
//! ## Design Principles
//!
//! ### Dependency Inversion
//! The application layer depends on domain abstractions, not concrete
//! implementations. All external dependencies are injected through interfaces.
//!
//! ### Single Responsibility
//! Each application service, handler, and command has a single, well-defined
//! purpose.
//!
//! ### Separation of Concerns
//! Commands handle data, handlers handle logic, services handle coordination.
//!
//! ### Testability
//! All components are designed to be easily testable with dependency injection
//! and clear interfaces.
//!
//! ## Error Handling
//!
//! The application layer handles errors from the domain layer and translates
//! them into appropriate responses for the interface layer:
//!
//!
//! ## Testing Strategy
//!
//! Application layer components are tested with:
//!
//! - **Unit Tests**: Test individual handlers and services with mocked
//!   dependencies
//! - **Integration Tests**: Test complete workflows with real implementations
//! - **Contract Tests**: Verify interfaces between layers

pub mod commands;
pub mod services;
pub mod use_cases;
pub mod utilities;
