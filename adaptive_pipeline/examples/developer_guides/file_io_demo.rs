//! # File I/O Service Demonstration
//!
//! This example demonstrates the comprehensive file I/O capabilities of the adaptive pipeline system.
//! It showcases various file operations including reading, writing, memory mapping, chunked processing,
//! integrity verification, and performance optimization features.
//!
//! ## Overview
//!
//! The file I/O service provides:
//!
//! - **Memory Mapping**: Efficient handling of large files using memory-mapped I/O
//! - **Chunked Processing**: Streaming operations for very large files
//! - **Integrity Verification**: SHA-256 checksums for data consistency
//! - **Async Operations**: Non-blocking I/O operations using Tokio
//! - **Performance Optimization**: Configurable buffer sizes and concurrent operations
//! - **Cross-Platform Support**: Works on Windows, macOS, and Linux
//!
//! ## Features Demonstrated
//!
//! ### Basic File Operations
//! - Writing data to files with various options
//! - Reading files with different strategies (memory mapping vs. streaming)
//! - File metadata retrieval and validation
//!
//! ### Advanced Features
//! - Memory mapping for large files (>512MB configurable threshold)
//! - Chunked reading and writing for streaming operations
//! - Integrity verification with SHA-256 checksums
//! - Performance statistics and monitoring
//!
//! ### Configuration Options
//! - Customizable chunk sizes for optimal performance
//! - Memory mapping thresholds and buffer sizes
//! - Concurrent operation limits
//! - Checksum verification settings
//!
//! ## Usage
//!
//! Run this example with:
//!
//! ```bash
//! cargo run --example file_io_demo
//! ```
//!
//! The example will:
//! 1. Create a test file with sample data
//! 2. Demonstrate various reading strategies
//! 3. Show chunked processing capabilities
//! 4. Verify data integrity with checksums
//! 5. Display performance statistics
//! 6. Clean up temporary files
//!
//! ## Performance Considerations
//!
//! - **Memory Mapping**: Used for files larger than the configured threshold
//! - **Chunked Processing**: Efficient for streaming large files
//! - **Buffer Sizes**: Configurable for optimal I/O performance
//! - **Concurrent Operations**: Limits prevent resource exhaustion
//!
//! ## Error Handling
//!
//! The example demonstrates proper error handling for:
//! - File system permissions
//! - Disk space limitations
//! - Checksum verification failures
//! - Memory mapping failures
//!
//! ## Integration
//!
//! This service integrates with:
//! - Pipeline processing stages
//! - Compression and encryption services
//! - Metrics collection and monitoring
//! - Configuration management

