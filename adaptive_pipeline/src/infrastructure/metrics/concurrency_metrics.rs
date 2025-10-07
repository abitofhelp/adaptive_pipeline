// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Concurrency Metrics
//!
//! This module provides metrics for monitoring and tuning concurrency and
//! resource utilization in the pipeline system.
//!
//! ## Educational Purpose
//!
//! These metrics demonstrate:
//! - How to observe resource saturation
//! - When to tune resource limits
//! - Impact of concurrency on throughput and latency
//!
//! ## Metric Types
//!
//! **Gauges** - Instant values (e.g., tokens available right now)
//! **Counters** - Cumulative values (e.g., total wait time)
//! **Histograms** - Distribution of values (e.g., P50/P95/P99 wait times)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use adaptive_pipeline::infrastructure::metrics::CONCURRENCY_METRICS;
//!
//! // Record resource acquisition
//! CONCURRENCY_METRICS.record_cpu_wait(wait_duration);
//!
//! // Check saturation
//! let saturation = CONCURRENCY_METRICS.cpu_saturation_percent();
//! if saturation > 80.0 {
//!     println!("CPU-saturated: consider increasing workers");
//! }
//! ```

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Simple histogram for latency distribution tracking
///
/// ## Educational: Why a histogram?
///
/// Averages hide problems! Consider:
/// - Average wait: 10ms (looks fine)
/// - But P99 wait: 500ms (users experience terrible latency!)
///
/// Histograms show the full distribution, revealing tail latencies.
#[derive(Debug)]
pub struct Histogram {
    /// Bucket boundaries in milliseconds
    /// [0-1ms, 1-5ms, 5-10ms, 10-50ms, 50-100ms, 100+ms]
    buckets: Vec<AtomicU64>,
    bucket_boundaries: Vec<u64>,
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Histogram {
    pub fn new() -> Self {
        // Educational: Chosen boundaries capture common latency ranges
        // Adjust based on your system's characteristics
        let bucket_boundaries = vec![1, 5, 10, 50, 100];
        let buckets = bucket_boundaries
            .iter()
            .map(|_| AtomicU64::new(0))
            .chain(std::iter::once(AtomicU64::new(0))) // +inf bucket
            .collect();

        Self {
            buckets,
            bucket_boundaries,
        }
    }

    /// Record a value in milliseconds
    pub fn record(&self, value_ms: u64) {
        // Find appropriate bucket
        let bucket_idx = self
            .bucket_boundaries
            .iter()
            .position(|&boundary| value_ms < boundary)
            .unwrap_or(self.bucket_boundaries.len());

        self.buckets[bucket_idx].fetch_add(1, Ordering::Relaxed);
    }

    /// Get total count across all buckets
    pub fn count(&self) -> u64 {
        self.buckets.iter().map(|b| b.load(Ordering::Relaxed)).sum()
    }

    /// Get rough percentile estimate
    ///
    /// ## Educational: Why "rough"?
    ///
    /// True percentiles require sorting all values. Histograms trade
    /// precision for memory efficiency by bucketing values.
    pub fn percentile(&self, p: f64) -> u64 {
        let total = self.count();
        if total == 0 {
            return 0;
        }

        let target = (((total as f64) * p) / 100.0) as u64;
        let mut cumulative = 0u64;

        for (i, bucket) in self.buckets.iter().enumerate() {
            cumulative += bucket.load(Ordering::Relaxed);
            if cumulative >= target {
                return if i < self.bucket_boundaries.len() {
                    self.bucket_boundaries[i]
                } else {
                    100 // +inf bucket
                };
            }
        }

        0
    }

    /// Reset histogram
    pub fn reset(&self) {
        for bucket in &self.buckets {
            bucket.store(0, Ordering::Relaxed);
        }
    }
}

/// Concurrency metrics for resource manager monitoring
///
/// ## Educational: Observability-Driven Tuning
///
/// These metrics answer key questions:
/// - "Are we CPU-saturated?" → cpu_saturation_percent()
/// - "Are we I/O-saturated?" → io_saturation_percent()
/// - "What's causing latency?" → wait time histograms
/// - "How much memory are we using?" → memory_used_bytes
///
/// Use these to guide tuning decisions!
#[derive(Debug)]
pub struct ConcurrencyMetrics {
    // === CPU Metrics ===
    /// Current number of available CPU tokens (gauge)
    cpu_tokens_available: AtomicUsize,

    /// Total CPU tokens configured (static)
    cpu_tokens_total: usize,

