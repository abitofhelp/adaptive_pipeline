// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Use Cases
//!
//! This module implements the use cases (business workflows) of the adaptive
//! pipeline system. Use cases represent the specific business operations that
//! users can perform, orchestrating domain services and entities to accomplish
//! business goals.
//!
//! ## Overview
//!
//! Use cases follow the Clean Architecture pattern and serve as:
//!
//! - **Entry Points**: Primary entry points for business operations
//! - **Orchestrators**: Coordinate multiple domain services and repositories
//! - **Transaction Boundaries**: Define consistent transaction scopes
//! - **Business Rules**: Implement application-specific business rules
//! - **Error Handlers**: Translate domain errors into application responses
//!
//! ## Use Case Pattern
//!
//! Each use case follows a consistent pattern:
//!
//!
//! ## Core Use Cases
//!
//! ### Process File Use Case
//!
//! Process a file through the adaptive pipeline:
//!
//!
//! ### Create Pipeline Use Case
//!
//! Create a new pipeline configuration:
//!
//!
//! ### List Pipelines Use Case
//!
//! Retrieve a paginated list of pipelines:
//!
//!
//! ### Restore File Use Case
//!
//! Restore a processed file to its original format:
//!
//!
//! ## Use Case Composition
//!
//! Use cases can compose other use cases for complex workflows:
//!
//!
//! ## Transaction Management
//!
//! Use cases define transaction boundaries:
//!
//!
//! ## Error Handling
//!
//! Use cases handle and translate errors:
//!
//!
//! ## Testing
//!
//! Use cases are tested with mocked dependencies:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[tokio::test]
//!     async fn test_process_file_use_case() {
//!         // Arrange: Create mocked dependencies
//!         // Act: Execute use case
//!         // Assert: Verify output and side effects
//!     }
//!
//!     #[tokio::test]
//!     async fn test_error_handling() {
//!         // Test error scenarios
//!     }
//! }
//! ```

// Use cases module - each CLI command has a corresponding use case
pub mod benchmark_system;
pub mod compare_files;
pub mod create_pipeline;
pub mod delete_pipeline;
pub mod list_pipelines;
pub mod process_file;
pub mod restore_file;
pub mod show_pipeline;
pub mod validate_config;
pub mod validate_file;

// Re-export use cases for convenient access
pub use benchmark_system::BenchmarkSystemUseCase;
pub use compare_files::CompareFilesUseCase;
pub use create_pipeline::CreatePipelineUseCase;
pub use delete_pipeline::DeletePipelineUseCase;
pub use list_pipelines::ListPipelinesUseCase;
pub use process_file::{ProcessFileConfig, ProcessFileUseCase};
pub use restore_file::create_restoration_pipeline;
pub use show_pipeline::ShowPipelineUseCase;
pub use validate_config::ValidateConfigUseCase;
pub use validate_file::ValidateFileUseCase;
