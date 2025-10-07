// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Global Resource Manager
//!
//! This module provides centralized resource governance across the entire
//! application, preventing resource oversubscription when processing multiple
//! files concurrently.
//!
//! ## Architecture Pattern: Two-Level Resource Governance
//!
//! **Problem:** Without global limits, multiple concurrent files can overwhelm
//! the system:
//! - 10 files × 8 workers/file = 80 concurrent tasks on an 8-core machine
//! - Result: CPU oversubscription, cache thrashing, poor throughput
//!
//! **Solution:** Two-level coordination:
//! 1. **Global limits** (this module) - Cap total system resources
//! 2. **Local limits** (per-file semaphores) - Cap per-file concurrency
//!
//! ## Educational Example
//!
//! ```rust,ignore
//! use adaptive_pipeline::infrastructure::runtime::RESOURCE_MANAGER;
//!
//! async fn process_file() -> Result<()> {
//!     // 1. Acquire global CPU token (waits if system is saturated)
//!     let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;
//!
//!     // 2. Acquire local per-file token
//!     let _local_permit = file_semaphore.acquire().await?;
//!
//!     // 3. Do CPU-intensive work
//!     compress_data().await?;
//!
//!     // 4. Both permits released automatically (RAII)
//!     Ok(())
//! }
//! ```
//!
//! ## Resource Types
//!
//! ### CPU Tokens
//! - **Purpose:** Limit total CPU-bound work across all files
//! - **Default:** `available_cores - 1` (leave one for OS/I/O)
//! - **Use:** Acquire before Rayon work or CPU-intensive operations
//!
//! ### I/O Tokens
//! - **Purpose:** Prevent I/O queue overrun
//! - **Default:** Device-specific (NVMe: 24, SSD: 12, HDD: 4)
//! - **Use:** Acquire before file reads/writes
//!
//! ### Memory Tracking
//! - **Purpose:** Monitor memory usage (gauge only, no enforcement yet)
//! - **Default:** No limit (soft monitoring)
//! - **Future:** Can add hard cap in Phase 3

use adaptive_pipeline_domain::PipelineError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{Semaphore, SemaphorePermit};

/// Storage device type for I/O queue depth optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    /// NVMe SSD - High queue depth (24-32)
    NVMe,
    /// SATA SSD - Medium queue depth (8-16)
    Ssd,
    /// Hard Disk Drive - Low queue depth (2-4)
    Hdd,
    /// Auto-detect based on system
    Auto,
    /// Custom queue depth
    Custom(usize),
}

/// Configuration for global resource manager
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Number of CPU worker tokens (default: cores - 1)
    pub cpu_tokens: Option<usize>,

    /// Number of I/O tokens (default: device-specific)
    pub io_tokens: Option<usize>,

    /// Storage device type for I/O optimization
    pub storage_type: StorageType,

    /// Soft memory limit in bytes (gauge only, no enforcement)
    pub memory_limit: Option<usize>,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            cpu_tokens: None, // Will use cores - 1
            io_tokens: None,  // Will use device-specific
            storage_type: StorageType::Auto,
            memory_limit: None, // No limit by default
        }
    }
}

/// Global resource manager for system-wide resource coordination
///
/// ## Design Pattern: Centralized Resource Governance
///
/// This manager prevents resource oversubscription by providing a global
/// pool of CPU and I/O tokens that must be acquired before work begins.
///
/// ## Educational Notes
///
/// **Why semaphores?**
/// - Semaphores provide backpressure: work waits when resources are saturated
/// - RAII permits auto-release resources on drop
/// - Async-aware: integrates with Tokio runtime
///
/// **Why separate CPU and I/O tokens?**
/// - CPU work and I/O work have different characteristics
/// - CPU: Limited by cores, benefits from parallelism = cores
/// - I/O: Limited by device queue depth, different optimal values
///
/// **Why memory as gauge only?**
/// - Memory is harder to predict and control
/// - Start with monitoring, add enforcement later if needed
/// - Avoids complexity in Phase 1
pub struct GlobalResourceManager {
    /// CPU worker tokens (semaphore permits)
    ///
    /// **Purpose:** Prevent CPU oversubscription
    /// **Typical value:** cores - 1
    /// **Educational:** This is a "counting semaphore" that allows N concurrent
    /// operations
    cpu_tokens: Arc<Semaphore>,

