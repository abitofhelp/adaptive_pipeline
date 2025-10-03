//! # Compression Service
//!
//! This module provides domain-level compression services for the adaptive
//! pipeline system. It defines the compression service interface and related
//! types for handling data compression and decompression operations within the
//! pipeline processing workflow.
//!
//! ## Overview
//!
//! The compression service provides:
//!
//! - **Algorithm Support**: Multiple compression algorithms (Brotli, Gzip,
//!   Zstd, Lz4)
//! - **Configurable Compression**: Adjustable compression levels and parameters
//! - **Streaming Processing**: Chunk-by-chunk compression for large files
//! - **Performance Optimization**: Algorithm selection based on data
//!   characteristics
//! - **Error Handling**: Comprehensive error reporting and recovery
//!
//! ## Architecture
//!
//! The compression service follows Domain-Driven Design principles:
//!
//! - **Domain Service**: `CompressionService` trait defines the contract
//! - **Configuration**: `CompressionConfig` encapsulates compression parameters
//! - **Algorithms**: `CompressionAlgorithm` enum provides type-safe algorithm
//!   selection
//! - **Levels**: `CompressionLevel` enum balances speed vs. compression ratio
//!
//! ## Usage Examples
//!
//! ### Basic Compression

//!
//! ### Algorithm Selection

//!
//! ## Performance Considerations
//!
//! ### Algorithm Characteristics
//!
//! | Algorithm | Speed | Ratio | Memory | Use Case |
//! |-----------|-------|-------|--------|-----------|
//! | Lz4       | Fast  | Good  | Low    | Real-time processing |
//! | Gzip      | Medium| Good  | Medium | General purpose |
//! | Zstd      | Medium| Better| Medium | Modern balanced choice |
//! | Brotli    | Slow  | Best  | High   | Maximum compression |
//!
//! ### Compression Levels
//!
//! - **Fastest**: Minimal compression, maximum speed
//! - **Fast**: Light compression, good speed
//! - **Balanced**: Optimal speed/ratio balance
//! - **Best**: Maximum compression, slower processing
//! - **Custom**: Fine-tuned level for specific requirements
//!
//! ## Error Handling
//!
//! The compression service handles various error conditions:
//!
//! - **Compression Failures**: Algorithm-specific errors
//! - **Memory Limitations**: Out-of-memory conditions
//! - **Data Corruption**: Invalid or corrupted input
//! - **Configuration Errors**: Invalid parameters or settings
//!
//! ## Thread Safety
//!
//! All compression service implementations are thread-safe and can be used
//! concurrently across multiple threads. The service maintains no mutable state
//! and all operations are stateless.
//!
//! ## Integration
//!
//! The compression service integrates with:
//!
//! - **Pipeline Processing**: Core pipeline stage processing
//! - **File Processor**: High-level file processing workflows
//! - **Metrics Collection**: Performance monitoring and statistics
//! - **Error Reporting**: Comprehensive error tracking and reporting

use crate::{FileChunk, PipelineError, ProcessingContext};

// NOTE: Domain traits are synchronous. Async execution is an infrastructure concern.
// Infrastructure can provide async adapters that wrap sync implementations.

/// Compression algorithms supported by the adaptive pipeline system
///
/// This enum provides type-safe selection of compression algorithms with
/// different performance characteristics and use cases. Each algorithm offers
/// different trade-offs between compression speed, compression ratio, and
/// memory usage.
///
/// # Algorithm Characteristics
///
/// - **Brotli**: Best compression ratio, slower processing, higher memory usage
/// - **Gzip**: Good balance of speed and compression, widely supported
/// - **Zstd**: Modern algorithm with excellent speed/ratio balance
/// - **Lz4**: Fastest compression, good for real-time processing
/// - **Custom**: User-defined algorithms for specialized requirements
///
/// # Examples
///
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionAlgorithm {
    Brotli,
    Gzip,
    Zstd,
    Lz4,
    Custom(String),
}

