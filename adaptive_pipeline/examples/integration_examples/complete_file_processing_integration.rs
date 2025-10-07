//! # Complete File Processing Integration Demo
//!
//! This comprehensive integration example demonstrates end-to-end file processing
//! using the adaptive pipeline system. It showcases the integration of multiple
//! services working together to process files with various algorithms and configurations.
//!
//! ## Overview
//!
//! This demo demonstrates:
//!
//! - **Service Integration**: Multiple services working together seamlessly
//! - **File Processing Pipeline**: Complete file processing workflow
//! - **Memory Management**: Efficient memory usage with configurable options
//! - **Error Handling**: Comprehensive error handling and recovery
//! - **Performance Optimization**: Various optimization strategies
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                File Processing Pipeline                         │
//! │                                                                 │
//! │  ┌─────────────┐    ┌─────────────────┐    ┌─────────────────┐  │
//! │  │   File I/O  │───▶│ File Processor  │───▶│   Checksum      │  │
//! │  │   Service   │    │    Service      │    │   Processor     │  │
//! │  └─────────────┘    └─────────────────┘    └─────────────────┘  │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Features Demonstrated
//!
//! ### File I/O Operations
//! - Memory-mapped file reading for large files
//! - Chunked processing for streaming operations
//! - Configurable buffer sizes and chunk sizes
//! - Integrity verification with checksums
//!
//! ### Processing Pipeline
//! - Chain processing with multiple processors
//! - Configurable processing parameters
//! - Error handling and recovery
//! - Performance monitoring and metrics
//!
//! ### Integration Patterns
//! - Service composition and dependency injection
//! - Configuration management across services
//! - Resource sharing and optimization
//! - Async processing with proper error propagation
//!
//! ## Usage
//!
//! Run the integration demo:
//!
//! ```bash
//! cargo run --example complete_file_processing_integration
//! ```
//!
//! ## Expected Output
//!
//! The demo will show:
//! 1. Test file creation with various sizes
//! 2. File processing with different configurations
//! 3. Performance metrics and timing information
//! 4. Error handling demonstrations
//! 5. Resource usage statistics

use std::path::Path;
use std::sync::Arc;
use std::io::Write;
use tempfile::NamedTempFile;
use async_trait::async_trait;

