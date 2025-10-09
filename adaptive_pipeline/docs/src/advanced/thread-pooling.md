# Thread Pooling

**Version:** 0.1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter explores the pipeline's thread pool architecture, configuration strategies, and tuning guidelines for optimal performance across different workload types.

## Overview

The pipeline employs a **dual thread pool** architecture that separates async I/O operations from sync CPU-bound computations:

- **Tokio Runtime**: Handles async I/O operations (file reads/writes, database queries, network)
- **Rayon Thread Pools**: Handles parallel CPU-bound operations (compression, encryption, hashing)

This separation prevents CPU-intensive work from blocking async tasks and ensures optimal resource utilization.

## Thread Pool Architecture

### Dual Pool Design

```text
┌─────────────────────────────────────────────────────────────┐
│                    Async Layer (Tokio)                      │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Multi-threaded Tokio Runtime                        │   │
│  │  - Work-stealing scheduler                           │   │
│  │  - Async I/O operations                              │   │
│  │  - Task coordination                                 │   │
│  │  - Event loop management                             │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                                │
                                │ spawn_blocking()
                                ▼
┌─────────────────────────────────────────────────────────────┐
│                  Sync Layer (Rayon Pools)                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  CPU-Bound Pool (rayon-cpu-{N})                      │   │
│  │  - Compression operations                            │   │
│  │  - Encryption/decryption                             │   │
│  │  - Checksum calculation                              │   │
│  │  - Complex transformations                           │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Mixed Workload Pool (rayon-mixed-{N})               │   │
│  │  - File processing with transformations              │   │
│  │  - Database operations with calculations             │   │
│  │  - Network operations with data processing           │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Thread Naming Convention

All worker threads are named for debugging and profiling:

- **Tokio threads**: Default Tokio naming (`tokio-runtime-worker-{N}`)
- **Rayon CPU pool**: `rayon-cpu-{N}` where N is the thread index
- **Rayon mixed pool**: `rayon-mixed-{N}` where N is the thread index

This naming convention enables:
- Easy identification in profilers (e.g., `perf`, `flamegraph`)
- Clear thread attribution in stack traces
- Simplified debugging of concurrency issues

## Tokio Runtime Configuration

### Default Configuration

The pipeline uses Tokio's default multi-threaded runtime:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Tokio runtime initialized with default settings
    run_pipeline().await
}
```

**Default Tokio Settings:**
- **Worker threads**: Equal to number of CPU cores (via `std::thread::available_parallelism()`)
- **Thread stack size**: 2 MiB per thread
- **Work-stealing**: Enabled (automatic task balancing)
- **I/O driver**: Enabled (async file I/O, network)
- **Time driver**: Enabled (async timers, delays)

### Custom Runtime Configuration

For advanced use cases, you can configure the Tokio runtime explicitly:

```rust
use tokio::runtime::Runtime;

fn create_custom_runtime() -> Runtime {
    Runtime::builder()
        .worker_threads(8)          // Override thread count
        .thread_stack_size(3 * 1024 * 1024) // 3 MiB stack
        .thread_name("pipeline-async")
        .enable_all()               // Enable I/O and time drivers
        .build()
        .expect("Failed to create Tokio runtime")
}
```

**Configuration Parameters:**
- `worker_threads(usize)`: Number of worker threads (defaults to CPU cores)
- `thread_stack_size(usize)`: Stack size per thread in bytes (default: 2 MiB)
- `thread_name(impl Into<String>)`: Thread name prefix for debugging
- `enable_all()`: Enable both I/O and time drivers

## Rayon Thread Pool Configuration

### Global Pool Manager

The pipeline uses a global `RAYON_POOLS` manager with two specialized pools:

```rust
use adaptive_pipeline::infrastructure::config::rayon_config::RAYON_POOLS;

// Access CPU-bound pool
let cpu_pool = RAYON_POOLS.cpu_bound_pool();

// Access mixed workload pool
let mixed_pool = RAYON_POOLS.mixed_workload_pool();
```

**Implementation** (`pipeline/src/infrastructure/config/rayon_config.rs`):

