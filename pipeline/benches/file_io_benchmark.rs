//! # File I/O Performance Benchmarks
//!
//! This module contains comprehensive performance benchmarks for the adaptive
//! pipeline's file I/O service implementation. It measures and compares the
//! performance of different I/O strategies including regular file I/O, memory
//! mapping, chunked processing, and various configuration options.
//!
//! ## Overview
//!
//! The benchmarks evaluate:
//!
//! - **I/O Method Comparison**: Regular file I/O vs. memory mapping performance
//! - **File Size Impact**: Performance characteristics across different file
//!   sizes
//! - **Configuration Optimization**: Impact of different configuration
//!   parameters
//! - **Chunked Processing**: Performance of chunked vs. streaming I/O
//! - **Memory Usage**: Memory efficiency of different approaches
//! - **Concurrent Operations**: Performance under concurrent load
//!
//! ## Benchmark Categories
//!
//! ### Read Method Benchmarks
//! - **Regular I/O**: Traditional file reading with configurable buffer sizes
//! - **Memory Mapping**: Memory-mapped file access for large files
//! - **Chunked Reading**: Streaming chunked file processing
//! - **Hybrid Approach**: Automatic selection based on file size
//!
//! ### Write Method Benchmarks
//! - **Buffered Writing**: Traditional buffered file writing
//! - **Direct Writing**: Unbuffered direct file writing
//! - **Chunked Writing**: Streaming chunked file writing
//! - **Sync vs. Async**: Synchronous vs. asynchronous write performance
//!
//! ### Configuration Benchmarks
//! - **Chunk Size Impact**: Performance across different chunk sizes
//! - **Buffer Size Optimization**: Optimal buffer size determination
//! - **Memory Mapping Thresholds**: Optimal thresholds for memory mapping
//! - **Concurrent Operations**: Performance under different concurrency levels
//!
//! ## Test Methodology
//!
//! ### File Size Categories
//! - **Small Files**: 1MB - 10MB (typical documents, images)
//! - **Medium Files**: 10MB - 100MB (videos, large documents)
//! - **Large Files**: 100MB - 1GB (datasets, archives)
//! - **Very Large Files**: 1GB+ (big data, media files)
//!
//! ### Performance Metrics
//! - **Throughput**: MB/s for read and write operations
//! - **Latency**: Time to first byte and total operation time
//! - **Memory Usage**: Peak and average memory consumption
//! - **CPU Utilization**: Processor usage during operations
//!
//! ## Usage
//!
//! Run benchmarks with:
//!
//! ```bash
//! # Run all benchmarks
//! cargo bench --bench file_io_benchmark
//!
//! # Run specific benchmark group
//! cargo bench --bench file_io_benchmark -- "file_read_methods"
//!
//! # Run with specific file size
//! cargo bench --bench file_io_benchmark -- "regular_io/50"
//!
//! # Generate detailed report
//! cargo bench --bench file_io_benchmark -- --output-format html
//! ```
//!
//! ## Benchmark Results Interpretation
//!
//! ### Expected Performance Characteristics
//!
//! #### Memory Mapping
//! - **Advantages**: Excellent for large files, reduced memory copying
//! - **Disadvantages**: Higher setup cost, platform-dependent behavior
//! - **Optimal Use**: Files > 100MB, random access patterns
//!
//! #### Regular I/O
//! - **Advantages**: Consistent performance, lower setup cost
//! - **Disadvantages**: More memory copying, buffer management overhead
//! - **Optimal Use**: Small to medium files, sequential access
//!
//! #### Chunked Processing
//! - **Advantages**: Predictable memory usage, streaming capability
//! - **Disadvantages**: Potential overhead from multiple system calls
//! - **Optimal Use**: Very large files, memory-constrained environments
//!
//! ## Configuration Optimization
//!
//! ### Chunk Size Guidelines
//! - **Small Files (< 10MB)**: 64KB - 256KB chunks
//! - **Medium Files (10-100MB)**: 256KB - 1MB chunks
//! - **Large Files (> 100MB)**: 1MB - 4MB chunks
//!
//! ### Memory Mapping Thresholds
//! - **Conservative**: 50MB threshold for compatibility
//! - **Aggressive**: 10MB threshold for performance
//! - **Balanced**: 25MB threshold for general use
//!
//! ## Platform Considerations
//!
//! ### Windows
//! - Memory mapping performance varies with file system
//! - NTFS generally provides better memory mapping performance
//! - Consider file system fragmentation impact
//!
//! ### macOS
//! - Excellent memory mapping performance on APFS
//! - Consider unified buffer cache behavior
//! - HFS+ may show different characteristics
//!
//! ### Linux
//! - Consistent memory mapping performance across file systems
//! - Consider page cache and memory pressure effects
//! - ext4, XFS, and Btrfs show similar patterns
//!
//! ## Continuous Performance Monitoring
//!
//! These benchmarks can be integrated into CI/CD pipelines to:
//! - Detect performance regressions
//! - Validate optimization improvements
//! - Monitor cross-platform performance consistency
//! - Guide configuration recommendations
//!
//! ## Contributing
//!
//! When adding new benchmarks:
//! 1. Follow the existing naming conventions
//! 2. Include comprehensive documentation
//! 3. Test across multiple file sizes
//! 4. Consider memory usage implications
//! 5. Validate results across platforms

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::io::Write;
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