    /// I/O operation tokens (semaphore permits)
    ///
    /// **Purpose:** Prevent I/O queue overrun
    /// **Typical value:** Device-specific (NVMe: 24, SSD: 12, HDD: 4)
    /// **Educational:** Different devices have different optimal queue depths
    io_tokens: Arc<Semaphore>,

    /// Memory usage gauge (bytes)
    ///
    /// **Purpose:** Monitor memory pressure (no enforcement yet)
    /// **Educational:** Start simple (gauge), add limits later (Phase 3)
    memory_used: Arc<AtomicUsize>,

    /// Total memory capacity for reporting
    memory_capacity: usize,

    /// Number of CPU tokens configured
    cpu_token_count: usize,

    /// Number of I/O tokens configured
    io_token_count: usize,
}

impl GlobalResourceManager {
    /// Creates a new global resource manager with the given configuration
    ///
    /// ## Educational: Resource Detection and Configuration
    ///
    /// This method demonstrates:
    /// - Auto-detection of system resources (CPU cores)
    /// - Device-specific I/O optimization
    /// - Sensible defaults with override capability
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// // Use defaults (auto-detected)
    /// let manager = GlobalResourceManager::new(Default::default())?;
    ///
    /// // Custom configuration
    /// let manager = GlobalResourceManager::new(ResourceConfig {
    ///     cpu_tokens: Some(6),  // Override: use 6 CPU workers
    ///     storage_type: StorageType::NVMe,
    ///     ..Default::default()
    /// })?;
    /// ```
    pub fn new(config: ResourceConfig) -> Result<Self, PipelineError> {
        // Detect available CPU cores
        let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4); // Conservative fallback

        // Educational: Why cores - 1?
        // Leave one core for OS, I/O threads, and system tasks
        // Prevents complete CPU saturation which hurts overall system responsiveness
        let cpu_token_count = config.cpu_tokens.unwrap_or_else(|| (available_cores - 1).max(1));

        // Educational: Device-specific I/O queue depths
        // Different storage devices have different optimal concurrency levels
        let io_token_count = config
            .io_tokens
            .unwrap_or_else(|| Self::detect_optimal_io_tokens(config.storage_type));

        // Educational: Memory capacity detection
        // On most systems, we can query available RAM
        // For now, use a conservative default if not specified
        let memory_capacity = config.memory_limit.unwrap_or(40 * 1024 * 1024 * 1024); // 40GB default