```rust
pub struct RayonPoolManager {
    cpu_bound_pool: Arc<rayon::ThreadPool>,
    mixed_workload_pool: Arc<rayon::ThreadPool>,
}

impl RayonPoolManager {
    pub fn new() -> Result<Self, PipelineError> {
        let available_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(WorkerCount::DEFAULT_WORKERS);

        // CPU-bound pool: Use optimal worker count for CPU-intensive ops
        let cpu_worker_count = WorkerCount::optimal_for_processing_type(
            100 * 1024 * 1024,  // Assume 100MB default file size
            available_cores,
            true,               // CPU-intensive = true
        );

        let cpu_bound_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_worker_count.count())
            .thread_name(|i| format!("rayon-cpu-{}", i))
            .build()?;

        // Mixed workload pool: Use fewer threads to avoid contention
        let mixed_worker_count = (available_cores / 2).max(WorkerCount::MIN_WORKERS);

        let mixed_workload_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(mixed_worker_count)
            .thread_name(|i| format!("rayon-mixed-{}", i))
            .build()?;

        Ok(Self {
            cpu_bound_pool: Arc::new(cpu_bound_pool),
            mixed_workload_pool: Arc::new(mixed_workload_pool),
        })
    }
}

// Global static instance
pub static RAYON_POOLS: std::sync::LazyLock<RayonPoolManager> =
    std::sync::LazyLock::new(|| RayonPoolManager::new()
        .expect("Failed to initialize Rayon pools"));
```

### CPU-Bound Pool

**Purpose**: Maximize throughput for CPU-intensive operations

**Workload Types:**
- Data compression (Brotli, LZ4, Zstandard)
- Data encryption/decryption (AES-256-GCM, ChaCha20-Poly1305)
- Checksum calculation (SHA-256, BLAKE3)
- Complex data transformations

**Thread Count Strategy:**
```rust
// Uses WorkerCount::optimal_for_processing_type()
// For CPU-intensive work, allocates up to available_cores threads
let cpu_worker_count = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    true  // CPU-intensive
);
```

**Typical Thread Counts** (on 8-core system):
- Small files (< 50 MB): 6-14 workers (aggressive parallelism)
- Medium files (50-500 MB): 5-12 workers (balanced approach)
- Large files (> 500 MB): 8-12 workers (moderate parallelism)
- Huge files (> 2 GB): 3-6 workers (conservative, avoid overhead)

### Mixed Workload Pool

**Purpose**: Handle operations with both CPU and I/O components

**Workload Types:**
- File processing with transformations
- Database operations with calculations
- Network operations with data processing

**Thread Count Strategy:**
```rust
// Uses half the cores to avoid contention with I/O operations
let mixed_worker_count = (available_cores / 2).max(WorkerCount::MIN_WORKERS);
```

**Typical Thread Counts** (on 8-core system):
- 4 threads (half of 8 cores)
- Minimum: 1 thread (WorkerCount::MIN_WORKERS)
- Maximum: 16 threads (half of MAX_WORKERS = 32)

## The spawn_blocking Pattern

### Purpose

`tokio::task::spawn_blocking` bridges async and sync code by running synchronous CPU-bound operations on a dedicated blocking thread pool **without blocking the Tokio async runtime**.

### When to Use spawn_blocking

**Always use for:**
- ✅ CPU-intensive operations (compression, encryption, hashing)
- ✅ Blocking I/O that can't be made async
- ✅ Long-running computations (> 10-100 μs)
- ✅ Sync library calls that may block

**Never use for:**
- ❌ Quick operations (< 10 μs)
- ❌ Already async operations
- ❌ Pure data transformations (use inline instead)

### Usage Pattern

**Basic spawn_blocking:**

```rust
use tokio::task;

async fn compress_chunk_async(
    chunk: FileChunk,
    config: &CompressionConfig,
) -> Result<FileChunk, PipelineError> {
    let config = config.clone();

    // Move CPU-bound work to blocking thread pool
    task::spawn_blocking(move || {
        // This runs on a dedicated blocking thread
        compress_chunk_sync(chunk, &config)
    })
    .await
    .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
}

fn compress_chunk_sync(
    chunk: FileChunk,
    config: &CompressionConfig,
) -> Result<FileChunk, PipelineError> {
    // Expensive CPU-bound operation
    // This will NOT block the Tokio async runtime
    let compressed_data = brotli::compress(&chunk.data, config.level)?;
    Ok(FileChunk::new(compressed_data))
}
```

**Rayon inside spawn_blocking** (recommended for batch parallelism):

```rust
async fn compress_chunks_parallel(
    chunks: Vec<FileChunk>,
    config: &CompressionConfig,
) -> Result<Vec<FileChunk>, PipelineError> {
    use rayon::prelude::*;

    let config = config.clone();

    // Entire Rayon batch runs on blocking thread pool
    tokio::task::spawn_blocking(move || {
        // Use CPU-bound pool for compression
        RAYON_POOLS.cpu_bound_pool().install(|| {
            chunks
                .into_par_iter()
                .map(|chunk| compress_chunk_sync(chunk, &config))
                .collect::<Result<Vec<_>, _>>()
        })
    })
    .await
    .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
}
```

