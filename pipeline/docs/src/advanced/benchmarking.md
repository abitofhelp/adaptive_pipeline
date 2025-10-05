# Benchmarking

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter covers the pipeline's benchmark suite, methodologies for measuring performance, and techniques for interpreting benchmark results to guide optimization decisions.

## Overview

The pipeline uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for rigorous, statistical benchmarking. Criterion provides:

- **Statistical Analysis**: Measures mean, standard deviation, and percentiles
- **Regression Detection**: Identifies performance regressions automatically
- **HTML Reports**: Generates detailed visualizations and comparisons
- **Outlier Detection**: Filters statistical outliers for consistent results
- **Iterative Measurement**: Automatically determines iteration counts

**Benchmark Location:** `pipeline/benches/file_io_benchmark.rs`

## Benchmark Suite

The benchmark suite covers four main categories:

### 1. Read Method Benchmarks

**Purpose**: Compare regular file I/O vs memory-mapped I/O across different file sizes.

**File Sizes Tested:**
- 1 MB (small files)
- 10 MB (medium files)
- 50 MB (large files)
- 100 MB (very large files)

**Methods Compared:**
- `regular_io`: Traditional buffered file reading
- `memory_mapped`: Memory-mapped file access (mmap)

**Configuration:**
```rust
let service = FileIOServiceImpl::new(FileIOConfig {
    default_chunk_size: 64 * 1024,     // 64KB chunks
    max_mmap_size: 1024 * 1024 * 1024, // 1GB threshold
    enable_memory_mapping: true,
    ..Default::default()
});
```

**Expected Results:**
- **Small files (< 10 MB)**: Regular I/O faster (lower setup cost)
- **Large files (> 50 MB)**: Memory mapping faster (reduced copying)

### 2. Chunk Size Benchmarks

**Purpose**: Determine optimal chunk sizes for different workloads.

**Chunk Sizes Tested:**
- 4 KB (4096 bytes)
- 8 KB (8192 bytes)
- 16 KB (16384 bytes)
- 32 KB (32768 bytes)
- 64 KB (65536 bytes)
- 128 KB (131072 bytes)

**File Size:** 10 MB (representative medium file)

**Methods:**
- `regular_io`: Chunked reading with various sizes
- `memory_mapped`: Memory mapping with logical chunk sizes

**Expected Results:**
- **Smaller chunks (4-16 KB)**: Higher overhead, more syscalls
- **Medium chunks (32-64 KB)**: Good balance for most workloads
- **Larger chunks (128 KB+)**: Lower syscall overhead, higher memory usage

### 3. Checksum Calculation Benchmarks

**Purpose**: Measure overhead of integrity verification.

**Benchmarks:**
- `with_checksums`: File reading with SHA-256 checksum calculation
- `without_checksums`: File reading without checksums
- `checksum_only`: Standalone checksum calculation

**File Size:** 10 MB

**Expected Results:**
- Checksum overhead: 10-30% depending on CPU and chunk size
- Larger chunks reduce relative overhead (fewer per-chunk hash operations)

### 4. Write Operation Benchmarks

**Purpose**: Compare write performance across data sizes and options.

**Data Sizes Tested:**
- 1 KB (tiny writes)
- 10 KB (small writes)
- 100 KB (medium writes)
- 1000 KB / 1 MB (large writes)

**Write Options:**
- `write_data`: Buffered write without checksum
- `write_data_with_checksum`: Buffered write with SHA-256 checksum
- `write_data_sync`: Buffered write with fsync

**Expected Results:**
- Write throughput: 100-1000 MB/s (buffered, no sync)
- Checksum overhead: 10-30%
- Sync overhead: 10-100x depending on storage device

## Running Benchmarks

### Basic Usage

**Run all benchmarks:**
```bash
cargo bench --bench file_io_benchmark
```

**Output:**
```text
file_read_methods/regular_io/1    time:   [523.45 µs 528.12 µs 533.28 µs]
file_read_methods/regular_io/10   time:   [5.1234 ms 5.2187 ms 5.3312 ms]
file_read_methods/memory_mapped/1 time:   [612.34 µs 618.23 µs 624.91 µs]
file_read_methods/memory_mapped/10 time: [4.8923 ms 4.9721 ms 5.0584 ms]
...
```

