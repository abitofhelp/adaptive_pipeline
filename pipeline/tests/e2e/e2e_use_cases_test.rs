//! # End-to-End Use Case Tests
//!
//! E2E tests for all use cases via the CLI interface. These tests verify
//! complete workflows from CLI invocation through to final results, using
//! real pipeline binaries and file I/O.

use std::process::Command;
use tempfile::TempDir;
use tokio::fs;

/// Helper to get the pipeline binary path
fn get_pipeline_bin() -> &'static str {
    env!("CARGO_BIN_EXE_pipeline")
}

/// Helper to calculate SHA256 checksum
fn calculate_sha256(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Tests CreatePipelineUseCase via CLI
#[tokio::test]
async fn test_e2e_create_pipeline_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_create.db");

    // Create a pipeline using the CLI (which uses CreatePipelineUseCase)
    let output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-create-uc", "--stages", "brotli,aes256gcm"])
        .output()
        .expect("Failed to run create command");

    assert!(
        output.status.success(),
        "Pipeline creation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test-create-uc"), "Pipeline name not in output");
}

/// Tests ListPipelinesUseCase via CLI
#[tokio::test]
async fn test_e2e_list_pipelines_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_list.db");

    // Create multiple pipelines
    for name in &["test-list-1", "test-list-2", "test-list-3"] {
        Command::new(get_pipeline_bin())
            .env("ADAPIPE_SQLITE_PATH", &db_path)
            .args(&["create", "--name", name, "--stages", "brotli"])
            .output()
            .expect("Failed to create pipeline");
    }

    // List pipelines using CLI (which uses ListPipelinesUseCase)
    let output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["list"])
        .output()
        .expect("Failed to run list command");

    assert!(
        output.status.success(),
        "List command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test-list-1"), "Pipeline 1 not listed");
    assert!(stdout.contains("test-list-2"), "Pipeline 2 not listed");
    assert!(stdout.contains("test-list-3"), "Pipeline 3 not listed");
    assert!(stdout.contains("Found 3 pipeline(s)") || stdout.contains("3"), "Count not shown");
}

/// Tests ShowPipelineUseCase via CLI
#[tokio::test]
async fn test_e2e_show_pipeline_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_show.db");

    // Create a pipeline with multiple stages
    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-show-uc", "--stages", "brotli,aes256gcm,sha256"])
        .output()
        .expect("Failed to create pipeline");

    // Show pipeline details using CLI (which uses ShowPipelineUseCase)
    let output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["show", "test-show-uc"])
        .output()
        .expect("Failed to run show command");

    assert!(
        output.status.success(),
        "Show command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test-show-uc"), "Pipeline name not shown");
    assert!(stdout.contains("brotli"), "Stage 1 not shown");
    assert!(stdout.contains("aes256gcm"), "Stage 2 not shown");
    assert!(stdout.contains("sha256"), "Stage 3 not shown");
}

/// Tests DeletePipelineUseCase via CLI
#[tokio::test]
async fn test_e2e_delete_pipeline_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_delete.db");

    // Create a pipeline
    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-delete-uc", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    // Delete using CLI with --force (which uses DeletePipelineUseCase)
    let output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["delete", "test-delete-uc", "--force"])
        .output()
        .expect("Failed to run delete command");

    assert!(
        output.status.success(),
        "Delete command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify pipeline is gone
    let list_output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["list"])
        .output()
        .expect("Failed to list pipelines");

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(!stdout.contains("test-delete-uc"), "Pipeline still exists after deletion");
}

/// Tests ProcessFileUseCase via CLI
#[tokio::test]
async fn test_e2e_process_file_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_process.db");
    let input_file = temp_dir.path().join("input.txt");
    let output_file = temp_dir.path().join("output.adapipe");

    // Create test input
    let test_data = b"ProcessFileUseCase E2E test data.\n".repeat(50);
    fs::write(&input_file, &test_data).await.unwrap();

    // Create pipeline
    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-process-uc", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    // Process file using CLI (which uses ProcessFileUseCase)
    let output = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&[
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--pipeline",
            "test-process-uc",
        ])
        .output()
        .expect("Failed to run process command");

    assert!(
        output.status.success(),
        "Process command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output file was created
    assert!(output_file.exists(), ".adapipe file was not created");

    // Verify it's a valid .adapipe file
    let file_data = fs::read(&output_file).await.unwrap();
    assert!(file_data.len() > 0, "Output file is empty");
}