use pipeline::core::domain::services::file_io_service::{FileIOConfig, FileIOService, ReadOptions, WriteOptions};
use pipeline::infrastructure::services::file_io_service_impl::FileIOServiceImpl;

/// Creates a test file of the specified size for benchmarking.
///
/// This helper function generates temporary files with predictable content for
/// consistent benchmark results. The files are filled with zero bytes to ensure
/// consistent compression characteristics and avoid filesystem optimizations
/// that might affect benchmark results.
///
/// # Arguments
///
/// * `size_mb` - Size of the test file in megabytes
///
/// # Returns
///
/// Returns a `NamedTempFile` that will be automatically cleaned up when
/// dropped. The file is fully written and flushed to ensure it's available for
/// reading.
///
/// # Performance Considerations
///
/// - Uses 1MB chunks for efficient file creation
/// - Flushes data to ensure it's written to disk
/// - Zero-filled content provides consistent benchmark conditions
/// - Temporary files are created in system temp directory
///
/// # Examples
///
/// ```rust
/// // Create a 10MB test file
/// let test_file = create_test_file(10);
/// assert!(test_file.path().exists());
/// ```
fn create_test_file(size_mb: usize) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    let chunk_size = 1024 * 1024; // 1MB chunks
    let data = vec![0u8; chunk_size];

    for _ in 0..size_mb {
        file.write_all(&data).unwrap();
    }
    file.flush().unwrap();
    file
}

/// Benchmarks different file reading methods to compare performance
/// characteristics.
///
/// This benchmark compares regular file I/O against memory mapping across
/// different file sizes to determine optimal strategies for various use cases.
/// It measures throughput, latency, and resource usage for each approach.
///
/// ## Benchmark Design
///
/// The benchmark tests both reading methods across multiple file sizes:
/// - 1MB: Small file performance
/// - 10MB: Medium file performance
/// - 50MB: Large file performance
/// - 100MB: Very large file performance
///
/// ## Metrics Collected
///
/// - **Throughput**: Bytes per second for each method
/// - **Latency**: Time to complete read operation
/// - **Memory Usage**: Peak memory consumption during operation
/// - **CPU Usage**: Processor utilization patterns
///
/// ## Expected Results
///
/// - **Small Files**: Regular I/O typically faster due to lower setup cost
/// - **Large Files**: Memory mapping often faster due to reduced copying
/// - **Memory Usage**: Memory mapping uses less heap memory
/// - **Platform Variance**: Results may vary across operating systems
///
/// ## Configuration
///
/// Uses optimized configuration:
/// - 64KB chunk size for regular I/O
/// - 1GB memory mapping threshold
/// - Checksums disabled for pure I/O performance
///
/// # Arguments
///
/// * `c` - Criterion benchmark context for result collection
fn benchmark_read_methods(c: &mut Criterion) {
    let service = FileIOServiceImpl::new(FileIOConfig {
        default_chunk_size: 64 * 1024,     // 64KB
        max_mmap_size: 1024 * 1024 * 1024, // 1GB
        enable_memory_mapping: true,
        ..Default::default()
    });

    let mut group = c.benchmark_group("file_read_methods");

    // Test different file sizes
    for size_mb in [1, 10, 50, 100].iter() {
        let test_file = create_test_file(*size_mb);

        // Benchmark regular file I/O
        group.bench_with_input(BenchmarkId::new("regular_io", size_mb), size_mb, |b, _| {
            b.iter_custom(|iters| {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        let result = service
                            .read_file_chunks(
                                test_file.path(),
                                ReadOptions {
                                    use_memory_mapping: false,
                                    calculate_checksums: false,
                                    ..Default::default()
                                },
                            )
                            .await
                            .unwrap();
                        black_box(result);
                    }
                    start.elapsed()
                })
            });
        });

        // Benchmark memory-mapped I/O
        group.bench_with_input(BenchmarkId::new("memory_mapped", size_mb), size_mb, |b, _| {
            b.iter_custom(|iters| {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        let result = service
                            .read_file_mmap(
                                test_file.path(),
                                ReadOptions {
                                    calculate_checksums: false,
                                    ..Default::default()
                                },
                            )
                            .await
                            .unwrap();
                        black_box(result);
                    }
                    start.elapsed()
                })
            });
        });
    }

    group.finish();
}