**Key Points:**
- `spawn_blocking` returns a `JoinHandle<T>`
- Must `.await` the handle to get the result
- Errors are wrapped in `JoinError` (handle with `map_err`)
- Data must be `Send + 'static` (use `clone()` or `move`)

### spawn_blocking Thread Pool

Tokio maintains a **separate blocking thread pool** for `spawn_blocking`:

- **Default size**: 512 threads (very large to handle many blocking operations)
- **Thread creation**: On-demand (lazy initialization)
- **Thread lifetime**: Threads terminate after being idle for 10 seconds
- **Stack size**: 2 MiB per thread (same as Tokio async threads)

**Why 512 threads?**
- Blocking operations may wait indefinitely (file I/O, database queries)
- Need many threads to prevent starvation
- Threads are cheap (created on-demand, destroyed when idle)

## Worker Count Optimization

### WorkerCount Value Object

The pipeline uses a sophisticated `WorkerCount` value object for adaptive thread allocation:

```rust
use adaptive_pipeline_domain::value_objects::WorkerCount;

// Optimal for file size (empirically validated)
let workers = WorkerCount::optimal_for_file_size(file_size);

// Optimal for file size + system resources
let workers = WorkerCount::optimal_for_file_and_system(file_size, available_cores);

// Optimal for processing type (CPU-intensive vs I/O-intensive)
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    is_cpu_intensive,
);
```

### Optimization Strategies

**Empirically Validated** (based on benchmarks):

| File Size         | Worker Count | Strategy                  | Performance Gain |
|-------------------|--------------|---------------------------|------------------|
| < 1 MB (tiny)     | 1-2          | Minimal parallelism       | N/A              |
| 1-50 MB (small)   | 6-14         | Aggressive parallelism    | +102% (5 MB)     |
| 50-500 MB (medium)| 5-12         | Balanced approach         | +70% (50 MB)     |
| 500 MB-2 GB (large)| 8-12        | Moderate parallelism      | Balanced         |
| > 2 GB (huge)     | 3-6          | Conservative strategy     | +76% (2 GB)      |

**Why these strategies?**

- **Small files**: High parallelism amortizes task overhead quickly
- **Medium files**: Balanced approach for consistent performance
- **Large files**: Moderate parallelism to manage memory and coordination overhead
- **Huge files**: Conservative to avoid excessive memory pressure and thread contention

### Configuration Options

**Via CLI:**

```rust
use bootstrap::config::AppConfig;

let config = AppConfig::builder()
    .app_name("pipeline")
    .worker_threads(8)  // Override automatic detection
    .build();
```

**Via Environment Variable:**

```bash
export ADAPIPE_WORKER_COUNT=8
./pipeline process input.txt
```

**Programmatic:**

```rust
let worker_count = config.worker_threads()
    .map(WorkerCount::new)
    .unwrap_or_else(|| WorkerCount::default_for_system());
```

## Tuning Guidelines

### 1. Start with Defaults

**Recommendation**: Use default settings for most workloads.

```rust
// Let WorkerCount choose optimal values
let workers = WorkerCount::optimal_for_file_size(file_size);
```

**Why?**
- Empirically validated strategies
- Adapts to system resources
- Handles edge cases (tiny files, huge files)

### 2. CPU-Intensive Workloads

**Symptoms:**
- High CPU utilization (> 80%)
- Low I/O wait time
- Operations: Compression, encryption, hashing

**Tuning:**

```rust
// Use CPU-bound pool
RAYON_POOLS.cpu_bound_pool().install(|| {
    // Parallel CPU-intensive work
});

// Or increase worker count
let workers = WorkerCount::new(available_cores);
```

**Guidelines:**
- Match worker count to CPU cores (1x to 1.5x)
- Avoid excessive oversubscription (> 2x cores)
- Monitor context switching with `perf stat`

### 3. I/O-Intensive Workloads

**Symptoms:**
- Low CPU utilization (< 50%)
- High I/O wait time
- Operations: File reads, database queries, network

**Tuning:**

```rust
// Use mixed workload pool or reduce workers
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    false,  // Not CPU-intensive
);
```

**Guidelines:**
- Use fewer workers (0.5x to 1x cores)
- Rely on async I/O instead of parallelism
- Consider increasing I/O buffer sizes

### 4. Memory-Constrained Systems

**Symptoms:**
- High memory usage
- Swapping or OOM errors
- Large files (> 1 GB)

**Tuning:**

