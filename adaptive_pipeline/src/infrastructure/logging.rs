// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Infrastructure Logging
//!
//! This module provides comprehensive logging and observability capabilities
//! for the infrastructure layer. It implements structured logging, distributed
//! tracing, and integration with observability platforms.
//!
//! ## Overview
//!
//! The logging infrastructure provides:
//!
//! - **Structured Logging**: JSON-formatted logs with contextual information
//! - **Log Levels**: Configurable severity levels (error, warn, info, debug,
//!   trace)
//! - **Distributed Tracing**: Trace requests across services and components
//! - **Log Aggregation**: Integration with log aggregation systems
//! - **Performance Monitoring**: Low-overhead logging for production
//! - **Security**: Sensitive data filtering and redaction
//!
//! ## Design Principles
//!
//! ### Structured Logging
//! All logs are structured for easy parsing and analysis:
//!
//!
//! ### Log Levels
//! Use appropriate log levels for different scenarios:
//!
//!
//! ## Distributed Tracing
//!
//! Track requests across service boundaries:
//!
//!
//! ## Log Formatting
//!
//! ### JSON Format
//! Production logs use JSON for machine parsing:
//!
//! ```json
//! {
//!   "timestamp": "2024-01-15T10:30:00.123Z",
//!   "level": "INFO",
//!   "message": "Processing file",
//!   "trace_id": "550e8400-e29b-41d4-a716-446655440000",
//!   "span_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
//!   "context": {
//!     "file_path": "/data/input.txt",
//!     "pipeline_id": "secure-backup",
//!     "file_size_bytes": 1024000
//!   },
//!   "host": "server-01",
//!   "service": "pipeline",
//!   "version": "1.0.0"
//! }
//! ```
//!
//! ### Human-Readable Format
//! Development logs use human-readable format:
//!
//! ```text
//! 2024-01-15 10:30:00.123 INFO [trace:550e8400] Processing file
//!   file_path: /data/input.txt
//!   pipeline_id: secure-backup
//!   file_size_bytes: 1024000
//! ```
//!
//! ## Configuration
//!
//! Configure logging through the observability service:
//!
//!
//! ## Log Outputs
//!
//! ### Console Output
//! Log to stdout/stderr:
//!
//!
//! ### File Output
//! Log to rotating files:
//!
//!
//! ### Syslog Output
//! Log to syslog:
//!
//!
//! ## Sensitive Data Filtering
//!
//! Automatically redact sensitive information:
//!
//!
//! ## Performance Considerations
//!
//! ### Async Logging
//! Logs are written asynchronously to avoid blocking:
//!
//!
//! ### Sampling
//! Sample high-volume logs:
//!
//!
//! ## Integration Examples
//!
//! ### With Domain Events
//!
//!
//! ### With Error Handling

pub mod observability;
pub use observability::*;
