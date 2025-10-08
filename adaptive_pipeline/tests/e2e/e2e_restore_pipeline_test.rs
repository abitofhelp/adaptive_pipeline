// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # End-to-End Restore Pipeline Tests
//!
//! E2E tests for restoration pipeline: .adapipe → original file with reverse-
//! order stage processing (decryption, decompression, checksum validation).

use std::fs;
use tempfile::TempDir;

use adaptive_pipeline_domain::entities::pipeline_stage::StageType;
use adaptive_pipeline_domain::value_objects::binary_file_format::FileHeader;
use adaptive_pipeline_domain::value_objects::file_chunk::FileChunk;

// Import the restore functions from restoration module
use adaptive_pipeline::create_restoration_pipeline;

/// Tests complete restore workflow: .adapipe header → restoration pipeline with
/// proper stage ordering.
#[tokio::test]
async fn test_e2e_complete_restore_workflow() {
    let _temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create a test file header representing a processed .adapipe file
    let header = FileHeader::new(
        "e2e_test.txt".to_string(),
        42, // Small test size
        "test_checksum_123".to_string(),
    )
    .add_compression_step("brotli", 6);

    // Create restoration pipeline
    let pipeline_result = create_restoration_pipeline(&header).await;
    assert!(pipeline_result.is_ok(), "Failed to create restoration pipeline");

    let pipeline = pipeline_result.unwrap();
    assert!(
        !pipeline.stages().is_empty(),
        "Pipeline should have at least verification stage"
    );

    // Verify pipeline properties
    assert!(pipeline.name().starts_with("__restore__"));
    println!("✅ E2E complete restore workflow test passed");
}

/// Tests restoration pipeline stage ordering for multi-stage processing.
///
/// This test validates that restoration stages are properly ordered in reverse
/// of the original processing pipeline to correctly restore the original file.
///
/// # Test Scenario
/// - Creates header with compression and encryption steps
/// - Generates restoration pipeline with proper reverse ordering
/// - Validates stage count and sequence
/// - Ensures correct restoration stage types
///
/// # Expected Stage Order
/// 1. Input checksum validation
/// 2. Decryption (reverse of last applied encryption)
/// 3. Decompression (reverse of first applied compression)
/// 4. File verification
/// 5. Output checksum validation
#[tokio::test]
async fn test_e2e_restoration_stage_ordering() {
    let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
        .add_compression_step("brotli", 6) // Applied first
        .add_encryption_step("aes256gcm", "argon2", 32, 12); // Applied second

    let pipeline = create_restoration_pipeline(&header).await.unwrap();
    let stages = pipeline.stages();

    // Restoration should be in reverse order:
    // 1. input_checksum (automatic)
    // 2. Decryption (reverse of encryption - last applied)
    // 3. Decompression (reverse of compression - first applied)
    // 4. Verification (always present)
    // 5. output_checksum (automatic)
    assert_eq!(stages.len(), 5);
    assert_eq!(stages[0].name(), "input_checksum");
    assert_eq!(stages[1].name(), "decryption");
    assert_eq!(stages[2].name(), "decompression");
    assert_eq!(stages[3].name(), "verification");
    assert_eq!(stages[4].name(), "output_checksum");

    println!("✅ E2E restoration stage ordering test passed");
}

