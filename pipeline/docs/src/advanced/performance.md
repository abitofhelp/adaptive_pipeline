# Performance Optimization

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter explores performance optimization strategies for the adaptive pipeline, including benchmarking methodologies, tuning parameters, and common performance bottlenecks with their solutions.

## Overview

The pipeline is designed for high-performance file processing with several optimization strategies:

1. **Adaptive Configuration**: Automatically selects optimal settings based on file characteristics
2. **Parallel Processing**: Leverages multi-core systems with Tokio and Rayon
3. **Resource Management**: Prevents oversubscription with CPU/I/O token governance
4. **Memory Efficiency**: Streaming processing with bounded memory usage
5. **I/O Optimization**: Memory mapping, chunked I/O, and device-specific tuning

**Performance Goals:**
- **Throughput**: 100-500 MB/s for compression/encryption pipelines
- **Latency**: < 100 ms overhead for small files (< 10 MB)
- **Memory**: Bounded memory usage regardless of file size
- **Scalability**: Linear scaling up to available CPU cores

## Performance Metrics

### Throughput

**Definition**: Bytes processed per second

```rust
use pipeline_domain::entities::ProcessingMetrics;

let metrics = ProcessingMetrics::new();
metrics.start();

// ... process data ...
metrics.add_bytes_processed(file_size);

metrics.end();

println!("Throughput: {:.2} MB/s", metrics.throughput_mb_per_second());
```

**Typical Values:**
- **Uncompressed I/O**: 500-2000 MB/s (limited by storage device)
- **LZ4 compression**: 300-600 MB/s (fast, low compression)
- **Brotli compression**: 50-150 MB/s (slow, high compression)
- **AES-256-GCM encryption**: 400-800 MB/s (hardware-accelerated)
- **ChaCha20-Poly1305**: 200-400 MB/s (software)

### Latency

**Definition**: Time from start to completion

**Components:**
- **Setup overhead**: File opening, thread pool initialization (1-5 ms)
- **I/O time**: Reading/writing chunks (varies by device and size)
- **Processing time**: Compression, encryption, hashing (varies by algorithm)
- **Coordination overhead**: Task spawning, semaphore acquisition (< 1 ms)

**Optimization Strategies:**
- Minimize setup overhead by reusing resources
- Use memory mapping for large files to reduce I/O time
- Choose faster algorithms (LZ4 vs Brotli, ChaCha20 vs AES)
- Batch small operations to amortize coordination overhead

### Memory Usage

**Formula:**
```text
Peak Memory ≈ chunk_size × active_workers × files_concurrent
```

**Example:**
```text
chunk_size = 64 MB
active_workers = 7
files_concurrent = 1
Peak Memory ≈ 64 MB × 7 × 1 = 448 MB
```

**Monitoring:**
```rust
use pipeline::infrastructure::metrics::CONCURRENCY_METRICS;

let mem_mb = CONCURRENCY_METRICS.memory_used_mb();
let mem_pct = CONCURRENCY_METRICS.memory_utilization_percent();

println!("Memory: {:.2} MB ({:.1}%)", mem_mb, mem_pct);
```

## Optimization Strategies

### 1. Chunk Size Optimization

**Impact**: Chunk size affects memory usage, I/O efficiency, and parallelism.

**Adaptive Chunk Sizing:**

```rust
use pipeline_domain::value_objects::ChunkSize;

// Automatically selects optimal chunk size based on file size
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

println!("Optimal chunk size: {}", chunk_size);  // e.g., "4.0MB"
```

**Guidelines:**

| File Size        | Chunk Size | Rationale                                    |
|------------------|------------|----------------------------------------------|
| < 10 MB (small)  | 64-256 KB  | Minimize memory, enable fine-grained parallelism |
| 10-100 MB (medium) | 256 KB-1 MB | Balance memory and I/O efficiency            |
| 100 MB-1 GB (large) | 1-4 MB    | Reduce I/O overhead, acceptable memory usage |
| > 1 GB (huge)    | 4-16 MB    | Maximize I/O throughput, still bounded memory |

