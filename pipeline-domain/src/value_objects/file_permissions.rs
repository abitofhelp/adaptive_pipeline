// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # File Permissions Value Objects
//!
//! This module provides comprehensive file permission management and validation
//! for the adaptive pipeline system. It includes permission modeling,
//! restoration validation, and security enforcement to ensure safe file
//! operations across different platforms.
//!
//! ## Features
//!
//! - **Permission Modeling**: Cross-platform file permission representation
//! - **Validation Rules**: Domain-driven permission validation for file
//!   operations
//! - **Restoration Safety**: Comprehensive checks for file restoration
//!   operations
//! - **Security Enforcement**: Prevention of unauthorized file access and
//!   modification
//! - **Cross-Platform Support**: Unified permission handling across Unix and
//!   Windows
//!
//! ## Architecture
//!
//! The module follows Domain-Driven Design principles with value objects for
//! permissions and domain services for validation. It integrates with the
//! pipeline's security model to ensure all file operations respect system and
//! user-defined permission constraints.
//!
//! ## Usage Examples

use crate::PipelineError;
use std::path::Path;

/// File permission requirements and validation rules for secure file
/// operations.
///
/// `FilePermissions` is a value object that encapsulates file access
/// permissions in a cross-platform manner, supporting both Unix-style
/// permission bits and Windows-style access control for comprehensive file
/// security management.
///
/// ## Key Features
///
/// - **Cross-Platform Support**: Unified permission model for Unix and Windows
/// - **Permission Validation**: Ensures operations respect security constraints
/// - **Immutable Design**: Value object semantics prevent accidental
///   modification
/// - **Standard Presets**: Common permission patterns for typical use cases
/// - **Compatibility Checking**: Validates if permissions meet requirements
///
/// ## Permission Model
///
/// The permission model supports three basic access types:
/// - **Read**: Ability to read file contents
/// - **Write**: Ability to modify file contents
/// - **Execute**: Ability to execute the file (Unix) or run as program
///   (Windows)
///
/// ## Usage Patterns
///
///
/// ## Security Considerations
///
/// - Permissions are validated before file operations
/// - Default permissions follow principle of least privilege
/// - Cross-platform compatibility ensures consistent security
/// - Mode bits provide fine-grained control on Unix systems
///
/// ## Cross-Language Compatibility
///
/// - **JSON**: Object with read/write/execute boolean fields
/// - **Go**: Struct with similar field layout and os.FileMode integration
/// - **Python**: Dataclass with stat module compatibility
/// - **Database**: Separate columns for each permission type plus mode
#[derive(Debug, Clone, PartialEq)]
pub struct FilePermissions {
    /// Whether the file can be read
    pub read: bool,
    /// Whether the file can be written
    pub write: bool,
    /// Whether the file can be executed
    pub execute: bool,
    /// Unix-style permission bits for fine-grained control
    pub mode: Option<u32>,
}

impl FilePermissions {
    /// Creates permissions for read-only access (0o444).
    ///
    /// This creates a permission set that allows reading but prevents writing
    /// or executing. Suitable for configuration files, documentation, or
    /// any content that should not be modified during pipeline operations.
    ///
    /// # Returns
    ///
    /// A `FilePermissions` instance with read-only access.
    ///
    /// # Examples
    ///
    ///
    /// # Use Cases
    ///
    /// - Configuration files that should not be modified
    /// - Documentation and reference materials
    /// - Backup files for integrity protection
    /// - Template files used for generation
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            execute: false,
            mode: Some(0o444),
        }
    }

    /// Creates permissions for read-write access (0o644).
    ///
    /// This creates a permission set that allows reading and writing but
    /// prevents execution. This is the most common permission set for data
    /// files, logs, and other content that needs to be modified during
    /// pipeline operations.
    ///
    /// # Returns
    ///
    /// A `FilePermissions` instance with read-write access.
    ///
    /// # Examples
    ///
    ///
    /// # Use Cases
    ///
    /// - Data files processed by the pipeline
    /// - Log files and output files
    /// - Temporary files during processing
    /// - Cache files and intermediate results
    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            mode: Some(0o644),
        }
    }

    /// Creates permissions for full access including execution (0o755).
    ///
    /// This creates a permission set that allows reading, writing, and
    /// executing. Should be used sparingly and only for files that
    /// genuinely need to be executable, such as scripts or binary
    /// executables.
    ///
    /// # Returns
    ///
    /// A `FilePermissions` instance with full access.
    ///
    /// # Examples
    ///
    ///
    /// # Security Warning
    ///
    /// Execute permissions should be granted carefully as they allow the file
    /// to be run as a program. Only use this for legitimate executables and
    /// scripts.
    ///
    /// # Use Cases
    ///
    /// - Shell scripts and batch files
    /// - Binary executables
    /// - Pipeline processing scripts
    /// - Utility programs and tools
    pub fn full_access() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
            mode: Some(0o755),
        }
    }

    /// Validates if the current permissions satisfy the specified requirements.
    ///
    /// This method checks whether the current permission set provides at least
    /// the access level specified in the required permissions. It uses
    /// logical AND operations to ensure all required permissions are
    /// available.
    ///
    /// # Arguments
    ///
    /// * `required` - The minimum permissions that must be satisfied
    ///
    /// # Returns
    ///
    /// `true` if current permissions meet or exceed requirements, `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    ///
    /// # Logic
    ///
    /// The satisfaction logic works as follows:
    /// - If required.read is true, self.read must be true
    /// - If required.write is true, self.write must be true
    /// - If required.execute is true, self.execute must be true
    /// - All conditions must be met for satisfaction
    pub fn satisfies(&self, required: &FilePermissions) -> bool {
        (!required.read || self.read) && (!required.write || self.write) && (!required.execute || self.execute)
    }
}

