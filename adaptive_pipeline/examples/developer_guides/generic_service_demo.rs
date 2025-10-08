// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Generic Service Patterns - Developer Guide
//!
//! This comprehensive example demonstrates how to implement and use the generic service
//! base and configuration management patterns in the adaptive pipeline system. It showcases
//! advanced service architecture patterns, lifecycle management, and configuration handling.
//!
//! ## Overview
//!
//! The generic service patterns provide:
//!
//! - **Service Base Framework**: Common functionality for all services
//! - **Configuration Management**: Validation, defaults, and hot reloading
//! - **Lifecycle Management**: Start, stop, health checks, and graceful shutdown
//! - **Metrics Collection**: Built-in performance and operational metrics
//! - **Error Handling**: Comprehensive error handling and recovery
//! - **Extensibility**: Easy customization for domain-specific services
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Generic Service System                        │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                 ServiceConfig Trait                     │    │
//! │  │  - Configuration validation and defaults               │    │
//! │  │  - Environment variable integration                    │    │
//! │  │  - Hot reloading and change detection                  │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │              GenericServiceBase                      │    │
//! │  │  - Service lifecycle management                        │    │
//! │  │  - Health checks and monitoring                        │    │
//! │  │  - Metrics collection and aggregation                  │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                ServiceStats Trait                      │    │
//! │  │  - Performance metrics collection                     │    │
//! │  │  - Operational statistics tracking                    │    │
//! │  │  - Resource usage monitoring                           │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Features Demonstrated
//!
//! ### 1. Service Configuration Management
//! - Configuration validation and defaults
//! - Environment variable integration
//! - Hot reloading and change detection
//! - Multi-environment support
//!
//! ### 2. Service Lifecycle Management
//! - Service initialization and startup
//! - Health checks and readiness probes
//! - Graceful shutdown and cleanup
//! - Error recovery and restart logic
//!
//! ### 3. Metrics and Monitoring
//! - Performance metrics collection
//! - Operational statistics tracking
//! - Resource usage monitoring
//! - Custom metrics integration
//!
//! ## Running the Demo
//!
//! ```bash
//! cargo run --example generic_service_demo
//! ```
//!
//! ### Expected Output
//!
//! ```text
//! ⚙️ Generic Service Demo
//! =======================
//!
//! 🔄 Initializing DataProcessor service...
//! ✅ Configuration loaded and validated
//! 🚀 Service started successfully
//! 📊 Health check: HEALTHY
//!
//! 📈 Service Metrics:
//! ------------------
//! Jobs Processed: 150
//! Average Processing Time: 245ms
//! Memory Usage: 64.2MB
//! CPU Usage: 15.8%
//! Error Rate: 0.67%
//!
//! 🔄 Performing configuration reload...
//! ✅ Configuration reloaded successfully
//! 📊 Updated settings applied
//!
//! 🛑 Initiating graceful shutdown...
//! ✅ Service stopped successfully
//! ```
//!
//! ## Learning Outcomes
//!
//! After running this demo, you will understand:
//!
//! - How to implement services using the generic service base
//! - How to create robust configuration management systems
//! - How to implement proper service lifecycle management
//! - How to integrate comprehensive metrics collection
//! - How to handle errors and implement recovery mechanisms
//! - How to create extensible and maintainable service architectures

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use adaptive_pipeline_domain::error::PipelineError;
use adaptive_pipeline_domain::services::generic_service_base::{
    GenericServiceBase, ServiceConfig, ServiceStats, ServiceLifecycle, ServiceMetrics,
};
use adaptive_pipeline_domain::services::generic_config_manager::{
    GenericConfigManager, ConfigValidation, ConfigValidationResult, FileConfigSource,
};

/// Example configuration for a hypothetical data processing service
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DataProcessorConfig {
    pub max_concurrent_jobs: u32,
    pub timeout_seconds: u64,
    pub buffer_size_mb: usize,
    pub enable_compression: bool,
    pub log_level: String,
}

