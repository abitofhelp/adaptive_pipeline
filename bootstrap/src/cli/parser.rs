// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # CLI Parser
//!
//! Command-line interface parsing using clap.
//!
//! This module defines the CLI structure and handles argument parsing.
//! Security validation happens in the validator module after parsing.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main CLI structure
#[derive(Parser, Debug, Clone)]
#[command(name = "pipeline")]
#[command(about = concat!("Optimized Adaptive Pipeline RS v", env!("CARGO_PKG_VERSION")))]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    // === Resource Configuration Flags ===
    // Educational: These flags control the GlobalResourceManager's token allocation
    // for CPU-bound and I/O-bound operations.
    /// Override CPU worker thread count
    ///
    /// Controls the number of concurrent CPU-bound operations (compression,
    /// encryption). Default: num_cpus - 1 (reserves 1 core for I/O and
    /// coordination)
    ///
    /// Educational: Setting this too high causes thrashing, too low wastes
    /// cores. Monitor CPU saturation metrics to tune appropriately.
    #[arg(long)]
    pub cpu_threads: Option<usize>,

    /// Override I/O worker thread count
    ///
    /// Controls the number of concurrent I/O operations (file reads/writes).
    /// Default: Device-specific (NVMe: 24, SSD: 12, HDD: 4)
    ///
    /// Educational: This should match your storage device's queue depth for
    /// optimal throughput. Check --storage-type if auto-detection is
    /// incorrect.
    #[arg(long)]
    pub io_threads: Option<usize>,

    /// Specify storage device type for I/O optimization
    ///
    /// Affects default I/O thread count if --io-threads not specified.
    /// Values: nvme (queue depth 24), ssd (12), hdd (4)
    /// Default: auto-detect based on filesystem characteristics
    ///
    /// Educational: Different storage devices have different optimal queue
    /// depths. NVMe handles more concurrent I/O than SSD, which handles
    /// more than HDD.
    #[arg(long, value_parser = parse_storage_type)]
    pub storage_type: Option<String>,

    /// Channel depth for pipeline stages (Reader → Workers → Writer)
    ///
    /// Controls backpressure in the three-stage pipeline architecture.
    /// Default: 4
    ///
    /// Educational: Lower values reduce memory usage but may cause stalls.
    /// Higher values increase buffering but consume more memory.
    /// Optimal value depends on chunk processing time and I/O latency.
    ///
    /// Example: If chunk processing = 2ms and I/O = 1ms, depth=4 keeps pipeline
    /// full.
    #[arg(long, default_value = "4")]
    pub channel_depth: usize,
}

/// CLI subcommands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Process a file through a pipeline
    Process {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Pipeline name or ID
        #[arg(short, long)]
        pipeline: String,

        /// Chunk size in MB
        #[arg(long)]
        chunk_size_mb: Option<usize>,

        /// Number of parallel workers
        #[arg(long)]
        workers: Option<usize>,
    },

    /// Create a new pipeline
    Create {
        /// Pipeline name
        #[arg(short, long)]
        name: String,

        /// Pipeline stages (comma-separated: compression,encryption,integrity)
        #[arg(short, long)]
        stages: String,

        /// Save pipeline to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List available pipelines
    List,

    /// Show pipeline details
    Show {
        /// Pipeline name
        pipeline: String,
    },

    /// Delete a pipeline
    Delete {
        /// Pipeline name to delete
        pipeline: String,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Benchmark system performance
    Benchmark {
        /// Test file path
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Test data size in MB
        #[arg(long, default_value = "100")]
        size_mb: usize,

        /// Number of iterations
        #[arg(long, default_value = "3")]
        iterations: usize,
    },

    /// Validate pipeline configuration
    Validate {
        /// Pipeline configuration file
        config: PathBuf,
    },

    /// Validate .adapipe processed file
    ValidateFile {
        /// .adapipe file to validate
        #[arg(short, long)]
        file: PathBuf,

        /// Perform full streaming validation (decrypt/decompress and verify
        /// checksum)
        #[arg(long)]
        full: bool,
    },

    /// Restore original file from .adapipe file
    Restore {
        /// .adapipe file to restore from
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for restored file (optional - uses original
        /// directory if not specified)
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// Create directories without prompting
        #[arg(long)]
        mkdir: bool,

        /// Overwrite existing files without prompting
        #[arg(long)]
        overwrite: bool,
    },

    /// Compare original file against .adapipe file
    Compare {
        /// Original file to compare
        #[arg(short, long)]
        original: PathBuf,

        /// .adapipe file to compare against
        #[arg(short, long)]
        adapipe: PathBuf,

        /// Show detailed differences
        #[arg(long)]
        detailed: bool,
    },
}

/// Parse and validate storage type from CLI argument
///
/// Educational: Custom value parser for clap that validates
/// storage type strings and provides helpful error messages.
fn parse_storage_type(s: &str) -> Result<String, String> {
    match s.to_lowercase().as_str() {
        "nvme" | "ssd" | "hdd" => Ok(s.to_lowercase()),
        _ => Err(format!("Invalid storage type '{}'. Valid options: nvme, ssd, hdd", s)),
    }
}

/// Parse CLI arguments
///
/// This is the entry point for CLI parsing. It uses clap to parse
/// arguments and returns the parsed CLI structure.
///
/// # Returns
///
/// Parsed `Cli` structure with all arguments
///
/// # Panics
///
/// Clap will exit the process with appropriate error message if parsing fails
pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_storage_type_valid() {
        assert_eq!(parse_storage_type("nvme").unwrap(), "nvme");
        assert_eq!(parse_storage_type("SSD").unwrap(), "ssd");
        assert_eq!(parse_storage_type("HDD").unwrap(), "hdd");
    }

    #[test]
    fn test_parse_storage_type_invalid() {
        assert!(parse_storage_type("invalid").is_err());
        assert!(parse_storage_type("usb").is_err());
    }
}
