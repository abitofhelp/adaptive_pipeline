// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # File Chunk Value Object
//!
//! This module provides the `FileChunk` value object, which represents an
//! immutable chunk of file data for processing within the adaptive pipeline
//! system. It follows Domain-Driven Design principles and ensures data
//! integrity throughout processing.
//!
//! ## Overview
//!
//! The file chunk value object provides:
//!
//! - **Immutable Data**: Once created, chunks cannot be modified
//! - **Unique Identity**: Each chunk has a unique UUID for tracking
//! - **Sequence Ordering**: Chunks maintain sequence numbers for reassembly
//! - **Integrity Verification**: Optional checksums for data integrity
//! - **Metadata Tracking**: Creation timestamps and processing metadata
//!
//! ## Design Principles
//!
//! The file chunk follows Domain-Driven Design value object principles:
//!
//! - **Immutability**: Once created, chunks cannot be modified
//! - **Value Semantics**: Chunks are compared by value, not identity
//! - **Self-Validation**: Chunks validate their own data integrity
//! - **Rich Behavior**: Chunks provide methods for common operations
//!
//! ## Chunk Structure
//!
//! ### Core Data
//! - **ID**: Unique UUID for chunk identification and tracking
//! - **Sequence Number**: Position in the original file for reassembly
//! - **Offset**: Byte offset in the original file
//! - **Size**: Validated chunk size within system limits
//! - **Data**: The actual chunk data bytes
//!
//! ### Metadata
//! - **Checksum**: Optional SHA-256 checksum for integrity verification
//! - **Is Final**: Flag indicating if this is the last chunk in a file
//! - **Created At**: UTC timestamp of chunk creation
//!
//! ## Usage Examples
//!
//! ### Basic Chunk Creation

//!
//! ### Chunk with Checksum

//!
//! ### Chunk Processing Chain

//!
//! ## Chunk Validation
//!
//! ### Data Integrity

//!
//! ### Sequence Validation

//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Data Storage**: Chunks store data in `Vec<u8>` for efficient access
//! - **Metadata Overhead**: Minimal metadata overhead per chunk
//! - **Cloning**: Chunks can be cloned efficiently for processing
//!
//! ### Processing Efficiency
//!
//! - **Immutable Design**: Prevents accidental mutations during processing
//! - **Builder Pattern**: Efficient creation of modified chunks
//! - **Lazy Checksum**: Checksums are calculated only when needed
//!
//! ### Memory Management
//!
//! - **Automatic Cleanup**: Chunks are automatically cleaned up when dropped
//! - **Reference Counting**: Use `Arc<FileChunk>` for shared ownership
//! - **Streaming**: Chunks can be processed in streaming fashion
//!
//! ## Thread Safety
//!
//! The file chunk is fully thread-safe:
//!
//! - **Immutable**: Once created, chunks cannot be modified
//! - **Send + Sync**: Chunks can be safely sent between threads
//! - **No Shared State**: No mutable shared state to synchronize
//!
//! ## Serialization
//!
//! ### JSON Serialization

//!
//! ### Binary Serialization

//!
//! ## Integration
//!
//! The file chunk integrates with:
//!
//! - **File Processing**: Core unit of file processing operations
//! - **Pipeline Stages**: Passed between processing stages
//! - **Storage Systems**: Serialized for persistent storage
//! - **Network Transport**: Transmitted between distributed components
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Invalid Size**: Chunk size outside valid bounds
//! - **Invalid Data**: Corrupted or invalid chunk data
//! - **Checksum Mismatch**: Data integrity verification failures
//! - **Sequence Errors**: Invalid sequence numbers or ordering
//!
//! ### Recovery Strategies
//!
//! - **Retry Logic**: Automatic retry for transient failures
//! - **Fallback Processing**: Alternative processing for corrupted chunks
//! - **Error Reporting**: Detailed error context for debugging
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Compression**: Built-in compression for chunk data
//! - **Encryption**: Encrypted chunk data for security
//! - **Streaming**: Streaming chunk processing for large files
//! - **Caching**: Intelligent caching of frequently accessed chunks

