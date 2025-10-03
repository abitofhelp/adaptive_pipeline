// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # File Processor Service Interface
//!
//! This module defines the domain service interface for file processing
//! operations within the adaptive pipeline system. It provides abstractions for
//! coordinating file processing workflows, chunk management, and processing
//! statistics.
//!
//! ## Overview
//!
//! The file processor service provides:
//!
//! - **File Processing Coordination**: Orchestrates file processing workflows
//! - **Chunk Management**: Manages file chunking and chunk processing
//! - **Processing Statistics**: Collects and reports processing metrics
//! - **Error Handling**: Comprehensive error handling and recovery
//! - **Parallel Processing**: Support for parallel chunk processing
//!
//! ## Architecture
//!
//! The service follows Domain-Driven Design principles:
//!
//! - **Domain Interface**: `FileProcessorService` trait defines the contract
//! - **Configuration**: `FileProcessorConfig` encapsulates processing
//!   parameters
//! - **Chunk Processing**: `ChunkProcessor` trait for pluggable processing
//!   logic
//! - **Statistics**: Comprehensive processing statistics and metrics
//!
//! ## Key Features
//!
//! ### File Processing Workflow
//!
//! - **File Analysis**: Analyze files to determine optimal processing strategy
//! - **Chunk Creation**: Divide files into appropriately sized chunks
//! - **Parallel Processing**: Process chunks concurrently for better
//!   performance
//! - **Result Aggregation**: Collect and aggregate processing results
//!
//! ### Chunk Processing
//!
//! - **Pluggable Processors**: Support for custom chunk processing logic
//! - **Processing Pipeline**: Chain multiple processors for complex workflows
//! - **Error Isolation**: Isolate errors to individual chunks when possible
//! - **Progress Tracking**: Real-time progress monitoring and reporting
//!
//! ### Performance Optimization
//!
//! - **Adaptive Chunking**: Dynamic chunk size adjustment based on performance
//! - **Memory Management**: Efficient memory usage with chunk recycling
//! - **Parallel Execution**: Configurable parallel processing capabilities
//! - **Resource Management**: Intelligent resource allocation and cleanup
//!
//! ## Usage Examples
//!
//! ### Basic File Processing

//!
//! ### Custom Chunk Processor

//!
//! ### Parallel Processing

//!
//! ## Configuration
//!
//! ### File Processor Configuration
//!
//! The service behavior is controlled through `FileProcessorConfig`:
//!
//! - **File Size Limits**: Maximum file size for processing
//! - **Chunk Size**: Preferred chunk size for processing
//! - **Memory Mapping**: Enable/disable memory mapping for large files
//! - **Concurrency**: Maximum number of concurrent file operations
//! - **Integrity Verification**: Enable/disable file integrity checks
//! - **Temporary Directory**: Location for intermediate processing files
//!
//! ### Performance Tuning
//!
//! - **Chunk Size**: Optimize chunk size based on processing characteristics
//! - **Concurrency**: Balance concurrency with system resources
//! - **Memory Mapping**: Use memory mapping for large files
//! - **Buffer Management**: Efficient buffer allocation and reuse
//!
//! ## Processing Statistics
//!
//! ### Collected Metrics
//!
//! - **Processing Time**: Total and per-chunk processing times
//! - **Throughput**: Processing throughput in bytes/second
//! - **Chunk Statistics**: Number of chunks processed and their sizes
//! - **Error Rates**: Processing error rates and failure analysis
//! - **Resource Usage**: Memory and CPU usage during processing
//!
//! ### Performance Analysis
//!
//! - **Bottleneck Identification**: Identify processing bottlenecks
//! - **Optimization Recommendations**: Suggest configuration optimizations
//! - **Trend Analysis**: Track performance trends over time
//!
//! ## Error Handling
//!
//! ### Processing Errors
//!
//! - **Chunk-Level Errors**: Isolate errors to individual chunks
//! - **File-Level Errors**: Handle file-level processing failures
//! - **System Errors**: Handle system resource and I/O errors
//! - **Configuration Errors**: Validate configuration parameters
//!
//! ### Recovery Strategies
//!
//! - **Retry Logic**: Automatic retry for transient failures
//! - **Partial Processing**: Continue processing unaffected chunks
//! - **Fallback Processing**: Alternative processing strategies
//! - **Error Reporting**: Detailed error context and suggestions
//!
//! ## Integration
//!
//! The file processor service integrates with:
//!
//! - **File I/O Service**: Uses file I/O service for reading and writing
//! - **Chunk Processors**: Coordinates with pluggable chunk processors
//! - **Pipeline Service**: Integrated into pipeline processing workflow
//! - **Metrics Service**: Reports processing metrics and statistics
//!
//! ## Thread Safety
//!
//! The service interface is designed for thread safety:
//!
//! - **Concurrent Processing**: Safe concurrent processing of multiple files
//! - **Shared Resources**: Safe sharing of processing resources
//! - **State Management**: Thread-safe state management and coordination
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Streaming Processing**: Real-time streaming file processing
//! - **Distributed Processing**: Support for distributed chunk processing
//! - **Adaptive Optimization**: Automatic optimization based on performance
//! - **Advanced Scheduling**: Sophisticated chunk scheduling strategies

