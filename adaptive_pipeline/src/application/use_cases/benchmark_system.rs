// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Benchmark System Use Case
//!
//! This module implements comprehensive pipeline performance benchmarking.
//! It tests various chunk sizes, worker counts, and file sizes to identify
//! optimal configurations.
//!
//! ## Overview
//!
//! The Benchmark System use case provides:
//!
//! - **Performance Testing**: Measure throughput and processing time
//! - **Configuration Optimization**: Test different chunk/worker combinations
//! - **Adaptive Validation**: Compare adaptive settings against alternatives
//! - **Report Generation**: Create detailed markdown reports
//! - **Multiple File Sizes**: Test scalability across different file sizes
//!
//! ## Test Matrix
//!
//! - **File Sizes**: 1MB, 5MB, 10MB, 50MB, 100MB, 500MB, 1GB, 2GB
//! - **Chunk Sizes**: 1MB, 2MB, 4MB, 8MB, 16MB, 32MB, 64MB, 128MB
//! - **Worker Counts**: 1 to (2 × CPU cores), max 16
//! - **Iterations**: Configurable (default: 3)

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::{ info, warn };

use crate::infrastructure::metrics::MetricsService;
use adaptive_pipeline_domain::value_objects::chunk_size::ChunkSize;
use adaptive_pipeline_domain::value_objects::worker_count::WorkerCount;

/// Benchmark result for a single configuration.
#[derive(Debug, Clone)]
struct BenchmarkResult {
    file_size_mb: usize,
    chunk_size_mb: usize,
    worker_count: usize,
    avg_throughput_mbps: f64,
    avg_duration_secs: f64,
    config_type: String,
}

/// Single test iteration result.
#[derive(Debug)]
struct TestResult {
    avg_throughput_mbps: f64,
    avg_duration_secs: f64,
}

/// Use case for benchmarking pipeline performance.
///
/// This use case performs comprehensive performance testing across multiple
/// configurations to identify optimal settings for different file sizes.
pub struct BenchmarkSystemUseCase;

impl BenchmarkSystemUseCase {
    /// Creates a new Benchmark System use case.
    pub fn new() -> Self {
        Self
    }

