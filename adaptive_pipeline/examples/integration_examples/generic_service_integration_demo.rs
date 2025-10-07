//! # Generic Service Integration Demo
//!
//! This comprehensive integration example demonstrates how to combine all four generic
//! patterns provided by the adaptive pipeline system to create a complete, production-ready
//! service implementation with configuration management, metrics collection, and result building.
//!
//! ## Overview
//!
//! This demo integrates four key generic patterns:
//!
//! - **GenericServiceBase**: Common service functionality and lifecycle management
//! - **GenericConfigManager**: Configuration validation and management
//! - **GenericResultBuilder**: Fluent API for building operation results
//! - **GenericMetricsCollector**: Standardized metrics collection and aggregation
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Complete Service Implementation                   │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                 Generic Service Base                     │    │
//! │  │  - Service lifecycle management                         │    │
//! │  │  - Health checks and monitoring                         │    │
//! │  │  - Common service operations                            │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │               Generic Config Manager                    │    │
//! │  │  - Configuration validation and loading                 │    │
//! │  │  - Environment variable integration                     │    │
//! │  │  - Hot reloading and change detection                   │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │               Generic Result Builder                    │    │
//! │  │  - Fluent API for building operation results           │    │
//! │  │  - Type-safe result construction                        │    │
//! │  │  - Metadata and timing information                      │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │             Generic Metrics Collector                  │    │
//! │  │  - Standardized metrics collection                      │    │
//! │  │  - Automatic aggregation and reporting                  │    │
//! │  │  - Time-based filtering and analysis                    │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Integration Benefits
//!
//! Combining these patterns provides:
//!
//! - **Consistent Architecture**: Standardized patterns across all services
//! - **Reduced Boilerplate**: Common functionality provided by generic implementations
//! - **Type Safety**: Compile-time guarantees for configuration and results
//! - **Observability**: Built-in metrics and monitoring capabilities
//! - **Maintainability**: Clear separation of concerns and responsibilities
//!
//! ## Usage
//!
//! Run the integration demo:
//!
//! ```bash
//! cargo run --example generic_service_integration_demo
//! ```
//!
//! ## Expected Output
//!
//! The demo will show:
//! 1. Service initialization with configuration validation
//! 2. Operation execution with metrics collection
//! 3. Result building with fluent API
//! 4. Metrics aggregation and reporting
//! 5. Service lifecycle management

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Import all the generics we'll be using
use pipeline_domain::{
    PipelineError,
    services::{
        // Generic Service Base
        GenericServiceBase, ServiceConfig, ServiceStats,
        // Generic Config Manager
        GenericConfigManager, ConfigValidation, ConfigValidationResult,
        // Generic Result Builder
        GenericResultBuilder, OperationResult, GenericProcessingResult,
        // Generic Metrics Collector
        GenericMetricsCollector, CollectibleMetrics, MetricsEnabled,
        // Convenience macros
        result_builder, metrics_collector
    }
};

/// Example service configuration that implements ServiceConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleServiceConfig {
    pub max_concurrent_operations: usize,
    pub timeout_seconds: u64,
    pub enable_caching: bool,
    pub cache_size_mb: usize,
    pub retry_attempts: u32,
}

impl ServiceConfig for ExampleServiceConfig {
    fn validate(&self) -> Result<(), PipelineError> {
        if self.max_concurrent_operations == 0 {
            return Err(PipelineError::configuration_error(
                "max_concurrent_operations must be greater than 0"
            ));
        }
        if self.timeout_seconds == 0 {
            return Err(PipelineError::configuration_error(
                "timeout_seconds must be greater than 0"
            ));
        }
        if self.cache_size_mb > 1024 {
            return Err(PipelineError::configuration_error(
                "cache_size_mb cannot exceed 1024MB"
            ));
        }
        Ok(())
    }