**Trade-offs:**
- **Small chunks**: ✅ Lower memory, better parallelism ❌ Higher I/O overhead
- **Large chunks**: ✅ Lower I/O overhead ❌ Higher memory, less parallelism

### 2. Worker Count Optimization

**Impact**: Worker count affects CPU utilization and resource contention.

**Adaptive Worker Count:**

```rust
use pipeline_domain::value_objects::WorkerCount;

// File size + system resources + processing type
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    is_cpu_intensive,  // true for compression/encryption
);

println!("Optimal workers: {}", workers);  // e.g., "8 workers"
```

**Empirically Validated Strategies:**

| File Size         | Worker Count | Strategy                  | Benchmark Result |
|-------------------|--------------|---------------------------|------------------|
| 5 MB (small)      | 9            | Aggressive parallelism    | +102% speedup    |
| 50 MB (medium)    | 5            | Balanced approach         | +70% speedup     |
| 2 GB (huge)       | 3            | Conservative (avoid overhead) | +76% speedup     |

**Why these strategies work:**
- **Small files**: Task overhead is amortized quickly with many workers
- **Medium files**: Balanced to avoid both under-utilization and over-subscription
- **Huge files**: Fewer workers prevent memory pressure and coordination overhead

### 3. Memory Mapping vs Regular I/O

**When to use memory mapping:**
- ✅ Files > 100 MB (amortizes setup cost)
- ✅ Random access patterns (page cache efficiency)
- ✅ Read-heavy workloads (no write overhead)

**When to use regular I/O:**
- ✅ Files < 10 MB (lower setup cost)
- ✅ Sequential access patterns (streaming)
- ✅ Write-heavy workloads (buffered writes)

**Configuration:**

```rust
use pipeline::infrastructure::services::FileIOServiceImpl;
use pipeline_domain::services::FileIOConfig;

let config = FileIOConfig {
    enable_memory_mapping: true,
    max_mmap_size: 1024 * 1024 * 1024,  // 1 GB threshold
    default_chunk_size: 64 * 1024,       // 64 KB chunks
    ..Default::default()
};

let service = FileIOServiceImpl::new(config);
```

**Benchmark Results** (from `pipeline/benches/file_io_benchmark.rs`):

| File Size | Regular I/O | Memory Mapping | Winner          |
|-----------|-------------|----------------|-----------------|
| 1 MB      | 2000 MB/s   | 1500 MB/s      | Regular I/O     |
| 10 MB     | 1800 MB/s   | 1900 MB/s      | Comparable      |
| 50 MB     | 1500 MB/s   | 2200 MB/s      | Memory Mapping  |
| 100 MB    | 1400 MB/s   | 2500 MB/s      | Memory Mapping  |

### 4. Compression Algorithm Selection

**Performance vs Compression Ratio:**

| Algorithm | Compression Speed | Decompression Speed | Ratio | Use Case            |
|-----------|-------------------|---------------------|-------|---------------------|
| **LZ4**   | 500-700 MB/s      | 2000-3000 MB/s      | 2-3x  | Real-time, low latency |
| **Zstd**  | 200-400 MB/s      | 600-800 MB/s        | 3-5x  | Balanced, general use |
| **Brotli**| 50-150 MB/s       | 300-500 MB/s        | 4-8x  | Storage, high compression |

**Adaptive Selection:**

```rust
use pipeline_domain::services::CompressionPriority;

// Automatic algorithm selection
let config = service.get_optimal_config(
    "data.bin",
    &sample_data,
    CompressionPriority::Speed,  // or CompressionPriority::Ratio
)?;

println!("Selected: {:?}", config.algorithm);
```

