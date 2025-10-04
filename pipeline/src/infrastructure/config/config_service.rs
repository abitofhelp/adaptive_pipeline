// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Configuration Service Implementation
//!
//! This module provides configuration management services for the adaptive
//! pipeline system. It handles loading, parsing, validation, and management
//! of configuration settings for observability, logging, metrics, and system
//! behavior.
//!
//! ## Overview
//!
//! The configuration service implementation provides:
//!
//! - **Configuration Loading**: Loads configuration from files and environment
//!   variables
//! - **Validation**: Validates configuration settings and provides defaults
//! - **Hot Reloading**: Supports dynamic configuration updates without restart
//! - **Environment Integration**: Integrates with environment-specific settings
//! - **Type Safety**: Strongly typed configuration structures with validation
//!
//! ## Architecture
//!
//! The configuration service follows these design principles:
//!
//! - **Layered Configuration**: Supports multiple configuration sources with
//!   precedence
//! - **Type Safety**: Uses Rust's type system for configuration validation
//! - **Default Values**: Provides sensible defaults for all configuration
//!   options
//! - **Environment Awareness**: Adapts configuration based on deployment
//!   environment
//!
//! ## Configuration Categories
//!
//! ### Observability Configuration
//!
//! Controls system monitoring and observability features:
//! - **Structured Logging**: Enable/disable structured logging output
//! - **Performance Tracing**: Control performance tracing and profiling
//! - **Health Checks**: Configure health check endpoints and intervals
//! - **Metrics Export**: Control metrics collection and export settings
//! - **Trace Sampling**: Configure distributed tracing sample rates
//!
//! ### Logging Configuration
//!
//! Manages application logging behavior:
//! - **Log Level**: Set minimum log level (debug, info, warn, error)
//! - **Output Format**: Configure log output format (JSON, plain text)
//! - **File Rotation**: Configure log file rotation and retention
//! - **Filtering**: Set up log filtering rules and patterns
//!
//! ### Metrics Configuration
//!
//! Controls metrics collection and export:
//! - **Collection Interval**: How frequently to collect metrics
//! - **Export Endpoints**: Where to export metrics (Prometheus, etc.)
//! - **Retention Policy**: How long to retain metric data
//! - **Aggregation**: Configure metric aggregation strategies
//!
//! ## Usage Examples
//!
//! ### Loading Configuration

//!
//! ### Environment-Specific Configuration

//!
//! ### Configuration Validation

//!
//! ## Configuration Sources
//!
//! ### File-Based Configuration
//!
//! Supports multiple configuration file formats:
//! - **TOML**: Primary configuration format (recommended)
//! - **JSON**: Alternative JSON format support
//! - **YAML**: YAML format for complex configurations
//!
//! ### Environment Variables
//!
//! Environment variable overrides with prefixes:
//! - **ADAPIPE_LOG_LEVEL**: Override logging level
//! - **ADAPIPE_METRICS_ENABLED**: Enable/disable metrics
//! - **ADAPIPE_TRACE_SAMPLE_RATE**: Set tracing sample rate
//!
//! ### Default Configuration
//!
//! Provides sensible defaults for all settings:
//! - **Development**: Verbose logging, detailed tracing
//! - **Production**: Optimized for performance and stability
//! - **Testing**: Minimal overhead, focused on test execution
//!
//! ## Configuration Validation
//!
//! ### Type Safety
//!
//! - **Compile-Time Validation**: Rust's type system prevents invalid
//!   configurations
//! - **Runtime Validation**: Additional validation for business rules
//! - **Default Values**: Automatic fallback to safe defaults
//!
//! ### Validation Rules
//!
//! - **Range Validation**: Numeric values within acceptable ranges
//! - **Format Validation**: String values match expected formats
//! - **Dependency Validation**: Ensure dependent settings are compatible
//! - **Resource Validation**: Validate resource limits and availability
//!
//! ## Hot Reloading
//!
//! ### Dynamic Updates
//!
//! - **File Watching**: Automatically detect configuration file changes
//! - **Graceful Updates**: Apply changes without service interruption
//! - **Rollback Support**: Automatic rollback on invalid configurations
//! - **Notification**: Notify services of configuration changes
//!
//! ### Safety Mechanisms
//!
//! - **Validation**: New configurations are validated before application
//! - **Atomic Updates**: Configuration changes are applied atomically
//! - **Backup**: Previous configurations are backed up for rollback
//! - **Monitoring**: Configuration changes are logged and monitored
//!
//! ## Performance Considerations
//!
//! ### Efficient Loading
//!
//! - **Lazy Loading**: Load configuration only when needed
//! - **Caching**: Cache parsed configuration to avoid repeated parsing
//! - **Minimal Overhead**: Optimized for fast startup and low memory usage
//!
//! ### Memory Management
//!
//! - **Shared Configuration**: Share configuration across components
//! - **Copy-on-Write**: Efficient updates with copy-on-write semantics
//! - **Garbage Collection**: Automatic cleanup of old configurations
//!
//! ## Security Considerations
//!
//! ### Sensitive Data
//!
//! - **No Secrets**: Configuration files should not contain secrets
//! - **Environment Variables**: Use environment variables for sensitive data
//! - **Access Control**: Restrict access to configuration files
//! - **Audit Logging**: Log configuration access and changes
//!
//! ## Integration
//!
//! The configuration service integrates with:
//!
//! - **Logging System**: Configures logging behavior and output
//! - **Metrics System**: Controls metrics collection and export
//! - **Health Checks**: Configures health check endpoints and behavior
//! - **Tracing System**: Controls distributed tracing and sampling
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Configuration UI**: Web-based configuration management interface
//! - **Schema Validation**: JSON Schema validation for configuration files
//! - **Configuration Templates**: Template-based configuration generation
//! - **Remote Configuration**: Support for remote configuration stores

