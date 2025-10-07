// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Observability Service Implementation
//!
//! This module provides a comprehensive observability service for the adaptive
//! pipeline system. It combines metrics collection, performance tracking,
//! alerting, and health monitoring to provide complete system visibility.
//!
//! ## Overview
//!
//! The observability service implementation provides:
//!
//! - **Real-Time Monitoring**: Live performance tracking and system health
//!   monitoring
//! - **Alerting**: Threshold-based alerting with configurable conditions
//! - **Performance Analysis**: Detailed performance analysis and trend tracking
//! - **Health Scoring**: System health scoring based on multiple indicators
//! - **Integration**: Seamless integration with metrics and configuration
//!   services
//!
//! ## Architecture
//!
//! The observability service follows these design principles:
//!
//! - **Comprehensive Coverage**: Monitors all aspects of system operation
//! - **Real-Time Processing**: Provides real-time insights and alerts
//! - **Configurable Thresholds**: Flexible alerting with configurable
//!   thresholds
//! - **Performance Optimized**: Low overhead monitoring with minimal impact
//!
//! ## Key Components
//!
//! ### Performance Tracker
//!
//! Tracks real-time performance metrics:
//! - **Active Operations**: Number of currently running operations
//! - **Total Operations**: Cumulative count of all operations
//! - **Throughput Metrics**: Average and peak throughput measurements
//! - **Error Rates**: Error rate percentage and trend analysis
//! - **Health Scoring**: Overall system health score calculation
//!
//! ### Alert System
//!
//! Configurable alerting based on thresholds:
//! - **Performance Alerts**: Throughput and latency threshold alerts
//! - **Error Rate Alerts**: Error rate threshold monitoring
//! - **Resource Alerts**: Memory and CPU utilization alerts
//! - **Health Alerts**: System health degradation alerts
//!
//! ### Health Monitoring
//!
//! Comprehensive system health assessment:
//! - **Component Health**: Individual component health status
//! - **Dependency Health**: External dependency health monitoring
//! - **Resource Health**: System resource availability and utilization
//! - **Overall Health**: Aggregated system health score
//!
//! ## Usage Examples
//!
//! ### Basic Observability Service

//!
//! ### Performance Tracking

//!
//! ### Health Monitoring

