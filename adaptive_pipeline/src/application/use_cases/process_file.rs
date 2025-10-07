// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Process File Use Case
//!
//! This module implements the core file processing use case. It orchestrates
//! the complete workflow of processing files through configured pipelines with
//! compression, encryption, and transformation stages.
//!
//! ## Overview
//!
//! The Process File use case provides:
//!
//! - **Pipeline Processing**: Execute files through multi-stage pipelines
//! - **Adaptive Configuration**: Intelligent chunk/worker optimization
//! - **Binary Format Output**: Generate .adapipe binary format files
//! - **Metrics Collection**: Comprehensive performance monitoring
//! - **Error Handling**: Robust error reporting and recovery
//! - **Progress Tracking**: Real-time processing status
//!
//! ## Processing Pipeline
//!
//! 1. **Configuration**: Load pipeline, validate settings
//! 2. **Adaptive Sizing**: Determine optimal chunk size and worker count
//! 3. **Stage Execution**: Process through compression, encryption, transforms
//! 4. **Metrics Tracking**: Monitor performance at each stage
//! 5. **Output Generation**: Write .adapipe binary format
//! 6. **Integrity Verification**: Calculate and verify checksums
//!
//! ## Binary Format
//!
//! Output files use the `.adapipe` format containing:
//! - Original file metadata
//! - Processing pipeline information
//! - Chunk boundaries and checksums
//! - Compression/encryption metadata
//!
//! ## Performance
//!
//! - **Adaptive Chunk Sizing**: Automatic optimization based on file size
//! - **Parallel Processing**: Multi-worker concurrent chunk processing
//! - **Streaming I/O**: Memory-efficient chunk-based processing
//! - **Resource Management**: CPU and I/O token management

use anyhow::Result;
use byte_unit::Byte;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error, warn};

use crate::application::services::pipeline::ConcurrentPipeline;
use crate::infrastructure::adapters::file_io::TokioFileIO;
use crate::infrastructure::repositories::sqlite_pipeline::SqlitePipelineRepository;
use crate::infrastructure::adapters::{MultiAlgoCompression, MultiAlgoEncryption};
use crate::infrastructure::logging::ObservabilityService;
use crate::infrastructure::metrics::MetricsService;
use crate::infrastructure::runtime::stage_executor::BasicStageExecutor;
use crate::infrastructure::services::{
    Base64EncodingService, AdapipeFormat, DebugService, PassThroughService,
    PiiMaskingService, TeeService,
};
use adaptive_pipeline_domain::entities::security_context::{Permission, SecurityContext, SecurityLevel};
use adaptive_pipeline_domain::services::PipelineService;
use adaptive_pipeline_domain::value_objects::chunk_size::ChunkSize;
use adaptive_pipeline_domain::value_objects::worker_count::WorkerCount;

/// Configuration for file processing operations.
#[derive(Debug, Clone)]
pub struct ProcessFileConfig {
    pub input: PathBuf,
    pub output: PathBuf,
    pub pipeline: String,
    pub chunk_size_mb: Option<usize>,
    pub workers: Option<usize>,
    pub channel_depth: Option<usize>,
}

/// Use case for processing files through pipelines.
///
/// This is the core use case that orchestrates the entire file processing
/// workflow, from loading the pipeline configuration through executing all
/// stages and generating the output .adapipe file.
pub struct ProcessFileUseCase {
    metrics_service: Arc<MetricsService>,
    observability_service: Arc<ObservabilityService>,
    pipeline_repository: Arc<SqlitePipelineRepository>,
}

impl ProcessFileUseCase {
    /// Creates a new Process File use case.
    ///
    /// # Parameters
    ///
    /// * `metrics_service` - Metrics collection service
    /// * `observability_service` - Observability and health monitoring
    /// * `pipeline_repository` - Repository for pipeline data access
    pub fn new(
        metrics_service: Arc<MetricsService>,
        observability_service: Arc<ObservabilityService>,
        pipeline_repository: Arc<SqlitePipelineRepository>,
    ) -> Self {
        Self {
            metrics_service,
            observability_service,
            pipeline_repository,
        }
    }

