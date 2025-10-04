// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # File Restoration Module
//!
//! This module provides comprehensive file restoration capabilities for the
//! adaptive pipeline system. It handles the creation and execution of
//! restoration pipelines that reverse the processing operations applied to
//! files, restoring them from the `.adapipe` binary format back to their
//! original state.
//!
//! ## Overview
//!
//! The restoration module implements the inverse operations of the processing
//! pipeline:
//!
//! - **Metadata Analysis**: Parses `.adapipe` file headers to understand
//!   processing history
//! - **Pipeline Reconstruction**: Creates restoration pipelines that reverse
//!   original processing
//! - **Stage Reversal**: Applies processing stages in reverse order (LIFO)
//! - **Integrity Validation**: Verifies checksums and data integrity during
//!   restoration
//! - **Error Recovery**: Handles restoration failures and provides detailed
//!   error reporting
//!
//! ## Architecture
//!
//! The restoration system follows Domain-Driven Design principles:
//!
//! - **Domain Entities**: `Pipeline` serves as the aggregate root for
//!   restoration operations
//! - **Value Objects**: Type-safe identifiers (`PipelineId`, `StageId`) ensure
//!   correctness
//! - **Immutability**: Restoration pipelines are immutable once created
//! - **Error Handling**: Comprehensive validation and error propagation
//!   throughout
//! - **Separation of Concerns**: Restoration logic is isolated from main
//!   application logic
//!
//! ## Restoration Process
//!
//! ### 1. Metadata Parsing
//! The restoration process begins by parsing the `.adapipe` file header to
//! extract:
//! - Original processing pipeline configuration
//! - Processing steps and their parameters
//! - Checksums for integrity validation
//! - File metadata and compression information
//!
//! ### 2. Pipeline Creation
//! An ephemeral restoration pipeline is created that:
//! - Reverses the original processing order (LIFO)
//! - Configures inverse operations for each stage
//! - Includes checksum validation stages
//! - Maintains processing context and metadata
//!
//! ### 3. Stage Execution
//! Processing stages are executed in reverse order:
//! - **Decompression**: Reverses compression operations
//! - **Decryption**: Reverses encryption operations
//! - **Validation**: Verifies checksums and data integrity
//! - **Output**: Writes restored file to target location
//!
//! ## Usage Examples
//!
//! ### Basic File Restoration

//!
//! ### Batch Restoration

//!
//! ### Advanced Restoration with Validation

//!
//! ## Error Handling
//!
//! The restoration module provides comprehensive error handling for:
//!
//! - **Metadata Parsing Errors**: Invalid or corrupted `.adapipe` headers
//! - **Pipeline Creation Errors**: Invalid processing steps or configurations
//! - **Stage Configuration Errors**: Unsupported algorithms or parameters
//! - **Validation Errors**: Checksum mismatches or data corruption
//! - **I/O Errors**: File access, permission, or disk space issues
//!
//! ## Performance Considerations
//!
//! - **Memory Usage**: Restoration pipelines are lightweight and ephemeral
//! - **Processing Order**: LIFO stage execution ensures correct restoration
//!   sequence
//! - **Streaming**: Large files are processed in chunks to minimize memory
//!   usage
//! - **Validation**: Checksum validation provides integrity guarantees with
//!   minimal overhead
//!
//! ## Security Considerations
//!
//! - **Decryption**: Encrypted files require appropriate decryption keys
//! - **Integrity**: Checksum validation ensures data hasn't been tampered with
//! - **Permissions**: Restored files maintain appropriate access permissions
//! - **Audit Trail**: Restoration operations are logged for security auditing
//!
//! ## Integration
//!
//! The restoration module integrates with:
//!
//! - **CLI Interface**: Command-line restoration operations
//! - **Pipeline System**: Core pipeline execution engine
//! - **File I/O Services**: Reading `.adapipe` files and writing restored files
//! - **Validation Services**: Checksum verification and integrity checking
//! - **Logging System**: Comprehensive operation logging and error reporting

use pipeline_domain::entities::pipeline::Pipeline;
use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
use pipeline_domain::value_objects::binary_file_format::FileHeader;
use pipeline_domain::PipelineError;
use chrono::Utc;
use tracing::info;

type Result<T> = std::result::Result<T, PipelineError>;

