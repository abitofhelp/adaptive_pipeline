//! # Encryption Benchmark Value Object
//!
//! This module provides the [`EncryptionBenchmark`] value object for capturing
//! and analyzing encryption performance metrics in the adaptive pipeline
//! system. It enables performance monitoring, algorithm comparison, and
//! optimization decisions based on real-world benchmarks.
//!
//! ## Features
//!
//! - **Performance Metrics**: Comprehensive capture of throughput, latency,
//!   memory, and CPU usage
//! - **Algorithm Comparison**: Standardized benchmarking across different
//!   encryption algorithms
//! - **Temporal Tracking**: RFC3339-compliant timestamp recording for trend
//!   analysis
//! - **Serialization Support**: Full serde compatibility for persistence and
//!   transmission
//! - **Immutable Design**: Value object semantics with snapshot-based
//!   performance data
//!
//! ## Architecture
//!
//! The `EncryptionBenchmark` follows Domain-Driven Design principles as a value
//! object, representing an immutable snapshot of encryption performance at a
//! specific point in time. It integrates with the pipeline's monitoring and
//! optimization systems to guide algorithm selection and resource allocation
//! decisions.
//!
//! ## Usage Examples
//!
//! ### Creating Benchmark Results
//!
//!
//! ### Comparing Algorithm Performance
//!
//!
//!
//! ## Performance Analysis
//!
//! ### Throughput Analysis
//!
//! Throughput measurements provide insight into sustained data processing rates:
//!
//! - **High Throughput (>200 MB/s)**: Excellent for large file processing
//! - **Medium Throughput (50-200 MB/s)**: Good for general-purpose encryption
//! - **Low Throughput (<50 MB/s)**: May indicate CPU bottleneck or algorithm
//!   inefficiency
//!
//! ### Latency Considerations
//!
//! Latency affects responsiveness in interactive scenarios:
//!
//! - **Low Latency (<10ms)**: Suitable for real-time processing
//! - **Medium Latency (10-100ms)**: Acceptable for batch processing
//! - **High Latency (>100ms)**: May require optimization or algorithm change
//!
//! ### Resource Utilization
//!
//! Memory and CPU usage inform resource allocation decisions:
//!
//! - **Memory Usage**: Higher usage may indicate buffering or inefficient
//!   implementation
//! - **CPU Usage**: Should correlate with throughput; high CPU with low
//!   throughput indicates inefficiency
//!
//! ### Efficiency Metrics
//!
//! Calculate efficiency ratios for informed decisions:
//!
//!
//!
//! ## Serialization and Persistence
//!
//! ### JSON Serialization
//!
//!
//! ### Database Storage
//!
//! Benchmark data maps well to relational databases:
//!
//! ```sql
//! CREATE TABLE encryption_benchmarks (
//!     id SERIAL PRIMARY KEY,
//!     algorithm VARCHAR(50) NOT NULL,
//!     throughput_mbps DOUBLE PRECISION NOT NULL,
//!     latency_ms INTEGER NOT NULL,
//!     memory_usage_mb DOUBLE PRECISION NOT NULL,
//!     cpu_usage_percent DOUBLE PRECISION NOT NULL,
//!     file_size_mb DOUBLE PRECISION NOT NULL,
//!     timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
//!     INDEX idx_algorithm (algorithm),
//!     INDEX idx_timestamp (timestamp)
//! );
//! ```
//!
//! ### Time Series Analysis
//!
//! The RFC3339-compliant timestamp enables trend analysis:
//!
//! - Track performance changes over time
//! - Identify performance regressions
//! - Correlate performance with system changes
//! - Compare algorithm performance across versions
//!
use crate::services::datetime_serde;
use crate::services::encryption_service::EncryptionAlgorithm;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Encryption performance benchmark results for algorithm comparison and
/// optimization.
///
/// `EncryptionBenchmark` is a value object that captures comprehensive
/// performance metrics for encryption operations, enabling data-driven
/// decisions about algorithm selection, resource allocation, and system
/// optimization in the adaptive pipeline.
///
/// ## Key Features
///
/// - **Comprehensive Metrics**: Captures throughput, latency, memory usage, and
///   CPU utilization
/// - **Algorithm Tracking**: Associates metrics with specific encryption
///   algorithms
/// - **Temporal Context**: Includes RFC3339-compliant timestamp for trend
///   analysis
/// - **Immutable Snapshot**: Represents performance at a specific point in time
/// - **Serialization Ready**: Full serde support for persistence and data
///   exchange
///
/// ## Performance Metrics
///
/// - `throughput_mbps`: Data processing rate in megabytes per second
/// - `latency`: Time delay from operation start to completion
/// - `memory_usage_mb`: Peak memory consumption during encryption
/// - `cpu_usage_percent`: CPU utilization percentage during operation
/// - `file_size_mb`: Size of the file being encrypted (for context)
///
/// ## Usage Patterns
///
///
/// ## Integration with Monitoring
///
/// The benchmark integrates with the pipeline's monitoring and optimization
/// systems:
///
/// - **Performance Tracking**: Historical performance data for trend analysis
/// - **Algorithm Selection**: Data-driven algorithm choice based on workload
///   characteristics
/// - **Resource Planning**: Memory and CPU requirement estimation
/// - **Optimization Feedback**: Performance regression detection and
///   optimization validation
///
/// ## Thread Safety
///
/// `EncryptionBenchmark` is thread-safe through immutability. All fields are
/// read-only after construction, making it safe to share across threads without
/// synchronization.
///
/// ## Cross-Language Compatibility
///
/// The benchmark data structure maps well to other languages:
///
/// - **JSON**: Direct serialization for REST APIs and configuration
/// - **Go**: Struct with similar field types and JSON tags
/// - **Python**: Dataclass or NamedTuple with datetime handling
/// - **Database**: Relational table with appropriate column types
///
/// ## Performance Considerations
///
/// - Lightweight value object with minimal memory overhead
/// - Efficient serialization through serde derive macros
/// - Immutable design eliminates defensive copying
/// - Timestamp generation uses UTC to avoid timezone complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionBenchmark {
    /// The encryption algorithm that was benchmarked
    pub algorithm: EncryptionAlgorithm,

    /// Data processing throughput in megabytes per second
    pub throughput_mbps: f64,

    /// Operation latency from start to completion
    pub latency: Duration,

    /// Peak memory usage during encryption in megabytes
    pub memory_usage_mb: f64,

    /// CPU utilization percentage during the operation
    pub cpu_usage_percent: f64,

    /// Size of the file being encrypted in megabytes (for context)
    pub file_size_mb: f64,

    /// RFC3339-compliant timestamp when the benchmark was recorded
    #[serde(with = "datetime_serde")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl EncryptionBenchmark {
    /// Creates a new encryption benchmark with the specified performance
    /// metrics.
    ///
    /// The benchmark captures a snapshot of encryption performance at the
    /// current time, providing comprehensive metrics for algorithm
    /// comparison and optimization decisions.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The encryption algorithm that was benchmarked
    /// * `throughput_mbps` - Data processing rate in megabytes per second
    /// * `latency` - Operation latency from start to completion
    /// * `memory_usage_mb` - Peak memory consumption during encryption
    /// * `cpu_usage_percent` - CPU utilization percentage during operation
    /// * `file_size_mb` - Size of the file being encrypted (for context)
    ///
    /// # Returns
    ///
    /// A new `EncryptionBenchmark` instance with the current UTC timestamp.
    ///
    /// # Examples
    ///
    ///
    /// # Performance Metrics Guidelines
    ///
    /// - **Throughput**: Measure sustained data processing rate, not peak burst
    /// - **Latency**: Include full operation time from API call to completion
    /// - **Memory**: Capture peak usage, not average or final usage
    /// - **CPU**: Measure during active encryption, not including I/O wait
    /// - **File Size**: Provide context for performance scaling analysis
    pub fn new(
        algorithm: EncryptionAlgorithm,
        throughput_mbps: f64,
        latency: Duration,
        memory_usage_mb: f64,
        cpu_usage_percent: f64,
        file_size_mb: f64,
    ) -> Self {
        Self {
            algorithm,
            throughput_mbps,
            latency,
            memory_usage_mb,
            cpu_usage_percent,
            file_size_mb,
            timestamp: chrono::Utc::now(),
        }
    }
}