use std::path::Path;
use adaptive_pipeline::infrastructure::services::TokioFileIO;
use adaptive_pipeline_domain::services::file_io_service::{
    FileIOService, ReadOptions, WriteOptions, FileIOConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the file I/O service with custom configuration
    let config = FileIOConfig {
        default_chunk_size: 32 * 1024, // 32KB chunks
        max_mmap_size: 512 * 1024 * 1024, // 512MB max for memory mapping
        enable_memory_mapping: true,
        buffer_size: 4096,
        verify_checksums: true,
        max_concurrent_operations: 5,
    };
    
    let mut file_service = TokioFileIO::new(config);
    
    println!("File I/O Service Demo");
    println!("====================");
    
    // Demo 1: Create a test file with sample data
    let test_file_path = Path::new("demo_test_file.txt");
    let sample_data = b"Hello, World! This is a demonstration of the file I/O service with memory mapping capabilities.
This service can handle large files efficiently using memory mapping when appropriate.
It also supports chunked reading and writing for streaming operations.
The service includes integrity checking with SHA-256 checksums.
This is additional content to make the sample data larger than the minimum chunk size.
We need at least 1024 bytes for the chunk size validation to pass.
This text is being repeated to reach the minimum size requirement.
The file I/O service is designed to handle both small and large files efficiently.
Memory mapping is used for large files to improve performance.
Chunked processing allows for streaming operations on very large files.
Integrity verification ensures data consistency throughout the process.
The service supports various file operations including copy, move, and delete.
Statistics tracking provides insights into performance characteristics.
This should now be sufficient data to exceed the minimum chunk size requirement.
Additional padding text to ensure we have enough bytes for the demonstration.
The service is fully async and supports concurrent operations for better performance.";
    
    println!("1. Writing sample data to file...");
    let write_result = file_service.write_file_data(
        test_file_path,
        sample_data,
        WriteOptions {
            create_dirs: true,
            calculate_checksums: true,
            sync: true,
            ..Default::default()
        },
    ).await?;
    
    println!("   ✓ Wrote {} bytes", write_result.bytes_written);
    if let Some(checksum) = &write_result.checksum {
        println!("   ✓ File checksum: {}", checksum);
    }
    
    // Demo 2: Read the file using regular I/O
    println!("\n2. Reading file using regular I/O...");
    let read_result = file_service.read_file_chunks(
        test_file_path,
        ReadOptions {
            chunk_size: Some(1024), // Minimum chunk size for demo
            calculate_checksums: true,
            use_memory_mapping: false,
            ..Default::default()
        },
    ).await?;
    
    println!("   ✓ Read {} bytes in {} chunks", 
             read_result.bytes_read, 
             read_result.chunks.len());
    println!("   ✓ File size: {} bytes", read_result.file_info.size);
    println!("   ✓ Memory mapped: {}", read_result.file_info.is_memory_mapped);
    
    // Demo 3: Read the file using memory mapping
    println!("\n3. Reading file using memory mapping...");
    let mmap_result = file_service.read_file_mmap(
        test_file_path,
        ReadOptions {
            chunk_size: Some(1024),
            calculate_checksums: true,
            ..Default::default()
        },
    ).await?;
    
    println!("   ✓ Read {} bytes in {} chunks", 
             mmap_result.bytes_read, 
             mmap_result.chunks.len());
    println!("   ✓ Memory mapped: {}", mmap_result.file_info.is_memory_mapped);
    
    // Demo 4: Verify file integrity
    println!("\n4. Verifying file integrity...");
    let calculated_checksum = file_service.calculate_file_checksum(test_file_path).await?;
    println!("   ✓ Calculated checksum: {}", calculated_checksum);
    
    if let Some(original_checksum) = &write_result.checksum {
        let is_valid = file_service.validate_file_integrity(
            test_file_path, 
            original_checksum
        ).await?;
        println!("   ✓ Integrity check: {}", if is_valid { "PASSED" } else { "FAILED" });
    }
    
    // Demo 5: File operations
    println!("\n5. Performing file operations...");
    let copy_path = Path::new("demo_test_file_copy.txt");
    let copy_result = file_service.copy_file(
        test_file_path,
        copy_path,
        WriteOptions::default(),
    ).await?;
    
    println!("   ✓ Copied file ({} bytes)", copy_result.bytes_written);
    
    // Demo 6: Get file information
    println!("\n6. Getting file information...");
    let file_info = file_service.get_file_info(test_file_path).await?;
    println!("   ✓ File path: {}", file_info.path.display());
    println!("   ✓ File size: {} bytes", file_info.size);
    println!("   ✓ Modified: {:?}", file_info.modified_at);
    
    // Demo 7: Show statistics
    println!("\n7. Service statistics:");
    let stats = file_service.get_stats();
    println!("   ✓ Bytes read: {}", stats.bytes_read);
    println!("   ✓ Bytes written: {}", stats.bytes_written);
    println!("   ✓ Chunks processed: {}", stats.chunks_processed);
    println!("   ✓ Files processed: {}", stats.files_processed);
    println!("   ✓ Memory mapped files: {}", stats.memory_mapped_files);
    println!("   ✓ Total processing time: {} ms", stats.total_processing_time_ms);
    
    // Demo 8: Stream processing
    println!("\n8. Streaming file chunks...");
    let mut stream = file_service.stream_file_chunks(
        test_file_path,
        ReadOptions {
            chunk_size: Some(1024),
            ..Default::default()
        },
    ).await?;
    
    use futures::StreamExt;
    let mut chunk_count = 0;
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                chunk_count += 1;
                println!("   ✓ Streamed chunk {} ({} bytes)", 
                         chunk.sequence_number(), 
                         chunk.data_len());
            }
            Err(e) => {
                println!("   ✗ Error streaming chunk: {}", e);
                break;
            }
        }
    }
    
    // Cleanup
    println!("\n9. Cleaning up...");
    file_service.delete_file(test_file_path).await?;
    file_service.delete_file(copy_path).await?;
    println!("   ✓ Deleted test files");
    
    println!("\nDemo completed successfully!");
    Ok(())
}
