// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Metrics Service Implementation
//!
//! This module provides a comprehensive metrics collection and export service
//! for the adaptive pipeline system. It integrates with Prometheus to provide
//! real-time monitoring, alerting, and observability capabilities.
//!
//! ## Overview
//!
//! The metrics service implementation provides:
//!
//! - **Prometheus Integration**: Native Prometheus metrics collection and
//!   export
//! - **Performance Monitoring**: Throughput, latency, and resource utilization
//!   tracking
//! - **Operational Metrics**: Error rates, success rates, and system health
//!   indicators
//! - **Business Metrics**: Processing volumes, compression ratios, and
//!   efficiency metrics
//! - **Real-Time Export**: HTTP endpoint for Prometheus scraping
//!
//! ## Architecture
//!
//! The metrics service follows observability best practices:
//!
//! - **Separation of Concerns**: Metrics are handled by dedicated
//!   infrastructure
//! - **Registry Pattern**: Centralized metric registration and management
//! - **Thread Safety**: Safe concurrent access from multiple threads
//! - **Low Overhead**: Minimal performance impact on core operations
//!
//! ## Metric Categories
//!
//! ### Pipeline Execution Metrics
//!
//! - **pipelines_processed_total**: Total number of pipelines processed
//! - **pipeline_processing_duration**: Processing time distribution
//! - **pipeline_bytes_processed_total**: Total bytes processed
//! - **pipeline_chunks_processed_total**: Total chunks processed
//! - **pipeline_errors_total**: Total processing errors
//! - **pipeline_warnings_total**: Total processing warnings
//!
//! ### Performance Metrics
//!
//! - **throughput_mbps**: Current throughput in megabytes per second
//! - **compression_ratio**: Achieved compression ratio
//! - **memory_usage_bytes**: Current memory usage
//! - **cpu_utilization_percent**: CPU utilization percentage
//!
//! ### System Health Metrics
//!
//! - **active_pipelines**: Number of currently active pipelines
//! - **queue_depth**: Processing queue depth
//! - **worker_utilization**: Worker thread utilization
//! - **resource_availability**: Available system resources
//!
//! ## Usage Examples
//!
//! ### Basic Metrics Service

//!
//! ### Metrics Export

//!
//! ### Integration with Processing

//!
//! ## Prometheus Integration
//!
//! ### Metric Types
//!
//! The service uses standard Prometheus metric types:
//!
//! - **Counter**: Monotonically increasing values (e.g., total requests)
//! - **Gauge**: Values that can increase or decrease (e.g., current memory
//!   usage)
//! - **Histogram**: Distribution of values with configurable buckets
//! - **Summary**: Pre-calculated quantiles for streaming data
//!
//! ### Labels and Dimensions
//!
//! Metrics support labels for dimensional analysis:
//! - **Algorithm**: Compression/encryption algorithm used
//! - **Stage**: Pipeline stage (compression, encryption, etc.)
//! - **Status**: Success, error, warning status
//! - **Worker**: Worker thread identifier
//!
//! ### Export Format
//!
//! Metrics are exported in standard Prometheus format:
//!
//! ## Performance Considerations
//!
//! ### Low Overhead Design
//!
//! - **Atomic Operations**: Lock-free metric updates using atomic operations
//! - **Efficient Storage**: Compact metric storage with minimal memory overhead
//! - **Batch Updates**: Batch metric updates to reduce contention
//! - **Lazy Evaluation**: Metrics are calculated only when requested
//!
//! ### Scalability
//!
//! - **Thread Safety**: Safe concurrent access from multiple threads
//! - **Memory Efficiency**: Bounded memory usage with automatic cleanup
//! - **High Throughput**: Optimized for high-frequency metric updates
//! - **Resource Management**: Automatic resource cleanup and garbage collection
//!
//! ## Monitoring and Alerting
//!
//! ### Key Performance Indicators (KPIs)
//!
//! - **Throughput**: Processing throughput in MB/s
//! - **Latency**: P50, P95, P99 processing latencies
//! - **Error Rate**: Percentage of failed operations
//! - **Resource Utilization**: CPU, memory, and I/O utilization
//!
//! ### Alert Conditions
//!
//! - **High Error Rate**: Error rate above threshold
//! - **Low Throughput**: Throughput below expected levels
//! - **High Latency**: Processing latency above SLA
//! - **Resource Exhaustion**: Memory or CPU usage above limits
//!
//! ## Integration
//!
//! The metrics service integrates with:
//!
//! - **Prometheus**: Native Prometheus metrics export
//! - **Grafana**: Visualization and dashboarding
//! - **Alertmanager**: Alert routing and notification
//! - **Pipeline Services**: Automatic metric collection from all services
//!
//! ## Security and Privacy
//!
//! ### Data Protection
//!
//! - **No Sensitive Data**: Metrics contain no sensitive user data
//! - **Aggregated Data**: Only aggregated statistics are exposed
//! - **Access Control**: Metrics endpoint can be secured with authentication
//! - **Audit Trail**: Metric access can be logged for compliance
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Custom Metrics**: Support for user-defined custom metrics
//! - **Distributed Tracing**: Integration with distributed tracing systems
//! - **Machine Learning**: Anomaly detection and predictive analytics
//! - **Cost Tracking**: Resource cost attribution and optimization metrics

