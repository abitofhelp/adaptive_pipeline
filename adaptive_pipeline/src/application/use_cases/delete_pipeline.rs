// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Delete Pipeline Use Case
//!
//! This module implements the use case for deleting pipelines from the system.
//! It includes safety mechanisms such as confirmation prompts and detailed
//! display of the pipeline being deleted.
//!
//! ## Overview
//!
//! The Delete Pipeline use case provides:
//!
//! - **Safe Deletion**: Confirmation prompt to prevent accidental deletions
//! - **Force Option**: Bypass confirmation for automated/scripted usage
//! - **Pipeline Preview**: Display pipeline details before deletion
//! - **Error Handling**: Handle missing pipelines and deletion failures
//! - **User Feedback**: Clear success/cancellation messages
//!
//! ## Architecture
//!
//! Following Clean Architecture and Domain-Driven Design principles:
//!
//! - **Use Case Layer**: Orchestrates the deletion workflow
//! - **Repository Pattern**: Delegates data access to repository interface
//! - **Dependency Inversion**: Depends on abstractions, not implementations
//! - **Single Responsibility**: Focused solely on pipeline deletion
//!
//! ## Business Rules
//!
//! - Pipelines are looked up by name (user-friendly identifier)
//! - Missing pipelines return clear error messages
//! - Interactive mode requires user confirmation (y/yes to proceed)
//! - Force mode bypasses confirmation (for automation)
//! - Deleted pipelines are removed permanently from the repository
//! - Pipeline details are displayed before deletion for verification
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use pipeline::application::use_cases::DeletePipelineUseCase;
//!
//! // Interactive deletion (requires confirmation)
//! let use_case = DeletePipelineUseCase::new(pipeline_repository);
//! use_case.execute("old-pipeline".to_string(), false).await?;
//!
//! // Force deletion (no confirmation)
//! use_case.execute("old-pipeline".to_string(), true).await?;
//! ```

use anyhow::Result;
use std::io::{ self, Write };
use std::sync::Arc;
use tracing::info;

use crate::infrastructure::repositories::sqlite_pipeline::SqlitePipelineRepository;

/// Use case for deleting pipelines from the system.
///
/// This use case handles pipeline deletion with safety mechanisms to prevent
/// accidental data loss. It supports both interactive mode (with confirmation)
/// and force mode (for automation).
///
/// ## Responsibilities
///
/// - Look up pipeline by name in repository
/// - Display pipeline details for verification
/// - Prompt for user confirmation (unless force mode)
/// - Delete pipeline from repository
/// - Provide feedback on success or cancellation
///
/// ## Dependencies
///
/// - **Pipeline Repository**: For retrieving and deleting pipeline data
///
/// ## Example
///
/// ```rust,ignore
/// let use_case = DeletePipelineUseCase::new(pipeline_repository);
///
/// // Interactive mode
/// match use_case.execute("test-pipeline".to_string(), false).await {
///     Ok(()) => println!("Pipeline deleted or cancelled"),
///     Err(e) => eprintln!("Failed to delete pipeline: {}", e),
/// }
/// ```
pub struct DeletePipelineUseCase {
    pipeline_repository: Arc<SqlitePipelineRepository>,
}

impl DeletePipelineUseCase {
    /// Creates a new Delete Pipeline use case.
    ///
    /// # Parameters
    ///
    /// * `pipeline_repository` - Repository for accessing pipeline data
    ///
    /// # Returns
    ///
    /// A new instance of `DeletePipelineUseCase`
    pub fn new(pipeline_repository: Arc<SqlitePipelineRepository>) -> Self {
        Self { pipeline_repository }
    }

    /// Executes the delete pipeline use case.
    ///
    /// Deletes a pipeline from the system with optional confirmation prompt.
    /// In interactive mode, displays pipeline details and requires user
    /// confirmation before deletion. In force mode, deletes immediately without
    /// confirmation.
    ///
    /// ## Parameters
    ///
    /// * `pipeline_name` - Name of the pipeline to delete
    /// * `force` - If true, bypass confirmation prompt (for automation)
    ///
    /// ## Behavior
    ///
    /// **Interactive Mode** (`force = false`):
    /// 1. Look up pipeline by name
    /// 2. Display pipeline details
    /// 3. Prompt user for confirmation (y/yes to proceed)
    /// 4. Delete if confirmed, cancel otherwise
    ///
    /// **Force Mode** (`force = true`):
    /// 1. Look up pipeline by name
    /// 2. Display pipeline details
    /// 3. Delete immediately without confirmation
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Pipeline deleted successfully or deletion cancelled
    /// - `Err(anyhow::Error)` - Pipeline not found or deletion failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Pipeline not found with given name
    /// - Repository connection failures
    /// - Database deletion errors
    /// - I/O errors reading user input (interactive mode)
    ///
    /// ## Example Output (Interactive Mode)
    ///
    /// ```text
    /// === Pipeline to Delete ===
    /// Name: test-pipeline
    /// ID: 01H2X3Y4Z5A6B7C8D9E0F1G2H3
    /// Stages: 4
    /// Created: 2025-10-05 14:30:00 UTC
    ///
    /// Are you sure you want to delete pipeline 'test-pipeline'? [y/N]: y
    /// ✅ Pipeline 'test-pipeline' deleted successfully
    /// ```
    ///
    /// ## Example Output (Force Mode)
    ///
    /// ```text
    /// === Pipeline to Delete ===
    /// Name: test-pipeline
    /// ID: 01H2X3Y4Z5A6B7C8D9E0F1G2H3
    /// Stages: 4
    /// Created: 2025-10-05 14:30:00 UTC
    ///
    /// ✅ Pipeline 'test-pipeline' deleted successfully
    /// ```
    pub async fn execute(&self, pipeline_name: String, force: bool) -> Result<()> {
        info!("Deleting pipeline: {}", pipeline_name);

        // Find pipeline by name first (verify it exists)
        let pipeline = self.pipeline_repository
            .find_by_name(&pipeline_name).await
            .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
            .ok_or_else(|| anyhow::anyhow!("Pipeline '{}' not found", pipeline_name))?;

        // Show pipeline details before deletion for user verification
        println!("\n=== Pipeline to Delete ===");
        println!("Name: {}", pipeline.name());
        println!("ID: {}", pipeline.id());
        println!("Stages: {}", pipeline.stages().len());
        println!("Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));

        // Confirmation prompt unless --force is used
        if !force {
            print!("\nAre you sure you want to delete pipeline '{}'? [y/N]: ", pipeline_name);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            if input != "y" && input != "yes" {
                println!("Pipeline deletion cancelled.");
                return Ok(());
            }
        }

        // Delete the pipeline from repository
        self.pipeline_repository
            .delete(pipeline.id().clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to delete pipeline: {}", e))?;

        println!("✅ Pipeline '{}' deleted successfully", pipeline_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Tests for use cases typically use mock repositories
    // Full integration tests should use real repositories in tests/integration/

    #[tokio::test]
    #[ignore] // Requires database setup and user interaction
    async fn test_delete_pipeline_with_real_repository() {
        // This test would require a real database setup and mock stdin
        // For now, marked as ignored
        // See tests/integration/ for full end-to-end tests
    }

    #[tokio::test]
    #[ignore] // Requires mock repository
    async fn test_delete_pipeline_not_found() {
        // Test error handling for missing pipeline
        // Requires mock repository setup
    }

    #[tokio::test]
    #[ignore] // Requires mock repository and stdin
    async fn test_delete_pipeline_cancelled() {
        // Test cancellation when user says "no"
        // Requires mock repository and stdin
    }
}
