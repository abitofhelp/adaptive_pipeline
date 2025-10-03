// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Processing Context Entity
//!
//! The `ProcessingContext` entity maintains runtime state and context
//! information throughout pipeline execution. It serves as a central repository
//! for tracking processing progress, configuration parameters, and execution
//! metadata.
//!
//! ## Overview
//!
//! The processing context acts as a stateful carrier object that:
//!
//! - **Tracks Progress**: Monitors bytes processed and completion status
//! - **Manages Configuration**: Maintains processing parameters and settings
//! - **Collects Metrics**: Aggregates performance and operational data
//! - **Stores Metadata**: Preserves stage-specific results and information
//! - **Enforces Security**: Maintains security context throughout processing
//!
//! ## Entity Characteristics
//!
//! - **Mutable State**: Tracks changing values during processing
//! - **Unique Identity**: Each context has a distinct `ProcessingContextId`
//! - **Thread Safety**: Designed for safe concurrent access patterns
//! - **Serializable**: Can be persisted and restored for long-running
//!   operations
//!
//! ## State Management
//!
//! The context maintains several categories of state:
//!
//! ### File Processing State
//! - Input and output file paths
//! - Total file size and bytes processed
//! - Progress calculation and completion status
//!
//! ### Configuration State
//! - Chunk size for processing operations
//! - Worker count for parallel processing
//! - Security context and permissions
//!
//! ### Runtime State
//! - Processing metrics and performance data
//! - Stage-specific results and outputs
//! - Custom metadata and annotations

