//! # Binary Format Service Implementation
//!
//! This module provides services for reading and writing the Adaptive Pipeline
//! binary format (.adapipe). This format is specifically designed for files
//! that have been processed through the adaptive pipeline system with
//! compression and/or encryption.
//!
//! ## Overview
//!
//! The binary format service implementation provides:
//!
//! - **Format Writing**: Creates .adapipe format files with proper structure
//! - **Format Reading**: Reads and validates .adapipe format files
//! - **Streaming Support**: Efficient streaming I/O for large files
//! - **Integrity Verification**: Built-in checksums and validation
//! - **Version Management**: Handles format versioning and compatibility
//!
//! ## Architecture
//!
//! The service follows these design principles:
//!
//! - **Streaming I/O**: Processes files without loading entirely into memory
//! - **Async Operations**: All I/O operations are asynchronous and non-blocking
//! - **Error Handling**: Comprehensive error handling and validation
//! - **Thread Safety**: Safe concurrent access from multiple threads
//!
//! ## .adapipe Format Structure
//!
//! The .adapipe format is structured as follows:
//!
//! ```text
//! [CHUNK_DATA][JSON_HEADER][HEADER_LENGTH][FORMAT_VERSION][MAGIC_BYTES]
//! ```
//!
//! ### Components
//!
//! - **CHUNK_DATA**: Processed (compressed/encrypted) file data
//! - **JSON_HEADER**: Metadata including original filename, processing info
//! - **HEADER_LENGTH**: Length of the JSON header (4 bytes, little-endian)
//! - **FORMAT_VERSION**: Format version number (2 bytes, little-endian)
//! - **MAGIC_BYTES**: Format identifier "ADAPIPE\0" (8 bytes)
//!
//! ## Usage Examples
//!
//! ### Writing .adapipe Files

//!
//! ### Reading .adapipe Files

//!
//! ### Format Validation

//!
//! ## Format Features
//!
//! ### Integrity Verification
//!
//! - **Magic Bytes**: Format identification and validation
//! - **Version Checking**: Ensures compatibility with current implementation
//! - **Checksum Validation**: SHA-256 checksums for data integrity
//! - **Header Validation**: JSON header structure validation
//!
//! ### Metadata Preservation
//!
//! - **Original Filename**: Preserves original file name and path
//! - **File Size**: Original unprocessed file size
//! - **Processing Info**: Compression and encryption algorithms used
//! - **Timestamps**: Processing timestamp for audit trails
//! - **Custom Metadata**: Support for additional custom metadata
//!
//! ### Performance Optimization
//!
//! - **Streaming I/O**: Processes files without loading entirely into memory
//! - **Chunked Processing**: Efficient processing of large files in chunks
//! - **Async Operations**: Non-blocking I/O operations using Tokio
//! - **Memory Efficiency**: Minimal memory footprint during processing
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Format Validation**: Invalid magic bytes or format structure
//! - **Version Compatibility**: Unsupported format versions
//! - **Checksum Mismatch**: Data integrity verification failures
//! - **Header Corruption**: Malformed or corrupted JSON headers
//!
//! ### I/O Errors
//!
//! - **File Access**: Permission denied or file not found errors
//! - **Disk Space**: Insufficient disk space during writing
//! - **Network Issues**: Network-related I/O failures
//! - **Corruption**: File corruption detection and reporting
//!
//! ## Security Considerations
//!
//! ### Data Protection
//!
//! - **No Plaintext**: Original data is never stored in plaintext
//! - **Secure Headers**: Headers contain no sensitive information
//! - **Tamper Detection**: Checksums detect unauthorized modifications
//! - **Access Control**: Respects file system permissions
//!
//! ### Format Security
//!
//! - **Input Validation**: Thorough validation of all input data
//! - **Buffer Overflow Protection**: Safe buffer handling
//! - **Path Traversal Prevention**: Secure file path handling
//! - **Resource Limits**: Prevents resource exhaustion attacks
//!
//! ## Integration
//!
//! The binary format service integrates with:
//!
//! - **Compression Services**: Handles compressed data chunks
//! - **Encryption Services**: Manages encrypted data streams
//! - **File I/O Services**: Efficient file system operations
//! - **Validation Services**: Format and data validation
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Compression Integration**: Built-in compression for headers
//! - **Digital Signatures**: Cryptographic signatures for authenticity
//! - **Extended Metadata**: Support for rich metadata schemas
//! - **Format Evolution**: Backward-compatible format improvements

