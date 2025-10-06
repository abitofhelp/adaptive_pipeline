// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Stage Executor Interface
//!
//! This module defines the interface for executing pipeline stages and managing
//! the computational resources required for stage processing operations.
//!
//! ## Overview
//!
//! The `StageExecutor` trait provides a standardized interface for:
//!
//! - **Stage Execution**: Process file chunks through pipeline stages
//! - **Parallel Processing**: Execute stages on multiple chunks concurrently
//! - **Resource Management**: Estimate and manage computational resources
//! - **Validation**: Verify stage compatibility and configuration
//! - **Lifecycle Management**: Handle stage preparation and cleanup
//!
//! ## Architecture
//!
//!
//! ## Stage Execution Model
//!
//! ### Single Chunk Processing
//! Process individual file chunks through stages:
//!
//!
//! ### Parallel Processing
//! Process multiple chunks concurrently for improved performance:
//!
//!
//! ## Resource Management
//!
//! ### Resource Estimation
//! Estimate computational resources before execution:

//!
//! ### Resource Requirements Configuration
//!
//! ## Stage Validation
//!
//! ### Compatibility Checking
//!
//! ## Stage Lifecycle Management
//!
//! ### Preparation and Cleanup
//!
//! ## Implementation Guidelines
//!
//! ### Custom Stage Executor
//     supported_stages: HashMap<String, Box<dyn std::fmt::Display>>,
//     resource_estimator: Box<dyn std::fmt::Display>,
// }
//
// #[async_trait]
// impl StageExecutor for CustomStageExecutor {
//     fn execute(
//         &self,
//         stage: &str,
//         chunk: String,
//         context: String,
//     ) -> String {
//         // 1. Validate stage
//         self.validate_configuration(stage).unwrap();
//
//         // 2. Get appropriate processor
//         let processor = self.get_stage_processor(stage).unwrap();
//
//         // 3. Execute processing
//         let start_time = std::time::Instant::now();
//         let result = processor.process(chunk, stage.configuration(),
// context).unwrap();
//
//         // 4. Update metrics
//         let duration = start_time.elapsed();
//         context.record_stage_execution(stage.name(), duration,
// result.data().len());
//
//         Ok(result)
//     }
//
//     fn execute_parallel(
//         &self,
//         stage: &str,
//         chunks: String,
//         context: String,
//     ) -> Result<String, String> {
//         use futures::future::try_join_all;
//
//         // Execute chunks in parallel
//         let futures = chunks.into_iter().map(|chunk| {
//             let stage = stage.clone();
//             let mut chunk_context = context.clone();
//             move {
//                 self.execute(&stage, chunk, &mut chunk_context)
//             }
//         });
//
//         try_join_all(futures).unwrap()
//     }
//
//     // ... implement other methods ...
// }
// ```
// 
// ## Error Handling
//
// Stage executors should handle various error conditions:
//
// - **Configuration Errors**: Invalid stage parameters
// - **Resource Exhaustion**: Insufficient memory or CPU
// - **Processing Failures**: Stage-specific processing errors
// - **Timeout Errors**: Operations exceeding time limits
// - **Compatibility Issues**: Unsupported stage types
//
// ## Performance Considerations
//
// ### Optimization Strategies
// - **Parallel Processing**: Utilize multiple CPU cores
// - **Memory Management**: Efficient chunk processing
// - **Resource Pooling**: Reuse expensive resources
// - **Caching**: Cache computation results when possible
// - **Streaming**: Process data in streams for large files
//
// ### Monitoring and Metrics
// - Track processing times per stage type
// - Monitor resource utilization
// - Measure throughput and latency
// - Alert on error rates and failures

use crate::{FileChunk, PipelineError, PipelineStage, ProcessingContext};
use async_trait::async_trait;

