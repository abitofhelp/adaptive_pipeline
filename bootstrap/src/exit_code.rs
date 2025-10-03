// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Exit Code Management
//!
//! Provides standardized Unix exit codes following BSD `sysexits.h` conventions.
//!
//! ## Exit Code Conventions
//!
//! - **0**: Success
//! - **1**: General error
//! - **2**: Misuse of shell command (reserved by Bash)
//! - **64-78**: Specific error conditions (BSD sysexits.h)
//! - **126**: Command cannot execute
//! - **127**: Command not found
//! - **128+N**: Fatal signal N (e.g., 130 = SIGINT)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bootstrap::exit_code::ExitCode;
//!
//! fn run_application() -> Result<(), Box<dyn std::error::Error>> {
//!     // Application logic here
//!     Ok(())
//! }
//!
//! fn main() {
//!     let result = run_application();
//!     let exit_code = match result {
//!         Ok(_) => ExitCode::Success,
//!         Err(e) => ExitCode::from_error(e.as_ref()),
//!     };
//!     std::process::exit(exit_code.as_i32());
//! }
//! ```

use std::fmt;

/// Exit codes following Unix conventions (BSD sysexits.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ExitCode {
    /// Successful termination (0)
    #[default]
    Success = 0,

    /// General error (1)
    Error = 1,

    /// Command line usage error (64)
    /// - Invalid arguments
    /// - Missing required arguments
    /// - Unknown flags
    UsageError = 64,

    /// Data format error (65)
    /// - Invalid input data
    /// - Malformed configuration
    /// - Parse errors
    DataError = 65,

    /// Cannot open input (66)
    /// - File not found
    /// - Cannot read file
    /// - Permission denied on input
    NoInput = 66,

    /// User does not exist (67)
    /// - Unknown user specified
    /// - Invalid user context
    NoUser = 67,

    /// Host name unknown (68)
    /// - Unknown host
    /// - Cannot resolve hostname
    NoHost = 68,

    /// Service unavailable (69)
    /// - Required service not running
    /// - Dependency not available
    /// - External service unreachable
    Unavailable = 69,

    /// Internal software error (70)
    /// - Unexpected error
    /// - Assertion failure
    /// - Internal consistency check failed
    Software = 70,

    /// System error (71)
    /// - OS error
    /// - System call failed
    /// - Fork failed
    OsError = 71,

    /// Critical OS file missing (72)
    /// - Required system file not found
    /// - Missing configuration file
    OsFile = 72,

    /// Cannot create output file (73)
    /// - Cannot write output
    /// - Disk full
    /// - Permission denied on output
    CantCreate = 73,

    /// I/O error (74)
    /// - Read error
    /// - Write error
    /// - Network I/O error
    IoError = 74,

    /// Temporary failure, retry (75)
    /// - Resource temporarily unavailable
    /// - Retry operation
    TempFail = 75,

    /// Remote error in protocol (76)
    /// - Protocol violation
    /// - Invalid response
    /// - Communication error
    Protocol = 76,

    /// Permission denied (77)
    /// - Insufficient privileges
    /// - Access denied
    /// - Not authorized
    NoPerm = 77,

    /// Configuration error (78)
    /// - Invalid configuration
    /// - Missing required configuration
    /// - Configuration validation failed
    Config = 78,

    /// Interrupted by signal (SIGINT - Ctrl+C) (130)
    /// - User interrupted (Ctrl+C)
    /// - SIGINT received
    Interrupted = 130,

    /// Terminated by signal (SIGTERM) (143)
    /// - SIGTERM received
    /// - Graceful shutdown requested
    Terminated = 143,
}