/// Domain rules for file restoration permission validation
#[derive(Debug)]
pub struct FileRestorationPermissionRules;

impl FileRestorationPermissionRules {
    /// Validates if a file can be restored to the given path
    pub fn validate_restoration_permissions(
        target_path: &Path,
        overwrite_allowed: bool,
    ) -> Result<FileRestorationPermissionValidation, PipelineError> {
        let mut validation = FileRestorationPermissionValidation::new(target_path.to_path_buf());

        // Rule 1: Target file existence check
        if target_path.exists() {
            validation.target_exists = true;
            if !overwrite_allowed {
                validation.add_violation(
                    PermissionViolationType::FileExists,
                    "Target file already exists and overwrite is not allowed".to_string(),
                );
            }
        }

        // Rule 2: Parent directory must exist or be creatable
        if let Some(parent) = target_path.parent() {
            validation.parent_directory = Some(parent.to_path_buf());
            if !parent.exists() {
                validation.parent_directory_exists = false;
                validation.add_violation(
                    PermissionViolationType::DirectoryMissing,
                    format!("Parent directory does not exist: {}", parent.display()),
                );
            }
        }

        // Rule 3: Required permissions
        validation.required_permissions = FilePermissions::read_write();

        Ok(validation)
    }
}

/// Result of file restoration permission validation
#[derive(Debug)]
pub struct FileRestorationPermissionValidation {
    /// Target file path
    pub target_path: std::path::PathBuf,
    /// Whether target file exists
    pub target_exists: bool,
    /// Parent directory path
    pub parent_directory: Option<std::path::PathBuf>,
    /// Whether parent directory exists
    pub parent_directory_exists: bool,
    /// Required permissions for restoration
    pub required_permissions: FilePermissions,
    /// List of permission violations
    pub violations: Vec<PermissionViolation>,
}

impl FileRestorationPermissionValidation {
    pub fn new(target_path: std::path::PathBuf) -> Self {
        Self {
            target_path,
            target_exists: false,
            parent_directory: None,
            parent_directory_exists: true,
            required_permissions: FilePermissions::read_write(),
            violations: Vec::new(),
        }
    }

    pub fn add_violation(&mut self, violation_type: PermissionViolationType, message: String) {
        self.violations.push(PermissionViolation {
            violation_type,
            message,
        });
    }

    /// Returns true if validation passed (no violations)
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    /// Returns all violation messages
    pub fn violation_messages(&self) -> Vec<&str> {
        self.violations.iter().map(|v| v.message.as_str()).collect()
    }
}

/// Types of permission violations
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionViolationType {
    FileExists,
    DirectoryMissing,
    InsufficientPermissions,
    ReadOnlyFile,
    ReadOnlyFilesystem,
    DiskSpaceInsufficient,
}

/// Represents a specific permission violation
#[derive(Debug, Clone)]
pub struct PermissionViolation {
    pub violation_type: PermissionViolationType,
    pub message: String,
}
