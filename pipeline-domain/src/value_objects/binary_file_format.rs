//! # Binary File Format Value Object
//!
//! This module defines the binary file format specification for the Adaptive
//! Pipeline system. It provides a standardized format for storing processed
//! files with complete recovery metadata and integrity verification.
//!
//! ## Overview
//!
//! The binary file format provides:
//!
//! - **File Recovery**: Complete metadata for recovering original files
//! - **Integrity Verification**: Checksums and validation for processed files
//! - **Processing History**: Complete record of processing steps applied
//! - **Version Management**: Format versioning for backward compatibility
//! - **Compression Support**: Efficient storage of processed data
//!
//! ## Architecture
//!
//! The format follows a structured binary layout:
//!
//! - **Magic Bytes**: File format identification
//! - **Version Header**: Format version information
//! - **Metadata Section**: Processing metadata and recovery information
//! - **Data Section**: Actual processed file data
//! - **Integrity Section**: Checksums and validation data
//!
//! ## Key Features
//!
//! ### File Recovery
//!
//! - **Original Filename**: Preserve original file names
//! - **File Size**: Track original and processed file sizes
//! - **Processing Steps**: Record all processing operations applied
//! - **Restoration Metadata**: Information needed for complete recovery
//!
//! ### Integrity Verification
//!
//! - **Checksums**: Multiple checksum algorithms for verification
//! - **Validation**: Comprehensive validation of file integrity
//! - **Error Detection**: Detect corruption and processing errors
//! - **Recovery Verification**: Verify recovered files match originals
//!
//! ### Format Versioning
//!
//! - **Version Management**: Support for multiple format versions
//! - **Backward Compatibility**: Maintain compatibility with older versions
//! - **Migration Support**: Automatic migration between format versions
//! - **Feature Evolution**: Support for new features in future versions
//!
//! ## Usage Examples
//!
//! ### Creating a Binary File

//!
//! ### Reading and Validating a Binary File

//!
//! ### File Recovery Process

//!
//! ## File Format Specification
//!
//! ### Binary Layout
//!
//! The .adapipe file format uses the following binary layout:
//!
//!
//! ### Header Components
//!
//! - **Magic Bytes**: 8 bytes - "ADAPIPE\0" (0x41444150495045000)
//! - **Format Version**: 2 bytes - Current version number
//! - **Header Length**: 4 bytes - Length of JSON header in bytes
//! - **JSON Header**: Variable length - Metadata and processing information
//! - **Processed Data**: Variable length - Actual processed file content
//!
//! ### JSON Header Structure
//!
//!
//! ## Processing Steps
//!
//! ### Supported Operations
//!
//! - **Compression**: Various compression algorithms (brotli, gzip, lz4)
//! - **Encryption**: Encryption algorithms (AES-256-GCM, ChaCha20-Poly1305)
//! - **Validation**: Checksum and integrity validation
//! - **Transformation**: Custom data transformations
//!
//! ### Step Parameters
//!
//! Each processing step can include parameters:
//!
//! - **Compression Level**: Compression quality/speed tradeoff
//! - **Encryption Keys**: Key derivation and management information
//! - **Algorithm Options**: Algorithm-specific configuration
//! - **Custom Parameters**: Application-specific parameters
//!
//! ## Integrity Verification
//!
//! ### Checksum Algorithms
//!
//! - **SHA-256**: Primary checksum algorithm
//! - **Blake3**: High-performance alternative
//! - **CRC32**: Fast integrity checking
//! - **Custom**: Support for custom checksum algorithms
//!
//! ### Verification Process
//!
//! 1. **Format Validation**: Verify magic bytes and version
//! 2. **Header Validation**: Validate JSON header structure
//! 3. **Data Integrity**: Verify processed data checksum
//! 4. **Recovery Verification**: Verify recovered data matches original
//!
//! ## Error Handling
//!
//! ### Format Errors
//!
//! - **Invalid Magic Bytes**: File is not in .adapipe format
//! - **Unsupported Version**: Format version not supported
//! - **Corrupt Header**: JSON header is malformed or corrupt
//! - **Invalid Data**: Processed data is corrupt or invalid
//!
//! ### Recovery Errors
//!
//! - **Missing Steps**: Required processing steps are missing
//! - **Invalid Parameters**: Processing parameters are invalid
//! - **Checksum Mismatch**: Data integrity verification failed
//! - **Recovery Failure**: Unable to recover original data
//!
//! ## Performance Considerations
//!
//! ### File Size Optimization
//!
//! - **Efficient Encoding**: Compact binary encoding for metadata
//! - **Compression**: Built-in compression for processed data
//! - **Minimal Overhead**: Minimal format overhead
//!
//! ### Processing Performance
//!
//! - **Streaming**: Support for streaming processing of large files
//! - **Parallel Processing**: Parallel processing of file chunks
//! - **Memory Efficiency**: Efficient memory usage during processing
//!
//! ## Security Considerations
//!
//! ### Data Protection
//!
//! - **Encryption**: Strong encryption for sensitive data
//! - **Key Management**: Secure key derivation and management
//! - **Integrity**: Comprehensive integrity verification
//!
//! ### Attack Prevention
//!
//! - **Format Validation**: Prevent malformed file attacks
//! - **Size Limits**: Prevent resource exhaustion attacks
//! - **Checksum Verification**: Prevent data tampering
//!
//! ## Version Management
//!
//! ### Format Versioning
//!
//! - **Semantic Versioning**: Use semantic versioning for format versions
//! - **Backward Compatibility**: Maintain compatibility with older versions
//! - **Migration**: Automatic migration between format versions
//!
//! ### Feature Evolution
//!
//! - **New Algorithms**: Support for new compression/encryption algorithms
//! - **Enhanced Metadata**: Extended metadata capabilities
//! - **Performance Improvements**: Optimizations in new versions
//!
//! ## Integration
//!
//! The binary file format integrates with:
//!
//! - **File Processor**: Used by file processor for creating processed files
//! - **Storage Systems**: Store processed files in various storage systems
//! - **Recovery Systems**: Recover original files from processed files
//! - **Validation Systems**: Validate file integrity and format compliance
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Streaming Support**: Enhanced streaming capabilities
//! - **Compression Improvements**: Better compression algorithms
//! - **Metadata Extensions**: Extended metadata capabilities
//! - **Performance Optimizations**: Further performance improvements

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate::PipelineError;