**Guidelines:**
- **Speed priority**: LZ4 for streaming, real-time processing
- **Balanced**: Zstandard for general-purpose use
- **Ratio priority**: Brotli for archival, storage optimization

### 5. Encryption Algorithm Selection

**Performance Characteristics:**

| Algorithm           | Throughput    | Security | Hardware Support |
|---------------------|---------------|----------|------------------|
| **AES-256-GCM**     | 400-800 MB/s  | Excellent | Yes (AES-NI)    |
| **ChaCha20-Poly1305** | 200-400 MB/s | Excellent | No              |
| **XChaCha20-Poly1305** | 180-350 MB/s | Excellent | No              |

**Configuration:**

```rust
use pipeline_domain::services::EncryptionAlgorithm;

// Use AES-256-GCM if hardware support available
let algorithm = if has_aes_ni() {
    EncryptionAlgorithm::Aes256Gcm  // 2-4x faster with AES-NI
} else {
    EncryptionAlgorithm::ChaCha20Poly1305  // Software fallback
};
```

## Common Bottlenecks

### 1. CPU Bottleneck

**Symptoms:**
- CPU saturation > 80%
- High CPU wait times (P95 > 50 ms)
- Low I/O utilization

**Causes:**
- Too many CPU-intensive operations (compression, encryption)
- Insufficient worker count for CPU-bound work
- Slow algorithms (Brotli on large files)

**Solutions:**

```rust
// Increase CPU tokens to match cores
let config = ResourceConfig {
    cpu_tokens: Some(available_cores),  // Use all cores
    ..Default::default()
};

// Use faster algorithms
let compression = CompressionAlgorithm::Lz4;  // Instead of Brotli
let encryption = EncryptionAlgorithm::Aes256Gcm;  // With AES-NI

// Optimize worker count
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    true,  // CPU-intensive = true
);
```

### 2. I/O Bottleneck

**Symptoms:**
- I/O saturation > 80%
- High I/O wait times (P95 > 100 ms)
- Low CPU utilization

**Causes:**
- Too many concurrent I/O operations
- Small chunk sizes causing excessive syscalls
- Storage device queue depth exceeded

**Solutions:**

```rust
// Increase chunk size to reduce I/O overhead
let chunk_size = ChunkSize::from_mb(4)?;  // 4 MB chunks

// Reduce I/O concurrency for HDD
let config = ResourceConfig {
    storage_type: StorageType::HDD,  // 4 I/O tokens
    ..Default::default()
};

// Use memory mapping for large files
let use_mmap = file_size > 100 * 1024 * 1024;  // > 100 MB
```

**I/O Optimization by Device:**

| Device Type | Optimal Chunk Size | I/O Tokens | Strategy              |
|-------------|--------------------|-----------|-----------------------|
| **HDD**     | 1-4 MB             | 4         | Sequential, large chunks |
| **SSD**     | 256 KB-1 MB        | 12        | Balanced              |
| **NVMe**    | 64 KB-256 KB       | 24        | Parallel, small chunks |

### 3. Memory Bottleneck

**Symptoms:**
- Memory utilization > 80%
- Swapping (check `vmstat`)
- OOM errors

**Causes:**
- Too many concurrent chunks allocated
- Large chunk size × high worker count
- Memory leaks or unbounded buffers

**Solutions:**

```rust
// Reduce chunk size
let chunk_size = ChunkSize::from_mb(16)?;  // Smaller chunks

// Limit concurrent workers
let config = ResourceConfig {
    cpu_tokens: Some(3),  // Fewer workers = less memory
    ..Default::default()
};

// Monitor memory closely
if CONCURRENCY_METRICS.memory_utilization_percent() > 80.0 {
    warn!("High memory usage, reducing chunk size");
    chunk_size = ChunkSize::from_mb(8)?;
}
```

### 4. Coordination Overhead

**Symptoms:**
- High task spawn latency
- Context switching > 10k/sec
- Low overall throughput despite low resource usage

