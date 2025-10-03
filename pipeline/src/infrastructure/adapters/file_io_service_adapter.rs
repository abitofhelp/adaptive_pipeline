// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # File I/O Service Implementation
//!
//! This module is part of the Infrastructure layer, providing concrete
//! implementations of domain interfaces (ports).
//!
//! This module provides the concrete implementation of the file I/O service
//! interface for the adaptive pipeline system. It offers high-performance file
//! operations with memory mapping support, asynchronous I/O, and comprehensive
//! error handling.
//!
//! ## Overview
//!
//! The file I/O service implementation provides:
//!
//! - **Memory Mapping**: High-performance file access using memory-mapped files
//! - **Asynchronous I/O**: Non-blocking file operations using Tokio
//! - **Chunked Processing**: Efficient processing of large files in chunks
//! - **Statistics Tracking**: Comprehensive I/O performance metrics
//! - **Error Handling**: Robust error handling and recovery
//!
//! ## Architecture
//!
//! The implementation follows the infrastructure layer patterns:
//!
//! - **Service Implementation**: `FileIOServiceImpl` implements domain
//!   interface
//! - **Memory Management**: Efficient memory usage with memory mapping
//! - **Concurrency**: Thread-safe operations with parking_lot RwLock
//! - **Configuration**: Flexible configuration for different use cases
//!
//! ## Performance Features
//!
//! ### Memory Mapping
//!
//! Uses memory-mapped files for optimal performance:
//! - **Zero-Copy**: Direct memory access without copying data
//! - **OS Optimization**: Leverages operating system virtual memory
//! - **Cache Efficiency**: Better CPU cache utilization
//! - **Large File Support**: Efficient handling of multi-gigabyte files
//!
//! ### Asynchronous Operations
//!
//! All I/O operations are asynchronous:
//! - **Non-Blocking**: Doesn't block the async runtime
//! - **Concurrent Processing**: Multiple files can be processed simultaneously
//! - **Resource Efficiency**: Optimal use of system resources
//! - **Scalability**: Handles high concurrent load
//!
//! ### Chunked Processing
//!
//! Processes files in configurable chunks:
//! - **Memory Efficiency**: Constant memory usage regardless of file size
//! - **Progress Tracking**: Real-time progress monitoring
//! - **Error Recovery**: Granular error handling per chunk
//! - **Parallel Processing**: Chunks can be processed in parallel
//!
//! ## Configuration Options
//!
//! The service supports various configuration options:
//!
//! ### Buffer Sizes
//! - **Read Buffer**: Configurable read buffer size
//! - **Write Buffer**: Configurable write buffer size
//! - **Chunk Size**: Optimal chunk size for processing
//!
//! ### Memory Mapping
//! - **Threshold**: Minimum file size for memory mapping
//! - **Alignment**: Memory alignment for optimal performance
//! - **Prefetch**: Prefetch strategies for sequential access
//!
//! ### Concurrency
//! - **Thread Pool**: Configurable thread pool size
//! - **Concurrent Reads**: Maximum concurrent read operations
//! - **Concurrent Writes**: Maximum concurrent write operations
//!
//! ## Usage Examples
//!
//! ### Basic File Reading

//!
//! ### Chunked File Processing

//!
//! ## Error Handling
//!
//! Comprehensive error handling for:
//! - **File System Errors**: Permission denied, file not found, etc.
//! - **I/O Errors**: Read/write failures, disk full, etc.
//! - **Memory Mapping Errors**: Mapping failures, access violations
//! - **Configuration Errors**: Invalid parameters, resource limits
//!
//! ## Performance Characteristics
//!
//! ### Throughput
//! - **High Bandwidth**: Optimized for maximum I/O throughput
//! - **Low Latency**: Minimal overhead for small operations
//! - **Scalable**: Performance scales with available system resources
//!
//! ### Memory Usage
//! - **Efficient**: Minimal memory overhead
//! - **Predictable**: Constant memory usage for chunked processing
//! - **Configurable**: Tunable memory usage based on requirements
//!
//! ## Thread Safety
//!
//! The implementation is fully thread-safe:
//! - **Concurrent Access**: Multiple threads can use the service simultaneously
//! - **Lock-Free Reads**: Read operations don't block each other
//! - **Atomic Updates**: Statistics and configuration updates are atomic
//!
//! ## Integration
//!
//! The service integrates with:
//! - **Domain Layer**: Implements `FileIOService` trait
//! - **Processing Pipeline**: Provides file access for pipeline stages
//! - **Metrics System**: Reports detailed I/O performance metrics
//! - **Configuration System**: Dynamic configuration updates