    /// Executes the process file use case.
    ///
    /// Processes an input file through a configured pipeline, generating an
    /// output `.adapipe` binary format file with comprehensive metadata.
    ///
    /// ## Parameters
    ///
    /// * `config` - Processing configuration (input, output, pipeline, options)
    ///
    /// ## Processing Steps
    ///
    /// 1. **Load Pipeline**: Retrieve pipeline configuration from repository
    /// 2. **Determine Chunk Size**: Adaptive or user-specified chunk size
    /// 3. **Determine Worker Count**: Optimal parallelism for file size
    /// 4. **Create Services**: Instantiate compression, encryption, I/O services
    /// 5. **Execute Pipeline**: Process file through all configured stages
    /// 6. **Collect Metrics**: Track performance at each stage
    /// 7. **Generate Output**: Write .adapipe binary format with metadata
    /// 8. **Report Results**: Display comprehensive processing summary
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - File processed successfully
    /// - `Err(anyhow::Error)` - Processing failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Input file not found or unreadable
    /// - Pipeline not found in repository
    /// - Processing stage failures
    /// - Output file write errors
    /// - Insufficient permissions
    pub async fn execute(&self, config: ProcessFileConfig) -> Result<()> {
        let ProcessFileConfig {
            input,
            output,
            pipeline,
            chunk_size_mb,
            workers,
            channel_depth,
        } = config;

        // Ensure output file has .adapipe extension
        let output = if output.extension().is_none_or(|ext| ext != "adapipe") {
            output.with_extension("adapipe")
        } else {
            output
        };

        debug!(
            "Processing file: {} -> {} (.adapipe format)",
            input.display(),
            output.display()
        );
        debug!("Pipeline: {}", pipeline);

        // Get file size for processing metrics
        let actual_input_size = fs::metadata(&input)?.len();
        debug!(
            "Input file size: {} bytes ({})",
            actual_input_size,
            Byte::from_u128(actual_input_size as u128)
                .unwrap_or_default()
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string()
        );

        // Determine chunk size: user override with validation or adaptive
        let (actual_chunk_size_bytes, chunk_size_source) =
            Self::determine_chunk_size(actual_input_size, chunk_size_mb);

        debug!(
            "Final chunk size: {} bytes ({}) - {}",
            actual_chunk_size_bytes,
            Byte::from_u128(actual_chunk_size_bytes as u128)
                .unwrap_or_default()
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string(),
            chunk_size_source
        );

        if let Some(worker_count) = workers {
            debug!("Using {} workers", worker_count);
        }

        // Create security context
        let security_context = SecurityContext::with_permissions(
            None,
            vec![
                Permission::Read,
                Permission::Write,
                Permission::Compress,
                Permission::Encrypt,
            ],
            SecurityLevel::Internal,
        );

        debug!("Security context: {:?}", security_context.security_level());

        // Load pipeline from repository
        debug!("Loading pipeline configuration...");
        let pipeline_entity = self
            .pipeline_repository
            .find_by_name(&pipeline)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
            .ok_or_else(|| anyhow::anyhow!("Pipeline '{}' not found", pipeline))?;

        debug!(
            "Loaded pipeline '{}' with {} stages",
            pipeline_entity.name(),
            pipeline_entity.stages().len()
        );
        for stage in pipeline_entity.stages() {
            debug!("  - Stage: {} (type: {:?})", stage.name(), stage.stage_type());
        }

        // Create and configure pipeline service
        let pipeline_service = Self::create_pipeline_service(
            &self.metrics_service,
            &self.pipeline_repository,
        );

        // Track active pipeline processing
        self.metrics_service.increment_active_pipelines();
        let operation_tracker = self.observability_service.start_operation("file_processing").await;

        let processing_start = Instant::now();

        // Create metrics observer
        let metrics_observer = Arc::new(crate::infrastructure::metrics::MetricsObserver::new(
            self.metrics_service.clone(),
        ));

        // Process the file through the pipeline
        let processing_result = pipeline_service
            .process_file(
                pipeline_entity.id().clone(),
                input.as_path(),
                output.as_path(),
                security_context,
                workers,
                channel_depth,
                Some(metrics_observer),
            )
            .await;

        let total_processing_duration = processing_start.elapsed();

        // Always decrement active pipelines
        self.metrics_service.decrement_active_pipelines();

        match processing_result {
            Ok(mut metrics) => {
                debug!("File processing completed successfully");

                // Calculate compression ratio
                let compression_ratio = if actual_input_size > 0 {
                    metrics.output_file_size_bytes() as f64 / actual_input_size as f64
                } else {
                    0.0
                };
                if compression_ratio > 0.0 {
                    metrics.set_compression_ratio(compression_ratio);
                }

                self.observability_service
                    .record_processing_metrics(&metrics)
                    .await;
                operation_tracker.complete_with_metrics(&metrics).await;

                // Display processing summary
                Self::display_processing_summary(
                    &input,
                    &output,
                    actual_input_size,
                    actual_chunk_size_bytes,
                    total_processing_duration,
                    &metrics,
                    &pipeline_entity,
                    chunk_size_source,
                    workers,
                );

                Ok(())
            }
            Err(e) => {
                Self::display_processing_error(&input, &output, &e);
                error!("File processing failed: {}", e);
                Err(anyhow::anyhow!("File processing failed: {}", e))
            }
        }
    }

