// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Compare Files Use Case
//!
//! This module implements the use case for comparing original files against
//! their `.adapipe` processed counterparts. It verifies file integrity by
//! comparing sizes and checksums.
//!
//! ## Overview
//!
//! The Compare Files use case provides:
//!
//! - **Size Comparison**: Compare file sizes to detect changes
//! - **Checksum Verification**: Calculate and compare SHA-256 checksums
//! - **Metadata Display**: Show processing information from .adapipe file
//! - **Detailed Reporting**: Optional detailed comparison output
//! - **Change Detection**: Identify if files have been modified
//!
//! ## Use Cases
//!
//! - **Integrity Verification**: Verify a file hasn't changed since processing
//! - **Backup Validation**: Confirm backup .adapipe matches current file
//! - **Change Detection**: Determine if file needs reprocessing
//! - **Audit Trail**: Document file state at processing time
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use adaptive_pipeline::application::use_cases::CompareFilesUseCase;
//!
//! let use_case = CompareFilesUseCase::new();
//!
//! // Basic comparison
//! use_case.execute(
//!     PathBuf::from("data.txt"),
//!     PathBuf::from("data.adapipe"),
//!     false,
//! ).await?;
//!
//! // Detailed comparison
//! use_case.execute(
//!     PathBuf::from("data.txt"),
//!     PathBuf::from("data.adapipe"),
//!     true,
//! ).await?;
//! ```

use adaptive_pipeline_domain::value_objects::binary_file_format::FileHeader;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tracing::info;

/// Use case for comparing original files against .adapipe files.
///
/// This use case compares a current file against the metadata stored in
/// its corresponding `.adapipe` file to detect changes and verify integrity.
///
/// ## Responsibilities
///
/// - Validate both files exist
/// - Read .adapipe metadata
/// - Compare file sizes
/// - Calculate and compare SHA-256 checksums
/// - Display comparison results
/// - Provide detailed information if requested
///
/// ## Dependencies
///
/// - **FileHeader**: For parsing .adapipe metadata
/// - **SHA-256**: For checksum calculation
pub struct CompareFilesUseCase;

impl CompareFilesUseCase {
    /// Creates a new Compare Files use case.
    pub fn new() -> Self {
        Self
    }

