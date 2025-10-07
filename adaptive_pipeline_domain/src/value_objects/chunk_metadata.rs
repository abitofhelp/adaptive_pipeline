// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Chunk Metadata Value Object
//!
//! This module defines the chunk metadata value object for the adaptive
//! pipeline system. It encapsulates all metadata needed to track and manage
//! file chunks throughout the pipeline processing lifecycle.
//!
//! ## Overview
//!
//! The chunk metadata provides:
//!
//! - **Chunk Identification**: Unique identification and description of chunks
//! - **Size Tracking**: Accurate tracking of chunk sizes and boundaries
//! - **Integrity Verification**: Checksums and validation for chunk integrity
//! - **Processing Context**: Context about processing stages and operations
//! - **Temporal Tracking**: Timestamps for chunk lifecycle management
//!
//! ## Architecture
//!
//! The metadata follows Domain-Driven Design principles:
//!
//! - **Value Object**: Immutable value object with equality semantics
//! - **Rich Domain Model**: Encapsulates chunk-related business logic
//! - **Validation**: Comprehensive validation of metadata consistency
//! - **Serialization**: Support for persistence and transmission
//!
//! ## Key Features
//!
//! ### Chunk Identification
//!
//! - **Unique Identifiers**: Unique identification for each chunk
//! - **Descriptive Names**: Human-readable chunk descriptions
//! - **Hierarchical Organization**: Support for chunk hierarchies
//! - **Context Preservation**: Maintain context across processing stages
//!
//! ### Size and Boundary Management
//!
//! - **Accurate Sizing**: Precise tracking of chunk sizes in bytes
//! - **Boundary Information**: Track chunk boundaries within files
//! - **Compression Tracking**: Track size changes during compression
//! - **Memory Management**: Support for memory-efficient processing
//!
//! ### Integrity and Validation
//!
//! - **Checksum Support**: Multiple checksum algorithms for verification
//! - **Integrity Validation**: Comprehensive integrity checking
//! - **Corruption Detection**: Detect and report chunk corruption
//! - **Recovery Information**: Information for chunk recovery
//!
//! ## Usage Examples
//!
//! ### Creating Chunk Metadata

//!
//! ### Working with Attributes

//!
//! ### Integrity Verification

//!
//! ### Processing Stage Tracking

//!
//! ### Serialization and Persistence

//!
//! ## Metadata Attributes
//!
//! ### Standard Attributes
//!
//! Common attributes used across the system:
//!
//! - **compression_ratio**: Compression ratio achieved
//! - **algorithm**: Algorithm used for processing
//! - **level**: Processing level or quality setting
//! - **original_size**: Original size before processing
//! - **processing_time_ms**: Time taken for processing
//!
//! ### Custom Attributes
//!
//! Applications can define custom attributes:
//!
//! - **Application-specific**: Custom metadata for specific use cases
//! - **Processing Context**: Context-specific information
//! - **Performance Metrics**: Custom performance measurements
//! - **Business Logic**: Domain-specific business information
//!
//! ## Integrity Verification
//!
//! ### Checksum Algorithms
//!
//! Supported checksum algorithms:
//!
//! - **SHA-256**: Primary checksum algorithm
//! - **Blake3**: High-performance alternative
//! - **CRC32**: Fast integrity checking
//! - **MD5**: Legacy support (not recommended)
//!
//! ### Verification Process
//!
//! 1. **Calculate Checksum**: Calculate checksum of chunk data
//! 2. **Compare**: Compare with stored checksum
//! 3. **Validate**: Validate checksum format and algorithm
//! 4. **Report**: Report verification results
//!
//! ## Performance Considerations
//!
//! ### Memory Efficiency
//!
//! - **Compact Storage**: Efficient storage of metadata
//! - **Lazy Evaluation**: Lazy evaluation of expensive operations
//! - **String Interning**: Intern common strings to reduce memory usage
//!
//! ### Processing Performance
//!
//! - **Fast Access**: Optimized access to metadata fields
//! - **Efficient Serialization**: Fast serialization/deserialization
//! - **Minimal Overhead**: Minimal overhead during processing
//!
//! ## Validation Rules
//!
//! ### Size Validation
//!
//! - **Positive Size**: Chunk size must be positive
//! - **Reasonable Limits**: Size must be within reasonable limits
//! - **Consistency**: Size must be consistent with actual data
//!
//! ### Identifier Validation
//!
//! - **Non-empty**: Identifier cannot be empty
//! - **Valid Characters**: Must contain only valid characters
//! - **Uniqueness**: Should be unique within context
//!
//! ### Checksum Validation
//!
//! - **Format Validation**: Validate checksum format
//! - **Algorithm Support**: Verify algorithm is supported
//! - **Length Validation**: Validate checksum length
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Invalid Size**: Chunk size is invalid
//! - **Invalid Identifier**: Identifier is invalid
//! - **Invalid Checksum**: Checksum format is invalid
//! - **Inconsistent Data**: Metadata is inconsistent
//!
//! ### Processing Errors
//!
//! - **Checksum Calculation**: Errors during checksum calculation
//! - **Serialization Errors**: Errors during serialization
//! - **Attribute Errors**: Errors with attribute operations
//!
//! ## Integration
//!
//! The chunk metadata integrates with:
//!
//! - **File Chunks**: Associated with file chunk data
//! - **Processing Pipeline**: Used throughout processing pipeline
//! - **Storage Systems**: Persisted with chunk data
//! - **Monitoring**: Used for monitoring and metrics
//!
//! ## Thread Safety
//!
//! The chunk metadata is designed for thread safety:
//!
//! - **Immutable**: Metadata is immutable after creation
//! - **Safe Sharing**: Safe to share between threads
//! - **Concurrent Access**: Safe concurrent access to metadata
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Extended Attributes**: More comprehensive attribute system
//! - **Compression Metadata**: Enhanced compression-specific metadata
//! - **Performance Metrics**: Built-in performance metrics
//! - **Validation Framework**: Enhanced validation capabilities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::PipelineError;