**Causes:**
- Too many small tasks (excessive spawn_blocking calls)
- High semaphore contention
- Channel backpressure

**Solutions:**

```rust
// Batch small operations
if chunks.len() < 10 {
    // Sequential for small batches (avoid spawn overhead)
    for chunk in chunks {
        process_chunk_sync(chunk)?;
    }
} else {
    // Parallel for large batches
    tokio::task::spawn_blocking(move || {
        RAYON_POOLS.cpu_bound_pool().install(|| {
            chunks.into_par_iter().map(process_chunk_sync).collect()
        })
    }).await??
}

// Reduce worker count to lower contention
let workers = WorkerCount::new(available_cores / 2);
```

## Tuning Parameters

### Chunk Size Tuning

**Parameters:**

```rust
pub struct ChunkSize {
    pub const MIN_SIZE: usize = 1;              // 1 byte
    pub const MAX_SIZE: usize = 512 * 1024 * 1024;  // 512 MB
    pub const DEFAULT_SIZE: usize = 1024 * 1024;    // 1 MB
}
```

**Configuration:**

```rust
// Via ChunkSize value object
let chunk_size = ChunkSize::from_mb(4)?;

// Via CLI/config file
let chunk_size_mb = 4;
let chunk_size = ChunkSize::from_mb(chunk_size_mb)?;

// Adaptive (recommended)
let chunk_size = ChunkSize::optimal_for_file_size(file_size);
```

**Impact:**
- **Memory**: Directly proportional (2x chunk = 2x memory per worker)
- **I/O overhead**: Inversely proportional (2x chunk = 0.5x syscalls)
- **Parallelism**: Inversely proportional (2x chunk = 0.5x parallel units)

### Worker Count Tuning

**Parameters:**

```rust
pub struct WorkerCount {
    pub const MIN_WORKERS: usize = 1;
    pub const MAX_WORKERS: usize = 32;
    pub const DEFAULT_WORKERS: usize = 4;
}
```

**Configuration:**

```rust
// Manual
let workers = WorkerCount::new(8);

// Adaptive (recommended)
let workers = WorkerCount::optimal_for_file_size(file_size);

// With system resources
let workers = WorkerCount::optimal_for_file_and_system(
    file_size,
    available_cores,
);

// With processing type
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    is_cpu_intensive,
);
```

**Impact:**
- **Throughput**: Generally increases with workers (up to cores)
- **Memory**: Directly proportional (2x workers = 2x memory)
- **Context switching**: Increases with workers (diminishing returns > 2x cores)

### Resource Token Tuning

**CPU Tokens:**

```rust
let config = ResourceConfig {
    cpu_tokens: Some(7),  // cores - 1 (default)
    ..Default::default()
};
```

**I/O Tokens:**

```rust
let config = ResourceConfig {
    io_tokens: Some(24),          // Device-specific
    storage_type: StorageType::NVMe,
    ..Default::default()
};
```

**Impact:**
- **CPU tokens**: Limits total CPU-bound parallelism across all files
- **I/O tokens**: Limits total I/O concurrency across all files
- **Both**: Prevent system oversubscription

## Performance Monitoring

### Real-Time Metrics

```rust
use pipeline::infrastructure::metrics::CONCURRENCY_METRICS;
use std::time::Duration;

// Spawn monitoring task
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        // Resource saturation
        let cpu_sat = CONCURRENCY_METRICS.cpu_saturation_percent();
        let io_sat = CONCURRENCY_METRICS.io_saturation_percent();
        let mem_util = CONCURRENCY_METRICS.memory_utilization_percent();

        // Wait time percentiles
        let cpu_p95 = CONCURRENCY_METRICS.cpu_wait_p95();
        let io_p95 = CONCURRENCY_METRICS.io_wait_p95();

        info!(
            "Resources: CPU={:.1}%, I/O={:.1}%, Mem={:.1}% | Wait: CPU={}ms, I/O={}ms",
            cpu_sat, io_sat, mem_util, cpu_p95, io_p95
        );

        // Alert on issues
        if cpu_sat > 90.0 {
            warn!("CPU saturated - consider increasing workers or faster algorithms");
        }
        if mem_util > 80.0 {
            warn!("High memory - consider reducing chunk size or workers");
        }
    }
});
```

