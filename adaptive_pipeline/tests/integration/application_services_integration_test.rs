// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Application Services Integration Tests
//!
//! Framework-based integration tests for application layer services using our
//! validated testing framework. Tests end-to-end workflows with real
//! infrastructure services.
//!
//! ## Test Coverage
//!
//! - Full file processing and restoration workflows
//! - Real infrastructure service integration
//! - End-to-end pipeline execution
//! - Cross-service communication
//! - Error handling in realistic scenarios
//!
//! ## Test Framework
//!
//! Uses our validated testing framework with:
//! - Real infrastructure services
//! - Actual file I/O operations
//! - Complete pipeline workflows
//! - Performance measurement
//! - Comprehensive validation

use sha2::Digest;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

use adaptive_pipeline_domain::entities::pipeline::Pipeline;
use adaptive_pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
use adaptive_pipeline_domain::value_objects::binary_file_format::FileHeader;
use adaptive_pipeline_domain::PipelineError;

// ============================================================================
// APPLICATION SERVICES INTEGRATION TEST FRAMEWORK
// ============================================================================

/// Integration test framework for application services.
///
/// Provides structured testing patterns using our validated framework
/// for comprehensive end-to-end application service validation with
/// real infrastructure services.
#[allow(dead_code)]
struct ApplicationServicesIntegrationTestFramework;

#[allow(dead_code)]
impl ApplicationServicesIntegrationTestFramework {
    /// Creates a real .adapipe file using the actual FileHeader format.
    ///
    /// This method creates authentic .adapipe files with the proper format
    /// expected by the restoration service.
    async fn create_real_adapipe_file(
        temp_dir: &TempDir,
        filename: &str,
        original_data: &[u8],
    ) -> Result<PathBuf, PipelineError> {
        let input_path = temp_dir.path().join(format!("{}.txt", filename));
        let adapipe_path = temp_dir.path().join(format!("{}.adapipe", filename));

        // Write original data to input file
        fs::write(&input_path, original_data)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to create input file: {}", e)))?;

        // Calculate checksum of original data
        let original_checksum = format!("{:x}", sha2::Sha256::digest(original_data));

        // Create FileHeader with proper format
        let header = FileHeader::new(
            input_path.to_string_lossy().to_string(),
            original_data.len() as u64,
            original_checksum.clone(),
        )
        .with_output_checksum(original_checksum)
        .with_chunk_info(original_data.len() as u32, 1)
        .with_pipeline_id("integration-test-pipeline".to_string());

        // Create .adapipe file with proper format:
        // [ORIGINAL_DATA][FOOTER]
        // Footer format: [JSON_HEADER][HEADER_LENGTH][FORMAT_VERSION][MAGIC_BYTES]
        let mut file_content = Vec::new();

        // Add the original file data (pass-through for testing)
        file_content.extend_from_slice(original_data);

        // Add the footer with FileHeader
        let footer_bytes = header.to_footer_bytes()?;
        file_content.extend_from_slice(&footer_bytes);

        fs::write(&adapipe_path, file_content)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to create .adapipe file: {}", e)))?;

        Ok(adapipe_path)
    }

    /// Creates a test pipeline with realistic stages.
    fn create_test_pipeline(name: &str) -> Result<Pipeline, PipelineError> {
        let stages = vec![
            PipelineStage::new(
                "compress".to_string(),
                StageType::Compression,
                StageConfiguration::default(),
                1,
            )?,
            PipelineStage::new(
                "encrypt".to_string(),
                StageType::Encryption,
                StageConfiguration::default(),
                2,
            )?,
        ];

        Pipeline::new(name.to_string(), stages)
    }

    // Note: FileRestorationApplicationService removed - restoration now handled via
    // use cases

    /// Measures operation performance and logs execution time.
    fn measure_operation<F, R>(operation: F, operation_name: &str) -> R
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = operation();
        let duration = start.elapsed();
        println!("⏱️  {} completed in {:?}", operation_name, duration);
        result
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

// Note: FileRestorationApplicationService integration tests were removed during
// architecture refactoring. File restoration is now tested via:
// - CLI integration tests (end-to-end command flow)
// - Use case unit tests (business logic validation)
// - Benchmark suite (performance characteristics)
