//! Repository adapters for the adaptive pipeline system.
//!
//! This module contains adapter implementations that bridge domain repository
//! interfaces with concrete infrastructure implementations. These adapters
//! follow the Hexagonal Architecture pattern and implement the Dependency
//! Inversion Principle (DIP).
//!
//! ## Overview
//!
//! Repository adapters provide:
//!
//! - **Domain Interface Implementation**: Implement domain repository traits
//! - **Storage Backend Abstraction**: Abstract away specific storage
//!   implementations
//! - **Seamless Integration**: Enable switching between different storage
//!   backends
//! - **Type Safety**: Compile-time guarantees for repository operations
//! - **Clean Architecture**: Maintain proper layer separation and dependency
//!   inversion
//!
//! ## Available Adapters
//!
//! - **SQLite Repository Adapter**: Bridges domain repositories to SQLite
//!   storage
//! - **Stage Executor Adapter**: Bridges domain stage executor to
//!   infrastructure services
//! ### SQLite Repository Adapter
//!
//! The SQLite repository adapter bridges domain repository interfaces with
//! SQLite-based storage implementations:
//!
//! - **File-based Storage**: Persistent storage using SQLite database files
//! - **In-memory Storage**: Fast, temporary storage for testing and caching
//! - **Transaction Support**: Full ACID transaction support
//! - **Connection Pooling**: Efficient connection management
//! - **Schema Management**: Automatic schema creation and migration
//!
//! ## Usage Examples
//!
//! ### Basic Repository Adapter Usage
//!
//!
//! ## Architecture Benefits
//!
//! - **Dependency Inversion**: Domain depends on abstractions, not concrete
//!   implementations
//! - **Testability**: Easy to swap implementations for testing
//! - **Flexibility**: Runtime selection of storage backends
//! - **Maintainability**: Clear separation of concerns

pub mod generic_repository_adapter;
pub mod sqlite_base_repository;
pub mod sqlite_pipeline_repository_adapter;
pub mod sqlite_repository_adapter;
pub mod stage_executor_adapter;

// Re-export adapter implementations for easy access
pub use generic_repository_adapter::*;
pub use sqlite_base_repository::*;