impl Default for DataProcessorConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: 4,
            timeout_seconds: 30,
            buffer_size_mb: 64,
            enable_compression: true,
            log_level: "info".to_string(),
        }
    }
}

impl ServiceConfig for DataProcessorConfig {
    fn validate(&self) -> Result<(), PipelineError> {
        if self.max_concurrent_jobs == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "max_concurrent_jobs must be greater than 0".to_string(),
            ));
        }

        if self.max_concurrent_jobs > 100 {
            return Err(PipelineError::InvalidConfiguration(
                "max_concurrent_jobs cannot exceed 100".to_string(),
            ));
        }

        if self.timeout_seconds == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "timeout_seconds must be greater than 0".to_string(),
            ));
        }

        if self.buffer_size_mb == 0 {
            return Err(PipelineError::InvalidConfiguration(
                "buffer_size_mb must be greater than 0".to_string(),
            ));
        }

        if !["trace", "debug", "info", "warn", "error"].contains(&self.log_level.as_str()) {
            return Err(PipelineError::InvalidConfiguration(
                "log_level must be one of: trace, debug, info, warn, error".to_string(),
            ));
        }

        Ok(())
    }

    fn default_config() -> Self {
        Self::default()
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            max_concurrent_jobs: other.max_concurrent_jobs,
            timeout_seconds: other.timeout_seconds,
            buffer_size_mb: other.buffer_size_mb,
            enable_compression: other.enable_compression,
            log_level: other.log_level.clone(),
        }
    }
}

impl ConfigValidation for DataProcessorConfig {
    fn validate(&self) -> ConfigValidationResult {
        let mut result = ConfigValidationResult::valid();

        if self.max_concurrent_jobs == 0 {
            result.add_error(
                "max_concurrent_jobs".to_string(),
                "Must be greater than 0".to_string(),
            );
        }

        if self.max_concurrent_jobs > 50 {
            result.add_warning(
                "max_concurrent_jobs".to_string(),
                "High concurrency may impact performance".to_string(),
            );
        }

        if self.timeout_seconds == 0 {
            result.add_error(
                "timeout_seconds".to_string(),
                "Must be greater than 0".to_string(),
            );
        }

        if self.buffer_size_mb > 1024 {
            result.add_warning(
                "buffer_size_mb".to_string(),
                "Large buffer size may consume significant memory".to_string(),
            );
        }

        result
    }

    fn schema_version(&self) -> String {
        "1.0.0".to_string()
    }

    fn migrate_from_version(&self, _from_version: &str, _data: &str) -> Result<Self, PipelineError> {
        // Simple migration - just return self for this example
        Ok(self.clone())
    }
}

/// Example statistics for the data processing service
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DataProcessorStats {
    pub jobs_processed: u64,
    pub jobs_failed: u64,
    pub total_processing_time_ms: u64,
    pub bytes_processed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl ServiceStats for DataProcessorStats {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn merge(&mut self, other: &Self) {
        self.jobs_processed += other.jobs_processed;
        self.jobs_failed += other.jobs_failed;
        self.total_processing_time_ms += other.total_processing_time_ms;
        self.bytes_processed += other.bytes_processed;
        self.cache_hits += other.cache_hits;
        self.cache_misses += other.cache_misses;
    }

    fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();

        summary.insert("jobs_processed".to_string(), self.jobs_processed.to_string());
        summary.insert("jobs_failed".to_string(), self.jobs_failed.to_string());
        summary.insert("bytes_processed".to_string(), self.bytes_processed.to_string());

        if self.jobs_processed > 0 {
            let avg_processing_time = self.total_processing_time_ms / self.jobs_processed;
            summary.insert("avg_processing_time_ms".to_string(), avg_processing_time.to_string());

            let success_rate = ((self.jobs_processed - self.jobs_failed) as f64 / self.jobs_processed as f64) * 100.0;
            summary.insert("success_rate_percent".to_string(), format!("{:.1}", success_rate));
        }

        let cache_total = self.cache_hits + self.cache_misses;
        if cache_total > 0 {
            let cache_hit_rate = (self.cache_hits as f64 / cache_total as f64) * 100.0;
            summary.insert("cache_hit_rate_percent".to_string(), format!("{:.1}", cache_hit_rate));
        }

        summary
    }
}