### Processing Metrics

```rust
use pipeline_domain::entities::ProcessingMetrics;

let metrics = ProcessingMetrics::new();
metrics.start();

// Process file...
for chunk in chunks {
    metrics.add_bytes_processed(chunk.data.len() as u64);
}

metrics.end();

// Report performance
println!("Throughput: {:.2} MB/s", metrics.throughput_mb_per_second());
println!("Duration: {:.2}s", metrics.duration().as_secs_f64());
println!("Processed: {} MB", metrics.bytes_processed() / (1024 * 1024));

// Stage-specific metrics
for stage_metrics in metrics.stage_metrics() {
    println!("  {}: {:.2} MB/s", stage_metrics.stage_name, stage_metrics.throughput);
}
```

## Performance Best Practices

### 1. Use Adaptive Configuration

```rust
// ✅ Good: Let the system optimize
let chunk_size = ChunkSize::optimal_for_file_size(file_size);
let workers = WorkerCount::optimal_for_processing_type(
    file_size,
    available_cores,
    is_cpu_intensive,
);

// ❌ Bad: Fixed values
let chunk_size = ChunkSize::from_mb(1)?;
let workers = WorkerCount::new(8);
```

### 2. Choose Appropriate Algorithms

```rust
// ✅ Good: Algorithm selection based on priority
let compression_config = service.get_optimal_config(
    file_extension,
    &sample_data,
    CompressionPriority::Speed,  // or Ratio
)?;

// ❌ Bad: Always use same algorithm
let compression_config = CompressionConfig {
    algorithm: CompressionAlgorithm::Brotli,  // Slow!
    ..Default::default()
};
```

### 3. Monitor and Measure

```rust
// ✅ Good: Measure actual performance
let start = Instant::now();
let result = process_file(path).await?;
let duration = start.elapsed();

let throughput_mb_s = (file_size as f64 / duration.as_secs_f64()) / (1024.0 * 1024.0);
info!("Throughput: {:.2} MB/s", throughput_mb_s);

// ❌ Bad: Assume performance without measurement
let result = process_file(path).await?;
```

### 4. Batch Small Operations

```rust
// ✅ Good: Batch to amortize overhead
tokio::task::spawn_blocking(move || {
    RAYON_POOLS.cpu_bound_pool().install(|| {
        chunks.into_par_iter()
            .map(|chunk| process_chunk(chunk))
            .collect::<Result<Vec<_>, _>>()
    })
}).await??

// ❌ Bad: Spawn for each small operation
for chunk in chunks {
    tokio::task::spawn_blocking(move || {
        process_chunk(chunk)  // Excessive spawn overhead!
    }).await??
}
```

### 5. Use Device-Specific Settings

```rust
// ✅ Good: Configure for storage type
let config = ResourceConfig {
    storage_type: StorageType::NVMe,  // 24 I/O tokens
    io_tokens: Some(24),
    ..Default::default()
};

// ❌ Bad: One size fits all
let config = ResourceConfig {
    io_tokens: Some(12),  // May be suboptimal
    ..Default::default()
};
```

## Troubleshooting Performance Issues

### Issue 1: Low Throughput Despite Low Resource Usage

**Symptoms:**
- Throughput < 100 MB/s
- CPU usage < 50%
- I/O usage < 50%

**Diagnosis:**
```rust
// Check coordination overhead
let queue_depth = CONCURRENCY_METRICS.cpu_queue_depth();
let active_workers = CONCURRENCY_METRICS.active_workers();

println!("Queue: {}, Active: {}", queue_depth, active_workers);
```