    /// Executes the compare files use case.
    ///
    /// Compares an original file against its `.adapipe` counterpart by
    /// checking file size and calculating checksums.
    ///
    /// ## Parameters
    ///
    /// * `original` - Path to current/original file
    /// * `adapipe` - Path to corresponding .adapipe file
    /// * `detailed` - If true, show detailed processing information
    ///
    /// ## Comparison Steps
    ///
    /// **Step 1: File Existence**
    /// - Verify both files exist
    /// - Return error if either missing
    ///
    /// **Step 2: Size Comparison**
    /// - Get current file size
    /// - Read expected size from .adapipe metadata
    /// - Compare and report differences
    ///
    /// **Step 3: Checksum Comparison**
    /// - Calculate SHA-256 checksum of current file
    /// - Read expected checksum from .adapipe metadata
    /// - Compare and report match/mismatch
    ///
    /// **Step 4: Detailed Information** (if requested)
    /// - .adapipe creation timestamp
    /// - Pipeline ID
    /// - Chunk count
    /// - Compression algorithm (if used)
    /// - Encryption algorithm (if used)
    /// - Current file modification time
    ///
    /// **Step 5: Summary**
    /// - Overall comparison result
    /// - Suggestion to restore if files differ
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Comparison completed successfully
    /// - `Err(anyhow::Error)` - File access or comparison failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Original file not found
    /// - .adapipe file not found
    /// - Failed to read .adapipe metadata
    /// - Failed to calculate checksum
    /// - File I/O errors
    ///
    /// ## Example Output (Matching Files)
    ///
    /// ```text
    /// 🔍 Reading .adapipe file metadata...
    /// 📊 File Comparison:
    ///    Original file: /path/to/data.txt
    ///    .adapipe file: /path/to/data.adapipe
    ///
    /// 📏 Size Comparison:
    ///    Current file size: 1048576 bytes
    ///    Expected size (from .adapipe): 1048576 bytes
    ///    ✅ Size matches
    ///
    /// 🔐 Checksum Comparison:
    ///    Expected checksum (from .adapipe): abc123...
    ///    🔄 Calculating current file checksum...
    ///    Current file checksum: abc123...
    ///    ✅ Checksums match - files are identical
    ///
    /// 🎯 Comparison Summary:
    ///    ✅ Files are identical - no changes detected
    /// ```
    ///
    /// ## Example Output (Different Files)
    ///
    /// ```text
    /// 📏 Size Comparison:
    ///    Current file size: 1048580 bytes
    ///    Expected size (from .adapipe): 1048576 bytes
    ///    ❌ Size differs by 4 bytes
    ///
    /// 🔐 Checksum Comparison:
    ///    Expected checksum (from .adapipe): abc123...
    ///    Current file checksum: def456...
    ///    ❌ Checksums differ - files are not identical
    ///
    /// 🎯 Comparison Summary:
    ///    ❌ Files differ - changes detected
    ///    💡 Use 'restore' command to restore from .adapipe if needed
    /// ```
    pub async fn execute(&self, original: PathBuf, adapipe: PathBuf, detailed: bool) -> Result<()> {
        info!(
            "Comparing file against .adapipe: {} vs {}",
            original.display(),
            adapipe.display()
        );

        // Validate both files exist
        if !original.exists() {
            return Err(anyhow::anyhow!("Original file does not exist: {}", original.display()));
        }

        if !adapipe.exists() {
            return Err(anyhow::anyhow!(".adapipe file does not exist: {}", adapipe.display()));
        }

        // Read .adapipe metadata
        println!("🔍 Reading .adapipe file metadata...");
        let file_data = std::fs::read(&adapipe)?;
        let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
            .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

        // Get original file info
        let original_metadata = std::fs::metadata(&original)?;
        let original_size = original_metadata.len();

        println!("📊 File Comparison:");
        println!("   Original file: {}", original.display());
        println!("   .adapipe file: {}", adapipe.display());
        println!();

        // Compare file sizes
        println!("📏 Size Comparison:");
        println!("   Current file size: {} bytes", original_size);
        println!("   Expected size (from .adapipe): {} bytes", metadata.original_size);

        if original_size == metadata.original_size {
            println!("   ✅ Size matches");
        } else {
            println!(
                "   ❌ Size differs by {} bytes",
                ((original_size as i64) - (metadata.original_size as i64)).abs()
            );
        }

        // Compare checksums
        println!("\n🔐 Checksum Comparison:");
        println!("   Expected checksum (from .adapipe): {}", metadata.original_checksum);

        // Calculate current file checksum
        println!("   🔄 Calculating current file checksum...");

        let mut hasher = Sha256::new();
        let mut file = std::fs::File::open(&original)?;
        std::io::copy(&mut file, &mut hasher)?;
        let current_checksum = format!("{:x}", hasher.finalize());

        println!("   Current file checksum: {}", current_checksum);

        if current_checksum == metadata.original_checksum {
            println!("   ✅ Checksums match - files are identical");
        } else {
            println!("   ❌ Checksums differ - files are not identical");
        }

        // Show detailed information if requested
        if detailed {
            println!("\n📋 Detailed Information:");
            println!(
                "   .adapipe created: {}",
                metadata.processed_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
            println!("   Pipeline ID: {}", metadata.pipeline_id);
            println!("   Chunk count: {}", metadata.chunk_count);

            if metadata.is_compressed() {
                println!(
                    "   Compression: {}",
                    metadata.compression_algorithm().unwrap_or("unknown")
                );
            }

            if metadata.is_encrypted() {
                println!(
                    "   Encryption: {}",
                    metadata.encryption_algorithm().unwrap_or("unknown")
                );
            }

            let current_modified = original_metadata.modified()?;
            println!(
                "   Current file modified: {}",
                chrono::DateTime::<chrono::Utc>::from(current_modified).format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        // Summary
        println!("\n🎯 Comparison Summary:");
        if original_size == metadata.original_size && current_checksum == metadata.original_checksum {
            println!("   ✅ Files are identical - no changes detected");
        } else {
            println!("   ❌ Files differ - changes detected");
            if detailed {
                println!("   💡 Use 'restore' command to restore from .adapipe if needed");
            }
        }

        Ok(())
    }
}

impl Default for CompareFilesUseCase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires test files
    async fn test_compare_identical_files() {
        // Test with matching original and .adapipe
        // Requires test fixture files
    }

    #[tokio::test]
    #[ignore] // Requires test files
    async fn test_compare_different_files() {
        // Test with modified original file
        // Requires test fixture files
    }

    #[tokio::test]
    async fn test_compare_missing_original() {
        let use_case = CompareFilesUseCase::new();
        let result = use_case
            .execute(
                PathBuf::from("/nonexistent/file.txt"),
                PathBuf::from("/some/file.adapipe"),
                false,
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_missing_adapipe() {
        // Create temp file for original
        let temp_dir = tempfile::TempDir::new().unwrap();
        let original = temp_dir.path().join("test.txt");
        std::fs::write(&original, b"test data").unwrap();

        let use_case = CompareFilesUseCase::new();
        let result = use_case
            .execute(original, PathBuf::from("/nonexistent/file.adapipe"), false)
            .await;
        assert!(result.is_err());
    }
}
