//! # Application Configuration
//!
//! Bootstrap-phase configuration structure.
//!
//! ## Design Philosophy
//!
//! `AppConfig` holds **validated** configuration after:
//! 1. Command-line argument parsing
//! 2. Security validation
//! 3. Environment variable resolution
//! 4. Default value application
//!
//! ## Immutability
//!
//! All configuration is **immutable** after creation. This ensures:
//! - Thread safety (no synchronization needed)
//! - Predictable behavior
//! - Safe sharing across async tasks
//!
//! ## Usage
//!
//! ```rust
//! use bootstrap::config::{AppConfig, LogLevel};
//! use std::path::PathBuf;
//!
//! let config = AppConfig::builder()
//!     .app_name("my-app")
//!     .log_level(LogLevel::Info)
//!     .input_path(PathBuf::from("/path/to/input"))
//!     .build();
//!
//! println!("Running: {}", config.app_name());
//! ```

use std::path::PathBuf;

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Error messages only
    Error,
    /// Warnings and errors
    Warn,
    /// Info, warnings, and errors (default)
    Info,
    /// All messages including debug
    Debug,
    /// All messages including trace
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl LogLevel {
    /// Convert to tracing Level
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

/// Application configuration
///
/// Immutable configuration structure holding all bootstrap-phase settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Application name
    app_name: String,

    /// Log level
    log_level: LogLevel,

    /// Input file or directory path
    input_path: Option<PathBuf>,

    /// Output file or directory path
    output_path: Option<PathBuf>,

    /// Number of worker threads (None = automatic)
    worker_threads: Option<usize>,

    /// Enable verbose output
    verbose: bool,

    /// Dry run mode (no actual changes)
    dry_run: bool,
}

impl AppConfig {
    /// Create a new configuration builder
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder::default()
    }

    /// Get application name
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Get log level
    pub fn log_level(&self) -> LogLevel {
        self.log_level
    }

    /// Get input path
    pub fn input_path(&self) -> Option<&PathBuf> {
        self.input_path.as_ref()
    }

    /// Get output path
    pub fn output_path(&self) -> Option<&PathBuf> {
        self.output_path.as_ref()
    }

    /// Get worker thread count
    pub fn worker_threads(&self) -> Option<usize> {
        self.worker_threads
    }

    /// Check if verbose mode is enabled
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// Check if dry run mode is enabled
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

/// Builder for AppConfig
#[derive(Debug, Default)]
pub struct AppConfigBuilder {
    app_name: Option<String>,
    log_level: Option<LogLevel>,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    worker_threads: Option<usize>,
    verbose: bool,
    dry_run: bool,
}

impl AppConfigBuilder {
    /// Set application name
    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.app_name = Some(name.into());
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Set input path
    pub fn input_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.input_path = Some(path.into());
        self
    }

    /// Set output path
    pub fn output_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.output_path = Some(path.into());
        self
    }

    /// Set worker thread count
    pub fn worker_threads(mut self, count: usize) -> Self {
        self.worker_threads = Some(count);
        self
    }

    /// Enable verbose mode
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.verbose = enabled;
        self
    }

    /// Enable dry run mode
    pub fn dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    /// Build the configuration
    ///
    /// # Panics
    ///
    /// Panics if app_name was not set
    pub fn build(self) -> AppConfig {
        AppConfig {
            app_name: self.app_name.expect("app_name is required"),
            log_level: self.log_level.unwrap_or_default(),
            input_path: self.input_path,
            output_path: self.output_path,
            worker_threads: self.worker_threads,
            verbose: self.verbose,
            dry_run: self.dry_run,
        }
    }

    /// Try to build the configuration
    ///
    /// Returns Err if required fields are missing
    pub fn try_build(self) -> Result<AppConfig, String> {
        Ok(AppConfig {
            app_name: self.app_name.ok_or("app_name is required")?,
            log_level: self.log_level.unwrap_or_default(),
            input_path: self.input_path,
            output_path: self.output_path,
            worker_threads: self.worker_threads,
            verbose: self.verbose,
            dry_run: self.dry_run,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_minimal() {
        let config = AppConfig::builder().app_name("test-app").build();

        assert_eq!(config.app_name(), "test-app");
        assert_eq!(config.log_level(), LogLevel::Info); // default
        assert!(config.input_path().is_none());
        assert!(config.output_path().is_none());
        assert!(config.worker_threads().is_none());
        assert!(!config.is_verbose());
        assert!(!config.is_dry_run());
    }

    #[test]
    fn test_config_builder_full() {
        let config = AppConfig::builder()
            .app_name("full-app")
            .log_level(LogLevel::Debug)
            .input_path("/input")
            .output_path("/output")
            .worker_threads(8)
            .verbose(true)
            .dry_run(true)
            .build();

        assert_eq!(config.app_name(), "full-app");
        assert_eq!(config.log_level(), LogLevel::Debug);
        assert_eq!(
            config.input_path(),
            Some(&PathBuf::from("/input"))
        );
        assert_eq!(
            config.output_path(),
            Some(&PathBuf::from("/output"))
        );
        assert_eq!(config.worker_threads(), Some(8));
        assert!(config.is_verbose());
        assert!(config.is_dry_run());
    }

    #[test]
    #[should_panic(expected = "app_name is required")]
    fn test_config_builder_missing_app_name() {
        AppConfig::builder().build();
    }

    #[test]
    fn test_config_try_build_success() {
        let result = AppConfig::builder()
            .app_name("test")
            .try_build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_config_try_build_failure() {
        let result = AppConfig::builder().try_build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "app_name is required");
    }

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn test_log_level_to_tracing() {
        assert_eq!(
            LogLevel::Error.to_tracing_level(),
            tracing::Level::ERROR
        );
        assert_eq!(
            LogLevel::Warn.to_tracing_level(),
            tracing::Level::WARN
        );
        assert_eq!(
            LogLevel::Info.to_tracing_level(),
            tracing::Level::INFO
        );
        assert_eq!(
            LogLevel::Debug.to_tracing_level(),
            tracing::Level::DEBUG
        );
        assert_eq!(
            LogLevel::Trace.to_tracing_level(),
            tracing::Level::TRACE
        );
    }

    #[test]
    fn test_config_clone() {
        let config1 = AppConfig::builder()
            .app_name("clone-test")
            .log_level(LogLevel::Debug)
            .build();

        let config2 = config1.clone();

        assert_eq!(config1.app_name(), config2.app_name());
        assert_eq!(config1.log_level(), config2.log_level());
    }
}