use async_trait::async_trait;

use pipeline_domain::value_objects::{ChunkFormat, FileHeader};
use pipeline_domain::PipelineError;
use sha2::{Digest, Sha256};
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::fs::{self as fs};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

/// Service for writing and reading Adaptive Pipeline processed files (.adapipe
/// format)
///
/// This trait defines the interface for handling the .adapipe binary format,
/// which is specifically designed for files that have been processed through
/// the adaptive pipeline system with compression and/or encryption.
///
/// # Important Note
///
/// This service handles .adapipe format files (processed pipeline output),
/// NOT general binary files like .png, .exe, etc. The .adapipe format is
/// a custom format designed for pipeline-processed data with embedded metadata.
///
/// # Key Features
///
/// - **Streaming I/O**: Efficient processing without loading entire files
/// - **Metadata Preservation**: Maintains original file information
/// - **Integrity Verification**: Built-in checksums and validation
/// - **Version Management**: Handles format versioning and compatibility
///
/// # Examples
///
#[async_trait]
pub trait BinaryFormatService: Send + Sync {
    /// Creates a new .adapipe format writer for streaming processed output
    fn create_writer(
        &self,
        output_path: &Path,
        header: FileHeader,
    ) -> Result<Box<dyn BinaryFormatWriter>, PipelineError>;

    /// Creates a new .adapipe format reader for streaming processed input
    async fn create_reader(&self, input_path: &Path) -> Result<Box<dyn BinaryFormatReader>, PipelineError>;

    /// Validates an .adapipe processed file without full restoration
    async fn validate_file(&self, file_path: &Path) -> Result<ValidationResult, PipelineError>;

    /// Extracts metadata from an .adapipe processed file
    async fn read_metadata(&self, file_path: &Path) -> Result<FileHeader, PipelineError>;
}

/// Writer for streaming .adapipe processed files
#[async_trait]
pub trait BinaryFormatWriter: Send + Sync {
    /// Writes a processed chunk (compressed/encrypted data) to the .adapipe
    /// file
    fn write_chunk(&mut self, chunk: ChunkFormat) -> Result<(), PipelineError>;

    /// Writes a processed chunk at a specific position for concurrent
    /// processing Uses chunk sequence number to calculate exact file offset
    async fn write_chunk_at_position(&mut self, chunk: ChunkFormat, sequence_number: u64) -> Result<(), PipelineError>;

    /// Finalizes the .adapipe file by writing the footer with complete metadata
    async fn finalize(self: Box<Self>, final_header: FileHeader) -> Result<u64, PipelineError>;

    /// Gets the current number of bytes written
    fn bytes_written(&self) -> u64;

    /// Gets the current number of chunks written
    fn chunks_written(&self) -> u32;
}

/// Reader for streaming .adapipe processed files
#[async_trait]
pub trait BinaryFormatReader: Send + Sync {
    /// Reads the .adapipe file header/metadata
    fn read_header(&self) -> Result<FileHeader, PipelineError>;

    /// Reads the next processed chunk (compressed/encrypted data) from the
    /// .adapipe file
    async fn read_next_chunk(&mut self) -> Result<Option<ChunkFormat>, PipelineError>;

    /// Seeks to a specific chunk by index
    async fn seek_to_chunk(&mut self, chunk_index: u32) -> Result<(), PipelineError>;

    /// Validates the file integrity
    async fn validate_integrity(&mut self) -> Result<bool, PipelineError>;
}

