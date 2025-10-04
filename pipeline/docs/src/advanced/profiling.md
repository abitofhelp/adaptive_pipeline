# Profiling

**Version:** 0.1.0
**Date:** October 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter covers profiling tools and techniques for identifying performance bottlenecks, analyzing CPU usage, memory allocation patterns, and optimizing the pipeline's runtime characteristics.

## Overview

**Profiling** is the process of measuring where your program spends time and allocates memory. Unlike benchmarking (which measures aggregate performance), profiling provides detailed insights into:

- **CPU hotspots**: Which functions consume the most CPU time
- **Memory allocation**: Where and how much memory is allocated
- **Lock contention**: Where threads wait for synchronization
- **Cache misses**: Memory access patterns affecting performance

**When to profile:**
- ✅ After identifying performance issues in benchmarks
- ✅ When optimizing critical paths
- ✅ When investigating unexplained slowness
- ✅ Before and after major architectural changes

**When NOT to profile:**
- ❌ Before establishing performance goals
- ❌ On trivial workloads (profile representative cases)
- ❌ Without benchmarks (profile after quantifying the issue)

## Profiling Tools

### CPU Profiling Tools

| Tool | Platform | Sampling | Instrumentation | Overhead | Output |
|------|----------|----------|-----------------|----------|--------|
| **perf** | Linux | Yes | No | Low (1-5%) | Text/perf.data |
| **flamegraph** | All | Yes | No | Low (1-5%) | SVG |
| **samply** | All | Yes | No | Low (1-5%) | Firefox Profiler |
| **Instruments** | macOS | Yes | Optional | Low-Med | GUI |
| **VTune** | Linux/Windows | Yes | Optional | Med | GUI |

**Recommended for Rust:**
- **Linux**: `perf` + `flamegraph`
- **macOS**: `samply` or Instruments
- **Windows**: VTune or Windows Performance Analyzer

### Memory Profiling Tools

| Tool | Platform | Heap | Leaks | Peak Usage | Output |
|------|----------|------|-------|------------|--------|
| **valgrind (massif)** | Linux/macOS | Yes | No | Yes | Text/ms_print |
| **heaptrack** | Linux | Yes | Yes | Yes | GUI |
| **dhat** | Linux/macOS | Yes | No | Yes | JSON/Web UI |
| **Instruments** | macOS | Yes | Yes | Yes | GUI |

**Recommended for Rust:**
- **Linux**: `heaptrack` or `dhat`
- **macOS**: Instruments (Allocations template)
- **Memory leaks**: `valgrind (memcheck)`

## CPU Profiling

### Using `perf` (Linux)

**Setup:**

```bash
# Install perf
sudo apt-get install linux-tools-common linux-tools-generic  # Ubuntu/Debian
sudo dnf install perf  # Fedora

# Build with debug symbols
cargo build --release --bin pipeline

# Enable perf for non-root users
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```

**Record profile:**

```bash
# Profile pipeline execution
perf record --call-graph dwarf \
    ./target/release/pipeline process testdata/large-file.bin

# Output: perf.data
```

**Analyze results:**

```bash
# Interactive TUI
perf report

# Text summary
perf report --stdio

# Function call graph
perf report --stdio --no-children

# Top functions
perf report --stdio | head -20
```

**Example output:**

```text
# Overhead  Command   Shared Object     Symbol
# ........  ........  ................  .......................
    45.23%  pipeline  libbrotli.so      BrotliEncoderCompress
    18.91%  pipeline  pipeline          rayon::iter::collect
    12.34%  pipeline  libcrypto.so      AES_encrypt
     8.72%  pipeline  pipeline          tokio::runtime::task
     6.45%  pipeline  libc.so           memcpy
     3.21%  pipeline  pipeline          std::io::Write::write_all
```

**Interpretation:**
- **45% in BrotliEncoderCompress**: Compression is the hotspot
- **19% in rayon::iter::collect**: Parallel iteration overhead
- **12% in AES_encrypt**: Encryption is expensive but expected
- **9% in Tokio task**: Async runtime overhead

