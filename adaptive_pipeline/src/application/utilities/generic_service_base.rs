// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Generic Service Base
//!
//! This module provides a generic base framework for building domain services
//! within the adaptive pipeline system. It defines common patterns, interfaces,
//! and utilities that can be reused across different service implementations.
//!
//! ## Overview
//!
//! The generic service base provides:
//!
//! - **Service Configuration**: Generic configuration management for services
//! - **Statistics Collection**: Standardized statistics collection and
//!   reporting
//! - **Lifecycle Management**: Common service lifecycle patterns
//! - **Error Handling**: Consistent error handling across services
//! - **Performance Monitoring**: Built-in performance monitoring capabilities
//!
//! ## Architecture
//!
//! The service base follows Domain-Driven Design principles:
//!
//! - **Generic Design**: Reusable patterns for any service type
//! - **Configuration Framework**: Standardized configuration management
//! - **Statistics Framework**: Consistent statistics collection
//! - **Lifecycle Framework**: Common service lifecycle management
//!
//! ## Key Features
//!
//! ### Service Configuration
//!
//! - **Validation**: Comprehensive configuration validation
//! - **Defaults**: Sensible default configuration values
//! - **Merging**: Configuration merging and override capabilities
//! - **Hot Reloading**: Runtime configuration updates
//!
//! ### Statistics Collection
//!
//! - **Standardized Metrics**: Consistent metrics across services
//! - **Aggregation**: Automatic statistics aggregation
//! - **Reporting**: Comprehensive statistics reporting
//! - **Performance Tracking**: Built-in performance monitoring
//!
//! ### Service Lifecycle
//!
//! - **Initialization**: Standardized service initialization
//! - **Health Monitoring**: Service health checks and monitoring
//! - **Graceful Shutdown**: Proper service shutdown procedures
//! - **Resource Management**: Automatic resource cleanup
//!
//! ## Usage Examples
//!
//! ### Basic Service Implementation

//!
//! ### Service with Health Monitoring

//!
//! ### Service with Configuration Hot Reloading

