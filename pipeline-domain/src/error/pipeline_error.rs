// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Domain Error System
//!
//! This module provides a comprehensive, hierarchical error system for the
//! adaptive pipeline domain. It implements a robust error handling strategy
//! that categorizes failures, provides actionable error messages, and supports
//! both automated error recovery and human-readable diagnostics.
//!
//! ## Overview
//!
//! The error system is designed around Domain-Driven Design principles:
//!
//! - **Domain-Specific**: Errors are tailored to pipeline processing domain
//!   concepts
//! - **Hierarchical**: Errors are organized into logical categories for
//!   systematic handling
//! - **Actionable**: Each error provides sufficient context for debugging and
//!   recovery
//! - **Type-Safe**: Rust's type system ensures comprehensive error handling
//! - **Interoperable**: Seamless integration with standard library and
//!   third-party errors
//!
//! ## Error Architecture
//!
//! ### Error Categories
//!
//! The error system organizes failures into logical categories:
//!
//! #### Configuration Errors
//! - **InvalidConfiguration**: Malformed or missing configuration settings
//! - **IncompatibleStage**: Pipeline stages with incompatible configurations
//! - **ValidationError**: Data validation failures
//!
//! #### Processing Errors
//! - **ProcessingFailed**: General pipeline processing failures
//! - **CompressionError**: Compression/decompression operation failures
//! - **EncryptionError**: Encryption/decryption operation failures
//! - **IntegrityError**: Data integrity and checksum validation failures
//!
//! #### Security Errors
//! - **SecurityViolation**: Access control and permission violations
//! - **EncryptionError**: Cryptographic operation failures
//! - **IntegrityError**: Data tampering or corruption detection
//!
//! #### Infrastructure Errors
//! - **IoError**: File system and network I/O failures
//! - **DatabaseError**: Database operation failures
//! - **ResourceExhausted**: Memory, disk space, or other resource limitations
//! - **TimeoutError**: Operation timeout failures
//!
//! #### System Errors
//! - **InternalError**: Unexpected system failures
//! - **PluginError**: Plugin loading or execution failures
//! - **MetricsError**: Metrics collection and reporting failures
//! - **Cancelled**: User or system-initiated operation cancellation
//!
//! ## Error Handling Patterns
//!
//! ### Basic Error Creation and Handling
//!
//!
//!
//! ### Advanced Error Handling with Recovery
//!
//!
//!
//! ### Error Categorization and Logging
//!
//!
//!
//! ### Error Conversion and Interoperability
//!
//!
//!
//! ## Error Recovery Strategies
//!
//! ### Recoverable Errors
//!
//! Some errors indicate temporary conditions that can be retried:
//!
//! - **TimeoutError**: Network or I/O timeouts
//! - **ResourceExhausted**: Temporary resource limitations
//! - **IoError**: Transient file system issues
//!
//! ### Non-Recoverable Errors
//!
//! These errors indicate permanent failures requiring user intervention:
//!
//! - **SecurityViolation**: Access control violations
//! - **InvalidConfiguration**: Malformed configuration data
//! - **IntegrityError**: Data corruption or tampering
//!
//! ## Integration with External Systems
//!
//! The error system provides seamless integration with:
//!
//! - **Standard Library**: Automatic conversion from `std::io::Error`
//! - **Serialization**: Integration with `serde_json`, `toml`, and `serde_yaml`
//! - **Logging**: Structured error information for observability
//! - **Metrics**: Error categorization for monitoring and alerting
//!
//! ## Performance Considerations
//!
//! - **Zero-Cost**: Error types use `thiserror` for efficient error handling
//! - **Cloneable**: Errors can be cloned for logging and metrics without
//!   performance penalty
//! - **String Allocation**: Error messages are allocated only when errors occur
//! - **Category Lookup**: Error categorization uses efficient pattern matching
//!
//! ## Security Considerations
//!
//! - **Information Disclosure**: Error messages avoid exposing sensitive
//!   information
//! - **Audit Trail**: Security errors are clearly marked for audit logging
//! - **Attack Surface**: Error handling doesn't introduce additional attack
//!   vectors
//! - **Denial of Service**: Error creation and handling are resource-efficient

use thiserror::Error;