**Optimization targets:**
1. Use faster compression (LZ4 instead of Brotli)
2. Reduce Rayon parallelism overhead (larger chunks)
3. Use AES-NI hardware acceleration

### Using Flamegraph (All Platforms)

**Setup:**

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Linux: Install perf (see above)

# macOS: Install DTrace (built-in)

# Windows: Install WPA (Windows Performance Analyzer)
```

**Generate flamegraph:**

```bash
# Profile and generate SVG
cargo flamegraph --bin pipeline -- process testdata/large-file.bin

# Output: flamegraph.svg
open flamegraph.svg
```

**Flamegraph structure:**

```text
┌─────────────────────────────────────────────────────────────┐
│                     main (100%)                             │ ← Root
├─────────────────────┬───────────────────┬───────────────────┤
│ tokio::runtime (50%)│ rayon::iter (30%) │ other (20%)       │
├──────┬──────┬───────┼─────────┬─────────┴───────────────────┤
│ I/O  │ task │ ... │compress │ encrypt │ hash │ ...          │
│ 20%  │ 15%  │  15%│   18%   │   8%    │  4%  │              │
└──────┴──────┴───────┴─────────┴─────────┴──────┴─────────────┘
```

**Reading flamegraphs:**
- **Width**: Percentage of CPU time (wider = more time)
- **Height**: Call stack depth (bottom = entry point, top = leaf functions)
- **Color**: Random (for visual distinction, not meaningful)
- **Interactive**: Click to zoom, search functions

**Example analysis:**

If you see:
```text
main → tokio::runtime → spawn_blocking → rayon::par_iter → compress → brotli
                                                               45%
```

**Interpretation:**
- 45% of time spent in Brotli compression
- Called through Rayon parallel iteration
- Spawned from Tokio's blocking thread pool

**Action:**
- Evaluate if 45% compression time is acceptable
- If too slow, switch to LZ4 or reduce compression level

### Using Samply (All Platforms)

**Setup:**

```bash
# Install samply
cargo install samply
```

**Profile and view:**

```bash
# Profile and open in Firefox Profiler
samply record --release -- cargo run --bin pipeline -- process testdata/large-file.bin

# Automatically opens in Firefox Profiler web UI
```

**Firefox Profiler features:**
- **Timeline view**: See CPU usage over time
- **Call tree**: Hierarchical function breakdown
- **Flame graph**: Interactive flamegraph
- **Marker timeline**: Custom events and annotations
- **Thread activity**: Per-thread CPU usage

**Advantages over static flamegraphs:**
- ✅ Interactive (zoom, filter, search)
- ✅ Timeline view (see activity over time)
- ✅ Multi-threaded visualization
- ✅ Easy sharing (web-based)

## Memory Profiling

### Using Valgrind Massif (Linux/macOS)

**Setup:**

```bash
# Install valgrind
sudo apt-get install valgrind  # Ubuntu/Debian
brew install valgrind  # macOS (Intel only)
```

**Profile heap usage:**

```bash
# Run with Massif
valgrind --tool=massif --massif-out-file=massif.out \
    ./target/release/pipeline process testdata/large-file.bin

# Visualize results
ms_print massif.out
```

**Example output:**

```text
--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
  0              0                0                0             0            0
  1        123,456       64,000,000       64,000,000             0            0
  2        234,567      128,000,000      128,000,000             0            0
  3        345,678      192,000,000      192,000,000             0            0  ← Peak
  4        456,789      128,000,000      128,000,000             0            0
  5        567,890       64,000,000       64,000,000             0            0
  6        678,901                0                0             0            0

Peak memory: 192 MB

Top allocations:
    45.2%  (87 MB)  Vec::with_capacity (chunk buffer)
    23.1%  (44 MB)  Vec::with_capacity (compressed data)
    15.7%  (30 MB)  Box::new (encryption context)
    ...