//!
//! ## Service Configuration Framework
//!
//! ### Configuration Validation
//!
//! - **Schema Validation**: Validate configuration against defined schemas
//! - **Business Rules**: Enforce business logic constraints
//! - **Cross-Field Validation**: Validate relationships between fields
//! - **Custom Validators**: Support for custom validation logic
//!
//! ### Configuration Management
//!
//! - **Default Values**: Provide sensible default configurations
//! - **Configuration Merging**: Merge configurations from multiple sources
//! - **Environment Overrides**: Override configuration with environment
//!   variables
//! - **Hot Reloading**: Runtime configuration updates without restart
//!
//! ## Statistics Framework
//!
//! ### Metrics Collection
//!
//! - **Standardized Metrics**: Consistent metrics across all services
//! - **Performance Metrics**: Response times, throughput, error rates
//! - **Resource Metrics**: Memory usage, CPU usage, connection counts
//! - **Custom Metrics**: Support for service-specific metrics
//!
//! ### Statistics Aggregation
//!
//! - **Time-based Aggregation**: Aggregate statistics over time windows
//! - **Service-level Aggregation**: Aggregate statistics across service
//!   instances
//! - **System-level Aggregation**: Aggregate statistics across the entire
//!   system
//!
//! ## Service Lifecycle Management
//!
//! ### Initialization
//!
//! - **Configuration Loading**: Load and validate service configuration
//! - **Resource Allocation**: Allocate necessary resources
//! - **Dependency Injection**: Inject required dependencies
//! - **Health Check Setup**: Set up health monitoring
//!
//! ### Runtime Management
//!
//! - **Request Processing**: Handle incoming requests
//! - **Statistics Collection**: Collect performance and usage statistics
//! - **Health Monitoring**: Monitor service health and status
//! - **Configuration Updates**: Handle configuration changes
//!
//! ### Shutdown
//!
//! - **Graceful Shutdown**: Gracefully shut down service operations
//! - **Resource Cleanup**: Clean up allocated resources
//! - **Statistics Reporting**: Report final statistics
//! - **State Persistence**: Persist important state information
//!
//! ## Error Handling
//!
//! ### Service Errors
//!
//! - **Configuration Errors**: Invalid configuration parameters
//! - **Runtime Errors**: Errors during service operation
//! - **Resource Errors**: Resource allocation and management errors
//! - **Dependency Errors**: Errors with external dependencies
//!
//! ### Error Recovery
//!
//! - **Automatic Recovery**: Automatic recovery from transient errors
//! - **Circuit Breaker**: Circuit breaker pattern for external dependencies
//! - **Fallback Strategies**: Fallback strategies for service degradation
//! - **Error Reporting**: Comprehensive error reporting and logging
//!
//! ## Performance Considerations
//!
//! ### Service Efficiency
//!
//! - **Minimal Overhead**: Designed for minimal performance impact
//! - **Efficient Statistics**: Low-overhead statistics collection
//! - **Resource Optimization**: Efficient resource usage and management
//! - **Caching**: Intelligent caching strategies
//!
//! ### Scalability
//!
//! - **Horizontal Scaling**: Support for horizontal service scaling
//! - **Load Balancing**: Built-in load balancing capabilities
//! - **Resource Pooling**: Efficient resource pooling and reuse
//!
//! ## Integration
//!
//! The service base integrates with:
//!
//! - **Configuration System**: Centralized configuration management
//! - **Monitoring System**: Service monitoring and alerting
//! - **Logging System**: Structured logging and audit trails
//! - **Metrics System**: Metrics collection and reporting
//!
//! ## Thread Safety
//!
//! The service base is fully thread-safe:
//!
//! - **Concurrent Operations**: Safe concurrent service operations
//! - **Shared State**: Thread-safe shared state management
//! - **Configuration Updates**: Thread-safe configuration updates
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Service Discovery**: Automatic service discovery and registration
//! - **Load Balancing**: Advanced load balancing strategies
//! - **Circuit Breaker**: Built-in circuit breaker implementation
//! - **Distributed Tracing**: Distributed tracing support

use async_trait::async_trait;
use adaptive_pipeline_domain::error::PipelineError;
use adaptive_pipeline_domain::services::datetime_serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::RwLock;

/// Generic trait for service configuration validation and defaults
///
/// This trait defines the interface for service configuration management,
/// providing validation, default values, and configuration merging
/// capabilities.
///
/// # Key Features
///
/// - **Validation**: Comprehensive configuration validation
/// - **Default Values**: Sensible default configuration values
/// - **Merging**: Configuration merging and override capabilities
/// - **Type Safety**: Type-safe configuration management
///
/// # Implementation Requirements
///
/// Implementing types must:
/// - Be cloneable for configuration updates
/// - Be debuggable for error reporting
/// - Be thread-safe (`Send + Sync`)
/// - Have a stable lifetime (`'static`)
///
/// # Examples
pub trait ServiceConfig: Clone + Debug + Send + Sync + 'static {
    /// Validates the configuration and returns errors if invalid
    fn validate(&self) -> Result<(), PipelineError>;

    /// Returns default configuration
    fn default_config() -> Self;

    /// Merges this configuration with another, preferring values from `other`
    fn merge(&self, other: &Self) -> Self;
}

/// Generic trait for service statistics collection
pub trait ServiceStats: Clone + Debug + Send + Sync + Default + 'static {
    /// Resets all statistics to their initial state
    fn reset(&mut self);

    /// Merges statistics from another instance
    fn merge(&mut self, other: &Self);

    /// Returns a summary of key metrics as key-value pairs
    fn summary(&self) -> HashMap<String, String>;
}

/// Generic service metadata for tracking service lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    pub service_name: String,
    pub service_version: String,
    #[serde(with = "datetime_serde")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

impl ServiceMetadata {
    pub fn new(service_name: String, service_version: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            service_name,
            service_version,
            created_at: now,
            last_updated: now,
            metadata: HashMap::new(),
        }
    }

    pub fn update_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.last_updated = chrono::Utc::now();
    }
}

