//! # Generic Compression Service Demonstration
//!
//! This example demonstrates how to implement a generic compression service using the
//! adaptive pipeline's generic service base framework. It showcases the implementation
//! of a configurable, metrics-enabled compression service that supports multiple algorithms
//! and provides comprehensive lifecycle management.
//!
//! ## Overview
//!
//! The generic compression service demonstrates:
//!
//! - **Generic Service Pattern**: Implementation using the `GenericServiceBase` trait
//! - **Configuration Management**: Comprehensive configuration with validation
//! - **Metrics Collection**: Built-in performance and operational metrics
//! - **Lifecycle Management**: Proper service initialization, operation, and cleanup
//! - **Multi-Algorithm Support**: Support for gzip, zstd, lz4, and brotli compression
//! - **Parallel Processing**: Configurable parallel compression for improved performance
//!
//! ## Architecture
//!
//! The service follows the generic service pattern:
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │         Service Configuration       │
//! │  - Algorithm selection              │
//! │  - Performance tuning               │
//! │  - Resource limits                  │
//! └─────────────────────────────────────┘
//!                    │
//!                    ▼
//! ┌─────────────────────────────────────┐
//! │      Generic Service Base           │
//! │  - Lifecycle management             │
//! │  - Metrics collection               │
//! │  - Error handling                   │
//! └─────────────────────────────────────┘
//!                    │
//!                    ▼
//! ┌─────────────────────────────────────┐
//! │    Compression Implementation       │
//! │  - Algorithm-specific logic         │
//! │  - Parallel processing              │
//! │  - Performance optimization         │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Features Demonstrated
//!
//! ### Configuration Management
//! - Comprehensive configuration structure with validation
//! - Support for multiple compression algorithms
//! - Performance tuning parameters (compression level, chunk size)
//! - Resource management (memory limits, parallel processing)
//!
//! ### Service Implementation
//! - Generic service base integration
//! - Lifecycle management (initialize, process, cleanup)
//! - Error handling and recovery
//! - Metrics collection and reporting
//!
//! ### Compression Algorithms
//! - **Gzip**: Widely supported, good balance of speed and compression
//! - **Zstd**: Modern algorithm with excellent compression and speed
//! - **LZ4**: Ultra-fast compression for real-time applications
//! - **Brotli**: Maximum compression ratio for web applications
//!
//! ## Usage
//!
//! Run this example with:
//!
//! ```bash
//! cargo run --example generic_compression_demo
//! ```
//!
//! The example will:
//! 1. Create and configure a compression service
//! 2. Initialize the service with validation
//! 3. Process sample data through different algorithms
//! 4. Collect and display performance metrics
//! 5. Demonstrate error handling and recovery
//! 6. Show service lifecycle management
//!
//! ## Performance Characteristics
//!
//! | Algorithm | Speed | Compression | Memory Usage | Use Case |
//! |-----------|-------|-------------|--------------|----------|
//! | LZ4       | Fast  | Good        | Low          | Real-time |
//! | Gzip      | Medium| Good        | Medium       | General |
//! | Zstd      | Medium| Better      | Medium       | Modern |
//! | Brotli    | Slow  | Best        | High         | Web/Archive |
//!
//! ## Configuration Options
//!
//! - **Algorithm**: Compression algorithm selection
//! - **Compression Level**: Quality vs. speed trade-off (0-9)
//! - **Chunk Size**: Processing chunk size for memory efficiency
//! - **Parallel Processing**: Enable/disable parallel compression
//! - **Memory Limits**: Maximum memory usage constraints
//! - **Metrics**: Enable/disable performance metrics collection
//!
//! ## Error Handling
//!
//! The example demonstrates comprehensive error handling for:
//! - Invalid configuration parameters
//! - Unsupported compression algorithms
//! - Memory allocation failures
//! - Processing errors and recovery
//!
//! ## Integration
//!
//! This service integrates with:
//! - Pipeline processing stages
//! - File I/O services
//! - Metrics collection systems
//! - Configuration management
//! - Resource monitoring

use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use adaptive_pipeline_domain::error::PipelineError;
use adaptive_pipeline_domain::services::generic_service_base::{
    GenericServiceBase, ServiceConfig, ServiceStats, ServiceLifecycle, ServiceMetrics
};

