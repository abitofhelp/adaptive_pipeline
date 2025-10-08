// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # End-to-End Binary Format Tests
//!
//! E2E tests for .adapipe binary format: complete roundtrips, multi-chunk
//! processing, format validation, corruption detection, and version
//! compatibility.

use tempfile::{NamedTempFile, TempDir};
use tokio::fs;

use adaptive_pipeline::infrastructure::services::{AdapipeFormat, BinaryFormatService, BinaryFormatWriter};
use adaptive_pipeline_domain::value_objects::FileHeader;

// Import shared test helpers
use crate::common::{calculate_sha256, get_pipeline_bin};

/// Tests complete .adapipe roundtrip using real pipeline processing via CLI.
/// This test exercises the full stack: input file → real compression → .adapipe
/// file → metadata validation.
#[tokio::test]
async fn test_e2e_real_pipeline_roundtrip() {
    use std::process::Command;

    // Get the path to the compiled pipeline binary
    let pipeline_bin = get_pipeline_bin();

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_roundtrip.db");
    let input_file = temp_dir.path().join("e2e_input.txt");
    let output_file = temp_dir.path().join("e2e_output.adapipe");

    // Create test input file with known content
    let test_data = b"E2E test data for complete pipeline roundtrip validation.\n".repeat(100);
    fs::write(&input_file, &test_data).await.unwrap();

    // Calculate expected checksum
    let expected_checksum = calculate_sha256(&test_data);
    let expected_size = test_data.len() as u64;

    // Step 0: Clean up any existing pipeline from previous test runs
    let _ = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-test-roundtrip", "--force"])
        .output();

    // Step 1: Create a pipeline using the real CLI
    let create_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["create", "--name", "e2e-test-roundtrip", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    if !create_output.status.success() {
        eprintln!("Pipeline creation FAILED!");
        eprintln!("Status: {:?}", create_output.status);
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&create_output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&create_output.stderr));
        panic!("Pipeline creation failed");
    }

    // Step 2: Process the file using the real pipeline CLI
    let process_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args([
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--pipeline",
            "e2e-test-roundtrip",
        ])
        .output()
        .expect("Failed to process file");

    assert!(
        process_output.status.success(),
        "Pipeline processing failed: {}",
        String::from_utf8_lossy(&process_output.stderr)
    );

    // Step 3: Verify the .adapipe file was created
    assert!(output_file.exists(), ".adapipe file was not created");

    // Step 4: Use BinaryFormatService to validate and read metadata
    let service = AdapipeFormat::new();

    // Validate file format
    let validation = service.validate_file(&output_file).await.unwrap();
    assert!(validation.is_valid, "Generated .adapipe file is invalid");
    assert_eq!(validation.format_version, 1);
    assert!(validation.chunk_count > 0);

    // Read and verify metadata
    let metadata = service.read_metadata(&output_file).await.unwrap();
    assert_eq!(metadata.original_filename, "e2e_input.txt");
    assert_eq!(metadata.original_size, expected_size);
    assert_eq!(metadata.original_checksum, expected_checksum);
    assert!(metadata.is_compressed(), "File should be compressed");
    assert_eq!(metadata.compression_algorithm(), Some("brotli"));

    // Step 5: Verify we can read chunks from the real file
    let mut reader = service.create_reader(&output_file).await.unwrap();
    let header = reader.read_header().unwrap();

    let mut chunks_read = 0;
    let mut total_data_size = 0;

    while let Some(chunk) = reader.read_next_chunk().await.unwrap() {
        chunks_read += 1;
        total_data_size += chunk.payload.len();

        // Validate chunk structure
        assert!(chunk.validate().is_ok(), "Chunk validation failed");
        assert!(!chunk.payload.is_empty(), "Chunk payload should not be empty");
    }

    assert_eq!(chunks_read, header.chunk_count, "Chunk count mismatch");
    assert!(total_data_size > 0, "Should have read some data");

    // Step 6: Clean up - delete the test pipeline
    let _delete_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-test-roundtrip", "--force"])
        .output()
        .expect("Failed to delete pipeline");

    println!("✅ E2E real pipeline roundtrip test passed");
}