impl ExitCode {
    /// Convert to i32 for use with std::process::exit
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Create ExitCode from error type
    ///
    /// Maps common error types to appropriate exit codes:
    /// - I/O errors → IoError (74)
    /// - Parse errors → DataError (65)
    /// - Permission errors → NoPerm (77)
    /// - Not found errors → NoInput (66)
    /// - Invalid argument → UsageError (64)
    /// - Other errors → Error (1)
    pub fn from_error(error: &dyn std::error::Error) -> Self {
        let error_string = error.to_string().to_lowercase();

        // Check for specific error patterns
        if error_string.contains("permission") || error_string.contains("access denied") {
            ExitCode::NoPerm
        } else if error_string.contains("not found") || error_string.contains("no such") {
            ExitCode::NoInput
        } else if error_string.contains("invalid") || error_string.contains("argument") {
            ExitCode::UsageError
        } else if error_string.contains("parse") || error_string.contains("format") {
            ExitCode::DataError
        } else if error_string.contains("io") || error_string.contains("read") || error_string.contains("write") {
            ExitCode::IoError
        } else if error_string.contains("config") {
            ExitCode::Config
        } else if error_string.contains("unavailable") || error_string.contains("not available") {
            ExitCode::Unavailable
        } else {
            ExitCode::Error
        }
    }

    /// Get human-readable description of exit code
    pub fn description(self) -> &'static str {
        match self {
            ExitCode::Success => "Success",
            ExitCode::Error => "General error",
            ExitCode::UsageError => "Command line usage error",
            ExitCode::DataError => "Data format error",
            ExitCode::NoInput => "Cannot open input",
            ExitCode::NoUser => "User does not exist",
            ExitCode::NoHost => "Host name unknown",
            ExitCode::Unavailable => "Service unavailable",
            ExitCode::Software => "Internal software error",
            ExitCode::OsError => "System error",
            ExitCode::OsFile => "Critical OS file missing",
            ExitCode::CantCreate => "Cannot create output file",
            ExitCode::IoError => "I/O error",
            ExitCode::TempFail => "Temporary failure, retry",
            ExitCode::Protocol => "Remote error in protocol",
            ExitCode::NoPerm => "Permission denied",
            ExitCode::Config => "Configuration error",
            ExitCode::Interrupted => "Interrupted by signal (SIGINT)",
            ExitCode::Terminated => "Terminated by signal (SIGTERM)",
        }
    }

    /// Check if this is a success exit code
    pub fn is_success(self) -> bool {
        matches!(self, ExitCode::Success)
    }

    /// Check if this is an error exit code
    pub fn is_error(self) -> bool {
        !self.is_success()
    }

    /// Check if this represents a signal interruption
    pub fn is_signal(self) -> bool {
        matches!(self, ExitCode::Interrupted | ExitCode::Terminated)
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.description(), self.as_i32())
    }
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> i32 {
        code.as_i32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_code_values() {
        assert_eq!(ExitCode::Success.as_i32(), 0);
        assert_eq!(ExitCode::Error.as_i32(), 1);
        assert_eq!(ExitCode::UsageError.as_i32(), 64);
        assert_eq!(ExitCode::Config.as_i32(), 78);
        assert_eq!(ExitCode::Interrupted.as_i32(), 130);
        assert_eq!(ExitCode::Terminated.as_i32(), 143);
    }

    #[test]
    fn test_is_success() {
        assert!(ExitCode::Success.is_success());
        assert!(!ExitCode::Error.is_success());
        assert!(!ExitCode::UsageError.is_success());
    }

    #[test]
    fn test_is_error() {
        assert!(!ExitCode::Success.is_error());
        assert!(ExitCode::Error.is_error());
        assert!(ExitCode::Config.is_error());
    }

    #[test]
    fn test_is_signal() {
        assert!(ExitCode::Interrupted.is_signal());
        assert!(ExitCode::Terminated.is_signal());
        assert!(!ExitCode::Success.is_signal());
        assert!(!ExitCode::Error.is_signal());
    }

    #[test]
    fn test_default() {
        assert_eq!(ExitCode::default(), ExitCode::Success);
    }

    #[test]
    fn test_display() {
        let code = ExitCode::UsageError;
        let display = format!("{}", code);
        assert!(display.contains("Command line usage error"));
        assert!(display.contains("64"));
    }

    #[test]
    fn test_from_error() {
        use std::io;

        // Permission error
        let err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        assert_eq!(ExitCode::from_error(&err), ExitCode::NoPerm);

        // Not found error
        let err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        assert_eq!(ExitCode::from_error(&err), ExitCode::NoInput);
    }

    #[test]
    fn test_conversion_to_i32() {
        let code: i32 = ExitCode::Config.into();
        assert_eq!(code, 78);
    }
}