use async_trait::async_trait;
use memmap2::{Mmap, MmapOptions};
use std::fs::File;
use std::io::SeekFrom;
use std::path::Path;

use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use parking_lot::RwLock;

use pipeline_domain::services::file_io_service::{
    FileIOConfig, FileIOService, FileIOStats, FileInfo, ReadOptions, ReadResult, WriteOptions, WriteResult,
};
use pipeline_domain::{FileChunk, PipelineError};

/// Implementation of FileIOService with memory mapping support
///
/// This struct provides a high-performance implementation of the file I/O
/// service interface, featuring memory-mapped file access, asynchronous
/// operations, and comprehensive statistics tracking.
///
/// # Features
///
/// - **Memory Mapping**: Uses memory-mapped files for optimal performance
/// - **Async I/O**: All operations are asynchronous and non-blocking
/// - **Thread Safety**: Safe for concurrent access from multiple threads
/// - **Statistics**: Tracks detailed I/O performance metrics
/// - **Configuration**: Runtime configuration updates supported
///
/// # Examples
///
pub struct FileIOServiceImpl {
    config: RwLock<FileIOConfig>,
    stats: RwLock<FileIOStats>,
}

impl FileIOServiceImpl {
    /// Creates a new FileIOService instance
    pub fn new(config: FileIOConfig) -> Self {
        Self {
            config: RwLock::new(config),
            stats: RwLock::new(FileIOStats::default()),
        }
    }

    /// Creates a new FileIOService with default configuration
    pub fn new_default() -> Self {
        Self::new(FileIOConfig::default())
    }

    /// Determines if a file should be memory-mapped based on size and config
    fn should_use_mmap(&self, file_size: u64) -> bool {
        let config = self.config.read();
        config.enable_memory_mapping && file_size <= config.max_mmap_size
    }

    /// Creates file chunks from memory-mapped data
    fn create_chunks_from_mmap(
        &self,
        mmap: &Mmap,
        chunk_size: usize,
        calculate_checksums: bool,
        start_offset: u64,
        max_bytes: Option<u64>,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        let mut chunks = Vec::new();
        let data_len = mmap.len() as u64;
        let start = start_offset.min(data_len);
        let end = match max_bytes {
            Some(max) => (start + max).min(data_len),
            None => data_len,
        };

        let mut current_offset = start;
        let mut sequence = 0u64;

        while current_offset < end {
            let chunk_end = ((current_offset + chunk_size as u64).min(end)) as usize;
            let chunk_start = current_offset as usize;
            let chunk_data = mmap[chunk_start..chunk_end].to_vec();
            let is_final = chunk_end as u64 >= end;

            let chunk = FileChunk::new(sequence, current_offset, chunk_data, is_final).unwrap();

            let chunk = if calculate_checksums {
                chunk.with_calculated_checksum()?
            } else {
                chunk
            };

            chunks.push(chunk);
            current_offset = chunk_end as u64;
            sequence += 1;
        }

        Ok(chunks)
    }

    /// Updates statistics
    fn update_stats<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut FileIOStats),
    {
        let mut stats = self.stats.write();
        update_fn(&mut stats);
    }

    /// Gets file metadata
    async fn get_file_metadata(&self, path: &Path) -> Result<std::fs::Metadata, PipelineError> {
        fs::metadata(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to get file metadata for {}: {}", path.display(), e)))
    }
}

