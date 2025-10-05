# Chunking Strategy

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a comprehensive overview of the file chunking strategy in the adaptive pipeline system. Learn how files are split into manageable chunks, how chunk sizes are optimized, and how chunking enables efficient parallel processing.

---

## Table of Contents

- [Overview](#overview)
- [Chunking Architecture](#chunking-architecture)
- [Chunk Size Selection](#chunk-size-selection)
- [Chunking Algorithm](#chunking-algorithm)
- [Optimal Sizing Strategy](#optimal-sizing-strategy)
- [Memory Management](#memory-management)
- [Parallel Processing](#parallel-processing)
- [Adaptive Chunking](#adaptive-chunking)
- [Performance Characteristics](#performance-characteristics)
- [Usage Examples](#usage-examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Testing Strategies](#testing-strategies)
- [Next Steps](#next-steps)

---

## Overview

**Chunking** is the process of dividing files into smaller, manageable pieces (chunks) that can be processed independently. This strategy enables efficient memory usage, parallel processing, and scalable file handling regardless of file size.

### Key Benefits

- **Memory Efficiency**: Process files larger than available RAM
- **Parallel Processing**: Process multiple chunks concurrently
- **Fault Tolerance**: Failed chunks can be retried independently
- **Progress Tracking**: Track progress at chunk granularity
- **Scalability**: Handle files from bytes to terabytes

### Chunking Workflow

```text
Input File (100 MB)
        ↓
[Chunking Strategy]
        ↓
┌──────────┬──────────┬──────────┬──────────┐
│ Chunk 0  │ Chunk 1  │ Chunk 2  │ Chunk 3  │
│ (0-25MB) │(25-50MB) │(50-75MB) │(75-100MB)│
└──────────┴──────────┴──────────┴──────────┘
        ↓ parallel processing
┌──────────┬──────────┬──────────┬──────────┐
│Processed │Processed │Processed │Processed │
│ Chunk 0  │ Chunk 1  │ Chunk 2  │ Chunk 3  │
└──────────┴──────────┴──────────┴──────────┘
        ↓
Output File (processed)
```

### Design Principles

1. **Predictable Memory**: Bounded memory usage regardless of file size
2. **Optimal Sizing**: Empirically optimized chunk sizes for performance
3. **Independent Processing**: Each chunk can be processed in isolation
4. **Ordered Reassembly**: Chunks maintain sequence for correct reassembly
5. **Adaptive Strategy**: Chunk size adapts to file size and system resources

---

## Chunking Architecture

The chunking system uses a combination of value objects and algorithms to efficiently divide files.

### Chunking Components

```text
┌─────────────────────────────────────────────────────────┐
│                 Chunking Strategy                        │
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  ChunkSize   │  │ FileChunk    │  │  Chunking    │ │
│  │   (1B-512MB) │  │ (immutable)  │  │  Algorithm   │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                          │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│              Optimal Size Calculation                    │
│                                                          │
│  File Size → optimal_for_file_size() → ChunkSize       │
│                                                          │
│  - Small files  (<10 MB):    64-256 KB chunks          │
│  - Medium files (10-500 MB): 2-16 MB chunks            │
│  - Large files  (500MB-2GB): 64 MB chunks              │
│  - Huge files   (>2 GB):     128 MB chunks             │
└─────────────────────────────────────────────────────────┘
```

### Chunk Lifecycle

```text
1. Size Determination
   - Calculate optimal chunk size based on file size
   - Adjust for available memory if needed
   ↓
2. File Division
   - Read file in chunk-sized pieces
   - Create FileChunk with sequence number and offset
   ↓
3. Chunk Processing
   - Apply pipeline stages to each chunk
   - Process chunks in parallel if enabled
   ↓
4. Chunk Reassembly
   - Combine processed chunks by sequence number
   - Write to output file
```

---

## Chunk Size Selection

Chunk size is critical for performance and memory efficiency. The system supports validated sizes from 1 byte to 512 MB.

### Size Constraints

```rust
// ChunkSize constants
ChunkSize::MIN_SIZE  // 1 byte
ChunkSize::MAX_SIZE  // 512 MB
ChunkSize::DEFAULT   // 1 MB
```

### Creating Chunk Sizes

```rust
use pipeline_domain::ChunkSize;

// From bytes
let chunk = ChunkSize::new(1024 * 1024)?;  // 1 MB

// From kilobytes
let chunk_kb = ChunkSize::from_kb(512)?;  // 512 KB

// From megabytes
let chunk_mb = ChunkSize::from_mb(16)?;  // 16 MB

// Default size
let default_chunk = ChunkSize::default();  // 1 MB
```

### Size Validation

```rust
// ✅ Valid sizes
let valid = ChunkSize::new(64 * 1024)?;  // 64 KB - valid

// ❌ Invalid: too small
let too_small = ChunkSize::new(0);  // Error: must be ≥ 1 byte
assert!(too_small.is_err());

// ❌ Invalid: too large
let too_large = ChunkSize::new(600 * 1024 * 1024);  // Error: must be ≤ 512 MB
assert!(too_large.is_err());
```

### Size Trade-offs

| Chunk Size | Memory Usage | I/O Overhead | Parallelism | Best For |
|------------|--------------|--------------|-------------|----------|
| **Small (64-256 KB)** | Low | High | Excellent | Small files, limited memory |
| **Medium (1-16 MB)** | Moderate | Moderate | Good | Most use cases |
| **Large (64-128 MB)** | High | Low | Limited | Large files, ample memory |

---

## Chunking Algorithm

The chunking algorithm divides files into sequential chunks with proper metadata.

### Basic Chunking Process

```rust
pub fn chunk_file(
    file_path: &Path,
    chunk_size: ChunkSize,
) -> Result<Vec<FileChunk>, PipelineError> {
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();
    let mut chunks = Vec::new();
    let mut offset = 0;
    let mut sequence = 0;

    // Read file in chunks
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; chunk_size.bytes()];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;  // EOF
        }

        let data = buffer[..bytes_read].to_vec();
        let is_final = offset + bytes_read as u64 >= file_size;

        let chunk = FileChunk::new(sequence, offset, data, is_final)?;
        chunks.push(chunk);

        offset += bytes_read as u64;
        sequence += 1;
    }

    Ok(chunks)
}
```

### Chunk Metadata

Each chunk contains essential metadata:

```rust
pub struct FileChunk {
    id: Uuid,                 // Unique chunk identifier
    sequence_number: u64,     // Order in file (0-based)
    offset: u64,              // Byte offset in original file
    size: ChunkSize,          // Actual chunk size
    data: Vec<u8>,            // Chunk data
    checksum: Option<String>, // Optional checksum
    is_final: bool,           // Last chunk flag
    created_at: DateTime<Utc>,// Creation timestamp
}
```

### Calculating Chunk Count

```rust
// Calculate number of chunks needed
let file_size = 100 * 1024 * 1024;  // 100 MB
let chunk_size = ChunkSize::from_mb(4)?;  // 4 MB chunks

let num_chunks = chunk_size.chunks_needed_for_file(file_size);
println!("Need {} chunks", num_chunks);  // 25 chunks
```

---

## Optimal Sizing Strategy

The system uses empirically optimized chunk sizes based on comprehensive benchmarking.

### Optimization Strategy

```rust
pub fn optimal_for_file_size(file_size: u64) -> ChunkSize {
    let optimal_size = match file_size {
        // Small files: smaller chunks
        0..=1_048_576 => 64 * 1024,           // 64KB for ≤ 1MB
        1_048_577..=10_485_760 => 256 * 1024, // 256KB for ≤ 10MB

        // Medium files: empirically optimized
        10_485_761..=52_428_800 => 2 * 1024 * 1024,  // 2MB for ≤ 50MB
        52_428_801..=524_288_000 => 16 * 1024 * 1024, // 16MB for 50-500MB

        // Large files: larger chunks for efficiency
        524_288_001..=2_147_483_648 => 64 * 1024 * 1024, // 64MB for 500MB-2GB

        // Huge files: maximum throughput
        _ => 128 * 1024 * 1024, // 128MB for >2GB
    };

    ChunkSize { bytes: optimal_size.clamp(MIN_SIZE, MAX_SIZE) }
}
```

### Empirical Results

Benchmarking results that informed this strategy:

| File Size | Chunk Size | Throughput | Improvement |
|-----------|-----------|------------|-------------|
| 100 MB | 16 MB | ~300 MB/s | +43.7% vs 2 MB |
| 500 MB | 16 MB | ~320 MB/s | +56.2% vs 4 MB |
| 2 GB | 128 MB | ~350 MB/s | Baseline |

### Using Optimal Sizes

```rust
// Automatically select optimal chunk size
let file_size = 100 * 1024 * 1024;  // 100 MB
let optimal = ChunkSize::optimal_for_file_size(file_size);

println!("Optimal chunk size: {} MB", optimal.megabytes());  // 16 MB

// Check if current size is optimal
let current = ChunkSize::from_mb(4)?;
if !current.is_optimal_for_file(file_size) {
    println!("Warning: chunk size may be suboptimal");
}
```

---

## Memory Management

Chunking enables predictable memory usage regardless of file size.

### Bounded Memory Usage

```text
Without Chunking:
  File: 10 GB
  Memory: 10 GB (entire file in memory)

With Chunking (16 MB chunks):
  File: 10 GB
  Memory: 16 MB (single chunk in memory)
  Reduction: 640x less memory!
```

### Memory-Adaptive Chunking

```rust
// Adjust chunk size based on available memory
let available_memory = 100 * 1024 * 1024;  // 100 MB available
let max_parallel_chunks = 4;

let chunk_size = ChunkSize::from_mb(32)?;  // Desired 32 MB
let adjusted = chunk_size.adjust_for_memory(
    available_memory,
    max_parallel_chunks,
)?;

println!("Adjusted chunk size: {} MB", adjusted.megabytes());
// 25 MB (100 MB / 4 chunks)
```

### Memory Footprint Calculation

```rust
fn calculate_memory_footprint(
    chunk_size: ChunkSize,
    parallel_chunks: usize,
) -> usize {
    // Base memory per chunk
    let per_chunk = chunk_size.bytes();

    // Additional overhead (metadata, buffers, etc.)
    let overhead_per_chunk = 1024;  // ~1 KB overhead

    // Total memory footprint
    parallel_chunks * (per_chunk + overhead_per_chunk)
}

let chunk_size = ChunkSize::from_mb(4)?;
let memory = calculate_memory_footprint(chunk_size, 4);
println!("Memory footprint: {} MB", memory / (1024 * 1024));
// ~16 MB total
```

---

## Parallel Processing

Chunking enables efficient parallel processing of file data.

### Parallel Chunk Processing

```rust
use futures::future::try_join_all;

async fn process_chunks_parallel(
    chunks: Vec<FileChunk>,
) -> Result<Vec<FileChunk>, PipelineError> {
    // Process chunks in parallel
    let futures = chunks.into_iter().map(|chunk| {
        tokio::spawn(async move {
            process_chunk(chunk).await
        })
    });

    // Wait for all to complete
    let results = try_join_all(futures).await?;
    Ok(results.into_iter().collect::<Result<Vec<_>, _>>()?)
}
```

### Parallelism Trade-offs

```text
Sequential Processing:
  Time = num_chunks × time_per_chunk
  Memory = 1 × chunk_size

Parallel Processing (N threads):
  Time = (num_chunks / N) × time_per_chunk
  Memory = N × chunk_size
```

### Optimal Parallelism

```rust
// Calculate optimal parallelism
fn optimal_parallelism(
    file_size: u64,
    chunk_size: ChunkSize,
    available_memory: usize,
    cpu_cores: usize,
) -> usize {
    let num_chunks = chunk_size.chunks_needed_for_file(file_size) as usize;

    // Memory-based limit
    let memory_limit = available_memory / chunk_size.bytes();

    // CPU-based limit
    let cpu_limit = cpu_cores;

    // Chunk count limit
    let chunk_limit = num_chunks;

    // Take minimum of all limits
    memory_limit.min(cpu_limit).min(chunk_limit).max(1)
}

let file_size = 100 * 1024 * 1024;
let chunk_size = ChunkSize::from_mb(4)?;
let parallelism = optimal_parallelism(
    file_size,
    chunk_size,
    64 * 1024 * 1024,  // 64 MB available
    8,                  // 8 CPU cores
);
println!("Optimal parallelism: {} chunks", parallelism);
```

---

## Adaptive Chunking

The system can adapt chunk sizes dynamically based on conditions.

### Adaptive Sizing Triggers

```rust
pub enum AdaptiveTrigger {
    MemoryPressure,      // Reduce chunk size due to low memory
    SlowPerformance,     // Increase chunk size for better throughput
    NetworkLatency,      // Reduce chunk size for streaming
    CpuUtilization,      // Adjust based on CPU usage
}
```

### Dynamic Adjustment

```rust
async fn adaptive_chunking(
    file_path: &Path,
    initial_chunk_size: ChunkSize,
) -> Result<Vec<FileChunk>, PipelineError> {
    let mut chunk_size = initial_chunk_size;
    let mut chunks = Vec::new();
    let mut performance_samples = Vec::new();

    loop {
        let start = Instant::now();

        // Read next chunk
        let chunk = read_next_chunk(file_path, chunk_size)?;
        if chunk.is_none() {
            break;
        }

        let duration = start.elapsed();
        performance_samples.push(duration);

        chunks.push(chunk.unwrap());

        // Adapt chunk size based on performance
        if performance_samples.len() >= 5 {
            let avg_time = performance_samples.iter().sum::<Duration>() / 5;

            if avg_time > Duration::from_millis(100) {
                // Too slow, increase chunk size
                chunk_size = adjust_chunk_size(chunk_size, 1.5)?;
            } else if avg_time < Duration::from_millis(10) {
                // Too fast, reduce overhead by increasing size
                chunk_size = adjust_chunk_size(chunk_size, 1.2)?;
            }

            performance_samples.clear();
        }
    }

    Ok(chunks)
}

fn adjust_chunk_size(
    current: ChunkSize,
    factor: f64,
) -> Result<ChunkSize, PipelineError> {
    let new_size = (current.bytes() as f64 * factor) as usize;
    ChunkSize::new(new_size)
}
```

---

## Performance Characteristics

Chunking performance varies based on size and system characteristics.

### Throughput by Chunk Size

| Chunk Size | Read Speed | Write Speed | CPU Usage | Memory |
|------------|-----------|------------|-----------|--------|
| **64 KB** | ~40 MB/s | ~35 MB/s | 15% | Low |
| **1 MB** | ~120 MB/s | ~100 MB/s | 20% | Low |
| **16 MB** | ~300 MB/s | ~280 MB/s | 25% | Medium |
| **64 MB** | ~320 MB/s | ~300 MB/s | 30% | High |
| **128 MB** | ~350 MB/s | ~320 MB/s | 35% | High |

*Benchmarks on NVMe SSD with 8-core CPU*

### Latency Characteristics

```text
Small Chunks (64 KB):
  - Low latency per chunk: ~1-2ms
  - High overhead: many chunks
  - Good for: streaming, low memory

Large Chunks (128 MB):
  - High latency per chunk: ~400ms
  - Low overhead: few chunks
  - Good for: throughput, batch processing
```

### Performance Optimization

```rust
// Benchmark different chunk sizes
async fn benchmark_chunk_sizes(
    file_path: &Path,
    sizes: &[ChunkSize],
) -> Vec<(ChunkSize, Duration)> {
    let mut results = Vec::new();

    for &size in sizes {
        let start = Instant::now();
        let _ = chunk_file(file_path, size).await.unwrap();
        let duration = start.elapsed();

        results.push((size, duration));
    }

    results.sort_by_key(|(_, duration)| *duration);
    results
}

// Usage
let sizes = vec![
    ChunkSize::from_kb(64)?,
    ChunkSize::from_mb(1)?,
    ChunkSize::from_mb(16)?,
    ChunkSize::from_mb(64)?,
];

let results = benchmark_chunk_sizes(Path::new("./test.dat"), &sizes).await;
for (size, duration) in results {
    println!("{} MB: {:?}", size.megabytes(), duration);
}
```

---

## Usage Examples

### Example 1: Basic Chunking

```rust
use pipeline_domain::{ChunkSize, FileChunk};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./input.dat");

    // Determine optimal chunk size
    let file_size = std::fs::metadata(file_path)?.len();
    let chunk_size = ChunkSize::optimal_for_file_size(file_size);

    println!("File size: {} MB", file_size / (1024 * 1024));
    println!("Chunk size: {} MB", chunk_size.megabytes());

    // Chunk the file
    let chunks = chunk_file(file_path, chunk_size)?;
    println!("Created {} chunks", chunks.len());

    Ok(())
}
```

### Example 2: Memory-Adaptive Chunking

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./large_file.dat");
    let file_size = std::fs::metadata(file_path)?.len();

    // Start with optimal size
    let optimal = ChunkSize::optimal_for_file_size(file_size);

    // Adjust for available memory
    let available_memory = 100 * 1024 * 1024;  // 100 MB
    let max_parallel = 4;

    let chunk_size = optimal.adjust_for_memory(
        available_memory,
        max_parallel,
    )?;

    println!("Optimal: {} MB", optimal.megabytes());
    println!("Adjusted: {} MB", chunk_size.megabytes());

    let chunks = chunk_file(file_path, chunk_size)?;
    println!("Created {} chunks", chunks.len());

    Ok(())
}
```

### Example 3: Parallel Chunk Processing

```rust
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./input.dat");
    let chunk_size = ChunkSize::from_mb(16)?;

    // Create chunks
    let chunks = chunk_file(file_path, chunk_size)?;
    println!("Processing {} chunks in parallel", chunks.len());

    // Process in parallel
    let futures = chunks.into_iter().map(|chunk| {
        tokio::spawn(async move {
            // Simulate processing
            tokio::time::sleep(Duration::from_millis(10)).await;
            process_chunk(chunk).await
        })
    });

    let results = try_join_all(futures).await?;
    let processed: Vec<_> = results.into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    println!("Processed {} chunks", processed.len());

    Ok(())
}

async fn process_chunk(chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    // Transform chunk data
    let transformed_data = chunk.data().to_vec();
    Ok(FileChunk::new(
        chunk.sequence_number(),
        chunk.offset(),
        transformed_data,
        chunk.is_final(),
    )?)
}
```

### Example 4: Adaptive Chunking

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./test.dat");
    let initial_size = ChunkSize::from_mb(4)?;

    println!("Starting with {} MB chunks", initial_size.megabytes());

    let chunks = adaptive_chunking(file_path, initial_size).await?;

    println!("Created {} chunks with adaptive sizing", chunks.len());

    // Analyze chunk sizes
    for chunk in &chunks[0..5.min(chunks.len())] {
        println!("Chunk {}: {} bytes",
            chunk.sequence_number(),
            chunk.size().bytes()
        );
    }

    Ok(())
}
```

---

## Best Practices

### 1. Use Optimal Chunk Sizes

```rust
// ✅ Good: Use optimal sizing
let file_size = metadata(path)?.len();
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

// ❌ Bad: Fixed size for all files
let chunk_size = ChunkSize::from_mb(1)?;
```

### 2. Consider Memory Constraints

```rust
// ✅ Good: Adjust for available memory
let chunk_size = optimal.adjust_for_memory(
    available_memory,
    max_parallel_chunks,
)?;

// ❌ Bad: Ignore memory limits
let chunk_size = ChunkSize::from_mb(128)?;  // May cause OOM
```

### 3. Validate Chunk Sizes

```rust
// ✅ Good: Validate user input
let user_size_mb = 32;
let file_size = metadata(path)?.len();

match ChunkSize::validate_user_input(user_size_mb, file_size) {
    Ok(bytes) => {
        let chunk_size = ChunkSize::new(bytes)?;
        // Use validated size
    },
    Err(msg) => {
        eprintln!("Invalid chunk size: {}", msg);
        // Use optimal instead
        let chunk_size = ChunkSize::optimal_for_file_size(file_size);
    },
}
```

### 4. Monitor Chunk Processing

```rust
// ✅ Good: Track progress
for (i, chunk) in chunks.iter().enumerate() {
    let start = Instant::now();
    process_chunk(chunk)?;
    let duration = start.elapsed();

    println!("Chunk {}/{}: {:?}",
        i + 1, chunks.len(), duration);
}
```

### 5. Handle Edge Cases

```rust
// ✅ Good: Handle small files
if file_size < chunk_size.bytes() as u64 {
    // File fits in single chunk
    let chunk_size = ChunkSize::new(file_size as usize)?;
}

// ✅ Good: Handle empty files
if file_size == 0 {
    return Ok(Vec::new());  // No chunks needed
}
```

### 6. Use Checksums for Integrity

```rust
// ✅ Good: Add checksums to chunks
let chunk = FileChunk::new(seq, offset, data, is_final)?
    .with_calculated_checksum();

// Verify before processing
if !chunk.verify_checksum() {
    return Err(PipelineError::ChecksumMismatch);
}
```

---

## Troubleshooting

### Issue 1: Out of Memory with Large Chunks

**Symptom:**
```text
Error: Out of memory allocating chunk
```

**Solutions:**

```rust
// 1. Reduce chunk size
let smaller = ChunkSize::from_mb(4)?;  // Instead of 128 MB

// 2. Adjust for available memory
let adjusted = chunk_size.adjust_for_memory(
    available_memory,
    max_parallel,
)?;

// 3. Process sequentially instead of parallel
for chunk in chunks {
    process_chunk(chunk).await?;
    // Chunk dropped, memory freed
}
```

### Issue 2: Poor Performance with Small Chunks

**Symptom:** Processing is slower than expected.

**Diagnosis:**

```rust
let start = Instant::now();
let chunks = chunk_file(path, chunk_size)?;
let duration = start.elapsed();

println!("Chunking took: {:?}", duration);
println!("Chunks created: {}", chunks.len());
println!("Avg per chunk: {:?}", duration / chunks.len() as u32);
```

**Solutions:**

```rust
// 1. Use optimal chunk size
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

// 2. Increase chunk size
let larger = ChunkSize::from_mb(16)?;  // Instead of 1 MB

// 3. Benchmark different sizes
let results = benchmark_chunk_sizes(path, &sizes).await;
let (best_size, _) = results.first().unwrap();
```

### Issue 3: Too Many Chunks

**Symptom:**
```text
Created 10,000 chunks for 1 GB file
```

**Solutions:**

```rust
// 1. Increase chunk size
let chunk_size = ChunkSize::from_mb(16)?;  // ~63 chunks for 1 GB

// 2. Use optimal sizing
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

// 3. Set maximum chunk count
let max_chunks = 100;
let min_chunk_size = file_size / max_chunks as u64;
let chunk_size = ChunkSize::new(min_chunk_size as usize)?;
```

### Issue 4: Chunk Size Larger Than File

**Symptom:**
```text
Error: Chunk size 16 MB is larger than file size (1 MB)
```

**Solutions:**

```rust
// 1. Validate before chunking
let chunk_size = if file_size < chunk_size.bytes() as u64 {
    ChunkSize::new(file_size as usize)?
} else {
    chunk_size
};

// 2. Use validate_user_input
match ChunkSize::validate_user_input(user_size_mb, file_size) {
    Ok(bytes) => ChunkSize::new(bytes)?,
    Err(msg) => {
        eprintln!("Warning: {}", msg);
        ChunkSize::optimal_for_file_size(file_size)
    },
}
```

---

## Testing Strategies

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_size_validation() {
        // Valid sizes
        assert!(ChunkSize::new(1024).is_ok());
        assert!(ChunkSize::new(1024 * 1024).is_ok());

        // Invalid sizes
        assert!(ChunkSize::new(0).is_err());
        assert!(ChunkSize::new(600 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_optimal_sizing() {
        // Small file
        let small = ChunkSize::optimal_for_file_size(500_000);
        assert_eq!(small.bytes(), 64 * 1024);

        // Medium file
        let medium = ChunkSize::optimal_for_file_size(100 * 1024 * 1024);
        assert_eq!(medium.bytes(), 16 * 1024 * 1024);

        // Large file
        let large = ChunkSize::optimal_for_file_size(1_000_000_000);
        assert_eq!(large.bytes(), 64 * 1024 * 1024);
    }

    #[test]
    fn test_chunks_needed() {
        let chunk_size = ChunkSize::from_mb(4).unwrap();
        let file_size = 100 * 1024 * 1024;  // 100 MB

        let num_chunks = chunk_size.chunks_needed_for_file(file_size);
        assert_eq!(num_chunks, 25);  // 100 MB / 4 MB = 25
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_file_chunking() {
    // Create test file
    let test_file = create_test_file(10 * 1024 * 1024);  // 10 MB

    // Chunk the file
    let chunk_size = ChunkSize::from_mb(1).unwrap();
    let chunks = chunk_file(&test_file, chunk_size).unwrap();

    assert_eq!(chunks.len(), 10);

    // Verify sequences
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.sequence_number(), i as u64);
    }

    // Verify last chunk flag
    assert!(chunks.last().unwrap().is_final());
}
```

### Performance Tests

```rust
#[tokio::test]
async fn test_chunking_performance() {
    let test_file = create_test_file(100 * 1024 * 1024);  // 100 MB

    let sizes = vec![
        ChunkSize::from_mb(1).unwrap(),
        ChunkSize::from_mb(16).unwrap(),
        ChunkSize::from_mb(64).unwrap(),
    ];

    for size in sizes {
        let start = Instant::now();
        let chunks = chunk_file(&test_file, size).unwrap();
        let duration = start.elapsed();

        println!("{} MB chunks: {} chunks in {:?}",
            size.megabytes(), chunks.len(), duration);
    }
}
```

---

## Next Steps

After understanding chunking strategy, explore related topics:

### Related Implementation Topics

1. **[File I/O](file-io.md)**: File reading and writing with chunks
2. **[Binary Format](binary-format.md)**: How chunks are serialized

### Related Topics

- **[Stage Processing](stages.md)**: How stages process chunks
- **[Compression](compression.md)**: Compressing chunk data
- **[Encryption](encryption.md)**: Encrypting chunks

### Advanced Topics

- **[Performance Optimization](../advanced/performance.md)**: Optimizing chunking performance
- **[Concurrency Model](../advanced/concurrency.md)**: Parallel chunk processing

---

## Summary

**Key Takeaways:**

1. **Chunking** divides files into manageable pieces for efficient processing
2. **Chunk sizes** range from 1 byte to 512 MB with optimal sizes empirically determined
3. **Optimal sizing** adapts to file size: small files use small chunks, large files use large chunks
4. **Memory management** ensures bounded memory usage regardless of file size
5. **Parallel processing** enables concurrent chunk processing for better performance
6. **Adaptive chunking** can dynamically adjust chunk sizes based on performance
7. **Performance** varies significantly with chunk size (64 KB: ~40 MB/s, 128 MB: ~350 MB/s)

**Architecture File References:**
- **ChunkSize:** `pipeline-domain/src/value_objects/chunk_size.rs:169`
- **FileChunk:** `pipeline-domain/src/value_objects/file_chunk.rs:176`
- **Chunking Diagram:** `pipeline/docs/diagrams/chunk-processing.puml`
