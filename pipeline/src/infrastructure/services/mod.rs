// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

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
//! - **Base64EncodingService**: Production Base64 encoding/decoding stage
//! - **PiiMaskingService**: Production PII masking stage (non-reversible)
//! - **TeeService**: Production data inspection/debugging stage (pass-through)

pub mod base64_encoding_service;
pub mod binary_format_service;
pub mod pii_masking_service;
pub mod progress_indicator_service;
pub mod tee_service;

// Re-export service implementations
pub use base64_encoding_service::Base64EncodingService;
pub use binary_format_service::{BinaryFormatService, BinaryFormatServiceImpl, BinaryFormatWriter};
pub use pii_masking_service::PiiMaskingService;
pub use tee_service::TeeService;