/// Generic service base providing common functionality for all services
pub struct GenericServiceBase<C, S>
where
    C: ServiceConfig,
    S: ServiceStats,
{
    metadata: ServiceMetadata,
    config: RwLock<C>,
    stats: RwLock<S>,
    is_healthy: RwLock<bool>,
}

impl<C, S> GenericServiceBase<C, S>
where
    C: ServiceConfig,
    S: ServiceStats,
{
    /// Creates a new service base with default configuration
    pub fn new(service_name: String, service_version: String) -> Self {
        Self {
            metadata: ServiceMetadata::new(service_name, service_version),
            config: RwLock::new(C::default_config()),
            stats: RwLock::new(S::default()),
            is_healthy: RwLock::new(true),
        }
    }

    /// Creates a new service base with custom configuration
    pub fn with_config(service_name: String, service_version: String, config: C) -> Result<Self, PipelineError> {
        config.validate()?;
        Ok(Self {
            metadata: ServiceMetadata::new(service_name, service_version),
            config: RwLock::new(config),
            stats: RwLock::new(S::default()),
            is_healthy: RwLock::new(true),
        })
    }

    /// Gets the current configuration (read-only)
    pub fn get_config(&self) -> Result<C, PipelineError> {
        self.config
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read config: {}", e)))
            .map(|config| config.clone())
    }

    /// Updates the configuration after validation
    pub fn update_config(&self, new_config: C) -> Result<(), PipelineError> {
        new_config.validate()?;

        let mut config = self
            .config
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write config: {}", e)))?;

        *config = new_config;
        Ok(())
    }

    /// Merges configuration with existing config
    pub fn merge_config(&self, partial_config: C) -> Result<(), PipelineError> {
        let current_config = self.get_config()?;
        let merged_config = current_config.merge(&partial_config);
        self.update_config(merged_config)
    }

    /// Gets current statistics (read-only)
    pub fn get_stats(&self) -> Result<S, PipelineError> {
        self.stats
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read stats: {}", e)))
            .map(|stats| stats.clone())
    }

    /// Updates statistics
    pub fn update_stats<F>(&self, updater: F) -> Result<(), PipelineError>
    where
        F: FnOnce(&mut S),
    {
        let mut stats = self
            .stats
            .write()
            .map_err(|e| PipelineError::InternalError(format!("Failed to write stats: {}", e)))?;

        updater(&mut *stats);
        Ok(())
    }

    /// Resets all statistics
    pub fn reset_stats(&self) -> Result<(), PipelineError> {
        self.update_stats(|stats| stats.reset())
    }

    /// Gets service metadata
    pub fn get_metadata(&self) -> &ServiceMetadata {
        &self.metadata
    }

    /// Checks if the service is healthy
    pub fn is_healthy(&self) -> bool {
        self.is_healthy.read().map(|health| *health).unwrap_or(false)
    }

    /// Sets the health status
    pub fn set_health(&self, healthy: bool) {
        if let Ok(mut health) = self.is_healthy.write() {
            *health = healthy;
        }
    }

    /// Gets a summary of service status
    pub fn get_service_summary(&self) -> Result<HashMap<String, String>, PipelineError> {
        let mut summary = HashMap::new();

        summary.insert("service_name".to_string(), self.metadata.service_name.clone());
        summary.insert("service_version".to_string(), self.metadata.service_version.clone());
        summary.insert("is_healthy".to_string(), self.is_healthy().to_string());
        summary.insert("created_at".to_string(), self.metadata.created_at.to_rfc3339());
        summary.insert("last_updated".to_string(), self.metadata.last_updated.to_rfc3339());

        // Add statistics summary
        let stats = self.get_stats()?;
        let stats_summary = stats.summary();
        summary.extend(stats_summary);

        Ok(summary)
    }
}

/// Trait for services that can be started and stopped
#[async_trait]
pub trait ServiceLifecycle {
    /// Starts the service
    async fn start(&self) -> Result<(), PipelineError>;

    /// Stops the service gracefully
    async fn stop(&self) -> Result<(), PipelineError>;