/// Domain-specific errors for the pipeline processing system.
///
/// This enum represents all possible errors that can occur within the domain
/// layer. Each variant includes a descriptive message and is designed to
/// provide clear information about what went wrong and potentially how to fix
/// it.
///
/// ## Design Principles
///
/// - **Specific**: Each error type represents a specific failure mode
/// - **Actionable**: Error messages provide enough context for debugging
/// - **Categorized**: Errors are grouped by type for systematic handling
/// - **Recoverable**: Some errors indicate retry-able conditions
///
/// ## Error Handling Strategy
///
#[derive(Error, Debug, Clone)]
pub enum PipelineError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Incompatible stage: {0}")]
    IncompatibleStage(String),

    #[error("Invalid chunk: {0}")]
    InvalidChunk(String),

    #[error("Processing failed: {0}")]
    ProcessingFailed(String),

    #[error("Compression error: {0}")]
    CompressionError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Integrity check failed: {0}")]
    IntegrityError(String),

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Cancelled: {0}")]
    Cancelled(String),

    #[error("Pipeline not found: {0}")]
    PipelineNotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Metrics error: {0}")]
    MetricsError(String),
}

impl PipelineError {
    /// Creates a new configuration error
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        Self::InvalidConfiguration(msg.into())
    }

    /// Creates a new processing error
    pub fn processing_failed(msg: impl Into<String>) -> Self {
        Self::ProcessingFailed(msg.into())
    }

    /// Creates a new security violation error
    pub fn security_violation(msg: impl Into<String>) -> Self {
        Self::SecurityViolation(msg.into())
    }

    /// Creates a new resource exhausted error
    pub fn resource_exhausted(msg: impl Into<String>) -> Self {
        Self::ResourceExhausted(msg.into())
    }

    /// Creates a new IO error
    pub fn io_error(msg: impl Into<String>) -> Self {
        Self::IoError(msg.into())
    }

    /// Creates a new database error
    pub fn database_error(msg: impl Into<String>) -> Self {
        Self::DatabaseError(msg.into())
    }

    /// Creates a new internal error
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }

    /// Creates a new metrics error
    pub fn metrics_error(msg: impl Into<String>) -> Self {
        Self::MetricsError(msg.into())
    }

    /// Checks if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PipelineError::TimeoutError(_) | PipelineError::ResourceExhausted(_) | PipelineError::IoError(_)
        )
    }

    /// Checks if the error is a security-related error
    pub fn is_security_error(&self) -> bool {
        matches!(
            self,
            PipelineError::SecurityViolation(_) | PipelineError::EncryptionError(_) | PipelineError::IntegrityError(_)
        )
    }

    /// Gets the error category
    pub fn category(&self) -> &'static str {
        match self {
            PipelineError::InvalidConfiguration(_) => "configuration",
            PipelineError::IncompatibleStage(_) => "configuration",
            PipelineError::InvalidChunk(_) => "data",
            PipelineError::ProcessingFailed(_) => "processing",
            PipelineError::CompressionError(_) => "compression",
            PipelineError::EncryptionError(_) => "encryption",
            PipelineError::IntegrityError(_) => "integrity",
            PipelineError::SecurityViolation(_) => "security",
            PipelineError::ResourceExhausted(_) => "resource",
            PipelineError::IoError(_) => "io",
            PipelineError::DatabaseError(_) => "database",
            PipelineError::SerializationError(_) => "serialization",
            PipelineError::ValidationError(_) => "validation",
            PipelineError::PluginError(_) => "plugin",
            PipelineError::TimeoutError(_) => "timeout",
            PipelineError::Cancelled(_) => "cancellation",
            PipelineError::PipelineNotFound(_) => "pipeline",
            PipelineError::InternalError(_) => "internal",
            PipelineError::MetricsError(_) => "metrics",
        }
    }
}

// Implement conversion from standard library errors
impl From<std::io::Error> for PipelineError {
    fn from(err: std::io::Error) -> Self {
        PipelineError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for PipelineError {
    fn from(err: serde_json::Error) -> Self {
        PipelineError::SerializationError(err.to_string())
    }
}

// NOTE: TOML and YAML error conversions removed - serialization format is infrastructure concern
// If infrastructure needs these conversions, implement them in the infrastructure layer
// The domain only needs JSON serialization for configuration parameters

// impl From<toml::de::Error> for PipelineError {
//     fn from(err: toml::de::Error) -> Self {
//         PipelineError::SerializationError(err.to_string())
//     }
// }

// impl From<serde_yaml::Error> for PipelineError {
//     fn from(err: serde_yaml::Error) -> Self {
//         PipelineError::SerializationError(err.to_string())
//     }
// }
