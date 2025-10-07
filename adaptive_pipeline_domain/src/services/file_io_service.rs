// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # File I/O Service Interface
//!
//! Domain service trait for efficient file operations with chunked reading,
//! memory mapping for large files, streaming support, and async I/O.
//! Configurable chunk size, buffer size, and concurrency limits. Provides
//! checksum verification, metadata extraction, and comprehensive error
//! handling. Thread-safe operations. See mdBook for configuration and
//! optimization strategies.
//! - **Partial Results**: Return partial results when possible
//! - **Resource Cleanup**: Automatic cleanup on errors
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Streaming**: Process files without loading entirely into memory
//! - **Memory Mapping**: Efficient memory usage for large files
//! - **Buffer Management**: Efficient buffer allocation and reuse
//!
//! ### I/O Optimization
//!
//! - **Sequential Access**: Optimize for sequential file access patterns
//! - **Prefetching**: Intelligent prefetching for better performance
//! - **Caching**: File system cache utilization
//!
//! ## Integration
//!
//! The file I/O service integrates with:
//!
//! - **File Processor**: Used by file processor for chunk-based processing
//! - **Pipeline Service**: Integrated into pipeline processing workflow
//! - **Storage Systems**: Abstracts various storage backend implementations
//! - **Monitoring**: Provides metrics for I/O operations
//!
//! ## Thread Safety
//!
//! The service interface is designed for thread safety:
//!
//! - **Concurrent Operations**: Safe concurrent access to file operations
//! - **Resource Sharing**: Safe sharing of file handles and resources
//! - **State Management**: Thread-safe state management
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Compression**: Built-in compression for file operations
//! - **Encryption**: Transparent encryption/decryption during I/O
//! - **Network Storage**: Support for network-based storage systems
//! - **Caching**: Intelligent caching layer for frequently accessed files
//!
//! ## Architecture Note - Infrastructure Port
//!
//! **Important:** This service trait is **async** and represents an
//! **infrastructure port**, not a pure domain service. This is an intentional
//! exception to the "domain traits should be sync" principle.
//!
//! ### Why FileIOService is Async
//!
//! File I/O operations are inherently I/O-bound, not CPU-bound:
//! - **I/O-Bound Operations**: File operations involve waiting for disk I/O
//! - **Non-Blocking Benefits**: Async I/O prevents blocking the runtime
//! - **tokio Integration**: Async file operations integrate naturally with
//!   tokio
//! - **Performance**: Async I/O provides better concurrency for I/O operations
//!
//! ### Architectural Classification
//!
//! This trait is classified as an **infrastructure port** rather than a domain
//! service:
//! - **Domain Services**: CPU-bound business logic (compression, encryption,
//!   checksums)
//! - **Infrastructure Ports**: I/O-bound operations (file I/O, network,
//!   database)
//!
//! ### Design Trade-offs
//!
//! We considered making this sync (using std::fs) but chose async because:
//! 1. Most of the application uses tokio async runtime
//! 2. File operations benefit from non-blocking I/O
//! 3. Alternative would be to use blocking thread pool, adding complexity
//! 4. The trait is already an infrastructure concern (port/interface)
//!
//! ### References
//!
//! See REFACTORING_STATUS.md Phase 1, item 2 for full discussion.

use crate::{FileChunk, PipelineError};
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

/// Configuration for file I/O operations
///
/// This struct encapsulates all configuration parameters for file I/O
/// operations, providing fine-grained control over performance, memory usage,
/// and behavior.
///
/// # Key Configuration Areas
///
/// - **Chunk Processing**: Default chunk size and chunking behavior
/// - **Memory Management**: Memory mapping thresholds and buffer sizes
/// - **Concurrency**: Limits on concurrent operations
/// - **Verification**: Checksum verification settings
/// - **Performance**: Various performance optimization settings
///
/// # Examples
#[derive(Debug, Clone)]
pub struct FileIOConfig {
    /// Default chunk size for reading files
    pub default_chunk_size: usize,
    /// Maximum file size for memory mapping (in bytes)
    pub max_mmap_size: u64,
    /// Whether to use memory mapping for large files
    pub enable_memory_mapping: bool,
    /// Buffer size for streaming operations
    pub buffer_size: usize,
    /// Whether to verify checksums during read operations
    pub verify_checksums: bool,
    /// Maximum number of concurrent file operations
    pub max_concurrent_operations: usize,
}