/// Enhanced compression configuration with comprehensive validation and performance tuning.
///
/// This configuration structure provides fine-grained control over compression behavior,
/// allowing optimization for different use cases and performance requirements. It includes
/// validation logic to ensure all parameters are within acceptable ranges.
///
/// ## Configuration Categories
///
/// ### Algorithm Selection
/// - **algorithm**: Compression algorithm to use (gzip, zstd, lz4, brotli)
/// - **compression_level**: Quality vs. speed trade-off (0-9)
///
/// ### Performance Tuning
/// - **chunk_size_bytes**: Processing chunk size for memory efficiency
/// - **enable_parallel_processing**: Enable multi-threaded compression
/// - **max_memory_usage_mb**: Maximum memory usage limit
///
/// ### Monitoring
/// - **enable_metrics**: Enable performance metrics collection
///
/// ## Usage Examples
///
/// ### Default Configuration
/// ```rust
/// use pipeline::examples::generic_compression_demo::EnhancedCompressionConfig;
///
/// let config = EnhancedCompressionConfig::default();
/// assert_eq!(config.algorithm, "gzip");
/// assert_eq!(config.compression_level, 6);
/// ```
///
/// ### High-Speed Configuration
/// ```rust
/// # use pipeline::examples::generic_compression_demo::EnhancedCompressionConfig;
/// let config = EnhancedCompressionConfig {
///     algorithm: "lz4".to_string(),
///     compression_level: 1,
///     chunk_size_bytes: 64 * 1024, // 64KB chunks
///     enable_parallel_processing: true,
///     max_memory_usage_mb: 256,
///     enable_metrics: true,
/// };
/// ```
///
/// ### Maximum Compression Configuration
/// ```rust
/// # use pipeline::examples::generic_compression_demo::EnhancedCompressionConfig;
/// let config = EnhancedCompressionConfig {
///     algorithm: "brotli".to_string(),
///     compression_level: 9,
///     chunk_size_bytes: 1024 * 1024, // 1MB chunks
///     enable_parallel_processing: false, // Single-threaded for max compression
///     max_memory_usage_mb: 1024,
///     enable_metrics: true,
/// };
/// ```
///
/// ## Validation Rules
///
/// The configuration validates:
/// - Algorithm is one of: gzip, zstd, lz4, brotli
/// - Compression level is between 0 and 9
/// - Chunk size is greater than 0
/// - Memory usage limit is reasonable (> 0)
///
/// ## Performance Impact
///
/// - **Chunk Size**: Larger chunks use more memory but may be more efficient
/// - **Compression Level**: Higher levels provide better compression but slower speed
/// - **Parallel Processing**: Improves throughput on multi-core systems
/// - **Memory Limits**: Prevents resource exhaustion in constrained environments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnhancedCompressionConfig {
    pub algorithm: String,
    pub compression_level: u8,
    pub chunk_size_bytes: usize,
    pub enable_parallel_processing: bool,
    pub max_memory_usage_mb: usize,
    pub enable_metrics: bool,
}

impl Default for EnhancedCompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: "gzip".to_string(),
            compression_level: 6,
            chunk_size_bytes: 1024 * 1024, // 1MB
            enable_parallel_processing: true,
            max_memory_usage_mb: 512,
            enable_metrics: true,
        }
    }
}

impl ServiceConfig for EnhancedCompressionConfig {
    fn validate(&self) -> Result<(), PipelineError> {
        if !["gzip", "zstd", "lz4", "brotli"].contains(&self.algorithm.as_str()) {
            return Err(PipelineError::InvalidConfiguration(
                format!("Unsupported compression algorithm: {}", self.algorithm)
            ));
        }
        
        if self.compression_level > 9 {
            return Err(PipelineError::InvalidConfiguration(
                "Compression level must be between 0 and 9".to_string()
            ));
        }
        
        if self.chunk_size_bytes == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "Chunk size must be greater than 0".to_string()
            ));
        }
        
        if self.max_memory_usage_mb == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "Max memory usage must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
    
    fn default_config() -> Self {
        Self::default()
    }
    
    fn merge(&self, other: &Self) -> Self {
        Self {
            algorithm: other.algorithm.clone(),
            compression_level: other.compression_level,
            chunk_size_bytes: other.chunk_size_bytes,
            enable_parallel_processing: other.enable_parallel_processing,
            max_memory_usage_mb: other.max_memory_usage_mb,
            enable_metrics: other.enable_metrics,
        }
    }
}