use crate::services::datetime_serde;
use crate::{ChunkSize, PipelineError};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Represents an immutable chunk of file data for processing
///
/// This is a Value Object in Domain-Driven Design terms - it represents data
/// without identity that cannot be modified once created. Any "changes" create
/// new instances, ensuring data integrity and preventing accidental mutations
/// during processing.
///
/// # Key Features
///
/// - **Immutability**: Once created, chunks cannot be modified
/// - **Unique Identity**: Each chunk has a UUID for tracking and identification
/// - **Sequence Ordering**: Maintains sequence numbers for proper file
///   reassembly
/// - **Integrity Verification**: Optional checksums for data integrity
///   validation
/// - **Metadata Tracking**: Creation timestamps and processing metadata
///
/// # Design Principles
///
/// - **Value Object**: Compared by value, not identity
/// - **Self-Validation**: Validates its own data integrity
/// - **Builder Pattern**: Use methods like `with_checksum()` for modifications
/// - **Thread Safety**: Fully thread-safe due to immutability
///
/// # Examples
///
///
/// # Developer Notes
///
/// - Use builder methods like `with_checksum()` to create modified versions
/// - Processing stages should create new chunks rather than modifying existing
///   ones
/// - This design prevents data corruption and ensures thread safety
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileChunk {
    id: Uuid,
    sequence_number: u64,
    offset: u64,
    size: ChunkSize,
    data: Vec<u8>,
    checksum: Option<String>,
    is_final: bool,
    #[serde(with = "datetime_serde")]
    created_at: chrono::DateTime<chrono::Utc>,
}

impl FileChunk {
    /// Creates a new file chunk
    ///
    /// # Purpose
    /// Creates an immutable file chunk value object for pipeline processing.
    /// Chunks are the fundamental unit of file processing in the adaptive
    /// pipeline.
    ///
    /// # Why
    /// File chunking enables:
    /// - Parallel processing of large files
    /// - Memory-efficient streaming
    /// - Independent processing units
    /// - Granular error recovery
    ///
    /// # Arguments
    /// * `sequence_number` - The order of this chunk in the file (0-based)
    /// * `offset` - Byte offset in the original file where this chunk starts
    /// * `data` - The actual chunk data bytes (must not be empty)
    /// * `is_final` - Whether this is the last chunk in the file
    ///
    /// # Returns
    /// * `Ok(FileChunk)` - Successfully created chunk with unique UUID
    /// * `Err(PipelineError::InvalidChunk)` - Data is empty
    ///
    /// # Errors
    /// Returns `PipelineError::InvalidChunk` when data is empty.
    ///
    /// # Side Effects
    /// - Generates new UUID for chunk identification
    /// - Sets creation timestamp to current UTC time
    /// - Calculates chunk size from data length
    ///
    /// # Examples
    ///
    ///
    /// # Developer Notes
    /// - Each chunk gets a unique UUID for tracking across pipeline stages
    /// - Chunk size is automatically validated against system limits
    /// - Checksum is initially None - use `with_calculated_checksum()` to add
    /// - This is a Value Object - create new instances for "changes"
    pub fn new(sequence_number: u64, offset: u64, data: Vec<u8>, is_final: bool) -> Result<Self, PipelineError> {
        if data.is_empty() {
            return Err(PipelineError::InvalidChunk("Chunk data cannot be empty".to_string()));
        }

        let size = ChunkSize::new(data.len())?;

        Ok(FileChunk {
            id: Uuid::new_v4(),
            sequence_number,
            offset,
            size,
            data,
            checksum: None,
            is_final,
            created_at: chrono::Utc::now(),
        })
    }

    /// Creates a new file chunk with checksum
    ///
    /// # Developer Notes
    /// - This is a convenience constructor for chunks that already have
    ///   checksums
    /// - Prefer using `new()` followed by `with_checksum()` for clarity
    pub fn new_with_checksum(
        sequence_number: u64,
        offset: u64,
        data: Vec<u8>,
        checksum: String,
        is_final: bool,
    ) -> Result<Self, PipelineError> {
        let chunk = Self::new(sequence_number, offset, data, is_final)?;
        Ok(chunk.with_checksum(checksum))
    }

    // === Immutable Accessors ===

    /// Gets the chunk ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Gets the sequence number
    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    /// Gets the offset in the original file
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Gets the chunk size
    pub fn size(&self) -> &ChunkSize {
        &self.size
    }