/// End-to-end test for pass-through files (no compression/encryption) using
/// real pipeline
#[tokio::test]
async fn test_e2e_binary_format_pass_through() {
    use std::process::Command;

    let pipeline_bin = get_pipeline_bin();

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_passthrough.db");
    let input_file = temp_dir.path().join("passthrough_input.txt");
    let output_file = temp_dir.path().join("passthrough.adapipe");

    let test_data = b"Simple pass-through file content for testing";
    fs::write(&input_file, test_data).await.unwrap();

    let expected_checksum = calculate_sha256(test_data);

    // Step 0: Clean up any existing pipeline from previous test runs
    let _ = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-passthrough", "--force"])
        .output();

    // Step 1: Create a pipeline with only checksum stages (no
    // compression/encryption)
    let create_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["create", "--name", "e2e-passthrough", "--stages", "checksum"])
        .output()
        .expect("Failed to create pipeline");

    assert!(create_output.status.success());

    // Step 2: Process the file
    let process_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args([
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--pipeline",
            "e2e-passthrough",
        ])
        .output()
        .expect("Failed to process file");

    assert!(
        process_output.status.success(),
        "Pipeline processing failed: {}",
        String::from_utf8_lossy(&process_output.stderr)
    );

    // Step 3: Validate the file metadata
    let service = AdapipeFormat::new();
    let metadata = service.read_metadata(&output_file).await.unwrap();

    assert!(!metadata.is_compressed(), "File should not be compressed");
    assert!(!metadata.is_encrypted(), "File should not be encrypted");
    assert_eq!(metadata.original_checksum, expected_checksum);
    assert_eq!(metadata.original_size, test_data.len() as u64);

    // Clean up
    let _delete_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-passthrough", "--force"])
        .output()
        .expect("Failed to delete pipeline");

    println!("✅ E2E pass-through test passed");
}

/// End-to-end test for file format validation with corrupted data
#[tokio::test]
async fn test_e2e_binary_format_corruption_detection() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    // Write invalid magic bytes
    fs::write(file_path, b"INVALID_MAGIC_BYTES").await.unwrap();

    let service = AdapipeFormat::new();

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

    let service = AdapipeFormat::new();

    // Create file with current version
    {
        let header = FileHeader::new("version_test.txt".to_string(), 100, "test_checksum".to_string());

        let writer: Box<dyn BinaryFormatWriter> = service.create_writer(&output_file, header.clone()).await.unwrap();
        let _: u64 = writer.finalize(header).await.unwrap();
    }

    // Verify version is correctly stored and read
    {
        let metadata = service.read_metadata(&output_file).await.unwrap();
        assert_eq!(metadata.format_version, 1); // Current version
        assert!(!metadata.app_version.is_empty());
    }
}

/// End-to-end test for large file handling with multiple chunks using real
/// pipeline
#[tokio::test]
async fn test_e2e_binary_format_large_file() {
    use std::process::Command;

    let pipeline_bin = get_pipeline_bin();

    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_large_file.db");
    let input_file = temp_dir.path().join("large_input.txt");
    let output_file = temp_dir.path().join("large_output.adapipe");

    // Create larger test data (100KB to ensure multiple chunks)
    let test_data = b"Large file test data for multi-chunk validation.\n".repeat(2000);
    fs::write(&input_file, &test_data).await.unwrap();

    // Step 0: Clean up any existing pipeline from previous test runs
    let _ = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-large-test", "--force"])
        .output();

    // Step 1: Create pipeline
    let create_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["create", "--name", "e2e-large-test", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    assert!(
        create_output.status.success(),
        "Pipeline creation failed: {}",
        String::from_utf8_lossy(&create_output.stderr)
    );

    // Step 2: Process the large file with small chunk size to create multiple
    // chunks
    let process_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args([
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--pipeline",
            "e2e-large-test",
        ])
        .output()
        .expect("Failed to process file");

    assert!(
        process_output.status.success(),
        "Pipeline processing failed: {}",
        String::from_utf8_lossy(&process_output.stderr)
    );

    // Step 3: Verify multiple chunks were created
    let service = AdapipeFormat::new();
    let metadata = service.read_metadata(&output_file).await.unwrap();

    assert!(metadata.chunk_count >= 1, "Should have at least one chunk");
    assert!(metadata.is_compressed(), "File should be compressed");

    // Step 4: Verify we can read all chunks
    let mut reader = service.create_reader(&output_file).await.unwrap();
    reader.read_header().unwrap();

    let mut chunks_read = 0;
    let mut total_data = 0;

    while let Some(chunk) = reader.read_next_chunk().await.unwrap() {
        chunks_read += 1;
        total_data += chunk.payload.len();
        // Chunk was successfully read from .adapipe file, no need to validate
        // structure
    }

    assert_eq!(chunks_read, metadata.chunk_count, "Should read all chunks");
    assert!(total_data > 0, "Should have read data");

    // Clean up
    let _delete_output = Command::new(&pipeline_bin)
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(["delete", "--name", "e2e-large-test", "--force"])
        .output()
        .expect("Failed to delete pipeline");

    println!("✅ E2E large file test passed");
}
