// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Common Test Helpers
//!
//! Shared utilities for integration and end-to-end tests.

/// Get the path to the compiled pipeline binary
///
/// This helper tries the CARGO_BIN_EXE environment variable first (set by cargo
/// test), then falls back to constructing the path from CARGO_MANIFEST_DIR.
///
/// # Returns
///
/// The absolute path to the `adaptive_pipeline` binary.
///
/// # Panics
///
/// Panics if the path cannot be constructed or contains invalid UTF-8.
pub fn get_pipeline_bin() -> String {
    // Try environment variable first (set by cargo test)
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_adaptive_pipeline") {
        return path;
    }

    // Fallback: construct path from CARGO_MANIFEST_DIR
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut bin_path = std::path::PathBuf::from(manifest_dir);
    bin_path.pop(); // Go up to workspace root
    bin_path.push("target");
    bin_path.push(if cfg!(debug_assertions) { "debug" } else { "release" });
    bin_path.push("adaptive_pipeline");

    bin_path.to_str().expect("Invalid UTF-8 in binary path").to_string()
}

/// Calculate SHA256 checksum of data
///
/// # Arguments
///
/// * `data` - The data to hash
///
/// # Returns
///
/// Hex-encoded SHA256 hash as a string
pub fn calculate_sha256(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pipeline_bin_returns_path() {
        let bin_path = get_pipeline_bin();
        assert!(!bin_path.is_empty());
        assert!(bin_path.contains("adaptive_pipeline"));
    }

    #[test]
    fn test_calculate_sha256() {
        let data = b"test data";
        let hash = calculate_sha256(data);

        // SHA256 produces 64 hex characters
        assert_eq!(hash.len(), 64);

        // Should be deterministic
        assert_eq!(hash, calculate_sha256(data));
    }
}