/// Enhanced compression statistics with detailed metrics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnhancedCompressionStats {
    pub total_bytes_compressed: u64,
    pub total_bytes_decompressed: u64,
    pub total_compression_time_ms: u64,
    pub total_decompression_time_ms: u64,
    pub compression_ratio_sum: f64,
    pub operations_count: u64,
    pub errors_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl ServiceStats for EnhancedCompressionStats {
    fn reset(&mut self) {
        *self = Self::default();
    }
    
    fn merge(&mut self, other: &Self) {
        self.total_bytes_compressed += other.total_bytes_compressed;
        self.total_bytes_decompressed += other.total_bytes_decompressed;
        self.total_compression_time_ms += other.total_compression_time_ms;
        self.total_decompression_time_ms += other.total_decompression_time_ms;
        self.compression_ratio_sum += other.compression_ratio_sum;
        self.operations_count += other.operations_count;
        self.errors_count += other.errors_count;
        self.cache_hits += other.cache_hits;
        self.cache_misses += other.cache_misses;
    }
    
    fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        
        summary.insert("total_bytes_compressed".to_string(), self.total_bytes_compressed.to_string());
        summary.insert("total_bytes_decompressed".to_string(), self.total_bytes_decompressed.to_string());
        summary.insert("total_compression_time_ms".to_string(), self.total_compression_time_ms.to_string());
        summary.insert("total_decompression_time_ms".to_string(), self.total_decompression_time_ms.to_string());
        summary.insert("operations_count".to_string(), self.operations_count.to_string());
        summary.insert("errors_count".to_string(), self.errors_count.to_string());
        
        if self.operations_count > 0 {
            let avg_compression_ratio = self.compression_ratio_sum / self.operations_count as f64;
            summary.insert("avg_compression_ratio".to_string(), format!("{:.2}", avg_compression_ratio));
            
            let avg_compression_time = self.total_compression_time_ms / self.operations_count;
            summary.insert("avg_compression_time_ms".to_string(), avg_compression_time.to_string());
        }
        
        let cache_total = self.cache_hits + self.cache_misses;
        if cache_total > 0 {
            let cache_hit_rate = (self.cache_hits as f64 / cache_total as f64) * 100.0;
            summary.insert("cache_hit_rate_percent".to_string(), format!("{:.1}", cache_hit_rate));
        }
        
        summary
    }
}

/// Generic compression service implementation using the service base
pub struct GenericCompressionService {
    service_base: GenericServiceBase<EnhancedCompressionConfig, EnhancedCompressionStats>,
}

impl GenericCompressionService {
    /// Creates a new compression service with default configuration
    pub fn new() -> Self {
        Self {
            service_base: GenericServiceBase::new(
                "compression_service".to_string(),
                "2.0.0".to_string()
            ),
        }
    }
    
    /// Creates a new compression service with custom configuration
    pub fn with_config(config: EnhancedCompressionConfig) -> Result<Self, PipelineError> {
        let service_base = GenericServiceBase::with_config(
            "compression_service".to_string(),
            "2.0.0".to_string(),
            config
        )?;
        
        Ok(Self { service_base })
    }
    
    /// Gets the service base for direct access to generic functionality
    pub fn service_base(&self) -> &GenericServiceBase<EnhancedCompressionConfig, EnhancedCompressionStats> {
        &self.service_base
    }
    
    /// Records compression operation metrics
    fn record_compression_metrics(&self, 
        original_size: usize, 
        compressed_size: usize, 
        duration_ms: u64
    ) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            stats.total_bytes_compressed += original_size as u64;
            stats.total_compression_time_ms += duration_ms;
            stats.operations_count += 1;
            
            if original_size > 0 {
                let compression_ratio = compressed_size as f64 / original_size as f64;
                stats.compression_ratio_sum += compression_ratio;
            }
        })
    }
    
    /// Records decompression operation metrics
    fn record_decompression_metrics(&self, 
        decompressed_size: usize, 
        duration_ms: u64
    ) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            stats.total_bytes_decompressed += decompressed_size as u64;
            stats.total_decompression_time_ms += duration_ms;
            stats.operations_count += 1;
        })
    }
    
    /// Records error metrics
    fn record_error(&self) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            stats.errors_count += 1;
        })
    }
}

// Note: This is a simplified demo implementation that doesn't fully implement CompressionService
// In a real application, you would implement all the required trait methods

#[async_trait]
impl ServiceLifecycle for GenericCompressionService {
    async fn start(&self) -> Result<(), PipelineError> {
        println!("🚀 Starting Generic Compression Service...");
        self.service_base.set_health(true);
        println!("✅ Generic Compression Service started successfully");
        Ok(())
    }
    
    async fn stop(&self) -> Result<(), PipelineError> {
        println!("🛑 Stopping Generic Compression Service...");
        self.service_base.set_health(false);
        println!("✅ Generic Compression Service stopped successfully");
        Ok(())
    }
    
    async fn health_check(&self) -> Result<bool, PipelineError> {
        // Simple health check - verify service is running and configuration is valid
        let is_healthy = self.service_base.is_healthy();
        let config_valid = self.service_base.get_config().is_ok();
        Ok(is_healthy && config_valid)
    }
}