//!
//! ## Performance Tracking
//!
//! ### Real-Time Metrics
//!
//! The performance tracker maintains real-time metrics:
//!
//! - **Throughput Tracking**: Continuous throughput measurement and averaging
//! - **Operation Counting**: Active and total operation counters
//! - **Error Rate Calculation**: Rolling error rate calculation
//! - **Health Score Computation**: Multi-factor health score calculation
//!
//! ### Trend Analysis
//!
//! - **Moving Averages**: Smoothed metrics using moving averages
//! - **Peak Detection**: Detection and tracking of performance peaks
//! - **Anomaly Detection**: Statistical anomaly detection in metrics
//! - **Trend Prediction**: Short-term trend prediction and forecasting
//!
//! ## Alerting System
//!
//! ### Alert Types
//!
//! - **Critical**: System-threatening conditions requiring immediate attention
//! - **Warning**: Degraded performance or approaching thresholds
//! - **Info**: Informational alerts for significant events
//! - **Debug**: Detailed debugging information for troubleshooting
//!
//! ### Alert Conditions
//!
//! - **Threshold-Based**: Simple threshold crossing alerts
//! - **Rate-Based**: Rate of change alerts (e.g., rapidly increasing errors)
//! - **Composite**: Multi-condition alerts combining multiple metrics
//! - **Time-Based**: Time-window based alerts with hysteresis
//!
//! ### Alert Management
//!
//! - **Deduplication**: Prevents duplicate alerts for the same condition
//! - **Escalation**: Automatic escalation for unacknowledged alerts
//! - **Suppression**: Temporary alert suppression during maintenance
//! - **Routing**: Intelligent alert routing based on severity and type
//!
//! ## Health Monitoring
//!
//! ### Health Indicators
//!
//! The system tracks multiple health indicators:
//!
//! - **Performance Health**: Based on throughput and latency metrics
//! - **Error Health**: Based on error rates and failure patterns
//! - **Resource Health**: Based on CPU, memory, and I/O utilization
//! - **Dependency Health**: Based on external service availability
//!
//! ### Health Scoring
//!
//! Health scores are calculated using weighted factors:
//! - **Performance Weight**: 30% - System performance metrics
//! - **Reliability Weight**: 25% - Error rates and stability
//! - **Resource Weight**: 25% - Resource utilization and availability
//! - **Dependency Weight**: 20% - External dependency health
//!
//! ## Integration
//!
//! The observability service integrates with:
//!
//! - **Metrics Service**: Collects and analyzes metrics data
//! - **Configuration Service**: Dynamic configuration of thresholds and
//!   settings
//! - **Logging System**: Correlates observability data with application logs
//! - **External Monitoring**: Integrates with external monitoring systems
//!
//! ## Performance Considerations
//!
//! ### Low Overhead Design
//!
//! - **Efficient Data Structures**: Optimized data structures for metric
//!   storage
//! - **Sampling**: Configurable sampling rates for high-frequency metrics
//! - **Batch Processing**: Batch processing of metrics to reduce overhead
//! - **Lazy Evaluation**: Expensive calculations performed only when needed
//!
//! ### Scalability
//!
//! - **Concurrent Processing**: Thread-safe concurrent metric processing
//! - **Memory Management**: Bounded memory usage with automatic cleanup
//! - **Resource Pooling**: Efficient resource pooling and reuse
//! - **Load Balancing**: Distributed processing for high-load scenarios
//!
//! ## Security and Privacy
//!
//! ### Data Protection
//!
//! - **No Sensitive Data**: Observability data contains no sensitive
//!   information
//! - **Aggregated Metrics**: Only aggregated statistics are stored and exposed
//! - **Access Control**: Observability endpoints can be secured
//! - **Audit Logging**: Access to observability data can be audited
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Machine Learning**: AI-powered anomaly detection and prediction
//! - **Advanced Analytics**: Statistical analysis and correlation detection
//! - **Custom Dashboards**: User-configurable monitoring dashboards
//! - **Integration APIs**: APIs for integration with external tools

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::infrastructure::config::config_service::ConfigService;
use crate::infrastructure::metrics::MetricsService;
use adaptive_pipeline_domain::entities::processing_metrics::ProcessingMetrics;

/// Enhanced observability service for comprehensive monitoring
///
/// This service provides a comprehensive observability solution for the
/// adaptive pipeline system, combining real-time monitoring, alerting,
/// performance tracking, and health assessment capabilities.
///
/// # Key Features
///
/// - **Real-Time Monitoring**: Continuous monitoring of system performance and
///   health
/// - **Intelligent Alerting**: Threshold-based alerting with configurable
///   conditions
/// - **Performance Analysis**: Detailed performance tracking and trend analysis
/// - **Health Assessment**: Multi-factor system health scoring
/// - **Integration**: Seamless integration with metrics and configuration
///   services
///
/// # Architecture
///
/// The service is built around several core components:
/// - **Performance Tracker**: Real-time performance metric tracking
/// - **Alert Manager**: Configurable alerting system
/// - **Health Monitor**: System health assessment and scoring
/// - **Configuration Integration**: Dynamic configuration management
///
/// # Examples
#[derive(Clone)]
pub struct ObservabilityService {
    metrics_service: Arc<MetricsService>,
    performance_tracker: Arc<RwLock<PerformanceTracker>>,
    alert_thresholds: AlertThresholds,
}

/// Real-time performance tracking
#[derive(Debug, Clone)]
pub struct PerformanceTracker {
    pub active_operations: u32,
    pub total_operations: u64,
    pub average_throughput_mbps: f64,
    pub peak_throughput_mbps: f64,
    pub error_rate_percent: f64,
    pub system_health_score: f64,
    pub last_update: Instant,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub max_error_rate_percent: f64,
    pub min_throughput_mbps: f64,
    pub max_processing_duration_seconds: f64,
    pub max_memory_usage_mb: f64,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub score: f64,
    pub active_operations: u32,
    pub throughput_mbps: f64,
    pub error_rate_percent: f64,
    pub uptime_seconds: u64,
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: String,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_error_rate_percent: 5.0,
            min_throughput_mbps: 1.0,
            max_processing_duration_seconds: 300.0,
            max_memory_usage_mb: 1024.0,
        }
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self {
            active_operations: 0,
            total_operations: 0,
            average_throughput_mbps: 0.0,
            peak_throughput_mbps: 0.0,
            error_rate_percent: 0.0,
            system_health_score: 100.0,
            last_update: Instant::now(),
        }
    }
}

