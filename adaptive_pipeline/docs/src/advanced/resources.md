# Resource Management

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter explores the pipeline's resource management system, including CPU and I/O token governance, memory tracking, and resource optimization strategies for different workload types.

## Overview

The pipeline employs a **two-level resource governance** architecture that prevents system oversubscription when processing multiple files concurrently:

1. **Global Resource Manager**: Caps total system resources (CPU tokens, I/O tokens, memory)
2. **Local Resource Limits**: Caps per-file concurrency (semaphores within each file's processing)

This two-level approach ensures optimal resource utilization without overwhelming the system.

### Why Resource Governance?

**Problem without limits:**
```text
10 files × 8 workers/file = 80 concurrent tasks on an 8-core machine
Result: CPU oversubscription, cache thrashing, poor throughput
```

**Solution with resource governance:**
```text
Global CPU tokens: 7 (cores - 1)
Maximum 7 concurrent CPU-intensive tasks across all files
Result: Optimal CPU utilization, consistent performance
```

## Resource Management Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│              Global Resource Manager                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  CPU Tokens (Semaphore)                              │   │
│  │  - Default: cores - 1                                │   │
│  │  - Purpose: Prevent CPU oversubscription             │   │
│  │  - Used by: Rayon workers, CPU-bound operations      │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  I/O Tokens (Semaphore)                              │   │
│  │  - Default: Device-specific (NVMe:24, SSD:12, HDD:4) │   │
│  │  - Purpose: Prevent I/O queue overrun                │   │
│  │  - Used by: File reads/writes                        │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Memory Tracking (AtomicUsize)                       │   │
│  │  - Default: 40 GB capacity                           │   │
│  │  - Purpose: Monitor memory pressure (gauge only)     │   │
│  │  - Used by: Memory allocation/deallocation           │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                                │
                                │ acquire_cpu() / acquire_io()
                                ▼
┌─────────────────────────────────────────────────────────────┐
│              File-Level Processing                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Per-File Semaphore                                  │   │
│  │  - Local concurrency limit (e.g., 8 workers/file)    │   │
│  │  - Prevents single file from saturating system       │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## CPU Token Management

### Overview

**CPU tokens** limit the total number of concurrent CPU-bound operations across all files being processed. This prevents CPU oversubscription and cache thrashing.

**Implementation:** Semaphore-based with RAII (Resource Acquisition Is Initialization)

### Configuration

**Default CPU Token Count:**
```rust
// Automatic detection: cores - 1
let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
let cpu_token_count = (available_cores - 1).max(1);

// Example on 8-core machine: 7 CPU tokens
```

**Why `cores - 1`?**
- Leave one core for OS, I/O threads, and system tasks
- Prevents complete CPU saturation
- Improves overall system responsiveness
- Reduces context switching overhead

**Custom Configuration:**
```rust
use adaptive_pipeline::infrastructure::runtime::{init_resource_manager, ResourceConfig};

// Initialize with custom CPU token count
let config = ResourceConfig {
    cpu_tokens: Some(6),  // Override: use 6 CPU workers
    ..Default::default()
};

init_resource_manager(config)?;
```

### Usage Pattern

**Acquire CPU token before CPU-intensive work:**

```rust
use adaptive_pipeline::infrastructure::runtime::RESOURCE_MANAGER;

async fn process_chunk(chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    // 1. Acquire global CPU token (waits if system is saturated)
    let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;

    // 2. Do CPU-intensive work (compression, encryption, hashing)
    tokio::task::spawn_blocking(move || {
        RAYON_POOLS.cpu_bound_pool().install(|| {
            compress_and_encrypt(chunk)
        })
    })
    .await?

    // 3. Permit released automatically when _cpu_permit goes out of scope (RAII)
}
```

**Key Points:**
- `acquire_cpu()` returns a `SemaphorePermit` guard
- If all CPU tokens are in use, the call **waits** (backpressure)
- Permit is auto-released when dropped (RAII pattern)
- This creates natural flow control and prevents oversubscription

### Backpressure Mechanism

When all CPU tokens are in use:

```text
Timeline:
─────────────────────────────────────────────────────────────
Task 1: [===CPU work===]         (acquired token)
Task 2:   [===CPU work===]       (acquired token)
Task 3:     [===CPU work===]     (acquired token)
...
Task 7:             [===CPU===]  (acquired token - last one!)
Task 8:               ⏳⏳⏳     (waiting for token...)
Task 9:                 ⏳⏳⏳   (waiting for token...)
─────────────────────────────────────────────────────────────

When Task 1 completes → Task 8 acquires the released token
When Task 2 completes → Task 9 acquires the released token
```

This backpressure prevents overwhelming the CPU with too many concurrent tasks.

### Monitoring CPU Saturation

```rust
use adaptive_pipeline::infrastructure::metrics::CONCURRENCY_METRICS;

// Check CPU saturation percentage
let saturation = CONCURRENCY_METRICS.cpu_saturation_percent();

if saturation > 80.0 {
    println!("CPU-saturated: {}%", saturation);
    println!("Available tokens: {}", RESOURCE_MANAGER.cpu_tokens_available());
    println!("Total tokens: {}", RESOURCE_MANAGER.cpu_tokens_total());
}
```

**Saturation Interpretation:**
- **0-50%**: Underutilized, could increase worker count
- **50-80%**: Good utilization, healthy balance
- **80-95%**: High utilization, approaching saturation
- **95-100%**: Fully saturated, tasks frequently waiting

## I/O Token Management

### Overview

**I/O tokens** limit the total number of concurrent I/O operations to prevent overwhelming the storage device's queue depth.

**Why separate from CPU tokens?**
- I/O and CPU have different characteristics
- Storage devices have specific optimal queue depths
- Prevents I/O queue saturation independent of CPU usage

### Device-Specific Optimization

Different storage devices have different optimal I/O queue depths:

| Storage Type | Queue Depth | Rationale                                    |
|--------------|-------------|----------------------------------------------|
| **NVMe**     | 24          | Multiple parallel channels, low latency      |
| **SSD**      | 12          | Medium parallelism, good random access       |
| **HDD**      | 4           | Sequential access preferred, high seek latency |
| **Auto**     | 12          | Conservative default (assumes SSD)           |

**Implementation** (`pipeline/src/infrastructure/runtime/resource_manager.rs`):

```rust
pub enum StorageType {
    NVMe,   // 24 tokens
    SSD,    // 12 tokens
    HDD,    // 4 tokens
    Auto,   // 12 tokens (SSD default)
    Custom(usize),
}

fn detect_optimal_io_tokens(storage_type: StorageType) -> usize {
    match storage_type {
        StorageType::NVMe => 24,
        StorageType::SSD => 12,
        StorageType::HDD => 4,
        StorageType::Auto => 12,  // Conservative default
        StorageType::Custom(n) => n,
    }
}
```

### Configuration

**Custom I/O token count:**

```rust
let config = ResourceConfig {
    io_tokens: Some(24),              // Override: NVMe-optimized
    storage_type: StorageType::NVMe,
    ..Default::default()
};

init_resource_manager(config)?;
```

### Usage Pattern

**Acquire I/O token before file operations:**

```rust
async fn read_file_chunk(path: &Path, offset: u64, size: usize)
    -> Result<Vec<u8>, PipelineError>
{
    // 1. Acquire global I/O token (waits if I/O queue is full)
    let _io_permit = RESOURCE_MANAGER.acquire_io().await?;

    // 2. Perform I/O operation
    let mut file = tokio::fs::File::open(path).await?;
    file.seek(SeekFrom::Start(offset)).await?;

    let mut buffer = vec![0u8; size];
    file.read_exact(&mut buffer).await?;

    Ok(buffer)

    // 3. Permit auto-released when _io_permit goes out of scope
}
```

### Monitoring I/O Saturation

```rust
// Check I/O saturation
let io_saturation = CONCURRENCY_METRICS.io_saturation_percent();

if io_saturation > 80.0 {
    println!("I/O-saturated: {}%", io_saturation);
    println!("Available tokens: {}", RESOURCE_MANAGER.io_tokens_available());
    println!("Consider reducing concurrent I/O or optimizing chunk size");
}
```

## Memory Management

### Overview

The pipeline uses **memory tracking** (not enforcement) to monitor memory pressure and provide visibility into resource usage.

**Design Philosophy:**
- **Phase 1**: Monitor memory usage (current implementation)
- **Phase 2**: Soft limits with warnings
- **Phase 3**: Hard limits with enforcement (future)

**Why start with monitoring only?**
- Memory is harder to predict and control than CPU/I/O
- Avoids complexity in initial implementation
- Allows gathering real-world usage data before adding limits

### Memory Tracking

**Atomic counter** (`AtomicUsize`) tracks allocated memory:

```rust
use adaptive_pipeline::infrastructure::runtime::RESOURCE_MANAGER;

// Track memory allocation
let chunk_size = 1024 * 1024;  // 1 MB
RESOURCE_MANAGER.allocate_memory(chunk_size);

// ... use memory ...

// Track memory deallocation
RESOURCE_MANAGER.deallocate_memory(chunk_size);
```

**Usage with RAII guard:**

```rust
pub struct MemoryGuard {
    size: usize,
}

impl MemoryGuard {
    pub fn new(size: usize) -> Self {
        RESOURCE_MANAGER.allocate_memory(size);
        Self { size }
    }
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        RESOURCE_MANAGER.deallocate_memory(self.size);
    }
}

// Usage
let _guard = MemoryGuard::new(chunk_size);
// Memory automatically tracked on allocation and deallocation
```

### Configuration

**Set memory capacity:**

```rust
let config = ResourceConfig {
    memory_limit: Some(40 * 1024 * 1024 * 1024),  // 40 GB capacity
    ..Default::default()
};

init_resource_manager(config)?;
```

### Monitoring Memory Usage

```rust
use adaptive_pipeline::infrastructure::metrics::CONCURRENCY_METRICS;

// Check current memory usage
let used_bytes = RESOURCE_MANAGER.memory_used();
let used_mb = CONCURRENCY_METRICS.memory_used_mb();
let utilization = CONCURRENCY_METRICS.memory_utilization_percent();

println!("Memory: {:.2} MB / {} MB ({:.1}%)",
    used_mb,
    RESOURCE_MANAGER.memory_capacity() / (1024 * 1024),
    utilization
);

// Alert on high memory usage
if utilization > 80.0 {
    println!("⚠️  High memory usage: {:.1}%", utilization);
    println!("Consider reducing chunk size or worker count");
}
```

## Concurrency Metrics

### Overview

The pipeline provides comprehensive metrics for monitoring resource utilization, wait times, and saturation levels.

**Metric Types:**
- **Gauges**: Instant values (e.g., CPU tokens available)
- **Counters**: Cumulative values (e.g., total wait time)
- **Histograms**: Distribution of values (e.g., P50/P95/P99 wait times)

### CPU Metrics

```rust
use adaptive_pipeline::infrastructure::metrics::CONCURRENCY_METRICS;

// Instant metrics (gauges)
let cpu_available = CONCURRENCY_METRICS.cpu_tokens_available();
let cpu_saturation = CONCURRENCY_METRICS.cpu_saturation_percent();

// Wait time percentiles (histograms)
let p50 = CONCURRENCY_METRICS.cpu_wait_p50();  // Median wait time (ms)
let p95 = CONCURRENCY_METRICS.cpu_wait_p95();  // 95th percentile (ms)
let p99 = CONCURRENCY_METRICS.cpu_wait_p99();  // 99th percentile (ms)

println!("CPU Saturation: {:.1}%", cpu_saturation);
println!("Wait times: P50={} ms, P95={} ms, P99={} ms", p50, p95, p99);
```

**Why percentiles matter:**

Averages hide problems:
- Average wait: 10 ms (looks fine)
- P99 wait: 500 ms (users experience terrible latency!)

Histograms show the full distribution and reveal tail latencies.

### I/O Metrics

```rust
// I/O saturation and wait times
let io_saturation = CONCURRENCY_METRICS.io_saturation_percent();
let io_p95 = CONCURRENCY_METRICS.io_wait_p95();

if io_saturation > 80.0 && io_p95 > 50 {
    println!("⚠️  I/O bottleneck detected:");
    println!("  Saturation: {:.1}%", io_saturation);
    println!("  P95 wait time: {} ms", io_p95);
}
```

### Worker Metrics

```rust
// Worker tracking
let active = CONCURRENCY_METRICS.active_workers();
let spawned = CONCURRENCY_METRICS.tasks_spawned();
let completed = CONCURRENCY_METRICS.tasks_completed();

println!("Workers: {} active, {} spawned, {} completed",
    active, spawned, completed);

// Queue depth (backpressure indicator)
let queue_depth = CONCURRENCY_METRICS.cpu_queue_depth();
let queue_max = CONCURRENCY_METRICS.cpu_queue_depth_max();

if queue_depth > 100 {
    println!("⚠️  High queue depth: {} (max: {})", queue_depth, queue_max);
    println!("Workers can't keep up with reader - increase workers or optimize stages");
}
```

### Queue Metrics

**Queue depth** reveals whether workers can keep up with the reader:

- **Depth near 0**: Workers are faster than reader (good!)
- **Depth near capacity**: Workers are bottleneck (increase workers or optimize stages)
- **Depth at capacity**: Reader is blocked (severe backpressure)

```rust
// Queue wait time distribution
let queue_p50 = CONCURRENCY_METRICS.cpu_queue_wait_p50();
let queue_p95 = CONCURRENCY_METRICS.cpu_queue_wait_p95();
let queue_p99 = CONCURRENCY_METRICS.cpu_queue_wait_p99();

println!("Queue wait: P50={} ms, P95={} ms, P99={} ms",
    queue_p50, queue_p95, queue_p99);
```

## Resource Configuration

### ResourceConfig Structure

```rust
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
```

### Initialization Pattern

**In `main()`, before any operations:**

```rust
use adaptive_pipeline::infrastructure::runtime::{init_resource_manager, ResourceConfig, StorageType};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize resource manager with configuration
    let config = ResourceConfig {
        cpu_tokens: Some(6),              // 6 CPU workers
        io_tokens: Some(24),              // NVMe-optimized
        storage_type: StorageType::NVMe,
        memory_limit: Some(40 * 1024 * 1024 * 1024),  // 40 GB
    };

    init_resource_manager(config)
        .map_err(|e| anyhow!("Failed to initialize resources: {}", e))?;

    // 2. Now safe to use RESOURCE_MANAGER anywhere
    run_pipeline().await
}
```

**Global access pattern:**

```rust
async fn my_function() {
    // Access global resource manager
    let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;
    // ... CPU work ...
}
```

## Tuning Guidelines

### 1. CPU-Bound Workloads

**Symptoms:**
- High CPU saturation (> 80%)
- Low CPU wait times (< 10 ms P95)
- Heavy compression, encryption, or hashing

**Tuning:**

```rust
// Increase CPU tokens to match cores (remove safety margin)
let config = ResourceConfig {
    cpu_tokens: Some(available_cores),  // Use all cores
    ..Default::default()
};
```

**Trade-offs:**
- ✅ Higher throughput for CPU-bound work
- ❌ Reduced system responsiveness
- ❌ Higher context switching overhead

### 2. I/O-Bound Workloads

**Symptoms:**
- High I/O saturation (> 80%)
- High I/O wait times (> 50 ms P95)
- Many concurrent file operations

**Tuning:**

```rust
// Increase I/O tokens for high-throughput storage
let config = ResourceConfig {
    io_tokens: Some(32),              // Higher than default
    storage_type: StorageType::Custom(32),
    ..Default::default()
};
```

**Monitoring:**

```bash
# Check I/O queue depth on Linux
iostat -x 1

# Look for:
# - avgqu-sz (average queue size) - should be < I/O tokens
# - %util (device utilization) - should be 70-90%
```

### 3. Memory-Constrained Systems

**Symptoms:**
- High memory utilization (> 80%)
- Swapping or OOM errors
- Processing large files

**Tuning:**

```rust
// Reduce chunk size to lower memory usage
let chunk_size = ChunkSize::new(16 * 1024 * 1024);  // 16 MB (smaller)

// Reduce worker count to limit concurrent chunks
let config = ResourceConfig {
    cpu_tokens: Some(3),  // Fewer workers = less memory
    ..Default::default()
};
```

**Formula:**
```text
Peak Memory ≈ chunk_size × cpu_tokens × files_processed_concurrently

Example:
  chunk_size = 64 MB
  cpu_tokens = 7
  files = 3
  Peak Memory ≈ 64 MB × 7 × 3 = 1.3 GB
```

### 4. Mixed Workloads

**Symptoms:**
- Both CPU and I/O saturation
- Variable chunk processing times
- Compression + file I/O operations

**Tuning:**

```rust
// Balanced configuration
let config = ResourceConfig {
    cpu_tokens: Some(available_cores - 1),  // Leave headroom
    io_tokens: Some(12),                    // Moderate I/O concurrency
    storage_type: StorageType::SSD,
    ..Default::default()
};
```

**Best practices:**
- Monitor both CPU and I/O saturation
- Adjust based on bottleneck (CPU vs I/O)
- Use Rayon's mixed workload pool for hybrid operations

### 5. Multi-File Processing

**Symptoms:**
- Processing many files concurrently
- High queue depths
- Resource contention between files

**Tuning:**

```rust
// Global limits prevent oversubscription
let config = ResourceConfig {
    cpu_tokens: Some(7),   // Total across ALL files
    io_tokens: Some(12),   // Total across ALL files
    ..Default::default()
};

// Per-file semaphores limit individual file's concurrency
async fn process_file(path: &Path) -> Result<()> {
    let file_semaphore = Arc::new(Semaphore::new(4));  // Max 4 workers/file

    for chunk in chunks {
        let _global_cpu = RESOURCE_MANAGER.acquire_cpu().await?;  // Global limit
        let _local = file_semaphore.acquire().await?;             // Local limit

        // Process chunk...
    }

    Ok(())
}
```

**Two-level governance:**
- **Global**: Prevents system oversubscription (7 CPU tokens total)
- **Local**: Prevents single file from monopolizing resources (4 workers/file)

## Performance Characteristics

### Resource Acquisition Overhead

| Operation                     | Time       | Notes                           |
|-------------------------------|------------|---------------------------------|
| acquire_cpu() (fast path)     | ~100 ns    | Token immediately available     |
| acquire_cpu() (slow path)     | ~1-50 ms   | Must wait for token             |
| acquire_io() (fast path)      | ~100 ns    | Token immediately available     |
| allocate_memory() tracking    | ~10 ns     | Atomic increment (Relaxed)      |
| Metrics update                | ~50 ns     | Atomic operations               |

**Guidelines:**
- Fast path is negligible overhead
- Slow path (waiting) creates backpressure (intentional)
- Memory tracking is extremely low overhead

### Scalability

**Linear scaling** (ideal):
- CPU tokens = available cores
- I/O tokens matched to device queue depth
- Minimal waiting for resources

**Sub-linear scaling** (common with oversubscription):
- Too many CPU tokens (> 2x cores)
- Excessive context switching
- Cache thrashing

**Performance cliff** (avoid):
- No resource limits
- Uncontrolled parallelism
- System thrashing (swapping, CPU saturation)

## Best Practices

### 1. Always Acquire Resources Before Work

```rust
// ✅ Good: Acquire global resource token before work
async fn process_chunk(chunk: FileChunk) -> Result<FileChunk> {
    let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;

    tokio::task::spawn_blocking(move || {
        // CPU-intensive work
    }).await?
}

// ❌ Bad: No resource governance
async fn process_chunk(chunk: FileChunk) -> Result<FileChunk> {
    tokio::task::spawn_blocking(move || {
        // Uncontrolled parallelism!
    }).await?
}
```

### 2. Use RAII for Automatic Release

```rust
// ✅ Good: RAII guard auto-releases
let _permit = RESOURCE_MANAGER.acquire_cpu().await?;
// Work here...
// Permit released automatically when _permit goes out of scope

// ❌ Bad: Manual release (error-prone)
// Don't do this - no manual release API
```

### 3. Monitor Saturation Regularly

```rust
// ✅ Good: Periodic monitoring
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;

        let cpu_sat = CONCURRENCY_METRICS.cpu_saturation_percent();
        let io_sat = CONCURRENCY_METRICS.io_saturation_percent();
        let mem_util = CONCURRENCY_METRICS.memory_utilization_percent();

        info!("Resources: CPU={:.1}%, I/O={:.1}%, Mem={:.1}%",
            cpu_sat, io_sat, mem_util);
    }
});
```

### 4. Configure Based on Workload Type

```rust
// ✅ Good: Workload-specific configuration
let config = if is_cpu_intensive {
    ResourceConfig {
        cpu_tokens: Some(available_cores),
        io_tokens: Some(8),  // Lower I/O concurrency
        ..Default::default()
    }
} else {
    ResourceConfig {
        cpu_tokens: Some(available_cores / 2),
        io_tokens: Some(24),  // Higher I/O concurrency
        ..Default::default()
    }
};
```

### 5. Track Memory with Guards

```rust
// ✅ Good: RAII memory guard
pub struct ChunkBuffer {
    data: Vec<u8>,
    _memory_guard: MemoryGuard,
}

impl ChunkBuffer {
    pub fn new(size: usize) -> Self {
        let data = vec![0u8; size];
        let _memory_guard = MemoryGuard::new(size);
        Self { data, _memory_guard }
    }
}
// Memory automatically tracked on allocation and freed on drop
```

## Troubleshooting

### Issue 1: High CPU Wait Times

**Symptoms:**
- P95 CPU wait time > 50 ms
- Low CPU saturation (< 50%)
- Many tasks waiting for CPU tokens

**Causes:**
- Too few CPU tokens configured
- CPU tokens not matching actual cores

**Solutions:**

```rust
// Check current configuration
println!("CPU tokens: {}", RESOURCE_MANAGER.cpu_tokens_total());
println!("CPU saturation: {:.1}%", CONCURRENCY_METRICS.cpu_saturation_percent());
println!("CPU wait P95: {} ms", CONCURRENCY_METRICS.cpu_wait_p95());

// Increase CPU tokens
let config = ResourceConfig {
    cpu_tokens: Some(available_cores),  // Increase from cores-1
    ..Default::default()
};
init_resource_manager(config)?;
```

### Issue 2: I/O Queue Saturation

**Symptoms:**
- I/O saturation > 90%
- High I/O wait times (> 100 ms P95)
- `iostat` shows high `avgqu-sz`

**Causes:**
- Too many I/O tokens for storage device
- Storage device can't handle queue depth
- Sequential I/O on HDD with high concurrency

**Solutions:**

```rust
// Reduce I/O tokens for HDD
let config = ResourceConfig {
    storage_type: StorageType::HDD,  // Sets I/O tokens = 4
    ..Default::default()
};

// Or manually configure
let config = ResourceConfig {
    io_tokens: Some(4),  // Lower concurrency
    ..Default::default()
};
```

### Issue 3: Memory Pressure

**Symptoms:**
- Memory utilization > 80%
- Swapping (check `vmstat`)
- OOM killer activated

**Causes:**
- Too many concurrent chunks allocated
- Large chunk size × high worker count
- Memory leaks (not tracked properly)

**Solutions:**

```rust
// Reduce memory usage
let chunk_size = ChunkSize::new(16 * 1024 * 1024);  // Smaller chunks

let config = ResourceConfig {
    cpu_tokens: Some(3),  // Fewer concurrent chunks
    ..Default::default()
};

// Monitor memory closely
let mem_mb = CONCURRENCY_METRICS.memory_used_mb();
let mem_pct = CONCURRENCY_METRICS.memory_utilization_percent();

if mem_pct > 80.0 {
    error!("⚠️  High memory usage: {:.1}% ({:.2} MB)", mem_pct, mem_mb);
}
```

### Issue 4: High Queue Depth

**Symptoms:**
- CPU queue depth > 100
- High queue wait times (> 50 ms P95)
- Reader blocked waiting for workers

**Causes:**
- Workers can't keep up with reader
- Slow compression/encryption stages
- Insufficient worker count

**Solutions:**

```rust
// Check queue metrics
let depth = CONCURRENCY_METRICS.cpu_queue_depth();
let max_depth = CONCURRENCY_METRICS.cpu_queue_depth_max();
let wait_p95 = CONCURRENCY_METRICS.cpu_queue_wait_p95();

println!("Queue depth: {} (max: {})", depth, max_depth);
println!("Queue wait P95: {} ms", wait_p95);

// Increase workers to drain queue faster
let config = ResourceConfig {
    cpu_tokens: Some(available_cores),  // More workers
    ..Default::default()
};

// Or optimize stages (faster compression, fewer stages)
```

## Related Topics

- See [Thread Pooling](thread-pooling.md) for worker pool configuration
- See [Concurrency Model](concurrency.md) for async/await patterns
- See [Performance Optimization](../advanced/performance.md) for benchmarking
- See [Profiling](../advanced/profiling.md) for performance analysis

## Summary

The pipeline's resource management system provides:

1. **CPU Token Management**: Prevent CPU oversubscription with semaphore-based limits
2. **I/O Token Management**: Device-specific I/O queue depth optimization
3. **Memory Tracking**: Monitor memory usage with atomic counters (gauge only)
4. **Concurrency Metrics**: Comprehensive observability (gauges, counters, histograms)
5. **Two-Level Governance**: Global + local limits prevent system saturation

**Key Takeaways:**
- Always acquire resource tokens before CPU/I/O work
- Use RAII guards for automatic resource release
- Monitor saturation percentages and wait time percentiles
- Configure based on workload type (CPU-intensive vs I/O-intensive)
- Start with defaults, tune based on actual measurements
- Memory tracking is informational only (no enforcement yet)