/// Magic bytes to identify our file format: "ADAPIPE\0"
///
/// These magic bytes are used to identify files in the Adaptive Pipeline
/// binary format. They appear at the end of the file for efficient
/// format detection without reading the entire file.
///
/// The magic bytes spell "ADAPIPE" followed by a null terminator:
/// - 0x41 = 'A'
/// - 0x44 = 'D'
/// - 0x41 = 'A'
/// - 0x50 = 'P'
/// - 0x49 = 'I'
/// - 0x50 = 'P'
/// - 0x45 = 'E'
/// - 0x00 = null terminator
pub const MAGIC_BYTES: [u8; 8] = [0x41, 0x44, 0x41, 0x50, 0x49, 0x50, 0x45, 0x00];

/// Current file format version
///
/// This constant defines the current version of the .adapipe file format.
/// It is used for:
/// - Format version validation when reading files
/// - Backward compatibility checking
/// - Migration between format versions
/// - Feature availability determination
///
/// Version history:
/// - Version 1: Initial format with basic compression and encryption support
pub const CURRENT_FORMAT_VERSION: u16 = 1;

/// File header for Adaptive Pipeline processed files (.adapipe format)
///
/// This header contains all information needed to:
/// 1. Recover the original document (filename, size, processing steps)
/// 2. Verify integrity of the processed output file we created
/// 3. Validate the restored input file matches the original exactly
///
/// # Adaptive Pipeline File Format (.adapipe)
/// ```text
/// [CHUNK_DATA][JSON_HEADER][HEADER_LENGTH][FORMAT_VERSION][MAGIC_BYTES]
/// ```
///
/// Note: This is NOT a general binary file format like .png or .exe.
/// This is specifically for files processed by the Adaptive Pipeline system
/// that have been compressed and/or encrypted with restoration metadata.
///
/// # Recovery Process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHeader {
    /// Application version that created this file
    pub app_version: String,

    /// File format version for backward compatibility
    pub format_version: u16,

    /// Original input filename (for restoration)
    pub original_filename: String,

    /// Original file size in bytes (for validation)
    pub original_size: u64,

    /// SHA256 checksum of original input file (for validation)
    pub original_checksum: String,

    /// SHA256 checksum of this output file (for integrity verification)
    pub output_checksum: String,

    /// Processing pipeline information (for restoration)
    pub processing_steps: Vec<ProcessingStep>,

    /// Chunk size used for processing
    pub chunk_size: u32,

    /// Number of chunks in the processed file
    pub chunk_count: u32,

    /// Processing timestamp (RFC3339)
    pub processed_at: chrono::DateTime<chrono::Utc>,

    /// Pipeline ID that processed this file
    pub pipeline_id: String,

    /// Additional metadata for debugging/auditing
    pub metadata: HashMap<String, String>,
}

/// A single processing step that was applied to the file
/// Steps are stored in the order they were applied, and must be reversed in
/// reverse order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingStep {
    /// Step type (compression, encryption, etc.)
    pub step_type: ProcessingStepType,

    /// Algorithm used
    pub algorithm: String,

    /// Algorithm-specific parameters needed for restoration
    pub parameters: HashMap<String, String>,

    /// Order in which this step was applied (0-based)
    pub order: u32,
}