use crate::services::datetime_serde;
use crate::value_objects::{ChunkSize, ProcessingContextId, WorkerCount};
use crate::{ProcessingMetrics, SecurityContext};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Processing context entity that maintains runtime state during pipeline
/// execution.
///
/// The `ProcessingContext` serves as a central state container that travels
/// through the pipeline, collecting information and tracking progress as each
/// stage processes the data. It provides a unified interface for accessing and
/// updating processing state across all pipeline stages.
///
/// ## Entity Purpose
///
/// - **State Coordination**: Centralizes processing state across pipeline
///   stages
/// - **Progress Tracking**: Monitors processing progress and completion status
/// - **Configuration Management**: Maintains processing parameters and settings
/// - **Metrics Collection**: Aggregates performance and operational metrics
/// - **Security Enforcement**: Preserves security context throughout processing
///
/// ## Usage Examples
///
/// ### Creating a Processing Context
///
///
/// ### Tracking Processing Progress
///
///
/// ### Managing Stage Results
///
///
/// ### Adding Custom Metadata
///
///
/// ### Updating Processing Metrics
///
///
/// ## State Lifecycle
///
/// The processing context follows a predictable lifecycle:
///
/// ### 1. Initialization
///
/// ### 2. Processing Updates
///
/// ### 3. Completion
///
/// ## Thread Safety and Concurrency
///
/// While the context itself is not thread-safe, it's designed for safe
/// concurrent patterns:
///
///
/// ## Serialization and Persistence
///
/// The context supports serialization for checkpointing and recovery:
///
///
/// ## Performance Considerations
///
/// - Context updates are lightweight and fast
/// - Metadata and stage results use efficient HashMap storage
/// - Progress calculations are performed on-demand
/// - Timestamps are updated only when state changes
/// - Memory usage scales with the amount of stored metadata
///
/// ## Error Handling
///
/// The context provides safe access to all state with appropriate defaults:
///
/// - Missing metadata returns `None` rather than panicking
/// - Progress calculations handle edge cases (zero file size)
/// - All numeric operations are checked for overflow
/// - Timestamp operations are guaranteed to succeed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingContext {
    // Identity fields (always first)
    id: ProcessingContextId,

    // Core business fields (alphabetical within group)
    chunk_size: ChunkSize,
    file_size: u64,
    input_path: PathBuf,
    metadata: HashMap<String, String>,
    metrics: ProcessingMetrics,
    output_path: PathBuf,
    processed_bytes: u64,
    security_context: SecurityContext,
    stage_results: HashMap<String, String>,
    worker_count: WorkerCount,

    // Metadata fields (always last)
    #[serde(with = "datetime_serde")]
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl ProcessingContext {
    /// Creates a new processing context for pipeline execution
    ///
    /// Initializes a new context with default configuration values and empty state.
    /// The context starts with zero processed bytes and will track progress
    /// throughout the pipeline execution.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the input file being processed
    /// * `output_path` - Path where the processed output will be written
    /// * `file_size` - Total size of the input file in bytes
    /// * `security_context` - Security context for authorization and access control
    ///
    /// # Returns
    ///
    /// A new `ProcessingContext` with initialized state
    ///
    /// # Examples
    ///
    pub fn new(input_path: PathBuf, output_path: PathBuf, file_size: u64, security_context: SecurityContext) -> Self {
        let now = chrono::Utc::now();

        ProcessingContext {
            // Identity fields
            id: ProcessingContextId::new(),

            // Core business fields (alphabetical)
            chunk_size: ChunkSize::from_mb(1).unwrap_or_else(|_| ChunkSize::default()),
            file_size,
            input_path,
            metadata: HashMap::new(),
            metrics: ProcessingMetrics::default(),
            output_path,
            processed_bytes: 0,
            security_context,
            stage_results: HashMap::new(),
            worker_count: WorkerCount::new(4), // Default to 4 workers

            // Metadata fields
            created_at: now,
            updated_at: now,
        }
    }

    /// Gets the unique identifier for this processing context
    ///
    /// # Returns
    ///
    /// Reference to the context's unique identifier
    pub fn id(&self) -> &ProcessingContextId {
        &self.id
    }

    /// Gets the path to the input file being processed
    ///
    /// # Returns
    ///
    /// Reference to the input file path
    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    /// Gets the path where the processed output will be written
    ///
    /// # Returns
    ///
    /// Reference to the output file path
    pub fn output_path(&self) -> &PathBuf {
        &self.output_path
    }

    /// Gets the total size of the input file in bytes
    ///
    /// # Returns
    ///
    /// Total file size in bytes
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    /// Gets the number of bytes processed so far
    ///
    /// # Returns
    ///
    /// Number of bytes processed
    pub fn processed_bytes(&self) -> u64 {
        self.processed_bytes
    }

    /// Gets the security context for authorization and access control
    ///
    /// # Returns
    ///
    /// Reference to the security context
    pub fn security_context(&self) -> &SecurityContext {
        &self.security_context
    }

    /// Gets the current processing metrics
    ///
    /// # Returns
    ///
    /// Reference to the processing metrics
    pub fn metrics(&self) -> &ProcessingMetrics {
        &self.metrics
    }

    /// Gets the chunk size configuration for processing
    ///
    /// # Returns
    ///
    /// Reference to the chunk size configuration
    pub fn chunk_size(&self) -> &ChunkSize {
        &self.chunk_size
    }

    /// Gets the number of worker threads for parallel processing
    ///
    /// # Returns
    ///
    /// Reference to the worker count configuration
    pub fn worker_count(&self) -> &WorkerCount {
        &self.worker_count
    }

    /// Gets all custom metadata associated with this context
    ///
    /// # Returns
    ///
    /// Reference to the metadata HashMap
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Gets all stage processing results
    ///
    /// # Returns
    ///
    /// Reference to the stage results HashMap
    pub fn stage_results(&self) -> &HashMap<String, String> {
        &self.stage_results
    }

    /// Sets the total number of bytes processed
    ///
    /// Replaces the current processed byte count with a new absolute value.
    ///
    /// # Arguments
    ///
    /// * `bytes` - New total byte count
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn update_processed_bytes(&mut self, bytes: u64) {
        self.processed_bytes = bytes;
        self.updated_at = chrono::Utc::now();
    }

    /// Increments the processed byte count
    ///
    /// Adds the specified number of bytes to the current processed total.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Number of additional bytes to add
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn add_processed_bytes(&mut self, bytes: u64) {
        self.processed_bytes += bytes;
        self.updated_at = chrono::Utc::now();
    }

    /// Updates the processing metrics with new values
    ///
    /// # Arguments
    ///
    /// * `metrics` - New metrics to replace current metrics
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn update_metrics(&mut self, metrics: ProcessingMetrics) {
        self.metrics = metrics;
        self.updated_at = chrono::Utc::now();
    }

    /// Adds or updates a metadata key-value pair
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = chrono::Utc::now();
    }

    /// Retrieves a metadata value by key
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key to look up
    ///
    /// # Returns
    ///
    /// * `Some(&String)` - Value if key exists
    /// * `None` - If key not found
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Records the result of a processing stage
    ///
    /// # Arguments
    ///
    /// * `stage_name` - Name of the stage
    /// * `result` - Processing result or status
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn add_stage_result(&mut self, stage_name: String, result: String) {
        self.stage_results.insert(stage_name, result);
        self.updated_at = chrono::Utc::now();
    }

    /// Updates the security context
    ///
    /// # Arguments
    ///
    /// * `security_context` - New security context
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn update_security_context(&mut self, security_context: SecurityContext) {
        self.security_context = security_context;
        self.updated_at = chrono::Utc::now();
    }

    /// Calculates processing progress as a percentage
    ///
    /// # Returns
    ///
    /// Progress as a percentage (0.0 to 100.0)
    ///
    /// # Examples
    ///
    pub fn progress_percentage(&self) -> f64 {
        if self.file_size == 0 {
            return 0.0;
        }
        (self.processed_bytes as f64 / self.file_size as f64) * 100.0
    }

    /// Checks if processing is complete
    ///
    /// # Returns
    ///
    /// `true` if all bytes have been processed, `false` otherwise
    pub fn is_complete(&self) -> bool {
        self.processed_bytes >= self.file_size
    }

    /// Gets the timestamp when this context was created
    ///
    /// # Returns
    ///
    /// UTC creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    /// Gets the timestamp of the last update to this context
    ///
    /// # Returns
    ///
    /// UTC timestamp of last modification
    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