use crate::{FileChunk, PipelineError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
// Note: FileIOService imports moved to infrastructure layer to maintain Clean
// Architecture

// NOTE: FileProcessorService is async (infrastructure port - involves I/O operations).
// ChunkProcessor is synchronous (domain service - CPU-bound processing).
// See file_io_service.rs for explanation of I/O-bound vs CPU-bound async decisions.

/// Configuration for file processing operations
///
/// This struct encapsulates all configuration parameters for file processing
/// operations, providing control over performance, resource usage, and
/// behavior.
///
/// # Key Configuration Areas
///
/// - **File Limits**: Maximum file size and processing constraints
/// - **Chunk Processing**: Chunk size and chunking behavior
/// - **Memory Management**: Memory mapping and resource allocation
/// - **Concurrency**: Parallel processing and resource limits
/// - **Integrity**: File integrity verification settings
/// - **Storage**: Temporary file and directory management
///
/// # Examples
///
#[derive(Debug, Clone)]
pub struct FileProcessorConfig {
    /// Maximum file size to process (in bytes)
    pub max_file_size: u64,
    /// Preferred chunk size for processing
    pub processing_chunk_size: usize,
    /// Whether to use memory mapping for large files
    pub use_memory_mapping: bool,
    /// Maximum number of concurrent file operations
    pub max_concurrent_files: usize,
    /// Whether to verify file integrity before processing
    pub verify_integrity: bool,
    /// Temporary directory for intermediate files
    pub temp_dir: Option<std::path::PathBuf>,
}

impl Default for FileProcessorConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024 * 1024, // 10GB
            processing_chunk_size: 1024 * 1024,     // 1MB
            use_memory_mapping: true,
            max_concurrent_files: 4,
            verify_integrity: true,
            temp_dir: None,
        }
    }
}

/// Statistics for file processing operations
#[derive(Debug, Clone, Default)]
pub struct FileProcessingStats {
    /// Total files processed
    pub files_processed: u64,
    /// Total bytes processed
    pub bytes_processed: u64,
    /// Total processing time in milliseconds
    pub total_processing_time_ms: u64,
    /// Number of files that used memory mapping
    pub memory_mapped_files: u64,
    /// Number of integrity check failures
    pub integrity_failures: u64,
    /// Number of processing errors
    pub processing_errors: u64,
    /// Average processing speed (bytes per second)
    pub avg_processing_speed: f64,
}

/// Result of file processing operation
#[derive(Debug)]
pub struct FileProcessingResult {
    /// Input file path
    pub input_path: std::path::PathBuf,
    /// Output file path (if applicable)
    pub output_path: Option<std::path::PathBuf>,
    /// Number of chunks processed
    pub chunks_processed: u64,
    /// Total bytes processed
    pub bytes_processed: u64,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Whether memory mapping was used
    pub used_memory_mapping: bool,
    /// File integrity status
    pub integrity_verified: bool,
    /// Processing metadata
    pub metadata: HashMap<String, String>,
}

/// Trait for processing files with the pipeline system
#[async_trait]
pub trait FileProcessorService: Send + Sync {
    /// Processes a single file through the pipeline
    async fn process_file(
        &self,
        input_path: &Path,
        output_path: Option<&Path>,
        processor: Box<dyn ChunkProcessor>,
    ) -> Result<FileProcessingResult, PipelineError>;

