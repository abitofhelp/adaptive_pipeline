// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

// Enforce zero-panic production code at compile time
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]

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
//! use bootstrap::platform::create_platform;
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
pub mod platform;
pub mod exit_code;
pub mod logger;
pub mod signals;
pub mod config;
pub mod cli;
pub mod shutdown;

// Future modules (to be implemented)
// pub mod composition_root;
// pub mod app_runner;