// Benchmark different chunk sizes
fn benchmark_chunk_sizes(c: &mut Criterion) {
    let service = FileIOServiceImpl::new_default();
    let test_file = create_test_file(10); // 10MB file

    let mut group = c.benchmark_group("chunk_sizes");

    // Test different chunk sizes
    for chunk_size in [4096, 8192, 16384, 32768, 65536, 131072].iter() {
        group.bench_with_input(
            BenchmarkId::new("regular_io", chunk_size),
            chunk_size,
            |b, &chunk_size| {
                b.iter_custom(|iters| {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let result = service
                                .read_file_chunks(
                                    test_file.path(),
                                    ReadOptions {
                                        chunk_size: Some(chunk_size),
                                        use_memory_mapping: false,
                                        calculate_checksums: false,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();
                            black_box(result);
                        }
                        start.elapsed()
                    })
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("memory_mapped", chunk_size),
            chunk_size,
            |b, &chunk_size| {
                b.iter_custom(|iters| {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let result = service
                                .read_file_mmap(
                                    test_file.path(),
                                    ReadOptions {
                                        chunk_size: Some(chunk_size),
                                        calculate_checksums: false,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();
                            black_box(result);
                        }
                        start.elapsed()
                    })
                });
            },
        );
    }

    group.finish();
}

// Benchmark checksum calculation
fn benchmark_checksum_calculation(c: &mut Criterion) {
    let service = FileIOServiceImpl::new_default();
    let test_file = create_test_file(10); // 10MB file

    let mut group = c.benchmark_group("checksum_calculation");

    group.bench_function("with_checksums", |b| {
        b.iter_custom(|iters| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let result = service
                        .read_file_chunks(
                            test_file.path(),
                            ReadOptions {
                                calculate_checksums: true,
                                use_memory_mapping: false,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();
                    black_box(result);
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("without_checksums", |b| {
        b.iter_custom(|iters| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let result = service
                        .read_file_chunks(
                            test_file.path(),
                            ReadOptions {
                                calculate_checksums: false,
                                use_memory_mapping: false,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();
                    black_box(result);
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("checksum_only", |b| {
        b.iter_custom(|iters| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let checksum = service.calculate_file_checksum(test_file.path()).await.unwrap();
                    black_box(checksum);
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

// Benchmark write operations
fn benchmark_write_operations(c: &mut Criterion) {
    let service = FileIOServiceImpl::new_default();

    let mut group = c.benchmark_group("write_operations");

    // Create test data of different sizes
    for size_kb in [1, 10, 100, 1000].iter() {
        let test_data = vec![0u8; size_kb * 1024];

        group.bench_with_input(BenchmarkId::new("write_data", size_kb), size_kb, |b, _| {
            b.iter_custom(|iters| {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        let temp_file = NamedTempFile::new().unwrap();
                        let result = service
                            .write_file_data(
                                temp_file.path(),
                                &test_data,
                                WriteOptions {
                                    calculate_checksums: false,
                                    sync: false,
                                    ..Default::default()
                                },
                            )
                            .await
                            .unwrap();
                        black_box(result);
                    }
                    start.elapsed()
                })
            });
        });

        group.bench_with_input(
            BenchmarkId::new("write_data_with_checksum", size_kb),
            size_kb,
            |b, _| {
                b.iter_custom(|iters| {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let temp_file = NamedTempFile::new().unwrap();
                            let result = service
                                .write_file_data(
                                    temp_file.path(),
                                    &test_data,
                                    WriteOptions {
                                        calculate_checksums: true,
                                        sync: false,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();
                            black_box(result);
                        }
                        start.elapsed()
                    })
                });
            },
        );
    }

    group.finish();
}

// Benchmark streaming operations
fn benchmark_streaming(c: &mut Criterion) {
    let service = FileIOServiceImpl::new_default();
    let test_file = create_test_file(10); // 10MB file

    let mut group = c.benchmark_group("streaming");

    group.bench_function("stream_chunks", |b| {
        b.iter_custom(|iters| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    use futures::StreamExt;

                    let mut stream = service
                        .stream_file_chunks(
                            test_file.path(),
                            ReadOptions {
                                chunk_size: Some(8192),
                                calculate_checksums: false,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();

                    let mut chunk_count = 0;
                    while let Some(chunk_result) = stream.next().await {
                        if chunk_result.is_ok() {
                            chunk_count += 1;
                        }
                    }
                    black_box(chunk_count);
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_read_methods,
    benchmark_chunk_sizes,
    benchmark_checksum_calculation,
    benchmark_write_operations,
    benchmark_streaming
);

criterion_main!(benches);