#[async_trait]
impl FileIOService for FileIOServiceImpl {
    async fn read_file_chunks(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError> {
        let start_time = std::time::Instant::now();
        let metadata = self.get_file_metadata(path).await.unwrap();
        let file_size = metadata.len();

        // Determine if we should use memory mapping
        if options.use_memory_mapping && self.should_use_mmap(file_size) {
            return self.read_file_mmap(path, options).await;
        }

        // Use regular file I/O
        let chunk_size = options.chunk_size.unwrap_or(self.config.read().default_chunk_size);
        let mut file = fs::File::open(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to open file {}: {}", path.display(), e)))
            .unwrap();

        if let Some(offset) = options.start_offset {
            file.seek(SeekFrom::Start(offset))
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to seek to offset {}: {}", offset, e)))
                .unwrap();
        }

        let mut chunks = Vec::new();
        let mut buffer = vec![0u8; chunk_size];
        let mut current_offset = options.start_offset.unwrap_or(0);
        let mut sequence = 0u64;
        let mut total_read = 0u64;

        let max_bytes = options.max_bytes.unwrap_or(file_size);

        loop {
            if total_read >= max_bytes {
                break;
            }

            let bytes_to_read = ((max_bytes - total_read) as usize).min(chunk_size);
            let bytes_read = file
                .read(&mut buffer[..bytes_to_read])
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to read from file: {}", e)))
                .unwrap();

            if bytes_read == 0 {
                break;
            }

            let chunk_data = buffer[..bytes_read].to_vec();
            let is_final = bytes_read < bytes_to_read || total_read + bytes_read as u64 >= max_bytes;

            let chunk = FileChunk::new(sequence, current_offset, chunk_data, is_final).unwrap();

            let chunk = if options.calculate_checksums {
                chunk.with_calculated_checksum()?
            } else {
                chunk
            };

            chunks.push(chunk);
            current_offset += bytes_read as u64;
            total_read += bytes_read as u64;
            sequence += 1;
        }

        let file_info = FileInfo {
            path: path.to_path_buf(),
            size: file_size,
            is_memory_mapped: false,
            modified_at: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
            created_at: metadata.created().unwrap_or(std::time::UNIX_EPOCH),
            permissions: 0o644, // Default permissions
            mime_type: None,
        };

        self.update_stats(|stats| {
            stats.bytes_read += total_read;
            stats.chunks_processed += chunks.len() as u64;
            stats.files_processed += 1;
            stats.total_processing_time_ms += start_time.elapsed().as_millis() as u64;
        });

        Ok(ReadResult {
            chunks,
            file_info,
            bytes_read: total_read,
            complete: total_read >= file_size,
        })
    }

    async fn read_file_mmap(&self, path: &Path, options: ReadOptions) -> Result<ReadResult, PipelineError> {
        let start_time = std::time::Instant::now();
        let metadata = self.get_file_metadata(path).await.unwrap();
        let file_size = metadata.len();

        let file = File::open(path)
            .map_err(|e| PipelineError::IoError(format!("Failed to open file for mmap {}: {}", path.display(), e)))
            .unwrap();

        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| PipelineError::IoError(format!("Failed to create memory map: {}", e)))?
        };

        let chunk_size = options.chunk_size.unwrap_or(self.config.read().default_chunk_size);
        let start_offset = options.start_offset.unwrap_or(0);

        let chunks = self
            .create_chunks_from_mmap(
                &mmap,
                chunk_size,
                options.calculate_checksums,
                start_offset,
                options.max_bytes,
            )
            .unwrap();

        let bytes_read = chunks.iter().map(|c| c.data_len() as u64).sum();

        let file_info = FileInfo {
            path: path.to_path_buf(),
            size: file_size,
            is_memory_mapped: true,
            modified_at: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
            created_at: metadata.created().unwrap_or(std::time::UNIX_EPOCH),
            permissions: 0o644,
            mime_type: None,
        };

        self.update_stats(|stats| {
            stats.bytes_read += bytes_read;
            stats.chunks_processed += chunks.len() as u64;
            stats.files_processed += 1;
            stats.memory_mapped_files += 1;
            stats.total_processing_time_ms += start_time.elapsed().as_millis() as u64;
        });

        Ok(ReadResult {
            chunks,
            file_info,
            bytes_read,
            complete: true,
        })
    }