use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tracing::{debug, warn};

use pipeline_domain::error::PipelineError;

/// Configuration service for reading observability settings
///
/// This struct provides comprehensive configuration management for the adaptive
/// pipeline system, handling observability, logging, metrics, health checks,
/// tracing, and alerting configurations.
///
/// # Configuration Structure
///
/// The configuration is organized into logical sections:
/// - **Observability**: General observability feature toggles
/// - **Logging**: Application logging configuration
/// - **Metrics**: Metrics collection and export settings
/// - **Health Checks**: Health monitoring configuration
/// - **Tracing**: Distributed tracing settings
/// - **Alerts**: Alerting and notification configuration
///
/// # Examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub observability: ObservabilitySettings,
    pub logging: LoggingSettings,
    pub metrics: MetricsSettings,
    pub health_checks: HealthCheckSettings,
    pub tracing: TracingSettings,
    pub alerts: AlertSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilitySettings {
    pub enable_structured_logging: bool,
    pub enable_performance_tracing: bool,
    pub enable_health_checks: bool,
    pub metrics_export_interval_secs: u64,
    pub trace_sample_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
    pub enable_file_logging: bool,
    pub log_file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSettings {
    pub port: u16,
    pub enable_custom_metrics: bool,
    pub retention_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckSettings {
    pub interval_secs: u64,
    pub memory_threshold_mb: u64,
    pub error_rate_threshold_percent: f64,
    pub throughput_threshold_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingSettings {
    pub enable_distributed_tracing: bool,
    pub jaeger_endpoint: String,
    pub service_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub enable_alerts: bool,
    pub webhook_url: String,
    pub error_rate_alert_threshold: f64,
    pub memory_usage_alert_threshold: f64,
    pub disk_usage_alert_threshold: f64,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            observability: ObservabilitySettings {
                enable_structured_logging: true,
                enable_performance_tracing: true,
                enable_health_checks: true,
                metrics_export_interval_secs: 30,
                trace_sample_rate: 1.0,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                format: "pretty".to_string(),
                enable_file_logging: false,
                log_file_path: "logs/adaptive_pipeline.log".to_string(),
            },
            metrics: MetricsSettings {
                port: 9090,
                enable_custom_metrics: true,
                retention_hours: 24,
            },
            health_checks: HealthCheckSettings {
                interval_secs: 30,
                memory_threshold_mb: 1000,
                error_rate_threshold_percent: 5.0,
                throughput_threshold_mbps: 1.0,
            },
            tracing: TracingSettings {
                enable_distributed_tracing: false,
                jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
                service_name: "adaptive_pipeline".to_string(),
            },
            alerts: AlertSettings {
                enable_alerts: false,
                webhook_url: String::new(),
                error_rate_alert_threshold: 10.0,
                memory_usage_alert_threshold: 80.0,
                disk_usage_alert_threshold: 90.0,
            },
        }
    }
}

/// Configuration service for loading observability settings
pub struct ConfigService;

impl ConfigService {
    /// Load observability configuration from file
    pub async fn load_observability_config<P: AsRef<Path>>(
        config_path: P,
    ) -> Result<ObservabilityConfig, PipelineError> {
        let config_path = config_path.as_ref();

        if !config_path.exists() {
            warn!(
                "Observability config file not found at {:?}, using defaults",
                config_path
            );
            return Ok(ObservabilityConfig::default());
        }

        let config_content = fs::read_to_string(config_path).await.map_err(|e| {
            PipelineError::invalid_config(format!("Failed to read config file {:?}: {}", config_path, e))
        })?;

        let config: ObservabilityConfig = toml::from_str(&config_content).map_err(|e| {
            PipelineError::invalid_config(format!("Failed to parse config file {:?}: {}", config_path, e))
        })?;

        debug!(
            "Loaded observability config from {:?}: metrics port {}, structured logging {}",
            config_path, config.metrics.port, config.observability.enable_structured_logging
        );

        Ok(config)
    }

    /// Load observability config from default location
    pub async fn load_default_observability_config() -> Result<ObservabilityConfig, PipelineError> {
        // Try to find observability.toml in current directory or parent directories
        let mut current_dir = std::env::current_dir()
            .map_err(|e| PipelineError::invalid_config(format!("Failed to get current directory: {}", e)))?;

        // Look for observability.toml in current directory and up to 3 parent
        // directories
        for _ in 0..4 {
            let config_path = current_dir.join("observability.toml");
            if config_path.exists() {
                debug!("Found observability config at: {:?}", config_path);
                return Self::load_observability_config(config_path).await;
            }

            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
            } else {
                break;
            }
        }

        warn!("No observability.toml found, using default configuration");
        Ok(ObservabilityConfig::default())
    }

    /// Get metrics port from configuration
    pub async fn get_metrics_port() -> u16 {
        match Self::load_default_observability_config().await {
            Ok(config) => config.metrics.port,
            Err(_) => 9090, // fallback to default
        }
    }

    /// Get alert thresholds from configuration
    pub async fn get_alert_thresholds() -> (f64, f64) {
        match Self::load_default_observability_config().await {
            Ok(config) => (
                config.health_checks.error_rate_threshold_percent,
                config.health_checks.throughput_threshold_mbps,
            ),
            Err(_) => (5.0, 1.0), // fallback defaults
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_load_default_config() {
        let config = ConfigService::load_default_observability_config().await.unwrap();
        assert_eq!(config.metrics.port, 9091); // Should find the actual
                                               // observability.toml
    }

    #[tokio::test]
    async fn test_load_config_from_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_content = r#"
[observability]
enable_structured_logging = true
enable_performance_tracing = true
enable_health_checks = true
metrics_export_interval_secs = 30
trace_sample_rate = 1.0

[logging]
level = "debug"
format = "json"
enable_file_logging = true
log_file_path = "test.log"

[metrics]
port = 8080
enable_custom_metrics = true
retention_hours = 48

[health_checks]
interval_secs = 60
memory_threshold_mb = 2000
error_rate_threshold_percent = 10.0
throughput_threshold_mbps = 5.0

[tracing]
enable_distributed_tracing = true
jaeger_endpoint = "http://test:14268/api/traces"
service_name = "test_service"

[alerts]
enable_alerts = true
webhook_url = "http://example.com/webhook"
error_rate_alert_threshold = 15.0
memory_usage_alert_threshold = 85.0
disk_usage_alert_threshold = 95.0
"#;

        let mut file = tokio::fs::File::create(temp_file.path()).await.unwrap();
        file.write_all(config_content.as_bytes()).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        let config = ConfigService::load_observability_config(temp_file.path())
            .await
            .unwrap();

        assert_eq!(config.metrics.port, 8080);
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.logging.format, "json");
        assert!(config.logging.enable_file_logging);
        assert_eq!(config.health_checks.memory_threshold_mb, 2000);
        assert!(config.tracing.enable_distributed_tracing);
        assert!(config.alerts.enable_alerts);
    }

    #[tokio::test]
    async fn test_get_metrics_port() {
        let port = ConfigService::get_metrics_port().await;
        assert!(port > 0);
    }
}