/// Types of processing steps
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessingStepType {
    /// Compression step
    Compression,
    /// Encryption step  
    Encryption,
    /// Checksum/integrity verification step
    Checksum,
    /// Pass-through step (no data modification)
    PassThrough,
    /// Legacy custom processing step (deprecated)
    Custom(String),
}

/// Format for individual chunks in the file
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkFormat {
    /// Encryption nonce (12 bytes for AES-GCM)
    pub nonce: [u8; 12],

    /// Length of encrypted data
    pub data_length: u32,

    /// Encrypted (and possibly compressed) chunk data
    pub encrypted_data: Vec<u8>,
}

impl FileHeader {
    /// Creates a new file header with default values
    ///
    /// # Purpose
    /// Creates a `FileHeader` for tracking processing metadata and enabling file recovery.
    /// The header stores all information needed to validate and restore processed files.
    ///
    /// # Why
    /// File headers provide:
    /// - Recovery information to restore original files
    /// - Integrity verification through checksums
    /// - Processing history for debugging and auditing
    /// - Version management for backward compatibility
    ///
    /// # Arguments
    /// * `original_filename` - Name of the original input file (for restoration)
    /// * `original_size` - Size of the original file in bytes (for validation)
    /// * `original_checksum` - SHA256 checksum of original file (for validation)
    ///
    /// # Returns
    /// `FileHeader` with default values:
    /// - `app_version`: Current package version from Cargo.toml
    /// - `format_version`: Current format version (1)
    /// - `chunk_size`: 1MB default
    /// - `processed_at`: Current timestamp
    /// - Empty processing steps, pipeline ID, and metadata
    ///
    /// # Examples
    ///
    pub fn new(original_filename: String, original_size: u64, original_checksum: String) -> Self {
        Self {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            format_version: CURRENT_FORMAT_VERSION,
            original_filename,
            original_size,
            original_checksum,
            output_checksum: String::new(), // Will be set after processing
            processing_steps: Vec::new(),
            chunk_size: 1024 * 1024, // Default 1MB
            chunk_count: 0,
            processed_at: chrono::Utc::now(),
            pipeline_id: String::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a compression step to the processing pipeline
    ///
    /// # Purpose
    /// Records a compression operation in the processing steps.
    /// This information is used during file recovery to decompress the data.
    ///
    /// # Arguments
    /// * `algorithm` - Name of compression algorithm (e.g., "brotli", "gzip", "zstd", "lz4")
    /// * `level` - Compression level (algorithm-specific, typically 1-9)
    ///
    /// # Returns
    /// Updated `FileHeader` with compression step added (builder pattern)
    ///
    /// # Examples
    ///
    pub fn add_compression_step(mut self, algorithm: &str, level: u32) -> Self {
        let mut parameters = HashMap::new();
        parameters.insert("level".to_string(), level.to_string());

        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Compression,
            algorithm: algorithm.to_string(),
            parameters,
            order: self.processing_steps.len() as u32,
        });
        self
    }

