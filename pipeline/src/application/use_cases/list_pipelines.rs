// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # List Pipelines Use Case
//!
//! This module implements the use case for listing all available pipelines in
//! the system. It retrieves pipeline metadata from the repository and presents
//! it in a user-friendly format.
//!
//! ## Overview
//!
//! The List Pipelines use case provides:
//!
//! - **Pipeline Discovery**: Retrieve all active pipelines from the repository
//! - **Summary Information**: Display key metadata for each pipeline
//! - **User-Friendly Output**: Format pipeline information for CLI display
//! - **Error Handling**: Handle repository access failures gracefully
//!
//! ## Architecture
//!
//! Following Clean Architecture and Domain-Driven Design principles:
//!
//! - **Use Case Layer**: Orchestrates the listing workflow
//! - **Repository Pattern**: Delegates data access to repository interface
//! - **Dependency Inversion**: Depends on abstractions, not implementations
//! - **Single Responsibility**: Focused solely on listing pipelines
//!
//! ## Business Rules
//!
//! - Only active (non-archived) pipelines are displayed
//! - Pipelines are ordered by creation date (newest first)
//! - Empty pipeline list is handled with helpful user message
//! - All pipeline metadata is displayed: ID, name, status, stages, timestamps
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use pipeline::application::use_cases::ListPipelinesUseCase;
//!
//! let use_case = ListPipelinesUseCase::new(pipeline_repository);
//! use_case.execute().await?;
//! ```

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use crate::infrastructure::adapters::repositories::sqlite_pipeline_repository_adapter::SqlitePipelineRepository;

/// Use case for listing all available pipelines.
///
/// This use case retrieves all active pipelines from the repository and
/// displays them in a user-friendly format. It handles empty result sets
/// gracefully and provides helpful messages to guide users.
///
/// ## Responsibilities
///
/// - Query repository for all active pipelines
/// - Format pipeline metadata for display
/// - Handle empty result sets with user guidance
/// - Report errors during repository access
///
/// ## Dependencies
///
/// - **Pipeline Repository**: For retrieving pipeline data
///
/// ## Example
///
/// ```rust,ignore
/// let use_case = ListPipelinesUseCase::new(pipeline_repository);
/// match use_case.execute().await {
///     Ok(()) => println!("Pipelines listed successfully"),
///     Err(e) => eprintln!("Failed to list pipelines: {}", e),
/// }
/// ```
pub struct ListPipelinesUseCase {
    pipeline_repository: Arc<SqlitePipelineRepository>,
}

impl ListPipelinesUseCase {
    /// Creates a new List Pipelines use case.
    ///
    /// # Parameters
    ///
    /// * `pipeline_repository` - Repository for accessing pipeline data
    ///
    /// # Returns
    ///
    /// A new instance of `ListPipelinesUseCase`
    pub fn new(pipeline_repository: Arc<SqlitePipelineRepository>) -> Self {
        Self { pipeline_repository }
    }

    /// Executes the list pipelines use case.
    ///
    /// Retrieves all active pipelines from the repository and displays them
    /// in a formatted list with key metadata for each pipeline.
    ///
    /// ## Output Format
    ///
    /// For each pipeline, displays:
    /// - Pipeline name
    /// - Unique identifier (ULID)
    /// - Current status (Active, Archived, etc.)
    /// - Number of configured stages
    /// - Creation timestamp
    /// - Last update timestamp
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Pipelines listed successfully
    /// - `Err(anyhow::Error)` - Repository access failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Repository connection failures
    /// - Database query errors
    /// - Permission issues accessing pipeline data
    ///
    /// ## Example Output
    ///
    /// ```text
    /// Found 3 pipeline(s):
    ///
    /// Pipeline: compress-encrypt
    ///   ID: 01H2X3Y4Z5A6B7C8D9E0F1G2H3
    ///   Status: Active
    ///   Stages: 4
    ///   Created: 2025-10-05 14:30:00 UTC
    ///   Updated: 2025-10-05 14:30:00 UTC
    ///
    /// Pipeline: backup-pipeline
    ///   ID: 01H2X3Y4Z5A6B7C8D9E0F1G2H4
    ///   Status: Active
    ///   Stages: 2
    ///   Created: 2025-10-04 10:15:00 UTC
    ///   Updated: 2025-10-05 09:22:00 UTC
    /// ```
    pub async fn execute(&self) -> Result<()> {
        info!("Listing available pipelines:");

        // Query all pipelines from repository
        let pipelines = self
            .pipeline_repository
            .list_all()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query pipelines: {}", e))?;

        // Handle empty result set with helpful message
        if pipelines.is_empty() {
            println!("No pipelines found. Use 'pipeline create' to create a new pipeline.");
        } else {
            // Display pipeline summary
            println!("Found {} pipeline(s):", pipelines.len());
            println!();

            for pipeline in pipelines {
                println!("Pipeline: {}", pipeline.name());
                println!("  ID: {}", pipeline.id());
                println!("  Status: {}", pipeline.status());
                println!("  Stages: {}", pipeline.stages().len());
                println!("  Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));
                println!("  Updated: {}", pipeline.updated_at().format("%Y-%m-%d %H:%M:%S UTC"));
                println!();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests for use cases typically use mock repositories
    // Full integration tests should use real repositories in tests/integration/

    #[tokio::test]
    #[ignore] // Requires database setup
    async fn test_list_pipelines_with_real_repository() {
        // This test would require a real database setup
        // For now, marked as ignored
        // See tests/integration/ for full end-to-end tests
    }
}