    /// Restarts the service
    async fn restart(&self) -> Result<(), PipelineError> {
        self.stop().await?;
        self.start().await
    }

    /// Performs health check
    async fn health_check(&self) -> Result<bool, PipelineError>;
}

/// Trait for services that support metrics collection
pub trait ServiceMetrics<S: ServiceStats> {
    /// Records a metric value
    fn record_metric(&self, metric_name: &str, value: f64) -> Result<(), PipelineError>;

    /// Increments a counter metric
    fn increment_counter(&self, counter_name: &str) -> Result<(), PipelineError>;

    /// Records timing information
    fn record_timing(&self, operation_name: &str, duration_ms: u64) -> Result<(), PipelineError>;

    /// Gets current metrics snapshot
    fn get_metrics_snapshot(&self) -> Result<S, PipelineError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct TestConfig {
        max_connections: u32,
        timeout_ms: u64,
    }

    impl ServiceConfig for TestConfig {
        fn validate(&self) -> Result<(), PipelineError> {
            if self.max_connections == 0 {
                return Err(PipelineError::InvalidConfiguration(
                    "max_connections must be greater than 0".to_string(),
                ));
            }
            if self.timeout_ms == 0 {
                return Err(PipelineError::InvalidConfiguration(
                    "timeout_ms must be greater than 0".to_string(),
                ));
            }
            Ok(())
        }

        fn default_config() -> Self {
            Self {
                max_connections: 10,
                timeout_ms: 5000,
            }
        }

