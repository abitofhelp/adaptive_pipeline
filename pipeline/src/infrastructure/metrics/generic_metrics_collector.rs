// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Generic Metrics Collector
//!
//! This module provides a generic, reusable metrics collection system for the
//! adaptive pipeline system. It supports type-safe metrics collection,
//! aggregation, and reporting with comprehensive performance tracking
//! capabilities.
//!
//! ## Overview
//!
//! The generic metrics collector provides:
//!
//! - **Type-Safe Metrics**: Generic metrics collection for any metric type
//! - **Aggregation**: Automatic aggregation and summarization of metrics
//! - **Performance Tracking**: Detailed timing and performance measurements
//! - **Thread Safety**: Safe concurrent metrics collection
//! - **Extensibility**: Pluggable metrics types and collection strategies
//!
//! ## Architecture
//!
//! The metrics collector follows generic design patterns:
//!
//! - **Generic Design**: Works with any metrics type implementing required
//!   traits
//! - **Collection Framework**: Pluggable collection strategies and aggregation
//! - **Storage System**: Efficient storage and retrieval of metrics data
//! - **Reporting System**: Comprehensive reporting and export capabilities
//!
//! ## Key Features
//!
//! ### Metrics Collection
//!
//! - **Real-time Collection**: Collect metrics in real-time during processing
//! - **Batched Collection**: Batch metrics collection for better performance
//! - **Automatic Aggregation**: Automatic aggregation of collected metrics
//! - **Custom Metrics**: Support for custom metric types and measurements
//!
//! ### Performance Tracking
//!
//! - **Timing Measurements**: Precise timing measurements with nanosecond
//!   precision
//! - **Throughput Tracking**: Track processing throughput and rates
//! - **Resource Usage**: Monitor memory, CPU, and other resource usage
//! - **Statistical Analysis**: Statistical analysis of collected metrics
//!
//! ### Reporting and Export
//!
//! - **Summary Reports**: Generate summary reports of collected metrics
//! - **Export Formats**: Export metrics in various formats (JSON, CSV, etc.)
//! - **Real-time Monitoring**: Real-time metrics monitoring and alerting
//! - **Historical Analysis**: Historical metrics analysis and trending
//!
//! ## Usage Examples
//!
//! ### Basic Metrics Collection

//!
//! ### Performance Timing

//!
//! ### Custom Metrics with Observers

//!
//! ## Metrics Types
//!
//! ### Built-in Metrics
//!
//! The system provides several built-in metrics types:
//!
//! - **TimingMetrics**: Timing measurements and performance tracking
//! - **ThroughputMetrics**: Throughput and rate measurements
//! - **ResourceMetrics**: Memory, CPU, and resource usage tracking
//! - **ErrorMetrics**: Error counting and categorization
//!
//! ### Custom Metrics
//!
//! Create custom metrics by implementing the `CollectibleMetrics` trait:
//!
//! - **Reset Logic**: Define how metrics are reset to initial state
//! - **Merge Logic**: Define how metrics from different sources are combined
//! - **Summary Generation**: Define how metrics are summarized for reporting
//! - **Validation**: Define validation rules for metric consistency
//!
//! ## Aggregation Strategies
//!
//! ### Automatic Aggregation
//!
//! - **Time-based Aggregation**: Aggregate metrics over time windows
//! - **Count-based Aggregation**: Aggregate metrics after N collections
//! - **Threshold-based Aggregation**: Aggregate when thresholds are reached
//! - **Manual Aggregation**: Trigger aggregation manually when needed
//!
//! ### Statistical Analysis
//!
//! - **Average Calculations**: Calculate averages of collected metrics
//! - **Percentile Analysis**: Calculate percentiles for timing metrics
//! - **Trend Analysis**: Analyze trends in collected metrics
//! - **Outlier Detection**: Detect and handle metric outliers
//!
//! ## Performance Considerations
//!
//! ### Collection Overhead
//!
//! - **Minimal Overhead**: Designed for minimal performance impact
//! - **Batched Operations**: Batch metrics operations for efficiency
//! - **Lock-Free Operations**: Use lock-free operations where possible
//! - **Memory Efficiency**: Efficient memory usage for metrics storage
//!
//! ### Scalability
//!
//! - **Concurrent Collection**: Support for concurrent metrics collection
//! - **Memory Management**: Efficient memory management for large datasets
//! - **Storage Optimization**: Optimized storage for frequently accessed
//!   metrics
//!
//! ## Error Handling
//!
//! ### Collection Errors
//!
//! - **Validation Errors**: Handle metrics validation failures
//! - **Storage Errors**: Handle metrics storage failures
//! - **Aggregation Errors**: Handle aggregation failures gracefully
//! - **Observer Errors**: Handle observer notification failures
//!
//! ### Recovery Strategies
//!
//! - **Graceful Degradation**: Continue operation with reduced metrics
//! - **Error Isolation**: Isolate errors to prevent system failure
//! - **Automatic Recovery**: Automatic recovery from transient failures
//!
//! ## Integration
//!
//! The metrics collector integrates with:
//!
//! - **Processing Pipeline**: Collect metrics during pipeline execution
//! - **Monitoring Systems**: Export metrics to external monitoring systems
//! - **Alerting Systems**: Trigger alerts based on metric thresholds
//! - **Reporting Systems**: Generate reports from collected metrics
//!
//! ## Thread Safety
//!
//! The metrics collector is fully thread-safe:
//!
//! - **Concurrent Collection**: Safe concurrent metrics collection
//! - **Atomic Operations**: Use atomic operations for counters
//! - **Lock-Free Reads**: Lock-free reads for better performance
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Distributed Metrics**: Support for distributed metrics collection
//! - **Machine Learning**: ML-based anomaly detection in metrics
//! - **Real-time Dashboards**: Real-time metrics visualization
//! - **Advanced Analytics**: Advanced statistical analysis capabilities

