// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Domain Error Module
//!
//! This module provides the comprehensive error types for the domain layer,
//! implementing a structured error handling approach that categorizes all
//! possible failure scenarios in the pipeline processing system.
//!
//! ## Overview
//!
//! The error module defines domain-specific errors that:
//!
//! - **Express Business Failures**: Errors that represent violations of business rules
//! - **Maintain Type Safety**: Strongly-typed error variants for compile-time safety
//! - **Provide Context**: Rich error information for debugging and user feedback
//! - **Support Error Recovery**: Categorization enables appropriate recovery strategies
//! - **Enable Error Translation**: Clean conversion to application and interface layer errors
//!
//! ## Error Categories
//!
//! ### Configuration Errors
//! Errors related to invalid pipeline or system configuration:
//!
//!
//! ### Processing Errors
//! Errors that occur during file processing operations:
//!
//!
//! ### Validation Errors
//! Errors from input validation and business rule violations:
//!
//!
//! ### Infrastructure Errors
//! Errors from external systems and infrastructure:
//!
//!
//! ## Error Handling Patterns
//!
//! ### Pattern Matching
//! Use pattern matching for granular error handling:
//!
//!
//! ### Error Propagation
//! Use the `?` operator for clean error propagation:
//!
//!
//! ### Error Context
//! Add context to errors for better debugging:
//!
//!
//! ## Error Conversion
//!
//! Domain errors can be converted to other error types:
//!
//!
//! ## Testing with Errors
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     #[test]
//!     fn test_error_display() {
//!         // Test error messages are informative
//!     }
//!
//!     #[test]
//!     fn test_error_conversion() {
//!         // Test error type conversions
//!     }
//!
//!     #[test]
//!     fn test_error_recovery() {
//!         // Test error recovery strategies
//!     }
//! }
//! ```
//!
//! ## Best Practices
//!
//! - **Be Specific**: Use specific error variants for different failure scenarios
//! - **Include Context**: Always include relevant context in error messages
//! - **Avoid Strings**: Use typed error variants instead of generic string errors
//! - **Document Errors**: Document which errors can be returned from functions
//! - **Test Error Paths**: Ensure error handling paths are tested

mod pipeline_error;

pub use pipeline_error::PipelineError;