/// End-to-end test for file header serialization/deserialization roundtrip
#[tokio::test]
async fn test_e2e_file_header_roundtrip() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Create complex header with multiple processing steps
    let original_header = FileHeader::new(
        "complex_e2e_test.txt".to_string(),
        1024 * 1024, // 1MB
        "original_checksum_abc123".to_string(),
    )
    .add_compression_step("brotli", 6)
    .add_encryption_step("aes256gcm", "argon2", 32, 12)
    .with_chunk_info(1024 * 1024, 1)
    .with_pipeline_id("e2e-test-pipeline".to_string())
    .with_output_checksum("output_checksum_def456".to_string())
    .with_metadata("test_key".to_string(), "test_value".to_string());

    // Serialize to footer bytes
    let footer_bytes = original_header.to_footer_bytes().expect("Failed to serialize header");

    // Create a test .adapipe file
    let adapipe_path = temp_dir.path().join("e2e_test.adapipe");
    let test_data = b"This is end-to-end test data for roundtrip testing.";

    let mut file_content = Vec::new();
    file_content.extend_from_slice(test_data);
    file_content.extend_from_slice(&footer_bytes);

    fs::write(&adapipe_path, &file_content).expect("Failed to write test file");

    // Read back and deserialize
    let read_data = fs::read(&adapipe_path).expect("Failed to read test file");
    let (restored_header, footer_size) =
        FileHeader::from_footer_bytes(&read_data).expect("Failed to deserialize header");

    // Verify complete roundtrip
    assert_eq!(restored_header.original_filename, original_header.original_filename);
    assert_eq!(restored_header.original_size, original_header.original_size);
    assert_eq!(restored_header.original_checksum, original_header.original_checksum);
    assert_eq!(restored_header.output_checksum, original_header.output_checksum);
    assert_eq!(restored_header.pipeline_id, original_header.pipeline_id);
    assert_eq!(
        restored_header.processing_steps.len(),
        original_header.processing_steps.len()
    );

    // Verify processing capabilities
    assert!(restored_header.is_compressed());
    assert!(restored_header.is_encrypted());
    assert_eq!(restored_header.compression_algorithm(), Some("brotli"));
    assert_eq!(restored_header.encryption_algorithm(), Some("aes256gcm"));

    // Verify footer size calculation
    assert_eq!(footer_size, footer_bytes.len());
    assert_eq!(read_data.len() - footer_size, test_data.len());

    println!("✅ E2E file header roundtrip test passed");
}

/// End-to-end test for restoration pipeline with real-world scenario
#[tokio::test]
async fn test_e2e_real_world_document_restoration() {
    // Simulate a real-world .adapipe file scenario
    let header = FileHeader::new(
        "document.pdf".to_string(),
        5 * 1024 * 1024, // 5MB document
        "real_world_checksum_abc123def456".to_string(),
    )
    .add_compression_step("brotli", 9) // High compression
    .add_encryption_step("aes256gcm", "argon2", 32, 12)
    .with_chunk_info(1024 * 1024, 5) // 5 chunks of 1MB each
    .with_pipeline_id("document-processing-pipeline-v2".to_string())
    .with_output_checksum("processed_checksum_789xyz".to_string())
    .with_metadata("original_path".to_string(), "/documents/important.pdf".to_string());

    // Create restoration pipeline
    let pipeline = create_restoration_pipeline(&header)
        .await
        .expect("Failed to create restoration pipeline");

    // Verify pipeline structure for real-world scenario
    assert_eq!(pipeline.stages().len(), 5); // input_checksum + decryption + decompression + verification + output_checksum

    let stages = pipeline.stages();

    // Verify input checksum stage
    let input_checksum_stage = &stages[0];
    assert_eq!(input_checksum_stage.name(), "input_checksum");
    assert_eq!(input_checksum_stage.stage_type(), &StageType::Checksum);

    // Verify decryption stage
    let decryption_stage = &stages[1];
    assert_eq!(decryption_stage.name(), "decryption");
    assert_eq!(decryption_stage.stage_type(), &StageType::Encryption); // Decryption uses Encryption type
    assert_eq!(decryption_stage.configuration().algorithm, "aes256gcm");

    // Verify decompression stage
    let decompression_stage = &stages[2];
    assert_eq!(decompression_stage.name(), "decompression");
    assert_eq!(decompression_stage.stage_type(), &StageType::Compression);
    assert_eq!(decompression_stage.configuration().algorithm, "brotli");

    // Verify verification stage
    let verification_stage = &stages[3];
    assert_eq!(verification_stage.name(), "verification");
    assert_eq!(verification_stage.stage_type(), &StageType::Checksum);
    assert_eq!(verification_stage.configuration().algorithm, "sha256");

    // Verify output checksum stage
    let output_checksum_stage = &stages[4];
    assert_eq!(output_checksum_stage.name(), "output_checksum");
    assert_eq!(output_checksum_stage.stage_type(), &StageType::Checksum);

    // Verify pipeline naming for real-world scenario
    assert!(pipeline.name().starts_with("__restore__"));
    assert!(pipeline.name().contains("document-processing-pipeline-v2"));

    // Verify stage configurations are appropriate for large files
    for stage in stages.iter() {
        // Only check chunk size for user-defined stages, not automatic checksum stages
        if stage.name() != "input_checksum" && stage.name() != "output_checksum" {
            assert_eq!(stage.configuration().chunk_size, Some(1024 * 1024));
            // 1MB chunks
        }
        assert!(!stage.configuration().parallel_processing); // Sequential for
                                                             // restoration
    }

    println!("✅ E2E real-world document restoration test passed");
}