    /// Executes the benchmark system use case.
    ///
    /// Runs comprehensive benchmarks to test pipeline performance across
    /// various configurations, comparing adaptive settings against alternatives.
    ///
    /// ## Parameters
    ///
    /// * `file` - Optional existing file to use (otherwise generates test files)
    /// * `size_mb` - Specific file size to test (0 = test all default sizes)
    /// * `iterations` - Number of iterations per configuration (default: 3)
    ///
    /// ## Test Configurations
    ///
    /// For each file size, tests:
    /// 1. **Adaptive Configuration**: Recommended chunk/worker settings
    /// 2. **Chunk Variations**: Different chunk sizes with adaptive workers
    /// 3. **Worker Variations**: Different worker counts with adaptive chunk size
    ///
    /// ## Output
    ///
    /// Generates `pipeline_optimization_report.md` containing:
    /// - Performance comparison tables
    /// - Adaptive vs best configuration analysis
    /// - Detailed results for all tested configurations
    /// - Summary recommendations for each file size
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Benchmark completed successfully
    /// - `Err(anyhow::Error)` - Benchmark failed
    pub async fn execute(
        &self,
        file: Option<PathBuf>,
        size_mb: usize,
        iterations: usize
    ) -> Result<()> {
        info!("Running comprehensive pipeline optimization benchmark");
        info!("Test size: {}MB", size_mb);
        info!("Iterations: {}", iterations);

        // Create metrics service for benchmarking
        let metrics_service = Arc::new(MetricsService::new()?);

        // Test file sizes in MB
        let test_sizes = if size_mb > 0 {
            vec![size_mb]
        } else {
            vec![1, 5, 10, 50, 100, 500, 1000, 2048] // Default sizes up to 2GB
        };

        // Chunk sizes to test (in MB)
        let chunk_sizes = vec![1, 2, 4, 8, 16, 32, 64, 128];

        // Worker counts to test
        let available_cores = std::thread
            ::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let max_workers = (available_cores * 2).min(16);
        let worker_counts: Vec<usize> = (1..=max_workers).collect();

        println!(
            "\n========================================================================================================================"
        );
        println!(
            "========================================== PIPELINE OPTIMIZATION BENCHMARK \
             ==========================================="
        );
        println!(
            "========================================================================================================================"
        );
        println!("System Info:        {} CPU cores available", available_cores);
        println!("Test Iterations:    {}", iterations);
        println!("File Sizes:         {:?} MB", test_sizes);
        println!("Chunk Sizes:        {:?} MB", chunk_sizes);
        println!("Worker Counts:      {:?}", worker_counts);
        println!(
            "========================================================================================================================"
        );

        let mut results = Vec::new();

        for &test_size_mb in &test_sizes {
            println!("\n🔍 Testing file size: {} MB", test_size_mb);

            // Create or use test file
            let test_file = if let Some(ref provided_file) = file {
                provided_file.clone()
            } else {
                let test_file = PathBuf::from(format!("benchmark_test_{}mb.txt", test_size_mb));
                Self::generate_test_file(&test_file, test_size_mb).await?;
                test_file
            };

            // Get adaptive recommendations
            let file_size_bytes = (test_size_mb * 1024 * 1024) as u64;
            let adaptive_chunk = ChunkSize::optimal_for_file_size(file_size_bytes);
            let adaptive_workers = WorkerCount::optimal_for_file_size(file_size_bytes);

            println!(
                "   Adaptive recommendations: {} chunk, {} workers",
                adaptive_chunk.megabytes(),
                adaptive_workers.count()
            );

            // Test adaptive configuration first
            println!("   Testing adaptive configuration...");
            let adaptive_chunk_mb = ((adaptive_chunk.bytes() as f64) / (1024.0 * 1024.0)).max(
                1.0
            ) as usize;
            let adaptive_result = Self::run_benchmark_test(
                &test_file,
                test_size_mb,
                Some(adaptive_chunk_mb),
                Some(adaptive_workers.count()),
                iterations,
                &metrics_service
            ).await?;

            results.push(BenchmarkResult {
                file_size_mb: test_size_mb,
                chunk_size_mb: adaptive_chunk_mb,
                worker_count: adaptive_workers.count(),
                avg_throughput_mbps: adaptive_result.avg_throughput_mbps,
                avg_duration_secs: adaptive_result.avg_duration_secs,
                config_type: "Adaptive".to_string(),
            });

            // Test variations around adaptive values
            println!("   Testing variations around adaptive values...");

            // Test different chunk sizes with adaptive worker count
            for &chunk_mb in &chunk_sizes {
                if chunk_mb == (adaptive_chunk.megabytes() as usize) {
                    continue; // Skip adaptive (already tested)
                }

                let result = Self::run_benchmark_test(
                    &test_file,
                    test_size_mb,
                    Some(chunk_mb),
                    Some(adaptive_workers.count()),
                    iterations,
                    &metrics_service
                ).await?;

                results.push(BenchmarkResult {
                    file_size_mb: test_size_mb,
                    chunk_size_mb: chunk_mb,
                    worker_count: adaptive_workers.count(),
                    avg_throughput_mbps: result.avg_throughput_mbps,
                    avg_duration_secs: result.avg_duration_secs,
                    config_type: "Chunk Variation".to_string(),
                });
            }

            // Test different worker counts with adaptive chunk size
            for &workers in &worker_counts {
                if workers == adaptive_workers.count() {
                    continue; // Skip adaptive (already tested)
                }

                let result = Self::run_benchmark_test(
                    &test_file,
                    test_size_mb,
                    Some(adaptive_chunk_mb),
                    Some(workers),
                    iterations,
                    &metrics_service
                ).await?;

                results.push(BenchmarkResult {
                    file_size_mb: test_size_mb,
                    chunk_size_mb: adaptive_chunk_mb,
                    worker_count: workers,
                    avg_throughput_mbps: result.avg_throughput_mbps,
                    avg_duration_secs: result.avg_duration_secs,
                    config_type: "Worker Variation".to_string(),
                });
            }

            // Clean up generated test file
            if file.is_none() && test_file.exists() {
                std::fs::remove_file(&test_file)?;
            }
        }

        // Generate comprehensive report
        Self::generate_optimization_report(&results).await?;

        println!("\n✅ Benchmark completed successfully!");
        println!("📊 Check the generated optimization report for detailed results.");

        Ok(())
    }