/// Tests ValidateConfigUseCase via CLI
#[tokio::test]
async fn test_e2e_validate_config_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("test_config.toml");

    // Create a valid TOML config
    let config_content = r#"
[global]
default_chunk_size = "64KB"
default_workers = 4

[pipelines.test-pipeline]
name = "test-pipeline"
stages = ["brotli", "aes256gcm"]
"#;
    fs::write(&config_file, config_content).await.unwrap();

    // Validate using CLI (which uses ValidateConfigUseCase)
    let output = Command::new(get_pipeline_bin())
        .args(&["validate", config_file.to_str().unwrap()])
        .output()
        .expect("Failed to run validate command");

    assert!(
        output.status.success(),
        "Validate config command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("valid") || stdout.contains("✅"),
        "Success message not shown"
    );
}

/// Tests ValidateConfigUseCase with invalid config
#[tokio::test]
async fn test_e2e_validate_invalid_config_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("invalid_config.toml");

    // Create an invalid TOML config (malformed syntax)
    let invalid_content = r#"
[global
missing_closing_bracket = "oops"
"#;
    fs::write(&config_file, invalid_content).await.unwrap();

    // Validate using CLI - should fail
    let output = Command::new(get_pipeline_bin())
        .args(&["validate", config_file.to_str().unwrap()])
        .output()
        .expect("Failed to run validate command");

    assert!(
        !output.status.success(),
        "Validate should fail for invalid config"
    );
}

/// Tests ValidateFileUseCase via CLI
#[tokio::test]
async fn test_e2e_validate_file_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_validate_file.db");
    let input_file = temp_dir.path().join("input.txt");
    let adapipe_file = temp_dir.path().join("output.adapipe");

    // Create and process a file to get a valid .adapipe file
    let test_data = b"ValidateFileUseCase E2E test.\n".repeat(20);
    fs::write(&input_file, &test_data).await.unwrap();

    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-validate-file", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&[
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            adapipe_file.to_str().unwrap(),
            "--pipeline",
            "test-validate-file",
        ])
        .output()
        .expect("Failed to process file");

    // Validate the .adapipe file using CLI (which uses ValidateFileUseCase)
    let output = Command::new(get_pipeline_bin())
        .args(&["validate-file", "--file", adapipe_file.to_str().unwrap()])
        .output()
        .expect("Failed to run validate-file command");

    assert!(
        output.status.success(),
        "Validate file command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("valid") || stdout.contains("✅"),
        "Success message not shown"
    );
}

/// Tests CompareFilesUseCase via CLI
#[tokio::test]
async fn test_e2e_compare_files_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_compare.db");
    let input_file = temp_dir.path().join("original.txt");
    let adapipe_file = temp_dir.path().join("original.adapipe");

    // Create test file
    let test_data = b"CompareFilesUseCase E2E test data.\n".repeat(30);
    fs::write(&input_file, &test_data).await.unwrap();

    // Create pipeline and process
    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-compare", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&[
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            adapipe_file.to_str().unwrap(),
            "--pipeline",
            "test-compare",
        ])
        .output()
        .expect("Failed to process file");

    // Compare files using CLI (which uses CompareFilesUseCase)
    let output = Command::new(get_pipeline_bin())
        .args(&[
            "compare",
            "--original",
            input_file.to_str().unwrap(),
            "--adapipe",
            adapipe_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run compare command");

    assert!(
        output.status.success(),
        "Compare command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("identical") || stdout.contains("match"),
        "Files should be identical"
    );
}

/// Tests CompareFilesUseCase with modified file
#[tokio::test]
async fn test_e2e_compare_files_modified_use_case() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_compare_mod.db");
    let input_file = temp_dir.path().join("original.txt");
    let modified_file = temp_dir.path().join("modified.txt");
    let adapipe_file = temp_dir.path().join("original.adapipe");

    // Create original file
    let test_data = b"Original data.\n".repeat(20);
    fs::write(&input_file, &test_data).await.unwrap();

    // Create modified version
    let modified_data = b"Modified data - different content.\n".repeat(20);
    fs::write(&modified_file, &modified_data).await.unwrap();

    // Create pipeline and process original
    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", "test-compare-mod", "--stages", "brotli"])
        .output()
        .expect("Failed to create pipeline");

    Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&[
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            adapipe_file.to_str().unwrap(),
            "--pipeline",
            "test-compare-mod",
        ])
        .output()
        .expect("Failed to process file");

    // Compare modified file against original .adapipe
    let output = Command::new(get_pipeline_bin())
        .args(&[
            "compare",
            "--original",
            modified_file.to_str().unwrap(),
            "--adapipe",
            adapipe_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run compare command");

    assert!(
        output.status.success(),
        "Compare command should succeed even with different files"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("differ") || stdout.contains("not identical") || stdout.contains("❌"),
        "Should detect file differences"
    );
}

/// Tests BenchmarkSystemUseCase via CLI (smoke test only - full benchmark takes too long)
#[tokio::test]
async fn test_e2e_benchmark_system_use_case_smoke() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("benchmark_input.txt");

    // Create small test file
    let test_data = b"Benchmark test data.\n".repeat(100);
    fs::write(&test_file, &test_data).await.unwrap();

    // Run minimal benchmark (1 iteration, small size)
    let output = Command::new(get_pipeline_bin())
        .args(&[
            "benchmark",
            "--file",
            test_file.to_str().unwrap(),
            "--size-mb",
            "1",
            "--iterations",
            "1",
        ])
        .output()
        .expect("Failed to run benchmark command");

    assert!(
        output.status.success(),
        "Benchmark command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Benchmark") || stdout.contains("throughput") || stdout.contains("MB/s"),
        "Benchmark results not shown"
    );
}