/// Compression level settings that balance processing speed vs. compression
/// ratio
///
/// This enum provides predefined compression levels optimized for different use
/// cases, allowing users to choose the appropriate trade-off between
/// compression speed and the resulting compression ratio.
///
/// # Level Characteristics
///
/// - **Fastest**: Minimal compression for maximum speed (level 1-2)
/// - **Fast**: Light compression with good speed (level 3-4)
/// - **Balanced**: Optimal balance of speed and compression (level 5-6)
/// - **Best**: Maximum compression ratio, slower processing (level 9-11)
/// - **Custom**: User-defined level for fine-tuned control
///
/// # Performance Impact
///
/// Higher compression levels generally result in:
/// - Better compression ratios (smaller output)
/// - Increased processing time
/// - Higher memory usage
/// - More CPU utilization
///
/// # Examples
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionLevel {
    /// Fastest compression with minimal ratio optimization
    /// Suitable for real-time processing and streaming applications
    Fastest,

    /// Fast compression with light optimization
    /// Good for interactive applications requiring quick response
    Fast,

    /// Balanced compression optimizing both speed and ratio
    /// Recommended for most general-purpose applications
    Balanced,

    /// Best compression ratio with slower processing
    /// Ideal for archival storage and bandwidth-limited scenarios
    Best,

    /// Custom compression level for fine-tuned control
    /// Value interpretation depends on the specific algorithm
    Custom(u32),
}

/// Compression configuration that encapsulates all parameters for compression
/// operations
///
/// This configuration struct provides comprehensive control over compression
/// behavior, allowing fine-tuning of compression parameters for optimal
/// performance in different scenarios. The configuration is immutable and
/// thread-safe.
///
/// # Configuration Parameters
///
/// - **Algorithm**: The compression algorithm to use
/// - **Level**: Compression level balancing speed vs. ratio
/// - **Dictionary**: Optional pre-trained dictionary for better compression
/// - **Window Size**: Sliding window size for compression algorithms
/// - **Parallel Processing**: Enable multi-threaded compression when supported
///
/// # Examples
///
///
/// # Performance Considerations
///
/// - **Dictionary**: Pre-trained dictionaries can significantly improve
///   compression ratios for similar data patterns but require additional memory
/// - **Window Size**: Larger windows generally improve compression but use more
///   memory
/// - **Parallel Processing**: Can improve throughput on multi-core systems but
///   may increase memory usage and complexity
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// The compression algorithm to use for processing
    pub algorithm: CompressionAlgorithm,

    /// Compression level balancing speed vs. compression ratio
    pub level: CompressionLevel,

    /// Optional pre-trained dictionary for improved compression of similar data
    pub dictionary: Option<Vec<u8>>,

    /// Optional sliding window size for compression algorithms
    /// (algorithm-specific)
    pub window_size: Option<u32>,

    /// Enable parallel processing for supported algorithms
    pub parallel_processing: bool,
}