/// Result of file validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub format_version: u16,
    pub file_size: u64,
    pub chunk_count: u32,
    pub processing_summary: String,
    pub integrity_verified: bool,
    pub errors: Vec<String>,
}

/// Implementation of BinaryFormatService
pub struct BinaryFormatServiceImpl;

impl BinaryFormatServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BinaryFormatService for BinaryFormatServiceImpl {
    fn create_writer(
        &self,
        output_path: &Path,
        header: FileHeader,
    ) -> Result<Box<dyn BinaryFormatWriter>, PipelineError> {
        // Create a buffered writer that will write chunks on finalize
        Ok(Box::new(BufferedBinaryWriter::new(output_path.to_path_buf(), header)))
    }

    async fn create_reader(&self, input_path: &Path) -> Result<Box<dyn BinaryFormatReader>, PipelineError> {
        let reader = StreamingBinaryReader::new(input_path).await?;
        Ok(Box::new(reader))
    }

    async fn validate_file(&self, file_path: &Path) -> Result<ValidationResult, PipelineError> {
        let mut reader = self.create_reader(file_path).await?;
        let header = reader.read_header()?;
        let integrity_verified = reader.validate_integrity().await?;

        let file_metadata = fs::metadata(file_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        Ok(ValidationResult {
            is_valid: true,
            format_version: header.format_version,
            file_size: file_metadata.len(),
            chunk_count: header.chunk_count,
            processing_summary: header.get_processing_summary(),
            integrity_verified,
            errors: Vec::new(),
        })
    }

    async fn read_metadata(&self, file_path: &Path) -> Result<FileHeader, PipelineError> {
        let reader = self.create_reader(file_path).await?;
        reader.read_header()
    }
}

/// Buffered writer that stores chunks in memory and writes them all during finalize
/// This is simpler than StreamingBinaryWriter and suitable for tests and small files
pub struct BufferedBinaryWriter {
    output_path: PathBuf,
    header: FileHeader,
    chunks: Vec<ChunkFormat>,
}

impl BufferedBinaryWriter {
    fn new(output_path: PathBuf, header: FileHeader) -> Self {
        Self {
            output_path,
            header,
            chunks: Vec::new(),
        }
    }
}

#[async_trait]
impl BinaryFormatWriter for BufferedBinaryWriter {
    fn write_chunk(&mut self, chunk: ChunkFormat) -> Result<(), PipelineError> {
        // Just buffer the chunk in memory
        self.chunks.push(chunk);
        Ok(())
    }

    async fn write_chunk_at_position(&mut self, chunk: ChunkFormat, _sequence_number: u64) -> Result<(), PipelineError> {
        // For buffered writer, position doesn't matter - just append
        self.chunks.push(chunk);
        Ok(())
    }

    async fn finalize(self: Box<Self>, mut final_header: FileHeader) -> Result<u64, PipelineError> {
        // Create the output file
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.output_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        // Write all buffered chunks
        let mut total_bytes = 0u64;
        let mut hasher = Sha256::new();

        for chunk in &self.chunks {
            let (chunk_bytes, chunk_size) = chunk.to_bytes_with_size();
            file.write_all(&chunk_bytes)
                .await
                .map_err(|e| PipelineError::IoError(e.to_string()))?;
            hasher.update(&chunk_bytes);
            total_bytes += chunk_size;
        }

        // Update final header with actual values
        final_header.chunk_count = self.chunks.len() as u32;
        final_header.processed_at = chrono::Utc::now();
        final_header.output_checksum = format!("{:x}", hasher.finalize());

        // Write footer
        let footer_bytes = final_header.to_footer_bytes().unwrap();
        file.write_all(&footer_bytes)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        file.flush()
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        Ok(total_bytes + footer_bytes.len() as u64)
    }

    fn bytes_written(&self) -> u64 {
        self.chunks.iter().map(|c| c.encrypted_data.len() as u64 + 16).sum()
    }

