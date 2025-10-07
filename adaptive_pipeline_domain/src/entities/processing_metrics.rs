// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Processing Metrics Entities
//!
//! This module contains entities for collecting, tracking, and analyzing
//! performance metrics during pipeline processing operations. The metrics
//! system provides detailed insights into processing performance, resource
//! utilization, and operational health.
//!
//! ## Overview
//!
//! The metrics system captures:
//!
//! - **Performance Data**: Throughput, processing times, and completion rates
//! - **Resource Usage**: Memory consumption, CPU utilization, and I/O
//!   statistics
//! - **Operational Health**: Error rates, success rates, and warning counts
//! - **Stage-Specific Metrics**: Individual performance data for each pipeline
//!   stage
//! - **File Processing Stats**: Input/output file sizes, checksums, and
//!   compression ratios
//!
//! ## Metrics Architecture
//!
//! ### High-Resolution Timing
//! Uses `std::time::Instant` for precise internal timing measurements while
//! providing RFC3339 timestamps for serialization and external reporting.
//!
//! ### Hierarchical Structure
//! - **ProcessingMetrics**: Overall pipeline processing metrics
//! - **StageMetrics**: Individual stage performance metrics
//!
//! ### Real-Time Calculation
//! Metrics are calculated and updated in real-time as processing progresses,
//! providing immediate feedback on performance characteristics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Processing metrics entity for comprehensive performance tracking and
/// analysis.
///
/// `ProcessingMetrics` collects and maintains detailed performance data
/// throughout pipeline execution. It provides real-time insights into
/// processing speed, resource utilization, error rates, and overall operational
/// health.
///
/// ## Metrics Categories
///
/// ### Processing Progress
/// - **Bytes Processed**: Total data processed and remaining
/// - **Chunks Processed**: Number of data chunks completed
/// - **Completion Status**: Progress percentage and estimated time remaining
///
/// ### Performance Metrics
/// - **Throughput**: Bytes per second and MB/s processing rates
/// - **Duration**: Total processing time and stage-specific timings
/// - **Efficiency**: Success rates and error statistics
///
/// ### File Information
/// - **Input/Output Sizes**: File sizes before and after processing
/// - **Checksums**: Integrity verification data
/// - **Compression Ratios**: Data reduction achieved
///
/// ### Stage Analytics
/// - **Individual Stage Performance**: Per-stage timing and throughput
/// - **Resource Usage**: Memory and CPU consumption by stage
/// - **Error Tracking**: Stage-specific error and warning counts
///
/// ## Usage Examples
///
/// ### Basic Metrics Tracking
///
///
/// ### Complete Processing Workflow
///
///
/// ### Error and Warning Tracking
///
///
/// ### Time Estimation
///
///
/// ### Merging Metrics from Multiple Sources
///
///
/// ## Serialization and Persistence
///
/// Metrics support serialization for logging and analysis:
///
///
/// ## Performance Considerations
///
/// - High-resolution timing uses `Instant` for accuracy
/// - Throughput calculations are performed on-demand
/// - Memory usage scales with the number of tracked stages
/// - Serialization excludes `Instant` fields for compatibility
/// - Real-time updates have minimal performance overhead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetrics {
    bytes_processed: u64,
    bytes_total: u64,
    chunks_processed: u64,
    chunks_total: u64,
    // High-resolution timing for internal use
    #[serde(skip)]
    start_time: Option<Instant>,
    #[serde(skip)]
    end_time: Option<Instant>,
    // RFC3339 timestamps for serialization
    start_time_rfc3339: Option<String>,
    end_time_rfc3339: Option<String>,
    processing_duration: Option<Duration>,
    throughput_bytes_per_second: f64,
    compression_ratio: Option<f64>,
    error_count: u64,
    warning_count: u64,
    // File information
    input_file_size_bytes: u64,
    output_file_size_bytes: u64,
    input_file_checksum: Option<String>,
    output_file_checksum: Option<String>,
    stage_metrics: std::collections::HashMap<String, StageMetrics>,
}

/// Stage-specific metrics entity for detailed performance analysis.
///
/// `StageMetrics` tracks performance data for individual pipeline stages,
/// providing granular insights into the efficiency and resource usage of
/// each processing step within the pipeline.
///
/// ## Stage Performance Data
///
/// ### Processing Metrics
/// - **Bytes Processed**: Amount of data processed by the stage
/// - **Processing Time**: Total time spent in the stage
/// - **Throughput**: Data processing rate (bytes per second)
///
/// ### Quality Metrics
/// - **Error Count**: Number of errors encountered
/// - **Success Rate**: Percentage of successful operations
///
/// ### Resource Metrics
/// - **Memory Usage**: Peak memory consumption (optional)
/// - **CPU Usage**: CPU utilization percentage (optional)
///
/// ## Usage Examples
///
/// ### Creating and Updating Stage Metrics
///
///
/// ### Comparing Stage Performance
///
///
/// ### Resource Monitoring
///
///
/// ### Error Rate Analysis
///
///
/// ## Integration with ProcessingMetrics
///
/// Stage metrics are typically collected and aggregated by `ProcessingMetrics`:
///
///
/// ## Performance Characteristics
///
/// - Lightweight structure with minimal memory overhead
/// - Real-time throughput calculation based on processed data and time
/// - Optional resource metrics to avoid unnecessary overhead
/// - Thread-safe when used with appropriate synchronization
/// - Efficient serialization for logging and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetrics {
    pub stage_name: String,
    pub bytes_processed: u64,
    pub processing_time: Duration,
    pub throughput: f64,
    pub error_count: u64,
    pub success_rate: f64,
    pub memory_usage: Option<u64>,
    pub cpu_usage: Option<f64>,
}

