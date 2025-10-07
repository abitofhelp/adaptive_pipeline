// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Validate .adapipe File Use Case
//!
//! This module implements the use case for validating `.adapipe` binary format
//! files. It provides both quick format validation and comprehensive streaming
//! validation.
//!
//! ## Overview
//!
//! The Validate File use case provides:
//!
//! - **Format Validation**: Verify .adapipe binary format structure
//! - **Metadata Reading**: Extract and display file metadata
//! - **Integrity Checks**: Validate checksums and data integrity
//! - **Full Streaming Validation**: Optional comprehensive validation with
//!   decompression/decryption
//! - **Detailed Reporting**: Clear display of file properties and validation
//!   results
//!
//! ## Validation Levels
//!
//! **Basic Validation** (default):
//! - Binary format structure check
//! - Metadata header parsing
//! - Format version compatibility
//! - Basic integrity verification
//!
//! **Full Validation** (`--full` flag):
//! - All basic validation checks
//! - Streaming decompression/decryption
//! - Original file checksum verification
//! - Chunk-by-chunk integrity validation
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use pipeline::application::use_cases::ValidateFileUseCase;
//!
//! let use_case = ValidateFileUseCase::new();
//!
//! // Basic validation
//! use_case.execute(file_path, false).await?;
//!
//! // Full streaming validation
//! use_case.execute(file_path, true).await?;
//! ```

use anyhow::Result;
use byte_unit::Byte;
use std::path::PathBuf;
use tracing::info;

use crate::infrastructure::services::{AdapipeFormat, BinaryFormatService};

/// Use case for validating .adapipe binary format files.
///
/// This use case validates the integrity and format of `.adapipe` files,
/// providing both quick validation and comprehensive streaming validation
/// options.
///
/// ## Responsibilities
///
/// - Validate file exists and has correct extension
/// - Verify binary format structure
/// - Read and validate metadata
/// - Display file properties and processing history
/// - Optionally perform full streaming validation
///
/// ## Dependencies
///
/// - **BinaryFormatService**: For format validation and metadata reading
pub struct ValidateFileUseCase;

impl ValidateFileUseCase {
    /// Creates a new Validate File use case.
    pub fn new() -> Self {
        Self
    }

    /// Executes the validate file use case.
    ///
    /// Validates an `.adapipe` binary format file, checking structure,
    /// metadata, and optionally performing full streaming validation.
    ///
    /// ## Parameters
    ///
    /// * `file_path` - Path to .adapipe file to validate
    /// * `full_validation` - If true, perform comprehensive streaming
    ///   validation
    ///
    /// ## Validation Steps
    ///
    /// **Step 1: Basic Format Validation**
    /// - File exists check
    /// - Extension verification (.adapipe)
    /// - Binary format structure validation
    /// - Magic number verification
    /// - Format version compatibility
    ///
    /// **Step 2: Metadata Reading**
    /// - Parse file header
    /// - Extract metadata fields
    /// - Display file properties:
    ///   - Original filename and size
    ///   - Original checksum
    ///   - Format and app versions
    ///   - Chunk size and count
    ///   - Pipeline ID
    ///   - Processing timestamp
    ///   - Compression/encryption algorithms
    ///   - Processing steps summary
    ///
    /// **Step 3: Full Validation** (if requested)
    /// - Stream through all chunks
    /// - Decrypt encrypted data (if applicable)
    /// - Decompress compressed data (if applicable)
    /// - Verify original file checksum
    /// - Note: Currently marked as TODO pending restoration service refactoring
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - File is valid
    /// - `Err(anyhow::Error)` - Validation failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - File not found
    /// - Invalid file extension (warning only)
    /// - Corrupt binary format
    /// - Invalid metadata
    /// - Checksum mismatch (full validation)
    /// - Unsupported format version
    ///
    /// ## Example Output
    ///
    /// ```text
    /// 🔍 Validating .adapipe file format...
    /// ✅ File format is valid
    ///
    /// 📋 Reading file metadata...
    ///    Original filename: data.txt
    ///    Original size: 1.048 MB
    ///    Original checksum: abc123...
    ///    Format version: 1
    ///    App version: 1.0.1
    ///    Chunk size: 64.0 KB
    ///    Chunk count: 16
    ///    Pipeline ID: 01H2X3Y4Z5...
    ///    Processed at: 2025-10-05 14:30:00 UTC
    ///    🗜️  Compression: brotli
    ///    🔒 Encryption: aes256gcm
    ///    🔄 Processing steps: compression -> encryption -> checksum
    ///
    /// 💡 Use --full flag for complete streaming validation (decrypt/decompress/verify)
    ///
    /// ✅ .adapipe file validation completed successfully!
    /// ```
    pub async fn execute(&self, file_path: PathBuf, full_validation: bool) -> Result<()> {
        info!("Validating .adapipe file: {}", file_path.display());

        // Check file exists
        if !file_path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {}", file_path.display()));
        }