**Causes:**
- Too few workers (underutilization)
- Small batch sizes (high spawn overhead)
- Synchronous bottlenecks

**Solutions:**
```rust
// Increase workers
let workers = WorkerCount::new(available_cores);

// Batch operations
let batch_size = 100;
for batch in chunks.chunks(batch_size) {
    process_batch(batch).await?;
}
```

### Issue 2: Inconsistent Performance

**Symptoms:**
- Performance varies widely between runs
- High P99 latencies (> 10x P50)

**Diagnosis:**
```rust
// Check wait time distribution
let p50 = CONCURRENCY_METRICS.cpu_wait_p50();
let p95 = CONCURRENCY_METRICS.cpu_wait_p95();
let p99 = CONCURRENCY_METRICS.cpu_wait_p99();

println!("Wait times: P50={}ms, P95={}ms, P99={}ms", p50, p95, p99);
```

**Causes:**
- Resource contention (high wait times)
- GC pauses or memory pressure
- External system interference

**Solutions:**
```rust
// Reduce contention
let config = ResourceConfig {
    cpu_tokens: Some(available_cores - 2),  // Leave headroom
    ..Default::default()
};

// Monitor memory
if mem_util > 70.0 {
    chunk_size = ChunkSize::from_mb(chunk_size_mb / 2)?;
}
```

### Issue 3: Memory Growth

**Symptoms:**
- Memory usage grows over time
- Eventually triggers OOM or swapping

**Diagnosis:**
```rust
// Track memory trends
let mem_start = CONCURRENCY_METRICS.memory_used_mb();
// ... process files ...
let mem_end = CONCURRENCY_METRICS.memory_used_mb();

if mem_end > mem_start * 1.5 {
    warn!("Memory grew {:.1}%", ((mem_end - mem_start) / mem_start) * 100.0);
}
```

**Causes:**
- Memory leaks (improper cleanup)
- Unbounded queues or buffers
- Large chunk size with many workers

**Solutions:**
```rust
// Use RAII guards for cleanup
struct ChunkBuffer {
    data: Vec<u8>,
    _guard: MemoryGuard,
}

// Limit queue depth
let (tx, rx) = tokio::sync::mpsc::channel(100);  // Bounded channel

// Reduce chunk size
let chunk_size = ChunkSize::from_mb(16)?;  // Smaller
```

## Related Topics

- See [Benchmarking](benchmarking.md) for detailed benchmark methodology
- See [Profiling](profiling.md) for CPU and memory profiling techniques
- See [Thread Pooling](thread-pooling.md) for worker configuration
- See [Resource Management](resources.md) for token governance

## Summary

The pipeline's performance optimization system provides:

1. **Adaptive Configuration**: Automatic chunk size and worker count optimization
2. **Algorithm Selection**: Choose algorithms based on speed/ratio priority
3. **Resource Governance**: Prevent oversubscription with token limits
4. **Memory Efficiency**: Bounded memory usage with streaming processing
5. **Comprehensive Monitoring**: Real-time metrics and performance tracking

**Key Takeaways:**
- Use adaptive configuration (ChunkSize::optimal_for_file_size, WorkerCount::optimal_for_processing_type)
- Choose algorithms based on workload (LZ4 for speed, Brotli for ratio)
- Monitor metrics regularly (CPU/I/O saturation, wait times, throughput)
- Tune based on bottleneck (CPU: increase workers/faster algorithms, I/O: increase chunk size, Memory: reduce chunk/workers)
- Benchmark and measure actual performance (don't assume)

**Performance Goals Achieved:**
- ✅ Throughput: 100-500 MB/s (algorithm-dependent)
- ✅ Latency: < 100 ms overhead for small files
- ✅ Memory: Bounded usage (chunk_size × workers × files)
- ✅ Scalability: Linear scaling up to available cores