    /// Processes multiple files concurrently
    async fn process_files_batch(
        &self,
        file_pairs: Vec<(std::path::PathBuf, Option<std::path::PathBuf>)>,
        processor: Box<dyn ChunkProcessor>,
    ) -> Result<Vec<FileProcessingResult>, PipelineError>;

    /// Processes a file in-place (modifying the original)
    async fn process_file_in_place(
        &self,
        file_path: &Path,
        processor: Box<dyn ChunkProcessor>,
    ) -> Result<FileProcessingResult, PipelineError>;

    /// Validates file integrity before processing
    async fn validate_file_before_processing(&self, file_path: &Path) -> Result<bool, PipelineError>;

    /// Gets processing statistics
    fn get_processing_stats(&self) -> FileProcessingStats;

    /// Resets processing statistics
    fn reset_processing_stats(&mut self);

    /// Gets the current configuration
    fn get_config(&self) -> FileProcessorConfig;

    /// Updates the configuration
    fn update_config(&mut self, config: FileProcessorConfig);
}

/// Trait for processing individual file chunks
///
/// ## Developer Notes - Immutable Processing Pattern
/// This trait follows DDD Value Object principles where FileChunk is immutable.
/// Instead of mutating chunks, processors return new chunk instances.
/// This ensures data integrity and prevents accidental mutations.
///
/// ## Architecture Note - Synchronous Domain Service
///
/// This trait is **synchronous** following DDD principles. Chunk processing
/// is CPU-bound (compression, encryption, checksums), not I/O-bound.
/// The domain layer defines *what* operations exist, not *how* they execute.
///
/// For async contexts, infrastructure adapters can wrap chunk processors
/// using `tokio::spawn_blocking` or similar mechanisms.
///
/// ### Usage Pattern:
pub trait ChunkProcessor: Send + Sync {
    /// Processes a single chunk of data and returns a new processed chunk
    ///
    /// # Arguments
    /// * `chunk` - The input chunk to process (immutable reference)
    ///
    /// # Returns
    /// * `Ok(FileChunk)` - New chunk with processing results
    /// * `Err(PipelineError)` - Processing failed
    ///
    /// # Developer Notes
    /// - Input chunk is never modified (immutability)
    /// - Return new chunk with changes applied
    /// - Use chunk.with_data() or chunk.with_checksum() for modifications
    ///
    /// # Note on Async
    ///
    /// This method is synchronous (CPU-bound operations). For async contexts,
    /// use infrastructure adapters that wrap this in `tokio::spawn_blocking`.
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError>;

    /// Returns the processor name for logging/debugging
    fn name(&self) -> &str;

    /// Returns whether this processor modifies chunk data
    fn modifies_data(&self) -> bool;

    /// Returns whether this processor requires sequential processing
    fn requires_sequential_processing(&self) -> bool {
        false
    }
}

/// Generic service adapter for chunk processing
/// This adapter allows any service implementing the appropriate trait to be
/// used as a ChunkProcessor
#[allow(dead_code)]
pub struct ServiceAdapter<T> {
    service: T,
    name: String,
}

impl<T> ServiceAdapter<T> {
    pub fn new(service: T, name: String) -> Self {
        Self { service, name }
    }
}

// Note: Specific ChunkProcessor implementations for
// ServiceAdapter<CompressionService> and ServiceAdapter<EncryptionService>
// should be implemented in the infrastructure layer to maintain proper
// dependency direction and Clean Architecture principles.

/// Processor that applies multiple processors in sequence
pub struct ChainProcessor {
    pub processors: Vec<Box<dyn ChunkProcessor>>,
}

impl ChunkProcessor for ChainProcessor {
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        let mut current_chunk = chunk.clone();
        for processor in &self.processors {
            current_chunk = processor.process_chunk(&current_chunk)?;
        }
        Ok(current_chunk)
    }

    fn name(&self) -> &str {
        "ChainProcessor"
    }

    fn modifies_data(&self) -> bool {
        self.processors.iter().any(|p| p.modifies_data())
    }

    fn requires_sequential_processing(&self) -> bool {
        self.processors.iter().any(|p| p.requires_sequential_processing())
    }
}