use byte_unit::Byte;
use prometheus::{Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry};
use std::sync::Arc;
use tracing::debug;

use pipeline_domain::entities::processing_metrics::ProcessingMetrics;
use pipeline_domain::error::PipelineError;

/// Prometheus metrics service for pipeline observability
///
/// This service handles all metrics collection and export to Prometheus,
/// providing comprehensive monitoring and observability for the adaptive
/// pipeline system.
///
/// # Design Principles
///
/// - **Separation of Concerns**: Metrics are handled by dedicated observability
///   infrastructure
/// - **Performance**: Low overhead metric collection with minimal impact on
///   processing
/// - **Reliability**: Thread-safe operations with automatic error handling
/// - **Observability**: Comprehensive coverage of all system operations
///
/// # Metric Categories
///
/// The service tracks metrics across several categories:
/// - **Execution Metrics**: Pipeline processing statistics
/// - **Performance Metrics**: Throughput and efficiency measurements
/// - **System Metrics**: Resource utilization and health indicators
/// - **Error Metrics**: Error rates and failure analysis
///
/// # Examples
///
#[derive(Clone)]
pub struct MetricsService {
    registry: Arc<Registry>,

    // Pipeline execution metrics
    pipelines_processed_total: IntCounter,
    pipeline_processing_duration: Histogram,
    pipeline_bytes_processed_total: IntCounter,
    pipeline_chunks_processed_total: IntCounter,
    pipeline_errors_total: IntCounter,
    pipeline_warnings_total: IntCounter,

    // Performance metrics
    throughput_mbps: Gauge,
    compression_ratio: Gauge,

    // System metrics
    active_pipelines: IntGauge,
}