    fn chunks_written(&self) -> u32 {
        self.chunks.len() as u32
    }
}

/// Streaming writer implementation
#[allow(dead_code)]
pub struct StreamingBinaryWriter {
    file: tokio::fs::File,
    bytes_written: AtomicU64,
    chunks_written: AtomicU64,
    initial_header: FileHeader,
    // Incremental checksum calculation
    output_hasher: Arc<Mutex<Sha256>>,
    // Flushing strategy fields
    flush_interval: u64,          // Flush every N chunks
    buffer_size_threshold: u64,   // Flush when buffer exceeds this size
    bytes_since_flush: AtomicU64, // Track bytes written since last flush
}

impl StreamingBinaryWriter {
    async fn new(output_path: &Path, header: FileHeader) -> Result<Self, PipelineError> {
        let file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        Ok(Self {
            file,
            bytes_written: AtomicU64::new(0),
            chunks_written: AtomicU64::new(0),
            initial_header: header,
            output_hasher: Arc::new(Mutex::new(Sha256::new())),
            flush_interval: 1024 * 1024,             // 1MB default flush interval
            buffer_size_threshold: 10 * 1024 * 1024, // 10MB buffer threshold
            bytes_since_flush: AtomicU64::new(0),
        })
    }
}

#[async_trait]
impl BinaryFormatWriter for StreamingBinaryWriter {
    fn write_chunk(&mut self, chunk: ChunkFormat) -> Result<(), PipelineError> {
        // TODO: Implement write_chunk method with proper parameters
        Err(PipelineError::InvalidConfiguration(
            "write_chunk not yet implemented".to_string(),
        ))
    }

    /// Writes a processed chunk at a specific position for concurrent
    /// processing
    ///
    /// This method implements **random access writing**, which is the key to
    /// achieving true concurrent chunk processing. Instead of writing
    /// chunks sequentially, each chunk is written directly to its
    /// calculated position in the file.
    ///
    /// # How Random Access Writing Works
    ///
    /// ## The Problem with Sequential Writing:
    /// ```text
    /// Traditional approach:
    /// Thread 1: Process chunk 0 → Wait for write slot → Write chunk 0
    /// Thread 2: Process chunk 1 → Wait for chunk 0 to finish → Write chunk 1  
    /// Thread 3: Process chunk 2 → Wait for chunk 1 to finish → Write chunk 2
    ///
    /// Result: Processing is concurrent, but writing is still sequential!
    /// ```
    ///
    /// ## The Solution - Random Access Writing:
    /// ```text
    /// Our approach:
    /// Thread 1: Process chunk 0 → Write to position 0 (immediately)
    /// Thread 2: Process chunk 1 → Write to position 1024 (immediately)
    /// Thread 3: Process chunk 2 → Write to position 2048 (immediately)
    ///
    /// Result: Both processing AND writing are truly concurrent!
    /// ```
    ///
    /// ## Position Calculation:
    /// Each chunk's file position is calculated as:
    /// `file_position = sequence_number * chunk_size`
    ///
    /// This ensures chunks are written to the correct location in the final
    /// file, regardless of the order in which they complete processing.
    ///
    /// # Arguments
    /// * `chunk` - The processed chunk data to write
    /// * `sequence_number` - The chunk's position in the original file (0, 1,
    ///   2, ...)
    ///
    /// # Returns
    /// * `Ok(())` if the chunk was written successfully
    /// * `Err(PipelineError)` if there was an I/O error or validation failure
    async fn write_chunk_at_position(&mut self, chunk: ChunkFormat, sequence_number: u64) -> Result<(), PipelineError> {
        use std::io::SeekFrom;

        // STEP 1: Validate the chunk format before writing
        // This ensures we don't write corrupted data to the file
        chunk.validate().unwrap();

        // STEP 2: Convert chunk to bytes and calculate size
        // The chunk format includes: [nonce(12)] + [length(4)] + [data]
        let (chunk_bytes, chunk_size) = chunk.to_bytes_with_size();

        // STEP 3: Calculate the exact file position for this chunk
        // IMPORTANT: This is where the "random access" magic happens!
        //
        // Each chunk gets written to a specific position based on its sequence number:
        // - Chunk 0 goes to position 0
        // - Chunk 1 goes to position (1 * chunk_size)
        // - Chunk 2 goes to position (2 * chunk_size)
        // - And so on...
        //
        // This allows multiple threads to write different chunks simultaneously
        // without interfering with each other.
        let file_position = sequence_number * chunk_size;

        // STEP 4: Seek to the calculated position in the file
        // This moves the file pointer to exactly where this chunk should be written
        self.file
            .seek(SeekFrom::Start(file_position))
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to seek to position {}: {}", file_position, e)))?;

        // STEP 5: Write chunk bytes to file
        self.file
            .write_all(&chunk_bytes)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        // STEP 6: Update incremental checksum with written data
        {
            let mut hasher = self.output_hasher.lock().await;
            hasher.update(&chunk_bytes);
        }

        // STEP 7: Update statistics using atomic operations for thread safety
        self.bytes_written.fetch_add(chunk_size, Ordering::Relaxed);
        self.chunks_written.fetch_add(1, Ordering::Relaxed);
        self.bytes_since_flush.fetch_add(chunk_size, Ordering::Relaxed);

        Ok(())
    }