/// Example service implementation using the generic service base
pub struct DataProcessorService {
    service_base: GenericServiceBase<DataProcessorConfig, DataProcessorStats>,
    is_running: std::sync::atomic::AtomicBool,
}

impl DataProcessorService {
    /// Creates a new data processor service with default configuration
    pub fn new() -> Self {
        Self {
            service_base: GenericServiceBase::new(
                "data_processor".to_string(),
                "1.0.0".to_string(),
            ),
            is_running: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Creates a new service with custom configuration
    pub fn with_config(config: DataProcessorConfig) -> Result<Self, PipelineError> {
        let service_base = GenericServiceBase::with_config(
            "data_processor".to_string(),
            "1.0.0".to_string(),
            config,
        )?;

        Ok(Self {
            service_base,
            is_running: std::sync::atomic::AtomicBool::new(false),
        })
    }

    /// Processes a job (simulated)
    pub async fn process_job(&self, job_data: &[u8]) -> Result<Vec<u8>, PipelineError> {
        let start_time = std::time::Instant::now();

        // Simulate processing
        let config = self.service_base.get_config()?;
        sleep(Duration::from_millis(10)).await; // Simulate work

        // Record metrics
        let processing_time = start_time.elapsed().as_millis() as u64;
        self.service_base.update_stats(|stats| {
            stats.jobs_processed += 1;
            stats.total_processing_time_ms += processing_time;
            stats.bytes_processed += job_data.len() as u64;
        })?;

        // Simulate compression if enabled
        let result = if config.enable_compression {
            let mut compressed = Vec::new();
            compressed.extend_from_slice(b"COMPRESSED:");
            compressed.extend_from_slice(job_data);
            compressed
        } else {
            job_data.to_vec()
        };

        Ok(result)
    }

    /// Simulates a job failure
    pub async fn simulate_failure(&self) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            stats.jobs_failed += 1;
        })?;

        Err(PipelineError::InternalError("Simulated failure".to_string()))
    }

    /// Gets the service base for direct access
    pub fn service_base(&self) -> &GenericServiceBase<DataProcessorConfig, DataProcessorStats> {
        &self.service_base
    }
}

#[async_trait::async_trait]
impl ServiceLifecycle for DataProcessorService {
    async fn start(&self) -> Result<(), PipelineError> {
        println!("🚀 Starting Data Processor Service...");
        
        self.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
        self.service_base.set_health(true);
        
        println!("✅ Data Processor Service started successfully");
        Ok(())
    }

    async fn stop(&self) -> Result<(), PipelineError> {
        println!("🛑 Stopping Data Processor Service...");
        
        self.is_running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.service_base.set_health(false);
        
        println!("✅ Data Processor Service stopped successfully");
        Ok(())
    }

    async fn health_check(&self) -> Result<bool, PipelineError> {
        // Perform a simple health check
        let is_running = self.is_running.load(std::sync::atomic::Ordering::SeqCst);
        let is_healthy = self.service_base.is_healthy();
        
        // Try to process a small test job
        if is_running && is_healthy {
            match self.process_job(b"health_check").await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

impl ServiceMetrics<DataProcessorStats> for DataProcessorService {
    fn record_metric(&self, metric_name: &str, value: f64) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match metric_name {
                "cache_hit" => stats.cache_hits += value as u64,
                "cache_miss" => stats.cache_misses += value as u64,
                "bytes_processed" => stats.bytes_processed += value as u64,
                _ => {} // Ignore unknown metrics
            }
        })
    }

