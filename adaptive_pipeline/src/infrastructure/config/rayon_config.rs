// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Rayon Thread Pool Configuration
//!
//! This module provides global Rayon thread pool configuration optimized for
//! different workload types in the pipeline system.
//!
//! ## Overview
//!
//! The Rayon pool manager provides:
//!
//! - **CPU-Bound Pool**: Optimized for CPU-intensive operations like
//!   compression, encryption
//! - **Mixed Workload Pool**: Balanced for operations with both CPU and I/O
//!   components
//! - **Adaptive Sizing**: Integrates with WorkerCount optimization strategies
//! - **Thread Naming**: Clear thread naming for debugging and profiling
//!
//! ## Usage
//!
//! ```rust,ignore
//! use adaptive_pipeline::infrastructure::config::rayon_config::RAYON_POOLS;
//!
//! // Use CPU-bound pool for intensive operations
//! let results = RAYON_POOLS.cpu_bound_pool().install(|| {
//!     chunks.par_iter()
//!         .map(|chunk| compress(chunk))
//!         .collect()
//! });
//! ```

use adaptive_pipeline_domain::error::PipelineError;
use adaptive_pipeline_domain::value_objects::WorkerCount;
use std::sync::Arc;

/// Rayon thread pool manager for different workload types
///
/// Provides pre-configured thread pools optimized for:
/// - CPU-bound operations (compression, encryption, checksums)
/// - Mixed workloads (combination of CPU and I/O)
pub struct RayonPoolManager {
    cpu_bound_pool: Arc<rayon::ThreadPool>,
    mixed_workload_pool: Arc<rayon::ThreadPool>,
}

impl RayonPoolManager {
    /// Creates a new Rayon pool manager with optimized thread pools
    ///
    /// # Returns
    /// - `Ok(RayonPoolManager)` - Successfully initialized pools
    /// - `Err(PipelineError)` - Failed to create thread pools
    ///
    /// # Thread Pool Configuration
    ///
    /// **CPU-Bound Pool:**
    /// - Uses optimal worker count for CPU-intensive operations
    /// - Based on available cores and WorkerCount optimization
    /// - Named threads: "rayon-cpu-{N}"
    ///
    /// **Mixed Workload Pool:**
    /// - Uses half the cores to avoid contention
    /// - Balances CPU and I/O operations
    /// - Named threads: "rayon-mixed-{N}"
    pub fn new() -> Result<Self, PipelineError> {
        let available_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(WorkerCount::DEFAULT_WORKERS);

        // CPU-bound pool: Use optimal worker count for CPU-intensive operations
        let cpu_worker_count = WorkerCount::optimal_for_processing_type(
            100 * 1024 * 1024, // Assume medium file size for default config
            available_cores,
            true, // CPU-intensive
        );

        let cpu_bound_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_worker_count.count())
            .thread_name(|i| format!("rayon-cpu-{}", i))
            .build()
            .map_err(|e| PipelineError::InternalError(format!("Failed to create CPU-bound pool: {}", e)))?;

        // Mixed workload pool: Use fewer threads to avoid contention
        let mixed_worker_count = (available_cores / 2).max(WorkerCount::MIN_WORKERS);

        let mixed_workload_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(mixed_worker_count)
            .thread_name(|i| format!("rayon-mixed-{}", i))
            .build()
            .map_err(|e| PipelineError::InternalError(format!("Failed to create mixed workload pool: {}", e)))?;

        Ok(Self {
            cpu_bound_pool: Arc::new(cpu_bound_pool),
            mixed_workload_pool: Arc::new(mixed_workload_pool),
        })
    }

    /// Returns a reference to the CPU-bound thread pool
    ///
    /// Use this pool for CPU-intensive operations like:
    /// - Data compression
    /// - Data encryption/decryption
    /// - Checksum calculation
    /// - Complex transformations
    pub fn cpu_bound_pool(&self) -> &Arc<rayon::ThreadPool> {
        &self.cpu_bound_pool
    }

    /// Returns a reference to the mixed workload thread pool
    ///
    /// Use this pool for operations with both CPU and I/O:
    /// - File processing with transformations
    /// - Database operations with calculations
    /// - Network operations with data processing
    pub fn mixed_workload_pool(&self) -> &Arc<rayon::ThreadPool> {
        &self.mixed_workload_pool
    }

    /// Returns the number of threads in the CPU-bound pool
    pub fn cpu_thread_count(&self) -> usize {
        self.cpu_bound_pool.current_num_threads()
    }

    /// Returns the number of threads in the mixed workload pool
    pub fn mixed_thread_count(&self) -> usize {
        self.mixed_workload_pool.current_num_threads()
    }
}

/// Global Rayon pool manager instance
///
/// This is initialized once at program startup and provides access to
/// pre-configured thread pools throughout the application.
///
/// # Panics
/// Will panic if Rayon pools cannot be initialized (should never happen in
/// normal operation)
#[allow(clippy::expect_used)]
pub static RAYON_POOLS: std::sync::LazyLock<RayonPoolManager> =
    std::sync::LazyLock::new(|| RayonPoolManager::new().expect("Failed to initialize Rayon pools"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rayon_pool_creation() {
        let manager = RayonPoolManager::new().unwrap();
        assert!(manager.cpu_thread_count() > 0);
        assert!(manager.mixed_thread_count() > 0);
    }

    #[test]
    fn test_global_pool_access() {
        let cpu_pool = RAYON_POOLS.cpu_bound_pool();
        assert!(cpu_pool.current_num_threads() > 0);

        let mixed_pool = RAYON_POOLS.mixed_workload_pool();
        assert!(mixed_pool.current_num_threads() > 0);
    }

    #[test]
    fn test_pool_sizing() {
        let manager = RayonPoolManager::new().unwrap();

        // CPU pool should have more threads than mixed pool (or equal if very few
        // cores)
        let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);

        if available_cores >= 4 {
            assert!(manager.cpu_thread_count() >= manager.mixed_thread_count());
        }
    }
}
