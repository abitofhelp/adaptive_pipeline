// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Command-Line Interface Module
//!
//! Bootstrap-layer CLI handling with security-first design.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  1. parser::parse()                 │  Parse CLI with clap
//! └─────────────────┬───────────────────┘
//!                   ↓
//! ┌─────────────────────────────────────┐
//! │  2. validator::validate()           │  Security validation
//! └─────────────────┬───────────────────┘
//!                   ↓
//! ┌─────────────────────────────────────┐
//! │  3. ValidatedConfig                 │  Safe, validated config
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Modules
//!
//! - `parser` - CLI structure and clap parsing
//! - `validator` - Security validation layer
//! - `commands` - Validated command parameters

pub mod parser;
pub mod validator;

pub use parser::{parse_cli, Cli, Commands};
pub use validator::{ParseError, SecureArgParser};

use std::path::PathBuf;

/// Validated CLI configuration
///
/// This structure holds all CLI arguments after security validation.
/// All paths are canonicalized and all values are range-checked.
#[derive(Debug, Clone)]
pub struct ValidatedCli {
    pub command: ValidatedCommand,
    pub verbose: bool,
    pub config: Option<PathBuf>,
    pub cpu_threads: Option<usize>,
    pub io_threads: Option<usize>,
    pub storage_type: Option<String>,
    pub channel_depth: usize,
}

/// Validated command variants
#[derive(Debug, Clone)]
pub enum ValidatedCommand {
    Process {
        input: PathBuf,
        output: PathBuf,
        pipeline: String,
        chunk_size_mb: Option<usize>,
        workers: Option<usize>,
    },
    Create {
        name: String,
        stages: String,
        output: Option<PathBuf>,
    },
    List,
    Show {
        pipeline: String,
    },
    Delete {
        pipeline: String,
        force: bool,
    },
    Benchmark {
        file: Option<PathBuf>,
        size_mb: usize,
        iterations: usize,
    },
    Validate {
        config: PathBuf,
    },
    ValidateFile {
        file: PathBuf,
        full: bool,
    },
    Restore {
        input: PathBuf,
        output_dir: Option<PathBuf>,
        mkdir: bool,
        overwrite: bool,
    },
    Compare {
        original: PathBuf,
        adapipe: PathBuf,
        detailed: bool,
    },
}

/// Parse and validate CLI arguments
///
/// This function combines parsing and validation:
/// 1. Parse CLI with clap
/// 2. Validate all paths with SecureArgParser
/// 3. Validate all numeric values
/// 4. Return ValidatedCli on success
///
/// # Returns
///
/// `ValidatedCli` with all arguments security-checked
///
/// # Errors
///
/// Returns `ParseError` if any validation fails
pub fn parse_and_validate() -> Result<ValidatedCli, ParseError> {
    let cli = parse_cli();
    validate_cli(cli)
}