    /// Determines optimal chunk size for file processing.
    fn determine_chunk_size(
        file_size: u64,
        user_chunk_mb: Option<usize>,
    ) -> (usize, &'static str) {
        let optimal_chunk_size = ChunkSize::optimal_for_file_size(file_size);

        if let Some(user_chunk_mb) = user_chunk_mb {
            match ChunkSize::validate_user_input(user_chunk_mb, file_size) {
                Ok(validated_bytes) => {
                    if validated_bytes == optimal_chunk_size.bytes() {
                        debug!(
                            "User-specified chunk size {} MB matches adaptive choice",
                            user_chunk_mb
                        );
                        (validated_bytes, "adaptive")
                    } else {
                        debug!(
                            "Using user-specified chunk size: {} MB ({} bytes)",
                            user_chunk_mb, validated_bytes
                        );
                        (validated_bytes, "user-override")
                    }
                }
                Err(warning) => {
                    warn!(
                        "User chunk size invalid: {}. Using adaptive: {} bytes",
                        warning,
                        optimal_chunk_size.bytes()
                    );
                    (optimal_chunk_size.bytes(), "adaptive-fallback")
                }
            }
        } else {
            debug!(
                "Using adaptive chunk size: {} bytes",
                optimal_chunk_size.bytes()
            );
            (optimal_chunk_size.bytes(), "adaptive")
        }
    }

    /// Creates and configures the pipeline service with all required dependencies.
    fn create_pipeline_service(
        metrics_service: &Arc<MetricsService>,
        pipeline_repository: &Arc<SqlitePipelineRepository>,
    ) -> ConcurrentPipeline {
        // Create services
        let compression_service = Arc::new(MultiAlgoCompression::new());
        let encryption_service = Arc::new(MultiAlgoEncryption::new());
        let file_io_service = Arc::new(TokioFileIO::new(Default::default()));
        let binary_format_service = Arc::new(AdapipeFormat::new());

        // Build stage service registry
        let mut stage_services: HashMap<
            String,
            Arc<dyn adaptive_pipeline_domain::services::StageService>,
        > = HashMap::new();

        // Register compression algorithms
        stage_services.insert(
            "brotli".to_string(),
            compression_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "gzip".to_string(),
            compression_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "zstd".to_string(),
            compression_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "lz4".to_string(),
            compression_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );

        // Register encryption algorithms
        stage_services.insert(
            "aes256gcm".to_string(),
            encryption_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "aes128gcm".to_string(),
            encryption_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "chacha20poly1305".to_string(),
            encryption_service.clone() as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );

        // Register transform stages
        stage_services.insert(
            "base64".to_string(),
            Arc::new(Base64EncodingService::new()) as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "pii_masking".to_string(),
            Arc::new(PiiMaskingService::new()) as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "tee".to_string(),
            Arc::new(TeeService::new()) as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "passthrough".to_string(),
            Arc::new(PassThroughService::new()) as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );
        stage_services.insert(
            "debug".to_string(),
            Arc::new(DebugService::new(metrics_service.clone()))
                as Arc<dyn adaptive_pipeline_domain::services::StageService>,
        );

        ConcurrentPipeline::new(
            compression_service,
            encryption_service,
            file_io_service,
            pipeline_repository.clone(),
            Arc::new(BasicStageExecutor::new(stage_services)),
            binary_format_service,
        )
    }