    fn default_config() -> Self {
        Self {
            max_concurrent_operations: 4,
            timeout_seconds: 30,
            enable_caching: true,
            cache_size_mb: 64,
            retry_attempts: 3,
        }
    }

    fn merge_with(&mut self, other: &Self) {
        self.max_concurrent_operations = other.max_concurrent_operations;
        self.timeout_seconds = other.timeout_seconds;
        self.enable_caching = other.enable_caching;
        self.cache_size_mb = other.cache_size_mb;
        self.retry_attempts = other.retry_attempts;
    }
}

/// Example service configuration that also implements ConfigValidation for GenericConfigManager
impl ConfigValidation for ExampleServiceConfig {
    fn validate(&self) -> ConfigValidationResult {
        let mut result = ConfigValidationResult::new();
        
        if self.max_concurrent_operations == 0 {
            result.add_error("max_concurrent_operations must be greater than 0");
        } else if self.max_concurrent_operations > 16 {
            result.add_warning("max_concurrent_operations > 16 may cause resource exhaustion");
        }
        
        if self.timeout_seconds == 0 {
            result.add_error("timeout_seconds must be greater than 0");
        } else if self.timeout_seconds > 300 {
            result.add_warning("timeout_seconds > 300 may cause long waits");
        }
        
        if self.cache_size_mb > 1024 {
            result.add_error("cache_size_mb cannot exceed 1024MB");
        } else if self.cache_size_mb < 16 {
            result.add_warning("cache_size_mb < 16 may reduce performance");
        }
        
        result
    }

    fn schema_version(&self) -> String {
        "1.0.0".to_string()
    }

    fn migrate_from_version(&self, from_version: &str, data: &str) -> Result<Self, PipelineError> {
        match from_version {
            "1.0.0" => serde_json::from_str(data)
                .map_err(|e| PipelineError::configuration_error(&format!("Migration failed: {}", e))),
            _ => Err(PipelineError::configuration_error(&format!(
                "Unsupported migration from version {}", from_version
            )))
        }
    }
}

/// Example service statistics that implement ServiceStats
#[derive(Debug, Clone, Default)]
pub struct ExampleServiceStats {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub total_processing_time_ms: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub retry_count: u64,
}

impl ServiceStats for ExampleServiceStats {
    fn reset(&mut self) {
        *self = Self::default();
    }
    
    fn merge(&mut self, other: &Self) {
        self.total_operations += other.total_operations;
        self.successful_operations += other.successful_operations;
        self.failed_operations += other.failed_operations;
        self.total_processing_time_ms += other.total_processing_time_ms;
        self.cache_hits += other.cache_hits;
        self.cache_misses += other.cache_misses;
        self.retry_count += other.retry_count;
    }
    
    fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("total_operations".to_string(), self.total_operations.to_string());
        summary.insert("successful_operations".to_string(), self.successful_operations.to_string());
        summary.insert("failed_operations".to_string(), self.failed_operations.to_string());
        summary.insert("success_rate".to_string(), 
            if self.total_operations > 0 {
                format!("{:.2}%", (self.successful_operations as f64 / self.total_operations as f64) * 100.0)
            } else {
                "0.00%".to_string()
            }
        );
        summary.insert("avg_processing_time_ms".to_string(),
            if self.total_operations > 0 {
                (self.total_processing_time_ms / self.total_operations).to_string()
            } else {
                "0".to_string()
            }
        );
        summary.insert("cache_hit_rate".to_string(),
            if self.cache_hits + self.cache_misses > 0 {
                format!("{:.2}%", (self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64) * 100.0)
            } else {
                "0.00%".to_string()
            }
        );
        summary
    }
}

/// Example metrics that implement CollectibleMetrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExampleMetrics {
    pub operation_count: u64,
    pub bytes_processed: u64,
    pub error_count: u64,
    pub average_latency_ms: f64,
    pub peak_memory_usage_mb: f64,
    pub cache_efficiency: f64,
}