/// Validate parsed CLI arguments
///
/// Applies security validation to all CLI arguments:
/// - Path canonicalization and security checks
/// - Numeric range validation
/// - String pattern validation
///
/// # Errors
///
/// Returns `ParseError` if any validation fails
fn validate_cli(cli: Cli) -> Result<ValidatedCli, ParseError> {
    // Validate global config path if provided
    let config = if let Some(ref path) = cli.config {
        // For output paths that don't exist yet, just validate the string
        SecureArgParser::validate_argument(&path.to_string_lossy())?;
        Some(path.clone())
    } else {
        None
    };

    // Validate channel depth
    if cli.channel_depth == 0 {
        return Err(ParseError::InvalidValue {
            arg: "channel-depth".to_string(),
            reason: "must be greater than 0".to_string(),
        });
    }

    // Validate CPU threads if specified
    if let Some(threads) = cli.cpu_threads {
        if threads == 0 || threads > 128 {
            return Err(ParseError::InvalidValue {
                arg: "cpu-threads".to_string(),
                reason: "must be between 1 and 128".to_string(),
            });
        }
    }

    // Validate I/O threads if specified
    if let Some(threads) = cli.io_threads {
        if threads == 0 || threads > 256 {
            return Err(ParseError::InvalidValue {
                arg: "io-threads".to_string(),
                reason: "must be between 1 and 256".to_string(),
            });
        }
    }

    // Validate command-specific arguments
    let command = match cli.command {
        Commands::Process {
            input,
            output,
            pipeline,
            chunk_size_mb,
            workers,
        } => {
            // Validate input file exists
            let validated_input = SecureArgParser::validate_path(&input.to_string_lossy())?;

            // Output file doesn't exist yet - validate string only
            SecureArgParser::validate_argument(&output.to_string_lossy())?;

            // Validate pipeline name (no dangerous patterns)
            SecureArgParser::validate_argument(&pipeline)?;

            // Validate chunk size if specified
            if let Some(size) = chunk_size_mb {
                if size == 0 || size > 1024 {
                    return Err(ParseError::InvalidValue {
                        arg: "chunk-size-mb".to_string(),
                        reason: "must be between 1 and 1024 MB".to_string(),
                    });
                }
            }

            // Validate workers if specified
            if let Some(w) = workers {
                if w == 0 || w > 128 {
                    return Err(ParseError::InvalidValue {
                        arg: "workers".to_string(),
                        reason: "must be between 1 and 128".to_string(),
                    });
                }
            }

            ValidatedCommand::Process {
                input: validated_input,
                output,
                pipeline,
                chunk_size_mb,
                workers,
            }
        }
        Commands::Create { name, stages, output } => {
            SecureArgParser::validate_argument(&name)?;
            SecureArgParser::validate_argument(&stages)?;

            if let Some(ref path) = output {
                SecureArgParser::validate_argument(&path.to_string_lossy())?;
            }

            ValidatedCommand::Create { name, stages, output }
        }
        Commands::List => ValidatedCommand::List,
        Commands::Show { pipeline } => {
            SecureArgParser::validate_argument(&pipeline)?;
            ValidatedCommand::Show { pipeline }
        }
        Commands::Delete { pipeline, force } => {
            SecureArgParser::validate_argument(&pipeline)?;
            ValidatedCommand::Delete { pipeline, force }
        }
        Commands::Benchmark {
            file,
            size_mb,
            iterations,
        } => {
            let validated_file = if let Some(ref path) = file {
                Some(SecureArgParser::validate_path(&path.to_string_lossy())?)
            } else {
                None
            };

            if size_mb == 0 || size_mb > 100_000 {
                return Err(ParseError::InvalidValue {
                    arg: "size-mb".to_string(),
                    reason: "must be between 1 and 100000 MB".to_string(),
                });
            }

            if iterations == 0 || iterations > 1000 {
                return Err(ParseError::InvalidValue {
                    arg: "iterations".to_string(),
                    reason: "must be between 1 and 1000".to_string(),
                });
            }

            ValidatedCommand::Benchmark {
                file: validated_file,
                size_mb,
                iterations,
            }
        }
        Commands::Validate { config } => {
            let validated_config = SecureArgParser::validate_path(&config.to_string_lossy())?;
            ValidatedCommand::Validate {
                config: validated_config,
            }
        }
        Commands::ValidateFile { file, full } => {
            let validated_file = SecureArgParser::validate_path(&file.to_string_lossy())?;
            ValidatedCommand::ValidateFile {
                file: validated_file,
                full,
            }
        }
        Commands::Restore {
            input,
            output_dir,
            mkdir,
            overwrite,
        } => {
            let validated_input = SecureArgParser::validate_path(&input.to_string_lossy())?;

            let validated_output_dir = if let Some(ref path) = output_dir {
                // Output dir might not exist yet
                SecureArgParser::validate_argument(&path.to_string_lossy())?;
                Some(path.clone())
            } else {
                None
            };

            ValidatedCommand::Restore {
                input: validated_input,
                output_dir: validated_output_dir,
                mkdir,
                overwrite,
            }
        }
        Commands::Compare {
            original,
            adapipe,
            detailed,
        } => {
            let validated_original = SecureArgParser::validate_path(&original.to_string_lossy())?;
            let validated_adapipe = SecureArgParser::validate_path(&adapipe.to_string_lossy())?;
            ValidatedCommand::Compare {
                original: validated_original,
                adapipe: validated_adapipe,
                detailed,
            }
        }
    };

    Ok(ValidatedCli {
        command,
        verbose: cli.verbose,
        config,
        cpu_threads: cli.cpu_threads,
        io_threads: cli.io_threads,
        storage_type: cli.storage_type,
        channel_depth: cli.channel_depth,
    })
}