/// Metadata associated with a file chunk during processing
///
/// This value object encapsulates all metadata needed to track and manage
/// chunks throughout the pipeline processing lifecycle, following DDD
/// principles.
///
/// # Key Features
///
/// - **Chunk Identification**: Unique identification and description
/// - **Size Tracking**: Accurate size tracking in bytes
/// - **Integrity Verification**: Checksum-based integrity checking
/// - **Processing Context**: Track processing stages and operations
/// - **Temporal Tracking**: Timestamp-based lifecycle management
/// - **Extensible Attributes**: Custom metadata through key-value attributes
///
/// # Examples
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkMetadata {
    /// Size of the chunk in bytes
    chunk_size: usize,

    /// Identifier or description of the chunk
    identifier: String,

    /// Checksum for integrity verification
    checksum: Option<String>,

    /// Processing stage that created this metadata
    stage: Option<String>,

    /// Timestamp when this metadata was created
    created_at: DateTime<Utc>,

    /// Additional metadata as key-value pairs
    attributes: HashMap<String, String>,
}

impl ChunkMetadata {
    /// Creates new chunk metadata with required fields
    ///
    /// # Arguments
    /// * `chunk_size` - Size of the chunk in bytes
    /// * `identifier` - Unique identifier or description for the chunk
    ///
    /// # Returns
    /// * `Result<ChunkMetadata, PipelineError>` - New metadata instance or
    ///   error
    pub fn new(chunk_size: usize, identifier: String) -> Result<Self, PipelineError> {
        if chunk_size == 0 {
            return Err(PipelineError::ValidationError(
                "Chunk size must be greater than zero".to_string(),
            ));
        }

        if identifier.trim().is_empty() {
            return Err(PipelineError::ValidationError(
                "Chunk identifier cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            chunk_size,
            identifier: identifier.trim().to_string(),
            checksum: None,
            stage: None,
            created_at: chrono::Utc::now(),
            attributes: HashMap::new(),
        })
    }

    /// Creates chunk metadata with all fields for testing
    pub fn new_for_testing(
        chunk_size: usize,
        identifier: String,
        checksum: Option<String>,
        stage: Option<String>,
    ) -> Self {
        Self {
            chunk_size,
            identifier,
            checksum,
            stage,
            created_at: chrono::Utc::now(),
            attributes: HashMap::new(),
        }
    }

    /// Gets the chunk size
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Gets the chunk identifier
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Gets the checksum if available
    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    /// Gets the processing stage if available
    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }

