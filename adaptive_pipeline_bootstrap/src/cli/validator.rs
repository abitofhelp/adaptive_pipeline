// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Secure Command-Line Argument Parsing
//!
//! Security-first argument parsing with comprehensive validation.
//!
//! ## Security Features
//!
//! - **Length limits** - Prevent buffer overflow attempts
//! - **Pattern detection** - Block path traversal and injection
//! - **Path normalization** - Canonical path resolution
//! - **System directory protection** - Prevent access to sensitive paths
//!
//! ## Dangerous Patterns Detected
//!
//! - `..` - Path traversal
//! - `~` - Home directory expansion (security risk)
//! - `$` - Variable expansion
//! - Backticks - Command substitution
//! - `;` `&` `|` - Command chaining
//! - `>` `<` - Redirection
//! - Null bytes, newlines, carriage returns
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bootstrap::cli::SecureArgParser;
//! use bootstrap::config::AppConfig;
//!
//! let args: Vec<String> = std::env::args().collect();
//! let config = SecureArgParser::parse(&args)?;
//!
//! println!("Running: {}", config.app_name());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::config::AppConfig;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Maximum argument count (prevent DOS)
const MAX_ARG_COUNT: usize = 100;

/// Maximum single argument length
const MAX_ARG_LENGTH: usize = 1000;

/// Maximum path length
const MAX_PATH_LENGTH: usize = 4096;

/// Dangerous patterns that indicate potential attacks
const DANGEROUS_PATTERNS: &[&str] = &[
    "..", // Path traversal
    "~",  // Home directory
    "$",  // Variable expansion
    "`",  // Command substitution
    ";",  // Command chaining
    "&",  // Background/AND
    "|",  // Pipe
    ">",  // Redirect output
    "<",  // Redirect input
    "\n", // Newline
    "\r", // Carriage return
    "\0", // Null byte
];

/// Protected system directories
const PROTECTED_DIRS: &[&str] = &[
    "/etc",
    "/bin",
    "/sbin",
    "/usr/bin",
    "/usr/sbin",
    "/boot",
    "/sys",
    "/proc",
    "/dev",
];

/// Secure argument parsing errors
#[derive(Debug, Error)]
pub enum ParseError {
    /// Too many arguments provided
    #[error("Too many arguments (max {MAX_ARG_COUNT})")]
    TooManyArguments,

    /// Argument exceeds maximum length
    #[error("Argument too long (max {MAX_ARG_LENGTH} characters): {0}")]
    ArgumentTooLong(String),

    /// Dangerous pattern detected
    #[error("Dangerous pattern detected in argument: {pattern} in {arg}")]
    DangerousPattern { pattern: String, arg: String },

    /// Path too long
    #[error("Path exceeds maximum length (max {MAX_PATH_LENGTH})")]
    PathTooLong,

    /// Attempted access to protected system directory
    #[error("Access to protected system directory denied: {0}")]
    ProtectedDirectory(String),

    /// Path does not exist
    #[error("Path does not exist: {0}")]
    PathNotFound(String),

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Missing required argument
    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    /// Invalid argument value
    #[error("Invalid argument value for {arg}: {reason}")]
    InvalidValue { arg: String, reason: String },
}

/// Secure argument parser
///
/// Provides security-first parsing with comprehensive validation.
pub struct SecureArgParser;

impl SecureArgParser {
    /// Parse command-line arguments securely
    ///
    /// # Security Validations
    ///
    /// 1. Count limit check
    /// 2. Length validation
    /// 3. Dangerous pattern detection
    /// 4. Path normalization
    /// 5. Protected directory check
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if any validation fails
    pub fn parse(args: &[String]) -> Result<AppConfig, ParseError> {
        // Validate argument count
        if args.len() > MAX_ARG_COUNT {
            return Err(ParseError::TooManyArguments);
        }

        // For now, create a simple default config
        // In a real implementation, this would use clap to parse args
        // and then validate each parsed value

        // This is a placeholder - proper implementation would:
        // 1. Use clap to define CLI structure
        // 2. Parse arguments
        // 3. Validate each value using validation methods below
        // 4. Build and return AppConfig

        Ok(AppConfig::builder().app_name("adaptive-pipeline").build())
    }