### Benchmark Groups

**Run specific benchmark group:**

```bash
# Read methods only
cargo bench --bench file_io_benchmark -- "file_read_methods"

# Chunk sizes only
cargo bench --bench file_io_benchmark -- "chunk_sizes"

# Checksums only
cargo bench --bench file_io_benchmark -- "checksum_calculation"

# Write operations only
cargo bench --bench file_io_benchmark -- "write_operations"
```

### Specific Benchmarks

**Run benchmarks matching a pattern:**

```bash
# All regular_io benchmarks
cargo bench --bench file_io_benchmark -- "regular_io"

# Only 50MB file benchmarks
cargo bench --bench file_io_benchmark -- "/50"

# Memory-mapped with 64KB chunks
cargo bench --bench file_io_benchmark -- "memory_mapped/64"
```

### Output Formats

**HTML report (default):**
```bash
cargo bench --bench file_io_benchmark

# Opens: target/criterion/report/index.html
open target/criterion/report/index.html
```

**Save baseline for comparison:**
```bash
# Save current performance as baseline
cargo bench --bench file_io_benchmark -- --save-baseline main

# Compare against baseline after changes
cargo bench --bench file_io_benchmark -- --baseline main
```

**Example comparison output:**
```text
file_read_methods/regular_io/50
                        time:   [24.512 ms 24.789 ms 25.091 ms]
                        change: [-5.2341% -4.1234% -2.9876%] (p = 0.00 < 0.05)
                        Performance has improved.
```

## Interpreting Results

### Understanding Output

**Criterion output format:**
```text
benchmark_name          time:   [lower_bound estimate upper_bound]
                        change: [lower% estimate% upper%] (p = X.XX < 0.05)
```

**Components:**
- **lower_bound**: 95% confidence interval lower bound
- **estimate**: Best estimate (typically median)
- **upper_bound**: 95% confidence interval upper bound
- **change**: Performance change vs baseline (if available)
- **p-value**: Statistical significance (< 0.05 = significant)

**Example:**
```text
file_read_methods/regular_io/50
                        time:   [24.512 ms 24.789 ms 25.091 ms]
                        change: [-5.2341% -4.1234% -2.9876%] (p = 0.00 < 0.05)
```

**Interpretation:**
- **Time**: File reading takes ~24.79 ms (median)
- **Confidence**: 95% certain actual time is between 24.51-25.09 ms
- **Change**: 4.12% faster than baseline (statistically significant)
- **Conclusion**: Performance improvement confirmed

### Throughput Calculation

**Calculate MB/s from benchmark time:**

```text
File size: 50 MB
Time: 24.789 ms = 0.024789 seconds

Throughput = 50 MB / 0.024789 s = 2017 MB/s
```

**Rust code:**
```rust
let file_size_mb = 50.0;
let time_ms = 24.789;
let throughput_mb_s = file_size_mb / (time_ms / 1000.0);

println!("Throughput: {:.2} MB/s", throughput_mb_s);  // 2017 MB/s
```

### Regression Detection

**Performance regression indicators:**

✅ **Improvement** (faster):
```text
change: [-10.234% -8.123% -6.012%] (p = 0.00 < 0.05)
Performance has improved.
```

❌ **Regression** (slower):
```text
change: [+6.234% +8.456% +10.891%] (p = 0.00 < 0.05)
Performance has regressed.
```

⚠️ **No significant change**:
```text
change: [-1.234% +0.456% +2.123%] (p = 0.42 > 0.05)
No change in performance detected.
```

**Statistical significance:**
- **p < 0.05**: Change is statistically significant (95% confidence)
- **p > 0.05**: Change could be noise (not statistically significant)

### HTML Report Navigation

Criterion generates interactive HTML reports at `target/criterion/report/index.html`.

**Report Sections:**

1. **Summary**: All benchmarks with comparisons
2. **Individual Reports**: Detailed analysis per benchmark
3. **Violin Plots**: Distribution visualization
4. **History**: Performance over time

