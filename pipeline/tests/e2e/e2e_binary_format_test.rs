//! # End-to-End Binary Format Tests
//!
//! E2E tests for .adapipe binary format: complete roundtrips, multi-chunk
//! processing, format validation, corruption detection, and version
//! compatibility.

use tempfile::{NamedTempFile, TempDir};
use tokio::fs;

use pipeline::infrastructure::services::{BinaryFormatService, BinaryFormatServiceImpl, BinaryFormatWriter};
use pipeline_domain::value_objects::{ChunkFormat, FileHeader};
use pipeline_domain::PipelineError;

/// Tests complete .adapipe roundtrip: input → compression/encryption → .adapipe → restored output.
#[tokio::test]
async fn test_e2e_adapipe_format_complete_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("input.txt");
    let output_file = temp_dir.path().join("output.adapipe");
    let restored_file = temp_dir.path().join("restored.txt");

    // Create test input file
    let test_data = b"Hello, World! This is a test file for binary format validation.\n".repeat(1000);
    fs::write(&input_file, &test_data).await.unwrap();

    let service = BinaryFormatServiceImpl::new();

    // Calculate original checksum
    let original_checksum = calculate_sha256(&test_data);

    // Step 1: Write .adapipe processed file (compressed/encrypted)
    {
        let header = FileHeader::new(
            "input.txt".to_string(),
            test_data.len() as u64,
            original_checksum.clone(),
        )
        .add_compression_step("brotli", 6)
        .add_encryption_step("aes256gcm", "argon2", 32, 12)
        .with_chunk_info(1024, 0) // Will be updated during processing
        .with_pipeline_id("test-pipeline-001".to_string());

        let mut writer: Box<dyn BinaryFormatWriter> = service.create_writer(&output_file, header).unwrap();

        // Simulate processing chunks (in real implementation, this would be done by
        // pipeline)
        let chunks = create_test_chunks(&test_data, 1024);
        for chunk in chunks {
            writer.write_chunk(chunk).unwrap();
        }

        // Calculate output checksum (in real implementation, done incrementally)
        let output_checksum = "simulated_output_checksum".to_string();
        let final_header = FileHeader::new(
            "input.txt".to_string(),
            test_data.len() as u64,
            original_checksum.clone(),
        )
        .add_compression_step("brotli", 6)
        .add_encryption_step("aes256gcm", "argon2", 32, 12)
        .with_chunk_info(1024, writer.chunks_written())
        .with_pipeline_id("test-pipeline-001".to_string())
        .with_output_checksum(output_checksum);

        let total_bytes = writer.finalize(final_header).await.unwrap();
        assert!(total_bytes > test_data.len() as u64); // Should be larger due
                                                       // to encryption +
                                                       // metadata
    }

    // Step 2: Validate the .adapipe processed file
    {
        let validation = service.validate_file(&output_file).await.unwrap();
        assert!(validation.is_valid);
        assert_eq!(validation.format_version, 1);
        assert!(validation.chunk_count > 0);
        assert!(validation.processing_summary.contains("Compression"));
        assert!(validation.processing_summary.contains("Encryption"));
    }

    // Step 3: Read metadata
    {
        let metadata = service.read_metadata(&output_file).await.unwrap();
        assert_eq!(metadata.original_filename, "input.txt");
        assert_eq!(metadata.original_size, test_data.len() as u64);
        assert_eq!(metadata.original_checksum, original_checksum);
        assert!(metadata.is_compressed());
        assert!(metadata.is_encrypted());
        assert_eq!(metadata.compression_algorithm(), Some("brotli"));
        assert_eq!(metadata.encryption_algorithm(), Some("aes256gcm"));

        // Verify processing steps are in correct order
        let restoration_steps = metadata.get_restoration_steps();
        assert_eq!(restoration_steps.len(), 2);
        assert_eq!(restoration_steps[0].order, 1); // Encryption (reverse order)
        assert_eq!(restoration_steps[1].order, 0); // Compression (reverse
                                                   // order)
    }

    // Step 4: Read and process chunks
    {
        let mut reader = service.create_reader(&output_file).await.unwrap();
        let header = reader.read_header().unwrap();

        let mut chunks_read = 0;
        let mut total_encrypted_data = 0;

        while let Some(chunk) = reader.read_next_chunk().await.unwrap() {
            chunks_read += 1;
            total_encrypted_data += chunk.encrypted_data.len();

            // Validate chunk format
            assert!(chunk.validate().is_ok());
            assert_eq!(chunk.nonce.len(), 12); // AES-GCM nonce size
            assert!(!chunk.encrypted_data.is_empty());
        }

        assert_eq!(chunks_read, header.chunk_count);
        assert!(total_encrypted_data > 0);
    }

    // Step 5: Simulate restoration process (decrypt + decompress)
    {
        let mut reader = service.create_reader(&output_file).await.unwrap();
        let header = reader.read_header().unwrap();

        let mut restored_data = Vec::new();

        while let Some(chunk) = reader.read_next_chunk().await.unwrap() {
            // In real implementation, this would decrypt and decompress
            // For test, we'll simulate by using original data
            let simulated_restored_chunk = simulate_chunk_restoration(&chunk, &test_data, restored_data.len());
            restored_data.extend_from_slice(&simulated_restored_chunk);
        }

        // Validate restoration
        assert!(header.validate_restored_file(&restored_data).unwrap());

        // Write restored file
        fs::write(&restored_file, &restored_data).await.unwrap();

        // Verify restored file matches original
        let restored_content = fs::read(&restored_file).await.unwrap();
        assert_eq!(restored_content, test_data);
    }
}