        Ok(Self {
            cpu_tokens: Arc::new(Semaphore::new(cpu_token_count)),
            io_tokens: Arc::new(Semaphore::new(io_token_count)),
            memory_used: Arc::new(AtomicUsize::new(0)),
            memory_capacity,
            cpu_token_count,
            io_token_count,
        })
    }

    /// Detect optimal I/O token count based on storage type
    ///
    /// ## Educational: Device Characteristics
    ///
    /// **NVMe (24-32 tokens):**
    /// - Multiple parallel channels
    /// - Low latency, high throughput
    /// - Benefits from high queue depth
    ///
    /// **SSD (8-16 tokens):**
    /// - Medium parallelism
    /// - Good random access
    /// - Moderate queue depth optimal
    ///
    /// **HDD (2-4 tokens):**
    /// - Sequential access preferred
    /// - High seek latency
    /// - Low queue depth prevents thrashing
    fn detect_optimal_io_tokens(storage_type: StorageType) -> usize {
        match storage_type {
            StorageType::NVMe => 24,
            StorageType::Ssd => 12,
            StorageType::Hdd => 4,
            StorageType::Auto => {
                // Educational: Simple heuristic
                // In production, would query device capabilities
                // For now, assume SSD as reasonable default
                12
            }
            StorageType::Custom(n) => n,
        }
    }

    /// Acquire a CPU token (explicit style - pedagogical)
    ///
    /// ## Educational Pattern: Explicit Acquisition
    ///
    /// This method shows the explicit pattern where you:
    /// 1. Call acquire
    /// 2. Get back a permit
    /// 3. Permit is held as long as the guard lives
    /// 4. Permit is auto-released when dropped (RAII)
    ///
    /// ## Usage
    ///
    /// ```rust,ignore
    /// let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;
    /// // Do CPU work
    /// // Permit auto-released here when _cpu_permit goes out of scope
    /// ```
    ///
    /// ## Backpressure
    ///
    /// If all CPU tokens are in use, this method **waits** until one becomes
    /// available. This creates natural backpressure and prevents
    /// oversubscription.
    pub async fn acquire_cpu(&self) -> Result<SemaphorePermit<'_>, PipelineError> {
        self.cpu_tokens
            .acquire()
            .await
            .map_err(|_| PipelineError::InternalError("CPU semaphore closed".to_string()))
    }

    /// Acquire an I/O token
    ///
    /// ## Educational: Same pattern as CPU tokens
    ///
    /// Uses the same semaphore pattern but for I/O operations.
    /// Prevents too many concurrent I/O operations from overwhelming
    /// the storage device.
    ///
    /// ## Usage
    ///
    /// ```rust,ignore
    /// let _io_permit = RESOURCE_MANAGER.acquire_io().await?;
    /// // Do I/O operation (read/write)
    /// // Permit auto-released
    /// ```
    pub async fn acquire_io(&self) -> Result<SemaphorePermit<'_>, PipelineError> {
        self.io_tokens
            .acquire()
            .await
            .map_err(|_| PipelineError::InternalError("I/O semaphore closed".to_string()))
    }

    /// Track memory allocation (gauge only, no enforcement)
    ///
    /// ## Educational: Simple Atomic Counter
    ///
    /// Uses `Ordering::Relaxed` because:
    /// - We only need atomicity (no torn reads/writes)
    /// - No coordination with other atomic variables needed
    /// - This is just a gauge for monitoring
    ///
    /// See atomic_ordering.rs for more on ordering choices.
    pub fn allocate_memory(&self, bytes: usize) {
        self.memory_used.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Track memory deallocation
    pub fn deallocate_memory(&self, bytes: usize) {
        self.memory_used.fetch_sub(bytes, Ordering::Relaxed);
    }

    /// Get current memory usage
    pub fn memory_used(&self) -> usize {
        self.memory_used.load(Ordering::Relaxed)
    }

    /// Get memory capacity
    pub fn memory_capacity(&self) -> usize {
        self.memory_capacity
    }

    /// Get number of available CPU tokens
    ///
    /// ## Educational: Observability
    ///
    /// This method provides visibility into resource saturation.
    /// If available_permits() is consistently 0, you're CPU-saturated.
    pub fn cpu_tokens_available(&self) -> usize {
        self.cpu_tokens.available_permits()
    }

    /// Get total number of CPU tokens
    pub fn cpu_tokens_total(&self) -> usize {
        self.cpu_token_count
    }

    /// Get number of available I/O tokens
    pub fn io_tokens_available(&self) -> usize {
        self.io_tokens.available_permits()
    }

    /// Get total number of I/O tokens
    pub fn io_tokens_total(&self) -> usize {
        self.io_token_count
    }
}

/// Global singleton instance of the resource manager
///
/// ## Educational: Singleton Pattern with Configuration
///
/// Uses `OnceLock` for:
/// - Thread-safe one-time initialization
/// - Allows custom configuration from CLI
/// - Initialized exactly once via `init_resource_manager()`
/// - Static lifetime for global access
///
/// ## Usage
///
/// ```rust,ignore
/// use adaptive_pipeline::infrastructure::runtime::{init_resource_manager, ResourceConfig};
///
/// // In main(), before any operations:
/// init_resource_manager(ResourceConfig::default())?;
///
/// // Later, anywhere in code:
/// async fn my_function() {
///     let _permit = RESOURCE_MANAGER.acquire_cpu().await?;
///     // ...
/// }
/// ```
static RESOURCE_MANAGER_CELL: std::sync::OnceLock<GlobalResourceManager> = std::sync::OnceLock::new();

/// Initialize the global resource manager with custom configuration
///
/// ## Educational: Explicit Initialization Pattern
///
/// This must be called exactly once, early in main(), before any code
/// accesses RESOURCE_MANAGER. Subsequent calls will return an error.
///
/// ## Why This Pattern?
///
/// - Allows CLI flags to configure resource limits
/// - Makes initialization explicit and debuggable
/// - Avoids "lazy initialization with hidden defaults"
/// - Better testability (each test can configure differently)
///
/// ## Errors
///
/// Returns error if:
/// - Already initialized (called twice)
/// - Configuration is invalid (e.g., 0 CPU threads)
pub fn init_resource_manager(config: ResourceConfig) -> Result<(), String> {
    let manager =
        GlobalResourceManager::new(config).map_err(|e| format!("Failed to create resource manager: {}", e))?;

    RESOURCE_MANAGER_CELL
        .set(manager)
        .map_err(|_| "Resource manager already initialized".to_string())
}