/// Domain service interface for compression operations in the adaptive pipeline
/// system
///
/// This trait defines the contract for compression services that handle data
/// compression and decompression operations. Implementations provide
/// algorithm-specific compression logic while maintaining consistent interfaces
/// across different compression algorithms.
///
/// # Design Principles
///
/// - **Stateless Operations**: All methods are stateless and thread-safe
/// - **Chunk-Based Processing**: Operates on file chunks for streaming support
/// - **Configuration-Driven**: Behavior controlled through configuration
///   objects
/// - **Error Handling**: Comprehensive error reporting through `PipelineError`
/// - **Context Integration**: Integrates with processing context for state
///   management
///
/// # Implementation Requirements
///
/// Implementations must:
/// - Be thread-safe (`Send + Sync`)
/// - Handle all supported compression algorithms
/// - Provide consistent error handling
/// - Support streaming operations through chunk processing
/// - Maintain compression metadata and statistics
///
/// # Usage Examples
///
/// # Architecture Note
///
/// This trait is **synchronous** following DDD principles. The domain layer
/// defines *what* operations exist, not *how* they execute. Async execution
/// is an infrastructure concern. Infrastructure adapters can wrap this trait
/// to provide async interfaces when needed.
pub trait CompressionService: Send + Sync {
    /// Compresses a file chunk using the specified configuration
    ///
    /// This method compresses the data contained in a file chunk according to
    /// the provided compression configuration. The operation is stateless
    /// and can be called concurrently from multiple threads.
    ///
    /// # Parameters
    ///
    /// - `chunk`: The file chunk containing data to compress
    /// - `config`: Compression configuration specifying algorithm and
    ///   parameters
    /// - `context`: Processing context for state management and metadata
    ///
    /// # Returns
    ///
    /// Returns a new `FileChunk` containing the compressed data, or a
    /// `PipelineError` if compression fails.
    ///
    /// # Errors
    ///
    /// - `CompressionError`: Algorithm-specific compression failures
    /// - `ConfigurationError`: Invalid compression configuration
    /// - `MemoryError`: Insufficient memory for compression operation
    /// - `DataError`: Invalid or corrupted input data
    ///
    /// # Note on Async
    ///
    /// This method is synchronous in the domain. For async contexts,
    /// use `AsyncCompressionAdapter` from the infrastructure layer.
    fn compress_chunk(
        &self,
        chunk: FileChunk,
        config: &CompressionConfig,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Decompresses a file chunk using the specified configuration
    ///
    /// This method decompresses the data contained in a file chunk that was
    /// previously compressed using a compatible compression algorithm. The
    /// decompression parameters must match those used during compression.
    ///
    /// # Parameters
    ///
    /// - `chunk`: The file chunk containing compressed data to decompress
    /// - `config`: Compression configuration specifying algorithm and
    ///   parameters
    /// - `context`: Processing context for state management and metadata
    ///
    /// # Returns
    ///
    /// Returns a new `FileChunk` containing the decompressed data, or a
    /// `PipelineError` if decompression fails.
    ///
    /// # Errors
    ///
    /// - `DecompressionError`: Algorithm-specific decompression failures
    /// - `ConfigurationError`: Mismatched compression configuration
    /// - `MemoryError`: Insufficient memory for decompression operation
    /// - `DataCorruptionError`: Corrupted or invalid compressed data
    ///
    /// # Note on Async
    ///
    /// This method is synchronous in the domain. For async contexts,
    /// use `AsyncCompressionAdapter` from the infrastructure layer.
    fn decompress_chunk(
        &self,
        chunk: FileChunk,
        config: &CompressionConfig,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Estimates compression ratio for given data
    ///
    /// # Note
    ///
    /// Parallel processing of chunks is an infrastructure concern.
    /// Use infrastructure adapters for batch/parallel operations.
    fn estimate_compression_ratio(
        &self,
        data_sample: &[u8],
        algorithm: &CompressionAlgorithm,
    ) -> Result<f64, PipelineError>;

    /// Gets optimal compression configuration for file type
    ///
    /// Analyzes file characteristics and recommends configuration.
    fn get_optimal_config(
        &self,
        file_extension: &str,
        data_sample: &[u8],
        performance_priority: CompressionPriority,
    ) -> Result<CompressionConfig, PipelineError>;

    /// Validates compression configuration
    ///
    /// Checks if the configuration is valid and supported.
    fn validate_config(&self, config: &CompressionConfig) -> Result<(), PipelineError>;

    /// Gets supported algorithms
    ///
    /// Returns list of compression algorithms supported by this implementation.
    fn supported_algorithms(&self) -> Vec<CompressionAlgorithm>;

    /// Benchmarks compression performance
    ///
    /// Tests compression performance with sample data.
    fn benchmark_algorithm(
        &self,
        algorithm: &CompressionAlgorithm,
        test_data: &[u8],
    ) -> Result<CompressionBenchmark, PipelineError>;
}

/// Compression priority for optimization
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionPriority {
    Speed,
    Ratio,
    Balanced,
}

/// Compression benchmark results
#[derive(Debug, Clone)]
pub struct CompressionBenchmark {
    pub algorithm: CompressionAlgorithm,
    pub compression_ratio: f64,
    pub compression_speed_mbps: f64,
    pub decompression_speed_mbps: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

impl Default for CompressionBenchmark {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Brotli,
            compression_ratio: 0.5,
            compression_speed_mbps: 100.0,
            decompression_speed_mbps: 200.0,
            memory_usage_mb: 64.0,
            cpu_usage_percent: 50.0,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Brotli,
            level: CompressionLevel::Balanced,
            dictionary: None,
            window_size: None,
            parallel_processing: true,
        }
    }
}

impl CompressionConfig {
    /// Creates a new compression configuration
    pub fn new(algorithm: CompressionAlgorithm) -> Self {
        Self {
            algorithm,
            ..Default::default()
        }
    }