/// Interface for executing pipeline stages on file chunks
///
/// This trait defines the contract for stage execution engines that process
/// file chunks through various pipeline stages. Implementations handle the
/// actual transformation logic for different stage types.
///
/// # Thread Safety
///
/// Implementations must be thread-safe (`Send + Sync`) to support
/// concurrent execution across multiple threads.
///
/// # Design Principles
///
/// - **Async-First**: All operations are asynchronous for scalability
/// - **Resource Aware**: Provides resource estimation and management
/// - **Extensible**: Supports custom stage types through configuration
/// - **Robust**: Comprehensive error handling and validation
#[async_trait]
pub trait StageExecutor: Send + Sync {
    /// Executes a stage on a file chunk
    async fn execute(
        &self,
        stage: &PipelineStage,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Executes a stage on multiple chunks in parallel
    async fn execute_parallel(
        &self,
        stage: &PipelineStage,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Validates if a stage can be executed
    async fn can_execute(&self, stage: &PipelineStage) -> Result<bool, PipelineError>;

    /// Gets the supported stage types
    fn supported_stage_types(&self) -> Vec<String>;

    /// Estimates processing time for a stage
    async fn estimate_processing_time(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<std::time::Duration, PipelineError>;

    /// Gets resource requirements for a stage
    async fn get_resource_requirements(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<ResourceRequirements, PipelineError>;

    /// Prepares a stage for execution (initialization)
    async fn prepare_stage(&self, stage: &PipelineStage, context: &ProcessingContext) -> Result<(), PipelineError>;

    /// Cleans up after stage execution
    async fn cleanup_stage(&self, stage: &PipelineStage, context: &ProcessingContext) -> Result<(), PipelineError>;

    /// Validates stage configuration
    async fn validate_configuration(&self, stage: &PipelineStage) -> Result<(), PipelineError>;

    /// Validates that stages are ordered correctly based on their positions.
    ///
    /// Ensures that PreBinary stages come before PostBinary stages in the pipeline.
    /// Stages with position Any can appear anywhere.
    ///
    /// # Arguments
    ///
    /// * `stages` - The ordered sequence of pipeline stages to validate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Stages are properly ordered
    /// * `Err(PipelineError)` - Stages are misordered or position cannot be determined
    ///
    /// # Examples
    ///
    /// ```text
    /// Valid ordering:
    ///   [PreBinary] -> [PreBinary] -> [PostBinary] -> [PostBinary]
    ///   [PreBinary] -> [Any] -> [PostBinary]
    ///
    /// Invalid ordering:
    ///   [PostBinary] -> [PreBinary]  // PreBinary after PostBinary
    /// ```
    async fn validate_stage_ordering(&self, stages: &[PipelineStage]) -> Result<(), PipelineError>;
}

/// Resource requirements for stage execution
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub memory_bytes: u64,
    pub cpu_cores: u32,
    pub disk_space_bytes: u64,
    pub network_bandwidth_bps: Option<u64>,
    pub gpu_memory_bytes: Option<u64>,
    pub estimated_duration: std::time::Duration,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            memory_bytes: 64 * 1024 * 1024, // 64MB default
            cpu_cores: 1,
            disk_space_bytes: 0,
            network_bandwidth_bps: None,
            gpu_memory_bytes: None,
            estimated_duration: std::time::Duration::from_secs(1),
        }
    }
}

impl ResourceRequirements {
    /// Creates new resource requirements
    pub fn new(memory_bytes: u64, cpu_cores: u32, disk_space_bytes: u64) -> Self {
        Self {
            memory_bytes,
            cpu_cores,
            disk_space_bytes,
            ..Default::default()
        }
    }

    /// Sets network bandwidth requirement
    pub fn with_network_bandwidth(mut self, bandwidth_bps: u64) -> Self {
        self.network_bandwidth_bps = Some(bandwidth_bps);
        self
    }

    /// Sets GPU memory requirement
    pub fn with_gpu_memory(mut self, gpu_memory_bytes: u64) -> Self {
        self.gpu_memory_bytes = Some(gpu_memory_bytes);
        self
    }

    /// Sets estimated duration
    pub fn with_duration(mut self, duration: std::time::Duration) -> Self {
        self.estimated_duration = duration;
        self
    }

    /// Scales requirements by a factor
    pub fn scale(&mut self, factor: f64) {
        self.memory_bytes = (self.memory_bytes as f64 * factor) as u64;
        self.disk_space_bytes = (self.disk_space_bytes as f64 * factor) as u64;

        if let Some(bandwidth) = self.network_bandwidth_bps {
            self.network_bandwidth_bps = Some((bandwidth as f64 * factor) as u64);
        }

        if let Some(gpu_memory) = self.gpu_memory_bytes {
            self.gpu_memory_bytes = Some((gpu_memory as f64 * factor) as u64);
        }

        self.estimated_duration = std::time::Duration::from_secs_f64(self.estimated_duration.as_secs_f64() * factor);
    }

    /// Merges with another resource requirement (takes maximum)
    pub fn merge(&mut self, other: &ResourceRequirements) {
        self.memory_bytes = self.memory_bytes.max(other.memory_bytes);
        self.cpu_cores = self.cpu_cores.max(other.cpu_cores);
        self.disk_space_bytes = self.disk_space_bytes.max(other.disk_space_bytes);

        self.network_bandwidth_bps = match (self.network_bandwidth_bps, other.network_bandwidth_bps) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        self.gpu_memory_bytes = match (self.gpu_memory_bytes, other.gpu_memory_bytes) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        self.estimated_duration = self.estimated_duration.max(other.estimated_duration);
    }
}