impl CollectibleMetrics for ExampleMetrics {
    fn aggregate(&mut self, other: &Self) {
        let total_ops = self.operation_count + other.operation_count;
        
        // Weighted average for latency
        if total_ops > 0 {
            self.average_latency_ms = (self.average_latency_ms * self.operation_count as f64 + 
                                     other.average_latency_ms * other.operation_count as f64) / total_ops as f64;
        }
        
        self.operation_count = total_ops;
        self.bytes_processed += other.bytes_processed;
        self.error_count += other.error_count;
        self.peak_memory_usage_mb = self.peak_memory_usage_mb.max(other.peak_memory_usage_mb);
        
        // Weighted average for cache efficiency
        if total_ops > 0 {
            self.cache_efficiency = (self.cache_efficiency * self.operation_count as f64 + 
                                   other.cache_efficiency * other.operation_count as f64) / total_ops as f64;
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("operation_count".to_string(), self.operation_count.to_string());
        summary.insert("bytes_processed".to_string(), self.bytes_processed.to_string());
        summary.insert("error_count".to_string(), self.error_count.to_string());
        summary.insert("error_rate".to_string(), 
            if self.operation_count > 0 {
                format!("{:.2}%", (self.error_count as f64 / self.operation_count as f64) * 100.0)
            } else {
                "0.00%".to_string()
            }
        );
        summary.insert("average_latency_ms".to_string(), format!("{:.2}", self.average_latency_ms));
        summary.insert("peak_memory_usage_mb".to_string(), format!("{:.2}", self.peak_memory_usage_mb));
        summary.insert("cache_efficiency".to_string(), format!("{:.2}%", self.cache_efficiency * 100.0));
        summary
    }
}

/// Example operation result type
#[derive(Debug, Clone)]
pub struct ExampleOperationResult {
    pub input_data: String,
    pub output_data: String,
    pub metrics: ExampleMetrics,
}

impl OperationResult for ExampleOperationResult {
    type Input = String;
    type Output = String;
    type Metrics = ExampleMetrics;

    fn new(input: Self::Input, output: Self::Output, metrics: Self::Metrics) -> Self {
        Self {
            input_data: input,
            output_data: output,
            metrics,
        }
    }

    fn input(&self) -> &Self::Input {
        &self.input_data
    }

    fn output(&self) -> &Self::Output {
        &self.output_data
    }

    fn metrics(&self) -> &Self::Metrics {
        &self.metrics
    }

    fn success(&self) -> bool {
        self.metrics.error_count == 0
    }
}

/// Example service that integrates all four generics
pub struct ExampleService {
    // Generic Service Base provides common functionality
    base: GenericServiceBase<ExampleServiceConfig, ExampleServiceStats>,
    
    // Generic Config Manager handles configuration
    config_manager: GenericConfigManager<ExampleServiceConfig>,
    
    // Generic Metrics Collector tracks metrics
    metrics_collector: GenericMetricsCollector<ExampleMetrics>,
    
    // Service-specific state
    cache: HashMap<String, String>,
}

impl ExampleService {
    pub fn new(config: ExampleServiceConfig) -> Result<Self, PipelineError> {
        let base = GenericServiceBase::new("example_service", config.clone())?;
        let config_manager = GenericConfigManager::new(config)?;
        let metrics_collector = metrics_collector!(ExampleMetrics, "example_service");
        
        Ok(Self {
            base,
            config_manager,
            metrics_collector,
            cache: HashMap::new(),
        })
    }