use adaptive_pipeline::infrastructure::services::TokioFileIO;
use adaptive_pipeline::application::services::StreamingFileProcessor;
use adaptive_pipeline_domain::services::{
    file_io_service::{FileIOService, FileIOConfig, ReadOptions, WriteOptions},
    file_processor_service::{
        FileProcessorService, FileProcessorConfig, ChunkProcessor, ChainProcessor
    },
    checksum_service::ChecksumProcessor
};
use adaptive_pipeline_domain::{FileChunk, PipelineError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Complete File Processing Demo");
    println!("============================");
    
    // Initialize services
    let file_io_config = FileIOConfig {
        default_chunk_size: 32 * 1024, // 32KB
        max_mmap_size: 100 * 1024 * 1024, // 100MB
        enable_memory_mapping: true,
        verify_checksums: true,
        ..Default::default()
    };
    
    let file_io_service = Arc::new(TokioFileIO::new(file_io_config));
    let processor_config = FileProcessorConfig {
        processing_chunk_size: 16 * 1024, // 16KB for processing
        use_memory_mapping: true,
        verify_integrity: true,
        max_concurrent_files: 3,
        ..Default::default()
    };

    let mut file_processor = StreamingFileProcessor::new(file_io_service.clone(), processor_config);
    
    // Create test files with different sizes
    println!("\n1. Creating test files...");
    let test_files = create_test_files().await?;
    
    for (i, file) in test_files.iter().enumerate() {
        let info = file_io_service.get_file_info(file.path()).await?;
        println!("   ✓ Test file {}: {} bytes", i + 1, info.size);
    }
    
    // Demo 1: Basic file processing with checksum verification
    println!("\n2. Processing files with checksum verification...");
    let checksum_processor = Box::new(ChecksumProcessor { verify_existing: false });
    
    for (i, file) in test_files.iter().enumerate() {
        let result = file_processor.process_file(
            file.path(),
            None,
            Box::new(ChecksumProcessor { verify_existing: false }),
        ).await?;
        
        println!("   ✓ File {}: {} chunks, {} bytes, {}ms, mmap: {}", 
                 i + 1,
                 result.chunks_processed,
                 result.bytes_processed,
                 result.processing_time_ms,
                 result.used_memory_mapping);
    }
    
    // Demo 2: File processing with checksum
    println!("\n3. Processing files with checksum validation...");
    let mut processed_files = Vec::new();
    
    for (i, file) in test_files.iter().enumerate() {
        let processed_path = file.path().with_extension("processed");
        let checksum_processor = Box::new(ChecksumProcessor::sha256_processor(false));
        
        let result = file_processor.process_file(
            file.path(),
            Some(&processed_path),
            checksum_processor,
        ).await?;
        
        processed_files.push(processed_path);
        
        println!("  File {}: {} bytes processed", 
                 i + 1, 
                 result.bytes_processed);
    }
    
    // Demo 3: File processing with different chunk sizes
    println!("\n4. Processing files with different chunk sizes...");
    
    for (i, file) in processed_files.iter().enumerate() {
        let validated_path = file.with_extension("validated");
        let validation_processor = Box::new(ChecksumProcessor::sha256_processor(true)); // Verify existing checksums
        
        let result = file_processor.process_file(
            file,
            Some(&validated_path),
            validation_processor,
        ).await?;
        
        println!("  File {}: {} bytes validated", 
                 i + 1, 
                 result.bytes_processed);
        
        // Clean up processed file
        file_io_service.delete_file(file).await?;
        // Clean up validated file
        file_io_service.delete_file(&validated_path).await?;
    }
    
    // Demo 4: Chain processing (multiple checksum processors)
    println!("\n5. Chain processing (multiple checksum processors)...");
    
    let chain_processor = Box::new(ChainProcessor {
        processors: vec![
            Box::new(ChecksumProcessor::sha256_processor(false)),
            Box::new(ChecksumProcessor::sha256_processor(true)), // Verify the checksum we just added
        ],
    });
    
    let test_file = &test_files[0]; // Use first test file
    let processed_path = test_file.path().with_extension("chain_processed");
    
    let result = file_processor.process_file(
        test_file.path(),
        Some(&processed_path),
        chain_processor,
    ).await?;
    
    let original_size = file_io_service.get_file_info(test_file.path()).await?.size;
    let processed_size = file_io_service.get_file_info(&processed_path).await?.size;
    
    println!("   ✓ Chain processed: {} -> {} bytes in {}ms", 
             original_size, processed_size, result.processing_time_ms);
    
    // Clean up processed file
    file_io_service.delete_file(&processed_path).await?;
    
    // Demo 5: Batch processing
    println!("\n6. Batch processing multiple files...");
    
    let file_pairs: Vec<_> = test_files.iter().enumerate().map(|(i, file)| {
        let output_path = file.path().with_extension(&format!("batch_{}", i));
        (file.path().to_path_buf(), Some(output_path))
    }).collect();
    
    let batch_processor = Box::new(ChecksumProcessor { verify_existing: false });
    let batch_results = file_processor.process_files_batch(file_pairs.clone(), batch_processor).await?;
    
    println!("   ✓ Processed {} files in batch", batch_results.len());
    for (i, result) in batch_results.iter().enumerate() {
        println!("     - File {}: {} bytes in {}ms", 
                 i + 1, result.bytes_processed, result.processing_time_ms);
    }
    
    // Clean up batch output files
    for (_, output_path) in file_pairs {
        if let Some(path) = output_path {
            if file_io_service.file_exists(&path).await? {
                file_io_service.delete_file(&path).await?;
            }
        }
    }
    
    // Demo 6: Performance statistics
    println!("\n7. Performance statistics:");
    let stats = file_processor.get_processing_stats();
    println!("   ✓ Files processed: {}", stats.files_processed);
    println!("   ✓ Bytes processed: {}", stats.bytes_processed);
    println!("   ✓ Total time: {} ms", stats.total_processing_time_ms);
    println!("   ✓ Memory mapped files: {}", stats.memory_mapped_files);
    println!("   ✓ Average speed: {:.2} MB/s", stats.avg_processing_speed / (1024.0 * 1024.0));
    
    // Demo 7: File I/O service statistics
    println!("\n8. File I/O service statistics:");
    let io_stats = file_io_service.get_stats();
    println!("   ✓ Bytes read: {}", io_stats.bytes_read);
    println!("   ✓ Bytes written: {}", io_stats.bytes_written);
    println!("   ✓ Chunks processed: {}", io_stats.chunks_processed);
    println!("   ✓ Files processed: {}", io_stats.files_processed);
    println!("   ✓ Memory mapped files: {}", io_stats.memory_mapped_files);
    
    // Demo 8: Streaming large file
    println!("\n9. Streaming large file processing...");
    if let Some(large_file) = test_files.iter().find(|f| {
        // Find a file that might be large enough for streaming demo
        std::fs::metadata(f.path()).map(|m| m.len() > 1024).unwrap_or(false)
    }) {
        use futures::StreamExt;
        
        let mut stream = file_io_service.stream_file_chunks(
            large_file.path(),
            ReadOptions {
                chunk_size: Some(1024), // Small chunks for demo
                ..Default::default()
            },
        ).await?;
        
        let mut chunk_count = 0;
        let mut total_bytes = 0;
        
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    chunk_count += 1;
                    total_bytes += chunk.data_len();
                    
                    // Process chunk here (for demo, just count)
                    if chunk_count % 10 == 0 {
                        println!("   ✓ Streamed {} chunks ({} bytes)", chunk_count, total_bytes);
                    }
                }
                Err(e) => {
                    println!("   ✗ Stream error: {}", e);
                    break;
                }
            }
        }
        
        println!("   ✓ Streaming complete: {} chunks, {} bytes", chunk_count, total_bytes);
    }
    
    println!("\nDemo completed successfully!");
    println!("All temporary files have been cleaned up.");
    
    Ok(())
}

async fn create_test_files() -> Result<Vec<NamedTempFile>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    // Small file (1KB)
    let mut small_file = NamedTempFile::new()?;
    let small_data = "Hello, World! ".repeat(73); // ~1KB
    small_file.write_all(small_data.as_bytes())?;
    small_file.flush()?;
    files.push(small_file);
    
    // Medium file (64KB)
    let mut medium_file = NamedTempFile::new()?;
    let medium_data = "This is a medium-sized test file for demonstrating file processing capabilities. ".repeat(819); // ~64KB
    medium_file.write_all(medium_data.as_bytes())?;
    medium_file.flush()?;
    files.push(medium_file);
    
    // Large file (1MB)
    let mut large_file = NamedTempFile::new()?;
    let chunk = vec![0u8; 1024]; // 1KB chunk
    for i in 0..1024 {
        // Write pattern to make it more interesting
        let pattern = format!("Chunk {} - ", i).into_bytes();
        large_file.write_all(&pattern)?;
        large_file.write_all(&chunk[pattern.len()..])?;
    }
    large_file.flush()?;
    files.push(large_file);
    
    Ok(files)
}