    /// Simulates pipeline processing for benchmarking.
    async fn simulate_pipeline_processing(
        input_file: &PathBuf,
        output_file: &PathBuf,
        chunk_size_mb: usize,
        worker_count: usize
    ) -> Result<()> {
        use std::io::{ Read, Write };
        use tokio::task;

        let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
        let mut input = std::fs::File::open(input_file)?;
        let mut output = std::fs::File::create(output_file)?;

        // Read file in chunks
        let mut buffer = vec![0u8; chunk_size_bytes];
        let mut chunks = Vec::new();

        loop {
            let bytes_read = input.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            chunks.push(buffer[..bytes_read].to_vec());
        }

        // Process chunks with simulated concurrency
        let chunk_count = chunks.len();
        let chunks_per_worker = chunk_count.div_ceil(worker_count);

        let mut handles = Vec::new();
        for worker_id in 0..worker_count {
            let start_idx = worker_id * chunks_per_worker;
            let end_idx = ((worker_id + 1) * chunks_per_worker).min(chunk_count);

            if start_idx < chunk_count {
                let worker_chunks = chunks[start_idx..end_idx].to_vec();
                let handle = task::spawn(async move {
                    // Simulate processing work
                    for chunk in &worker_chunks {
                        // Simple processing simulation: XOR each byte
                        let _processed: Vec<u8> = chunk
                            .iter()
                            .map(|&b| b ^ 0x42)
                            .collect();
                        // Small delay to simulate work
                        tokio::time::sleep(std::time::Duration::from_micros(1)).await;
                    }
                    worker_chunks
                });
                handles.push(handle);
            }
        }

        // Collect results and write to output
        for handle in handles {
            let processed_chunks = handle.await.unwrap();
            for chunk in processed_chunks {
                output.write_all(&chunk)?;
            }
        }

        output.flush()?;
        Ok(())
    }

    /// Generates a test file of specified size.
    async fn generate_test_file(path: &PathBuf, size_mb: usize) -> Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::create(path)?;
        let chunk_size = 1024 * 1024; // 1MB chunks
        let data = vec![b'A'; chunk_size];

        for _ in 0..size_mb {
            file.write_all(&data)?;
        }