```rust
// Reduce worker count to limit memory
let workers = WorkerCount::new(3);  // Conservative

// Or increase chunk size to reduce memory overhead
let chunk_size = ChunkSize::new(64 * 1024 * 1024);  // 64 MB chunks
```

**Guidelines:**
- Limit workers to 3-6 for huge files (> 2 GB)
- Increase chunk size to reduce parallel memory allocations
- Monitor RSS memory with `htop` or `ps`

### 5. Profiling and Measurement

**Tools:**
- **perf**: CPU profiling, context switches, cache misses
- **flamegraph**: Visual CPU time breakdown
- **htop**: Real-time CPU and memory usage
- **tokio-console**: Async task monitoring

**Example: perf stat:**

```bash
perf stat -e cycles,instructions,context-switches,cache-misses \
    ./pipeline process large-file.bin
```

**Metrics to monitor:**
- **CPU utilization**: Should be 70-95% for CPU-bound work
- **Context switches**: High (> 10k/sec) indicates oversubscription
- **Cache misses**: High indicates memory contention
- **Task throughput**: Measure chunks processed per second

## Common Patterns

### Pattern 1: Fan-Out CPU Work

Distribute CPU-bound work across Rayon pool:

```rust
use rayon::prelude::*;

async fn process_chunks_parallel(
    chunks: Vec<FileChunk>,
) -> Result<Vec<FileChunk>, PipelineError> {
    tokio::task::spawn_blocking(move || {
        RAYON_POOLS.cpu_bound_pool().install(|| {
            chunks
                .into_par_iter()
                .map(|chunk| {
                    // CPU-intensive per-chunk work
                    compress_and_encrypt(chunk)
                })
                .collect::<Result<Vec<_>, _>>()
        })
    })
    .await??
}
```

### Pattern 2: Bounded Parallelism

Limit concurrent CPU-bound tasks with semaphore:

```rust
use tokio::sync::Semaphore;

async fn process_with_limit(
    chunks: Vec<FileChunk>,
    max_parallel: usize,
) -> Result<Vec<FileChunk>, PipelineError> {
    let semaphore = Arc::new(Semaphore::new(max_parallel));

    let futures = chunks.into_iter().map(|chunk| {
        let permit = semaphore.clone();
        async move {
            let _guard = permit.acquire().await.unwrap();

            // Run CPU-bound work on blocking pool
            tokio::task::spawn_blocking(move || {
                compress_chunk_sync(chunk)
            })
            .await?
        }
    });

    futures::future::try_join_all(futures).await
}
```

### Pattern 3: Mixed Async/Sync Pipeline

Combine async I/O with sync CPU-bound stages:

```rust
async fn process_file_pipeline(path: &Path) -> Result<(), PipelineError> {
    // Stage 1: Async file read
    let chunks = read_file_chunks(path).await?;

    // Stage 2: Sync CPU-bound processing (spawn_blocking + Rayon)
    let processed = tokio::task::spawn_blocking(move || {
        RAYON_POOLS.cpu_bound_pool().install(|| {
            chunks.into_par_iter()
                .map(|chunk| compress_and_encrypt(chunk))
                .collect::<Result<Vec<_>, _>>()
        })
    })
    .await??;

    // Stage 3: Async file write
    write_chunks(path, processed).await?;

    Ok(())
}
```

## Performance Characteristics

### Thread Creation Overhead

| Operation                          | Time        | Notes                              |
|------------------------------------|-------------|------------------------------------|
| Tokio runtime initialization       | ~1-5 ms     | One-time cost at startup           |
| Rayon pool creation                | ~500 μs     | One-time cost (global static)      |
| spawn_blocking task                | ~10-50 μs   | Per-task overhead                  |
| Rayon parallel iteration           | ~5-20 μs    | Per-iteration overhead             |
| Thread context switch              | ~1-5 μs     | Depends on workload and OS         |

**Guidelines:**
- Amortize overhead with work units > 100 μs
- Batch small operations to reduce per-task overhead
- Avoid spawning tasks for trivial work (< 10 μs)

### Scalability

**Linear Scaling** (ideal):
- CPU-bound operations with independent chunks
- Small files (< 50 MB) with 6-14 workers
- Minimal memory contention

**Sub-Linear Scaling** (common):
- Large files (> 500 MB) due to memory bandwidth limits
- Mixed workloads with I/O contention
- High worker counts (> 12) with coordination overhead

**Performance Cliff** (avoid):
- Excessive worker count (> 2x CPU cores)
- Memory pressure causing swapping
- Lock contention in shared data structures

## Best Practices

### 1. Use Appropriate Thread Pool

