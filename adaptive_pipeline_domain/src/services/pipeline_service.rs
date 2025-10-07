// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Pipeline Service Interface
//!
//! This module defines the domain service interface for pipeline processing
//! operations within the adaptive pipeline system. It provides abstractions for
//! executing processing pipelines, managing pipeline lifecycle, and monitoring
//! execution.
//!
//! ## Overview
//!
//! The pipeline service provides:
//!
//! - **Pipeline Execution**: Execute configured processing pipelines
//! - **Lifecycle Management**: Manage pipeline creation, execution, and cleanup
//! - **Progress Monitoring**: Real-time progress tracking and reporting
//! - **Event Handling**: Comprehensive event system for pipeline operations
//! - **Resource Management**: Efficient resource allocation and management
//!
//! ## Architecture
//!
//! The service follows Domain-Driven Design principles:
//!
//! - **Domain Interface**: `PipelineService` trait defines the contract
//! - **Event System**: Observer pattern for pipeline events
//! - **Processing Context**: Maintains state throughout pipeline execution
//! - **Security Integration**: Integrated security context and validation
//!
//! ## Key Features
//!
//! ### Pipeline Execution
//!
//! - **Stage Orchestration**: Coordinate execution of pipeline stages
//! - **Data Flow**: Manage data flow between processing stages
//! - **Error Handling**: Comprehensive error handling and recovery
//! - **Resource Allocation**: Intelligent resource allocation and cleanup
//!
//! ### Progress Monitoring
//!
//! - **Real-time Updates**: Live progress updates during execution
//! - **Performance Metrics**: Detailed performance metrics and statistics
//! - **Event Notifications**: Comprehensive event system for monitoring
//! - **Throughput Tracking**: Track processing throughput and efficiency
//!
//! ### Security Integration
//!
//! - **Security Context**: Integrated security context validation
//! - **Access Control**: Enforce access control policies
//! - **Audit Logging**: Comprehensive audit trail of operations
//! - **Compliance**: Support for regulatory compliance requirements
//!
//! ## Usage Examples
//!
//! ### Basic Pipeline Execution

//!
//! ### Pipeline with Observer

//!
//! ### Batch Pipeline Processing

