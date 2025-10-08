// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


use ulid::Ulid;

fn main() {
    println!("Generating proper ULIDs for database:");
    
    // Pipeline IDs
    let test_multi_stage_id = Ulid::new();
    let image_processing_id = Ulid::new();
    
    println!("Pipeline IDs:");
    println!("  test-multi-stage: {}", test_multi_stage_id);
    println!("  image-processing: {}", image_processing_id);
    
    // Stage IDs for test-multi-stage
    let input_checksum_id = Ulid::new();
    let compression_id = Ulid::new();
    let encryption_id = Ulid::new();
    let output_checksum_id = Ulid::new();
    
    println!("\nStage IDs for test-multi-stage:");
    println!("  input_checksum: {}", input_checksum_id);
    println!("  compression: {}", compression_id);
    println!("  encryption: {}", encryption_id);
    println!("  output_checksum: {}", output_checksum_id);
    
    // Stage IDs for image-processing
    let input_validation_id = Ulid::new();
    let image_compression_id = Ulid::new();
    
    println!("\nStage IDs for image-processing:");
    println!("  input_validation: {}", input_validation_id);
    println!("  image_compression: {}", image_compression_id);
    
    // Other IDs
    let session1_id = Ulid::new();
    let session2_id = Ulid::new();
    let chunk1_id = Ulid::new();
    let chunk2_id = Ulid::new();
    let security1_id = Ulid::new();
    let security2_id = Ulid::new();
    
    println!("\nOther IDs:");
    println!("  session1: {}", session1_id);
    println!("  session2: {}", session2_id);
    println!("  chunk1: {}", chunk1_id);
    println!("  chunk2: {}", chunk2_id);
    println!("  security1: {}", security1_id);
    println!("  security2: {}", security2_id);
}