impl Default for FileIOConfig {
    fn default() -> Self {
        Self {
            default_chunk_size: 1024 * 1024,   // 1MB (matches ChunkSize minimum)
            max_mmap_size: 1024 * 1024 * 1024, // 1GB
            enable_memory_mapping: true,
            buffer_size: 8192, // 8KB
            verify_checksums: true,
            max_concurrent_operations: 10,
        }
    }
}

/// Information about a file being processed
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File path
    pub path: std::path::PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Whether the file is memory-mapped
    pub is_memory_mapped: bool,
    /// File modification time
    pub modified_at: std::time::SystemTime,
    /// File creation time
    pub created_at: std::time::SystemTime,
    /// File permissions (Unix-style)
    pub permissions: u32,
    /// MIME type if detectable
    pub mime_type: Option<String>,
}

/// Statistics for file I/O operations
#[derive(Debug, Clone, Default)]
pub struct FileIOStats {
    /// Total bytes read
    pub bytes_read: u64,
    /// Total bytes written
    pub bytes_written: u64,
    /// Number of chunks processed
    pub chunks_processed: u64,
    /// Number of files processed
    pub files_processed: u64,
    /// Number of memory-mapped files
    pub memory_mapped_files: u64,
    /// Total processing time in milliseconds
    pub total_processing_time_ms: u64,
    /// Number of checksum verifications
    pub checksum_verifications: u64,
    /// Number of failed operations
    pub failed_operations: u64,
}

/// Options for reading files
#[derive(Debug, Clone)]
pub struct ReadOptions {
    /// Chunk size for reading
    pub chunk_size: Option<usize>,
    /// Starting offset
    pub start_offset: Option<u64>,
    /// Maximum bytes to read
    pub max_bytes: Option<u64>,
    /// Whether to calculate checksums
    pub calculate_checksums: bool,
    /// Whether to use memory mapping if available
    pub use_memory_mapping: bool,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            chunk_size: None,
            start_offset: None,
            max_bytes: None,
            calculate_checksums: true,
            use_memory_mapping: true,
        }
    }
}

/// Options for writing files
#[derive(Debug, Clone)]
pub struct WriteOptions {
    /// Whether to append to existing file
    pub append: bool,
    /// Whether to create parent directories
    pub create_dirs: bool,
    /// File permissions to set
    pub permissions: Option<u32>,
    /// Whether to sync to disk immediately
    pub sync: bool,
    /// Whether to calculate checksums
    pub calculate_checksums: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            append: false,
            create_dirs: true,
            permissions: None,
            sync: false,
            calculate_checksums: true,
        }
    }
}

/// Result of a file read operation
#[derive(Debug)]
pub struct ReadResult {
    /// File chunks read
    pub chunks: Vec<FileChunk>,
    /// File information
    pub file_info: FileInfo,
    /// Total bytes read
    pub bytes_read: u64,
    /// Whether the entire file was read
    pub complete: bool,
}

/// Result of a file write operation
#[derive(Debug)]
pub struct WriteResult {
    /// File path written to
    pub path: std::path::PathBuf,
    /// Total bytes written
    pub bytes_written: u64,
    /// File checksum if calculated
    pub checksum: Option<String>,
    /// Whether the operation was successful
    pub success: bool,
}