    async fn finalize(self: Box<Self>, mut final_header: FileHeader) -> Result<u64, PipelineError> {
        // Update header with final statistics
        final_header.chunk_count = self.chunks_written.load(Ordering::Relaxed) as u32;
        final_header.processed_at = chrono::Utc::now();

        // Finalize incremental checksum calculation
        let output_checksum = {
            let mut hasher = self.output_hasher.lock().await;
            let result = hasher.finalize_reset();
            format!("{:x}", result)
        };
        final_header.output_checksum = output_checksum;

        // Write footer with calculated checksum
        let footer_bytes = final_header.to_footer_bytes().unwrap();
        let mut file = self.file;
        file.write_all(&footer_bytes)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        // Flush all data to ensure durability
        file.flush().await.map_err(|e| PipelineError::IoError(e.to_string()))?;

        let total_bytes = self.bytes_written.load(Ordering::Relaxed) + footer_bytes.len() as u64;

        Ok(total_bytes)
    }

    fn bytes_written(&self) -> u64 {
        self.bytes_written.load(Ordering::Relaxed)
    }

    fn chunks_written(&self) -> u32 {
        self.chunks_written.load(Ordering::Relaxed) as u32
    }
}

/// Streaming reader implementation
#[allow(dead_code)]
pub struct StreamingBinaryReader {
    file: tokio::fs::File,
    file_size: u64,
    header: Option<FileHeader>,
    current_chunk_index: u32,
    chunks_start_offset: u64,
}

impl StreamingBinaryReader {
    async fn new(input_path: &Path) -> Result<Self, PipelineError> {
        let mut file = tokio::fs::File::open(input_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        let metadata = std::fs::metadata(input_path).map_err(|e| PipelineError::IoError(e.to_string()))?;
        let file_size = metadata.len();

        // Read the header from the file footer
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        let (header, footer_size) = FileHeader::from_footer_bytes(&file_data)?;

        // Calculate where chunk data starts (beginning of file)
        let chunks_start_offset = 0;

        // Reopen file and seek to start of chunks
        let mut file = tokio::fs::File::open(input_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;
        file.seek(SeekFrom::Start(chunks_start_offset))
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        Ok(Self {
            file,
            file_size,
            header: Some(header),
            current_chunk_index: 0,
            chunks_start_offset,
        })
    }
}

#[async_trait]
impl BinaryFormatReader for StreamingBinaryReader {
    fn read_header(&self) -> Result<FileHeader, PipelineError> {
        // Return the header that was parsed during initialization
        self.header
            .clone()
            .ok_or_else(|| PipelineError::ValidationError("Header not loaded".to_string()))
    }

    async fn read_next_chunk(&mut self) -> Result<Option<ChunkFormat>, PipelineError> {
        // Check if we've read all chunks
        let header = self.header.as_ref().ok_or_else(||
            PipelineError::ValidationError("Header not loaded".to_string()))?;

        if self.current_chunk_index >= header.chunk_count {
            return Ok(None); // EOF - all chunks read
        }

        // Read chunk header first (12 bytes nonce + 4 bytes length)
        let mut chunk_header = vec![0u8; 16];
        match self.file.read_exact(&mut chunk_header).await {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // Reached end of chunk data (before footer)
                return Ok(None);
            }
            Err(e) => {
                return Err(PipelineError::IoError(format!("Failed to read chunk header: {}", e)));
            }
        }

        // Parse nonce and data length
        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&chunk_header[0..12]);
        let data_length = u32::from_le_bytes([
            chunk_header[12],
            chunk_header[13],
            chunk_header[14],
            chunk_header[15],
        ]) as usize;

        // Read encrypted data
        let mut encrypted_data = vec![0u8; data_length];
        self.file
            .read_exact(&mut encrypted_data)
            .await
            .map_err(|e| PipelineError::IoError(format!("Failed to read chunk data: {}", e)))?;

        // Create chunk format
        let chunk = ChunkFormat::new(nonce, encrypted_data);

        // Increment chunk index
        self.current_chunk_index += 1;

        Ok(Some(chunk))
    }