    /// Sets compression level
    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// Sets dictionary
    pub fn with_dictionary(mut self, dictionary: Vec<u8>) -> Self {
        self.dictionary = Some(dictionary);
        self
    }

    /// Sets window size
    pub fn with_window_size(mut self, size: u32) -> Self {
        self.window_size = Some(size);
        self
    }

    /// Sets parallel processing
    pub fn with_parallel_processing(mut self, enabled: bool) -> Self {
        self.parallel_processing = enabled;
        self
    }

    /// Creates a speed-optimized configuration
    pub fn for_speed(algorithm: CompressionAlgorithm) -> Self {
        Self {
            algorithm,
            level: CompressionLevel::Fastest,
            dictionary: None,
            window_size: None,
            parallel_processing: true,
        }
    }

    /// Creates a ratio-optimized configuration
    pub fn for_ratio(algorithm: CompressionAlgorithm) -> Self {
        Self {
            algorithm,
            level: CompressionLevel::Best,
            dictionary: None,
            window_size: None,
            parallel_processing: false, // Better compression with single thread
        }
    }
}

impl CompressionLevel {
    /// Gets the numeric level for the compression algorithm
    pub fn to_numeric(&self, algorithm: &CompressionAlgorithm) -> u32 {
        match (self, algorithm) {
            (CompressionLevel::Fastest, CompressionAlgorithm::Brotli) => 1,
            (CompressionLevel::Fast, CompressionAlgorithm::Brotli) => 3,
            (CompressionLevel::Balanced, CompressionAlgorithm::Brotli) => 6,
            (CompressionLevel::Best, CompressionAlgorithm::Brotli) => 11,

            (CompressionLevel::Fastest, CompressionAlgorithm::Gzip) => 1,
            (CompressionLevel::Fast, CompressionAlgorithm::Gzip) => 3,
            (CompressionLevel::Balanced, CompressionAlgorithm::Gzip) => 6,
            (CompressionLevel::Best, CompressionAlgorithm::Gzip) => 9,

            (CompressionLevel::Fastest, CompressionAlgorithm::Zstd) => 1,
            (CompressionLevel::Fast, CompressionAlgorithm::Zstd) => 3,
            (CompressionLevel::Balanced, CompressionAlgorithm::Zstd) => 9,
            (CompressionLevel::Best, CompressionAlgorithm::Zstd) => 19,

            (CompressionLevel::Custom(level), _) => *level,

            _ => 6, // Default balanced level
        }
    }
}

impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionAlgorithm::Brotli => write!(f, "Brotli"),
            CompressionAlgorithm::Gzip => write!(f, "Gzip"),
            CompressionAlgorithm::Zstd => write!(f, "Zstd"),
            CompressionAlgorithm::Lz4 => write!(f, "LZ4"),
            CompressionAlgorithm::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

impl std::fmt::Display for CompressionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionLevel::Fastest => write!(f, "Fastest"),
            CompressionLevel::Fast => write!(f, "Fast"),
            CompressionLevel::Balanced => write!(f, "Balanced"),
            CompressionLevel::Best => write!(f, "Best"),
            CompressionLevel::Custom(level) => write!(f, "Custom({})", level),
        }
    }
}