```

**Interpretation:**
- **Peak memory**: 192 MB
- **Main contributor**: 87 MB chunk buffers (45%)
- **Growth pattern**: Linear increase then decrease (expected for streaming)

**Optimization:**
- Reduce chunk size to lower peak memory
- Reuse buffers instead of allocating new ones
- Use `SmallVec` for small allocations

### Using Heaptrack (Linux)

**Setup:**

```bash
# Install heaptrack
sudo apt-get install heaptrack heaptrack-gui  # Ubuntu/Debian
```

**Profile and analyze:**

```bash
# Record heap usage
heaptrack ./target/release/pipeline process testdata/large-file.bin

# Output: heaptrack.pipeline.12345.gz

# Analyze with GUI
heaptrack_gui heaptrack.pipeline.12345.gz
```

**Heaptrack GUI features:**
- **Summary**: Total allocations, peak memory, leaked memory
- **Top allocators**: Functions allocating the most
- **Flame graph**: Allocation call chains
- **Timeline**: Memory usage over time
- **Leak detection**: Allocated but never freed

**Example metrics:**

```text
Total allocations: 1,234,567
Total allocated: 15.2 GB
Peak heap: 192 MB
Peak RSS: 256 MB
Leaked: 0 bytes
```

**Top allocators:**
```text
1. Vec::with_capacity (chunk buffer)        5.4 GB  (35%)
2. tokio::spawn (task allocation)           2.1 GB  (14%)
3. Vec::with_capacity (compressed data)     1.8 GB  (12%)
```

### Using DHAT (Linux/macOS)

**Setup:**

```bash
# Install valgrind with DHAT
sudo apt-get install valgrind
```

**Profile with DHAT:**

```bash
# Run with DHAT
valgrind --tool=dhat --dhat-out-file=dhat.out \
    ./target/release/pipeline process testdata/large-file.bin

# Output: dhat.out.12345

# View in web UI
dhat_viewer dhat.out.12345
# Or upload to https://nnethercote.github.io/dh_view/dh_view.html
```

**DHAT metrics:**

- **Total bytes allocated**: Cumulative allocation size
- **Total blocks allocated**: Number of allocations
- **Peak bytes**: Maximum heap size
- **Average block size**: Typical allocation size
- **Short-lived**: Allocations freed quickly

**Example output:**

```text
Total:     15.2 GB in 1,234,567 blocks
Peak:      192 MB
At t-gmax: 187 MB in 145 blocks
At t-end:  0 B in 0 blocks

Top allocation sites:
  1. 35.4% (5.4 GB in 98,765 blocks)
     Vec::with_capacity (file_io.rs:123)
     ← chunk buffer allocation

  2. 14.2% (2.1 GB in 567,890 blocks)
     tokio::spawn (runtime.rs:456)
     ← task overhead

  3. 11.8% (1.8 GB in 45,678 blocks)
     Vec::with_capacity (compression.rs:789)
     ← compressed buffer
```

## Profiling Workflows

### Workflow 1: Identify CPU Hotspot

**1. Establish baseline (benchmark):**

```bash
cargo bench --bench file_io_benchmark -- --save-baseline before
```

**2. Profile with perf + flamegraph:**

```bash
cargo flamegraph --bin pipeline -- process testdata/large-file.bin
open flamegraph.svg
```

**3. Identify hotspot:**

Look for wide bars in flamegraph (e.g., 45% in `BrotliEncoderCompress`).

**4. Optimize:**

```rust
// Before: Brotli (slow, high compression)
let compression = CompressionAlgorithm::Brotli;

// After: LZ4 (fast, lower compression)
let compression = CompressionAlgorithm::Lz4;
```

**5. Verify improvement:**

```bash
cargo flamegraph --bin pipeline -- process testdata/large-file.bin
# Check if Brotli bar shrunk