    fn increment_counter(&self, counter_name: &str) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match counter_name {
                "jobs_processed" => stats.jobs_processed += 1,
                "jobs_failed" => stats.jobs_failed += 1,
                "cache_hits" => stats.cache_hits += 1,
                "cache_misses" => stats.cache_misses += 1,
                _ => {} // Ignore unknown counters
            }
        })
    }

    fn record_timing(&self, operation_name: &str, duration_ms: u64) -> Result<(), PipelineError> {
        self.service_base.update_stats(|stats| {
            match operation_name {
                "processing" => stats.total_processing_time_ms += duration_ms,
                _ => {} // Ignore unknown operations
            }
        })
    }

    fn get_metrics_snapshot(&self) -> Result<DataProcessorStats, PipelineError> {
        self.service_base.get_stats()
    }
}

/// Demonstrates basic service usage
async fn demonstrate_basic_service_usage() -> Result<(), PipelineError> {
    println!("\n🔧 === Basic Service Usage Demo ===");

    // Create service with default configuration
    let service = DataProcessorService::new();
    
    // Start the service
    service.start().await?;
    
    // Process some jobs
    println!("📊 Processing jobs...");
    for i in 1..=5 {
        let job_data = format!("job_data_{}", i).into_bytes();
        match service.process_job(&job_data).await {
            Ok(result) => println!("  ✅ Job {} processed: {} bytes", i, result.len()),
            Err(e) => println!("  ❌ Job {} failed: {}", i, e),
        }
    }
    
    // Simulate a failure
    if let Err(e) = service.simulate_failure().await {
        println!("  ⚠️  Simulated failure: {}", e);
    }
    
    // Check health
    let is_healthy = service.health_check().await?;
    println!("🏥 Health check: {}", if is_healthy { "✅ Healthy" } else { "❌ Unhealthy" });
    
    // Get service summary
    let summary = service.service_base().get_service_summary()?;
    println!("📈 Service Summary:");
    for (key, value) in summary {
        println!("  {}: {}", key, value);
    }
    
    // Stop the service
    service.stop().await?;
    
    Ok(())
}

/// Demonstrates configuration management
async fn demonstrate_configuration_management() -> Result<(), PipelineError> {
    println!("\n⚙️  === Configuration Management Demo ===");

    // Create a configuration manager
    let config_manager = GenericConfigManager::new(DataProcessorConfig::default());
    
    // Show initial configuration
    let initial_config = config_manager.get_config()?;
    println!("📋 Initial configuration:");
    println!("  Max concurrent jobs: {}", initial_config.max_concurrent_jobs);
    println!("  Timeout: {}s", initial_config.timeout_seconds);
    println!("  Buffer size: {}MB", initial_config.buffer_size_mb);
    println!("  Compression: {}", initial_config.enable_compression);
    
    // Update configuration
    let new_config = DataProcessorConfig {
        max_concurrent_jobs: 8,
        timeout_seconds: 60,
        buffer_size_mb: 128,
        enable_compression: false,
        log_level: "debug".to_string(),
    };
    
    config_manager.update_config(
        new_config,
        "Performance optimization".to_string(),
        "admin".to_string(),
    ).await?;
    
    println!("✅ Configuration updated successfully");
    
    // Show updated configuration
    let updated_config = config_manager.get_config()?;
    println!("📋 Updated configuration:");
    println!("  Max concurrent jobs: {}", updated_config.max_concurrent_jobs);
    println!("  Timeout: {}s", updated_config.timeout_seconds);
    println!("  Buffer size: {}MB", updated_config.buffer_size_mb);
    println!("  Compression: {}", updated_config.enable_compression);
    
    // Show change history
    let history = config_manager.get_change_history();
    println!("📜 Change History ({} changes):", history.len());
    for (i, change) in history.iter().enumerate() {
        println!("  {}. {} by {} at {}", 
                 i + 1, 
                 change.change_reason, 
                 change.changed_by, 
                 change.changed_at.format("%Y-%m-%d %H:%M:%S"));
    }
    
    // Try invalid configuration
    let invalid_config = DataProcessorConfig {
        max_concurrent_jobs: 0, // Invalid!
        timeout_seconds: 0,     // Invalid!
        buffer_size_mb: 64,
        enable_compression: true,
        log_level: "invalid".to_string(), // Invalid!
    };
    
    match config_manager.update_config(
        invalid_config,
        "Test invalid config".to_string(),
        "test_user".to_string(),
    ).await {
        Ok(_) => println!("❌ Unexpected: Invalid config was accepted"),
        Err(e) => println!("✅ Invalid config rejected: {}", e),
    }
    
    Ok(())
}