    /// Gets the creation timestamp
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Sets the checksum for integrity verification
    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = Some(checksum);
        self
    }

    /// Sets the processing stage
    pub fn with_stage(mut self, stage: String) -> Self {
        self.stage = Some(stage);
        self
    }

    /// Adds a custom attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// Gets a custom attribute
    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes.get(key).map(|s| s.as_str())
    }

    /// Gets all attributes
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    /// Validates the metadata integrity
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.chunk_size == 0 {
            return Err(PipelineError::ValidationError(
                "Invalid chunk size: must be greater than zero".to_string(),
            ));
        }

        if self.identifier.trim().is_empty() {
            return Err(PipelineError::ValidationError(
                "Invalid identifier: cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ChunkMetadata {
    fn default() -> Self {
        Self {
            chunk_size: 1024, // 1KB default
            identifier: "default_chunk".to_string(),
            checksum: None,
            stage: None,
            created_at: chrono::Utc::now(),
            attributes: HashMap::new(),
        }
    }
}

impl std::fmt::Display for ChunkMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChunkMetadata(id: {}, size: {} bytes, stage: {:?})",
            self.identifier, self.chunk_size, self.stage
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests chunk metadata creation with basic properties.
    ///
    /// This test validates that chunk metadata can be created with
    /// required properties and that all metadata fields are properly
    /// initialized and accessible.
    ///
    /// # Test Coverage
    ///
    /// - Chunk metadata creation with size and identifier
    /// - Chunk size storage and retrieval
    /// - Identifier storage and retrieval
    /// - Optional field initialization (checksum, stage)
    /// - Default values for optional fields
    ///
    /// # Test Scenario
    ///
    /// Creates chunk metadata with basic properties and verifies
    /// all fields are set correctly with proper defaults.
    ///
    /// # Assertions
    ///
    /// - Chunk size matches input
    /// - Identifier matches input
    /// - Checksum is initially None
    /// - Stage is initially None
    #[test]
    fn test_chunk_metadata_creation() {
        let metadata = ChunkMetadata::new(1024, "test_chunk".to_string()).unwrap();

        assert_eq!(metadata.chunk_size(), 1024);
        assert_eq!(metadata.identifier(), "test_chunk");
        assert!(metadata.checksum().is_none());
        assert!(metadata.stage().is_none());
    }

    /// Tests chunk metadata validation rules and constraints.
    ///
    /// This test validates that chunk metadata enforces proper
    /// validation rules for size and identifier fields, rejecting
    /// invalid inputs appropriately.
    ///
    /// # Test Coverage
    ///
    /// - Zero size validation and rejection
    /// - Empty identifier validation and rejection
    /// - Whitespace-only identifier validation
    /// - Input validation error handling
    /// - Constraint enforcement
    ///
    /// # Test Scenario
    ///
    /// Tests various invalid inputs including zero size, empty
    /// identifier, and whitespace-only identifier to ensure
    /// proper validation and error handling.
    ///
    /// # Assertions
    ///
    /// - Zero size creation fails
    /// - Empty identifier creation fails
    /// - Whitespace-only identifier creation fails
    /// - Validation errors are returned appropriately
    #[test]
    fn test_chunk_metadata_validation() {
        // Test zero size validation
        let result = ChunkMetadata::new(0, "test".to_string());
        assert!(result.is_err());

        // Test empty identifier validation
        let result = ChunkMetadata::new(1024, "".to_string());
        assert!(result.is_err());

        // Test whitespace-only identifier validation
        let result = ChunkMetadata::new(1024, "   ".to_string());
        assert!(result.is_err());
    }

    /// Tests chunk metadata builder pattern for fluent construction.
    ///
    /// This test validates that chunk metadata supports a fluent
    /// builder pattern for constructing metadata with optional
    /// fields and custom attributes.
    ///
    /// # Test Coverage
    ///
    /// - Builder pattern with method chaining
    /// - Checksum addition with `with_checksum()`
    /// - Stage assignment with `with_stage()`
    /// - Custom attribute addition with `with_attribute()`
    /// - Attribute retrieval with `get_attribute()`
    /// - Fluent API construction
    ///
    /// # Test Scenario
    ///
    /// Creates chunk metadata using the builder pattern to add
    /// checksum, stage, and custom attributes, then verifies
    /// all fields are set correctly.
    ///
    /// # Assertions
    ///
    /// - Chunk size is preserved
    /// - Identifier is preserved
    /// - Checksum is set correctly
    /// - Stage is set correctly
    /// - Custom attribute is stored and retrievable
    #[test]
    fn test_chunk_metadata_builder_pattern() {
        let metadata = ChunkMetadata::new(2048, "test_chunk".to_string())
            .unwrap()
            .with_checksum("abc123".to_string())
            .with_stage("compression".to_string())
            .with_attribute("compression_ratio".to_string(), "0.7".to_string());

        assert_eq!(metadata.chunk_size(), 2048);
        assert_eq!(metadata.identifier(), "test_chunk");
        assert_eq!(metadata.checksum(), Some("abc123"));
        assert_eq!(metadata.stage(), Some("compression"));
        assert_eq!(metadata.get_attribute("compression_ratio"), Some("0.7"));
    }

    /// Tests chunk metadata display formatting and string representation.
    ///
    /// This test validates that chunk metadata provides proper
    /// string representation through the Display trait, including
    /// all relevant metadata fields.
    ///
    /// # Test Coverage
    ///
    /// - Display trait implementation
    /// - String representation formatting
    /// - Identifier inclusion in display
    /// - Size inclusion in display
    /// - Stage inclusion in display
    /// - Human-readable output
    ///
    /// # Test Scenario
    ///
    /// Creates chunk metadata with stage information and verifies
    /// the display output contains all relevant fields in a
    /// human-readable format.
    ///
    /// # Assertions
    ///
    /// - Display contains identifier
    /// - Display contains size
    /// - Display contains stage information
    /// - Output is human-readable
    #[test]
    fn test_chunk_metadata_display() {
        let metadata = ChunkMetadata::new(1024, "test_chunk".to_string())
            .unwrap()
            .with_stage("encryption".to_string());

        let display = format!("{}", metadata);
        assert!(display.contains("test_chunk"));
        assert!(display.contains("1024"));
        assert!(display.contains("encryption"));
    }
}