    /// Total time spent waiting for CPU tokens (counter, milliseconds)
    cpu_wait_total_ms: AtomicU64,

    /// Histogram of CPU token wait times
    cpu_wait_histogram: Mutex<Histogram>,

    // === I/O Metrics ===
    /// Current number of available I/O tokens (gauge)
    io_tokens_available: AtomicUsize,

    /// Total I/O tokens configured (static)
    io_tokens_total: usize,

    /// Total time spent waiting for I/O tokens (counter, milliseconds)
    io_wait_total_ms: AtomicU64,

    /// Histogram of I/O token wait times
    io_wait_histogram: Mutex<Histogram>,

    // === Memory Metrics ===
    /// Current memory usage in bytes (gauge)
    memory_used_bytes: AtomicUsize,

    /// Memory capacity in bytes (static)
    memory_capacity_bytes: usize,

    // === Worker Metrics ===
    /// Number of currently active workers (gauge)
    active_workers: AtomicUsize,

    /// Total number of tasks spawned (counter)
    tasks_spawned: AtomicU64,

    /// Total number of tasks completed (counter)
    tasks_completed: AtomicU64,

    // === Channel Queue Metrics ===
    /// Current depth of CPU worker channel (gauge)
    /// Educational: Reveals backpressure - high depth means workers can't keep
    /// up
    cpu_queue_depth: AtomicUsize,

    /// Maximum CPU queue depth observed (gauge)
    /// Educational: Shows peak backpressure during processing
    cpu_queue_depth_max: AtomicUsize,

    /// Histogram of time chunks wait in CPU queue
    /// Educational: Queue wait time indicates worker saturation
    cpu_queue_wait_histogram: Mutex<Histogram>,
}

impl ConcurrencyMetrics {
    pub fn new(cpu_tokens: usize, io_tokens: usize, memory_capacity: usize) -> Self {
        Self {
            cpu_tokens_available: AtomicUsize::new(cpu_tokens),
            cpu_tokens_total: cpu_tokens,
            cpu_wait_total_ms: AtomicU64::new(0),
            cpu_wait_histogram: Mutex::new(Histogram::new()),

            io_tokens_available: AtomicUsize::new(io_tokens),
            io_tokens_total: io_tokens,
            io_wait_total_ms: AtomicU64::new(0),
            io_wait_histogram: Mutex::new(Histogram::new()),

            memory_used_bytes: AtomicUsize::new(0),
            memory_capacity_bytes: memory_capacity,

            active_workers: AtomicUsize::new(0),
            tasks_spawned: AtomicU64::new(0),
            tasks_completed: AtomicU64::new(0),

            // Queue metrics
            cpu_queue_depth: AtomicUsize::new(0),
            cpu_queue_depth_max: AtomicUsize::new(0),
            cpu_queue_wait_histogram: Mutex::new(Histogram::new()),
        }
    }

    // === CPU Metrics ===

    /// Update CPU tokens available (from ResourceManager)
    pub fn update_cpu_tokens_available(&self, available: usize) {
        self.cpu_tokens_available.store(available, Ordering::Relaxed);
    }

    /// Get CPU tokens available
    pub fn cpu_tokens_available(&self) -> usize {
        self.cpu_tokens_available.load(Ordering::Relaxed)
    }

    /// Get CPU saturation percentage
    ///
    /// ## Educational: What is saturation?
    ///
    /// Saturation = (tokens_in_use / total_tokens) × 100
    /// - 0%: Idle, not utilizing resources
    /// - 50%: Good utilization
    /// - 80-90%: High utilization, approaching saturation
    /// - 100%: Fully saturated, tasks waiting
    pub fn cpu_saturation_percent(&self) -> f64 {
        let available = self.cpu_tokens_available.load(Ordering::Relaxed);
        let in_use = self.cpu_tokens_total.saturating_sub(available);
        ((in_use as f64) / (self.cpu_tokens_total as f64)) * 100.0
    }

    /// Record CPU token wait time
    pub fn record_cpu_wait(&self, duration: Duration) {
        let ms = duration.as_millis() as u64;
        self.cpu_wait_total_ms.fetch_add(ms, Ordering::Relaxed);

        if let Ok(hist) = self.cpu_wait_histogram.lock() {
            hist.record(ms);
        }
    }

    /// Get CPU wait time percentile
    pub fn cpu_wait_p50(&self) -> u64 {
        self.cpu_wait_histogram.lock().map(|h| h.percentile(50.0)).unwrap_or(0)
    }