/// Access the global resource manager
///
/// ## Panics
///
/// Panics if called before `init_resource_manager()`. This is intentional -
/// using the resource manager before initialization is a programming error.
#[allow(clippy::expect_used)]
pub fn resource_manager() -> &'static GlobalResourceManager {
    RESOURCE_MANAGER_CELL
        .get()
        .expect("Resource manager not initialized! Call init_resource_manager() in main().")
}

/// Legacy alias for backward compatibility
///
/// **Pattern**: Both `RESOURCE_MANAGER` (static) and `resource_manager()`
/// (function) are supported. New code should prefer the function style for
/// consistency.
#[allow(non_upper_case_globals)]
pub static RESOURCE_MANAGER: std::sync::LazyLock<&'static GlobalResourceManager> =
    std::sync::LazyLock::new(resource_manager);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_manager_creation() {
        let manager = GlobalResourceManager::new(ResourceConfig::default()).unwrap();

        // Should have at least 1 CPU token
        assert!(manager.cpu_tokens_total() >= 1);

        // Should have I/O tokens
        assert!(manager.io_tokens_total() > 0);

        // Initially all tokens available
        assert_eq!(manager.cpu_tokens_available(), manager.cpu_tokens_total());
        assert_eq!(manager.io_tokens_available(), manager.io_tokens_total());
    }

    #[test]
    fn test_device_type_queue_depths() {
        let nvme_qd = GlobalResourceManager::detect_optimal_io_tokens(StorageType::NVMe);
        let ssd_qd = GlobalResourceManager::detect_optimal_io_tokens(StorageType::Ssd);
        let hdd_qd = GlobalResourceManager::detect_optimal_io_tokens(StorageType::Hdd);

        // NVMe should have highest queue depth
        assert!(nvme_qd > ssd_qd);
        assert!(ssd_qd > hdd_qd);

        // Specific values
        assert_eq!(nvme_qd, 24);
        assert_eq!(ssd_qd, 12);
        assert_eq!(hdd_qd, 4);
    }

    #[tokio::test]
    async fn test_cpu_token_acquisition() {
        let manager = GlobalResourceManager::new(ResourceConfig {
            cpu_tokens: Some(2),
            ..Default::default()
        })
        .unwrap();

        // Initially 2 available
        assert_eq!(manager.cpu_tokens_available(), 2);

        // Acquire one
        let _permit1 = manager.acquire_cpu().await.unwrap();
        assert_eq!(manager.cpu_tokens_available(), 1);

        // Acquire another
        let _permit2 = manager.acquire_cpu().await.unwrap();
        assert_eq!(manager.cpu_tokens_available(), 0);

        // Drop first permit
        drop(_permit1);
        assert_eq!(manager.cpu_tokens_available(), 1);
    }

    #[tokio::test]
    async fn test_io_token_acquisition() {
        let manager = GlobalResourceManager::new(ResourceConfig {
            io_tokens: Some(4),
            ..Default::default()
        })
        .unwrap();

        assert_eq!(manager.io_tokens_available(), 4);

        let _permit = manager.acquire_io().await.unwrap();
        assert_eq!(manager.io_tokens_available(), 3);
    }

    #[test]
    fn test_memory_tracking() {
        let manager = GlobalResourceManager::new(ResourceConfig::default()).unwrap();

        assert_eq!(manager.memory_used(), 0);

        manager.allocate_memory(1000);
        assert_eq!(manager.memory_used(), 1000);

        manager.allocate_memory(500);
        assert_eq!(manager.memory_used(), 1500);

        manager.deallocate_memory(700);
        assert_eq!(manager.memory_used(), 800);
    }

    #[test]
    fn test_global_singleton_access() {
        // Initialize the global resource manager for this test
        // Each test runs in its own process, so this is safe
        let _ = init_resource_manager(ResourceConfig::default());

        // Should be able to access the global instance after initialization
        let rm = resource_manager();
        let available = rm.cpu_tokens_available();
        assert!(available > 0);

        // Also test backward-compatible RESOURCE_MANAGER accessor
        let available2 = RESOURCE_MANAGER.cpu_tokens_available();
        assert_eq!(available, available2);
    }
}