    /// Gets the chunk data (immutable reference)
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Gets the checksum if available
    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    /// Checks if this is the final chunk
    pub fn is_final(&self) -> bool {
        self.is_final
    }

    /// Gets the creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    /// Gets the actual data length
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the chunk is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // === Immutable Builder Pattern Methods ===

    /// Creates a new FileChunk with updated data
    ///
    /// # Developer Notes
    /// - This creates a completely new chunk instance
    /// - The old chunk remains unchanged (immutability)
    /// - Checksum is cleared since data changed
    /// - Use this pattern: `let new_chunk =
    ///   old_chunk.with_data(new_data).unwrap();`
    pub fn with_data(&self, data: Vec<u8>) -> Result<Self, PipelineError> {
        if data.is_empty() {
            return Err(PipelineError::InvalidChunk("Chunk data cannot be empty".to_string()));
        }

        let size = ChunkSize::new(data.len())?;

        Ok(FileChunk {
            id: Uuid::new_v4(), // New chunk gets new ID
            sequence_number: self.sequence_number,
            offset: self.offset,
            size,
            data,
            checksum: None, // Clear checksum when data changes
            is_final: self.is_final,
            created_at: chrono::Utc::now(), // New creation time
        })
    }

    /// Creates a new FileChunk with a checksum
    ///
    /// # Developer Notes
    /// - This preserves all other data and adds/updates the checksum
    /// - Use this after processing: `let verified_chunk =
    ///   chunk.with_checksum(hash);`
    pub fn with_checksum(&self, checksum: String) -> Self {
        FileChunk {
            id: self.id,
            sequence_number: self.sequence_number,
            offset: self.offset,
            size: self.size,
            data: self.data.clone(),
            checksum: Some(checksum),
            is_final: self.is_final,
            created_at: self.created_at,
        }
    }

    /// Creates a new FileChunk with calculated SHA-256 checksum
    ///
    /// # Developer Notes
    /// - Calculates SHA-256 hash of current data
    /// - Returns new chunk with checksum set
    /// - Original chunk remains unchanged
    pub fn with_calculated_checksum(&self) -> Result<Self, PipelineError> {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let digest = hasher.finalize();
        let checksum = hex::encode(digest);
        Ok(self.with_checksum(checksum))
    }

    /// Creates a new FileChunk without data (for security)
    ///
    /// # Developer Notes
    /// - Creates new chunk with empty data vector
    /// - Useful for secure cleanup while preserving metadata
    /// - Checksum is cleared since data is gone
    pub fn without_data(&self) -> Self {
        FileChunk {
            id: self.id,
            sequence_number: self.sequence_number,
            offset: self.offset,
            size: ChunkSize::new(0).unwrap_or_else(|_| ChunkSize::default()), /* Empty chunk - ChunkSize(0) should
                                                                               * never fail, but handle it safely */
            data: Vec::new(),
            checksum: None, // Clear checksum
            is_final: self.is_final,
            created_at: self.created_at,
        }
    }

    // === Verification Methods (Read-Only) ===

    /// Verifies the chunk integrity using the stored checksum
    ///
    /// # Purpose
    /// Validates that chunk data has not been corrupted by comparing the stored
    /// SHA-256 checksum against a freshly calculated hash of the current data.
    ///
    /// # Why
    /// Integrity verification provides:
    /// - Detection of data corruption during processing or storage
    /// - Confidence in pipeline operations
    /// - Early error detection before expensive operations
    /// - Compliance with data integrity requirements
    ///
    /// # Returns
    /// * `Ok(true)` - Checksum matches, data is intact
    /// * `Ok(false)` - Checksum mismatch, data corrupted
    /// * `Err(PipelineError::InvalidChunk)` - No checksum available
    ///
    /// # Errors
    /// Returns `PipelineError::InvalidChunk` when the chunk has no stored
    /// checksum.
    ///
    /// # Examples
    ///
    ///
    /// # Developer Notes
    /// - This method is read-only and doesn't modify the chunk
    /// - Use before critical processing to ensure data integrity
    /// - Consider verification before expensive operations like encryption
    pub fn verify_integrity(&self) -> Result<bool, PipelineError> {
        if let Some(stored_checksum) = &self.checksum {
            let mut hasher = Sha256::new();
            hasher.update(&self.data);
            let digest = hasher.finalize();
            let calculated_checksum = hex::encode(digest);
            Ok(calculated_checksum == *stored_checksum)
        } else {
            Err(PipelineError::InvalidChunk(
                "No checksum available for verification".to_string(),
            ))
        }
    }

