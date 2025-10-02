//! # Infrastructure Services Module
//!
//! This module contains infrastructure-specific services that don't fit into
//! adapters, repositories, or other infrastructure categories. These are
//! cross-cutting infrastructure concerns.
//!
//! ## Services
//!
//! - **BinaryFormatService**: Binary .adapipe format reading and writing
//! - **ProgressIndicator**: Real-time progress tracking and terminal output

pub mod binary_format_service;
pub mod progress_indicator_service;

// Re-export service implementations
pub use binary_format_service::{
    BinaryFormatService, BinaryFormatServiceImpl, BinaryFormatWriter,
};