/// Trait for file I/O operations with memory mapping support
#[async_trait]
pub trait FileIOService: Send + Sync {
    /// Reads a file and returns it as chunks
    async fn read_file_chunks(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError>;

    /// Reads a file using memory mapping if possible
    async fn read_file_mmap(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError>;

    /// Writes chunks to a file
    async fn write_file_chunks(
        &self,
        path: &Path,
        chunks: &[FileChunk],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError>;

    /// Writes data directly to a file
    async fn write_file_data(
        &self,
        path: &Path,
        data: &[u8],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError>;

    /// Gets information about a file
    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, PipelineError>;

    /// Checks if a file exists
    async fn file_exists(&self, path: &Path) -> Result<bool, PipelineError>;

    /// Deletes a file
    async fn delete_file(&self, path: &Path) -> Result<(), PipelineError>;

    /// Copies a file
    async fn copy_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError>;

    /// Moves a file
    async fn move_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError>;

    /// Creates a directory
    async fn create_directory(&self, path: &Path) -> Result<(), PipelineError>;

    /// Checks if a directory exists
    async fn directory_exists(&self, path: &Path) -> Result<bool, PipelineError>;

    /// Lists files in a directory
    async fn list_directory(&self, path: &Path) -> Result<Vec<FileInfo>, PipelineError>;

    /// Gets the current configuration
    fn get_config(&self) -> FileIOConfig;

    /// Updates the configuration
    fn update_config(&mut self, config: FileIOConfig);

    /// Gets I/O statistics
    fn get_stats(&self) -> FileIOStats;

    /// Resets I/O statistics
    fn reset_stats(&mut self);

    /// Validates file integrity using checksums
    async fn validate_file_integrity(&self, path: &Path, expected_checksum: &str) -> Result<bool, PipelineError>;

    /// Calculates file checksum
    async fn calculate_file_checksum(&self, path: &Path) -> Result<String, PipelineError>;

    /// Streams file chunks for processing
    async fn stream_file_chunks(
        &self,
        path: &Path,
        options: ReadOptions,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<FileChunk, PipelineError>> + Send>>, PipelineError>;

    /// Writes a single chunk to a file (for streaming writes)
    async fn write_chunk_to_file(
        &self,
        path: &Path,
        chunk: &FileChunk,
        options: WriteOptions,
        is_first_chunk: bool,
    ) -> Result<WriteResult, PipelineError>;
}

/// Implementation of FileIOService for `Arc<dyn FileIOService>`
/// This enables shared ownership of FileIOService trait objects
#[async_trait]
impl FileIOService for Arc<dyn FileIOService> {
    async fn read_file_chunks(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError> {
        (**self).read_file_chunks(path, options).await
    }

    async fn read_file_mmap(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError> {
        (**self).read_file_mmap(path, options).await
    }

    async fn write_file_chunks(
        &self,
        path: &Path,
        chunks: &[FileChunk],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        (**self).write_file_chunks(path, chunks, options).await
    }

    async fn write_file_data(
        &self,
        path: &Path,
        data: &[u8],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        (**self).write_file_data(path, data, options).await
    }

    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, PipelineError> {
        (**self).get_file_info(path).await
    }

    async fn file_exists(&self, path: &Path) -> Result<bool, PipelineError> {
        (**self).file_exists(path).await
    }

    async fn delete_file(&self, path: &Path) -> Result<(), PipelineError> {
        (**self).delete_file(path).await
    }

    async fn copy_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        (**self).copy_file(source, destination, options).await
    }

    async fn move_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        (**self).move_file(source, destination, options).await
    }

    async fn create_directory(&self, path: &Path) -> Result<(), PipelineError> {
        (**self).create_directory(path).await
    }

    async fn directory_exists(&self, path: &Path) -> Result<bool, PipelineError> {
        (**self).directory_exists(path).await
    }

    async fn list_directory(&self, path: &Path) -> Result<Vec<FileInfo>, PipelineError> {
        (**self).list_directory(path).await
    }

    fn get_config(&self) -> FileIOConfig {
        (**self).get_config()
    }

    fn update_config(&mut self, _config: FileIOConfig) {
        // Note: This is a no-op for Arc since we can't get mutable access
        // In practice, config updates should be done through the concrete type
        // This is intentionally a no-op to avoid panicking in production code
    }

    fn get_stats(&self) -> FileIOStats {
        (**self).get_stats()
    }

    fn reset_stats(&mut self) {
        // Note: This is a no-op for Arc since we can't get mutable access
        // In practice, stats resets should be done through the concrete type
        // This is intentionally a no-op to avoid panicking in production code
    }

    async fn validate_file_integrity(&self, path: &Path, expected_checksum: &str) -> Result<bool, PipelineError> {
        (**self).validate_file_integrity(path, expected_checksum).await
    }

    async fn calculate_file_checksum(&self, path: &Path) -> Result<String, PipelineError> {
        (**self).calculate_file_checksum(path).await
    }

    async fn stream_file_chunks(
        &self,
        path: &Path,
        options: ReadOptions,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<FileChunk, PipelineError>> + Send>>, PipelineError>
    {
        (**self).stream_file_chunks(path, options).await
    }

    async fn write_chunk_to_file(
        &self,
        path: &Path,
        chunk: &FileChunk,
        options: WriteOptions,
        is_first_chunk: bool,
    ) -> Result<WriteResult, PipelineError> {
        (**self).write_chunk_to_file(path, chunk, options, is_first_chunk).await
    }
}