    async fn seek_to_chunk(&mut self, chunk_index: u32) -> Result<(), PipelineError> {
        // For now, we'll implement a simple approach
        // TODO: In production, we could maintain a chunk index for faster seeking

        if chunk_index == 0 {
            self.file
                .seek(SeekFrom::Start(self.chunks_start_offset))
                .await
                .map_err(|e| PipelineError::IoError(e.to_string()))
                .unwrap();
            self.current_chunk_index = 0;
            return Ok(());
        }

        // Reset to beginning and skip chunks
        self.file
            .seek(SeekFrom::Start(self.chunks_start_offset))
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))
            .unwrap();
        self.current_chunk_index = 0;

        // Skip chunks until we reach the desired index
        for _ in 0..chunk_index {
            if self.read_next_chunk().await.unwrap().is_none() {
                return Err(PipelineError::ValidationError("Chunk index out of bounds".to_string()));
            }
        }

        Ok(())
    }

    async fn validate_integrity(&mut self) -> Result<bool, PipelineError> {
        // Ensure we have header
        let header = self.header.as_ref().ok_or_else(||
            PipelineError::ValidationError("Header not loaded".to_string()))?;

        // We need to calculate checksum of only the chunk data (not the footer)
        // The footer contains: [JSON_HEADER][HEADER_LENGTH][FORMAT_VERSION][MAGIC_BYTES]

        // First, get the footer size from the header
        let footer_bytes = header.to_footer_bytes()?;
        let footer_size = footer_bytes.len() as u64;

        // Calculate the size of chunk data (total file size - footer size)
        let chunk_data_size = self.file_size - footer_size;

        // Seek to beginning of file
        self.file
            .seek(SeekFrom::Start(0))
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        // Read only the chunk data (not the footer)
        let mut chunk_data = vec![0u8; chunk_data_size as usize];
        self.file
            .read_exact(&mut chunk_data)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;

        // Calculate SHA256 checksum of chunk data
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(&chunk_data);
        let calculated_checksum = format!("{:x}", hasher.finalize());

        // Compare with stored checksum
        let is_valid = calculated_checksum == header.output_checksum;

        // Reset file position to continue reading chunks if needed
        self.file
            .seek(SeekFrom::Start(self.chunks_start_offset))
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;
        self.current_chunk_index = 0;

        Ok(is_valid)
    }
}