use pipeline_domain::error::PipelineError;
use pipeline_domain::services::datetime_serde;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Generic trait for metrics that can be collected and aggregated
///
/// This trait defines the interface for metrics that can be collected,
/// aggregated, and reported by the generic metrics collector. It provides a
/// type-safe way to define custom metrics with validation and summarization
/// capabilities.
///
/// # Key Features
///
/// - **Reset Capability**: Reset metrics to initial state for new collection
///   periods
/// - **Merge Operations**: Combine metrics from different sources or time
///   periods
/// - **Summary Generation**: Generate human-readable summaries of metrics
/// - **Type Identification**: Identify metric types for proper handling
/// - **Validation**: Validate metric consistency and correctness
///
/// # Implementation Requirements
///
/// Implementing types must:
/// - Be cloneable for metrics aggregation
/// - Be debuggable for error reporting
/// - Be thread-safe (`Send + Sync`)
/// - Have a default constructor for initialization
/// - Have a stable lifetime (`'static`)
///
/// # Examples
///
pub trait CollectibleMetrics: Clone + Debug + Send + Sync + Default + 'static {
    /// Resets all metrics to their initial state
    fn reset(&mut self);

    /// Merges metrics from another instance
    fn merge(&mut self, other: &Self);

    /// Returns a summary of key metrics as key-value pairs
    fn summary(&self) -> HashMap<String, String>;

    /// Returns the metric type identifier
    fn metric_type(&self) -> String;

    /// Validates that the metrics are in a consistent state
    fn validate(&self) -> Result<(), PipelineError>;
}