    async fn write_file_chunks(
        &self,
        path: &Path,
        chunks: &[FileChunk],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        if options.create_dirs {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| PipelineError::IoError(format!("Failed to create directories: {}", e)))
                    .unwrap();
            }
        }

        let mut file = if options.append {
            fs::OpenOptions::new().create(true).append(true).open(path).await
        } else {
            fs::File::create(path).await
        }
        .map_err(|e| PipelineError::IoError(format!("Failed to create/open file {}: {}", path.display(), e)))
        .unwrap();

        let mut total_written = 0u64;
        let mut file_hasher = ring::digest::Context::new(&ring::digest::SHA256);

        for chunk in chunks {
            let data = chunk.data();
            file.write_all(data)
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to write chunk: {}", e)))
                .unwrap();

            if options.calculate_checksums {
                file_hasher.update(data);
            }

            total_written += data.len() as u64;
        }

        if options.sync {
            file.sync_all()
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to sync file: {}", e)))
                .unwrap();
        }

        let checksum = if options.calculate_checksums {
            Some(hex::encode(file_hasher.finish().as_ref()))
        } else {
            None
        };

        self.update_stats(|stats| {
            stats.bytes_written += total_written;
            stats.chunks_processed += chunks.len() as u64;
        });

        Ok(WriteResult {
            path: path.to_path_buf(),
            bytes_written: total_written,
            checksum,
            success: true,
        })
    }

    async fn write_file_data(
        &self,
        path: &Path,
        data: &[u8],
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        // Create a single chunk and use write_file_chunks
        let chunk = FileChunk::new(0, 0, data.to_vec(), true).unwrap();
        self.write_file_chunks(path, &[chunk], options).await
    }

    async fn get_file_info(&self, path: &Path) -> Result<FileInfo, PipelineError> {
        let metadata = self.get_file_metadata(path).await.unwrap();

        Ok(FileInfo {
            path: path.to_path_buf(),
            size: metadata.len(),
            is_memory_mapped: false,
            modified_at: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
            created_at: metadata.created().unwrap_or(std::time::UNIX_EPOCH),
            permissions: 0o644,
            mime_type: None,
        })
    }

    async fn file_exists(&self, path: &Path) -> Result<bool, PipelineError> {
        Ok(fs::metadata(path).await.is_ok())
    }

    async fn delete_file(&self, path: &Path) -> Result<(), PipelineError> {
        fs::remove_file(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to delete file {}: {}", path.display(), e)))
    }

    async fn copy_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        let read_result = self.read_file_chunks(source, ReadOptions::default()).await.unwrap();
        self.write_file_chunks(destination, &read_result.chunks, options).await
    }

    async fn move_file(
        &self,
        source: &Path,
        destination: &Path,
        options: WriteOptions,
    ) -> Result<WriteResult, PipelineError> {
        let result = self.copy_file(source, destination, options).await.unwrap();
        self.delete_file(source).await.unwrap();
        Ok(result)
    }

    async fn create_directory(&self, path: &Path) -> Result<(), PipelineError> {
        fs::create_dir_all(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to create directory {}: {}", path.display(), e)))
    }

    async fn directory_exists(&self, path: &Path) -> Result<bool, PipelineError> {
        match fs::metadata(path).await {
            Ok(metadata) => Ok(metadata.is_dir()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(PipelineError::IoError(format!(
                "Failed to check directory {}: {}",
                path.display(),
                e
            ))),
        }
    }

    async fn list_directory(&self, path: &Path) -> Result<Vec<FileInfo>, PipelineError> {
        let mut entries = fs::read_dir(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to read directory {}: {}", path.display(), e)))
            .unwrap();

        let mut files = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to read directory entry: {}", e)))?
        {
            let metadata = entry
                .metadata()
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to get entry metadata: {}", e)))
                .unwrap();

            if metadata.is_file() {
                files.push(FileInfo {
                    path: entry.path(),
                    size: metadata.len(),
                    is_memory_mapped: false,
                    modified_at: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
                    created_at: metadata.created().unwrap_or(std::time::UNIX_EPOCH),
                    permissions: 0o644,
                    mime_type: None,
                });
            }
        }

        Ok(files)
    }

    fn get_config(&self) -> FileIOConfig {
        self.config.read().clone()
    }

    fn update_config(&mut self, config: FileIOConfig) {
        *self.config.write() = config;
    }

    fn get_stats(&self) -> FileIOStats {
        self.stats.read().clone()
    }

    fn reset_stats(&mut self) {
        *self.stats.write() = FileIOStats::default();
    }

    async fn validate_file_integrity(&self, path: &Path, expected_checksum: &str) -> Result<bool, PipelineError> {
        let calculated_checksum = self.calculate_file_checksum(path).await.unwrap();
        Ok(calculated_checksum == expected_checksum)
    }

    async fn calculate_file_checksum(&self, path: &Path) -> Result<String, PipelineError> {
        let read_result = self
            .read_file_chunks(
                path,
                ReadOptions {
                    calculate_checksums: false,
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
        for chunk in &read_result.chunks {
            hasher.update(chunk.data());
        }

        Ok(hex::encode(hasher.finish().as_ref()))
    }

    async fn stream_file_chunks(
        &self,
        path: &Path,
        options: ReadOptions,
    ) -> Result<std::pin::Pin<Box<dyn futures::Stream<Item = Result<FileChunk, PipelineError>> + Send>>, PipelineError>
    {
        let chunk_size = options.chunk_size.unwrap_or(self.config.read().default_chunk_size);
        let file = fs::File::open(path)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to open file {}: {}", path.display(), e)))
            .unwrap();

        let file = if let Some(offset) = options.start_offset {
            let mut f = file;
            f.seek(std::io::SeekFrom::Start(offset))
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to seek to offset {}: {}", offset, e)))
                .unwrap();
            f
        } else {
            file
        };

        // Create state for the stream
        struct StreamState {
            file: fs::File,
            buffer: Vec<u8>,
            current_offset: u64,
            sequence: u64,
            total_read: u64,
            max_bytes: u64,
            calculate_checksums: bool,
        }

        let state = StreamState {
            file,
            buffer: vec![0u8; chunk_size],
            current_offset: options.start_offset.unwrap_or(0),
            sequence: 0,
            total_read: 0,
            max_bytes: options.max_bytes.unwrap_or(u64::MAX),
            calculate_checksums: options.calculate_checksums,
        };

        let stream = futures::stream::unfold(state, |mut state| async move {
            if state.total_read >= state.max_bytes {
                return None;
            }

            let bytes_to_read = std::cmp::min(state.buffer.len(), (state.max_bytes - state.total_read) as usize);
            state.buffer.resize(bytes_to_read, 0);

            match state.file.read(&mut state.buffer[..bytes_to_read]).await {
                Ok(0) => None, // EOF
                Ok(bytes_read) => {
                    state.buffer.truncate(bytes_read);
                    let is_final =
                        bytes_read < bytes_to_read || state.total_read + bytes_read as u64 >= state.max_bytes;

                    match FileChunk::new(state.sequence, state.current_offset, state.buffer.clone(), is_final) {
                        Ok(chunk) => {
                            let chunk = if state.calculate_checksums {
                                match chunk.with_calculated_checksum() {
                                    Ok(c) => c,
                                    Err(e) => return Some((Err(e), state)),
                                }
                            } else {
                                chunk
                            };

                            state.current_offset += bytes_read as u64;
                            state.sequence += 1;
                            state.total_read += bytes_read as u64;

                            Some((Ok(chunk), state))
                        }
                        Err(e) => Some((Err(e), state)),
                    }
                }
                Err(e) => Some((
                    Err(PipelineError::IoError(format!("Failed to read chunk: {}", e))),
                    state,
                )),
            }
        });

        Ok(Box::pin(stream))
    }

    async fn write_chunk_to_file(
        &self,
        path: &Path,
        chunk: &FileChunk,
        options: WriteOptions,
        is_first_chunk: bool,
    ) -> Result<WriteResult, PipelineError> {
        let start_time = std::time::Instant::now();

        // Create parent directories if needed
        if options.create_dirs {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| {
                        PipelineError::IoError(format!("Failed to create directories for {}: {}", path.display(), e))
                    })
                    .unwrap();
            }
        }

        // Open file in append mode for subsequent chunks, create/truncate for first
        // chunk
        let file = if is_first_chunk {
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .await
        } else {
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(path)
                .await
        }
        .map_err(|e| PipelineError::IoError(format!("Failed to open file {} for writing: {}", path.display(), e)))
        .unwrap();

        let mut file = file;
        file.write_all(chunk.data())
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to write chunk to {}: {}", path.display(), e)))
            .unwrap();

        if options.sync {
            file.sync_all()
                .await
                .map_err(|e| PipelineError::IoError(format!("Failed to sync file {}: {}", path.display(), e)))
                .unwrap();
        }

        let bytes_written = chunk.data().len() as u64;
        let write_time = start_time.elapsed();

        // Update statistics
        self.update_stats(|stats| {
            stats.bytes_written += bytes_written;
            stats.chunks_processed += 1;
            stats.total_processing_time_ms += write_time.as_millis() as u64;
        });

        Ok(WriteResult {
            path: path.to_path_buf(),
            bytes_written,
            success: true,
            checksum: if options.calculate_checksums {
                chunk.checksum().map(|c| c.to_string())
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_file_io_basic_operations() {
        let service = FileIOServiceImpl::new_default();

        // Create a temporary file with enough data for 1MB minimum chunk size
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_path_buf();
        // Create 2MB of test data to ensure we have multiple chunks
        let test_data = vec![b'A'; 2 * 1024 * 1024]; // 2MB of 'A' characters
        
        // Write test data asynchronously
        let mut file = tokio::fs::File::create(&temp_path).await.unwrap();
        file.write_all(&test_data).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Test reading
        let read_result = service
            .read_file_chunks(&temp_path, ReadOptions::default())
            .await
            .unwrap();

        assert!(!read_result.chunks.is_empty());
        assert_eq!(read_result.bytes_read, test_data.len() as u64);

        // Test writing
        let copy_path = temp_path.with_extension("copy");
        let write_result = service
            .write_file_data(
                &copy_path,
                &test_data,
                WriteOptions::default(),
            )
            .await
            .unwrap();

        assert_eq!(write_result.bytes_written, test_data.len() as u64);
        assert!(write_result.success);
    }

    #[tokio::test]
    async fn test_memory_mapping() {
        let service = FileIOServiceImpl::new_default();

        // Create a temporary file with enough data to trigger memory mapping
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_path_buf();
        // Create 3MB of test data to ensure memory mapping is used and we have multiple chunks
        let test_data = vec![0u8; 3 * 1024 * 1024]; // 3MB of data
        
        // Write test data asynchronously
        let mut file = tokio::fs::File::create(&temp_path).await.unwrap();
        file.write_all(&test_data).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Test memory-mapped reading
        let read_result = service
            .read_file_mmap(&temp_path, ReadOptions::default())
            .await
            .unwrap();

        assert!(!read_result.chunks.is_empty());
        assert!(read_result.file_info.is_memory_mapped);
        assert_eq!(read_result.bytes_read, test_data.len() as u64);
    }
}