impl Default for ProcessingMetrics {
    fn default() -> Self {
        Self {
            bytes_processed: 0,
            bytes_total: 0,
            chunks_processed: 0,
            chunks_total: 0,
            start_time: None,
            end_time: None,
            start_time_rfc3339: None,
            end_time_rfc3339: None,
            processing_duration: None,
            throughput_bytes_per_second: 0.0,
            compression_ratio: None,
            error_count: 0,
            warning_count: 0,
            input_file_size_bytes: 0,
            output_file_size_bytes: 0,
            input_file_checksum: None,
            output_file_checksum: None,
            stage_metrics: std::collections::HashMap::new(),
        }
    }
}

impl ProcessingMetrics {
    /// Creates new processing metrics
    pub fn new(bytes_total: u64, chunks_total: u64) -> Self {
        Self {
            bytes_total,
            chunks_total,
            ..Default::default()
        }
    }

    /// Starts the processing timer
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.start_time_rfc3339 = Some(Utc::now().to_rfc3339());
    }

    /// Ends the processing timer
    pub fn end(&mut self) {
        self.end_time = Some(Instant::now());
        self.end_time_rfc3339 = Some(Utc::now().to_rfc3339());
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            self.processing_duration = Some(end.duration_since(start));
            self.calculate_throughput();
        }
    }

    /// Updates bytes processed
    pub fn update_bytes_processed(&mut self, bytes: u64) {
        self.bytes_processed = bytes;
        self.calculate_throughput();
    }

    /// Adds bytes processed
    pub fn add_bytes_processed(&mut self, bytes: u64) {
        self.bytes_processed += bytes;
        self.calculate_throughput();
    }

    /// Updates chunks processed
    pub fn update_chunks_processed(&mut self, chunks: u64) {
        self.chunks_processed = chunks;
    }

    /// Adds chunks processed
    pub fn add_chunks_processed(&mut self, chunks: u64) {
        self.chunks_processed += chunks;
    }

    /// Sets the compression ratio
    pub fn set_compression_ratio(&mut self, ratio: f64) {
        self.compression_ratio = Some(ratio);
    }

    /// Increments error count
    pub fn increment_errors(&mut self) {
        self.error_count += 1;
    }

    /// Increments warning count
    pub fn increment_warnings(&mut self) {
        self.warning_count += 1;
    }

    /// Adds stage metrics
    pub fn add_stage_metrics(&mut self, metrics: StageMetrics) {
        self.stage_metrics.insert(metrics.stage_name.clone(), metrics);
    }

    /// Gets bytes processed
    pub fn bytes_processed(&self) -> u64 {
        self.bytes_processed
    }

    /// Gets total bytes
    pub fn bytes_total(&self) -> u64 {
        self.bytes_total
    }

    /// Gets chunks processed
    pub fn chunks_processed(&self) -> u64 {
        self.chunks_processed
    }

    /// Gets total chunks
    pub fn chunks_total(&self) -> u64 {
        self.chunks_total
    }

    /// Gets processing duration
    pub fn processing_duration(&self) -> Option<Duration> {
        self.processing_duration
    }

    /// Gets start time as `DateTime<Utc>`
    pub fn start_time(&self) -> Option<DateTime<Utc>> {
        self.start_time_rfc3339
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc)))
    }

    /// Gets end time as `DateTime<Utc>`
    pub fn end_time(&self) -> Option<DateTime<Utc>> {
        self.end_time_rfc3339
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&Utc)))
    }

    /// Gets throughput in bytes per second
    pub fn throughput_bytes_per_second(&self) -> f64 {
        self.throughput_bytes_per_second
    }

    /// Gets throughput in MB/s
    pub fn throughput_mb_per_second(&self) -> f64 {
        self.throughput_bytes_per_second / (1024.0 * 1024.0)
    }

    /// Gets compression ratio
    pub fn compression_ratio(&self) -> Option<f64> {
        self.compression_ratio
    }

    /// Gets error count
    pub fn error_count(&self) -> u64 {
        self.error_count
    }

    /// Gets warning count
    pub fn warning_count(&self) -> u64 {
        self.warning_count
    }

    /// Gets stage metrics
    pub fn stage_metrics(&self) -> &std::collections::HashMap<String, StageMetrics> {
        &self.stage_metrics
    }

    /// Gets input file size in bytes
    pub fn input_file_size_bytes(&self) -> u64 {
        self.input_file_size_bytes
    }

    /// Gets output file size in bytes
    pub fn output_file_size_bytes(&self) -> u64 {
        self.output_file_size_bytes
    }

    /// Gets input file size in MiB
    pub fn input_file_size_mib(&self) -> f64 {
        (self.input_file_size_bytes as f64) / (1024.0 * 1024.0)
    }

    /// Gets output file size in MiB
    pub fn output_file_size_mib(&self) -> f64 {
        (self.output_file_size_bytes as f64) / (1024.0 * 1024.0)
    }

    /// Gets input file checksum
    pub fn input_file_checksum(&self) -> &Option<String> {
        &self.input_file_checksum
    }

    /// Gets output file checksum
    pub fn output_file_checksum(&self) -> &Option<String> {
        &self.output_file_checksum
    }

    /// Calculates processing progress as percentage
    pub fn progress_percentage(&self) -> f64 {
        if self.bytes_total == 0 {
            return 0.0;
        }
        ((self.bytes_processed as f64) / (self.bytes_total as f64)) * 100.0
    }

    /// Calculates chunk progress as percentage
    pub fn chunk_progress_percentage(&self) -> f64 {
        if self.chunks_total == 0 {
            return 0.0;
        }
        ((self.chunks_processed as f64) / (self.chunks_total as f64)) * 100.0
    }

    /// Estimates remaining time
    pub fn estimated_remaining_time(&self) -> Option<Duration> {
        if self.throughput_bytes_per_second <= 0.0 || self.bytes_processed == 0 {
            return None;
        }

        let remaining_bytes = self.bytes_total.saturating_sub(self.bytes_processed);
        let remaining_seconds = (remaining_bytes as f64) / self.throughput_bytes_per_second;
        Some(Duration::from_secs_f64(remaining_seconds))
    }

    /// Checks if processing is complete
    pub fn is_complete(&self) -> bool {
        self.bytes_processed >= self.bytes_total && self.chunks_processed >= self.chunks_total
    }

    /// Calculates overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.chunks_processed == 0 {
            return 0.0;
        }
        let successful_chunks = self.chunks_processed.saturating_sub(self.error_count);
        (successful_chunks as f64) / (self.chunks_processed as f64)
    }

    /// Sets input file size and checksum
    pub fn set_input_file_info(&mut self, size_bytes: u64, checksum: Option<String>) {
        self.input_file_size_bytes = size_bytes;
        self.input_file_checksum = checksum;
    }

    /// Sets output file size and checksum
    pub fn set_output_file_info(&mut self, size_bytes: u64, checksum: Option<String>) {
        self.output_file_size_bytes = size_bytes;
        self.output_file_checksum = checksum;
    }

    /// Calculates throughput based on current metrics
    fn calculate_throughput(&mut self) {
        if let Some(duration) = self.processing_duration {
            let seconds = duration.as_secs_f64();
            if seconds > 0.0 {
                self.throughput_bytes_per_second = (self.bytes_processed as f64) / seconds;
            }
        } else if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            let seconds = elapsed.as_secs_f64();
            if seconds > 0.0 {
                self.throughput_bytes_per_second = (self.bytes_processed as f64) / seconds;
            }
        }
    }

    /// Merges metrics from another instance
    pub fn merge(&mut self, other: &ProcessingMetrics) {
        self.bytes_processed += other.bytes_processed;
        self.chunks_processed += other.chunks_processed;
        self.error_count += other.error_count;
        self.warning_count += other.warning_count;

        // Merge stage metrics
        for (stage_name, stage_metrics) in &other.stage_metrics {
            self.stage_metrics.insert(stage_name.clone(), stage_metrics.clone());
        }

        // Recalculate throughput
        self.calculate_throughput();
    }
}