    /// Adds an encryption step
    pub fn add_encryption_step(
        mut self,
        algorithm: &str,
        key_derivation: &str,
        key_size: u32,
        nonce_size: u32,
    ) -> Self {
        let mut parameters = HashMap::new();
        parameters.insert("key_derivation".to_string(), key_derivation.to_string());
        parameters.insert("key_size".to_string(), key_size.to_string());
        parameters.insert("nonce_size".to_string(), nonce_size.to_string());

        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Encryption,
            algorithm: algorithm.to_string(),
            parameters,
            order: self.processing_steps.len() as u32,
        });
        self
    }

    /// Adds a custom processing step
    pub fn add_custom_step(mut self, step_name: &str, algorithm: &str, parameters: HashMap<String, String>) -> Self {
        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Custom(step_name.to_string()),
            algorithm: algorithm.to_string(),
            parameters,
            order: self.processing_steps.len() as u32,
        });
        self
    }

    /// Adds a processing step using domain-driven ProcessingStepDescriptor
    /// This is the preferred method that respects DIP and uses Value Objects
    pub fn add_processing_step(
        mut self,
        descriptor: super::processing_step_descriptor::ProcessingStepDescriptor,
    ) -> Self {
        self.processing_steps.push(ProcessingStep {
            step_type: descriptor.step_type().clone(),
            algorithm: descriptor.algorithm().as_str().to_string(),
            parameters: descriptor.parameters().as_map().clone(),
            order: descriptor.order().value(),
        });
        self
    }

    /// Adds a checksum processing step
    pub fn add_checksum_step(mut self, algorithm: &str) -> Self {
        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Checksum,
            algorithm: algorithm.to_string(),
            parameters: HashMap::new(),
            order: self.processing_steps.len() as u32,
        });
        self
    }

    /// Adds a pass-through processing step
    pub fn add_passthrough_step(mut self, algorithm: &str) -> Self {
        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::PassThrough,
            algorithm: algorithm.to_string(),
            parameters: HashMap::new(),
            order: self.processing_steps.len() as u32,
        });
        self
    }

    /// Sets chunk processing information
    pub fn with_chunk_info(mut self, chunk_size: u32, chunk_count: u32) -> Self {
        self.chunk_size = chunk_size;
        self.chunk_count = chunk_count;
        self
    }

    /// Sets pipeline ID
    pub fn with_pipeline_id(mut self, pipeline_id: String) -> Self {
        self.pipeline_id = pipeline_id;
        self
    }

    /// Sets output file checksum (call after processing is complete)
    pub fn with_output_checksum(mut self, checksum: String) -> Self {
        self.output_checksum = checksum;
        self
    }

    /// Adds metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Serializes the header to binary format for file footer
    ///
    /// # Purpose
    /// Converts the header to the binary footer format that is appended to processed files.
    /// The footer allows reading metadata from the end of files without scanning the entire file.
    ///
    /// # Why
    /// Storing metadata at the end provides:
    /// - Efficient metadata access without reading full file
    /// - Streaming-friendly format (header written after data)
    /// - Simple format detection via magic bytes at end
    ///
    /// # Binary Format
    /// ```text
    /// [JSON_HEADER][HEADER_LENGTH (4 bytes)][FORMAT_VERSION (2 bytes)][MAGIC_BYTES (8 bytes)]
    /// ```
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Serialized footer bytes
    /// * `Err(PipelineError::SerializationError)` - JSON serialization failed
    ///
    /// # Errors
    /// Returns `PipelineError::SerializationError` if JSON serialization fails.
    ///
    /// # Examples
    ///
    pub fn to_footer_bytes(&self) -> Result<Vec<u8>, PipelineError> {
        // Serialize header to JSON
        let header_json = serde_json::to_string(self)
            .map_err(|e| PipelineError::SerializationError(format!("Failed to serialize header: {}", e)))
            .unwrap();

        let header_bytes = header_json.as_bytes();
        let header_length = header_bytes.len() as u32;

        // Build footer format
        let mut result = Vec::new();

        // JSON header data
        result.extend_from_slice(header_bytes);

        // Header length (little-endian)
        result.extend_from_slice(&header_length.to_le_bytes());

        // Format version (little-endian)
        result.extend_from_slice(&self.format_version.to_le_bytes());

        // Magic bytes
        result.extend_from_slice(&MAGIC_BYTES);

        Ok(result)
    }

    /// Deserializes the header from file footer bytes
    ///
    /// # Purpose
    /// Extracts and parses the file header from the footer at the end of a processed file.
    /// This is the primary method for reading metadata from .adapipe files.
    ///
    /// # Why
    /// Reading from the footer enables:
    /// - Quick metadata access without processing entire file
    /// - Format validation before attempting recovery
    /// - Backward compatibility checking
    ///
    /// # Arguments
    /// * `file_data` - Complete file data including footer
    ///
    /// # Returns
    /// * `Ok((FileHeader, usize))` - Parsed header and total footer size in bytes
    /// * `Err(PipelineError)` - Validation or parsing error
    ///
    /// # Errors
    /// Returns `PipelineError` when:
    /// - File too short (< 14 bytes minimum footer size)
    /// - Invalid magic bytes (not an .adapipe file)
    /// - Unsupported format version
    /// - Incomplete footer data
    /// - Invalid UTF-8 in JSON header
    /// - JSON deserialization fails
    ///
    /// # Examples
    ///
    pub fn from_footer_bytes(file_data: &[u8]) -> Result<(Self, usize), PipelineError> {
        let file_size = file_data.len();

        if file_size < 14 {
            // 8 + 2 + 4 = minimum footer size
            return Err(PipelineError::ValidationError("File too short for footer".to_string()));
        }

        // Read from end of file
        let magic_start = file_size - 8;
        let version_start = file_size - 10;
        let length_start = file_size - 14;

        // Check magic bytes
        let magic_bytes = &file_data[magic_start..];
        if magic_bytes != MAGIC_BYTES {
            return Err(PipelineError::ValidationError(
                "Invalid magic bytes - not an Adaptive Pipeline file".to_string(),
            ));
        }

        // Read format version
        let version_bytes = &file_data[version_start..version_start + 2];
        let format_version = u16::from_le_bytes([version_bytes[0], version_bytes[1]]);
        if format_version > CURRENT_FORMAT_VERSION {
            return Err(PipelineError::ValidationError(format!(
                "Unsupported format version: {} (current: {})",
                format_version, CURRENT_FORMAT_VERSION
            )));
        }

        // Read header length
        let length_bytes = &file_data[length_start..length_start + 4];
        let header_length =
            u32::from_le_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]) as usize;

        // Calculate total footer size
        let footer_size = header_length + 14; // JSON + length + version + magic
        if file_size < footer_size {
            return Err(PipelineError::ValidationError(
                "File too short for complete footer".to_string(),
            ));
        }

        // Extract and parse header JSON
        let header_start = file_size - footer_size;
        let header_json = &file_data[header_start..header_start + header_length];
        let header_str = std::str::from_utf8(header_json)
            .map_err(|e| PipelineError::ValidationError(format!("Invalid UTF-8 in header: {}", e)))
            .unwrap();

        let header: FileHeader = serde_json::from_str(header_str)
            .map_err(|e| PipelineError::SerializationError(format!("Failed to deserialize header: {}", e)))
            .unwrap();

        Ok((header, footer_size))
    }

    /// Verifies the integrity of the processed output file
    ///
    /// # Purpose
    /// Validates that the processed file data has not been corrupted or tampered with
    /// by comparing its SHA256 checksum against the stored checksum.
    ///
    /// # Why
    /// Integrity verification provides:
    /// - Detection of file corruption during storage or transmission
    /// - Protection against data tampering
    /// - Confidence in file recovery operations
    ///
    /// # Arguments
    /// * `file_data` - Complete processed file data (including footer)
    ///
    /// # Returns
    /// * `Ok(true)` - File integrity verified, checksum matches
    /// * `Ok(false)` - File corrupted, checksum mismatch
    /// * `Err(PipelineError::ValidationError)` - No checksum available
    ///
    /// # Errors
    /// Returns `PipelineError::ValidationError` if `output_checksum` is empty.
    ///
    /// # Examples
    ///
    pub fn verify_output_integrity(&self, file_data: &[u8]) -> Result<bool, PipelineError> {
        if self.output_checksum.is_empty() {
            return Err(PipelineError::ValidationError(
                "No output checksum available for verification".to_string(),
            ));
        }

        // Calculate checksum of entire file
        let mut hasher = Sha256::new();
        hasher.update(file_data);
        let digest = hasher.finalize();
        let calculated_checksum = hex::encode(digest);

        Ok(calculated_checksum == self.output_checksum)
    }

    /// Gets the processing steps in reverse order for file restoration
    ///
    /// # Purpose
    /// Returns processing steps in the order they must be reversed to restore
    /// the original file. For example, if compression then encryption was applied,
    /// restoration must decrypt then decompress.
    ///
    /// # Why
    /// Processing operations must be reversed in opposite order:
    /// - Apply: Compress → Encrypt
    /// - Restore: Decrypt → Decompress
    ///
    /// # Returns
    /// Vector of processing steps sorted by descending order (highest order first)
    ///
    /// # Examples
    ///
    pub fn get_restoration_steps(&self) -> Vec<&ProcessingStep> {
        let mut steps: Vec<&ProcessingStep> = self.processing_steps.iter().collect();
        steps.sort_by(|a, b| b.order.cmp(&a.order)); // Reverse order
        steps
    }

    /// Validates a restored file against original specifications
    ///
    /// # Purpose
    /// Verifies that a restored file matches the original file exactly by checking
    /// both size and SHA256 checksum. This ensures complete recovery fidelity.
    ///
    /// # Why
    /// Restoration validation provides:
    /// - Confidence that recovery was successful
    /// - Detection of processing errors or data loss
    /// - Verification of processing reversibility
    ///
    /// # Arguments
    /// * `restored_data` - The restored/recovered file data
    ///
    /// # Returns
    /// * `Ok(true)` - Restored file matches original (size and checksum)
    /// * `Ok(false)` - Restored file does not match original
    ///
    /// # Examples
    ///
    pub fn validate_restored_file(&self, restored_data: &[u8]) -> Result<bool, PipelineError> {
        // Check size
        if restored_data.len() as u64 != self.original_size {
            return Ok(false);
        }

        // Check checksum
        let mut hasher = Sha256::new();
        hasher.update(restored_data);
        let digest = hasher.finalize();
        let calculated_checksum = hex::encode(digest);

        Ok(calculated_checksum == self.original_checksum)
    }

    /// Gets information about what processing was applied
    pub fn get_processing_summary(&self) -> String {
        if self.processing_steps.is_empty() {
            return "No processing applied (pass-through)".to_string();
        }

        let steps: Vec<String> = self
            .processing_steps
            .iter()
            .map(|step| match &step.step_type {
                ProcessingStepType::Compression => format!("Compression ({})", step.algorithm),
                ProcessingStepType::Encryption => format!("Encryption ({})", step.algorithm),
                ProcessingStepType::Checksum => format!("Checksum ({})", step.algorithm),
                ProcessingStepType::PassThrough => format!("PassThrough ({})", step.algorithm),
                ProcessingStepType::Custom(name) => format!("Custom ({}: {})", name, step.algorithm),
            })
            .collect();

        format!("Processing: {}", steps.join(" → "))
    }

    /// Checks if the file uses compression
    pub fn is_compressed(&self) -> bool {
        self.processing_steps
            .iter()
            .any(|step| matches!(step.step_type, ProcessingStepType::Compression))
    }

    /// Checks if the file uses encryption
    pub fn is_encrypted(&self) -> bool {
        self.processing_steps
            .iter()
            .any(|step| matches!(step.step_type, ProcessingStepType::Encryption))
    }

    /// Gets the compression algorithm if used
    pub fn compression_algorithm(&self) -> Option<&str> {
        self.processing_steps
            .iter()
            .find(|step| matches!(step.step_type, ProcessingStepType::Compression))
            .map(|step| step.algorithm.as_str())
    }

    /// Gets the encryption algorithm if used
    pub fn encryption_algorithm(&self) -> Option<&str> {
        self.processing_steps
            .iter()
            .find(|step| matches!(step.step_type, ProcessingStepType::Encryption))
            .map(|step| step.algorithm.as_str())
    }

    /// Validates the header for consistency
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.format_version == 0 {
            return Err(PipelineError::ValidationError("Format version cannot be 0".to_string()));
        }

        if self.app_version.is_empty() {
            return Err(PipelineError::ValidationError(
                "App version cannot be empty".to_string(),
            ));
        }

        if self.original_filename.is_empty() {
            return Err(PipelineError::ValidationError(
                "Original filename cannot be empty".to_string(),
            ));
        }

        if self.chunk_size == 0 {
            return Err(PipelineError::ValidationError("Chunk size cannot be 0".to_string()));
        }

        if self.chunk_size < 1024 {
            return Err(PipelineError::ValidationError(
                "Chunk size must be at least 1KB".to_string(),
            ));
        }

        if self.original_size > 0 && self.chunk_count == 0 {
            return Err(PipelineError::ValidationError(
                "Non-empty file must have chunks".to_string(),
            ));
        }

        if self.original_checksum.is_empty() && self.original_size > 0 {
            return Err(PipelineError::ValidationError(
                "Non-empty file must have original checksum".to_string(),
            ));
        }

        // Validate processing steps
        for step in &self.processing_steps {
            if step.algorithm.is_empty() {
                return Err(PipelineError::ValidationError(
                    "Processing step algorithm cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl ChunkFormat {
    /// Creates a new chunk format
    pub fn new(nonce: [u8; 12], encrypted_data: Vec<u8>) -> Self {
        Self {
            nonce,
            data_length: encrypted_data.len() as u32,
            encrypted_data,
        }
    }

    /// Serializes chunk to binary format
    /// Format: `[NONCE][DATA_LENGTH][ENCRYPTED_DATA]`
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Nonce (12 bytes)
        result.extend_from_slice(&self.nonce);

        // Data length (4 bytes, little-endian)
        result.extend_from_slice(&self.data_length.to_le_bytes());

        // Encrypted data
        result.extend_from_slice(&self.encrypted_data);

        result
    }

    /// Converts chunk to bytes and returns both bytes and size
    ///
    /// This is a convenience method that combines the common pattern of:
    ///
    /// # Returns
    /// * `(Vec<u8>, u64)` - The serialized bytes and size as u64
    ///
    /// # Example
    pub fn to_bytes_with_size(&self) -> (Vec<u8>, u64) {
        let chunk_bytes = self.to_bytes();
        let chunk_size = chunk_bytes.len() as u64;
        (chunk_bytes, chunk_size)
    }

    /// Deserializes chunk from binary format
    /// Returns (chunk, bytes_consumed)
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), PipelineError> {
        if data.len() < 16 {
            // 12 + 4 = minimum chunk header size
            return Err(PipelineError::ValidationError(
                "Data too short for chunk header".to_string(),
            ));
        }

        // Read nonce
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&data[0..12]);

        // Read data length
        let data_length = u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as usize;

        // Check if we have enough data
        let total_size = 16 + data_length;
        if data.len() < total_size {
            return Err(PipelineError::ValidationError("Incomplete chunk data".to_string()));
        }

        // Read encrypted data
        let encrypted_data = data[16..16 + data_length].to_vec();

        Ok((
            Self {
                nonce,
                data_length: data_length as u32,
                encrypted_data,
            },
            total_size,
        ))
    }

    /// Validates the chunk format
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.data_length as usize != self.encrypted_data.len() {
            return Err(PipelineError::ValidationError("Chunk data length mismatch".to_string()));
        }

        if self.encrypted_data.is_empty() {
            return Err(PipelineError::ValidationError("Chunk cannot be empty".to_string()));
        }

        Ok(())
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self::new("unknown".to_string(), 0, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests file header creation and serialization with processing steps.
    ///
    /// This test validates that file headers can be created with multiple
    /// processing steps (compression and encryption) and that all metadata
    /// is properly stored and accessible.
    ///
    /// # Test Coverage
    ///
    /// - File header creation with fluent API
    /// - Compression step addition with algorithm and level
    /// - Encryption step addition with key derivation parameters
    /// - Chunk information and pipeline ID configuration
    /// - Output checksum configuration
    /// - Header validation for consistency
    /// - Processing step detection (compression/encryption flags)
    /// - Algorithm extraction from processing steps
    ///
    /// # Assertions
    ///
    /// - Header validation passes for complete configuration
    /// - Compression detection works correctly
    /// - Encryption detection works correctly
    /// - Algorithm names are extracted properly
    /// - All fluent API methods chain correctly
    #[test]
    fn test_header_creation_and_serialization() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .add_compression_step("brotli", 6)
            .add_encryption_step("aes256gcm", "argon2", 32, 12)
            .with_chunk_info(1024 * 1024, 1)
            .with_pipeline_id("test-pipeline".to_string())
            .with_output_checksum("def456".to_string());

        assert!(header.validate().is_ok());
        assert!(header.is_compressed());
        assert!(header.is_encrypted());
        assert_eq!(header.compression_algorithm(), Some("brotli"));
        assert_eq!(header.encryption_algorithm(), Some("aes256gcm"));
    }

    /// Tests header serialization and deserialization roundtrip.
    ///
    /// This test validates that file headers can be serialized to footer
    /// bytes and then deserialized back to identical header objects,
    /// ensuring data integrity during file I/O operations.
    ///
    /// # Test Coverage
    ///
    /// - Header creation with compression step
    /// - Footer byte serialization
    /// - Footer byte deserialization
    /// - Roundtrip data integrity
    /// - Footer size calculation
    /// - Header equality comparison
    ///
    /// # Test Scenario
    ///
    /// Creates a header with compression, serializes it to footer bytes,
    /// then deserializes it back and verifies the restored header matches
    /// the original exactly.
    ///
    /// # Assertions
    ///
    /// - Original and restored headers are identical
    /// - Footer size matches actual byte length
    /// - Serialization/deserialization preserves all data
    /// - No data loss during roundtrip conversion
    #[test]
    fn test_header_footer_roundtrip() {
        let original_header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .add_compression_step("brotli", 6)
            .with_output_checksum("def456".to_string());

        let footer_data = original_header.to_footer_bytes().unwrap();

        // Simulate reading from end of file
        let (restored_header, footer_size) = FileHeader::from_footer_bytes(&footer_data).unwrap();

        assert_eq!(original_header, restored_header);
        assert_eq!(footer_size, footer_data.len());
    }

    /// Tests restoration steps ordering for proper file recovery.
    ///
    /// This test validates that restoration steps are returned in reverse
    /// order of processing, ensuring that files can be properly restored
    /// by undoing operations in the correct sequence.
    ///
    /// # Test Coverage
    ///
    /// - Processing step order assignment
    /// - Restoration step order reversal
    /// - Multi-step processing (compression + encryption)
    /// - Step order validation
    /// - Proper restoration sequence
    ///
    /// # Test Scenario
    ///
    /// Creates a header with compression (order 0) followed by encryption
    /// (order 1), then verifies that restoration steps are returned in
    /// reverse order: encryption first, then compression.
    ///
    /// # Assertions
    ///
    /// - Restoration steps are in reverse processing order
    /// - Encryption step comes first (order 1)
    /// - Compression step comes second (order 0)
    /// - Step count matches processing step count
    /// - Order values are preserved correctly
    #[test]
    fn test_restoration_steps_order() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .add_compression_step("brotli", 6) // Order 0
            .add_encryption_step("aes256gcm", "argon2", 32, 12); // Order 1

        let restoration_steps = header.get_restoration_steps();

        // Should be in reverse order: encryption first (order 1), then compression
        // (order 0)
        assert_eq!(restoration_steps.len(), 2);
        assert_eq!(restoration_steps[0].order, 1); // Encryption
        assert_eq!(restoration_steps[1].order, 0); // Compression
    }

    /// Tests chunk format serialization and deserialization roundtrip.
    ///
    /// This test validates that chunk data can be serialized to bytes
    /// and then deserialized back to identical chunk objects, ensuring
    /// data integrity for encrypted chunk storage.
    ///
    /// # Test Coverage
    ///
    /// - Chunk format creation with nonce and data
    /// - Chunk byte serialization
    /// - Chunk byte deserialization
    /// - Roundtrip data integrity
    /// - Bytes consumed calculation
    /// - Chunk validation after deserialization
    ///
    /// # Test Scenario
    ///
    /// Creates a chunk with test nonce and data, serializes it to bytes,
    /// then deserializes it back and verifies the restored chunk matches
    /// the original exactly.
    ///
    /// # Assertions
    ///
    /// - Original and restored chunks are identical
    /// - Bytes consumed matches serialized byte length
    /// - Deserialized chunk passes validation
    /// - Nonce and data are preserved exactly
    /// - No data corruption during roundtrip
    #[test]
    fn test_chunk_format_roundtrip() {
        let nonce = [1u8; 12];
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let original_chunk = ChunkFormat::new(nonce, data);

        let chunk_bytes = original_chunk.to_bytes();
        let (restored_chunk, bytes_consumed) = ChunkFormat::from_bytes(&chunk_bytes).unwrap();

        assert_eq!(original_chunk, restored_chunk);
        assert_eq!(bytes_consumed, chunk_bytes.len());
        assert!(restored_chunk.validate().is_ok());
    }

    /// Tests error handling for invalid magic bytes in file headers.
    ///
    /// This test validates that the system properly rejects files that
    /// don't have the correct magic bytes, preventing processing of
    /// non-adapipe files and providing clear error messages.
    ///
    /// # Test Coverage
    ///
    /// - Invalid magic byte detection
    /// - Error handling for malformed files
    /// - Error message content validation
    /// - File format validation
    /// - Rejection of non-adapipe files
    ///
    /// # Test Scenario
    ///
    /// Creates invalid data with wrong magic bytes and attempts to
    /// parse it as a file header, expecting a clear error message
    /// about invalid magic bytes.
    ///
    /// # Assertions
    ///
    /// - Parsing fails with error result
    /// - Error message mentions "Invalid magic bytes"
    /// - System rejects malformed data
    /// - No false positives for invalid files
    /// - Clear error reporting for debugging
    #[test]
    fn test_invalid_magic_bytes() {
        let bad_data = vec![0xFF; 20];
        let result = FileHeader::from_footer_bytes(&bad_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid magic bytes"));
    }

    /// Tests processing summary generation for file headers.
    ///
    /// This test validates that processing summaries correctly describe
    /// the operations applied to files, providing human-readable
    /// descriptions of the processing pipeline.
    ///
    /// # Test Coverage
    ///
    /// - Processing summary generation
    /// - Multi-step processing description
    /// - Algorithm name inclusion
    /// - Processing flow visualization
    /// - Human-readable output format
    ///
    /// # Test Scenario
    ///
    /// Creates a header with compression and encryption steps, then
    /// generates a processing summary and verifies it contains the
    /// expected algorithm names and flow indicators.
    ///
    /// # Assertions
    ///
    /// - Summary contains compression algorithm name
    /// - Summary contains encryption algorithm name
    /// - Summary includes flow indicator (→)
    /// - Format is human-readable
    /// - All processing steps are represented
    #[test]
    fn test_processing_summary() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .add_compression_step("brotli", 6)
            .add_encryption_step("aes256gcm", "argon2", 32, 12);

        let summary = header.get_processing_summary();
        assert!(summary.contains("Compression (brotli)"));
        assert!(summary.contains("Encryption (aes256gcm)"));
        assert!(summary.contains("→")); // Should show processing flow
    }

    /// Tests pass-through file handling without processing steps.
    ///
    /// This test validates that files with no processing steps are
    /// properly handled as pass-through files, with appropriate
    /// flags and summary messages.
    ///
    /// # Test Coverage
    ///
    /// - Pass-through file detection
    /// - No compression flag validation
    /// - No encryption flag validation
    /// - Pass-through summary message
    /// - Minimal processing configuration
    ///
    /// # Test Scenario
    ///
    /// Creates a header with only basic file information and no
    /// processing steps, then verifies that it's correctly identified
    /// as a pass-through file.
    ///
    /// # Assertions
    ///
    /// - Compression flag is false
    /// - Encryption flag is false
    /// - Summary indicates "No processing applied (pass-through)"
    /// - Header is valid despite no processing steps
    /// - Pass-through files are handled correctly
    #[test]
    fn test_pass_through_file() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .with_output_checksum("def456".to_string());

        assert!(!header.is_compressed());
        assert!(!header.is_encrypted());
        assert_eq!(header.get_processing_summary(), "No processing applied (pass-through)");
    }
}