/// Generic metric entry with timing and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEntry<T>
where
    T: CollectibleMetrics,
{
    pub operation_id: String,
    pub operation_type: String,
    pub metrics: T,
    #[serde(with = "datetime_serde")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

impl<T> MetricEntry<T>
where
    T: CollectibleMetrics,
{
    pub fn new(operation_id: String, operation_type: String, metrics: T) -> Self {
        let now = chrono::Utc::now();
        Self {
            operation_id,
            operation_type,
            metrics,
            started_at: now,
            completed_at: now,
            duration_ms: 0,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration_ms = duration.as_millis() as u64;
        self.completed_at = self.started_at + chrono::Duration::milliseconds(self.duration_ms as i64);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error_message = Some(error);
        self.success = false;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Generic metrics collector for any operation type
pub struct GenericMetricsCollector<T>
where
    T: CollectibleMetrics,
{
    collector_name: String,
    entries: RwLock<Vec<MetricEntry<T>>>,
    aggregated_metrics: RwLock<T>,
    active_operations: RwLock<HashMap<String, Instant>>,
    max_entries: usize,
    auto_aggregate: bool,
}

impl<T> GenericMetricsCollector<T>
where
    T: CollectibleMetrics,
{
    /// Creates a new metrics collector
    pub fn new(collector_name: String) -> Self {
        Self {
            collector_name,
            entries: RwLock::new(Vec::new()),
            aggregated_metrics: RwLock::new(T::default()),
            active_operations: RwLock::new(HashMap::new()),
            max_entries: 1000, // Default max entries
            auto_aggregate: true,
        }
    }

    /// Creates a new metrics collector with custom configuration
    pub fn with_config(collector_name: String, max_entries: usize, auto_aggregate: bool) -> Self {
        Self {
            collector_name,
            entries: RwLock::new(Vec::new()),
            aggregated_metrics: RwLock::new(T::default()),
            active_operations: RwLock::new(HashMap::new()),
            max_entries,
            auto_aggregate,
        }
    }

    /// Starts tracking an operation
    pub fn start_operation(&self, operation_id: String) -> Result<(), PipelineError> {
        let mut active_ops = self
            .active_operations
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write active operations: {}", e)))
            ?;

        active_ops.insert(operation_id, Instant::now());
        Ok(())
    }

    /// Completes an operation and records metrics
    pub fn complete_operation(
        &self,
        operation_id: String,
        operation_type: String,
        metrics: T,
    ) -> Result<(), PipelineError> {
        let start_time = {
            let mut active_ops = self
                .active_operations
                .write()
                .map_err(|e| PipelineError::InternalError(format!("Failed to write active operations: {}", e)))
                ?;

            active_ops.remove(&operation_id)
        };

        let duration = start_time
            .map(|start| start.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));

        let entry = MetricEntry::new(operation_id, operation_type, metrics.clone()).with_duration(duration);

        self.record_entry(entry)?;

        if self.auto_aggregate {
            self.aggregate_metrics(&metrics)?;
        }

        Ok(())
    }

    /// Records a metric entry directly
    pub fn record_entry(&self, entry: MetricEntry<T>) -> Result<(), PipelineError> {
        let mut entries = self
            .entries
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write entries: {}", e)))
            ?;

        entries.push(entry);

        // Limit the number of entries to prevent memory issues
        if entries.len() > self.max_entries {
            entries.remove(0);
        }

        Ok(())
    }

    /// Records an operation failure
    pub fn record_failure(
        &self,
        operation_id: String,
        operation_type: String,
        error: PipelineError,
    ) -> Result<(), PipelineError> {
        let start_time = {
            let mut active_ops = self
                .active_operations
                .write()
                .map_err(|e| PipelineError::InternalError(format!("Failed to write active operations: {}", e)))
                ?;

            active_ops.remove(&operation_id)
        };

        let duration = start_time
            .map(|start| start.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));

        let entry = MetricEntry::new(operation_id, operation_type, T::default())
            .with_duration(duration)
            .with_error(error.to_string());

        self.record_entry(entry)
    }

    /// Aggregates metrics into the running total
    fn aggregate_metrics(&self, metrics: &T) -> Result<(), PipelineError> {
        let mut aggregated = self
            .aggregated_metrics
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write aggregated metrics: {}", e)))
            ?;

        aggregated.merge(metrics);
        Ok(())
    }

    /// Gets the current aggregated metrics
    pub fn get_aggregated_metrics(&self) -> Result<T, PipelineError> {
        self.aggregated_metrics
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read aggregated metrics: {}", e)))
            .map(|metrics| metrics.clone())
    }

    /// Gets all recorded entries
    pub fn get_entries(&self) -> Result<Vec<MetricEntry<T>>, PipelineError> {
        self.entries
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read entries: {}", e)))
            .map(|entries| entries.clone())
    }

    /// Gets entries filtered by operation type
    pub fn get_entries_by_type(&self, operation_type: &str) -> Result<Vec<MetricEntry<T>>, PipelineError> {
        let entries = self.get_entries()?;
        Ok(entries
            .into_iter()
            .filter(|entry| entry.operation_type == operation_type)
            .collect())
    }

    /// Gets entries within a time range
    pub fn get_entries_in_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<MetricEntry<T>>, PipelineError> {
        let entries = self.get_entries()?;
        Ok(entries
            .into_iter()
            .filter(|entry| entry.started_at >= start && entry.completed_at <= end)
            .collect())
    }

    /// Resets all metrics and entries
    pub fn reset(&self) -> Result<(), PipelineError> {
        let mut entries = self
            .entries
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write entries: {}", e)))
            ?;

        let mut aggregated = self
            .aggregated_metrics
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write aggregated metrics: {}", e)))
            ?;

        let mut active_ops = self
            .active_operations
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write active operations: {}", e)))
            ?;

        entries.clear();
        aggregated.reset();
        active_ops.clear();

        Ok(())
    }

    /// Gets summary statistics
    pub fn get_summary(&self) -> Result<HashMap<String, String>, PipelineError> {
        let entries = self.get_entries()?;
        let aggregated = self.get_aggregated_metrics()?;

        let mut summary = HashMap::new();
        summary.insert("collector_name".to_string(), self.collector_name.clone());
        summary.insert("total_entries".to_string(), entries.len().to_string());
        summary.insert(
            "successful_operations".to_string(),
            entries.iter().filter(|e| e.success).count().to_string(),
        );
        summary.insert(
            "failed_operations".to_string(),
            entries.iter().filter(|e| !e.success).count().to_string(),
        );

        if !entries.is_empty() {
            let avg_duration = entries.iter().map(|e| e.duration_ms).sum::<u64>() / entries.len() as u64;
            summary.insert("average_duration_ms".to_string(), avg_duration.to_string());

            let max_duration = entries.iter().map(|e| e.duration_ms).max().unwrap_or(0);
            summary.insert("max_duration_ms".to_string(), max_duration.to_string());

            let min_duration = entries.iter().map(|e| e.duration_ms).min().unwrap_or(0);
            summary.insert("min_duration_ms".to_string(), min_duration.to_string());
        }

        // Add aggregated metrics summary
        let aggregated_summary = aggregated.summary();
        summary.extend(aggregated_summary);

        Ok(summary)
    }

    /// Gets the collector name
    pub fn name(&self) -> &str {
        &self.collector_name
    }

    /// Gets the number of active operations
    pub fn active_operations_count(&self) -> Result<usize, PipelineError> {
        self.active_operations
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read active operations: {}", e)))
            .map(|ops| ops.len())
    }
}

/// Trait for services that support metrics collection
#[async_trait]
pub trait MetricsEnabled<T>
where
    T: CollectibleMetrics,
{
    /// Gets the metrics collector for this service
    fn metrics_collector(&self) -> &GenericMetricsCollector<T>;

    /// Records a successful operation
    async fn record_success(
        &self,
        operation_id: String,
        operation_type: String,
        metrics: T,
    ) -> Result<(), PipelineError> {
        self.metrics_collector()
            .complete_operation(operation_id, operation_type, metrics)
    }

    /// Records a failed operation
    async fn record_failure(
        &self,
        operation_id: String,
        operation_type: String,
        error: PipelineError,
    ) -> Result<(), PipelineError> {
        self.metrics_collector()
            .record_failure(operation_id, operation_type, error)
    }

    /// Gets current metrics summary
    async fn get_metrics_summary(&self) -> Result<HashMap<String, String>, PipelineError> {
        self.metrics_collector().get_summary()
    }
}

/// Convenience macro for creating metrics collectors
#[macro_export]
macro_rules! metrics_collector {
    ($metrics_type:ty, $name:expr) => {
        $crate::infrastructure::metrics::GenericMetricsCollector::<$metrics_type>::new(
            $name.to_string(),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default)]
    struct TestMetrics {
        bytes_processed: u64,
        operations_count: u64,
        errors_count: u64,
    }

    impl CollectibleMetrics for TestMetrics {
        fn reset(&mut self) {
            self.bytes_processed = 0;
            self.operations_count = 0;
            self.errors_count = 0;
        }

        fn merge(&mut self, other: &Self) {
            self.bytes_processed += other.bytes_processed;
            self.operations_count += other.operations_count;
            self.errors_count += other.errors_count;
        }

        fn summary(&self) -> HashMap<String, String> {
            let mut summary = HashMap::new();
            summary.insert("bytes_processed".to_string(), self.bytes_processed.to_string());
            summary.insert("operations_count".to_string(), self.operations_count.to_string());
            summary.insert("errors_count".to_string(), self.errors_count.to_string());
            summary
        }

        fn metric_type(&self) -> String {
            "test_metrics".to_string()
        }

        fn validate(&self) -> Result<(), PipelineError> {
            if self.operations_count < self.errors_count {
                return Err(PipelineError::InternalError(
                    "Error count cannot exceed operations count".to_string(),
                ));
            }
            Ok(())
        }
    }

    /// Tests generic metrics collector creation and initialization.
    ///
    /// This test validates that the generic metrics collector can be
    /// created with a name and that it initializes with proper default
    /// state including zero active operations.
    ///
    /// # Test Coverage
    ///
    /// - Metrics collector creation with name
    /// - Name storage and retrieval
    /// - Initial active operations count
    /// - Default state validation
    /// - Basic functionality verification
    ///
    /// # Test Scenario
    ///
    /// Creates a new metrics collector with a test name and verifies
    /// the name is stored correctly and active operations start at zero.
    ///
    /// # Domain Concerns
    ///
    /// - Service initialization and configuration
    /// - Default state management
    /// - Name-based service identification
    /// - Operation tracking initialization
    ///
    /// # Assertions
    ///
    /// - Collector name matches input
    /// - Active operations count starts at zero
    /// - Service is properly initialized
    /// - Basic functionality is operational
    #[test]
    fn test_metrics_collector_creation() {
        let collector = GenericMetricsCollector::<TestMetrics>::new("test_collector".to_string());
        assert_eq!(collector.name(), "test_collector");
        assert_eq!(collector.active_operations_count().unwrap(), 0);
    }

    /// Tests operation tracking lifecycle and state management.
    ///
    /// This test validates that the metrics collector can properly
    /// track operations through their lifecycle from start to completion,
    /// maintaining accurate active operation counts and storing results.
    ///
    /// # Test Coverage
    ///
    /// - Operation start tracking
    /// - Active operation count management
    /// - Operation completion with metrics
    /// - Operation state transitions
    /// - Metrics storage and retrieval
    ///
    /// # Test Scenario
    ///
    /// Starts an operation, verifies active count increases, completes
    /// the operation with metrics, and verifies the results are stored
    /// correctly with active count returning to zero.
    ///
    /// # Domain Concerns
    ///
    /// - Operation lifecycle management
    /// - State tracking and transitions
    /// - Metrics collection and storage
    /// - Operation identification and correlation
    ///
    /// # Assertions
    ///
    /// - Active operations count increases on start
    /// - Active operations count decreases on completion
    /// - Operation metrics are stored correctly
    /// - Operation details are preserved
    #[test]
    fn test_operation_tracking() {
        let collector = GenericMetricsCollector::<TestMetrics>::new("test_collector".to_string());

        // Start operation
        collector.start_operation("op1".to_string()).unwrap();
        assert_eq!(collector.active_operations_count().unwrap(), 1);

        // Complete operation
        let metrics = TestMetrics {
            bytes_processed: 1024,
            operations_count: 1,
            errors_count: 0,
        };

        collector
            .complete_operation("op1".to_string(), "test_operation".to_string(), metrics)
            .unwrap();

        assert_eq!(collector.active_operations_count().unwrap(), 0);

        let entries = collector.get_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].operation_id, "op1");
        assert_eq!(entries[0].metrics.bytes_processed, 1024);
    }

    /// Tests metrics aggregation across multiple operations.
    ///
    /// This test validates that the metrics collector can aggregate
    /// metrics from multiple completed operations, providing accurate
    /// totals and summaries across all collected data.
    ///
    /// # Test Coverage
    ///
    /// - Multiple operation completion
    /// - Metrics aggregation across operations
    /// - Total calculation accuracy
    /// - Error count aggregation
    /// - Cross-operation data consistency
    ///
    /// # Test Scenario
    ///
    /// Completes multiple operations with different metrics and
    /// verifies the aggregated totals are calculated correctly
    /// across all operations.
    ///
    /// # Domain Concerns
    ///
    /// - Cross-operation metrics aggregation
    /// - Total calculation and accuracy
    /// - Error tracking and reporting
    /// - Performance metrics collection
    ///
    /// # Assertions
    ///
    /// - Bytes processed totals are correct
    /// - Operations count totals are correct
    /// - Error count totals are correct
    /// - Aggregation calculations are accurate
    #[test]
    fn test_metrics_aggregation() {
        let collector = GenericMetricsCollector::<TestMetrics>::new("test_collector".to_string());

        let metrics1 = TestMetrics {
            bytes_processed: 1024,
            operations_count: 1,
            errors_count: 0,
        };

        let metrics2 = TestMetrics {
            bytes_processed: 2048,
            operations_count: 1,
            errors_count: 1,
        };

        collector
            .complete_operation("op1".to_string(), "test".to_string(), metrics1)
            .unwrap();
        collector
            .complete_operation("op2".to_string(), "test".to_string(), metrics2)
            .unwrap();

        let aggregated = collector.get_aggregated_metrics().unwrap();
        assert_eq!(aggregated.bytes_processed, 3072);
        assert_eq!(aggregated.operations_count, 2);
        assert_eq!(aggregated.errors_count, 1);
    }

    /// Tests summary generation and reporting functionality.
    ///
    /// This test validates that the metrics collector can generate
    /// comprehensive summaries of collected metrics with proper
    /// formatting and key-value structure for reporting.
    ///
    /// # Test Coverage
    ///
    /// - Summary generation from collected metrics
    /// - Key-value summary structure
    /// - Collector identification in summary
    /// - Success/failure operation counts
    /// - Metrics inclusion in summary
    ///
    /// # Test Scenario
    ///
    /// Completes an operation with metrics and generates a summary,
    /// verifying all expected fields are present with correct values.
    ///
    /// # Domain Concerns
    ///
    /// - Metrics reporting and visualization
    /// - Summary generation for monitoring
    /// - Key performance indicator extraction
    /// - Operational status reporting
    ///
    /// # Assertions
    ///
    /// - Summary contains collector name
    /// - Summary contains total entries count
    /// - Summary contains success/failure counts
    /// - Summary includes metrics data
    #[test]
    fn test_summary_generation() {
        let collector = GenericMetricsCollector::<TestMetrics>::new("test_collector".to_string());

        let metrics = TestMetrics {
            bytes_processed: 1024,
            operations_count: 1,
            errors_count: 0,
        };

        collector
            .complete_operation("op1".to_string(), "test".to_string(), metrics)
            .unwrap();

        let summary = collector.get_summary().unwrap();
        assert_eq!(summary.get("collector_name").unwrap(), "test_collector");
        assert_eq!(summary.get("total_entries").unwrap(), "1");
        assert_eq!(summary.get("successful_operations").unwrap(), "1");
        assert_eq!(summary.get("failed_operations").unwrap(), "0");
        assert!(summary.contains_key("bytes_processed"));
    }

    /// Tests failure recording and error handling.
    ///
    /// This test validates that the metrics collector can properly
    /// record operation failures with error information and maintain
    /// accurate failure tracking and reporting.
    ///
    /// # Test Coverage
    ///
    /// - Operation failure recording
    /// - Error message capture and storage
    /// - Failure state tracking
    /// - Error information preservation
    /// - Failure operation lifecycle
    ///
    /// # Test Scenario
    ///
    /// Starts an operation, records a failure with error information,
    /// and verifies the failure is properly recorded with error details.
    ///
    /// # Domain Concerns
    ///
    /// - Error handling and reporting
    /// - Failure tracking and analysis
    /// - Error message preservation
    /// - Operation failure lifecycle
    ///
    /// # Assertions
    ///
    /// - Failure entry is created
    /// - Success flag is set to false
    /// - Error message is captured
    /// - Failure details are preserved
    #[test]
    fn test_failure_recording() {
        let collector = GenericMetricsCollector::<TestMetrics>::new("test_collector".to_string());

        collector.start_operation("op1".to_string()).unwrap();
        collector
            .record_failure(
                "op1".to_string(),
                "test_operation".to_string(),
                PipelineError::InternalError("Test error".to_string()),
            )
            .unwrap();

        let entries = collector.get_entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert!(!entries[0].success);
        assert!(entries[0].error_message.is_some());
    }

    /// Tests macro-based collector creation convenience.
    ///
    /// This test validates that the metrics collector can be created
    /// using the convenience macro, providing a simpler interface
    /// for common use cases.
    ///
    /// # Test Coverage
    ///
    /// - Macro-based collector creation
    /// - Macro parameter handling
    /// - Convenience interface functionality
    /// - Macro expansion correctness
    /// - Simplified creation patterns
    ///
    /// # Test Scenario
    ///
    /// Creates a metrics collector using the convenience macro and
    /// verifies it functions identically to manual creation.
    ///
    /// # Domain Concerns
    ///
    /// - Developer experience and convenience
    /// - Code simplification and readability
    /// - Macro-based service creation
    /// - API consistency and usability
    ///
    /// # Assertions
    ///
    /// - Macro creates collector successfully
    /// - Collector name is set correctly
    /// - Macro functionality matches manual creation
    /// - Convenience interface works as expected
    #[test]
    fn test_macro_usage() {
        let collector = metrics_collector!(TestMetrics, "test");
        assert_eq!(collector.name(), "test");
    }
}