//!
//! ## Event System
//!
//! ### Processing Observer
//!
//! The `ProcessingObserver` trait provides hooks for monitoring pipeline
//! execution:
//!
//! - **Chunk Events**: Track individual chunk processing
//! - **Progress Updates**: Receive periodic progress updates
//! - **Lifecycle Events**: Monitor pipeline start and completion
//! - **Error Events**: Handle processing errors and failures
//!
//! ### Event Types
//!
//! - **on_processing_started**: Called when pipeline execution begins
//! - **on_chunk_started**: Called when a chunk begins processing
//! - **on_chunk_completed**: Called when a chunk completes processing
//! - **on_progress_update**: Called periodically with progress information
//! - **on_processing_completed**: Called when pipeline execution completes
//! - **on_error**: Called when errors occur during processing
//!
//! ## Processing Context
//!
//! ### Context Management
//!
//! - **State Tracking**: Track processing state throughout execution
//! - **Resource Management**: Manage resources and cleanup
//! - **Security Context**: Maintain security context and validation
//! - **Metrics Collection**: Collect processing metrics and statistics
//!
//! ### Context Lifecycle
//!
//! - **Initialization**: Initialize context before processing
//! - **Stage Transitions**: Update context between processing stages
//! - **Error Handling**: Maintain context during error conditions
//! - **Cleanup**: Clean up context after processing completion
//!
//! ## Security Integration
//!
//! ### Security Context
//!
//! - **Authentication**: Verify user authentication and authorization
//! - **Access Control**: Enforce access control policies
//! - **Audit Logging**: Log all security-relevant operations
//! - **Compliance**: Support regulatory compliance requirements
//!
//! ### Security Validation
//!
//! - **Input Validation**: Validate all input parameters
//! - **Permission Checks**: Verify permissions for file operations
//! - **Resource Limits**: Enforce resource usage limits
//! - **Threat Detection**: Detect and prevent security threats
//!
//! ## Error Handling
//!
//! ### Error Categories
//!
//! - **Configuration Errors**: Invalid pipeline configuration
//! - **Processing Errors**: Errors during pipeline execution
//! - **Security Errors**: Security violations and access denials
//! - **Resource Errors**: Resource exhaustion and allocation failures
//!
//! ### Recovery Strategies
//!
//! - **Retry Logic**: Automatic retry for transient failures
//! - **Fallback Processing**: Alternative processing strategies
//! - **Partial Results**: Return partial results when possible
//! - **Graceful Degradation**: Graceful handling of service failures
//!
//! ## Performance Considerations
//!
//! ### Execution Optimization
//!
//! - **Parallel Processing**: Parallel execution of pipeline stages
//! - **Resource Pooling**: Efficient resource pooling and reuse
//! - **Memory Management**: Optimized memory usage and cleanup
//! - **I/O Optimization**: Efficient file I/O operations
//!
//! ### Monitoring and Metrics
//!
//! - **Performance Metrics**: Detailed performance monitoring
//! - **Resource Usage**: Track resource utilization
//! - **Bottleneck Detection**: Identify performance bottlenecks
//! - **Optimization Recommendations**: Suggest performance improvements
//!
//! ## Integration
//!
//! The pipeline service integrates with:
//!
//! - **Pipeline Repository**: Load and manage pipeline configurations
//! - **File Processor**: Execute file processing operations
//! - **Security Service**: Validate security context and permissions
//! - **Metrics Service**: Report processing metrics and statistics
//!
//! ## Thread Safety
//!
//! The service interface is designed for thread safety:
//!
//! - **Concurrent Execution**: Safe concurrent pipeline execution
//! - **Shared Resources**: Safe sharing of pipeline resources
//! - **State Management**: Thread-safe state management
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Distributed Execution**: Support for distributed pipeline execution
//! - **Dynamic Scaling**: Automatic scaling based on workload
//! - **Advanced Scheduling**: Sophisticated pipeline scheduling
//! - **Machine Learning**: ML-based optimization and prediction

use crate::entities::security_context::SecurityLevel;
use crate::entities::{Pipeline, ProcessingContext, SecurityContext};
use crate::repositories::stage_executor::ResourceRequirements;
use crate::services::datetime_serde;
use crate::value_objects::{FileChunk, PipelineId};
use crate::{PipelineError, ProcessingMetrics};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

/// Observer trait for pipeline processing events
///
/// This trait defines the interface for observing pipeline processing events,
/// enabling real-time monitoring, progress tracking, and event handling during
/// pipeline execution.
///
/// # Key Features
///
/// - **Event Notifications**: Receive notifications for various pipeline events
/// - **Progress Monitoring**: Track processing progress in real-time
/// - **Performance Metrics**: Access detailed performance metrics
/// - **Error Handling**: Handle processing errors and failures
/// - **Lifecycle Management**: Monitor pipeline lifecycle events
///
/// # Event Types
///
/// - **Chunk Events**: Individual chunk processing start/completion
/// - **Progress Events**: Periodic progress updates with throughput
/// - **Lifecycle Events**: Pipeline start/completion events
/// - **Error Events**: Processing errors and failure notifications
///
/// # Examples
#[async_trait]
pub trait ProcessingObserver: Send + Sync {
    /// Called when a chunk starts processing
    async fn on_chunk_started(&self, chunk_id: u64, size: usize) {}

    /// Called when a chunk completes processing
    async fn on_chunk_completed(&self, chunk_id: u64, duration: std::time::Duration) {}

    /// Called periodically with progress updates
    async fn on_progress_update(&self, _bytes_processed: u64, _total_bytes: u64, _throughput_mbps: f64) {}