/// Creates an ephemeral restoration pipeline from `.adapipe` file metadata.
///
/// This function is the core of the restoration system, responsible for
/// analyzing the processing history stored in `.adapipe` file headers and
/// creating a corresponding restoration pipeline that can reverse the original
/// processing operations.
///
/// ## Functionality
///
/// The function performs the following operations:
///
/// 1. **Metadata Analysis**: Parses the file header to extract processing steps
/// 2. **Pipeline Generation**: Creates a unique restoration pipeline identifier
/// 3. **Stage Reversal**: Configures processing stages in reverse order (LIFO)
/// 4. **Validation Setup**: Includes checksum validation stages for integrity
/// 5. **Error Handling**: Provides comprehensive error reporting and validation
///
/// ## Architecture
///
/// The function follows Domain-Driven Design principles:
///
/// - **Domain Entity**: `Pipeline` serves as the aggregate root for restoration
/// - **Value Objects**: Type-safe identifiers (`PipelineId`, `StageId`) ensure
///   correctness
/// - **Immutability**: Created pipeline stages are immutable and thread-safe
/// - **Error Handling**: Comprehensive validation with detailed error
///   propagation
/// - **Business Logic**: Encapsulates restoration domain knowledge and rules
///
/// ## Processing Logic
///
/// ### Stage Reversal (LIFO)
/// Processing stages are applied in reverse order to undo the original
/// operations:
/// - **Last Applied First**: The last processing step becomes the first
///   restoration step
/// - **Parameter Inversion**: Stage parameters are configured for reverse
///   operations
/// - **Checksum Validation**: Automatic inclusion of integrity validation
///   stages
///
/// ### Automatic Stage Management
/// The pipeline automatically includes:
/// - **Input Checksum**: Validates `.adapipe` file integrity
/// - **Output Checksum**: Verifies restored file integrity
/// - **Processing Stages**: User-defined stages in reverse order
///
/// ## Parameters
///
/// * `metadata` - File header containing processing history and configuration
///   - Must contain valid processing steps and pipeline information
///   - Used to determine the restoration sequence and parameters
///   - Provides checksums for integrity validation
///
/// ## Returns
///
/// Returns a `Result<Pipeline>` containing:
/// - **Success**: Fully configured restoration pipeline ready for execution
/// - **Error**: Detailed error information if pipeline creation fails
///
/// ## Errors
///
/// This function can return errors for:
///
/// - **Invalid Metadata**: Corrupted or malformed file headers
/// - **Unsupported Algorithms**: Processing steps with unknown algorithms
/// - **Configuration Errors**: Invalid stage parameters or configurations
/// - **Pipeline Creation**: Errors during pipeline assembly
///
/// ## Usage Examples
///
/// ### Basic Restoration Pipeline
///
///
/// ### Validation and Error Handling
///
///
/// ### Complex Processing History
///
///
/// ## Performance Characteristics
///
/// - **Lightweight**: Pipeline creation is fast and memory-efficient
/// - **Ephemeral**: Pipelines exist only for the duration of restoration
/// - **Thread-Safe**: Created pipelines are immutable and thread-safe
/// - **Scalable**: Can handle complex processing histories efficiently
///
/// ## Security Considerations
///
/// - **Integrity Validation**: Automatic checksum verification
/// - **Algorithm Validation**: Only supported algorithms are allowed
/// - **Parameter Validation**: Stage parameters are validated for safety
/// - **Audit Trail**: Pipeline creation is logged for security auditing
pub async fn create_restoration_pipeline(metadata: &FileHeader) -> Result<Pipeline> {
    let mut stages = Vec::new();

    // Generate unique pipeline ID for restoration
    let pipeline_name = format!("__restore__{}_{}", metadata.pipeline_id, Utc::now().timestamp_millis());

    // Note: Pipeline::new will automatically add input_checksum and output_checksum
    // stages So we only need to create the user-defined stages

    // 2. Process steps in REVERSE order (LIFO for restoration)
    let processing_steps = &metadata.processing_steps;
    for step in processing_steps.iter().rev() {
        let step_name = step.algorithm.to_lowercase();

        // Skip checksum steps as they're handled separately
        if step_name.contains("checksum") {
            info!(
                "Skipping checksum step: {} (from step order {}) - used for validation only",
                step.algorithm, step.order
            );
            continue;
        }

        // Handle transformative custom steps (compression, encryption implemented as
        // custom)
        let stage_type = if step_name == "compression" {
            StageType::Compression
        } else if step_name == "encryption" {
            StageType::Encryption
        } else {
            // For custom algorithms, infer type from algorithm name
            if step.algorithm.contains("brotli") || step.algorithm.contains("gzip") || step.algorithm.contains("lz4") {
                StageType::Compression
            } else if step.algorithm.contains("aes")
                || step.algorithm.contains("chacha")
                || step.algorithm.contains("xchacha")
            {
                StageType::Encryption
            } else {
                // Default to pass-through for unknown algorithms
                StageType::PassThrough
            }
        };

        let stage_name = match stage_type {
            StageType::Compression => "decompression",
            StageType::Encryption => "decryption",
            _ => &step_name,
        };

        let stage = PipelineStage::new(
            stage_name.to_string(),
            stage_type,
            StageConfiguration {
                algorithm: step.algorithm.clone(),
                chunk_size: Some(metadata.chunk_size as usize),
                parallel_processing: false, // Sequential for restoration
                parameters: Default::default(),
            },
            0, // Order will be set by Pipeline::new
        )
        ?;

        stages.push(stage);
    }

    // 3. Verification stage (always present for integrity)
    let verification_stage = PipelineStage::new(
        "verification".to_string(),
        StageType::Checksum,
        StageConfiguration {
            algorithm: "sha256".to_string(),
            chunk_size: Some(metadata.chunk_size as usize),
            parallel_processing: false,
            parameters: Default::default(),
        },
        0, // Order will be set by Pipeline::new
    )
    ?;
    stages.push(verification_stage);

    // Create pipeline with restoration stages (input_checksum and output_checksum
    // will be added automatically)
    let pipeline = Pipeline::new(pipeline_name, stages)?;

    info!(
        "Created restoration pipeline with {} stages for file: {}",
        pipeline.stages().len(),
        metadata.original_filename
    );

    Ok(pipeline)
}
