// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Application Commands
//!
//! This module implements the Command pattern as part of the CQRS (Command
//! Query Responsibility Segregation) architecture. Commands represent
//! operations that change system state and are designed to be immutable,
//! self-contained instructions that can be validated, logged, and executed.
//!
//! ## Overview
//!
//! Application commands provide:
//!
//! - **State Modification**: Commands represent operations that change system
//!   state
//! - **Validation**: Commands can be validated before execution
//! - **Auditability**: Commands provide a clear audit trail of operations
//! - **Testability**: Commands can be easily unit tested in isolation
//! - **Serialization**: Commands can be serialized for queuing or persistence
//!
//! ## CQRS Architecture
//!
//! ### Command Characteristics
//!
//! Commands in this system have the following characteristics:
//!
//! - **Immutable**: Once created, commands cannot be modified
//! - **Self-Contained**: Commands contain all information needed for execution
//! - **Validated**: Commands are validated before execution
//! - **Auditable**: Commands provide clear operation tracking
//! - **Asynchronous**: Commands can be executed asynchronously
//!
//! ### Command Lifecycle
//!
//! The typical command lifecycle follows these steps:
//!
//! 1. **Creation**: Command is created with required parameters
//! 2. **Validation**: Command is validated for correctness
//! 3. **Authorization**: User permissions are checked
//! 4. **Execution**: Command is executed by appropriate handler
//! 5. **Result**: Command execution result is returned
//! 6. **Audit**: Command execution is logged for audit trail
//!
//! ## Command Types
//!
//! ### File Restoration Commands
//!
//! Commands related to restoring files from the pipeline format:
//!
//! - `RestoreFileCommand`: Restore a file from .adapipe format
//! - Future commands for batch restoration, selective restoration, etc.
//!
//! ## Usage Patterns
//!
//! ### Basic Command Creation and Execution

//!
//! ### Command Validation

//!
//! ### Command Builder Pattern

//!
//! ### Asynchronous Command Processing

//!
//! ## Best Practices
//!
//! ### Command Design
//!
//! - **Immutability**: Commands should be immutable after creation
//! - **Self-Validation**: Commands should validate their own parameters
//! - **Rich Information**: Include all necessary context in the command
//! - **Clear Intent**: Command names should clearly indicate their purpose
//!
//! ### Error Handling
//!
//! - **Validation Errors**: Catch validation errors before execution
//! - **Execution Errors**: Handle execution failures gracefully
//! - **Rollback**: Design commands to support rollback when possible
//! - **Audit Trail**: Log all command executions for debugging
//!
//! ### Performance
//!
//! - **Async Execution**: Use async patterns for I/O-bound operations
//! - **Batch Processing**: Group related commands for efficiency
//! - **Resource Management**: Properly manage resources during execution
//! - **Timeout Handling**: Implement appropriate timeouts for long operations
//!
//! ## Testing Strategies
//!
//! ### Unit Testing Commands

//!
//! ### Integration Testing
//!
//! Test commands with real handlers and infrastructure:

use std::path::PathBuf;

/// Command to restore a file from .adapipe format.
///
/// This command encapsulates all the information needed to restore a file from
/// the pipeline's compressed and processed format back to its original form.
/// It follows the Command pattern and supports the builder pattern for
/// flexible configuration.
///
/// ## Command Properties
///
/// - **Immutable**: Once created, the command cannot be modified (except
///   through builder methods)
/// - **Self-Contained**: Contains all information needed for execution
/// - **Validatable**: Can be validated before execution
/// - **Auditable**: Provides clear operation tracking
///
/// ## Usage Examples
///
/// ### Basic File Restoration
///
///
/// ### Advanced Configuration
///
///
/// ### Batch Processing Setup
///
///
/// ## Validation
///
/// Commands should be validated before execution to ensure:
///
/// - Source file exists and is readable
/// - Target directory is writable
/// - Sufficient disk space is available
/// - Permissions allow the operation
///
/// ## Error Handling
///
/// Command execution can fail for various reasons:
///
/// - Source file not found or corrupted
/// - Insufficient permissions
/// - Disk space exhausted
/// - Target file already exists (when overwrite is false)
/// - Invalid .adapipe format
///
/// ## Performance Considerations
///
/// - Large files may require streaming restoration
/// - Multiple commands can be processed concurrently
/// - Progress tracking may be needed for long operations
/// - Memory usage scales with file size and concurrency
#[derive(Debug, Clone)]
pub struct RestoreFileCommand {
    /// Path to the .adapipe file to restore from
    pub source_adapipe_path: PathBuf,
    /// Target directory or file path for restoration
    pub target_path: PathBuf,
    /// Whether to overwrite existing files
    pub overwrite: bool,
    /// Whether to create missing directories
    pub create_directories: bool,
    /// Whether to validate permissions before restoration
    pub validate_permissions: bool,
}

impl RestoreFileCommand {
    pub fn new(source_adapipe_path: PathBuf, target_path: PathBuf) -> Self {
        Self {
            source_adapipe_path,
            target_path,
            overwrite: false,
            create_directories: true,
            validate_permissions: true,
        }
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn with_create_directories(mut self, create_directories: bool) -> Self {
        self.create_directories = create_directories;
        self
    }

    pub fn with_permission_validation(mut self, validate: bool) -> Self {
        self.validate_permissions = validate;
        self
    }
}

/// Result of file restoration command
#[derive(Debug)]
#[allow(dead_code)]
pub struct RestoreFileResult {
    /// Path where the file was restored
    pub restored_path: PathBuf,
    /// Number of bytes restored
    pub bytes_restored: u64,
    /// Whether checksum validation passed
    pub checksum_verified: bool,
    /// Calculated checksum of restored file
    pub calculated_checksum: String,
    /// Time taken for restoration
    pub restoration_time_ms: u64,
}
