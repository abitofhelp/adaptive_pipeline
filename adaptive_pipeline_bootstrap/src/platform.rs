// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Platform Abstraction Module
//!
//! This module provides platform-specific abstractions for operating system
//! functionality, following the pattern used in the Ada project.
//!
//! ## Architecture Pattern
//!
//! Following hexagonal architecture principles:
//! - **Interface**: `Platform` trait defines the contract
//! - **Implementations**:
//!   - `UnixPlatform`: POSIX implementation (Linux + macOS)
//!   - `WindowsPlatform`: Windows API implementation
//! - **Selection**: Compile-time platform selection via `#[cfg]`
//!
//! ## Design Philosophy
//!
//! The bootstrap module sits OUTSIDE the enterprise application layers,
//! so it can access platform-specific APIs directly. This abstraction:
//!
//! 1. **Isolates** OS-specific code to one module
//! 2. **Enables** testing via trait mocking
//! 3. **Provides** consistent API across platforms
//! 4. **Avoids** scattered conditional compilation
//!
//! ## Usage
//!
//! ```rust
//! use bootstrap::platform::create_platform;
//!
//! let platform = create_platform();
//! println!("Running on: {}", platform.platform_name());
//! println!("CPU cores: {}", platform.cpu_count());
//! ```

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

// Re-export implementations
#[cfg(unix)]
pub use unix::UnixPlatform;

#[cfg(windows)]
pub use windows::WindowsPlatform;

/// Platform-specific errors
#[derive(Debug, Error)]
pub enum PlatformError {
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Feature not supported on this platform
    #[error("Not supported on this platform: {0}")]
    NotSupported(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Generic platform error
    #[error("Platform error: {0}")]
    Other(String),
}

/// Platform abstraction trait for OS-specific operations
///
/// This trait provides a clean interface for platform-specific functionality,
/// allowing the bootstrap layer to work with different operating systems
/// without conditional compilation throughout the codebase.
///
/// ## Design Principles
///
/// - **Stateless**: All methods are stateless and thread-safe
/// - **Async-aware**: File operations are async-compatible
/// - **Error-handling**: All fallible operations return `Result`
/// - **Cross-platform**: Same interface works on Unix and Windows
///
/// ## Implementation Notes
///
/// Implementations should use native platform APIs:
/// - Unix: POSIX APIs via `libc`, `/proc`, `/sys`
/// - Windows: Windows API via `winapi`
/// - Fallbacks: Standard Rust APIs when platform APIs unavailable
#[async_trait]
pub trait Platform: Send + Sync {
    // === System Information ===

    /// Get the system page size for memory alignment
    ///
    /// Used for:
    /// - Memory-mapped I/O alignment
    /// - Buffer sizing optimizations
    /// - Cache-friendly allocations
    ///
    /// # Returns
    /// Page size in bytes (typically 4096 on most systems)
    fn page_size(&self) -> usize;

    /// Get the number of available CPU cores
    ///
    /// Returns the number of logical processors available to the process.
    /// Used for determining optimal parallelism levels.
    ///
    /// # Returns
    /// Number of CPU cores (at least 1)
    fn cpu_count(&self) -> usize;

    /// Get total system memory in bytes
    ///
    /// # Returns
    /// Total physical memory in bytes
    ///
    /// # Errors
    /// Returns error if system information cannot be retrieved
    fn total_memory(&self) -> Result<u64, PlatformError>;

    /// Get available system memory in bytes
    ///
    /// # Returns
    /// Available (free) memory in bytes
    ///
    /// # Errors
    /// Returns error if system information cannot be retrieved
    fn available_memory(&self) -> Result<u64, PlatformError>;

    // === Platform Constants ===

    /// Get the platform-specific line separator
    ///
    /// # Returns
    /// - Unix: `"\n"`
    /// - Windows: `"\r\n"`
    fn line_separator(&self) -> &'static str;

    /// Get the platform-specific path separator for PATH environment variable
    ///
    /// # Returns
    /// - Unix: `':'`
    /// - Windows: `';'`
    fn path_separator(&self) -> char;

    /// Get the platform name
    ///
    /// # Returns
    /// Platform identifier: "linux", "macos", "windows", etc.
    fn platform_name(&self) -> &'static str;

    /// Get the platform-specific temporary directory
    ///
    /// # Returns
    /// Path to system temp directory
    fn temp_dir(&self) -> PathBuf;

    // === Security & Permissions ===

    /// Check if running with elevated privileges
    ///
    /// # Returns
    /// - Unix: `true` if effective UID is 0 (root)
    /// - Windows: `true` if running as Administrator
    fn is_elevated(&self) -> bool;

    /// Set file permissions (Unix-specific, no-op on Windows)
    ///
    /// # Arguments
    /// - `path`: Path to file
    /// - `mode`: Unix permission bits (e.g., 0o644)
    ///
    /// # Errors
    /// Returns error if permissions cannot be set
    fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), PlatformError>;

    /// Check if a path points to an executable file
    ///
    /// # Arguments
    /// - `path`: Path to check
    ///
    /// # Returns
    /// - Unix: `true` if execute bit set
    /// - Windows: `true` if extension is .exe, .bat, .cmd, .com
    fn is_executable(&self, path: &Path) -> bool;

    // === File Operations ===

    /// Flush file buffers to disk
    ///
    /// Ensures all buffered data is written to physical storage.
    ///
    /// # Arguments
    /// - `file`: File to sync
    ///
    /// # Errors
    /// Returns error if sync operation fails
    async fn sync_file(&self, file: &tokio::fs::File) -> Result<(), PlatformError>;
}

// === Platform Selection ===

#[cfg(unix)]
type PlatformImpl = UnixPlatform;

#[cfg(windows)]
type PlatformImpl = WindowsPlatform;

/// Create the platform-specific implementation
///
/// This function returns the appropriate platform implementation
/// for the current operating system, selected at compile time.
///
/// # Returns
/// Boxed platform implementation
///
/// # Examples
///
/// ```rust
/// use bootstrap::platform::create_platform;
///
/// let platform = create_platform();
/// println!("Running on: {}", platform.platform_name());
/// ```
pub fn create_platform() -> Box<dyn Platform> {
    Box::new(PlatformImpl::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_platform() {
        let platform = create_platform();

        // Should have at least one CPU
        assert!(platform.cpu_count() >= 1);

        // Page size should be reasonable
        let page_size = platform.page_size();
        assert!(page_size >= 512);
        assert!(page_size <= 65536);

        // Platform name should not be empty
        assert!(!platform.platform_name().is_empty());
    }

    #[test]
    fn test_line_separator() {
        let platform = create_platform();
        let sep = platform.line_separator();

        #[cfg(unix)]
        assert_eq!(sep, "\n");

        #[cfg(windows)]
        assert_eq!(sep, "\r\n");
    }

    #[test]
    fn test_path_separator() {
        let platform = create_platform();
        let sep = platform.path_separator();

        #[cfg(unix)]
        assert_eq!(sep, ':');

        #[cfg(windows)]
        assert_eq!(sep, ';');
    }
}