**Key Metrics in Report:**
- **Mean**: Average execution time
- **Median**: 50th percentile (typical case)
- **Std Dev**: Variability in measurements
- **MAD**: Median Absolute Deviation (robust to outliers)
- **Slope**: Linear regression slope (for iteration scaling)

## Performance Baselines

### Establishing Baselines

**Save baseline before optimizations:**

```bash
# 1. Run benchmarks on main branch
git checkout main
cargo bench --bench file_io_benchmark -- --save-baseline main

# 2. Switch to feature branch
git checkout feature/optimize-io

# 3. Run benchmarks and compare
cargo bench --bench file_io_benchmark -- --baseline main
```

**Baseline management:**

```bash
# List all baselines
ls target/criterion/*/base/

# Delete old baseline
rm -rf target/criterion/*/baseline_name/

# Compare two baselines
cargo bench -- --baseline old_baseline --save-baseline new_baseline
```

### Baseline Strategy

**Recommended baselines:**

1. **main**: Current production performance
2. **release-X.Y.Z**: Tagged release versions
3. **pre-optimization**: Before major optimization work
4. **target**: Performance goals

**Example workflow:**

```bash
# Establish target baseline (goals)
cargo bench -- --save-baseline target

# Work on optimizations...
# (make changes, run benchmarks)

# Compare to target
cargo bench -- --baseline target

# If goals met, update main baseline
cargo bench -- --save-baseline main
```

## Continuous Integration

### CI/CD Integration

**GitHub Actions example:**

```yaml
name: Benchmarks

on:
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run benchmarks
        run: |
          cargo bench --bench file_io_benchmark -- --save-baseline pr

      - name: Compare to main
        run: |
          git fetch origin main:main
          git checkout main
          cargo bench --bench file_io_benchmark -- --save-baseline main
          git checkout -
          cargo bench --bench file_io_benchmark -- --baseline main

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion/
```

### Regression Alerts

**Detect regressions in CI:**

```bash
#!/bin/bash
# scripts/check_benchmarks.sh

# Run benchmarks and save output
cargo bench --bench file_io_benchmark -- --baseline main > bench_output.txt

# Check for regressions
if grep -q "Performance has regressed" bench_output.txt; then
    echo "❌ Performance regression detected!"
    grep "Performance has regressed" bench_output.txt
    exit 1
else
    echo "✅ No performance regressions detected"
    exit 0
fi
```

## Benchmark Best Practices

### 1. Use Representative Workloads

```rust
// ✅ Good: Test realistic file sizes
for size_mb in [1, 10, 50, 100, 500, 1000].iter() {
    let test_file = create_test_file(*size_mb);
    benchmark_file_io(&test_file);
}

// ❌ Bad: Only tiny files
let test_file = create_test_file(1);  // Not representative!
```

### 2. Control Variables

```rust
// ✅ Good: Isolate what you're measuring
group.bench_function("compression_only", |b| {
    b.iter(|| {
        // Only benchmark compression, not I/O
        compress_data(black_box(&test_data))
    });
});

// ❌ Bad: Measuring multiple things
group.bench_function("compression_and_io", |b| {
    b.iter(|| {
        let data = read_file(path);  // I/O overhead!
        compress_data(&data)         // Compression
    });
});
```

### 3. Use `black_box` to Prevent Optimization

```rust
// ✅ Good: Prevent compiler from eliminating code
b.iter(|| {
    let result = expensive_operation();
    black_box(result);  // Ensures result is not optimized away
});

// ❌ Bad: Compiler may optimize away the work
b.iter(|| {
    expensive_operation();  // Result unused, may be eliminated!
});
```

### 4. Warm Up Before Measuring

```rust
// ✅ Good: Criterion handles warmup automatically
c.bench_function("my_benchmark", |b| {
    // Criterion runs warmup iterations automatically
    b.iter(|| expensive_operation());
});

// ❌ Bad: Manual warmup (unnecessary with Criterion)
for _ in 0..100 {
    expensive_operation();  // Criterion does this for you!
}
```

### 5. Measure Multiple Configurations