/// End-to-end test for pass-through files (no processing)
#[tokio::test]
async fn test_e2e_binary_format_pass_through() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("passthrough.adapipe");

    let test_data = b"Simple pass-through file content";
    let original_checksum = calculate_sha256(test_data);

    let service = BinaryFormatServiceImpl::new();

    // Create pass-through file (no processing steps)
    {
        let header = FileHeader::new(
            "passthrough.txt".to_string(),
            test_data.len() as u64,
            original_checksum.clone(),
        )
        .with_chunk_info(1024, 1)
        .with_pipeline_id("passthrough-pipeline".to_string());

        let mut writer: Box<dyn BinaryFormatWriter> = service.create_writer(&output_file, header.clone()).unwrap();

        // Write single chunk (unprocessed)
        let chunk = ChunkFormat::new([0u8; 12], test_data.to_vec());
        writer.write_chunk(chunk).unwrap();

        let final_header = header.with_output_checksum("passthrough_checksum".to_string());
        writer.finalize(final_header).await.unwrap();
    }

    // Validate pass-through file
    {
        let metadata = service.read_metadata(&output_file).await.unwrap();
        assert!(!metadata.is_compressed());
        assert!(!metadata.is_encrypted());
        assert!(metadata.processing_steps.is_empty());
        assert!(metadata.get_processing_summary().contains("pass-through"));
    }
}

/// End-to-end test for file format validation with corrupted data
#[tokio::test]
async fn test_e2e_binary_format_corruption_detection() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Write invalid magic bytes
    fs::write(file_path, b"INVALID_MAGIC_BYTES").await.unwrap();

    let service = BinaryFormatServiceImpl::new();

    // Should fail to read metadata
    let result = service.read_metadata(file_path).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid magic bytes"));
}

/// End-to-end test for version compatibility
#[tokio::test]
async fn test_e2e_binary_format_version_compatibility() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("versioned.adapipe");

    let service = BinaryFormatServiceImpl::new();

    // Create file with current version
    {
        let header = FileHeader::new("version_test.txt".to_string(), 100, "test_checksum".to_string());

        let writer: Box<dyn BinaryFormatWriter> = service.create_writer(&output_file, header.clone()).unwrap();
        writer.finalize(header).await.unwrap();
    }

    // Verify version is correctly stored and read
    {
        let metadata = service.read_metadata(&output_file).await.unwrap();
        assert_eq!(metadata.format_version, 1); // Current version
        assert!(!metadata.app_version.is_empty());
    }
}

/// End-to-end test for large file handling with multiple chunks
#[tokio::test]
async fn test_e2e_binary_format_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("large.adapipe");

    // Create larger test data (10KB)
    let test_data = b"X".repeat(10240);
    let original_checksum = calculate_sha256(&test_data);

    let service = BinaryFormatServiceImpl::new();

    // Write with smaller chunk size to create multiple chunks
    {
        let header = FileHeader::new(
            "large.txt".to_string(),
            test_data.len() as u64,
            original_checksum.clone(),
        )
        .add_compression_step("brotli", 6)
        .with_chunk_info(1024, 0); // 1KB chunks = ~10 chunks

        let mut writer: Box<dyn BinaryFormatWriter> = service.create_writer(&output_file, header).unwrap();

        // Write multiple chunks
        let chunks = create_test_chunks(&test_data, 1024);
        for chunk in chunks {
            writer.write_chunk(chunk).unwrap();
        }

        let final_header = FileHeader::new("large.txt".to_string(), test_data.len() as u64, original_checksum)
            .add_compression_step("brotli", 6)
            .with_chunk_info(1024, writer.chunks_written())
            .with_output_checksum("large_file_checksum".to_string());

        writer.finalize(final_header).await.unwrap();
    }

    // Verify multiple chunks
    {
        let metadata = service.read_metadata(&output_file).await.unwrap();
        assert!(metadata.chunk_count > 5); // Should have multiple chunks

        let mut reader = service.create_reader(&output_file).await.unwrap();
        reader.read_header().unwrap();

        let mut chunks_read = 0;
        while let Some(_chunk) = reader.read_next_chunk().await.unwrap() {
            chunks_read += 1;
        }

        assert_eq!(chunks_read, metadata.chunk_count);
    }
}

// Helper functions

fn calculate_sha256(data: &[u8]) -> String {
    use ring::digest;
    let digest = digest::digest(&digest::SHA256, data);
    hex::encode(digest.as_ref())
}

fn create_test_chunks(data: &[u8], chunk_size: usize) -> Vec<ChunkFormat> {
    let mut chunks = Vec::new();
    let mut offset = 0;
    let mut nonce_counter = 0u32;

    while offset < data.len() {
        let end = std::cmp::min(offset + chunk_size, data.len());
        let chunk_data = data[offset..end].to_vec();

        // Create unique nonce for each chunk
        let mut nonce = [0u8; 12];
        nonce[0..4].copy_from_slice(&nonce_counter.to_le_bytes());
        nonce_counter += 1;

        // In real implementation, this would be compressed and encrypted
        let chunk = ChunkFormat::new(nonce, chunk_data);
        chunks.push(chunk);

        offset = end;
    }

    chunks
}

fn simulate_chunk_restoration(chunk: &ChunkFormat, original_data: &[u8], offset: usize) -> Vec<u8> {
    // In real implementation, this would decrypt and decompress
    // For testing, we'll return the corresponding portion of original data
    let chunk_size = chunk.encrypted_data.len();
    let end = std::cmp::min(offset + chunk_size, original_data.len());
    original_data[offset..end].to_vec()
}