    /// Example operation that demonstrates all generics usage
    pub async fn process_data(&mut self, input: String) -> Result<GenericProcessingResult<String, String, ExampleMetrics>, PipelineError> {
        // Start metrics collection
        let operation_id = format!("process_{}", chrono::Utc::now().timestamp_nanos());
        self.metrics_collector.start_operation(&operation_id, "process_data").await?;
        
        // Use result builder for fluent API
        let mut result_builder = result_builder!(ExampleOperationResult);
        result_builder.with_input(input.clone());
        
        let start_time = Instant::now();
        
        // Simulate processing logic
        let output = match self.simulate_processing(&input).await {
            Ok(output) => {
                // Update service stats
                let mut stats = self.base.stats().write().unwrap();
                stats.successful_operations += 1;
                stats.total_operations += 1;
                stats.total_processing_time_ms += start_time.elapsed().as_millis() as u64;
                
                // Check cache
                if self.cache.contains_key(&input) {
                    stats.cache_hits += 1;
                } else {
                    stats.cache_misses += 1;
                    self.cache.insert(input.clone(), output.clone());
                }
                
                result_builder.with_output(output.clone());
                output
            }
            Err(e) => {
                // Update error stats
                let mut stats = self.base.stats().write().unwrap();
                stats.failed_operations += 1;
                stats.total_operations += 1;
                
                result_builder.with_error(e.clone());
                return Ok(result_builder.build());
            }
        };
        
        // Create metrics for this operation
        let processing_time = start_time.elapsed();
        let mut metrics = ExampleMetrics {
            operation_count: 1,
            bytes_processed: (input.len() + output.len()) as u64,
            error_count: 0,
            average_latency_ms: processing_time.as_millis() as f64,
            peak_memory_usage_mb: 2.5, // Simulated
            cache_efficiency: if self.cache.contains_key(&input) { 1.0 } else { 0.0 },
        };
        
        result_builder.with_metrics(metrics.clone());
        
        // Complete metrics collection
        self.metrics_collector.complete_operation(&operation_id, metrics).await?;
        
        Ok(result_builder.build())
    }
    
    async fn simulate_processing(&self, input: &str) -> Result<String, PipelineError> {
        // Simulate some processing time
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Simulate potential failure
        if input.contains("error") {
            return Err(PipelineError::processing_error("Simulated processing error"));
        }
        
        // Return processed result
        Ok(format!("processed: {}", input.to_uppercase()))
    }
    
    /// Get service health status using GenericServiceBase
    pub fn health_status(&self) -> bool {
        self.base.is_healthy()
    }
    
    /// Get service statistics using GenericServiceBase
    pub fn get_stats(&self) -> HashMap<String, String> {
        self.base.stats().read().unwrap().summary()
    }
    
    /// Get metrics summary using GenericMetricsCollector
    pub async fn get_metrics_summary(&self) -> Result<HashMap<String, String>, PipelineError> {
        Ok(self.metrics_collector.get_summary().await?)
    }
    
    /// Update configuration using GenericConfigManager
    pub async fn update_config(&mut self, new_config: ExampleServiceConfig) -> Result<(), PipelineError> {
        self.config_manager.update_config(new_config.clone()).await?;
        self.base.update_config(new_config)?;
        Ok(())
    }
    
