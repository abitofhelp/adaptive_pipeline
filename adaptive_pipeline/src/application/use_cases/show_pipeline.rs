// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Show Pipeline Use Case
//!
//! This module implements the use case for displaying detailed information
//! about a specific pipeline. It retrieves complete pipeline metadata including
//! stages, configuration, and processing metrics.
//!
//! ## Overview
//!
//! The Show Pipeline use case provides:
//!
//! - **Detailed Pipeline Information**: Display all metadata for a specific
//!   pipeline
//! - **Stage Breakdown**: Show configuration for each processing stage
//! - **Metrics Display**: Present processing statistics and performance metrics
//! - **Configuration View**: Display pipeline-level configuration parameters
//! - **Error Handling**: Handle missing pipelines with clear error messages
//!
//! ## Architecture
//!
//! Following Clean Architecture and Domain-Driven Design principles:
//!
//! - **Use Case Layer**: Orchestrates the detailed display workflow
//! - **Repository Pattern**: Delegates data access to repository interface
//! - **Dependency Inversion**: Depends on abstractions, not implementations
//! - **Single Responsibility**: Focused solely on displaying pipeline details
//!
//! ## Business Rules
//!
//! - Pipelines are looked up by name (user-friendly identifier)
//! - Missing pipelines return clear error messages
//! - All stage details are displayed with configuration parameters
//! - Pipeline metrics show processing history and statistics
//! - Configuration parameters are displayed if present
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use adaptive_pipeline::application::use_cases::ShowPipelineUseCase;
//!
//! let use_case = ShowPipelineUseCase::new(pipeline_repository);
//! use_case.execute("my-pipeline".to_string()).await?;
//! ```

use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use crate::infrastructure::repositories::sqlite_pipeline::SqlitePipelineRepository;

/// Use case for displaying detailed pipeline information.
///
/// This use case retrieves a specific pipeline by name and displays its
/// complete metadata, including stages, configuration, and processing metrics.
/// It provides comprehensive visibility into pipeline structure and behavior.
///
/// ## Responsibilities
///
/// - Look up pipeline by name in repository
/// - Format detailed pipeline information for display
/// - Display all stage configurations and parameters
/// - Show processing metrics and statistics
/// - Handle missing pipelines with clear error messages
///
/// ## Dependencies
///
/// - **Pipeline Repository**: For retrieving pipeline data
///
/// ## Example
///
/// ```rust,ignore
/// let use_case = ShowPipelineUseCase::new(pipeline_repository);
/// match use_case.execute("compress-encrypt".to_string()).await {
///     Ok(()) => println!("Pipeline details displayed"),
///     Err(e) => eprintln!("Failed to show pipeline: {}", e),
/// }
/// ```
pub struct ShowPipelineUseCase {
    pipeline_repository: Arc<SqlitePipelineRepository>,
}

impl ShowPipelineUseCase {
    /// Creates a new Show Pipeline use case.
    ///
    /// # Parameters
    ///
    /// * `pipeline_repository` - Repository for accessing pipeline data
    ///
    /// # Returns
    ///
    /// A new instance of `ShowPipelineUseCase`
    pub fn new(pipeline_repository: Arc<SqlitePipelineRepository>) -> Self {
        Self { pipeline_repository }
    }

