# Concurrency Model

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a comprehensive overview of the concurrency model in the adaptive pipeline system. Learn how async/await, Tokio runtime, and concurrent patterns enable high-performance, scalable file processing.

---

## Table of Contents

- [Overview](#overview)
- [Concurrency Architecture](#concurrency-architecture)
- [Async/Await Model](#asyncawait-model)
- [Tokio Runtime](#tokio-runtime)
- [Parallel Chunk Processing](#parallel-chunk-processing)
- [Concurrency Primitives](#concurrency-primitives)
- [Thread Pools and Workers](#thread-pools-and-workers)
- [Resource Management](#resource-management)
- [Concurrency Patterns](#concurrency-patterns)
- [Performance Considerations](#performance-considerations)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

---

## Overview

The **concurrency model** enables the adaptive pipeline to process files efficiently through parallel processing, async I/O, and concurrent chunk handling. The system uses Rust's async/await with the Tokio runtime for high-performance, scalable concurrency.

### Key Features

- **Async/Await**: Non-blocking asynchronous operations
- **Tokio Runtime**: Multi-threaded async runtime
- **Parallel Processing**: Concurrent chunk processing
- **Worker Pools**: Configurable thread pools
- **Resource Management**: Efficient resource allocation and cleanup
- **Thread Safety**: Safe concurrent access through Rust's type system

### Concurrency Stack

```text
┌─────────────────────────────────────────────────────────────┐
│                  Application Layer                          │
│  - Pipeline orchestration                                   │
│  - File processing coordination                             │
└─────────────────────────────────────────────────────────────┘
                         ↓ async
┌─────────────────────────────────────────────────────────────┐
│                   Tokio Runtime                             │
│  - Multi-threaded work-stealing scheduler                   │
│  - Async task execution                                     │
│  - I/O reactor                                              │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Concurrency Primitives                         │
│  ┌─────────┬──────────┬──────────┬─────────────┐           │
│  │ Mutex   │ RwLock   │ Semaphore│  Channel    │           │
│  │ (Sync)  │ (Shared) │ (Limit)  │ (Message)   │           │
│  └─────────┴──────────┴──────────┴─────────────┘           │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│                  Worker Threads                             │
│  - Chunk processing workers                                 │
│  - I/O workers                                              │
│  - Background tasks                                         │
└─────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Async-First**: All I/O operations are asynchronous
2. **Structured Concurrency**: Clear task ownership and lifetimes
3. **Safe Sharing**: Thread-safe sharing through Arc and sync primitives
4. **Resource Bounded**: Limited resource usage with semaphores
5. **Zero-Cost Abstractions**: Minimal overhead from async runtime

---

## Concurrency Architecture

The system uses a layered concurrency architecture with clear separation between sync and async code.

### Architectural Layers

```text
┌─────────────────────────────────────────────────────────────┐
│ Async Layer (I/O Bound)                                     │
│  ┌──────────────────────────────────────────────────┐      │
│  │  File I/O Service (async)                        │      │
│  │  - tokio::fs file operations                     │      │
│  │  - Async read/write                              │      │
│  └──────────────────────────────────────────────────┘      │
│  ┌──────────────────────────────────────────────────┐      │
│  │  Pipeline Service (async orchestration)          │      │
│  │  - Async workflow coordination                   │      │
│  │  - Task spawning and management                  │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Sync Layer (CPU Bound)                                      │
│  ┌──────────────────────────────────────────────────┐      │
│  │  Compression Service (sync)                      │      │
│  │  - CPU-bound compression algorithms              │      │
│  │  - No async overhead                             │      │
│  └──────────────────────────────────────────────────┘      │
│  ┌──────────────────────────────────────────────────┐      │
│  │  Encryption Service (sync)                       │      │
│  │  - CPU-bound encryption algorithms               │      │
│  │  - No async overhead                             │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Async vs Sync Decision

**Async for:**
- File I/O (tokio::fs)
- Network I/O
- Database operations
- Long-running waits

**Sync for:**
- CPU-bound compression
- CPU-bound encryption
- Hash calculations
- Pure computation

---

## Async/Await Model

The system uses Rust's async/await for non-blocking concurrency.

### Async Functions

```rust
// Async function definition
async fn process_file(
    path: &Path,
    chunk_size: ChunkSize,
) -> Result<Vec<FileChunk>, PipelineError> {
    // Await async operations
    let chunks = read_file_chunks(path, chunk_size).await?;

    // Process chunks
    let results = process_chunks_parallel(chunks).await?;

    Ok(results)
}
```

### Awaiting Futures

```rust
// Sequential awaits
let chunks = service.read_file_chunks(path, chunk_size).await?;
let processed = process_chunks(chunks).await?;
service.write_file_chunks(output_path, processed).await?;

// Parallel awaits with join
use tokio::try_join;

let (chunks1, chunks2) = try_join!(
    service.read_file_chunks(path1, chunk_size),
    service.read_file_chunks(path2, chunk_size),
)?;
```

### Async Traits

```rust
use async_trait::async_trait;

#[async_trait]
pub trait FileIOService: Send + Sync {
    async fn read_file_chunks(
        &self,
        path: &Path,
        chunk_size: ChunkSize,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    async fn write_file_chunks(
        &self,
        path: &Path,
        chunks: Vec<FileChunk>,
    ) -> Result<(), PipelineError>;
}
```

---

## Tokio Runtime

The system uses Tokio's multi-threaded runtime for async execution.

### Runtime Configuration

```rust
use tokio::runtime::Runtime;

// Multi-threaded runtime (default)
let runtime = Runtime::new()?;

// Custom configuration
let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(8)              // 8 worker threads
    .thread_name("pipeline-worker")
    .thread_stack_size(3 * 1024 * 1024)  // 3 MB stack
    .enable_all()                   // Enable I/O and time drivers
    .build()?;

// Execute async work
runtime.block_on(async {
    process_file(path, chunk_size).await?;
});
```

### Runtime Selection

```rust
// Multi-threaded runtime (CPU-bound + I/O)
#[tokio::main]
async fn main() {
    // Automatically uses multi-threaded runtime
    process_pipeline().await;
}

// Current-thread runtime (testing, single-threaded)
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Single-threaded runtime
    process_pipeline().await;
}
```

### Work-Stealing Scheduler

Tokio uses a work-stealing scheduler for load balancing:

```text
Thread 1: [Task A] [Task B] ────────> (idle, steals Task D)
Thread 2: [Task C] [Task D] [Task E]  (busy)
Thread 3: [Task F] ────────────────>  (idle, steals Task E)
```

---

## Parallel Chunk Processing

Chunks are processed concurrently for maximum throughput.

### Parallel Processing with try_join_all

```rust
use futures::future::try_join_all;

async fn process_chunks_parallel(
    chunks: Vec<FileChunk>,
) -> Result<Vec<FileChunk>, PipelineError> {
    // Spawn tasks for each chunk
    let futures = chunks.into_iter().map(|chunk| {
        tokio::spawn(async move {
            process_chunk(chunk).await
        })
    });

    // Wait for all to complete
    let results = try_join_all(futures).await?;

    // Collect results
    Ok(results.into_iter().collect::<Result<Vec<_>, _>>()?)
}
```

### Bounded Parallelism

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

async fn process_with_limit(
    chunks: Vec<FileChunk>,
    max_parallel: usize,
) -> Result<Vec<FileChunk>, PipelineError> {
    let semaphore = Arc::new(Semaphore::new(max_parallel));
    let futures = chunks.into_iter().map(|chunk| {
        let permit = semaphore.clone();
        async move {
            let _guard = permit.acquire().await.unwrap();
            process_chunk(chunk).await
        }
    });

    try_join_all(futures).await
}
```

### Pipeline Parallelism

```rust
// Stage 1: Read chunks
let chunks = read_chunks_stream(path, chunk_size);

// Stage 2: Process chunks (parallel)
let processed = chunks
    .map(|chunk| async move {
        tokio::spawn(async move {
            compress_chunk(chunk).await
        }).await
    })
    .buffer_unordered(8);  // Up to 8 chunks in flight

// Stage 3: Write chunks
write_chunks_stream(processed).await?;
```

---

## Concurrency Primitives

The system uses Tokio's async-aware concurrency primitives.

### Async Mutex

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

let shared_state = Arc::new(Mutex::new(HashMap::new()));

// Acquire lock asynchronously
let mut state = shared_state.lock().await;
state.insert(key, value);
// Lock automatically released when dropped
```

### Async RwLock

```rust
use tokio::sync::RwLock;
use std::sync::Arc;

let config = Arc::new(RwLock::new(PipelineConfig::default()));

// Multiple readers
let config_read = config.read().await;
let chunk_size = config_read.chunk_size;

// Single writer
let mut config_write = config.write().await;
config_write.chunk_size = ChunkSize::from_mb(16)?;
```

### Channels (mpsc)

```rust
use tokio::sync::mpsc;

// Create channel
let (tx, mut rx) = mpsc::channel::<FileChunk>(100);

// Send chunks
tokio::spawn(async move {
    for chunk in chunks {
        tx.send(chunk).await.unwrap();
    }
});

// Receive chunks
while let Some(chunk) = rx.recv().await {
    process_chunk(chunk).await?;
}
```

### Semaphores

```rust
use tokio::sync::Semaphore;

// Limit concurrent operations
let semaphore = Arc::new(Semaphore::new(4));  // Max 4 concurrent

for chunk in chunks {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    tokio::spawn(async move {
        let result = process_chunk(chunk).await;
        drop(permit);  // Release permit
        result
    });
}
```

---

## Thread Pools and Workers

Worker pools manage concurrent task execution.

### Worker Pool Configuration

```rust
pub struct WorkerPool {
    max_workers: usize,
    semaphore: Arc<Semaphore>,
}

impl WorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            semaphore: Arc::new(Semaphore::new(max_workers)),
        }
    }

    pub async fn execute<F, T>(&self, task: F) -> Result<T, PipelineError>
    where
        F: Future<Output = Result<T, PipelineError>> + Send + 'static,
        T: Send + 'static,
    {
        let _permit = self.semaphore.acquire().await.unwrap();
        tokio::spawn(task).await.unwrap()
    }
}
```

### Adaptive Worker Count

```rust
fn optimal_worker_count() -> usize {
    let cpu_count = num_cpus::get();

    // For I/O-bound: 2x CPU count
    // For CPU-bound: 1x CPU count
    // For mixed: 1.5x CPU count
    (cpu_count as f64 * 1.5) as usize
}

let worker_pool = WorkerPool::new(optimal_worker_count());
```

For detailed worker pool implementation, see [Thread Pooling](thread-pooling.md).

---

## Resource Management

Efficient resource management is critical for concurrent systems.

### Resource Limits

```rust
pub struct ResourceLimits {
    max_memory: usize,
    max_file_handles: usize,
    max_concurrent_tasks: usize,
}

impl ResourceLimits {
    pub fn calculate_max_parallel_chunks(&self, chunk_size: ChunkSize) -> usize {
        let memory_limit = self.max_memory / chunk_size.bytes();
        let task_limit = self.max_concurrent_tasks;

        memory_limit.min(task_limit)
    }
}
```

### Resource Tracking

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ResourceTracker {
    active_tasks: AtomicUsize,
    memory_used: AtomicUsize,
}

impl ResourceTracker {
    pub fn acquire_task(&self) -> TaskGuard {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
        TaskGuard { tracker: self }
    }
}

pub struct TaskGuard<'a> {
    tracker: &'a ResourceTracker,
}

impl Drop for TaskGuard<'_> {
    fn drop(&mut self) {
        self.tracker.active_tasks.fetch_sub(1, Ordering::SeqCst);
    }
}
```

For detailed resource management, see [Resource Management](resources.md).

---

## Concurrency Patterns

Common concurrency patterns used in the pipeline.

### Pattern 1: Fan-Out/Fan-In

```rust
// Fan-out: Distribute work to multiple workers
let futures = chunks.into_iter().map(|chunk| {
    tokio::spawn(async move {
        process_chunk(chunk).await
    })
});

// Fan-in: Collect results
let results = try_join_all(futures).await?;
```

### Pattern 2: Pipeline Pattern

```rust
use tokio_stream::StreamExt;

// Stage 1 → Stage 2 → Stage 3
let result = read_stream(path)
    .map(|chunk| compress_chunk(chunk))
    .buffer_unordered(8)
    .map(|chunk| encrypt_chunk(chunk))
    .buffer_unordered(8)
    .collect::<Vec<_>>()
    .await;
```

### Pattern 3: Worker Pool Pattern

```rust
let pool = WorkerPool::new(8);

for chunk in chunks {
    pool.execute(async move {
        process_chunk(chunk).await
    }).await?;
}
```

### Pattern 4: Rate Limiting

```rust
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_millis(100));

for chunk in chunks {
    interval.tick().await;  // Rate limit: 10 chunks/sec
    process_chunk(chunk).await?;
}
```

---

## Performance Considerations

### Tokio Task Overhead

| Operation | Cost | Notes |
|-----------|------|-------|
| **Spawn task** | ~1-2 μs | Very lightweight |
| **Context switch** | ~100 ns | Work-stealing scheduler |
| **Mutex lock** | ~50 ns | Uncontended case |
| **Channel send** | ~100-200 ns | Depends on channel type |

### Choosing Concurrency Level

```rust
fn optimal_concurrency(
    file_size: u64,
    chunk_size: ChunkSize,
    available_memory: usize,
) -> usize {
    let num_chunks = (file_size / chunk_size.bytes() as u64) as usize;
    let memory_limit = available_memory / chunk_size.bytes();
    let cpu_limit = num_cpus::get() * 2;

    num_chunks.min(memory_limit).min(cpu_limit)
}
```

### Avoiding Contention

```rust
// ❌ Bad: High contention
let counter = Arc::new(Mutex::new(0));
for _ in 0..1000 {
    let c = counter.clone();
    tokio::spawn(async move {
        *c.lock().await += 1;  // Lock contention!
    });
}

// ✅ Good: Reduce contention
let counter = Arc::new(AtomicUsize::new(0));
for _ in 0..1000 {
    let c = counter.clone();
    tokio::spawn(async move {
        c.fetch_add(1, Ordering::Relaxed);  // Lock-free!
    });
}
```

---

## Best Practices

### 1. Use Async for I/O, Sync for CPU

```rust
// ✅ Good: Async I/O
async fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    tokio::fs::read(path).await
}

// ✅ Good: Sync CPU-bound
fn compress_data(data: &[u8]) -> Result<Vec<u8>, Error> {
    brotli::compress(data)  // Sync, CPU-bound
}

// ❌ Bad: Async for CPU-bound
async fn compress_data_async(data: &[u8]) -> Result<Vec<u8>, Error> {
    // Unnecessary async overhead
    brotli::compress(data)
}
```

### 2. Spawn Blocking for Sync Code

```rust
// ✅ Good: Spawn blocking task
async fn process_chunk(chunk: FileChunk) -> Result<FileChunk, Error> {
    tokio::task::spawn_blocking(move || {
        // CPU-bound compression in blocking thread
        compress_sync(chunk)
    }).await?
}
```

### 3. Limit Concurrent Tasks

```rust
// ✅ Good: Bounded parallelism
let semaphore = Arc::new(Semaphore::new(max_concurrent));
for chunk in chunks {
    let permit = semaphore.clone();
    tokio::spawn(async move {
        let _guard = permit.acquire().await.unwrap();
        process_chunk(chunk).await
    });
}

// ❌ Bad: Unbounded parallelism
for chunk in chunks {
    tokio::spawn(async move {
        process_chunk(chunk).await  // May spawn thousands of tasks!
    });
}
```

### 4. Use Channels for Communication

```rust
// ✅ Good: Channel communication
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    while let Some(chunk) = rx.recv().await {
        process_chunk(chunk).await;
    }
});

tx.send(chunk).await?;
```

### 5. Handle Errors Properly

```rust
// ✅ Good: Proper error handling
let results: Result<Vec<_>, _> = try_join_all(futures).await;
match results {
    Ok(chunks) => { /* success */ },
    Err(e) => {
        error!("Processing failed: {}", e);
        // Cleanup resources
        return Err(e);
    }
}
```

---

## Troubleshooting

### Issue 1: Too Many Tokio Tasks

**Symptom:**
```text
thread 'tokio-runtime-worker' stack overflow
```

**Solutions:**

```rust
// 1. Limit concurrent tasks
let semaphore = Arc::new(Semaphore::new(100));

// 2. Use buffer_unordered
stream.buffer_unordered(10).collect().await

// 3. Increase stack size
tokio::runtime::Builder::new_multi_thread()
    .thread_stack_size(4 * 1024 * 1024)  // 4 MB
    .build()?;
```

### Issue 2: Mutex Deadlock

**Symptom:** Tasks hang indefinitely.

**Solutions:**

```rust
// 1. Always acquire locks in same order
async fn transfer(from: &Mutex<u64>, to: &Mutex<u64>, amount: u64) {
    let (first, second) = if ptr::eq(from, to) {
        panic!("Same account");
    } else if (from as *const _ as usize) < (to as *const _ as usize) {
        (from, to)
    } else {
        (to, from)
    };

    let mut a = first.lock().await;
    let mut b = second.lock().await;
    // Transfer logic
}

// 2. Use try_lock with timeout
tokio::time::timeout(Duration::from_secs(5), mutex.lock()).await??;
```

### Issue 3: Channel Backpressure

**Symptom:**
```text
Producer overwhelms consumer
```

**Solutions:**

```rust
// 1. Bounded channel
let (tx, rx) = mpsc::channel::<FileChunk>(100);  // Max 100 in flight

// 2. Apply backpressure
match tx.try_send(chunk) {
    Ok(()) => { /* sent */ },
    Err(TrySendError::Full(_)) => {
        // Wait and retry
        tokio::time::sleep(Duration::from_millis(10)).await;
    },
    Err(e) => return Err(e.into()),
}
```

---

## Next Steps

After understanding the concurrency model, explore specific implementations:

### Related Advanced Topics

1. **[Thread Pooling](thread-pooling.md)**: Worker pool implementation and optimization
2. **[Resource Management](resources.md)**: Memory and resource tracking

### Related Topics

- **[Performance Optimization](performance.md)**: Optimizing concurrent code
- **[File I/O](../implementation/file-io.md)**: Async file operations

---

## Summary

**Key Takeaways:**

1. **Async/Await** provides non-blocking concurrency for I/O operations
2. **Tokio Runtime** uses work-stealing for efficient task scheduling
3. **Parallel Processing** enables concurrent chunk processing for throughput
4. **Concurrency Primitives** (Mutex, RwLock, Semaphore) enable safe sharing
5. **Worker Pools** manage bounded concurrent task execution
6. **Resource Management** tracks and limits resource usage
7. **Patterns** (fan-out/fan-in, pipeline, worker pool) structure concurrent code

**Architecture File References:**
- **Pipeline Service:** `pipeline/src/application/services/pipeline_service.rs:189`
- **File Processor:** `pipeline/src/application/services/file_processor_service.rs:1`