    pub fn cpu_wait_p95(&self) -> u64 {
        self.cpu_wait_histogram.lock().map(|h| h.percentile(95.0)).unwrap_or(0)
    }

    pub fn cpu_wait_p99(&self) -> u64 {
        self.cpu_wait_histogram.lock().map(|h| h.percentile(99.0)).unwrap_or(0)
    }

    // === I/O Metrics ===

    /// Update I/O tokens available (from ResourceManager)
    pub fn update_io_tokens_available(&self, available: usize) {
        self.io_tokens_available.store(available, Ordering::Relaxed);
    }

    pub fn io_tokens_available(&self) -> usize {
        self.io_tokens_available.load(Ordering::Relaxed)
    }

    pub fn io_saturation_percent(&self) -> f64 {
        let available = self.io_tokens_available.load(Ordering::Relaxed);
        let in_use = self.io_tokens_total.saturating_sub(available);
        ((in_use as f64) / (self.io_tokens_total as f64)) * 100.0
    }

    /// Record I/O token wait time
    pub fn record_io_wait(&self, duration: Duration) {
        let ms = duration.as_millis() as u64;
        self.io_wait_total_ms.fetch_add(ms, Ordering::Relaxed);

        if let Ok(hist) = self.io_wait_histogram.lock() {
            hist.record(ms);
        }
    }

    pub fn io_wait_p50(&self) -> u64 {
        self.io_wait_histogram.lock().map(|h| h.percentile(50.0)).unwrap_or(0)
    }

    pub fn io_wait_p95(&self) -> u64 {
        self.io_wait_histogram.lock().map(|h| h.percentile(95.0)).unwrap_or(0)
    }

    pub fn io_wait_p99(&self) -> u64 {
        self.io_wait_histogram.lock().map(|h| h.percentile(99.0)).unwrap_or(0)
    }

    // === Memory Metrics ===

    pub fn update_memory_used(&self, bytes: usize) {
        self.memory_used_bytes.store(bytes, Ordering::Relaxed);
    }

    pub fn memory_used_bytes(&self) -> usize {
        self.memory_used_bytes.load(Ordering::Relaxed)
    }

    pub fn memory_used_mb(&self) -> f64 {
        (self.memory_used_bytes() as f64) / (1024.0 * 1024.0)
    }

    pub fn memory_capacity_bytes(&self) -> usize {
        self.memory_capacity_bytes
    }

    pub fn memory_utilization_percent(&self) -> f64 {
        ((self.memory_used_bytes() as f64) / (self.memory_capacity_bytes as f64)) * 100.0
    }

    // === Worker Metrics ===