    /// Executes the show pipeline use case.
    ///
    /// Retrieves a specific pipeline by name and displays its complete
    /// metadata, including all stages, configuration parameters, and
    /// processing metrics.
    ///
    /// ## Parameters
    ///
    /// * `pipeline_name` - Name of the pipeline to display
    ///
    /// ## Output Format
    ///
    /// Displays:
    /// - Pipeline metadata (ID, name, status, timestamps)
    /// - Detailed stage information with configurations
    /// - Stage parameters (if present)
    /// - Pipeline-level configuration (if present)
    /// - Processing metrics (bytes, chunks, errors, warnings)
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Pipeline details displayed successfully
    /// - `Err(anyhow::Error)` - Pipeline not found or repository access failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Pipeline not found with given name
    /// - Repository connection failures
    /// - Database query errors
    /// - Permission issues accessing pipeline data
    ///
    /// ## Example Output
    ///
    /// ```text
    /// === Pipeline Details ===
    /// ID: 01H2X3Y4Z5A6B7C8D9E0F1G2H3
    /// Name: compress-encrypt
    /// Status: Active
    /// Created: 2025-10-05 14:30:00 UTC
    /// Updated: 2025-10-05 14:30:00 UTC
    ///
    /// Stages (4):
    ///   1. input_checksum (Checksum)
    ///      Algorithm: sha256
    ///      Enabled: true
    ///      Order: 0
    ///
    ///   2. compression (Compression)
    ///      Algorithm: brotli
    ///      Enabled: true
    ///      Order: 1
    ///      Parameters:
    ///        level: 6
    ///
    ///   3. encryption (Encryption)
    ///      Algorithm: aes256gcm
    ///      Enabled: true
    ///      Order: 2
    ///
    ///   4. output_checksum (Checksum)
    ///      Algorithm: sha256
    ///      Enabled: true
    ///      Order: 3
    ///
    /// Metrics:
    ///   Bytes Processed: 1048576
    ///   Chunks Processed: 16
    ///   Error Count: 0
    ///   Warning Count: 0
    /// ```
    pub async fn execute(&self, pipeline_name: String) -> Result<()> {
        info!("Showing pipeline details: {}", pipeline_name);

        // Find pipeline by name (user-friendly lookup)
        let pipeline = self
            .pipeline_repository
            .find_by_name(&pipeline_name)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
            .ok_or_else(|| anyhow::anyhow!("Pipeline not found: {}", pipeline_name))?;

        // Display pipeline header
        println!("\n=== Pipeline Details ===");
        println!("ID: {}", pipeline.id());
        println!("Name: {}", pipeline.name());
        println!("Status: {}", pipeline.status());
        println!("Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Updated: {}", pipeline.updated_at().format("%Y-%m-%d %H:%M:%S UTC"));

        // Display stages
        println!("\nStages ({}):", pipeline.stages().len());
        for (index, stage) in pipeline.stages().iter().enumerate() {
            println!("  {}. {} ({:?})", index + 1, stage.name(), stage.stage_type());
            println!("     Algorithm: {}", stage.configuration().algorithm);
            println!("     Enabled: {}", stage.is_enabled());
            println!("     Order: {}", stage.order());

            // Display stage parameters if present
            if !stage.configuration().parameters.is_empty() {
                println!("     Parameters:");
                for (key, value) in &stage.configuration().parameters {
                    println!("       {}: {}", key, value);
                }
            }

            // Add spacing between stages
            if index < pipeline.stages().len() - 1 {
                println!();
            }
        }

        // Display pipeline-level configuration if present
        if !pipeline.configuration().is_empty() {
            println!("\nConfiguration:");
            for (key, value) in pipeline.configuration() {
                println!("  {}: {}", key, value);
            }
        }

        // Display processing metrics
        let metrics = pipeline.metrics();
        println!("\nMetrics:");
        println!("  Bytes Processed: {}", metrics.bytes_processed());
        println!("  Chunks Processed: {}", metrics.chunks_processed());
        println!("  Error Count: {}", metrics.error_count());
        println!("  Warning Count: {}", metrics.warning_count());

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // Note: Tests for use cases typically use mock repositories
    // Full integration tests should use real repositories in tests/integration/

    #[tokio::test]
    #[ignore] // Requires database setup
    async fn test_show_pipeline_with_real_repository() {
        // This test would require a real database setup
        // For now, marked as ignored
        // See tests/integration/ for full end-to-end tests
    }

    #[tokio::test]
    #[ignore] // Requires database setup
    async fn test_show_pipeline_not_found() {
        // Test error handling for missing pipeline
        // Requires mock repository setup
    }
}