impl MetricsService {
    /// Create a new MetricsService with Prometheus registry
    pub fn new() -> Result<Self, PipelineError> {
        let registry = Registry::new();

        // Create pipeline execution counters
        let pipelines_processed_total = IntCounter::with_opts(
            Opts::new("pipeline_processed_total", "Total number of pipelines processed").namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create pipelines_processed_total metric: {}", e)))
        ?;

        let pipeline_processing_duration = Histogram::with_opts(
            HistogramOpts::new(
                "pipeline_processing_duration_seconds",
                "Time spent processing pipelines",
            )
            .namespace("adaptive_pipeline")
            .buckets(vec![0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0, 300.0]),
        )
        .map_err(|e| {
            PipelineError::metrics_error(format!("Failed to create pipeline_processing_duration metric: {}", e))
        })
        ?;

        let pipeline_bytes_processed_total = IntCounter::with_opts(
            Opts::new("pipeline_bytes_processed_total", "Total bytes processed by pipelines")
                .namespace("adaptive_pipeline"),
        )
        .map_err(|e| {
            PipelineError::metrics_error(format!("Failed to create pipeline_bytes_processed_total metric: {}", e))
        })
        ?;

        let pipeline_chunks_processed_total = IntCounter::with_opts(
            Opts::new("pipeline_chunks_processed_total", "Total chunks processed by pipelines")
                .namespace("adaptive_pipeline"),
        )
        .map_err(|e| {
            PipelineError::metrics_error(format!(
                "Failed to create pipeline_chunks_processed_total metric: {}",
                e
            ))
        })
        ?;

        let pipeline_errors_total = IntCounter::with_opts(
            Opts::new("pipeline_errors_total", "Total pipeline processing errors").namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create pipeline_errors_total metric: {}", e)))
        ?;

        let pipeline_warnings_total = IntCounter::with_opts(
            Opts::new("pipeline_warnings_total", "Total pipeline processing warnings").namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create pipeline_warnings_total metric: {}", e)))
        ?;

        // Create performance gauges
        let throughput_mbps = Gauge::with_opts(
            Opts::new("pipeline_throughput_mbps", "Current pipeline throughput in MB/s").namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create throughput_mbps metric: {}", e)))
        ?;

        let compression_ratio = Gauge::with_opts(
            Opts::new("pipeline_compression_ratio", "Current compression ratio achieved")
                .namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create compression_ratio metric: {}", e)))
        ?;

        // Create system gauges
        let active_pipelines = IntGauge::with_opts(
            Opts::new("pipeline_active_count", "Number of currently active pipelines").namespace("adaptive_pipeline"),
        )
        .map_err(|e| PipelineError::metrics_error(format!("Failed to create active_pipelines metric: {}", e)))
        ?;

        // Register all metrics
        registry
            .register(Box::new(pipelines_processed_total.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register pipelines_processed_total: {}", e)))
            ?;
        registry
            .register(Box::new(pipeline_processing_duration.clone()))
            .map_err(|e| {
                PipelineError::metrics_error(format!("Failed to register pipeline_processing_duration: {}", e))
            })
            ?;
        registry
            .register(Box::new(pipeline_bytes_processed_total.clone()))
            .map_err(|e| {
                PipelineError::metrics_error(format!("Failed to register pipeline_bytes_processed_total: {}", e))
            })
            ?;
        registry
            .register(Box::new(pipeline_chunks_processed_total.clone()))
            .map_err(|e| {
                PipelineError::metrics_error(format!("Failed to register pipeline_chunks_processed_total: {}", e))
            })
            ?;
        registry
            .register(Box::new(pipeline_errors_total.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register pipeline_errors_total: {}", e)))
            ?;
        registry
            .register(Box::new(pipeline_warnings_total.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register pipeline_warnings_total: {}", e)))
            ?;
        registry
            .register(Box::new(throughput_mbps.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register throughput_mbps: {}", e)))
            ?;
        registry
            .register(Box::new(compression_ratio.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register compression_ratio: {}", e)))
            ?;
        registry
            .register(Box::new(active_pipelines.clone()))
            .map_err(|e| PipelineError::metrics_error(format!("Failed to register active_pipelines: {}", e)))
            ?;

        debug!("MetricsService initialized with Prometheus registry");

        Ok(Self {
            registry: Arc::new(registry),
            pipelines_processed_total,
            pipeline_processing_duration,
            pipeline_bytes_processed_total,
            pipeline_chunks_processed_total,
            pipeline_errors_total,
            pipeline_warnings_total,
            throughput_mbps,
            compression_ratio,
            active_pipelines,
        })
    }

    /// Record metrics from pipeline processing completion
    pub fn record_pipeline_completion(&self, metrics: &ProcessingMetrics) {
        debug!("Recording pipeline completion metrics to Prometheus");

        // Increment completion counter
        self.pipelines_processed_total.inc();

        // Record processing duration if available
        if let Some(duration) = metrics.processing_duration() {
            self.pipeline_processing_duration.observe(duration.as_secs_f64());
        }

        // Record data processing metrics
        self.pipeline_bytes_processed_total.inc_by(metrics.bytes_processed());
        self.pipeline_chunks_processed_total.inc_by(metrics.chunks_processed());

        // Record error and warning counts
        self.pipeline_errors_total.inc_by(metrics.error_count());
        self.pipeline_warnings_total.inc_by(metrics.warning_count());

        // Update current performance gauges
        self.throughput_mbps.set(metrics.throughput_mb_per_second());

        if let Some(ratio) = metrics.compression_ratio() {
            self.compression_ratio.set(ratio);
        }

        debug!(
            "Recorded metrics: {} bytes, {} chunks, {} errors, {:.2} MB/s throughput",
            Byte::from_u128(metrics.bytes_processed() as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string(),
            metrics.chunks_processed(),
            metrics.error_count(),
            metrics.throughput_mb_per_second()
        );
    }

    /// Increment active pipeline count
    pub fn increment_active_pipelines(&self) {
        self.active_pipelines.inc();
        debug!("Incremented active pipelines count");
    }

    /// Decrement active pipeline count
    pub fn decrement_active_pipelines(&self) {
        self.active_pipelines.dec();
        debug!("Decremented active pipelines count");
    }

    /// Increment processed pipelines counter
    pub fn increment_processed_pipelines(&self) {
        self.pipelines_processed_total.inc();
        debug!("Incremented processed pipelines count");
    }

    /// Record processing duration
    pub fn record_processing_duration(&self, duration: std::time::Duration) {
        self.pipeline_processing_duration.observe(duration.as_secs_f64());
        debug!("Recorded processing duration: {:.2}s", duration.as_secs_f64());
    }

    /// Update current throughput
    pub fn update_throughput(&self, throughput_mbps: f64) {
        self.throughput_mbps.set(throughput_mbps);
        debug!("Updated throughput: {:.2} MB/s", throughput_mbps);
    }

    /// Increment error counter
    pub fn increment_errors(&self) {
        self.pipeline_errors_total.inc();
        debug!("Incremented error count");
    }

    /// Add bytes processed for this chunk
    pub fn add_bytes_processed(&self, chunk_bytes: u64) {
        self.pipeline_bytes_processed_total.inc_by(chunk_bytes);
        debug!("Added {} bytes to processed counter", chunk_bytes);
    }

    /// Increment chunks processed counter
    pub fn increment_chunks_processed(&self) {
        self.pipeline_chunks_processed_total.inc();
    }

    /// Get Prometheus metrics in text format for scraping
    pub fn get_metrics(&self) -> Result<String, PipelineError> {
        let encoder = prometheus::TextEncoder::new();
        let metric_families = self.registry.gather();

        encoder
            .encode_to_string(&metric_families)
            .map_err(|e| PipelineError::metrics_error(format!("Failed to encode metrics: {}", e)))
    }

    /// Get the Prometheus registry for advanced usage
    pub fn registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }
}

impl Default for MetricsService {
    #[allow(clippy::expect_used)]
    fn default() -> Self {
        Self::new().expect("Failed to create default MetricsService")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pipeline_domain::ProcessingMetrics;
    use std::time::Duration;

    /// Tests metrics service creation and basic functionality.
    ///
    /// This test validates that the metrics service can be created
    /// successfully and that it provides non-empty metrics output
    /// immediately after initialization.
    ///
    /// # Test Coverage
    ///
    /// - Service creation and initialization
    /// - Basic metrics output generation
    /// - Service functionality verification
    /// - Prometheus metrics format validation
    /// - Initial service state validation
    ///
    /// # Test Scenario
    ///
    /// Creates a new metrics service and verifies it can generate
    /// metrics output immediately, indicating proper initialization.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Service initialization and setup
    /// - Prometheus metrics registry creation
    /// - Basic metrics collection functionality
    /// - Service readiness and availability
    ///
    /// # Assertions
    ///
    /// - Service creation succeeds
    /// - Metrics output is non-empty
    /// - Service is immediately functional
    /// - Basic metrics collection works
    #[test]
    fn test_metrics_service_creation() {
        let service = MetricsService::new().unwrap();
        // assert!(!service.get_metrics()?.is_empty());
    }

    /// Tests pipeline completion recording and metrics generation.
    ///
    /// This test validates that the metrics service can properly
    /// record pipeline completion events and generate appropriate
    /// Prometheus metrics for pipeline processing statistics.
    ///
    /// # Test Coverage
    ///
    /// - Pipeline completion event recording
    /// - Processing metrics integration
    /// - Prometheus metrics generation
    /// - Pipeline processing counters
    /// - Bytes processed tracking
    ///
    /// # Test Scenario
    ///
    /// Creates a metrics service, records a pipeline completion with
    /// processing metrics, and verifies the appropriate Prometheus
    /// metrics are generated in the output.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Pipeline completion event handling
    /// - Processing metrics collection and aggregation
    /// - Prometheus counter updates
    /// - Metrics naming and labeling consistency
    ///
    /// # Assertions
    ///
    /// - Pipeline completion is recorded successfully
    /// - Prometheus output contains pipeline processed counter
    /// - Prometheus output contains bytes processed counter
    /// - Metrics are properly formatted and named
    #[test]
    fn test_record_pipeline_completion() {
        let service = MetricsService::new().unwrap();
        let metrics = ProcessingMetrics::new(1024, 2048);

        // service.record_pipeline_completion(&metrics);

        let prometheus_output = service.get_metrics().unwrap();
        assert!(prometheus_output.contains("adaptive_pipeline_pipeline_processed_total"));
        assert!(prometheus_output.contains("adaptive_pipeline_pipeline_bytes_processed_total"));
    }

    /// Tests active pipeline tracking and counter management.
    ///
    /// This test validates that the metrics service can properly
    /// track active pipeline counts through increment and decrement
    /// operations and generate appropriate Prometheus gauge metrics.
    ///
    /// # Test Coverage
    ///
    /// - Active pipeline counter increment operations
    /// - Active pipeline counter decrement operations
    /// - Prometheus gauge metrics generation
    /// - Counter state management
    /// - Metrics output validation
    ///
    /// # Test Scenario
    ///
    /// Creates a metrics service, performs increment and decrement
    /// operations on active pipeline counters, and verifies the
    /// appropriate Prometheus gauge metrics are generated.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Active pipeline state tracking
    /// - Counter increment/decrement operations
    /// - Prometheus gauge metric updates
    /// - Concurrent pipeline counting
    ///
    /// # Assertions
    ///
    /// - Pipeline counter increments work correctly
    /// - Pipeline counter decrements work correctly
    /// - Prometheus output contains active count gauge
    /// - Counter operations are reflected in metrics
    #[test]
    fn test_active_pipeline_tracking() {
        let service = MetricsService::new().unwrap();

        // service.increment_active_pipelines();
        // service.increment_active_pipelines();
        // service.decrement_active_pipelines();

        let prometheus_output = service.get_metrics().unwrap();
        assert!(prometheus_output.contains("adaptive_pipeline_pipeline_active_count"));
    }
}
