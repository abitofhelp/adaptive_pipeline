// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Windows Platform Implementation
//!
//! Windows API implementation with cross-platform stubs.
//!
//! ## Implementation Notes
//!
//! - **On Windows**: Uses winapi crate for native Windows API calls
//! - **On Unix**: Provides stub implementations for cross-compilation
//!
//! ## Windows APIs Used (when on Windows)
//!
//! - `GlobalMemoryStatusEx` - Memory information
//! - `GetSystemInfo` - CPU count and page size
//! - `IsUserAnAdmin` - Privilege checking
//! - File APIs via tokio (cross-platform)

use super::{Platform, PlatformError};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// Windows platform implementation
///
/// Provides Windows-specific implementations on Windows,
/// and stub implementations on Unix for cross-compilation.
pub struct WindowsPlatform;

impl WindowsPlatform {
    /// Create a new Windows platform instance
    pub fn new() -> Self {
        Self
    }

    #[cfg(windows)]
    fn get_memory_info_impl() -> Result<(u64, u64), PlatformError> {
        use std::mem;
        use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

        unsafe {
            let mut mem_status: MEMORYSTATUSEX = mem::zeroed();
            mem_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;

            if GlobalMemoryStatusEx(&mut mem_status) != 0 {
                Ok((mem_status.ullTotalPhys, mem_status.ullAvailPhys))
            } else {
                Err(PlatformError::Other("GlobalMemoryStatusEx failed".to_string()))
            }
        }
    }

    #[cfg(not(windows))]
    fn get_memory_info_impl() -> Result<(u64, u64), PlatformError> {
        // Stub for cross-compilation
        Err(PlatformError::NotSupported(
            "Windows APIs not available on this platform".to_string(),
        ))
    }

    #[cfg(windows)]
    fn get_page_size_impl() -> usize {
        use std::mem;
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};

        unsafe {
            let mut sys_info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut sys_info);
            sys_info.dwPageSize as usize
        }
    }

    #[cfg(not(windows))]
    fn get_page_size_impl() -> usize {
        // Stub returns default page size
        4096
    }

    #[cfg(windows)]
    fn get_cpu_count_impl() -> usize {
        use std::mem;
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};

        unsafe {
            let mut sys_info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut sys_info);
            sys_info.dwNumberOfProcessors as usize
        }
    }

    #[cfg(not(windows))]
    fn get_cpu_count_impl() -> usize {
        // Stub returns 1
        1
    }

    #[cfg(windows)]
    fn is_elevated_impl() -> bool {
        // Manual FFI declaration since winapi doesn't properly expose IsUserAnAdmin
        #[link(name = "shell32")]
        extern "system" {
            fn IsUserAnAdmin() -> i32;
        }
        unsafe { IsUserAnAdmin() != 0 }
    }

    #[cfg(not(windows))]
    fn is_elevated_impl() -> bool {
        // Stub returns false
        false
    }
}

impl Default for WindowsPlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Platform for WindowsPlatform {
    fn page_size(&self) -> usize {
        Self::get_page_size_impl()
    }

    fn cpu_count(&self) -> usize {
        Self::get_cpu_count_impl()
    }

    fn total_memory(&self) -> Result<u64, PlatformError> {
        Self::get_memory_info_impl().map(|(total, _)| total)
    }

    fn available_memory(&self) -> Result<u64, PlatformError> {
        Self::get_memory_info_impl().map(|(_, available)| available)
    }

    fn line_separator(&self) -> &'static str {
        "\r\n"
    }

    fn path_separator(&self) -> char {
        ';'
    }

    fn platform_name(&self) -> &'static str {
        "windows"
    }

    fn temp_dir(&self) -> PathBuf {
        std::env::temp_dir()
    }

    fn is_elevated(&self) -> bool {
        Self::is_elevated_impl()
    }

    fn set_permissions(&self, _path: &Path, _mode: u32) -> Result<(), PlatformError> {
        // Windows doesn't use Unix-style permission bits
        // This is a no-op on Windows, returns Ok
        Ok(())
    }

    fn is_executable(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            matches!(ext_lower.as_str(), "exe" | "bat" | "cmd" | "com" | "ps1" | "msi")
        } else {
            false
        }
    }

    async fn sync_file(&self, file: &tokio::fs::File) -> Result<(), PlatformError> {
        // tokio's sync_all is cross-platform
        file.sync_all().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_platform_basics() {
        let platform = WindowsPlatform::new();

        // CPU count should be at least 1
        assert!(platform.cpu_count() >= 1);

        // Page size should be reasonable
        let page_size = platform.page_size();
        assert!(page_size >= 512);
        assert!(page_size <= 65536);
    }

    #[test]
    fn test_windows_platform_constants() {
        let platform = WindowsPlatform::new();

        assert_eq!(platform.line_separator(), "\r\n");
        assert_eq!(platform.path_separator(), ';');
        assert_eq!(platform.platform_name(), "windows");
    }

    #[test]
    fn test_executable_extensions() {
        let platform = WindowsPlatform::new();

        assert!(platform.is_executable(Path::new("program.exe")));
        assert!(platform.is_executable(Path::new("script.bat")));
        assert!(platform.is_executable(Path::new("script.cmd")));
        assert!(platform.is_executable(Path::new("installer.msi")));
        assert!(!platform.is_executable(Path::new("document.txt")));
        assert!(!platform.is_executable(Path::new("noextension")));
    }

    #[test]
    fn test_temp_dir() {
        let platform = WindowsPlatform::new();
        let temp = platform.temp_dir();
        // Just verify it returns a path
        assert!(!temp.as_os_str().is_empty());
    }
}