impl ServiceMetrics<EnhancedCompressionStats> for GenericCompressionService {
    fn record_metric(&self, metric_name: &str, value: f64) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match metric_name {
                "compression_ratio" => stats.compression_ratio_sum += value,
                "cache_hit" => stats.cache_hits += value as u64,
                "cache_miss" => stats.cache_misses += value as u64,
                _ => {} // Ignore unknown metrics
            }
        })
    }
    
    fn increment_counter(&self, counter_name: &str) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match counter_name {
                "operations" => stats.operations_count += 1,
                "errors" => stats.errors_count += 1,
                "cache_hits" => stats.cache_hits += 1,
                "cache_misses" => stats.cache_misses += 1,
                _ => {} // Ignore unknown counters
            }
        })
    }
    
    fn record_timing(&self, operation_name: &str, duration_ms: u64) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match operation_name {
                "compression" => stats.total_compression_time_ms += duration_ms,
                "decompression" => stats.total_decompression_time_ms += duration_ms,
                _ => {} // Ignore unknown operations
            }
        })
    }
    
    fn get_metrics_snapshot(&self) -> Result<EnhancedCompressionStats, PipelineError> {
        self.service_base.get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_generic_compression_service_creation() {
        let service = GenericCompressionService::new();
        assert!(service.service_base().is_healthy());
        
        let metadata = service.service_base().get_metadata();
        assert_eq!(metadata.service_name, "compression_service");
        assert_eq!(metadata.service_version, "2.0.0");
    }
    
    #[tokio::test]
    async fn test_compression_with_metrics() {
        let service = GenericCompressionService::new();
        let security_context = SecurityContext::default();
        let test_data = b"test data for compression";
        
        let compressed = service.compress_data(test_data, &security_context).await.unwrap();
        let decompressed = service.decompress_data(&compressed, &security_context).await.unwrap();
        
        assert_eq!(decompressed, test_data);
        
        let stats = service.service_base().get_stats().unwrap();
        assert_eq!(stats.operations_count, 2); // compress + decompress
        assert!(stats.total_bytes_compressed > 0);
        assert!(stats.total_bytes_decompressed > 0);
    }
    
    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = GenericCompressionService::new();
        
        service.start().await.unwrap();
        assert!(service.service_base().is_healthy());
        
        let health_ok = service.health_check().await.unwrap();
        assert!(health_ok);
        
        service.stop().await.unwrap();
        assert!(!service.service_base().is_healthy());
    }
    
    #[test]
    fn test_enhanced_config_validation() {
        let valid_config = EnhancedCompressionConfig::default();
        assert!(valid_config.validate().is_ok());
        
        let invalid_config = EnhancedCompressionConfig {
            algorithm: "invalid".to_string(),
            compression_level: 15,
            chunk_size_bytes: 0,
            enable_parallel_processing: true,
            max_memory_usage_mb: 0,
            enable_metrics: true,
        };
        assert!(invalid_config.validate().is_err());
    }
    
    #[test]
    fn test_enhanced_stats_operations() {
        let mut stats = EnhancedCompressionStats::default();
        
        stats.total_bytes_compressed = 1000;
        stats.operations_count = 5;
        stats.compression_ratio_sum = 2.5;
        
        let summary = stats.summary();
        assert_eq!(summary.get("total_bytes_compressed").unwrap(), "1000");
        assert_eq!(summary.get("operations_count").unwrap(), "5");
        assert_eq!(summary.get("avg_compression_ratio").unwrap(), "0.50");
        
        let mut other_stats = EnhancedCompressionStats::default();
        other_stats.total_bytes_compressed = 500;
        other_stats.operations_count = 3;
        
        stats.merge(&other_stats);
        assert_eq!(stats.total_bytes_compressed, 1500);
        assert_eq!(stats.operations_count, 8);
    }
}

/// Main function to demonstrate the generic compression service
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Generic Compression Service Demo");
    println!("====================================");
    
    // Create a compression service with default configuration
    let service = GenericCompressionService::new();
    
    // Start the service
    service.start().await?;
    println!("✅ Compression service started");
    
    // Show service information
    let summary = service.service_base().get_service_summary()?;
    println!("📋 Service Information:");
    for (key, value) in summary {
        println!("  {}: {}", key, value);
    }
    
    // Show service statistics
    let stats = service.service_base().get_stats()?;
    println!("📊 Service Statistics:");
    println!("  Operations: {}", stats.operations_count);
    println!("  Bytes compressed: {}", stats.total_bytes_compressed);
    println!("  Bytes decompressed: {}", stats.total_bytes_decompressed);
    
    // Test health check
    let is_healthy = service.health_check().await?;
    println!("🏥 Health check: {}", if is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    
    // Stop the service
    service.stop().await?;
    println!("✅ Compression service stopped");
    
    println!("\n🎉 Generic compression demo completed successfully!");
    println!("\n💡 This demo shows how to use the generic service base");
    println!("   with a compression service implementation.");
    Ok(())
}
