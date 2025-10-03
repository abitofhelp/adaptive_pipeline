// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Pipeline Requirements Value Object
//!
//! This module provides the [`PipelineRequirements`] value object for
//! specifying performance, security, and processing requirements for adaptive
//! pipeline operations. It enables configuration-driven optimization and
//! ensures consistent requirement enforcement across the entire pipeline
//! system.
//!
//! ## Features
//!
//! - **Performance Configuration**: Throughput, memory, and processing
//!   requirements
//! - **Security Settings**: Encryption and compression requirement
//!   specification
//! - **Resource Constraints**: Memory limits and chunk size optimization
//! - **Adaptive Processing**: Parallel processing and optimization flags
//! - **Serialization Support**: Full serde compatibility for configuration
//!   persistence
//!
//! ## Architecture
//!
//! The `PipelineRequirements` follows Domain-Driven Design principles as a
//! value object, representing immutable configuration requirements that guide
//! pipeline behavior. It integrates with the pipeline's optimization engine to
//! ensure operations meet specified performance and security criteria.
//!
//! ## Usage Examples

use serde::{Deserialize, Serialize};

/// Pipeline requirements for optimization, security, and performance
/// configuration.
///
/// `PipelineRequirements` is a value object that encapsulates all configuration
/// requirements for pipeline operations, enabling adaptive optimization based
/// on performance targets, security needs, and resource constraints.
///
/// ## Key Features
///
/// - **Security Configuration**: Encryption and compression requirement flags
/// - **Performance Tuning**: Throughput targets and memory constraints
/// - **Resource Management**: Chunk size and memory limit specification
/// - **Processing Mode**: Parallel vs sequential processing configuration
/// - **Adaptive Optimization**: Requirements guide automatic optimization
///   decisions
///
/// ## Configuration Categories
///
/// ### Security Requirements
/// - `compression_enabled`: Whether to apply compression to reduce
///   storage/bandwidth
/// - `encryption_enabled`: Whether to encrypt data for security compliance
///
/// ### Performance Requirements
/// - `parallel_processing`: Enable multi-threaded processing for performance
/// - `target_throughput_mbps`: Target processing speed in megabytes per second
///
/// ### Resource Requirements
/// - `chunk_size_mb`: Processing chunk size for memory and I/O optimization
/// - `max_memory_mb`: Maximum memory usage limit for resource-constrained
///   environments
///
/// ## Usage Patterns
///
///
/// ## Integration with Pipeline System
///
/// The requirements integrate with various pipeline components:
///
/// - **Optimization Engine**: Uses requirements to select optimal algorithms
/// - **Resource Manager**: Enforces memory and processing constraints
/// - **Security Layer**: Applies encryption and compression based on flags
/// - **Performance Monitor**: Validates actual performance against targets
///
/// ## Thread Safety
///
/// `PipelineRequirements` is thread-safe through immutability. All fields are
/// read-only after construction, making it safe to share across threads without
/// synchronization.
///
/// ## Cross-Language Compatibility
///
/// The requirements structure maps well to other languages:
///
/// - **JSON**: Direct serialization for configuration files and APIs
/// - **Go**: Struct with similar field types and JSON tags
/// - **Python**: Dataclass with type hints for configuration management
/// - **YAML**: Configuration file format for deployment settings
///
/// ## Performance Considerations
///
/// - Lightweight value object with minimal memory overhead
/// - Efficient serialization through serde derive macros
/// - Immutable design eliminates defensive copying
/// - Optional fields reduce memory usage when constraints are not specified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRequirements {
    /// Whether compression should be applied to reduce storage and bandwidth
    /// usage
    pub compression_enabled: bool,

    /// Whether encryption should be applied for data security and compliance
    pub encryption_enabled: bool,

    /// Whether parallel processing should be used to improve performance
    pub parallel_processing: bool,

    /// Processing chunk size in megabytes for memory and I/O optimization
    pub chunk_size_mb: usize,

    /// Maximum memory usage limit in megabytes (None = no limit)
    pub max_memory_mb: Option<usize>,

    /// Target throughput in megabytes per second (None = no target)
    pub target_throughput_mbps: Option<f64>,
}

impl Default for PipelineRequirements {
    /// Creates default pipeline requirements optimized for security and
    /// performance.
    ///
    /// The default configuration provides a balanced approach suitable for most
    /// production environments, emphasizing security while maintaining good
    /// performance.
    ///
    /// # Default Values
    ///
    /// - `compression_enabled`: `true` - Reduces storage and bandwidth usage
    /// - `encryption_enabled`: `true` - Ensures data security by default
    /// - `parallel_processing`: `true` - Leverages multi-core systems
    /// - `chunk_size_mb`: `1` - Conservative chunk size for memory efficiency
    /// - `max_memory_mb`: `None` - No memory limit (system-dependent)
    /// - `target_throughput_mbps`: `None` - No specific throughput target
    ///
    /// # Examples
    ///
    ///
    /// # Security by Default
    ///
    /// The default configuration follows security best practices by enabling
    /// both compression and encryption. This ensures that data is protected
    /// and storage is optimized unless explicitly configured otherwise.
    ///
    /// # Performance Considerations
    ///
    /// While security is prioritized, the defaults also enable parallel
    /// processing to maintain good performance. The conservative 1MB chunk
    /// size balances memory usage with processing efficiency.
    fn default() -> Self {
        Self {
            compression_enabled: true,
            encryption_enabled: true,
            parallel_processing: true,
            chunk_size_mb: 1,
            max_memory_mb: None,
            target_throughput_mbps: None,
        }
    }
}