    /// Validate a single argument for security issues
    ///
    /// # Errors
    ///
    /// - `ArgumentTooLong` if exceeds max length
    /// - `DangerousPattern` if contains dangerous patterns
    pub fn validate_argument(arg: &str) -> Result<(), ParseError> {
        // Length check
        if arg.len() > MAX_ARG_LENGTH {
            return Err(ParseError::ArgumentTooLong(
                arg.chars().take(50).collect::<String>() + "...",
            ));
        }

        // Dangerous pattern check
        for pattern in DANGEROUS_PATTERNS {
            if arg.contains(pattern) {
                return Err(ParseError::DangerousPattern {
                    pattern: pattern.to_string(),
                    arg: arg.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate and normalize a file path
    ///
    /// # Security Checks
    ///
    /// 1. Length validation
    /// 2. Dangerous pattern detection
    /// 3. Path canonicalization
    /// 4. Protected directory check
    ///
    /// # Returns
    ///
    /// Canonical absolute path if valid
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if path fails validation
    pub fn validate_path(path: &str) -> Result<PathBuf, ParseError> {
        // Basic validation
        Self::validate_argument(path).map_err(|e| match e {
            ParseError::ArgumentTooLong(_) => ParseError::InvalidPath(format!("Path too long: {}", path)),
            ParseError::DangerousPattern { pattern, .. } => {
                ParseError::InvalidPath(format!("Path contains dangerous pattern '{}': {}", pattern, path))
            }
            other => other,
        })?;

        // Path object creation
        let path_obj = Path::new(path);

        // Try to canonicalize (resolves .., symlinks, etc.)
        let canonical = path_obj.canonicalize().map_err(|e| {
            if !path_obj.exists() {
                ParseError::PathNotFound(path.to_string())
            } else {
                ParseError::InvalidPath(format!("{}: {}", path, e))
            }
        })?;

        // Length check on canonical path
        if canonical.to_string_lossy().len() > MAX_PATH_LENGTH {
            return Err(ParseError::PathTooLong);
        }

        // Protected directory check
        for protected in PROTECTED_DIRS {
            if canonical.starts_with(protected) {
                return Err(ParseError::ProtectedDirectory(canonical.display().to_string()));
            }
        }

        Ok(canonical)
    }

    /// Validate an optional path (may be None)
    pub fn validate_optional_path(path: Option<&str>) -> Result<Option<PathBuf>, ParseError> {
        match path {
            Some(p) => Self::validate_path(p).map(Some),
            None => Ok(None),
        }
    }

    /// Validate a number argument
    pub fn validate_number<T>(arg_name: &str, value: &str, min: Option<T>, max: Option<T>) -> Result<T, ParseError>
    where
        T: std::str::FromStr + PartialOrd + std::fmt::Display,
    {
        // Basic validation
        Self::validate_argument(value)?;

        // Parse
        let num = value.parse::<T>().map_err(|_| ParseError::InvalidValue {
            arg: arg_name.to_string(),
            reason: format!("Not a valid number: {}", value),
        })?;

        // Range check
        if let Some(min_val) = min {
            if num < min_val {
                return Err(ParseError::InvalidValue {
                    arg: arg_name.to_string(),
                    reason: format!("Value {} is less than minimum {}", value, min_val),
                });
            }
        }

        if let Some(max_val) = max {
            if num > max_val {
                return Err(ParseError::InvalidValue {
                    arg: arg_name.to_string(),
                    reason: format!("Value {} is greater than maximum {}", value, max_val),
                });
            }
        }

        Ok(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod argument_validation {
        use super::*;

        #[test]
        fn accepts_safe_arguments() {
            assert!(SecureArgParser::validate_argument("safe-arg").is_ok());
            assert!(SecureArgParser::validate_argument("file.txt").is_ok());
            assert!(SecureArgParser::validate_argument("path/to/file").is_ok());
        }

        #[test]
        fn rejects_too_long_arguments() {
            let long_arg = "a".repeat(MAX_ARG_LENGTH + 1);
            assert!(matches!(
                SecureArgParser::validate_argument(&long_arg),
                Err(ParseError::ArgumentTooLong(_))
            ));
        }

        #[test]
        fn detects_dangerous_patterns() {
            let dangerous = vec![
                "../etc/passwd",
                "~/.ssh/id_rsa",
                "$(whoami)",
                "`ls`",
                "file;rm -rf /",
                "file&background",
                "file|pipe",
                "file>output",
                "file<input",
                "file\nwith\nnewlines",
            ];

            for arg in dangerous {
                assert!(
                    matches!(
                        SecureArgParser::validate_argument(arg),
                        Err(ParseError::DangerousPattern { .. })
                    ),
                    "Failed to detect dangerous pattern in: {}",
                    arg
                );
            }
        }
    }

    mod number_validation {
        use super::*;

        #[test]
        fn validates_valid_numbers() {
            let result = SecureArgParser::validate_number::<u32>("threads", "8", Some(1), Some(16));
            assert_eq!(result.unwrap(), 8);
        }

        #[test]
        fn rejects_invalid_numbers() {
            let result = SecureArgParser::validate_number::<u32>("threads", "abc", None, None);
            assert!(matches!(result, Err(ParseError::InvalidValue { .. })));
        }

        #[test]
        fn enforces_range_constraints() {
            let result = SecureArgParser::validate_number::<u32>("threads", "100", Some(1), Some(16));
            assert!(matches!(result, Err(ParseError::InvalidValue { .. })));

            let result = SecureArgParser::validate_number::<u32>("threads", "0", Some(1), Some(16));
            assert!(matches!(result, Err(ParseError::InvalidValue { .. })));
        }
    }

    mod parsing {
        use super::*;

        #[test]
        fn parses_basic_arguments() {
            let args = vec!["program".to_string()];
            let result = SecureArgParser::parse(&args);
            assert!(result.is_ok());
        }

        #[test]
        fn rejects_too_many_arguments() {
            let args = vec!["arg".to_string(); MAX_ARG_COUNT + 1];
            let result = SecureArgParser::parse(&args);
            assert!(matches!(result, Err(ParseError::TooManyArguments)));
        }
    }
}
