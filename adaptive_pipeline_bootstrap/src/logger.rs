// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Bootstrap Logger
//!
//! Lightweight logging abstraction for the bootstrap phase.
//!
//! ## Design Rationale
//!
//! The bootstrap logger is a **simplified logging interface** specifically for
//! bootstrap-phase operations. It provides:
//!
//! - **Minimal API** - Only essential log levels
//! - **Trait-based** - Testable with no-op implementation
//! - **Integration-ready** - Can wrap tracing, env_logger, or custom loggers
//! - **Bootstrap-specific** - Separate from application logging
//!
//! ## Log Levels
//!
//! - **Error** - Fatal errors during bootstrap
//! - **Warn** - Non-fatal issues (missing optional config, etc.)
//! - **Info** - Normal bootstrap messages
//! - **Debug** - Detailed bootstrap information
//!
//! ## Usage
//!
//! ```rust
//! use bootstrap::logger::{BootstrapLogger, ConsoleLogger};
//!
//! let logger = ConsoleLogger::new();
//! logger.info("Starting application bootstrap");
//! logger.debug("Parsing command line arguments");
//! ```

#[cfg(test)]
use std::fmt;

/// Bootstrap logging abstraction
///
/// Provides a simple logging interface for bootstrap operations.
/// Implementations can use tracing, env_logger, or custom backends.
pub trait BootstrapLogger: Send + Sync {
    /// Log an error message
    ///
    /// Used for fatal errors during bootstrap that will cause termination.
    fn error(&self, message: &str);

    /// Log a warning message
    ///
    /// Used for non-fatal issues that may affect operation.
    fn warn(&self, message: &str);

    /// Log an info message
    ///
    /// Used for normal bootstrap progress messages.
    fn info(&self, message: &str);

    /// Log a debug message
    ///
    /// Used for detailed diagnostic information during bootstrap.
    fn debug(&self, message: &str);
}

/// Console logger implementation using tracing
///
/// Routes bootstrap logs through the tracing crate for consistent logging.
pub struct ConsoleLogger {
    prefix: String,
}

impl ConsoleLogger {
    /// Create a new console logger with default prefix
    pub fn new() -> Self {
        Self::with_prefix("bootstrap")
    }

    /// Create a new console logger with custom prefix
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into() }
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl BootstrapLogger for ConsoleLogger {
    fn error(&self, message: &str) {
        tracing::error!(target: "bootstrap", "[{}] {}", self.prefix, message);
    }

    fn warn(&self, message: &str) {
        tracing::warn!(target: "bootstrap", "[{}] {}", self.prefix, message);
    }

    fn info(&self, message: &str) {
        tracing::info!(target: "bootstrap", "[{}] {}", self.prefix, message);
    }

    fn debug(&self, message: &str) {
        tracing::debug!(target: "bootstrap", "[{}] {}", self.prefix, message);
    }
}

/// No-op logger for testing
///
/// Discards all log messages. Useful for testing bootstrap logic
/// without generating log output.
pub struct NoOpLogger;

impl NoOpLogger {
    /// Create a new no-op logger
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoOpLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl BootstrapLogger for NoOpLogger {
    fn error(&self, _message: &str) {}
    fn warn(&self, _message: &str) {}
    fn info(&self, _message: &str) {}
    fn debug(&self, _message: &str) {}
}

/// Capturing logger for testing
///
/// Captures log messages in memory for assertion in tests.
#[cfg(test)]
pub struct CapturingLogger {
    messages: std::sync::Arc<std::sync::Mutex<Vec<LogMessage>>>,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

#[cfg(test)]
impl CapturingLogger {
    pub fn new() -> Self {
        Self {
            messages: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn messages(&self) -> Vec<LogMessage> {
        self.messages.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.messages.lock().unwrap().clear();
    }

    fn log(&self, level: LogLevel, message: &str) {
        self.messages.lock().unwrap().push(LogMessage {
            level,
            message: message.to_string(),
        });
    }
}

#[cfg(test)]
impl BootstrapLogger for CapturingLogger {
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
}

#[cfg(test)]
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_logger_creation() {
        let logger = ConsoleLogger::new();
        // Just verify it doesn't panic
        logger.info("test message");
    }

    #[test]
    fn test_console_logger_with_prefix() {
        let logger = ConsoleLogger::with_prefix("custom");
        // Just verify it doesn't panic
        logger.debug("test message");
    }

    #[test]
    fn test_noop_logger() {
        let logger = NoOpLogger::new();
        // Should not panic or produce output
        logger.error("error");
        logger.warn("warning");
        logger.info("info");
        logger.debug("debug");
    }

    #[test]
    fn test_capturing_logger() {
        let logger = CapturingLogger::new();

        logger.error("error message");
        logger.warn("warning message");
        logger.info("info message");
        logger.debug("debug message");

        let messages = logger.messages();
        assert_eq!(messages.len(), 4);

        assert_eq!(messages[0].level, LogLevel::Error);
        assert_eq!(messages[0].message, "error message");

        assert_eq!(messages[1].level, LogLevel::Warn);
        assert_eq!(messages[1].message, "warning message");

        assert_eq!(messages[2].level, LogLevel::Info);
        assert_eq!(messages[2].message, "info message");

        assert_eq!(messages[3].level, LogLevel::Debug);
        assert_eq!(messages[3].message, "debug message");
    }

    #[test]
    fn test_capturing_logger_clear() {
        let logger = CapturingLogger::new();

        logger.info("message 1");
        logger.info("message 2");
        assert_eq!(logger.messages().len(), 2);

        logger.clear();
        assert_eq!(logger.messages().len(), 0);

        logger.info("message 3");
        assert_eq!(logger.messages().len(), 1);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
        assert_eq!(format!("{}", LogLevel::Warn), "WARN");
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
    }
}