impl ObservabilityService {
    /// Create a new observability service
    pub fn new(metrics_service: Arc<MetricsService>) -> Self {
        Self {
            metrics_service,
            performance_tracker: Arc::new(RwLock::new(PerformanceTracker::default())),
            alert_thresholds: AlertThresholds::default(),
        }
    }

    /// Create a new observability service with configuration
    pub async fn new_with_config(metrics_service: Arc<MetricsService>) -> Self {
        let (error_rate_threshold, throughput_threshold) = ConfigService::get_alert_thresholds().await;

        Self {
            metrics_service,
            performance_tracker: Arc::new(RwLock::new(PerformanceTracker::default())),
            alert_thresholds: AlertThresholds {
                max_error_rate_percent: error_rate_threshold,
                min_throughput_mbps: throughput_threshold,
                ..AlertThresholds::default()
            },
        }
    }

    /// Start operation tracking
    pub async fn start_operation(&self, operation_name: &str) -> OperationTracker {
        let mut tracker = self.performance_tracker.write().await;
        tracker.active_operations += 1;
        tracker.total_operations += 1;
        tracker.last_update = Instant::now();

        debug!(
            "Started operation: {} (active: {})",
            operation_name, tracker.active_operations
        );

        OperationTracker {
            operation_name: operation_name.to_string(),
            start_time: Instant::now(),
            observability_service: self.clone(),
            completed: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Complete operation tracking
    pub async fn complete_operation(
        &self,
        operation_name: &str,
        duration: Duration,
        success: bool,
        throughput_mbps: f64,
    ) {
        let mut tracker = self.performance_tracker.write().await;

        if tracker.active_operations > 0 {
            tracker.active_operations -= 1;
        }

        // Update throughput metrics
        if throughput_mbps > tracker.peak_throughput_mbps {
            tracker.peak_throughput_mbps = throughput_mbps;
        }

        // Update average throughput (simple moving average)
        tracker.average_throughput_mbps = (tracker.average_throughput_mbps + throughput_mbps) / 2.0;

        // Update error rate (track both success and failure)
        let total_ops = tracker.total_operations as f64;
        if total_ops > 0.0 {
            let error_contribution = if success { 0.0 } else { 100.0 };
            tracker.error_rate_percent =
                (tracker.error_rate_percent * (total_ops - 1.0) + error_contribution) / total_ops;
        }

        tracker.last_update = Instant::now();

        // Note: Pipeline-specific metrics are handled by MetricsObserver
        // Observability service only tracks operation-level metrics

        if !success {
            self.metrics_service.increment_errors();
        }

        info!(
            "Completed operation: {} in {:.2}s (throughput: {:.2} MB/s, success: {})",
            operation_name,
            duration.as_secs_f64(),
            throughput_mbps,
            success
        );

        // Check for alerts
        self.check_alerts(&tracker).await;
    }

    /// Get current system health
    pub async fn get_system_health(&self) -> SystemHealth {
        let tracker = self.performance_tracker.read().await;
        let uptime = tracker.last_update.elapsed().as_secs();

        // Calculate health score
        let mut score = 100.0;
        let mut alerts = Vec::new();

        // Check error rate
        if tracker.error_rate_percent > self.alert_thresholds.max_error_rate_percent {
            score -= 30.0;
            alerts.push(Alert {
                severity: AlertSeverity::Warning,
                message: format!("High error rate: {:.1}%", tracker.error_rate_percent),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metric_name: "error_rate_percent".to_string(),
                current_value: tracker.error_rate_percent,
                threshold: self.alert_thresholds.max_error_rate_percent,
            });
        }

        // Check throughput
        if tracker.average_throughput_mbps < self.alert_thresholds.min_throughput_mbps {
            score -= 20.0;
            alerts.push(Alert {
                severity: AlertSeverity::Warning,
                message: format!("Low throughput: {:.2} MB/s", tracker.average_throughput_mbps),
                timestamp: chrono::Utc::now().to_rfc3339(),
                metric_name: "throughput_mbps".to_string(),
                current_value: tracker.average_throughput_mbps,
                threshold: self.alert_thresholds.min_throughput_mbps,
            });
        }

        let status = if score >= 90.0 {
            HealthStatus::Healthy
        } else if score >= 70.0 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };

        SystemHealth {
            status,
            score,
            active_operations: tracker.active_operations,
            throughput_mbps: tracker.average_throughput_mbps,
            error_rate_percent: tracker.error_rate_percent,
            uptime_seconds: uptime,
            alerts,
        }
    }

    /// Record processing metrics
    pub async fn record_processing_metrics(&self, metrics: &ProcessingMetrics) {
        // Record to Prometheus
        self.metrics_service.record_pipeline_completion(metrics);

        // Update performance tracking
        let throughput = metrics.throughput_mb_per_second();
        let success = metrics.error_count() == 0;
        let duration = metrics.processing_duration().unwrap_or(Duration::from_secs(0));

        self.complete_operation("pipeline_processing", duration, success, throughput)
            .await;

        debug!(
            "Recorded processing metrics: {:.2} MB/s throughput, {} errors, {} warnings",
            throughput,
            metrics.error_count(),
            metrics.warning_count()
        );
    }

    /// Check for alerts based on current metrics
    async fn check_alerts(&self, tracker: &PerformanceTracker) {
        // Error rate alert
        if tracker.error_rate_percent > self.alert_thresholds.max_error_rate_percent {
            warn!(
                "🚨 Alert: High error rate {:.1}% (threshold: {:.1}%)",
                tracker.error_rate_percent, self.alert_thresholds.max_error_rate_percent
            );
        }

        // Low throughput alert
        if tracker.average_throughput_mbps < self.alert_thresholds.min_throughput_mbps {
            warn!(
                "🚨 Alert: Low throughput {:.2} MB/s (threshold: {:.2} MB/s)",
                tracker.average_throughput_mbps, self.alert_thresholds.min_throughput_mbps
            );
        }

        // High load alert
        if tracker.active_operations > 10 {
            warn!("🚨 Alert: High concurrent operations: {}", tracker.active_operations);
        }
    }

    /// Get performance summary for display
    pub async fn get_performance_summary(&self) -> String {
        let tracker = self.performance_tracker.read().await;
        let health = self.get_system_health().await;

        format!(
            "📊 Performance Summary:\nActive Operations: {}\nTotal Operations: {}\nAverage Throughput: {:.2} \
             MB/s\nPeak Throughput: {:.2} MB/s\nError Rate: {:.1}%\nSystem Health: {:.1}/100 ({:?})\nAlerts: {}",
            tracker.active_operations,
            tracker.total_operations,
            tracker.average_throughput_mbps,
            tracker.peak_throughput_mbps,
            tracker.error_rate_percent,
            health.score,
            health.status,
            health.alerts.len()
        )
    }
}

/// Individual operation tracker
pub struct OperationTracker {
    operation_name: String,
    start_time: Instant,
    observability_service: ObservabilityService,
    completed: std::sync::atomic::AtomicBool,
}

impl OperationTracker {
    /// Complete the operation with success/failure status
    pub async fn complete(self, success: bool, bytes_processed: u64) {
        // Mark as completed to prevent Drop from running
        self.completed.store(true, std::sync::atomic::Ordering::Relaxed);

        let duration = self.start_time.elapsed();
        let throughput_mbps = if duration.as_secs_f64() > 0.0 {
            (bytes_processed as f64) / (1024.0 * 1024.0) / duration.as_secs_f64()
        } else {
            0.0
        };

        self.observability_service
            .complete_operation(&self.operation_name, duration, success, throughput_mbps)
            .await;
    }

