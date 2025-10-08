// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

// Production code safety enforced via CI and `make lint-strict`
// (lib/bins checked separately from tests - tests may use unwrap/expect)

//! # Bootstrap Module
//!
//! The bootstrap module sits **outside** the enterprise application layers
//! (domain, application, infrastructure) and provides:
//!
//! - **Entry point** - Application lifecycle management
//! - **Platform abstraction** - OS-specific operations (POSIX vs Windows)
//! - **Signal handling** - Graceful shutdown (SIGTERM, SIGINT, SIGHUP)
//! - **Argument parsing** - Secure CLI argument validation
//! - **Dependency injection** - Composition root for wiring dependencies
//! - **Error handling** - Unix exit code mapping
//! - **Async coordination** - Shutdown coordination and cancellation
//!
//! ## Architecture Position
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │          BOOTSTRAP (This Module)            │
//! │  - Entry Point                              │
//! │  - DI Container (Composition Root)          │
//! │  - Platform Abstraction                     │
//! │  - Signal Handling                          │
//! │  - Secure Arg Parsing                       │
//! └─────────────────────────────────────────────┘
//!                      │
//!                      ▼
//! ┌─────────────────────────────────────────────┐
//! │         APPLICATION LAYER                   │
//! │  - Use Cases                                │
//! │  - Application Services                     │
//! └─────────────────────────────────────────────┘
//!                      │
//!                      ▼
//! ┌─────────────────────────────────────────────┐
//! │           DOMAIN LAYER                      │
//! │  - Business Logic                           │
//! │  - Domain Services                          │
//! │  - Entities & Value Objects                 │
//! └─────────────────────────────────────────────┘
//!                      ▲
//!                      │
//! ┌─────────────────────────────────────────────┐
//! │       INFRASTRUCTURE LAYER                  │
//! │  - Adapters                                 │
//! │  - Repositories                             │
//! │  - External Services                        │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! ## Key Design Principles
//!
//! 1. **Separation from Enterprise Layers**
//!    - Bootstrap can access all layers
//!    - Enterprise layers cannot access bootstrap
//!    - Clear architectural boundary
//!
//! 2. **Platform Abstraction**
//!    - Abstract OS-specific functionality behind traits
//!    - POSIX implementation for Linux/macOS
//!    - Windows implementation with cross-platform stubs
//!    - Compile-time platform selection
//!
//! 3. **Graceful Shutdown**
//!    - Signal handlers (SIGTERM, SIGINT, SIGHUP)
//!    - Cancellation token propagation
//!    - Grace period with timeout enforcement
//!    - Coordinated shutdown across components
//!
//! 4. **Security First**
//!    - Input validation for all arguments
//!    - Path traversal prevention
//!    - Injection attack protection
//!    - Privilege checking
//!
//! 5. **Testability**
//!    - All components behind traits
//!    - No-op implementations for testing
//!    - Dependency injection for mocking
//!
//! ## Usage Example
//!
//! ```rust
//! use adaptive_pipeline_bootstrap::platform::create_platform;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get platform abstraction
//!     let platform = create_platform();
//!     println!("Running on: {}", platform.platform_name());
//!
//!     // Bootstrap will handle:
//!     // - Argument parsing
//!     // - Signal handling setup
//!     // - Dependency wiring
//!     // - Application lifecycle
//!     // - Graceful shutdown
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Module Structure
//!
//! - `platform` - OS abstraction (Unix/Windows)
//! - `signals` - Signal handling (SIGTERM, SIGINT, SIGHUP)
//! - `cli` - Secure argument parsing
//! - `config` - Application configuration
//! - `exit_code` - Unix exit code enumeration
//! - `logger` - Bootstrap-specific logging
//! - `shutdown` - Shutdown coordination
//! - `composition_root` - Dependency injection container
//! - `app_runner` - Application lifecycle management

// Re-export modules
pub mod cli; // Now a module directory with parser and validator
pub mod config;
pub mod exit_code;
pub mod logger;
pub mod platform;
pub mod shutdown;
pub mod signals;

// Future modules (to be implemented)
// pub mod composition_root;
// pub mod app_runner;

// Re-export commonly used types
pub use cli::{parse_and_validate, ValidatedCli, ValidatedCommand};
pub use exit_code::{map_error_to_exit_code, result_to_exit_code, ExitCode};

/// Bootstrap and parse CLI arguments
///
/// This is the main entry point for the bootstrap layer.
/// It handles:
/// 1. CLI parsing with clap
/// 2. Security validation
/// 3. Returns validated configuration
///
/// The caller is responsible for:
/// - Running the application logic
/// - Mapping results to exit codes using `result_to_exit_code`
///
/// # Returns
///
/// `ValidatedCli` with all arguments security-checked and validated
///
/// # Errors
///
/// Returns `cli::ParseError` if CLI parsing or validation fails.
/// Clap will handle --help and --version automatically and exit the process.
///
/// # Example
///
/// ```no_run
/// use adaptive_pipeline_bootstrap::{bootstrap_cli, result_to_exit_code};
///
/// #[tokio::main]
/// async fn main() -> std::process::ExitCode {
///     // Parse and validate CLI
///     let validated_cli = match bootstrap_cli() {
///         Ok(cli) => cli,
///         Err(e) => {
///             eprintln!("CLI Error: {}", e);
///             return std::process::ExitCode::from(65); // EX_DATAERR
///         }
///     };
///
///     // Run application with validated config
///     let result = run_application(validated_cli).await;
///
///     // Map result to exit code
///     result_to_exit_code(result)
/// }
///
/// async fn run_application(cli: adaptive_pipeline_bootstrap::ValidatedCli) -> Result<(), String> {
///     // Application logic here
///     Ok(())
/// }
/// ```
pub fn bootstrap_cli() -> Result<ValidatedCli, cli::ParseError> {
    cli::parse_and_validate()
}