    pub fn worker_started(&self) {
        self.active_workers.fetch_add(1, Ordering::Relaxed);
        self.tasks_spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn worker_completed(&self) {
        self.active_workers.fetch_sub(1, Ordering::Relaxed);
        self.tasks_completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn active_workers(&self) -> usize {
        self.active_workers.load(Ordering::Relaxed)
    }

    pub fn tasks_spawned(&self) -> u64 {
        self.tasks_spawned.load(Ordering::Relaxed)
    }

    pub fn tasks_completed(&self) -> u64 {
        self.tasks_completed.load(Ordering::Relaxed)
    }

    // === Channel Queue Metrics ===

    /// Update CPU queue depth
    ///
    /// ## Educational: Observing Backpressure
    ///
    /// Queue depth reveals whether workers can keep up with the reader:
    /// - Depth near 0: Workers are faster than reader (good!)
    /// - Depth near capacity: Workers are bottleneck (increase workers or
    ///   optimize stages)
    /// - Depth at capacity: Reader is blocked (severe backpressure)
    pub fn update_cpu_queue_depth(&self, depth: usize) {
        self.cpu_queue_depth.store(depth, Ordering::Relaxed);

        // Track maximum depth observed
        let mut current_max = self.cpu_queue_depth_max.load(Ordering::Relaxed);
        while depth > current_max {
            match self.cpu_queue_depth_max.compare_exchange_weak(
                current_max,
                depth,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    break;
                }
                Err(x) => {
                    current_max = x;
                }
            }
        }
    }

    /// Get current CPU queue depth
    pub fn cpu_queue_depth(&self) -> usize {
        self.cpu_queue_depth.load(Ordering::Relaxed)
    }

    /// Get maximum CPU queue depth observed
    pub fn cpu_queue_depth_max(&self) -> usize {
        self.cpu_queue_depth_max.load(Ordering::Relaxed)
    }

    /// Record time a chunk waited in CPU queue
    pub fn record_cpu_queue_wait(&self, duration: Duration) {
        let ms = duration.as_millis() as u64;
        if let Ok(hist) = self.cpu_queue_wait_histogram.lock() {
            hist.record(ms);
        }
    }

    /// Get P50 (median) CPU queue wait time in milliseconds
    pub fn cpu_queue_wait_p50(&self) -> u64 {
        self.cpu_queue_wait_histogram
            .lock()
            .map(|h| h.percentile(50.0))
            .unwrap_or(0)
    }

    /// Get P95 CPU queue wait time in milliseconds
    pub fn cpu_queue_wait_p95(&self) -> u64 {
        self.cpu_queue_wait_histogram
            .lock()
            .map(|h| h.percentile(95.0))
            .unwrap_or(0)
    }

    /// Get P99 CPU queue wait time in milliseconds
    pub fn cpu_queue_wait_p99(&self) -> u64 {
        self.cpu_queue_wait_histogram
            .lock()
            .map(|h| h.percentile(99.0))
            .unwrap_or(0)
    }

    /// Reset all metrics (for testing/benchmarking)
    pub fn reset(&self) {
        self.cpu_wait_total_ms.store(0, Ordering::Relaxed);
        self.io_wait_total_ms.store(0, Ordering::Relaxed);
        self.tasks_spawned.store(0, Ordering::Relaxed);
        self.tasks_completed.store(0, Ordering::Relaxed);

        // Reset queue metrics
        self.cpu_queue_depth.store(0, Ordering::Relaxed);
        self.cpu_queue_depth_max.store(0, Ordering::Relaxed);

        if let Ok(hist) = self.cpu_wait_histogram.lock() {
            hist.reset();
        }
        if let Ok(hist) = self.io_wait_histogram.lock() {
            hist.reset();
        }
        if let Ok(hist) = self.cpu_queue_wait_histogram.lock() {
            hist.reset();
        }
    }
}

/// Global concurrency metrics instance
///
/// ## Educational: Lazy Initialization
///
/// Initialized from RESOURCE_MANAGER values on first access.
/// This ensures metrics match actual resource configuration.
pub static CONCURRENCY_METRICS: std::sync::LazyLock<Arc<ConcurrencyMetrics>> = std::sync::LazyLock::new(|| {
    use crate::infrastructure::runtime::RESOURCE_MANAGER;

    Arc::new(ConcurrencyMetrics::new(
        RESOURCE_MANAGER.cpu_tokens_total(),
        RESOURCE_MANAGER.io_tokens_total(),
        RESOURCE_MANAGER.memory_capacity(),
    ))
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_basic() {
        let hist = Histogram::new();

        hist.record(2); // 1-5ms bucket
        hist.record(7); // 5-10ms bucket
        hist.record(25); // 10-50ms bucket

        assert_eq!(hist.count(), 3);
    }

    #[test]
    fn test_concurrency_metrics_creation() {
        let metrics = ConcurrencyMetrics::new(8, 24, 1024 * 1024 * 1024);

        assert_eq!(metrics.cpu_tokens_available(), 8);
        assert_eq!(metrics.io_tokens_available(), 24);
        assert_eq!(metrics.cpu_saturation_percent(), 0.0);
    }

    #[test]
    fn test_saturation_calculation() {
        let metrics = ConcurrencyMetrics::new(10, 20, 1024);

        // Initially no saturation
        assert_eq!(metrics.cpu_saturation_percent(), 0.0);

        // Simulate 5 tokens in use
        metrics.update_cpu_tokens_available(5);
        assert_eq!(metrics.cpu_saturation_percent(), 50.0);

        // Fully saturated
        metrics.update_cpu_tokens_available(0);
        assert_eq!(metrics.cpu_saturation_percent(), 100.0);
    }

    #[test]
    fn test_wait_time_recording() {
        let metrics = ConcurrencyMetrics::new(8, 24, 1024);

        metrics.record_cpu_wait(Duration::from_millis(10));
        metrics.record_cpu_wait(Duration::from_millis(20));

        // Recorded in histogram
        assert!(metrics.cpu_wait_p50() > 0);
    }

    #[test]
    fn test_worker_tracking() {
        let metrics = ConcurrencyMetrics::new(8, 24, 1024);

        assert_eq!(metrics.active_workers(), 0);

        metrics.worker_started();
        assert_eq!(metrics.active_workers(), 1);
        assert_eq!(metrics.tasks_spawned(), 1);

        metrics.worker_completed();
        assert_eq!(metrics.active_workers(), 0);
        assert_eq!(metrics.tasks_completed(), 1);
    }
}