    /// Calculates SHA-256 checksum without modifying the chunk
    ///
    /// # Developer Notes
    /// - This is a pure function - doesn't modify the chunk
    /// - Use when you need the checksum but don't want to create a new chunk
    /// - For creating a chunk with checksum, use `with_calculated_checksum()`
    pub fn calculate_checksum(&self) -> Result<String, PipelineError> {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let digest = hasher.finalize();
        Ok(hex::encode(digest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests file chunk creation with basic properties.
    ///
    /// This test validates that file chunks can be created with
    /// required properties and that all metadata is properly
    /// stored and accessible.
    ///
    /// # Test Coverage
    ///
    /// - File chunk creation with minimum size requirement
    /// - Sequence number assignment
    /// - Offset position tracking
    /// - Data storage and retrieval
    /// - Final chunk flag handling
    /// - Checksum initialization (none by default)
    ///
    /// # Test Scenario
    ///
    /// Creates a file chunk with test data meeting minimum size
    /// requirements and verifies all properties are set correctly.
    ///
    /// # Assertions
    ///
    /// - Sequence number matches input
    /// - Offset matches input
    /// - Data is stored correctly
    /// - Final flag is set correctly
    /// - Checksum is initially None
    #[test]
    fn test_file_chunk_creation() {
        // Create test data that meets minimum chunk size requirement (1MB)
        let data = vec![42u8; ChunkSize::MIN_SIZE];
        let chunk = FileChunk::new(0, 0, data.clone(), false).unwrap();

        assert_eq!(chunk.sequence_number(), 0);
        assert_eq!(chunk.offset(), 0);
        assert_eq!(chunk.data(), &data);
        assert!(!chunk.is_final());
        assert!(chunk.checksum().is_none());
    }

    /// Tests file chunk immutability and data modification behavior.
    ///
    /// This test validates that file chunks are immutable and that
    /// data modifications create new chunk instances while preserving
    /// the original chunk unchanged.
    ///
    /// # Test Coverage
    ///
    /// - File chunk immutability
    /// - Data modification with `with_data()`
    /// - Original chunk preservation
    /// - New chunk creation
    /// - Unique ID generation for modified chunks
    ///
    /// # Test Scenario
    ///
    /// Creates a file chunk, modifies its data using `with_data()`,
    /// then verifies the original chunk is unchanged and the new
    /// chunk has different data and ID.
    ///
    /// # Assertions
    ///
    /// - Original chunk data is unchanged
    /// - New chunk has modified data
    /// - Chunk IDs are different
    /// - Immutability is preserved
    #[test]
    fn test_file_chunk_immutability() {
        let data = vec![42u8; ChunkSize::MIN_SIZE];
        let chunk1 = FileChunk::new(0, 0, data.clone(), false).unwrap();

        // Creating new chunk with different data
        let new_data = vec![99u8; ChunkSize::MIN_SIZE];
        let chunk2 = chunk1.with_data(new_data.clone()).unwrap();

        // Original chunk unchanged
        assert_eq!(chunk1.data(), &data);
        assert_eq!(chunk2.data(), &new_data);
        assert_ne!(chunk1.id(), chunk2.id()); // Different IDs
    }

    /// Tests file chunk checksum addition and preservation.
    ///
    /// This test validates that checksums can be added to file chunks
    /// while preserving the original chunk and maintaining the same
    /// chunk ID for checksum-only modifications.
    ///
    /// # Test Coverage
    ///
    /// - Checksum addition with `with_checksum()`
    /// - Original chunk preservation
    /// - Checksum storage and retrieval
    /// - ID preservation for checksum addition
    /// - Checksum immutability
    ///
    /// # Test Scenario
    ///
    /// Creates a file chunk without checksum, adds a checksum using
    /// `with_checksum()`, then verifies the original chunk is unchanged
    /// and the new chunk has the checksum with the same ID.
    ///
    /// # Assertions
    ///
    /// - Original chunk has no checksum
    /// - New chunk has the specified checksum
    /// - Chunk IDs are the same (checksum addition preserves ID)
    /// - Checksum is stored correctly
    #[test]
    fn test_file_chunk_with_checksum() {
        let data = vec![42u8; ChunkSize::MIN_SIZE];
        let chunk1 = FileChunk::new(0, 0, data, false).unwrap();
        let chunk2 = chunk1.with_checksum("test_hash".to_string());

        // Original chunk unchanged
        assert!(chunk1.checksum().is_none());
        assert_eq!(chunk2.checksum(), Some("test_hash"));
        assert_eq!(chunk1.id(), chunk2.id()); // Same ID for checksum addition
    }

    /// Tests file chunk automatic checksum calculation.
    ///
    /// This test validates that file chunks can automatically
    /// calculate checksums from their data and that the calculated
    /// checksum matches manual calculation.
    ///
    /// # Test Coverage
    ///
    /// - Automatic checksum calculation with `with_calculated_checksum()`
    /// - Manual checksum calculation with `calculate_checksum()`
    /// - Checksum accuracy verification
    /// - Original chunk preservation
    /// - Checksum consistency
    ///
    /// # Test Scenario
    ///
    /// Creates a file chunk, calculates its checksum automatically,
    /// then verifies the checksum matches manual calculation.
    ///
    /// # Assertions
    ///
    /// - Original chunk has no checksum
    /// - New chunk has calculated checksum
    /// - Calculated checksum matches manual calculation
    /// - Checksum calculation is accurate
    #[test]
    fn test_file_chunk_calculated_checksum() {
        let data = vec![42u8; ChunkSize::MIN_SIZE];
        let chunk = FileChunk::new(0, 0, data, false).unwrap();
        let chunk_with_checksum = chunk.with_calculated_checksum().unwrap();

        assert!(chunk.checksum().is_none());
        assert!(chunk_with_checksum.checksum().is_some());

        // Verify the checksum is correct
        let calculated = chunk.calculate_checksum().unwrap();
        assert_eq!(chunk_with_checksum.checksum(), Some(calculated.as_str()));
    }

    /// Tests file chunk integrity verification.
    ///
    /// This test validates that file chunks can verify their data
    /// integrity using checksums and that verification fails
    /// appropriately when checksums are missing.
    ///
    /// # Test Coverage
    ///
    /// - Integrity verification with `verify_integrity()`
    /// - Successful verification with valid checksum
    /// - Failed verification without checksum
    /// - Checksum-based data validation
    /// - Error handling for missing checksums
    ///
    /// # Test Scenario
    ///
    /// Creates a file chunk with calculated checksum and verifies
    /// integrity passes, then tests that chunks without checksums
    /// fail verification.
    ///
    /// # Assertions
    ///
    /// - Chunk with checksum passes integrity verification
    /// - Chunk without checksum fails integrity verification
    /// - Verification logic works correctly
    /// - Error handling is appropriate
    #[test]
    fn test_file_chunk_verify_integrity() {
        let data = vec![42u8; ChunkSize::MIN_SIZE];
        let chunk = FileChunk::new(0, 0, data, false).unwrap();
        let chunk_with_checksum = chunk.with_calculated_checksum().unwrap();

        // Should verify successfully
        assert!(chunk_with_checksum.verify_integrity().unwrap());

        // Chunk without checksum should error
        assert!(chunk.verify_integrity().is_err());
    }

    /// Tests file chunk rejection of empty data.
    ///
    /// This test validates that file chunks reject empty data
    /// during creation, ensuring all chunks contain meaningful
    /// data for processing.
    ///
    /// # Test Coverage
    ///
    /// - Empty data rejection during creation
    /// - Validation error handling
    /// - Minimum data requirements
    /// - Input validation
    /// - Error message clarity
    ///
    /// # Test Scenario
    ///
    /// Attempts to create a file chunk with empty data and
    /// verifies that creation fails with appropriate error.
    ///
    /// # Assertions
    ///
    /// - Empty data creation fails
    /// - Error is returned appropriately
    /// - Validation prevents invalid chunks
    /// - Input requirements are enforced
    #[test]
    fn test_empty_data_rejection() {
        let result = FileChunk::new(0, 0, vec![], false);
        assert!(result.is_err());
    }
}