    /// Get configuration validation using GenericConfigManager
    pub fn validate_config(&self, config: &ExampleServiceConfig) -> ConfigValidationResult {
        self.config_manager.validate_config(config)
    }
}

impl MetricsEnabled<ExampleMetrics> for ExampleService {
    fn metrics_collector(&self) -> &GenericMetricsCollector<ExampleMetrics> {
        &self.metrics_collector
    }
}

/// Example usage demonstration
pub async fn run_example() -> Result<(), PipelineError> {
    println!("=== Generic Service Integration Demo ===\n");
    
    // 1. Create service with configuration
    let config = ExampleServiceConfig {
        max_concurrent_operations: 8,
        timeout_seconds: 60,
        enable_caching: true,
        cache_size_mb: 128,
        retry_attempts: 3,
    };
    
    let mut service = ExampleService::new(config)?;
    println!("✅ Service created with configuration");
    
    // 2. Process some data
    let test_inputs = vec![
        "hello world",
        "test data",
        "another example",
        "hello world", // This will hit cache
    ];
    
    for input in test_inputs {
        println!("\n📊 Processing: '{}'", input);
        
        match service.process_data(input.to_string()).await {
            Ok(result) => {
                println!("  ✅ Success: {}", result.output().unwrap_or(&"No output".to_string()));
                println!("  📈 Metrics: {:?}", result.metrics());
                
                if !result.warnings().is_empty() {
                    println!("  ⚠️  Warnings: {:?}", result.warnings());
                }
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
            }
        }
    }
    
    // 3. Show service statistics
    println!("\n📊 Service Statistics:");
    let stats = service.get_stats();
    for (key, value) in stats {
        println!("  {}: {}", key, value);
    }
    
    // 4. Show metrics summary
    println!("\n📈 Metrics Summary:");
    let metrics = service.get_metrics_summary().await?;
    for (key, value) in metrics {
        println!("  {}: {}", key, value);
    }
    
    // 5. Test configuration update
    println!("\n🔧 Testing configuration update...");
    let new_config = ExampleServiceConfig {
        max_concurrent_operations: 16,
        timeout_seconds: 120,
        enable_caching: true,
        cache_size_mb: 256,
        retry_attempts: 5,
    };
    
    let validation = service.validate_config(&new_config);
    if validation.is_valid() {
        service.update_config(new_config).await?;
        println!("  ✅ Configuration updated successfully");
    } else {
        println!("  ❌ Configuration validation failed:");
        for error in validation.errors() {
            println!("    Error: {}", error);
        }
        for warning in validation.warnings() {
            println!("    Warning: {}", warning);
        }
    }
    
    // 6. Test error handling
    println!("\n🧪 Testing error handling...");
    match service.process_data("error test".to_string()).await {
        Ok(result) => {
            if let Some(error) = result.error() {
                println!("  ✅ Error properly captured: {}", error);
            } else {
                println!("  ❌ Expected error but got success");
            }
        }
        Err(e) => {
            println!("  ❌ Unexpected error: {}", e);
        }
    }
    
    println!("\n🎉 Demo completed successfully!");
    println!("\nThis example demonstrated:");
    println!("  ✅ GenericServiceBase - Common service functionality");
    println!("  ✅ GenericConfigManager - Configuration management");
    println!("  ✅ GenericResultBuilder - Fluent result building");
    println!("  ✅ GenericMetricsCollector - Metrics collection");
    println!("  ✅ Full integration of all four generics patterns");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_service_integration() {
        let config = ExampleServiceConfig::default_config();
        let mut service = ExampleService::new(config).unwrap();
        
        let result = service.process_data("test".to_string()).await.unwrap();
        assert!(result.success());
        assert_eq!(result.output().unwrap(), "processed: TEST");
    }
    
    #[test]
    fn test_config_validation() {
        let config = ExampleServiceConfig {
            max_concurrent_operations: 0, // Invalid
            timeout_seconds: 30,
            enable_caching: true,
            cache_size_mb: 64,
            retry_attempts: 3,
        };
        
        let validation = config.validate();
        assert!(!validation.is_valid());
        assert!(!validation.errors().is_empty());
    }
    
    #[test]
    fn test_metrics_aggregation() {
        let mut metrics1 = ExampleMetrics {
            operation_count: 5,
            bytes_processed: 1000,
            error_count: 1,
            average_latency_ms: 10.0,
            peak_memory_usage_mb: 5.0,
            cache_efficiency: 0.8,
        };
        
        let metrics2 = ExampleMetrics {
            operation_count: 3,
            bytes_processed: 600,
            error_count: 0,
            average_latency_ms: 15.0,
            peak_memory_usage_mb: 3.0,
            cache_efficiency: 0.9,
        };
        
        metrics1.aggregate(&metrics2);
        
        assert_eq!(metrics1.operation_count, 8);
        assert_eq!(metrics1.bytes_processed, 1600);
        assert_eq!(metrics1.error_count, 1);
        assert_eq!(metrics1.peak_memory_usage_mb, 5.0); // Max value
    }
}