impl StageMetrics {
    /// Creates new stage metrics
    pub fn new(stage_name: String) -> Self {
        Self {
            stage_name,
            bytes_processed: 0,
            processing_time: Duration::ZERO,
            throughput: 0.0,
            error_count: 0,
            success_rate: 0.0,
            memory_usage: None,
            cpu_usage: None,
        }
    }

    /// Updates the stage metrics
    pub fn update(&mut self, bytes_processed: u64, processing_time: Duration) {
        self.bytes_processed = bytes_processed;
        self.processing_time = processing_time;

        let seconds = processing_time.as_secs_f64();
        if seconds > 0.0 {
            self.throughput = (bytes_processed as f64) / seconds;
        }
    }

    /// Sets memory usage
    pub fn set_memory_usage(&mut self, memory_usage: u64) {
        self.memory_usage = Some(memory_usage);
    }

    /// Sets CPU usage
    pub fn set_cpu_usage(&mut self, cpu_usage: f64) {
        self.cpu_usage = Some(cpu_usage);
    }

    /// Increments error count
    pub fn increment_errors(&mut self) {
        self.error_count += 1;
    }

    /// Calculates success rate
    pub fn calculate_success_rate(&mut self, total_operations: u64) {
        if total_operations > 0 {
            let successful_operations = total_operations.saturating_sub(self.error_count);
            self.success_rate = (successful_operations as f64) / (total_operations as f64);
        }
    }
}