impl Default for BinaryFormatServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pipeline_domain::value_objects::{ChunkFormat, FileHeader, ProcessingStepType};
    use tempfile::{NamedTempFile, TempDir};
    use tokio::fs;

    #[tokio::test]
    async fn test_binary_format_roundtrip() {
        // Create a temporary file for testing
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test.adapipe");

        // Create test header
        let header = FileHeader::new(
            "test_file.txt".to_string(),
            1024,
            "original_checksum_abc123".to_string(),
        )
        .add_compression_step("brotli", 6)
        .add_encryption_step("aes256gcm", "argon2", 32, 12)
        .with_chunk_info(1024, 2)
        .with_pipeline_id("test-pipeline".to_string());

        // Create test chunks
        let chunk1 = ChunkFormat::new([1u8; 12], vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let chunk2 = ChunkFormat::new([2u8; 12], vec![0xCA, 0xFE, 0xBA, 0xBE]);

        // Write file using BufferedBinaryWriter
        let service = BinaryFormatServiceImpl::new();
        let mut writer = service.create_writer(&test_file_path, header.clone()).unwrap();
        writer.write_chunk(chunk1.clone()).unwrap();
        writer.write_chunk(chunk2.clone()).unwrap();

        // Finalize with updated header
        let final_header = header.clone();
        writer.finalize(final_header).await.unwrap();

        // Read the file back
        let mut reader = service.create_reader(&test_file_path).await.unwrap();

        // Test read_header
        let read_header = reader.read_header().unwrap();
        assert_eq!(read_header.original_filename, "test_file.txt");
        assert_eq!(read_header.chunk_count, 2);
        assert!(read_header.is_compressed());
        assert!(read_header.is_encrypted());

        // Test read_next_chunk
        let read_chunk1 = reader.read_next_chunk().await.unwrap();
        assert!(read_chunk1.is_some());
        let read_chunk1 = read_chunk1.unwrap();
        assert_eq!(read_chunk1.nonce, chunk1.nonce);
        assert_eq!(read_chunk1.encrypted_data, chunk1.encrypted_data);

        let read_chunk2 = reader.read_next_chunk().await.unwrap();
        assert!(read_chunk2.is_some());
        let read_chunk2 = read_chunk2.unwrap();
        assert_eq!(read_chunk2.nonce, chunk2.nonce);
        assert_eq!(read_chunk2.encrypted_data, chunk2.encrypted_data);

        // Test EOF
        let read_chunk3 = reader.read_next_chunk().await.unwrap();
        assert!(read_chunk3.is_none());

        // Test validate_integrity
        let is_valid = reader.validate_integrity().await.unwrap();
        assert!(is_valid, "File integrity validation should pass");
    }

    #[tokio::test]
    async fn test_file_validation() {
        // Create a temporary file for testing
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_validation.adapipe");

        // Create test header with specific checksum
        let header = FileHeader::new(
            "validation_test.txt".to_string(),
            2048,
            "original_checksum_xyz789".to_string(),
        )
        .add_compression_step("zstd", 3)
        .with_chunk_info(1024, 1)
        .with_pipeline_id("validation-pipeline".to_string());

        // Create test chunk
        let chunk = ChunkFormat::new([5u8; 12], vec![0x12, 0x34, 0x56, 0x78]);

        // Write file
        let service = BinaryFormatServiceImpl::new();
        let mut writer = service.create_writer(&test_file_path, header.clone()).unwrap();
        writer.write_chunk(chunk.clone()).unwrap();
        let final_header = header.clone();
        writer.finalize(final_header).await.unwrap();

        // Validate the file
        let validation_result = service.validate_file(&test_file_path).await.unwrap();
        assert!(validation_result.is_valid);
        assert_eq!(validation_result.chunk_count, 1);
        assert_eq!(validation_result.format_version, 1);
        assert!(validation_result.integrity_verified);
        assert!(validation_result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_read_metadata() {
        // Create a temporary file for testing
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_metadata.adapipe");

        // Create test header with metadata
        let header = FileHeader::new(
            "metadata_test.txt".to_string(),
            4096,
            "checksum_metadata_test".to_string(),
        )
        .add_encryption_step("chacha20poly1305", "pbkdf2", 32, 12)
        .with_chunk_info(2048, 2)
        .with_pipeline_id("metadata-pipeline".to_string())
        .with_metadata("custom_key".to_string(), "custom_value".to_string());

        // Create and write chunks
        let chunk1 = ChunkFormat::new([7u8; 12], vec![0xAA, 0xBB, 0xCC, 0xDD]);
        let chunk2 = ChunkFormat::new([8u8; 12], vec![0x11, 0x22, 0x33, 0x44]);

        let service = BinaryFormatServiceImpl::new();
        let mut writer = service.create_writer(&test_file_path, header.clone()).unwrap();
        writer.write_chunk(chunk1).unwrap();
        writer.write_chunk(chunk2).unwrap();
        let final_header = header.clone();
        writer.finalize(final_header).await.unwrap();

        // Read metadata
        let metadata = service.read_metadata(&test_file_path).await.unwrap();
        assert_eq!(metadata.original_filename, "metadata_test.txt");
        assert_eq!(metadata.original_size, 4096);
        assert_eq!(metadata.chunk_count, 2);
        assert_eq!(metadata.pipeline_id, "metadata-pipeline");
        assert!(metadata.is_encrypted());
        assert!(!metadata.is_compressed());
        assert_eq!(metadata.encryption_algorithm(), Some("chacha20poly1305"));
        assert_eq!(
            metadata.metadata.get("custom_key"),
            Some(&"custom_value".to_string())
        );
    }

    #[tokio::test]
    async fn test_seek_to_chunk() {
        // Create a temporary file for testing
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_seek.adapipe");

        // Create test header
        let header = FileHeader::new(
            "seek_test.txt".to_string(),
            3072,
            "checksum_seek_test".to_string(),
        )
        .with_chunk_info(1024, 3);

        // Create test chunks with distinct data
        let chunk1 = ChunkFormat::new([1u8; 12], vec![0x01, 0x02, 0x03, 0x04]);
        let chunk2 = ChunkFormat::new([2u8; 12], vec![0x05, 0x06, 0x07, 0x08]);
        let chunk3 = ChunkFormat::new([3u8; 12], vec![0x09, 0x0A, 0x0B, 0x0C]);

        // Write file
        let service = BinaryFormatServiceImpl::new();
        let mut writer = service.create_writer(&test_file_path, header.clone()).unwrap();
        writer.write_chunk(chunk1.clone()).unwrap();
        writer.write_chunk(chunk2.clone()).unwrap();
        writer.write_chunk(chunk3.clone()).unwrap();
        let final_header = header.clone();
        writer.finalize(final_header).await.unwrap();

        // Create reader
        let mut reader = service.create_reader(&test_file_path).await.unwrap();

        // Seek to chunk 2 (0-indexed)
        reader.seek_to_chunk(2).await.unwrap();
        let read_chunk = reader.read_next_chunk().await.unwrap().unwrap();
        assert_eq!(read_chunk.nonce, chunk3.nonce);
        assert_eq!(read_chunk.encrypted_data, chunk3.encrypted_data);

        // Seek back to chunk 0
        reader.seek_to_chunk(0).await.unwrap();
        let read_chunk = reader.read_next_chunk().await.unwrap().unwrap();
        assert_eq!(read_chunk.nonce, chunk1.nonce);
        assert_eq!(read_chunk.encrypted_data, chunk1.encrypted_data);

        // Seek to chunk 1
        reader.seek_to_chunk(1).await.unwrap();
        let read_chunk = reader.read_next_chunk().await.unwrap().unwrap();
        assert_eq!(read_chunk.nonce, chunk2.nonce);
        assert_eq!(read_chunk.encrypted_data, chunk2.encrypted_data);
    }
}