```rust
// ✅ Good: CPU-intensive work on CPU-bound pool
RAYON_POOLS.cpu_bound_pool().install(|| {
    chunks.par_iter().map(|c| compress(c)).collect()
});

// ❌ Bad: Using wrong pool or no pool
chunks.iter().map(|c| compress(c)).collect()  // Sequential!
```

### 2. Wrap Rayon with spawn_blocking

```rust
// ✅ Good: Rayon work inside spawn_blocking
tokio::task::spawn_blocking(move || {
    RAYON_POOLS.cpu_bound_pool().install(|| {
        // Parallel work
    })
})
.await?

// ❌ Bad: Rayon work directly in async context
RAYON_POOLS.cpu_bound_pool().install(|| {
    // This blocks the async runtime!
})
```

### 3. Let WorkerCount Optimize

```rust
// ✅ Good: Use empirically validated strategies
let workers = WorkerCount::optimal_for_file_size(file_size);

// ❌ Bad: Arbitrary fixed count
let workers = 8;  // May be too many or too few!
```

### 4. Monitor and Measure

```rust
// ✅ Good: Measure actual performance
let start = Instant::now();
let result = process_chunks(chunks).await?;
let duration = start.elapsed();
info!("Processed {} chunks in {:?}", chunks.len(), duration);

// ❌ Bad: Assume defaults are optimal without measurement
```

### 5. Avoid Oversubscription

```rust
// ✅ Good: Bounded parallelism based on cores
let max_workers = available_cores.min(WorkerCount::MAX_WORKERS);

// ❌ Bad: Unbounded parallelism
let workers = chunks.len();  // Could be thousands!
```

## Troubleshooting

### Issue 1: High Context Switching

**Symptoms:**
- `perf stat` shows > 10k context switches/sec
- High CPU usage but low throughput

**Causes:**
- Too many worker threads (> 2x cores)
- Rayon pool size exceeds optimal

**Solutions:**
```rust
// Reduce Rayon pool size
let workers = WorkerCount::new(available_cores);  // Not 2x

// Or use sequential processing for small workloads
if chunks.len() < 10 {
    chunks.into_iter().map(process).collect()
} else {
    chunks.into_par_iter().map(process).collect()
}
```

### Issue 2: spawn_blocking Blocking Async Runtime

**Symptoms:**
- Async tasks become slow
- Other async operations stall

**Causes:**
- Long-running CPU work directly in async fn (no spawn_blocking)
- Blocking I/O in async context

**Solutions:**
```rust
// ✅ Good: Use spawn_blocking for CPU work
async fn process_chunk(chunk: FileChunk) -> Result<FileChunk> {
    tokio::task::spawn_blocking(move || {
        // CPU-intensive work here
        compress_chunk_sync(chunk)
    })
    .await?
}

// ❌ Bad: CPU work blocking async runtime
async fn process_chunk(chunk: FileChunk) -> Result<FileChunk> {
    compress_chunk_sync(chunk)  // Blocks entire runtime!
}
```

### Issue 3: Memory Pressure with Many Workers

**Symptoms:**
- High memory usage (> 80% RAM)
- Swapping or OOM errors

**Causes:**
- Too many concurrent chunks allocated
- Large chunk size × high worker count

**Solutions:**
```rust
// Reduce worker count for large files
let workers = if file_size > 2 * GB {
    WorkerCount::new(3)  // Conservative for huge files
} else {
    WorkerCount::optimal_for_file_size(file_size)
};

// Or reduce chunk size
let chunk_size = ChunkSize::new(16 * MB);  // Smaller chunks
```

## Related Topics

- See [Concurrency Model](concurrency.md) for async/await patterns
- See [Resource Management](resources.md) for memory and task limits
- See [Performance Optimization](../advanced/performance.md) for benchmarking strategies
- See [Profiling](../advanced/profiling.md) for detailed performance analysis

## Summary

The pipeline's dual thread pool architecture provides:

1. **Tokio Runtime**: Async I/O operations with work-stealing scheduler
2. **Rayon Pools**: Parallel CPU-bound work with specialized pools
3. **spawn_blocking**: Bridge between async and sync without blocking
4. **WorkerCount**: Empirically validated thread allocation strategies
5. **Tuning**: Guidelines for CPU-intensive, I/O-intensive, and memory-constrained workloads

**Key Takeaways:**
- Use CPU-bound pool for compression, encryption, hashing
- Wrap Rayon work with `spawn_blocking` in async contexts
- Let `WorkerCount` optimize based on file size and system resources
- Monitor performance with `perf`, `flamegraph`, and metrics
- Tune based on actual measurements, not assumptions