        file.flush()?;
        Ok(())
    }

    /// Runs a single benchmark test configuration.
    async fn run_benchmark_test(
        test_file: &PathBuf,
        _file_size_mb: usize,
        chunk_size_mb: Option<usize>,
        worker_count: Option<usize>,
        iterations: usize,
        _metrics_service: &Arc<MetricsService>
    ) -> Result<TestResult> {
        let mut durations = Vec::new();
        let mut throughputs = Vec::new();

        for i in 0..iterations {
            let output_file = PathBuf::from(
                format!("benchmark_output_{}_{}.adapipe", std::process::id(), i)
            );

            let start_time = Instant::now();

            let result = Self::simulate_pipeline_processing(
                test_file,
                &output_file,
                chunk_size_mb.unwrap_or(1),
                worker_count.unwrap_or(1)
            ).await;

            let duration = start_time.elapsed();

            // Clean up output file
            if output_file.exists() {
                std::fs::remove_file(&output_file)?;
            }

            match result {
                Ok(_) => {
                    let duration_secs = duration.as_secs_f64();
                    let file_size_bytes = std::fs::metadata(test_file)?.len();
                    let throughput_mbps =
                        (file_size_bytes as f64) / (1024.0 * 1024.0) / duration_secs;

                    durations.push(duration_secs);
                    throughputs.push(throughput_mbps);
                }
                Err(e) => {
                    warn!("Benchmark iteration {} failed: {}", i, e);
                    durations.push(999.0);
                    throughputs.push(0.0);
                }
            }
        }

        let avg_duration = durations.iter().sum::<f64>() / (durations.len() as f64);
        let avg_throughput = throughputs.iter().sum::<f64>() / (throughputs.len() as f64);

        Ok(TestResult {
            avg_throughput_mbps: avg_throughput,
            avg_duration_secs: avg_duration,
        })
    }

    /// Generates comprehensive optimization report in markdown format.
    async fn generate_optimization_report(results: &[BenchmarkResult]) -> Result<()> {
        let report_file = PathBuf::from("pipeline_optimization_report.md");
        let mut report = String::new();

        report.push_str("# Pipeline Optimization Benchmark Report\n\n");
        report.push_str(
            &format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
        );

        // Group results by file size
        let mut file_sizes: Vec<usize> = results
            .iter()
            .map(|r| r.file_size_mb)
            .collect();
        file_sizes.sort_unstable();
        file_sizes.dedup();

        for file_size in &file_sizes {
            report.push_str(&format!("## File Size: {} MB\n\n", file_size));

            let size_results: Vec<_> = results
                .iter()
                .filter(|r| r.file_size_mb == *file_size)
                .collect();

            // Find best configuration
            let best_result = size_results
                .iter()
                .max_by(|a, b| {
                    a.avg_throughput_mbps
                        .partial_cmp(&b.avg_throughput_mbps)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .ok_or_else(|| anyhow::anyhow!("No benchmark results found"))?;

            let adaptive_result = size_results
                .iter()
                .find(|r| r.config_type == "Adaptive")
                .ok_or_else(|| anyhow::anyhow!("No adaptive results found"))?;

            report.push_str("**Adaptive Configuration:**\n");
            report.push_str(&format!("- Chunk Size: {} MB\n", adaptive_result.chunk_size_mb));
            report.push_str(&format!("- Worker Count: {}\n", adaptive_result.worker_count));
            report.push_str(
                &format!("- Throughput: {:.2} MB/s\n", adaptive_result.avg_throughput_mbps)
            );
            report.push_str(
                &format!("- Duration: {:.2} seconds\n\n", adaptive_result.avg_duration_secs)
            );

            report.push_str("**Best Configuration:**\n");
            report.push_str(&format!("- Chunk Size: {} MB\n", best_result.chunk_size_mb));
            report.push_str(&format!("- Worker Count: {}\n", best_result.worker_count));
            report.push_str(
                &format!("- Throughput: {:.2} MB/s\n", best_result.avg_throughput_mbps)
            );
            report.push_str(&format!("- Duration: {:.2} seconds\n", best_result.avg_duration_secs));
            report.push_str(&format!("- Configuration Type: {}\n\n", best_result.config_type));

            let improvement =
                ((best_result.avg_throughput_mbps - adaptive_result.avg_throughput_mbps) /
                    adaptive_result.avg_throughput_mbps) *
                100.0;

            if improvement > 0.0 {
                report.push_str(
                    &format!(
                        "**Performance Improvement:** {:.1}% faster than adaptive\n\n",
                        improvement
                    )
                );
            } else {
                report.push_str("**Performance:** Adaptive configuration is optimal\n\n");
            }

            // Detailed results table
            report.push_str("### Detailed Results\n\n");
            report.push_str(
                "| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |\n"
            );
            report.push_str(
                "|-----------------|---------|-------------------|--------------|-------------|\n"
            );

            let mut sorted_results = size_results.clone();
            sorted_results.sort_by(|a, b| {
                b.avg_throughput_mbps
                    .partial_cmp(&a.avg_throughput_mbps)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for result in sorted_results {
                report.push_str(
                    &format!(
                        "| {} | {} | {:.2} | {:.2} | {} |\n",
                        result.chunk_size_mb,
                        result.worker_count,
                        result.avg_throughput_mbps,
                        result.avg_duration_secs,
                        result.config_type
                    )
                );
            }

            report.push('\n');
        }

        // Summary recommendations
        report.push_str("## Summary Recommendations\n\n");

        for file_size in &file_sizes {
            let size_results: Vec<_> = results
                .iter()
                .filter(|r| r.file_size_mb == *file_size)
                .collect();

            let best_result = size_results
                .iter()
                .max_by(|a, b| {
                    a.avg_throughput_mbps
                        .partial_cmp(&b.avg_throughput_mbps)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .ok_or_else(|| anyhow::anyhow!("No benchmark results found"))?;

            report.push_str(
                &format!(
                    "- **{} MB files**: {} MB chunks, {} workers ({:.2} MB/s)\n",
                    file_size,
                    best_result.chunk_size_mb,
                    best_result.worker_count,
                    best_result.avg_throughput_mbps
                )
            );
        }

        // Write report to file
        std::fs::write(&report_file, report)?;

        println!("\n📊 Optimization report generated: {}", report_file.display());

        Ok(())
    }
}

impl Default for BenchmarkSystemUseCase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Expensive benchmark test
    async fn test_benchmark_small_file() {
        let use_case = BenchmarkSystemUseCase::new();
        let result = use_case.execute(None, 1, 1).await; // 1MB, 1 iteration
        assert!(result.is_ok());
    }
}