```rust
// ✅ Good: Test parameter space
let mut group = c.benchmark_group("chunk_sizes");
for chunk_size in [4096, 8192, 16384, 32768, 65536].iter() {
    group.bench_with_input(
        BenchmarkId::from_parameter(chunk_size),
        chunk_size,
        |b, &size| b.iter(|| process_with_chunk_size(size))
    );
}

// ❌ Bad: Single configuration
c.bench_function("process", |b| {
    b.iter(|| process_with_chunk_size(65536));  // Only one size!
});
```

## Example Benchmark Analysis

### Scenario: Optimizing Chunk Size

**1. Run baseline benchmarks:**

```bash
cargo bench --bench file_io_benchmark -- "chunk_sizes" --save-baseline before
```

**Results:**
```text
chunk_sizes/regular_io/4096   time:   [82.341 ms 83.129 ms 83.987 ms]
chunk_sizes/regular_io/16384  time:   [62.123 ms 62.891 ms 63.712 ms]
chunk_sizes/regular_io/65536  time:   [52.891 ms 53.523 ms 54.201 ms]
```

**2. Calculate throughput:**

```text
File size: 10 MB
4KB chunks:   10 / 0.083129 = 120.3 MB/s
16KB chunks:  10 / 0.062891 = 159.0 MB/s
64KB chunks:  10 / 0.053523 = 186.8 MB/s
```

**3. Analysis:**

- **4 KB chunks**: Slow (120 MB/s) due to syscall overhead
- **16 KB chunks**: Better (159 MB/s), balanced
- **64 KB chunks**: Best (187 MB/s), amortizes overhead

**4. Recommendation:**

Use 64 KB chunks for medium files (10-100 MB).

### Scenario: Memory Mapping Threshold

**1. Run benchmarks across file sizes:**

```bash
cargo bench --bench file_io_benchmark -- "file_read_methods"
```

**Results:**
```text
regular_io/1      time:   [523.45 µs 528.12 µs 533.28 µs]  → 1894 MB/s
memory_mapped/1   time:   [612.34 µs 618.23 µs 624.91 µs]  → 1618 MB/s

regular_io/50     time:   [24.512 ms 24.789 ms 25.091 ms]  → 2017 MB/s
memory_mapped/50  time:   [19.234 ms 19.512 ms 19.812 ms]  → 2563 MB/s

regular_io/100    time:   [52.123 ms 52.891 ms 53.712 ms]  → 1891 MB/s
memory_mapped/100 time:   [38.234 ms 38.712 ms 39.234 ms]  → 2584 MB/s
```

**2. Crossover analysis:**

- **1 MB**: Regular I/O faster (1894 vs 1618 MB/s)
- **50 MB**: Memory mapping faster (2563 vs 2017 MB/s) - **27% improvement**
- **100 MB**: Memory mapping faster (2584 vs 1891 MB/s) - **37% improvement**

**3. Recommendation:**

Use memory mapping for files > 10 MB (threshold between 1-50 MB).

## Related Topics

- See [Performance Optimization](performance.md) for optimization strategies
- See [Profiling](profiling.md) for CPU and memory profiling
- See [Thread Pooling](thread-pooling.md) for concurrency tuning
- See [Resource Management](resources.md) for resource limits

## Summary

The pipeline's benchmarking suite provides:

1. **Comprehensive Coverage**: Read/write operations, chunk sizes, checksums
2. **Statistical Rigor**: Criterion.rs with confidence intervals and regression detection
3. **Baseline Comparison**: Track performance changes over time
4. **CI/CD Integration**: Automated regression detection in pull requests
5. **HTML Reports**: Interactive visualizations and detailed analysis

**Key Takeaways:**
- Run benchmarks before and after optimizations (`--save-baseline`)
- Use representative workloads (realistic file sizes and configurations)
- Look for statistically significant changes (p < 0.05)
- Calculate throughput (MB/s) for intuitive performance comparison
- Integrate benchmarks into CI/CD for regression prevention
- Use HTML reports for detailed analysis and visualization

**Benchmark Commands:**
```bash
# Run all benchmarks
cargo bench --bench file_io_benchmark

# Save baseline
cargo bench --bench file_io_benchmark -- --save-baseline main

# Compare to baseline
cargo bench --bench file_io_benchmark -- --baseline main

# Specific group
cargo bench --bench file_io_benchmark -- "file_read_methods"
```