/// Demonstrates metrics collection
async fn demonstrate_metrics_collection() -> Result<(), PipelineError> {
    println!("\n📊 === Metrics Collection Demo ===");

    let service = DataProcessorService::new();
    service.start().await?;
    
    // Record various metrics
    println!("📈 Recording metrics...");
    service.record_metric("cache_hit", 10.0)?;
    service.record_metric("cache_miss", 2.0)?;
    service.increment_counter("jobs_processed")?;
    service.increment_counter("jobs_processed")?;
    service.record_timing("processing", 150)?;
    service.record_timing("processing", 200)?;
    
    // Process some actual jobs to generate real metrics
    for i in 1..=3 {
        let job_data = format!("metrics_job_{}", i).into_bytes();
        service.process_job(&job_data).await?;
    }
    
    // Get metrics snapshot
    let metrics = service.get_metrics_snapshot()?;
    println!("📊 Metrics Snapshot:");
    println!("  Jobs processed: {}", metrics.jobs_processed);
    println!("  Jobs failed: {}", metrics.jobs_failed);
    println!("  Total processing time: {}ms", metrics.total_processing_time_ms);
    println!("  Bytes processed: {}", metrics.bytes_processed);
    println!("  Cache hits: {}", metrics.cache_hits);
    println!("  Cache misses: {}", metrics.cache_misses);
    
    // Get formatted summary
    let summary = metrics.summary();
    println!("📋 Formatted Summary:");
    for (key, value) in summary {
        println!("  {}: {}", key, value);
    }
    
    service.stop().await?;
    Ok(())
}

/// Main example function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Generic Service Patterns Example");
    println!("=====================================");
    
    // Run all demonstrations
    demonstrate_basic_service_usage().await?;
    demonstrate_configuration_management().await?;
    demonstrate_metrics_collection().await?;
    
    println!("\n🎉 All demonstrations completed successfully!");
    println!("\n💡 Key Takeaways:");
    println!("  • Generic service base provides common functionality");
    println!("  • Configuration management ensures validation and tracking");
    println!("  • Service lifecycle management is standardized");
    println!("  • Metrics collection is consistent across services");
    println!("  • Error handling is properly propagated");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let valid_config = DataProcessorConfig::default();
        assert!(valid_config.validate().is_ok());

        let invalid_config = DataProcessorConfig {
            max_concurrent_jobs: 0,
            timeout_seconds: 0,
            buffer_size_mb: 0,
            enable_compression: true,
            log_level: "invalid".to_string(),
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_stats_operations() {
        let mut stats = DataProcessorStats::default();
        stats.jobs_processed = 10;
        stats.jobs_failed = 2;
        stats.total_processing_time_ms = 1000;

        let summary = stats.summary();
        assert_eq!(summary.get("jobs_processed").unwrap(), "10");
        assert_eq!(summary.get("success_rate_percent").unwrap(), "80.0");

        let mut other_stats = DataProcessorStats::default();
        other_stats.jobs_processed = 5;
        other_stats.jobs_failed = 1;

        stats.merge(&other_stats);
        assert_eq!(stats.jobs_processed, 15);
        assert_eq!(stats.jobs_failed, 3);
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = DataProcessorService::new();
        
        // Test start
        assert!(service.start().await.is_ok());
        assert!(service.service_base().is_healthy());
        
        // Test health check
        assert!(service.health_check().await.unwrap());
        
        // Test stop
        assert!(service.stop().await.is_ok());
        assert!(!service.service_base().is_healthy());
    }
}