    /// Called when processing starts
    async fn on_processing_started(&self, total_bytes: u64) {}

    /// Called when processing completes
    async fn on_processing_completed(
        &self,
        total_duration: std::time::Duration,
        final_metrics: Option<&ProcessingMetrics>,
    ) {
    }
}

/// Configuration for processing a file through a pipeline
///
/// Groups related parameters to avoid excessive function arguments.
/// This context is passed to `PipelineService::process_file`.
#[derive(Clone)]
pub struct ProcessFileContext {
    /// Pipeline identifier
    pub pipeline_id: PipelineId,
    /// Security context for processing
    pub security_context: SecurityContext,
    /// Optional override for number of worker threads
    pub user_worker_override: Option<usize>,
    /// Optional override for channel depth
    pub channel_depth_override: Option<usize>,
    /// Optional observer for progress tracking
    pub observer: Option<Arc<dyn ProcessingObserver>>,
}

impl ProcessFileContext {
    /// Creates a new process file context with the given pipeline ID and
    /// security context
    pub fn new(pipeline_id: PipelineId, security_context: SecurityContext) -> Self {
        Self {
            pipeline_id,
            security_context,
            user_worker_override: None,
            channel_depth_override: None,
            observer: None,
        }
    }

    /// Sets the worker count override
    pub fn with_workers(mut self, workers: usize) -> Self {
        self.user_worker_override = Some(workers);
        self
    }

    /// Sets the channel depth override
    pub fn with_channel_depth(mut self, depth: usize) -> Self {
        self.channel_depth_override = Some(depth);
        self
    }

    /// Sets the progress observer
    pub fn with_observer(mut self, observer: Arc<dyn ProcessingObserver>) -> Self {
        self.observer = Some(observer);
        self
    }
}

/// Domain service for pipeline operations
#[async_trait]
pub trait PipelineService: Send + Sync {
    /// Process a file through the pipeline
    async fn process_file(
        &self,
        input_path: &Path,
        output_path: &Path,
        context: ProcessFileContext,
    ) -> Result<ProcessingMetrics, PipelineError>;

    /// Processes file chunks through a pipeline
    async fn process_chunks(
        &self,
        pipeline: &Pipeline,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Validates a pipeline configuration
    async fn validate_pipeline(&self, pipeline: &Pipeline) -> Result<(), PipelineError>;

    /// Estimates processing time for a pipeline
    async fn estimate_processing_time(
        &self,
        pipeline: &Pipeline,
        file_size: u64,
    ) -> Result<std::time::Duration, PipelineError>;

    /// Gets resource requirements for a pipeline
    async fn get_resource_requirements(
        &self,
        pipeline: &Pipeline,
        file_size: u64,
    ) -> Result<ResourceRequirements, PipelineError>;

    /// Creates an optimized pipeline for a file type
    async fn create_optimized_pipeline(
        &self,
        file_path: &Path,
        requirements: PipelineRequirements,
    ) -> Result<Pipeline, PipelineError>;

    /// Monitors pipeline execution
    async fn monitor_execution(
        &self,
        pipeline_id: PipelineId,
        context: &ProcessingContext,
    ) -> Result<ExecutionStatus, PipelineError>;

    /// Pauses pipeline execution
    async fn pause_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError>;

    /// Resumes pipeline execution
    async fn resume_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError>;

    /// Cancels pipeline execution
    async fn cancel_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError>;

    /// Gets execution history for a pipeline
    async fn get_execution_history(
        &self,
        pipeline_id: PipelineId,
        limit: Option<usize>,
    ) -> Result<Vec<ExecutionRecord>, PipelineError>;
}

/// Requirements for pipeline creation
#[derive(Debug, Clone)]
pub struct PipelineRequirements {
    pub compression_required: bool,
    pub encryption_required: bool,
    pub integrity_required: bool,
    pub performance_priority: PerformancePriority,
    pub security_level: SecurityLevel,
    pub max_memory_usage: Option<u64>,
    pub max_processing_time: Option<std::time::Duration>,
    pub parallel_processing: bool,
}

/// Performance priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum PerformancePriority {
    Speed,
    Compression,
    Security,
    Balanced,
}