/// Tests complete workflow: create → process → validate → compare → delete
#[tokio::test]
async fn test_e2e_complete_workflow_all_use_cases() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_workflow.db");
    let input_file = temp_dir.path().join("workflow_input.txt");
    let output_file = temp_dir.path().join("workflow_output.adapipe");

    let test_data = b"Complete workflow test data.\n".repeat(100);
    fs::write(&input_file, &test_data).await.unwrap();

    let pipeline_name = "test-workflow";

    // Step 1: Create pipeline (CreatePipelineUseCase)
    // Note: Using only brotli to avoid needing encryption key configuration
    let create = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["create", "--name", pipeline_name, "--stages", "brotli"])
        .output()
        .expect("Create failed");
    assert!(create.status.success(), "Create failed");

    // Step 2: List pipelines (ListPipelinesUseCase)
    let list = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["list"])
        .output()
        .expect("List failed");
    assert!(list.status.success(), "List failed");
    assert!(String::from_utf8_lossy(&list.stdout).contains(pipeline_name));

    // Step 3: Show pipeline details (ShowPipelineUseCase)
    let show = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["show", pipeline_name])
        .output()
        .expect("Show failed");
    assert!(show.status.success(), "Show failed");

    // Step 4: Process file (ProcessFileUseCase)
    let process = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&[
            "process",
            "--input",
            input_file.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--pipeline",
            pipeline_name,
        ])
        .output()
        .expect("Process failed");
    assert!(
        process.status.success(),
        "Process failed: stdout={}, stderr={}",
        String::from_utf8_lossy(&process.stdout),
        String::from_utf8_lossy(&process.stderr)
    );
    assert!(output_file.exists(), "Output file not created");

    // Step 5: Validate .adapipe file (ValidateFileUseCase)
    let validate = Command::new(get_pipeline_bin())
        .args(&["validate-file", "--file", output_file.to_str().unwrap()])
        .output()
        .expect("Validate file failed");
    assert!(validate.status.success(), "Validate file failed");

    // Step 6: Compare files (CompareFilesUseCase)
    let compare = Command::new(get_pipeline_bin())
        .args(&[
            "compare",
            "--original",
            input_file.to_str().unwrap(),
            "--adapipe",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Compare failed");
    assert!(compare.status.success(), "Compare failed");

    // Step 7: Delete pipeline (DeletePipelineUseCase)
    let delete = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["delete", pipeline_name, "--force"])
        .output()
        .expect("Delete failed");
    assert!(delete.status.success(), "Delete failed");

    // Verify deletion
    let list_after = Command::new(get_pipeline_bin())
        .env("ADAPIPE_SQLITE_PATH", &db_path)
        .args(&["list"])
        .output()
        .expect("List after delete failed");
    assert!(!String::from_utf8_lossy(&list_after.stdout).contains(pipeline_name));
}