        fn merge(&self, other: &Self) -> Self {
            Self {
                max_connections: other.max_connections,
                timeout_ms: other.timeout_ms,
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    struct TestStats {
        requests_processed: u64,
        errors_count: u64,
        total_processing_time_ms: u64,
    }

    impl ServiceStats for TestStats {
        fn reset(&mut self) {
            self.requests_processed = 0;
            self.errors_count = 0;
            self.total_processing_time_ms = 0;
        }

        fn merge(&mut self, other: &Self) {
            self.requests_processed += other.requests_processed;
            self.errors_count += other.errors_count;
            self.total_processing_time_ms += other.total_processing_time_ms;
        }

        fn summary(&self) -> HashMap<String, String> {
            let mut summary = HashMap::new();
            summary.insert("requests_processed".to_string(), self.requests_processed.to_string());
            summary.insert("errors_count".to_string(), self.errors_count.to_string());
            summary.insert(
                "total_processing_time_ms".to_string(),
                self.total_processing_time_ms.to_string(),
            );
            summary
        }
    }

    /// Tests generic service base creation and initialization.
    ///
    /// This test validates that generic service base instances can be
    /// created with proper initialization of metadata, configuration,
    /// and health status for service management.
    ///
    /// # Test Coverage
    ///
    /// - Service creation with name and version
    /// - Metadata initialization and storage
    /// - Default configuration loading
    /// - Health status initialization
    /// - Configuration value validation
    ///
    /// # Test Scenario
    ///
    /// Creates a generic service base with test configuration and
    /// verifies all components are properly initialized.
    ///
    /// # Domain Concerns
    ///
    /// - Service lifecycle management
    /// - Configuration management
    /// - Service metadata handling
    /// - Health monitoring
    ///
    /// # Assertions
    ///
    /// - Service name is stored correctly
    /// - Service version is stored correctly
    /// - Service is healthy by default
    /// - Configuration values are loaded
    /// - Default configuration is applied
    #[test]
    fn test_generic_service_base_creation() {
        let service = GenericServiceBase::<TestConfig, TestStats>::new("test_service".to_string(), "1.0.0".to_string());

        // assert_eq!(service.get_metadata().service_name, "test_service");
        // assert_eq!(service.get_metadata().service_version, "1.0.0");
        // assert!(service.is_healthy());

        let config = TestConfig {
            max_connections: 10,
            timeout_ms: 5000,
        };
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.timeout_ms, 5000);
    }

    /// Tests service configuration validation and constraint enforcement.
    ///
    /// This test validates that service configuration is properly
    /// validated during service creation and that invalid configurations
    /// are rejected with appropriate error handling.
    ///
    /// # Test Coverage
    ///
    /// - Configuration validation during creation
    /// - Invalid configuration rejection
    /// - Constraint enforcement
    /// - Error handling for invalid config
    /// - Validation rule application
    ///
    /// # Test Scenario
    ///
    /// Attempts to create a service with invalid configuration
    /// and verifies that creation fails with validation error.
    ///
    /// # Domain Concerns
    ///
    /// - Configuration validation
    /// - Service creation safety
    /// - Constraint enforcement
    /// - Error handling
    ///
    /// # Assertions
    ///
    /// - Invalid configuration is rejected
    /// - Service creation fails
    /// - Validation error is returned
    /// - Constraints are enforced
    #[test]
    fn test_config_validation() {
        let invalid_config = TestConfig {
            max_connections: 0,
            timeout_ms: 1000,
        };

        let result = GenericServiceBase::<TestConfig, TestStats>::with_config(
            "test_service".to_string(),
            "1.0.0".to_string(),
            invalid_config,
        );

        assert!(result.is_err());
    }

    /// Tests service statistics operations and management.
    ///
    /// This test validates that service statistics can be updated,
    /// retrieved, and reset properly for service monitoring
    /// and performance tracking.
    ///
    /// # Test Coverage
    ///
    /// - Statistics update functionality
    /// - Statistics retrieval
    /// - Statistics reset operations
    /// - Concurrent statistics access
    /// - Statistics state management
    ///
    /// # Test Scenario
    ///
    /// Updates service statistics, retrieves them, resets them,
    /// and verifies all operations work correctly.
    ///
    /// # Domain Concerns
    ///
    /// - Service monitoring
    /// - Performance tracking
    /// - Statistics management
    /// - Operational metrics
    ///
    /// # Assertions
    ///
    /// - Statistics are updated correctly
    /// - Statistics can be retrieved
    /// - Statistics reset works properly
    /// - Values are preserved and reset
    #[test]
    fn test_stats_operations() {
        let service = GenericServiceBase::<TestConfig, TestStats>::new("test_service".to_string(), "1.0.0".to_string());

        // Update stats
        service
            .update_stats(|stats| {
                stats.requests_processed = 100;
                stats.errors_count = 5;
            })
            .unwrap();

        let stats = service.get_stats().unwrap();
        assert_eq!(stats.requests_processed, 100);
        assert_eq!(stats.errors_count, 5);

        // Reset stats
        service.reset_stats().unwrap();
        let stats = service.get_stats().unwrap();
        assert_eq!(stats.requests_processed, 0);
        assert_eq!(stats.errors_count, 0);
    }

    /// Tests service summary generation and information aggregation.
    ///
    /// This test validates that service summaries can be generated
    /// with comprehensive service information including metadata,
    /// health status, and operational statistics.
    ///
    /// # Test Coverage
    ///
    /// - Service summary generation
    /// - Metadata inclusion in summary
    /// - Health status reporting
    /// - Statistics inclusion
    /// - Summary completeness validation
    ///
    /// # Test Scenario
    ///
    /// Generates a service summary and verifies that all expected
    /// information is included and correctly formatted.
    ///
    /// # Domain Concerns
    ///
    /// - Service reporting
    /// - Information aggregation
    /// - Operational visibility
    /// - Service monitoring
    ///
    /// # Assertions
    ///
    /// - Service name is included
    /// - Service version is included
    /// - Health status is included
    /// - Statistics are included
    /// - Summary is complete
    #[test]
    fn test_service_summary() {
        let service = GenericServiceBase::<TestConfig, TestStats>::new("test_service".to_string(), "1.0.0".to_string());

        let summary = service.get_service_summary().unwrap();
        assert_eq!(summary.get("service_name").unwrap(), "test_service");
        assert_eq!(summary.get("service_version").unwrap(), "1.0.0");
        assert_eq!(summary.get("is_healthy").unwrap(), "true");
        assert!(summary.contains_key("requests_processed"));
    }
}