/// End-to-end test for file chunk processing in restoration context
#[tokio::test]
async fn test_e2e_file_chunk_processing() {
    // Test file chunk creation and processing for restoration
    let test_scenarios = vec![
        (vec![1, 2, 3, 4, 5], 0, 0, false),           // Small chunk, first
        (vec![0; 1024], 1, 5, false),                 // 1KB chunk, middle
        (vec![0xFF; 1024 * 1024], 2, 1024 + 5, true), // 1MB chunk, final
    ];

    for (data, seq, offset, is_final) in test_scenarios {
        let chunk_result = FileChunk::new(seq, offset, data.clone(), is_final);

        assert!(
            chunk_result.is_ok(),
            "Failed to create chunk for seq {}: {:?}",
            seq,
            chunk_result.err()
        );

        let chunk = chunk_result.unwrap();
        assert_eq!(chunk.sequence_number(), seq);
        assert_eq!(chunk.offset(), offset);
        assert_eq!(chunk.data(), &data);
        assert_eq!(chunk.is_final(), is_final);
    }

    println!("✅ E2E file chunk processing test passed");
}

/// End-to-end test for multi-stage restoration pipeline validation
#[tokio::test]
async fn test_e2e_multi_stage_restoration_validation() {
    // Create a complex header with multiple processing steps
    let header = FileHeader::new(
        "multi_stage_test.bin".to_string(),
        2 * 1024 * 1024, // 2MB file
        "multi_stage_checksum_123".to_string(),
    )
    .add_compression_step("brotli", 8)
    .add_encryption_step("aes256gcm", "argon2", 32, 12)
    .with_chunk_info(512 * 1024, 4) // 512KB chunks, 4 total
    .with_pipeline_id("multi-stage-e2e-test".to_string())
    .with_output_checksum("multi_stage_output_456".to_string());

    // Create restoration pipeline
    let pipeline = create_restoration_pipeline(&header)
        .await
        .expect("Failed to create multi-stage restoration pipeline");

    // Verify all stages are present and correctly ordered
    let stages = pipeline.stages();
    assert_eq!(stages.len(), 5);

    // Verify stage names and types
    let expected_stages = [
        ("input_checksum", StageType::Checksum),
        ("decryption", StageType::Encryption),
        ("decompression", StageType::Compression),
        ("verification", StageType::Checksum),
        ("output_checksum", StageType::Checksum),
    ];

    for (i, (expected_name, expected_type)) in expected_stages.iter().enumerate() {
        assert_eq!(stages[i].name(), *expected_name);
        assert_eq!(stages[i].stage_type(), expected_type);
    }

    // Verify pipeline metadata
    assert!(pipeline.name().contains("multi-stage-e2e-test"));
    assert_eq!(pipeline.stages().len(), 5);

    println!("✅ E2E multi-stage restoration validation test passed");
}