/// Pipeline execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    pub pipeline_id: PipelineId,
    pub status: ExecutionState,
    pub progress_percentage: f64,
    pub bytes_processed: u64,
    pub bytes_total: u64,
    pub current_stage: Option<String>,
    pub estimated_remaining_time: Option<std::time::Duration>,
    pub error_count: u64,
    pub warning_count: u64,
    #[serde(with = "datetime_serde")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Pipeline execution states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Pipeline execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub id: PipelineId,
    pub pipeline_id: PipelineId,
    pub input_path: std::path::PathBuf,
    pub output_path: std::path::PathBuf,
    pub status: ExecutionState,
    pub metrics: ProcessingMetrics,
    pub error_message: Option<String>,
    #[serde(with = "datetime_serde")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde::optional")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub security_context: SecurityContext,
}

impl Default for PipelineRequirements {
    fn default() -> Self {
        Self {
            compression_required: false,
            encryption_required: false,
            integrity_required: false,
            performance_priority: PerformancePriority::Balanced,
            security_level: SecurityLevel::Internal,
            max_memory_usage: None,
            max_processing_time: None,
            parallel_processing: true,
        }
    }
}

impl PipelineRequirements {
    /// Creates new pipeline requirements
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets compression requirement
    pub fn with_compression(mut self, required: bool) -> Self {
        self.compression_required = required;
        self
    }

    /// Sets encryption requirement
    pub fn with_encryption(mut self, required: bool) -> Self {
        self.encryption_required = required;
        self
    }

    /// Sets integrity requirement
    pub fn with_integrity(mut self, required: bool) -> Self {
        self.integrity_required = required;
        self
    }

    /// Sets performance priority
    pub fn with_performance_priority(mut self, priority: PerformancePriority) -> Self {
        self.performance_priority = priority;
        self
    }

    /// Sets security level
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    }

    /// Sets maximum memory usage
    pub fn with_max_memory(mut self, max_memory: u64) -> Self {
        self.max_memory_usage = Some(max_memory);
        self
    }

    /// Sets maximum processing time
    pub fn with_max_time(mut self, max_time: std::time::Duration) -> Self {
        self.max_processing_time = Some(max_time);
        self
    }

    /// Sets parallel processing
    pub fn with_parallel_processing(mut self, enabled: bool) -> Self {
        self.parallel_processing = enabled;
        self
    }
}

impl ExecutionStatus {
    /// Creates new execution status
    pub fn new(pipeline_id: PipelineId, bytes_total: u64) -> Self {
        let now = chrono::Utc::now();
        Self {
            pipeline_id,
            status: ExecutionState::Pending,
            progress_percentage: 0.0,
            bytes_processed: 0,
            bytes_total,
            current_stage: None,
            estimated_remaining_time: None,
            error_count: 0,
            warning_count: 0,
            started_at: now,
            updated_at: now,
        }
    }

    /// Updates the execution status
    pub fn update(&mut self, metrics: &ProcessingMetrics, current_stage: Option<String>) {
        self.bytes_processed = metrics.bytes_processed();
        self.progress_percentage = metrics.progress_percentage();
        self.current_stage = current_stage;
        self.estimated_remaining_time = metrics.estimated_remaining_time();
        self.error_count = metrics.error_count();
        self.warning_count = metrics.warning_count();
        self.updated_at = chrono::Utc::now();
    }

    /// Checks if execution is complete
    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            ExecutionState::Completed | ExecutionState::Failed | ExecutionState::Cancelled
        )
    }

    /// Checks if execution is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, ExecutionState::Running | ExecutionState::Paused)
    }
}