cargo bench --bench file_io_benchmark -- --baseline before
# Expect: Performance has improved
```

### Workflow 2: Reduce Memory Usage

**1. Profile heap usage:**

```bash
heaptrack ./target/release/pipeline process testdata/large-file.bin
heaptrack_gui heaptrack.pipeline.12345.gz
```

**2. Identify large allocations:**

Look for top allocators (e.g., 87 MB chunk buffers).

**3. Calculate optimal size:**

```text
Current: 64 MB chunks × 8 workers = 512 MB peak
Target: < 256 MB peak
Solution: 16 MB chunks × 8 workers = 128 MB peak
```

**4. Reduce chunk size:**

```rust
// Before
let chunk_size = ChunkSize::from_mb(64)?;

// After
let chunk_size = ChunkSize::from_mb(16)?;
```

**5. Verify reduction:**

```bash
heaptrack ./target/release/pipeline process testdata/large-file.bin
# Check peak memory: should be < 256 MB
```

### Workflow 3: Optimize Parallel Code

**1. Profile with perf:**

```bash
perf record --call-graph dwarf \
    ./target/release/pipeline process testdata/large-file.bin

perf report --stdio
```

**2. Check for synchronization overhead:**

```text
12.3%  pipeline  [kernel]     futex_wait    ← Lock contention
 8.7%  pipeline  pipeline     rayon::join    ← Coordination
 6.5%  pipeline  pipeline     Arc::clone     ← Reference counting
```

**3. Reduce contention:**

```rust
// Before: Shared mutex (high contention)
let counter = Arc::new(Mutex::new(0));

// After: Per-thread counters (no contention)
let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::Relaxed);
```

**4. Verify improvement:**

```bash
perf record ./target/release/pipeline process testdata/large-file.bin
perf report --stdio
# Check if futex_wait reduced
```

## Interpreting Results

### CPU Profile Patterns

**Pattern 1: Single Hotspot**

```text
compress_chunk: 65%  ← Dominant function
encrypt_chunk:  15%
write_chunk:    10%
other:          10%
```

**Action**: Optimize the 65% hotspot (use faster algorithm or optimize implementation).

**Pattern 2: Distributed Cost**

```text
compress: 20%
encrypt:  18%
hash:     15%
io:       22%
other:    25%
```

**Action**: No single hotspot. Profile deeper or optimize multiple functions.

**Pattern 3: Framework Overhead**

```text
tokio::runtime: 35%  ← High async overhead
rayon::iter:    25%  ← High parallel overhead
actual_work:    40%
```

**Action**: Reduce task spawning frequency, batch operations, or use sync code for CPU-bound work.

### Memory Profile Patterns

**Pattern 1: Linear Growth**

```text
Memory over time:
  0s:   0 MB
 10s:  64 MB
 20s: 128 MB  ← Growing linearly
 30s: 192 MB
```

**Likely cause**: Streaming processing with bounded buffers (normal).

**Pattern 2: Sawtooth**

```text
Memory over time:
  0-5s:   0 → 128 MB  ← Allocate
  5s:   128 →   0 MB  ← Free
  6-11s:  0 → 128 MB  ← Allocate again
```

**Likely cause**: Batch processing with periodic flushing (normal).

**Pattern 3: Unbounded Growth**

```text
Memory over time:
  0s:   0 MB
 10s:  64 MB
 20s: 158 MB  ← Growing faster than linear
 30s: 312 MB
```

**Likely cause**: Memory leak (allocated but never freed).

**Action**: Use heaptrack to identify leak source, ensure proper cleanup with RAII.

## Common Performance Issues

### Issue 1: Lock Contention

**Symptoms:**
- High `futex_wait` in perf
- Threads spending time in synchronization

**Diagnosis:**

```bash
perf record --call-graph dwarf ./target/release/pipeline ...
perf report | grep futex
```

**Solutions:**
```rust
// ❌ Bad: Shared mutex
let counter = Arc::new(Mutex::new(0));