    /// Displays comprehensive processing summary with metrics and stage details.
    #[allow(clippy::too_many_arguments)]
    fn display_processing_summary(
        input: &Path,
        output: &Path,
        actual_input_size: u64,
        actual_chunk_size_bytes: usize,
        total_processing_duration: std::time::Duration,
        metrics: &adaptive_pipeline_domain::entities::ProcessingMetrics,
        pipeline: &adaptive_pipeline_domain::entities::Pipeline,
        chunk_size_source: &str,
        workers: Option<usize>,
    ) {
        println!();

        let processing_seconds = total_processing_duration.as_secs_f64();
        let input_size_mb = actual_input_size as f64 / (1024.0 * 1024.0);
        let output_size_mb = metrics.output_file_size_bytes() as f64 / (1024.0 * 1024.0);
        let actual_throughput = if processing_seconds > 0.0 {
            input_size_mb / processing_seconds
        } else {
            0.0
        };

        let compression_info = if metrics.output_file_size_bytes() < actual_input_size {
            let reduction_percent =
                (1.0 - (metrics.output_file_size_bytes() as f64 / actual_input_size as f64)) * 100.0;
            format!(" ({:.1} MB, {:.1}% reduction)", output_size_mb, reduction_percent)
        } else if metrics.output_file_size_bytes() > actual_input_size {
            let expansion_percent =
                ((metrics.output_file_size_bytes() as f64 / actual_input_size as f64) - 1.0) * 100.0;
            format!(" ({:.1} MB, {:.1}% expansion)", output_size_mb, expansion_percent)
        } else {
            format!(" ({:.1} MB, unchanged)", output_size_mb)
        };

        println!("🎯 PROCESSING SUMMARY");

        // Create formatted box
        let status_text = format!(
            "STATUS: Processed {:.1} MB in {:.2}s -> {:.1} MB/s throughput",
            input_size_mb, processing_seconds, actual_throughput
        );

        let input_path = input.display().to_string();
        let input_text = if input_path.len() > 100 {
            format!("Input:   \"...{}\"", &input_path[input_path.len() - 95..])
        } else {
            format!("Input:   \"{}\"", input_path)
        };

        let output_path = output.display().to_string();
        let output_with_compression = format!("{}{}", output_path, compression_info);
        let output_text = if output_with_compression.len() > 100 {
            let base_output = if output_path.len() > 80 {
                format!("...{}", &output_path[output_path.len() - 75..])
            } else {
                output_path
            };
            format!("Output:  \"{}\"{}", base_output, compression_info)
        } else {
            format!("Output:  \"{}\"", output_with_compression)
        };

        let max_content_width = [status_text.len(), input_text.len(), output_text.len()]
            .iter()
            .max()
            .unwrap_or(&0)
            + 2;
        let box_width = max_content_width + 2;

        let horizontal_line = "─".repeat(box_width - 2);
        println!("┌{}┐", horizontal_line);
        println!("│ {:<width$} │", status_text, width = max_content_width - 2);
        println!("│ {:<width$} │", input_text, width = max_content_width - 2);
        println!("│ {:<width$} │", output_text, width = max_content_width - 2);
        println!("└{}┘", horizontal_line);
        println!();

        // Performance metrics
        let total_chunks = actual_input_size.div_ceil(actual_chunk_size_bytes as u64);
        let chunk_size_mb = actual_chunk_size_bytes as f64 / (1024.0 * 1024.0);

        println!("⚡ PERFORMANCE METRICS");
        println!("├─ Processing Time:   {:.3} seconds", processing_seconds);
        println!("├─ Throughput:        {:.1} MB/s", actual_throughput);
        println!(
            "├─ Total Chunks:      {} ({:.1} MB each)",
            total_chunks, chunk_size_mb
        );
        println!("└─ Errors:            {}", metrics.error_count());
        println!();

        // Adaptive configuration
        let available_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let optimal_workers = WorkerCount::optimal_for_file_size(actual_input_size);

        let (chunk_strategy, chunk_label) = match chunk_size_source {
            "user-override" => ("User-specified".to_string(), "user override".to_string()),
            "adaptive-fallback" => (
                format!(
                    "{} (fallback)",
                    ChunkSize::strategy_description(actual_input_size)
                ),
                "adaptive fallback".to_string(),
            ),
            _ => (
                ChunkSize::strategy_description(actual_input_size).to_string(),
                "adaptive".to_string(),
            ),
        };

        let (worker_strategy, worker_label, worker_count) = if let Some(user_workers) = workers {
            match WorkerCount::validate_user_input(user_workers, available_cores, actual_input_size)
            {
                Ok(_) => {
                    if user_workers == optimal_workers.count() {
                        (
                            WorkerCount::strategy_description(actual_input_size).to_string(),
                            "adaptive".to_string(),
                            optimal_workers.count(),
                        )
                    } else {
                        (
                            "User-specified".to_string(),
                            "user override".to_string(),
                            user_workers,
                        )
                    }
                }
                Err(_) => (
                    format!(
                        "{} (fallback)",
                        WorkerCount::strategy_description(actual_input_size)
                    ),
                    "adaptive fallback".to_string(),
                    optimal_workers.count(),
                ),
            }
        } else {
            (
                WorkerCount::strategy_description(actual_input_size).to_string(),
                "adaptive".to_string(),
                optimal_workers.count(),
            )
        };

        println!("🔧 ADAPTIVE CONFIGURATION");
        println!(
            "├─ Chunk Strategy:    {} → {:.1} MB ({})",
            chunk_strategy, chunk_size_mb, chunk_label
        );
        println!(
            "├─ Worker Strategy:   {} → {} workers ({}, {} cores available)",
            worker_strategy, worker_count, worker_label, available_cores
        );

        // Pipeline stages
        let stage_names: Vec<String> = pipeline
            .stages()
            .iter()
            .map(|stage| stage.name().to_string())
            .collect();

        if !stage_names.is_empty() {
            let stage_metrics_map = metrics.stage_metrics();

            if !stage_metrics_map.is_empty() {
                println!("└─ Pipeline Stages:   {}", stage_names.join(" → "));
                println!();
                println!("🔬 STAGE EXECUTION DETAILS");

                for (i, stage_name) in stage_names.iter().enumerate() {
                    let stage_num = i + 1;
                    let prefix = if i == stage_names.len() - 1 {
                        "└─"
                    } else {
                        "├─"
                    };

                    if let Some(stage_metrics) = stage_metrics_map.get(stage_name) {
                        let stage_time_ms = stage_metrics.processing_time.as_millis();
                        let stage_throughput_mb =
                            stage_metrics.throughput / (1024.0 * 1024.0);
                        let stage_mb_processed =
                            stage_metrics.bytes_processed as f64 / (1024.0 * 1024.0);
                        let status_icon = if stage_metrics.error_count == 0 {
                            "✅"
                        } else {
                            "❌"
                        };

                        println!(
                            "{} Stage {}: {} {} ({:.2} MB in {}ms → {:.1} MB/s)",
                            prefix,
                            stage_num,
                            stage_name.to_uppercase(),
                            status_icon,
                            stage_mb_processed,
                            stage_time_ms,
                            stage_throughput_mb
                        );

                        if stage_metrics.error_count > 0 {
                            println!(
                                "   │  └─ Errors: {}, Success Rate: {:.1}%",
                                stage_metrics.error_count,
                                stage_metrics.success_rate * 100.0
                            );
                        }
                    } else {
                        println!(
                            "{} Stage {}: {} ✅ (completed)",
                            prefix,
                            stage_num,
                            stage_name.to_uppercase()
                        );
                    }
                }
            } else {
                println!(
                    "└─ Pipeline Stages:   {} (all completed ✅)",
                    stage_names.join(" → ")
                );
            }
        } else {
            println!("└─ Pipeline Stages:   None");
        }
        println!();

        // File integrity
        println!("🔐 FILE INTEGRITY");
        match metrics.input_file_checksum() {
            Some(checksum) => {
                println!("├─ Input SHA256:      {} ✓", checksum);
            }
            None => println!("├─ Input SHA256:      Not Available"),
        }
        match metrics.output_file_checksum() {
            Some(checksum) => {
                println!("└─ Output SHA256:     {} ✓", checksum);
            }
            None => println!("└─ Output SHA256:     Not Available"),
        }
    }

    /// Displays processing error with clear formatting.
    fn display_processing_error(input: &Path, output: &Path, error: &impl std::fmt::Display) {
        println!();
        println!("========================================================================================================================");
        println!(
            "========================================== ADAPTIVE PIPELINE PROCESSING FAILED \
             ==========================================="
        );
        println!("========================================================================================================================");
        println!();
        println!("📁 INPUT FILE:      \"{}\"", input.display());
        println!("📦 OUTPUT FILE:     \"{}\"", output.display());
        println!("========================================================================================================================");
        println!();
        println!("Error:              {}", error);
        println!();
        println!("Final Status:       Failed");
        println!("========================================================================================================================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires full infrastructure setup
    async fn test_process_file_with_real_pipeline() {
        // This test requires real database, metrics service, etc.
        // See tests/integration/ for full end-to-end tests
    }
}