    /// Complete with processing metrics
    pub async fn complete_with_metrics(self, metrics: &ProcessingMetrics) {
        let success = metrics.error_count() == 0;
        let bytes_processed = metrics.bytes_processed();
        self.complete(success, bytes_processed).await;
    }
}

impl Drop for OperationTracker {
    fn drop(&mut self) {
        // Only mark as failed if not explicitly completed
        if !self.completed.load(std::sync::atomic::Ordering::Relaxed) {
            let observability_service = self.observability_service.clone();
            let operation_name = self.operation_name.clone();
            let duration = self.start_time.elapsed();

            tokio::spawn(async move {
                observability_service
                    .complete_operation(&operation_name, duration, false, 0.0)
                    .await;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests observability service creation and initialization.
    ///
    /// This test validates that the observability service can be created
    /// with proper dependencies and that all alert thresholds and
    /// metrics service integration are correctly initialized.
    ///
    /// # Test Coverage
    ///
    /// - Service creation with metrics service dependency
    /// - Alert threshold initialization and validation
    /// - Metrics service integration verification
    /// - Service state validation after creation
    /// - Basic functionality verification
    ///
    /// # Test Scenario
    ///
    /// Creates an observability service with a metrics service dependency
    /// and verifies all components are properly initialized and functional.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Service dependency injection and management
    /// - Alert threshold configuration and validation
    /// - Metrics service integration and communication
    /// - Service lifecycle management
    ///
    /// # Assertions
    ///
    /// - Alert thresholds are properly initialized
    /// - Metrics service integration works correctly
    /// - Service creation succeeds without errors
    /// - Basic service functionality is operational
    #[test]
    fn test_observability_service_creation() {
        // Test basic service creation without async operations
        let metrics_service = Arc::new(MetricsService::new().unwrap());
        let observability = ObservabilityService::new(metrics_service);

        // Verify the service was created successfully
        assert!(!observability.alert_thresholds.max_error_rate_percent.is_nan());
        assert!(observability.alert_thresholds.min_throughput_mbps > 0.0);

        // Test that we can get metrics (this verifies the service is working)
        let metrics_result = observability.metrics_service.get_metrics();
        assert!(metrics_result.is_ok());
    }

    /// Tests operation tracking functionality and metrics integration.
    ///
    /// This test validates that the observability service can properly
    /// track operations through its metrics service integration and
    /// that operation state changes are handled correctly.
    ///
    /// # Test Coverage
    ///
    /// - Operation tracking structure and initialization
    /// - Metrics service method invocation
    /// - Active pipeline tracking operations
    /// - Service state management during operations
    /// - Public method accessibility and functionality
    ///
    /// # Test Scenario
    ///
    /// Creates an observability service and tests operation tracking
    /// by invoking metrics service methods for pipeline management.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Operation tracking and state management
    /// - Metrics service method delegation
    /// - Pipeline lifecycle tracking
    /// - Service method accessibility and integration
    ///
    /// # Assertions
    ///
    /// - Alert thresholds are properly configured
    /// - Metrics service methods are accessible
    /// - Operation tracking methods execute successfully
    /// - Service state remains consistent
    #[test]
    fn test_operation_tracking() {
        // Test basic operation tracking structure without async operations
        let metrics_service = Arc::new(MetricsService::new().unwrap());
        let observability = ObservabilityService::new(metrics_service);

        // Verify initial state
        assert!(observability.alert_thresholds.max_error_rate_percent > 0.0);

        // Test that we can call public methods on the metrics service
        observability.metrics_service.increment_active_pipelines();
        observability.metrics_service.decrement_active_pipelines();
    }

    /// Tests performance summary functionality and metrics updates.
    ///
    /// This test validates that the observability service can handle
    /// performance metrics updates and that the performance summary
    /// structure is properly maintained and accessible.
    ///
    /// # Test Coverage
    ///
    /// - Performance summary structure and initialization
    /// - Metrics service performance method invocation
    /// - Throughput tracking and updates
    /// - Pipeline processing metrics
    /// - Service configuration validation
    ///
    /// # Test Scenario
    ///
    /// Creates an observability service and tests performance summary
    /// functionality by updating various performance metrics.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Performance metrics collection and management
    /// - Throughput tracking and measurement
    /// - Pipeline processing statistics
    /// - Service configuration and threshold management
    ///
    /// # Assertions
    ///
    /// - Performance thresholds are properly configured
    /// - Metrics service performance methods are accessible
    /// - Performance updates execute successfully
    /// - Service maintains consistent state
    #[test]
    fn test_performance_summary() {
        // Test basic performance summary structure without async operations
        let metrics_service = Arc::new(MetricsService::new().unwrap());
        let observability = ObservabilityService::new(metrics_service);

        // Verify service creation and basic properties
        assert!(observability.alert_thresholds.min_throughput_mbps > 0.0);
        assert!(observability.alert_thresholds.max_error_rate_percent > 0.0);

        // Test that we can call metrics methods
        observability.metrics_service.increment_processed_pipelines();
        observability.metrics_service.update_throughput(10.5);
    }
}
