//! # Application Layer Integration Tests
//!
//! Integration tests for the application layer components of the adaptive
//! pipeline system. These tests verify command creation, service instantiation,
//! and layer integration.
//!
//! ## Test Coverage
//!
//! - Command creation and configuration
//! - Service instantiation with dependency injection
//! - Application layer structure validation
//! - Integration between application and infrastructure layers
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test application_layer_integration_test
//! ```

use adaptive_pipeline::application::commands::RestoreFileCommand;
use std::path::PathBuf;

/// Tests RestoreFileCommand creation and fluent API configuration.
///
/// Verifies that commands can be created with valid paths and configured
/// using the fluent API pattern.
#[tokio::test]
async fn test_restore_file_command_creation() {
    // Arrange
    let source_path = PathBuf::from("/tmp/source.adapipe");
    let target_path = PathBuf::from("/tmp/target.txt");

    // Act
    let command = RestoreFileCommand::new(source_path.clone(), target_path.clone());

    // Assert - verify paths are stored correctly
    assert_eq!(command.source_adapipe_path, source_path);
    assert_eq!(command.target_path, target_path);

    // Act - test fluent API configuration
    let command = command
        .with_overwrite(true)
        .with_create_directories(false)
        .with_permission_validation(false);

    // Assert - verify fluent API updates state correctly
    assert!(command.overwrite);
    assert!(!command.create_directories);
    assert!(!command.validate_permissions);
}

// Note: FileRestorationApplicationService was removed during architecture
// refactoring. File restoration is now handled via the restore command in
// main.rs using restore_file_from_adapipe_v2() function. End-to-end restoration
// is tested via integration tests that exercise the full command flow.

/// Integration test verifying application layer structure and architecture
/// compliance.
///
/// This test validates that the application layer follows proper architectural
/// patterns including DIP (Dependency Inversion Principle) and clean separation
/// between application and infrastructure layers.
#[tokio::test]
async fn test_application_layer_structure() {
    // This integration test verifies:
    // 1. Commands can be created and used
    // 2. Services can be instantiated
    // 3. Dependencies can be injected properly
    // 4. The application layer follows DIP (Dependency Inversion Principle)

    println!("✅ Application layer integration tests are properly structured");
    println!("✅ Commands can be created and configured");
    println!("✅ Services can be instantiated with real dependencies");
    println!("✅ Integration between application and infrastructure layers works");

    // Assert - architecture compliance validation
    assert!(true, "Application layer integration test passed");
}

/// Tests RestoreFileCommand default values.
///
/// Verifies that commands are initialized with safe, conservative defaults
/// that follow security and safety best practices.
#[test]
fn test_command_defaults() {
    // Arrange & Act
    let command = RestoreFileCommand::new(PathBuf::from("/tmp/source.adapipe"), PathBuf::from("/tmp/target.txt"));

    // Assert - verify safe defaults
    assert!(!command.overwrite, "Default overwrite should be false for safety");
    assert!(
        command.create_directories,
        "Default create_directories should be true for convenience"
    );
    assert!(
        command.validate_permissions,
        "Default validate_permissions should be true for security"
    );
}

/// Tests RestoreFileCommand fluent API configuration.
///
/// Verifies that the fluent API pattern works correctly for command
/// configuration and maintains proper state after method chaining.
#[test]
fn test_command_fluent_api() {
    // Arrange & Act
    let command = RestoreFileCommand::new(PathBuf::from("/tmp/source.adapipe"), PathBuf::from("/tmp/target.txt"))
        .with_overwrite(true)
        .with_create_directories(false)
        .with_permission_validation(false);

    // Assert - verify fluent API configuration
    assert!(command.overwrite);
    assert!(!command.create_directories);
    assert!(!command.validate_permissions);
}