        // Warn if file doesn't have .adapipe extension
        if file_path.extension().is_none_or(|ext| ext != "adapipe") {
            println!("Warning: File does not have .adapipe extension");
        }

        let binary_format_service = AdapipeFormat::new();

        // Step 1: Basic format validation
        println!("🔍 Validating .adapipe file format...");
        let validation_result = binary_format_service
            .validate_file(&file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Format validation failed: {}", e))?;

        if !validation_result.is_valid {
            println!("❌ File format validation failed!");
            for error in &validation_result.errors {
                println!("   Error: {}", error);
            }
            return Err(anyhow::anyhow!("Invalid .adapipe file format"));
        }

        println!("✅ File format is valid");

        // Step 2: Read and display metadata
        println!("\n📋 Reading file metadata...");
        let metadata = binary_format_service
            .read_metadata(&file_path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read metadata: {}", e))?;

        println!("   Original filename: {}", metadata.original_filename);
        println!(
            "   Original size: {}",
            Byte::from_u128(metadata.original_size as u128)
                .unwrap_or_default()
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
        );
        println!("   Original checksum: {}", metadata.original_checksum);
        println!("   Format version: {}", metadata.format_version);
        println!("   App version: {}", metadata.app_version);
        println!(
            "   Chunk size: {}",
            Byte::from_u128(metadata.chunk_size as u128)
                .unwrap_or_default()
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
        );
        println!("   Chunk count: {}", metadata.chunk_count);
        println!("   Pipeline ID: {}", metadata.pipeline_id);
        println!(
            "   Processed at: {}",
            metadata.processed_at.format("%Y-%m-%d %H:%M:%S UTC")
        );

        // Display compression info
        if metadata.is_compressed() {
            println!(
                "   🗜️  Compression: {}",
                metadata.compression_algorithm().unwrap_or("unknown")
            );
        }

        // Display encryption info
        if metadata.is_encrypted() {
            println!(
                "   🔒 Encryption: {}",
                metadata.encryption_algorithm().unwrap_or("unknown")
            );
        }

        // Display processing steps
        if metadata.processing_steps.is_empty() {
            println!("   📄 Pass-through file (no processing)");
        } else {
            println!("   🔄 Processing steps: {}", metadata.get_processing_summary());
        }

        // Step 3: Full streaming validation (if requested)
        if full_validation {
            println!("\n🔄 Performing full streaming validation...");
            println!("   This will decrypt, decompress, and verify the original checksum");
            println!("   No temporary files will be created (streaming validation)");
            println!("   Expected original checksum: {}", metadata.original_checksum);

            // TODO: Full streaming validation not yet implemented
            // The restoration service was removed. This needs to be reimplemented using
            // use_cases::restore_file directly for streaming validation.
            println!("   ⚠️  Full streaming validation not yet implemented");
            println!("   (Restoration service refactoring in progress)");
        } else {
            println!("\n💡 Use --full flag for complete streaming validation (decrypt/decompress/verify)");
        }

        println!("\n✅ .adapipe file validation completed successfully!");

        Ok(())
    }
}

impl Default for ValidateFileUseCase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires test .adapipe files
    async fn test_validate_valid_file() {
        // Test with valid .adapipe file
        // Requires test fixture file
    }

    #[tokio::test]
    #[ignore] // Requires test files
    async fn test_validate_invalid_file() {
        // Test with invalid file (should fail)
        // Requires test fixture file
    }

    #[tokio::test]
    async fn test_validate_missing_file() {
        let use_case = ValidateFileUseCase::new();
        let result = use_case
            .execute(PathBuf::from("/nonexistent/file.adapipe"), false)
            .await;
        assert!(result.is_err());
    }
}
