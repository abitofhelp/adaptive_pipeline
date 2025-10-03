//! # Unix Platform Implementation
//!
//! POSIX-compliant implementation for Linux and macOS.
//!
//! ## Platform APIs Used
//!
//! - **System Info**: `libc::sysconf` for page size and CPU count
//! - **Memory Info**:
//!   - Linux: `/proc/meminfo` parsing
//!   - macOS: `sysctlbyname` syscalls
//! - **Security**: `libc::geteuid` for privilege checking
//! - **Permissions**: `std::os::unix::fs::PermissionsExt`
//! - **File Sync**: `tokio::fs::File::sync_all`

use super::{Platform, PlatformError};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// Unix (POSIX) platform implementation
///
/// Supports Linux and macOS using POSIX APIs and platform-specific syscalls.
pub struct UnixPlatform;

impl UnixPlatform {
    /// Create a new Unix platform instance
    pub fn new() -> Self {
        Self
    }

    /// Get memory information on Linux by parsing /proc/meminfo
    #[cfg(target_os = "linux")]
    fn get_memory_info_linux() -> Result<(u64, u64), PlatformError> {
        use std::fs;

        let meminfo = fs::read_to_string("/proc/meminfo")
            .map_err(|e| PlatformError::Other(format!("Failed to read /proc/meminfo: {}", e)))?;

        let mut total = None;
        let mut available = None;

        for line in meminfo.lines() {
            if let Some(value) = line.strip_prefix("MemTotal:") {
                total = value
                    .trim()
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|kb| kb * 1024); // Convert KB to bytes
            } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                available = value
                    .trim()
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|kb| kb * 1024); // Convert KB to bytes
            }

            if total.is_some() && available.is_some() {
                break;
            }
        }

        match (total, available) {
            (Some(t), Some(a)) => Ok((t, a)),
            _ => Err(PlatformError::Other(
                "Failed to parse memory info".to_string(),
            )),
        }
    }

    /// Get memory information on macOS using sysctl
    #[cfg(target_os = "macos")]
    fn get_memory_info_macos() -> Result<(u64, u64), PlatformError> {
        use std::mem;

        unsafe {
            // Get total memory
            let mut total: u64 = 0;
            let mut size = mem::size_of::<u64>();
            let name = b"hw.memsize\0".as_ptr() as *const i8;

            if libc::sysctlbyname(
                name,
                &mut total as *mut _ as *mut libc::c_void,
                &mut size,
                std::ptr::null_mut(),
                0,
            ) != 0
            {
                return Err(PlatformError::Other(
                    "Failed to get total memory via sysctl".to_string(),
                ));
            }

            // Get available memory (approximate using free memory)
            // Note: This is an approximation. For exact VM stats, would need mach APIs
            let mut available: u64 = 0;
            let mut avail_size = mem::size_of::<u64>();
            let avail_name = b"vm.page_free_count\0".as_ptr() as *const i8;

            if libc::sysctlbyname(
                avail_name,
                &mut available as *mut _ as *mut libc::c_void,
                &mut avail_size,
                std::ptr::null_mut(),
                0,
            ) != 0
            {
                // If can't get exact available, estimate as half of total
                // This is very rough but prevents errors
                available = total / 2;
            } else {
                // Convert pages to bytes
                let page_size = Self::page_size_impl();
                available *= page_size;
            }

            Ok((total, available))
        }
    }

    /// Internal implementation of page_size
    fn page_size_impl() -> u64 {
        unsafe {
            let size = libc::sysconf(libc::_SC_PAGESIZE);
            if size > 0 {
                size as u64
            } else {
                4096 // Default fallback
            }
        }
    }
}

impl Default for UnixPlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Platform for UnixPlatform {
    fn page_size(&self) -> usize {
        Self::page_size_impl() as usize
    }

    fn cpu_count(&self) -> usize {
        unsafe {
            let count = libc::sysconf(libc::_SC_NPROCESSORS_ONLN);
            if count > 0 {
                count as usize
            } else {
                1 // Fallback to 1 CPU
            }
        }
    }

    fn total_memory(&self) -> Result<u64, PlatformError> {
        #[cfg(target_os = "linux")]
        {
            Self::get_memory_info_linux().map(|(total, _)| total)
        }

        #[cfg(target_os = "macos")]
        {
            Self::get_memory_info_macos().map(|(total, _)| total)
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            Err(PlatformError::NotSupported(
                "Memory info not supported on this Unix variant".to_string(),
            ))
        }
    }

    fn available_memory(&self) -> Result<u64, PlatformError> {
        #[cfg(target_os = "linux")]
        {
            Self::get_memory_info_linux().map(|(_, available)| available)
        }

        #[cfg(target_os = "macos")]
        {
            Self::get_memory_info_macos().map(|(_, available)| available)
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            Err(PlatformError::NotSupported(
                "Memory info not supported on this Unix variant".to_string(),
            ))
        }
    }

    fn line_separator(&self) -> &'static str {
        "\n"
    }

    fn path_separator(&self) -> char {
        ':'
    }

    fn platform_name(&self) -> &'static str {
        #[cfg(target_os = "linux")]
        return "linux";

        #[cfg(target_os = "macos")]
        return "macos";

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        return "unix";
    }

    fn temp_dir(&self) -> PathBuf {
        std::env::temp_dir()
    }

    fn is_elevated(&self) -> bool {
        unsafe { libc::geteuid() == 0 }
    }

    fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), PlatformError> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let permissions = fs::Permissions::from_mode(mode);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    fn is_executable(&self, path: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(metadata) = std::fs::metadata(path) {
            let permissions = metadata.permissions();
            let mode = permissions.mode();
            // Check if any execute bit is set (owner, group, or other)
            (mode & 0o111) != 0
        } else {
            false
        }
    }

    async fn sync_file(&self, file: &tokio::fs::File) -> Result<(), PlatformError> {
        file.sync_all().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_platform_basics() {
        let platform = UnixPlatform::new();

        // CPU count should be at least 1
        assert!(platform.cpu_count() >= 1);

        // Page size should be reasonable
        let page_size = platform.page_size();
        assert!(page_size >= 512);
        assert!(page_size <= 65536);
    }

    #[test]
    fn test_unix_platform_constants() {
        let platform = UnixPlatform::new();

        assert_eq!(platform.line_separator(), "\n");
        assert_eq!(platform.path_separator(), ':');

        let name = platform.platform_name();
        assert!(name == "linux" || name == "macos" || name == "unix");
    }

    #[test]
    fn test_memory_info() {
        let platform = UnixPlatform::new();

        // Total memory should be retrievable
        let total = platform.total_memory();
        assert!(total.is_ok());
        if let Ok(t) = total {
            assert!(t > 0);
        }

        // Available memory should be retrievable
        let available = platform.available_memory();
        assert!(available.is_ok());
        if let Ok(a) = available {
            assert!(a > 0);
        }
    }

    #[test]
    fn test_temp_dir() {
        let platform = UnixPlatform::new();
        let temp = platform.temp_dir();
        assert!(temp.exists());
    }

    #[test]
    fn test_is_elevated() {
        let platform = UnixPlatform::new();
        // Just make sure it doesn't panic
        let _ = platform.is_elevated();
    }
}