// ✅ Good: Atomic
let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::Relaxed);
```

### Issue 2: Excessive Allocations

**Symptoms:**
- High `malloc` / `free` in flamegraph
- Poor cache performance

**Diagnosis:**

```bash
heaptrack ./target/release/pipeline ...
# Check "Total allocations" count
```

**Solutions:**
```rust
// ❌ Bad: Allocate per iteration
for chunk in chunks {
    let buffer = vec![0; size];  // New allocation every time!
    process(buffer);
}

// ✅ Good: Reuse buffer
let mut buffer = vec![0; size];
for chunk in chunks {
    process(&mut buffer);
    buffer.clear();
}
```

### Issue 3: Small Task Overhead

**Symptoms:**
- High `tokio::spawn` or `rayon::spawn` overhead
- More framework time than actual work

**Diagnosis:**

```bash
cargo flamegraph --bin pipeline ...
# Check width of spawn-related functions
```

**Solutions:**
```rust
// ❌ Bad: Spawn per chunk
for chunk in chunks {
    tokio::spawn(async move { process(chunk).await });
}

// ✅ Good: Batch chunks
tokio::task::spawn_blocking(move || {
    RAYON_POOLS.cpu_bound_pool().install(|| {
        chunks.into_par_iter().map(process).collect()
    })
})
```

## Profiling Best Practices

### 1. Profile Release Builds

```bash
# ✅ Good: Release mode
cargo build --release
perf record ./target/release/pipeline ...

# ❌ Bad: Debug mode (10-100x slower, misleading results)
cargo build
perf record ./target/debug/pipeline ...
```

### 2. Use Representative Workloads

```bash
# ✅ Good: Large realistic file
perf record ./target/release/pipeline process testdata/100mb-file.bin

# ❌ Bad: Tiny file (setup overhead dominates)
perf record ./target/release/pipeline process testdata/1kb-file.bin
```

### 3. Profile Multiple Scenarios

```bash
# Profile different file sizes
perf record -o perf-small.data ./target/release/pipeline process small.bin
perf record -o perf-large.data ./target/release/pipeline process large.bin

# Compare results
perf report -i perf-small.data
perf report -i perf-large.data
```

### 4. Combine Profiling Tools

```bash
# 1. Flamegraph for overview
cargo flamegraph --bin pipeline -- process test.bin

# 2. perf for detailed analysis
perf record --call-graph dwarf ./target/release/pipeline process test.bin
perf report

# 3. Heaptrack for memory
heaptrack ./target/release/pipeline process test.bin
```

### 5. Profile Before and After Optimizations

```bash
# Before
cargo flamegraph -o before.svg --bin pipeline -- process test.bin
heaptrack -o before.gz ./target/release/pipeline process test.bin

# Make changes...

# After
cargo flamegraph -o after.svg --bin pipeline -- process test.bin
heaptrack -o after.gz ./target/release/pipeline process test.bin

# Compare visually
open before.svg after.svg
```

## Related Topics

- See [Performance Optimization](performance.md) for optimization strategies
- See [Benchmarking](benchmarking.md) for performance measurement
- See [Thread Pooling](thread-pooling.md) for concurrency tuning
- See [Resource Management](resources.md) for resource limits

## Summary

Profiling tools and techniques provide:

1. **CPU Profiling**: Identify hotspots with perf, flamegraph, samply
2. **Memory Profiling**: Track allocations with heaptrack, DHAT, Massif
3. **Workflows**: Systematic approaches to optimization
4. **Pattern Recognition**: Understand common performance issues
5. **Best Practices**: Profile release builds with representative workloads

**Key Takeaways:**
- Use `cargo flamegraph` for quick CPU profiling (all platforms)
- Use `heaptrack` for comprehensive memory analysis (Linux)
- Profile release builds (`cargo build --release`)
- Use representative workloads (large realistic files)
- Combine benchmarking (quantify) with profiling (diagnose)
- Profile before and after optimizations to verify improvements

**Quick Start:**
```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin pipeline -- process large-file.bin

# Memory profiling (Linux)
sudo apt-get install heaptrack
heaptrack ./target/release/pipeline process large-file.bin
heaptrack_gui heaptrack.pipeline.*.gz
```
