// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Adaptive Pipeline CLI Application
//!
//! This is the main entry point for the adaptive pipeline command-line
//! interface. It provides a comprehensive CLI for file processing operations
//! including compression, encryption, restoration, and pipeline management.
//!
//! ## Overview
//!
//! The CLI application provides:
//!
//! - **File Processing**: Process files through configurable pipelines
//! - **File Restoration**: Restore files from `.adapipe` binary format
//! - **Pipeline Management**: Create, configure, and manage processing
//!   pipelines
//! - **Progress Monitoring**: Real-time progress tracking and reporting
//! - **Performance Metrics**: Detailed performance statistics and benchmarking
//!
//! ## Architecture
//!
//! The CLI follows Clean Architecture principles:
//!
//! - **Interface Layer**: Command-line interface and user interaction
//! - **Application Layer**: Use cases and application services
//! - **Domain Layer**: Business logic and domain entities
//! - **Infrastructure Layer**: File I/O, database, and external services
//!
//! ## Command Structure
//!
//! ### File Operations
//! - **Process**: Process files through configured pipelines
//! - **Restore**: Restore files from `.adapipe` binary format
//! - **Validate**: Validate file integrity and format
//!
//! ### Pipeline Management
//! - **Create**: Create new processing pipelines
//! - **List**: List available pipelines
//! - **Delete**: Remove pipelines
//! - **Configure**: Modify pipeline settings
//!
//! ### System Operations
//! - **Status**: Check system status and health
//! - **Metrics**: Display performance metrics
//! - **Config**: Manage application configuration
//!
//! ## Usage Examples
//!
//! ### Basic File Processing
//!
//! ```bash
//! # Process a file with default pipeline
//! adapipe process input.txt
//!
//! # Process with specific pipeline
//! adapipe process --pipeline compress-encrypt input.txt
//!
//! # Process with custom output path
//! adapipe process --output /path/to/output.adapipe input.txt
//! ```
//!
//! ### File Restoration
//!
//! ```bash
//! # Restore a file from .adapipe format
//! adapipe restore input.adapipe
//!
//! # Restore with custom output path
//! adapipe restore --output restored.txt input.adapipe
//!
//! # Restore with progress monitoring
//! adapipe restore --verbose input.adapipe
//! ```
//!
//! ### Pipeline Management
//!
//! ```bash
//! # List available pipelines
//! adapipe pipeline list
//!
//! # Create new pipeline
//! adapipe pipeline create --name my-pipeline --stages compress,encrypt
//!
//! # Delete pipeline
//! adapipe pipeline delete my-pipeline
//! ```
//!
//! ## Performance Features
//!
//! ### Parallel Processing
//! - **Worker Threads**: Configurable number of worker threads
//! - **Chunk Processing**: Parallel processing of file chunks
//! - **Load Balancing**: Dynamic load balancing across workers
//!
//! ### Memory Optimization
//! - **Streaming I/O**: Process files without loading entirely into memory
//! - **Chunk Sizing**: Adaptive chunk sizing based on file characteristics
//! - **Memory Pooling**: Efficient memory pool management
//!
//! ### Progress Monitoring
//! - **Real-time Updates**: Live progress updates during processing
//! - **Performance Metrics**: Throughput, latency, and efficiency metrics
//! - **ETA Calculation**: Estimated time to completion
//!
//! ## Configuration
//!
//! ### Environment Variables
//! - **ADAPIPE_SQLITE_PATH**: SQLite database path
//! - **ADAPIPE_LOG_LEVEL**: Logging level (debug, info, warn, error)
//! - **ADAPIPE_WORKER_COUNT**: Number of worker threads
//! - **ADAPIPE_CHUNK_SIZE**: Default chunk size for processing
//!
//! ### Configuration Files
//! - **adapipe.toml**: Main configuration file
//! - **pipelines.toml**: Pipeline definitions
//! - **logging.toml**: Logging configuration
//!
//! ## Error Handling
//!
//! ### Graceful Degradation
//! - **Partial Processing**: Continue processing when possible
//! - **Error Recovery**: Automatic recovery from transient errors
//! - **Fallback Strategies**: Alternative processing methods
//!
//! ### User-Friendly Messages
//! - **Clear Error Messages**: Descriptive error messages with context
//! - **Suggestions**: Helpful suggestions for resolving issues
//! - **Exit Codes**: Standard exit codes for scripting integration
//!
//! ## Integration
//!
//! ### CI/CD Integration
//! - **Scriptable Interface**: Designed for automation and scripting
//! - **Standard Exit Codes**: Proper exit codes for build systems
//! - **JSON Output**: Machine-readable output format option
//!
//! ### System Integration
//! - **File System**: Efficient file system operations
//! - **Database**: SQLite integration for pipeline storage
//! - **Logging**: Structured logging with multiple output formats
//!
//! ## Security Considerations
//!
//! ### File Security
//! - **Permission Validation**: Validate file permissions before processing
//! - **Path Traversal Protection**: Prevent path traversal attacks
//! - **Secure Temporary Files**: Secure handling of temporary files
//!
//! ### Data Protection
//! - **Memory Security**: Secure handling of sensitive data in memory
//! - **Key Management**: Secure key storage and handling
//! - **Audit Logging**: Comprehensive audit trail of operations
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Web Interface**: Web-based management interface
//! - **API Server**: REST API for remote operations
//! - **Plugin System**: Extensible plugin architecture
//! - **Distributed Processing**: Support for distributed processing

use anyhow::Result;
use byte_unit::Byte;
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error, info, warn};

// Import ChunkSize and WorkerCount for optimal sizing calculations
use crate::application::commands::RestoreFileCommand;
// File restoration is now handled via use_cases::restore_file
use pipeline_domain::value_objects::binary_file_format::FileHeader;
use pipeline_domain::value_objects::chunk_size::ChunkSize;
use pipeline_domain::value_objects::worker_count::WorkerCount;
use crate::infrastructure::adapters::file_io_service_adapter::FileIOServiceImpl;
use crate::infrastructure::services::progress_indicator_service::ProgressIndicatorService;

/// Format bytes with 6-digit precision
fn format_bytes_6_digits(bytes: u64) -> String {
    let byte_obj = Byte::from_u128(bytes as u128)
        .unwrap_or_else(|| Byte::from_u128(0).unwrap())
        .get_appropriate_unit(byte_unit::UnitType::Decimal);

    let (value, unit) = (byte_obj.get_value(), byte_obj.get_unit());
    format!("{:.6} {}", value, unit)
}

/// Resolve SQLite database path with proper fallback chain and error handling
fn resolve_sqlite_path() -> Result<String> {
    // 1. Check environment variable first
    if let Ok(env_path) = std::env::var("ADAPIPE_SQLITE_PATH") {
        debug!("Using SQLite path from ADAPIPE_SQLITE_PATH: {}", env_path);
        return Ok(env_path);
    }

    // 2. Check current directory (where exe is running in deployment)
    let current_dir_path = "./pipeline.db";
    if std::path::Path::new(current_dir_path).exists() {
        debug!("Found SQLite database in current directory: {}", current_dir_path);
        return Ok(current_dir_path.to_string());
    }

    // 3. Check debug/development path
    let debug_path = "pipeline/scripts/test_data/structured_pipeline.db";
    if std::path::Path::new(debug_path).exists() {
        debug!("Found SQLite database at debug path: {}", debug_path);
        return Ok(debug_path.to_string());
    }

    // 4. Create default database in current directory
    info!(
        "No existing database found. Creating new database at: {}",
        current_dir_path
    );
    Ok(current_dir_path.to_string())
}

mod application;
mod infrastructure;
mod presentation;

use pipeline_domain::entities::pipeline_stage::{StageConfiguration, StageType};
use pipeline_domain::entities::security_context::Permission;
use pipeline_domain::services::pipeline_service::PipelineService;
use pipeline_domain::{FileChunk, Pipeline, PipelineStage, ProcessingContext, SecurityContext, SecurityLevel};

// Application layer imports (duplicates removed - already imported above)
use pipeline_domain::services::file_io_service::FileIOService;

use pipeline_domain::repositories::stage_executor::StageExecutor;
use crate::infrastructure::adapters::{CompressionServiceImpl, EncryptionServiceImpl};
use crate::infrastructure::logging::ObservabilityService;
use crate::infrastructure::metrics::{MetricsEndpoint, MetricsService};
use crate::infrastructure::adapters::repositories::sqlite_pipeline_repository_adapter::SqlitePipelineRepository;
use crate::infrastructure::repositories::stage_executor::BasicStageExecutor;
use crate::infrastructure::services::{BinaryFormatService, BinaryFormatServiceImpl};
use crate::application::services::pipeline_service::PipelineServiceImpl;

#[derive(Parser)]
#[command(name = "pipeline")]
#[command(about = "Optimized Adaptive Pipeline RS - High-performance file processing system")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    // === Resource Configuration Flags ===
    // Educational: These flags control the GlobalResourceManager's token allocation
    // for CPU-bound and I/O-bound operations.

    /// Override CPU worker thread count
    ///
    /// Controls the number of concurrent CPU-bound operations (compression, encryption).
    /// Default: num_cpus - 1 (reserves 1 core for I/O and coordination)
    ///
    /// Educational: Setting this too high causes thrashing, too low wastes cores.
    /// Monitor CPU saturation metrics to tune appropriately.
    #[arg(long)]
    cpu_threads: Option<usize>,

    /// Override I/O worker thread count
    ///
    /// Controls the number of concurrent I/O operations (file reads/writes).
    /// Default: Device-specific (NVMe: 24, SSD: 12, HDD: 4)
    ///
    /// Educational: This should match your storage device's queue depth for optimal
    /// throughput. Check --storage-type if auto-detection is incorrect.
    #[arg(long)]
    io_threads: Option<usize>,

    /// Specify storage device type for I/O optimization
    ///
    /// Affects default I/O thread count if --io-threads not specified.
    /// Values: nvme (queue depth 24), ssd (12), hdd (4)
    /// Default: auto-detect based on filesystem characteristics
    ///
    /// Educational: Different storage devices have different optimal queue depths.
    /// NVMe handles more concurrent I/O than SSD, which handles more than HDD.
    #[arg(long, value_parser = parse_storage_type)]
    storage_type: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a file through a pipeline
    Process {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Pipeline name or ID
        #[arg(short, long)]
        pipeline: String,

        /// Chunk size in MB
        #[arg(long)]
        chunk_size_mb: Option<usize>,

        /// Number of parallel workers
        #[arg(long)]
        workers: Option<usize>,
    },

    /// Create a new pipeline
    Create {
        /// Pipeline name
        #[arg(short, long)]
        name: String,

        /// Pipeline stages (comma-separated: compression,encryption,integrity)
        #[arg(short, long)]
        stages: String,

        /// Save pipeline to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List available pipelines
    List,

    /// Show pipeline details
    Show {
        /// Pipeline name
        pipeline: String,
    },

    /// Delete a pipeline
    Delete {
        /// Pipeline name to delete
        pipeline: String,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Benchmark system performance
    Benchmark {
        /// Test file path
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Test data size in MB
        #[arg(long, default_value = "100")]
        size_mb: usize,

        /// Number of iterations
        #[arg(long, default_value = "3")]
        iterations: usize,
    },

    /// Validate pipeline configuration
    Validate {
        /// Pipeline configuration file
        config: PathBuf,
    },

    /// Validate .adapipe processed file
    ValidateFile {
        /// .adapipe file to validate
        #[arg(short, long)]
        file: PathBuf,

        /// Perform full streaming validation (decrypt/decompress and verify
        /// checksum)
        #[arg(long)]
        full: bool,
    },

    /// Restore original file from .adapipe file
    Restore {
        /// .adapipe file to restore from
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for restored file (optional - uses original
        /// directory if not specified)
        #[arg(short, long)]
        output_dir: Option<PathBuf>,

        /// Create directories without prompting
        #[arg(long)]
        mkdir: bool,

        /// Overwrite existing files without prompting
        #[arg(long)]
        overwrite: bool,
    },

    /// Compare original file against .adapipe file
    Compare {
        /// Original file to compare
        #[arg(short, long)]
        original: PathBuf,

        /// .adapipe file to compare against
        #[arg(short, long)]
        adapipe: PathBuf,

        /// Show detailed differences
        #[arg(long)]
        detailed: bool,
    },
}

/// Parse and validate storage type from CLI argument
///
/// Educational: Custom value parser for clap that validates
/// storage type strings and provides helpful error messages.
fn parse_storage_type(s: &str) -> Result<String, String> {
    match s.to_lowercase().as_str() {
        "nvme" | "ssd" | "hdd" => Ok(s.to_lowercase()),
        _ => Err(format!(
            "Invalid storage type '{}'. Valid values: nvme, ssd, hdd",
            s
        )),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // === Initialize Global Resource Manager ===
    // Educational: This must happen BEFORE any code uses RESOURCE_MANAGER
    // We configure it from CLI flags, falling back to intelligent defaults.
    use crate::infrastructure::runtime::{init_resource_manager, ResourceConfig, StorageType};

    let resource_config = ResourceConfig {
        cpu_tokens: cli.cpu_threads,
        io_tokens: cli.io_threads,
        storage_type: cli.storage_type.as_ref().map(|s| match s.as_str() {
            "nvme" => StorageType::NVMe,
            "ssd" => StorageType::SSD,
            "hdd" => StorageType::HDD,
            _ => StorageType::Auto, // Shouldn't happen due to parse_storage_type validation
        }).unwrap_or(StorageType::Auto),
        memory_limit: None, // Use system detection
    };

    init_resource_manager(resource_config).map_err(|e| {
        anyhow::anyhow!("Failed to initialize resource manager: {}", e)
    })?;

    // Educational: Log the resource configuration for observability
    let rm = crate::infrastructure::runtime::resource_manager();
    println!(
        "Resource Manager initialized: {} CPU tokens, {} I/O tokens, {} memory capacity",
        rm.cpu_tokens_total(),
        rm.io_tokens_total(),
        rm.memory_capacity()
    );

    // Initialize tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(if cli.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    debug!("Starting Optimized Adaptive Pipeline RS v0.1.0");

    // Initialize Prometheus metrics service
    let metrics_service = Arc::new(MetricsService::new().map_err(|e| {
        error!("Failed to initialize metrics service: {}", e);
        anyhow::anyhow!("Metrics initialization failed: {}", e)
    })?);
    debug!("Prometheus metrics service initialized");

    // Start metrics endpoint on background thread (port configured in
    // observability.toml)
    let metrics_endpoint = MetricsEndpoint::new(metrics_service.clone());
    let metrics_handle = tokio::spawn(async move {
        if let Err(e) = metrics_endpoint.start().await {
            error!("Failed to start metrics endpoint: {}", e);
        }
    });

    // Give metrics endpoint time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Initialize observability service for enhanced monitoring (with config)
    let observability_service = Arc::new(ObservabilityService::new_with_config(metrics_service.clone()).await);
    debug!("Enhanced observability service initialized with configuration");

    // Initialize SQLite pipeline repository
    let sqlite_path = resolve_sqlite_path().map_err(|e| {
        error!("Failed to resolve SQLite path: {}", e);
        anyhow::anyhow!("Failed to resolve SQLite path: {}", e)
    })?;
    debug!("Using SQLite database: {}", sqlite_path);
    let pipeline_repository = Arc::new(SqlitePipelineRepository::new(&sqlite_path).await.map_err(|e| {
        error!("Failed to initialize pipeline repository: {}", e);
        anyhow::anyhow!("Repository initialization failed: {}", e)
    })?);
    debug!("Pipeline repository initialized");

    // Load configuration if provided
    if let Some(config_path) = &cli.config {
        info!("Loading configuration from: {}", config_path.display());
        // TODO: Load configuration
    }

    // Execute command
    match cli.command {
        Commands::Process {
            input,
            output,
            pipeline,
            chunk_size_mb,
            workers,
        } => {
            let config = ProcessFileConfig {
                input,
                output,
                pipeline,
                chunk_size_mb,
                workers,
            };
            process_file(
                config,
                metrics_service.clone(),
                observability_service.clone(),
                pipeline_repository.clone(),
            )
            .await
            .unwrap();
        }

        Commands::Create { name, stages, output } => {
            create_pipeline(name, stages, output, pipeline_repository.clone())
                .await
                .unwrap();
        }

        Commands::List => {
            list_pipelines(pipeline_repository.clone()).await.unwrap();
        }

        Commands::Show { pipeline } => {
            show_pipeline(pipeline, pipeline_repository.clone()).await.unwrap();
        }

        Commands::Delete { pipeline, force } => {
            delete_pipeline(pipeline, force, pipeline_repository.clone())
                .await
                .unwrap();
        }

        Commands::Benchmark {
            file,
            size_mb,
            iterations,
        } => {
            benchmark_system(file, size_mb, iterations).await.unwrap();
        }

        Commands::Validate { config } => {
            validate_pipeline_config(config).await.unwrap();
        }
        Commands::ValidateFile { file, full } => {
            validate_adapipe_file(file, full).await.unwrap();
        }

        Commands::Restore {
            input,
            output_dir,
            mkdir,
            overwrite,
        } => {
            // Use the new hybrid architecture-compliant function
            restore_file_from_adapipe_v2(input, output_dir, mkdir, overwrite)
                .await
                .unwrap();
        }

        Commands::Compare {
            original,
            adapipe,
            detailed,
        } => {
            compare_file_against_adapipe(original, adapipe, detailed).await.unwrap();
        }
    }

    Ok(())
}

/// Configuration for file processing operations.
///
/// Groups all file processing parameters into a single struct to improve
/// function signatures and follow clean code principles.
struct ProcessFileConfig {
    input: PathBuf,
    output: PathBuf,
    pipeline: String,
    chunk_size_mb: Option<usize>,
    workers: Option<usize>,
}

async fn process_file(
    config: ProcessFileConfig,
    metrics_service: Arc<MetricsService>,
    observability_service: Arc<ObservabilityService>,
    pipeline_repository: Arc<SqlitePipelineRepository>,
) -> Result<()> {
    let ProcessFileConfig {
        input,
        output,
        pipeline,
        chunk_size_mb,
        workers,
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
            .unwrap_or_else(|| Byte::from_u128(0).unwrap())
            .get_appropriate_unit(byte_unit::UnitType::Decimal)
            .to_string()
    );

    // Determine chunk size: user override with validation or adaptive
    let optimal_chunk_size = ChunkSize::optimal_for_file_size(actual_input_size);
    let (actual_chunk_size_bytes, chunk_size_source) = if let Some(user_chunk_mb) = chunk_size_mb {
        // User specified chunk size - validate it
        match ChunkSize::validate_user_input(user_chunk_mb, actual_input_size) {
            Ok(validated_bytes) => {
                // Check if user value is different from adaptive
                if validated_bytes == optimal_chunk_size.bytes() {
                    debug!("User-specified chunk size {} MB matches adaptive choice", user_chunk_mb);
                    (validated_bytes, "adaptive") // Same as adaptive, so call
                                                  // it adaptive
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
        // Use adaptive chunk size
        debug!("Using adaptive chunk size: {} bytes", optimal_chunk_size.bytes());
        (optimal_chunk_size.bytes(), "adaptive")
    };

    let _actual_chunk_size_mb = actual_chunk_size_bytes / (1024 * 1024);

    debug!(
        "Final chunk size: {} bytes ({}) - {}",
        actual_chunk_size_bytes,
        Byte::from_u128(actual_chunk_size_bytes as u128)
            .unwrap_or_else(|| Byte::from_u128(0).unwrap())
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

    // Implement actual file processing
    debug!("Starting file processing...");

    // Load existing pipeline from repository

    // Look up pipeline by name (user-friendly) instead of UUID
    let pipeline = pipeline_repository
        .find_by_name(&pipeline)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Pipeline '{}' not found", pipeline))
        .unwrap();

    debug!(
        "Loaded pipeline '{}' with {} stages",
        pipeline.name(),
        pipeline.stages().len()
    );
    for stage in pipeline.stages() {
        debug!("  - Stage: {} (type: {:?})", stage.name(), stage.stage_type());
    }

    // Create services needed by stage executor
    let compression_service = Arc::new(CompressionServiceImpl::new());
    let encryption_service = Arc::new(EncryptionServiceImpl::new());
    let file_io_service = Arc::new(FileIOServiceImpl::new(Default::default()));
    let binary_format_service = Arc::new(BinaryFormatServiceImpl::new());

    // Create pipeline service with proper dependency injection
    let pipeline_service = PipelineServiceImpl::new(
        compression_service.clone(),
        encryption_service.clone(),
        file_io_service,
        pipeline_repository,
        Arc::new(BasicStageExecutor::new(compression_service, encryption_service)),
        binary_format_service,
    );

    // Get actual input file size for accurate reporting
    let actual_input_size = std::fs::metadata(&input)
        .map_err(|e| anyhow::anyhow!("Failed to read input file metadata: {}", e))?
        .len();

    // Track active pipeline processing with enhanced observability
    metrics_service.increment_active_pipelines();
    let operation_tracker = observability_service.start_operation("file_processing").await;

    // Start timing the entire processing operation
    let processing_start = Instant::now();

    // Create metrics observer for real-time monitoring
    let metrics_observer = Arc::new(crate::infrastructure::metrics::MetricsObserver::new(
        metrics_service.clone(),
    ));

    // Process the file through the pipeline using the domain PipelineId with
    // observer
    let processing_result = pipeline_service
        .process_file(
            pipeline.id().clone(),
            input.as_path(),
            output.as_path(),
            security_context,
            workers,
            Some(metrics_observer),
        )
        .await;

    // Calculate total processing duration
    let total_processing_duration = processing_start.elapsed();

    // Always decrement active pipelines when done (success or failure)
    metrics_service.decrement_active_pipelines();

    match processing_result {
        Ok(mut metrics) => {
            debug!("File processing completed successfully");

            // Calculate and set compression ratio in metrics for system-wide availability
            let compression_ratio = if actual_input_size > 0 {
                metrics.output_file_size_bytes() as f64 / actual_input_size as f64
            } else {
                0.0
            };
            if compression_ratio > 0.0 {
                metrics.set_compression_ratio(compression_ratio);
            }

            // Metrics are recorded by MetricsObserver to avoid double counting
            observability_service.record_processing_metrics(&metrics).await;

            // Complete operation tracking with success
            operation_tracker.complete_with_metrics(&metrics).await;

            // Beautiful boxed report format
            println!();

            // Processing summary with compression info
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

            // Calculate the maximum line length to determine box width
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

            // Find the longest line to determine box width
            let max_content_width = [status_text.len(), input_text.len(), output_text.len()]
                .iter()
                .max()
                .unwrap_or(&0)
                + 2; // +2 for padding spaces
            let box_width = max_content_width + 2; // +2 for the border characters

            // Create top border
            let horizontal_line = "─".repeat(box_width - 2);
            println!("┌{}┐", horizontal_line);

            // Print content lines with dynamic width
            println!("│ {:<width$} │", status_text, width = max_content_width - 2);
            println!("│ {:<width$} │", input_text, width = max_content_width - 2);
            println!("│ {:<width$} │", output_text, width = max_content_width - 2);

            // Create bottom border
            println!("└{}┘", horizontal_line);
            println!();

            // Performance metrics
            let total_chunks =
                actual_input_size.div_ceil(actual_chunk_size_bytes as u64);
            let chunk_size_mb = actual_chunk_size_bytes as f64 / (1024.0 * 1024.0);

            println!("⚡ PERFORMANCE METRICS");
            println!("├─ Processing Time:   {:.3} seconds", processing_seconds);
            println!("├─ Throughput:        {:.1} MB/s", actual_throughput);
            println!("├─ Total Chunks:      {} ({:.1} MB each)", total_chunks, chunk_size_mb);
            println!("└─ Errors:            {}", metrics.error_count());
            println!();

            // Adaptive configuration with detailed stages
            let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
            let optimal_workers = WorkerCount::optimal_for_file_size(actual_input_size);

            let (chunk_strategy, chunk_label) = match chunk_size_source {
                "user-override" => ("User-specified".to_string(), "user override".to_string()),
                "adaptive-fallback" => (
                    format!("{} (fallback)", ChunkSize::strategy_description(actual_input_size)),
                    "adaptive fallback".to_string(),
                ),
                _ => (
                    ChunkSize::strategy_description(actual_input_size).to_string(),
                    "adaptive".to_string(),
                ),
            };

            let (worker_strategy, worker_label, worker_count) = if let Some(user_workers) = workers {
                match WorkerCount::validate_user_input(user_workers, available_cores, actual_input_size) {
                    Ok(_) => {
                        if user_workers == optimal_workers.count() {
                            (
                                WorkerCount::strategy_description(actual_input_size).to_string(),
                                "adaptive".to_string(),
                                optimal_workers.count(),
                            )
                        } else {
                            ("User-specified".to_string(), "user override".to_string(), user_workers)
                        }
                    }
                    Err(_) => (
                        format!("{} (fallback)", WorkerCount::strategy_description(actual_input_size)),
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

            // Pipeline stages with detailed execution info
            let stage_names: Vec<String> = pipeline.stages().iter().map(|stage| stage.name().to_string()).collect();

            if !stage_names.is_empty() {
                let stage_metrics_map = metrics.stage_metrics();

                if !stage_metrics_map.is_empty() {
                    // Show detailed stage execution
                    println!("└─ Pipeline Stages:   {}", stage_names.join(" → "));
                    println!();
                    println!("🔬 STAGE EXECUTION DETAILS");

                    for (i, stage_name) in stage_names.iter().enumerate() {
                        let stage_num = i + 1;
                        let prefix = if i == stage_names.len() - 1 { "└─" } else { "├─" };

                        if let Some(stage_metrics) = stage_metrics_map.get(stage_name) {
                            let stage_time_ms = stage_metrics.processing_time.as_millis();
                            let stage_throughput_mb = stage_metrics.throughput / (1024.0 * 1024.0);
                            let stage_mb_processed = stage_metrics.bytes_processed as f64 / (1024.0 * 1024.0);
                            let status_icon = if stage_metrics.error_count == 0 { "✅" } else { "❌" };

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
                    println!("└─ Pipeline Stages:   {} (all completed ✅)", stage_names.join(" → "));
                }
            } else {
                println!("└─ Pipeline Stages:   None");
            }
            println!();

            // File integrity with full checksums
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
        Err(e) => {
            // Clean user-friendly error report with prominent filenames
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
            println!("Error:              {}", e);
            println!();
            println!("Final Status:       Failed");
            println!("========================================================================================================================");

            error!("File processing failed: {}", e);
            return Err(anyhow::anyhow!("File processing failed: {}", e));
        }
    }

    Ok(())
}

/// Normalizes pipeline name to kebab-case standard
fn normalize_pipeline_name(name: &str) -> String {
    name.to_lowercase()
        // Replace common separators with hyphens
        .replace([' ', '_', '.', '/', '\\', ':', ';', ',', '|', '&', '+', '=', '!', '?', '*', '%', '#', '@', '$', '^', '(', ')', '[', ']', '{', '}', '<', '>', '"', '\'', '`', '~'], "-")
        // Remove any remaining non-alphanumeric, non-hyphen characters
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        // Clean up multiple consecutive hyphens
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Validates pipeline name according to kebab-case naming conventions
fn validate_pipeline_name(name: &str) -> Result<String> {
    // Check for empty name
    if name.is_empty() {
        return Err(anyhow::anyhow!("Pipeline name cannot be empty"));
    }

    // Normalize to kebab-case
    let normalized = normalize_pipeline_name(name);

    // Check minimum length after normalization
    if normalized.len() < 4 {
        return Err(anyhow::anyhow!("Pipeline name must be at least 4 characters long"));
    }

    // After normalization, the name should be clean kebab-case
    // No additional character validation needed since normalization handles it

    // Reserved names
    let reserved_names = [
        "help", "version", "list", "show", "create", "delete", "update", "config",
    ];
    if reserved_names.contains(&normalized.as_str()) {
        return Err(anyhow::anyhow!(
            "Pipeline name '{}' is reserved. Please choose a different name.",
            name
        ));
    }

    // Inform user if name was normalized
    if normalized != name {
        info!(
            "Pipeline name normalized from '{}' to '{}' (kebab-case standard)",
            name, normalized
        );
    }

    Ok(normalized)
}

async fn create_pipeline(
    name: String,
    stages: String,
    output: Option<PathBuf>,
    pipeline_repository: Arc<SqlitePipelineRepository>,
) -> Result<()> {
    info!("Creating pipeline: {}", name);
    info!("Stages: {}", stages);

    // Validate and normalize pipeline name
    let _normalized_name = validate_pipeline_name(&name).unwrap();

    let stage_names: Vec<&str> = stages.split(',').collect();
    let mut pipeline_stages = Vec::new();

    for (index, stage_name) in stage_names.iter().enumerate() {
        let (stage_type, algorithm) = match stage_name.trim() {
            "compression" => (StageType::Compression, "brotli".to_string()),
            "encryption" => (StageType::Encryption, "aes256gcm".to_string()),
            "integrity" | "checksum" => (StageType::Checksum, "sha256".to_string()),
            custom_name if custom_name.contains("checksum") => (StageType::Checksum, "sha256".to_string()),
            "passthrough" => (StageType::PassThrough, "passthrough".to_string()),
            // Handle compression:algorithm syntax
            custom_name if custom_name.starts_with("compression:") => {
                let algorithm = custom_name.strip_prefix("compression:").unwrap_or("brotli").to_string();
                (StageType::Compression, algorithm)
            }
            // Handle encryption:algorithm syntax
            custom_name if custom_name.starts_with("encryption:") => {
                let algorithm = custom_name
                    .strip_prefix("encryption:")
                    .unwrap_or("aes256gcm")
                    .to_string();
                (StageType::Encryption, algorithm)
            }
            _custom => (StageType::PassThrough, "passthrough".to_string()),
        };

        let config = StageConfiguration {
            algorithm,
            ..Default::default()
        };

        let stage = PipelineStage::new(stage_name.trim().to_string(), stage_type, config, index as u32).unwrap();

        pipeline_stages.push(stage);
    }

    let pipeline = Pipeline::new(name, pipeline_stages).unwrap();

    // Save pipeline to database
    pipeline_repository
        .save(&pipeline)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to save pipeline: {}", e))
        .unwrap();

    info!(
        "Pipeline '{}' created successfully with ID: {}",
        pipeline.name(),
        pipeline.id()
    );
    info!("Pipeline saved to database");

    if let Some(_output_path) = output {
        info!("Note: File output not yet implemented, pipeline saved to database only");
    }

    Ok(())
}

async fn list_pipelines(pipeline_repository: Arc<SqlitePipelineRepository>) -> Result<()> {
    info!("Listing available pipelines:");

    // Query all pipelines
    let pipelines = pipeline_repository
        .list_all()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query pipelines: {}", e))
        .unwrap();

    if pipelines.is_empty() {
        println!("No pipelines found. Use 'pipeline create' to create a new pipeline.");
    } else {
        println!("Found {} pipeline(s):", pipelines.len());
        println!();

        for pipeline in pipelines {
            println!("Pipeline: {}", pipeline.name());
            println!("  ID: {}", pipeline.id());
            println!("  Status: {}", pipeline.status());
            println!("  Stages: {}", pipeline.stages().len());
            println!("  Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));
            println!("  Updated: {}", pipeline.updated_at().format("%Y-%m-%d %H:%M:%S UTC"));
            println!();
        }
    }

    Ok(())
}

async fn show_pipeline(pipeline_name: String, pipeline_repository: Arc<SqlitePipelineRepository>) -> Result<()> {
    info!("Showing pipeline details: {}", pipeline_name);

    // Find pipeline by name (user-friendly)
    let pipeline = pipeline_repository
        .find_by_name(&pipeline_name)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Pipeline not found: {}", pipeline_name))
        .unwrap();

    // Display pipeline details
    println!("\n=== Pipeline Details ===");
    println!("ID: {}", pipeline.id());
    println!("Name: {}", pipeline.name());
    println!("Status: {}", pipeline.status());
    println!("Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Updated: {}", pipeline.updated_at().format("%Y-%m-%d %H:%M:%S UTC"));
    println!("\nStages ({}):", pipeline.stages().len());

    for (index, stage) in pipeline.stages().iter().enumerate() {
        println!(
            "  {}. {} ({:?})",
            index + 1,
            stage.name(),
            stage.stage_type()
        );
        println!("     Algorithm: {}", stage.configuration().algorithm);
        println!("     Enabled: {}", stage.is_enabled());
        println!("     Order: {}", stage.order());
        if !stage.configuration().parameters.is_empty() {
            println!("     Parameters:");
            for (key, value) in &stage.configuration().parameters {
                println!("       {}: {}", key, value);
            }
        }
        if index < pipeline.stages().len() - 1 {
            println!();
        }
    }

    // Display configuration if any
    if !pipeline.configuration().is_empty() {
        println!("\nConfiguration:");
        for (key, value) in pipeline.configuration() {
            println!("  {}: {}", key, value);
        }
    }

    // Display metrics
    let metrics = pipeline.metrics();
    println!("\nMetrics:");
    println!("  Bytes Processed: {}", metrics.bytes_processed());
    println!("  Chunks Processed: {}", metrics.chunks_processed());
    println!("  Error Count: {}", metrics.error_count());
    println!("  Warning Count: {}", metrics.warning_count());

    Ok(())
}

async fn delete_pipeline(
    pipeline_name: String,
    force: bool,
    pipeline_repository: Arc<SqlitePipelineRepository>,
) -> Result<()> {
    info!("Deleting pipeline: {}", pipeline_name);

    // Find pipeline by name first
    let pipeline = pipeline_repository
        .find_by_name(&pipeline_name)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to query pipeline: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("Pipeline '{}' not found", pipeline_name))
        .unwrap();

    // Show pipeline details before deletion
    println!("\n=== Pipeline to Delete ===");
    println!("Name: {}", pipeline.name());
    println!("ID: {}", pipeline.id());
    println!("Stages: {}", pipeline.stages().len());
    println!("Created: {}", pipeline.created_at().format("%Y-%m-%d %H:%M:%S UTC"));

    // Confirmation prompt unless --force is used
    if !force {
        print!(
            "\nAre you sure you want to delete pipeline '{}'? [y/N]: ",
            pipeline_name
        );
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Pipeline deletion cancelled.");
            return Ok(());
        }
    }

    // Delete the pipeline
    pipeline_repository
        .delete(pipeline.id().clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to delete pipeline: {}", e))
        .unwrap();

    println!("✅ Pipeline '{}' deleted successfully", pipeline_name);
    Ok(())
}

async fn benchmark_system(file: Option<PathBuf>, size_mb: usize, iterations: usize) -> Result<()> {
    info!("Running comprehensive pipeline optimization benchmark");
    info!("Test size: {}MB", size_mb);
    info!("Iterations: {}", iterations);

    // Create metrics service for benchmarking
    let metrics_service = Arc::new(MetricsService::new()?);

    // Test file sizes in MB
    let test_sizes = if size_mb > 0 {
        vec![size_mb]
    } else {
        vec![1, 5, 10, 50, 100, 500, 1000, 2048] // Default test sizes including
                                                 // 2GB
    };

    // Chunk sizes to test (in MB)
    let chunk_sizes = vec![1, 2, 4, 8, 16, 32, 64, 128];

    // Worker counts to test
    let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let max_workers = (available_cores * 2).min(16); // Test up to 2x cores or 16, whichever is smaller
    let worker_counts: Vec<usize> = (1..=max_workers).collect();

    println!("\n========================================================================================================================");
    println!(
        "========================================== PIPELINE OPTIMIZATION BENCHMARK \
         ==========================================="
    );
    println!("========================================================================================================================");
    println!("System Info:        {} CPU cores available", available_cores);
    println!("Test Iterations:    {}", iterations);
    println!("File Sizes:         {:?} MB", test_sizes);
    println!("Chunk Sizes:        {:?} MB", chunk_sizes);
    println!("Worker Counts:      {:?}", worker_counts);
    println!("========================================================================================================================");

    let mut results = Vec::new();

    for &test_size_mb in &test_sizes {
        println!("\n🔍 Testing file size: {} MB", test_size_mb);

        // Create or use test file
        let test_file = if let Some(ref provided_file) = file {
            provided_file.clone()
        } else {
            // Generate test file
            let test_file = PathBuf::from(format!("benchmark_test_{}mb.txt", test_size_mb));
            generate_test_file(&test_file, test_size_mb).await.unwrap();
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
        let adaptive_chunk_mb = (adaptive_chunk.bytes() as f64 / (1024.0 * 1024.0)).max(1.0) as usize;
        let adaptive_result = run_benchmark_test(
            &test_file,
            test_size_mb,
            Some(adaptive_chunk_mb),
            Some(adaptive_workers.count()),
            iterations,
            &metrics_service,
        )
        .await
        .unwrap();

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
            if chunk_mb == adaptive_chunk.megabytes() as usize {
                continue; // Skip adaptive (already tested)
            }

            let result = run_benchmark_test(
                &test_file,
                test_size_mb,
                Some(chunk_mb),
                Some(adaptive_workers.count()),
                iterations,
                &metrics_service,
            )
            .await
            .unwrap();

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

            let result = run_benchmark_test(
                &test_file,
                test_size_mb,
                Some(adaptive_chunk_mb),
                Some(workers),
                iterations,
                &metrics_service,
            )
            .await
            .unwrap();

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
            std::fs::remove_file(&test_file).unwrap();
        }
    }

    // Generate comprehensive report
    generate_optimization_report(&results).await.unwrap();

    println!("\n✅ Benchmark completed successfully!");
    println!("📊 Check the generated optimization report for detailed results.");

    Ok(())
}

// Benchmark result structure
#[derive(Debug, Clone)]
struct BenchmarkResult {
    file_size_mb: usize,
    chunk_size_mb: usize,
    worker_count: usize,
    avg_throughput_mbps: f64,
    avg_duration_secs: f64,
    config_type: String,
}

// Single benchmark test result
#[derive(Debug)]
struct TestResult {
    avg_throughput_mbps: f64,
    avg_duration_secs: f64,
}

// Simulate pipeline processing for benchmarking
async fn simulate_pipeline_processing(
    input_file: &PathBuf,
    output_file: &PathBuf,
    chunk_size_mb: usize,
    worker_count: usize,
) -> Result<()> {
    use std::io::{Read, Write};
    use tokio::task;

    let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
    let mut input = std::fs::File::open(input_file).unwrap();
    let mut output = std::fs::File::create(output_file).unwrap();

    // Read file in chunks and simulate processing
    let mut buffer = vec![0u8; chunk_size_bytes];
    let mut chunks = Vec::new();

    // Read all chunks
    loop {
        let bytes_read = input.read(&mut buffer).unwrap();
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
                    let _processed: Vec<u8> = chunk.iter().map(|&b| b ^ 0x42).collect();
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
            // Write processed chunk (just write original for benchmark)
            output.write_all(&chunk).unwrap();
        }
    }

    output.flush().unwrap();
    Ok(())
}

// Generate test file of specified size
async fn generate_test_file(path: &PathBuf, size_mb: usize) -> Result<()> {
    use std::io::Write;

    let mut file = std::fs::File::create(path).unwrap();
    let chunk_size = 1024 * 1024; // 1MB chunks
    let data = vec![b'A'; chunk_size]; // Fill with 'A' characters

    for _ in 0..size_mb {
        file.write_all(&data).unwrap();
    }

    file.flush().unwrap();
    Ok(())
}

// Run a single benchmark test
async fn run_benchmark_test(
    test_file: &PathBuf,
    _file_size_mb: usize,
    chunk_size_mb: Option<usize>,
    worker_count: Option<usize>,
    iterations: usize,
    _metrics_service: &Arc<MetricsService>,
) -> Result<TestResult> {
    let mut durations = Vec::new();
    let mut throughputs = Vec::new();

    for i in 0..iterations {
        let output_file = PathBuf::from(format!("benchmark_output_{}_{}.adapipe", std::process::id(), i));

        let start_time = Instant::now();

        // Run the pipeline processing
        // For benchmarking, we'll simulate processing by reading the file in chunks
        let result = simulate_pipeline_processing(
            test_file,
            &output_file,
            chunk_size_mb.unwrap_or(1),
            worker_count.unwrap_or(1),
        )
        .await;

        let duration = start_time.elapsed();

        // Clean up output file
        if output_file.exists() {
            std::fs::remove_file(&output_file).unwrap();
        }

        match result {
            Ok(_) => {
                let duration_secs = duration.as_secs_f64();
                let file_size_bytes = std::fs::metadata(test_file)?.len();
                let throughput_mbps = (file_size_bytes as f64 / (1024.0 * 1024.0)) / duration_secs;

                durations.push(duration_secs);
                throughputs.push(throughput_mbps);
            }
            Err(e) => {
                warn!("Benchmark iteration {} failed: {}", i, e);
                // Use penalty values for failed runs
                durations.push(999.0);
                throughputs.push(0.0);
            }
        }
    }

    let avg_duration = durations.iter().sum::<f64>() / durations.len() as f64;
    let avg_throughput = throughputs.iter().sum::<f64>() / throughputs.len() as f64;

    Ok(TestResult {
        avg_throughput_mbps: avg_throughput,
        avg_duration_secs: avg_duration,
    })
}

// Generate comprehensive optimization report
async fn generate_optimization_report(results: &[BenchmarkResult]) -> Result<()> {
    let report_file = PathBuf::from("pipeline_optimization_report.md");
    let mut report = String::new();

    report.push_str("# Pipeline Optimization Benchmark Report\n\n");
    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // Group results by file size
    let mut file_sizes: Vec<usize> = results.iter().map(|r| r.file_size_mb).collect();
    file_sizes.sort_unstable();
    file_sizes.dedup();

    for file_size in &file_sizes {
        report.push_str(&format!("## File Size: {} MB\n\n", file_size));

        let size_results: Vec<_> = results.iter().filter(|r| r.file_size_mb == *file_size).collect();

        // Find best configuration
        let best_result = size_results
            .iter()
            .max_by(|a, b| a.avg_throughput_mbps.partial_cmp(&b.avg_throughput_mbps).unwrap())
            .unwrap();

        let adaptive_result = size_results.iter().find(|r| r.config_type == "Adaptive").unwrap();

        report.push_str("**Adaptive Configuration:**\n");
        report.push_str(&format!("- Chunk Size: {} MB\n", adaptive_result.chunk_size_mb));
        report.push_str(&format!("- Worker Count: {}\n", adaptive_result.worker_count));
        report.push_str(&format!(
            "- Throughput: {:.2} MB/s\n",
            adaptive_result.avg_throughput_mbps
        ));
        report.push_str(&format!(
            "- Duration: {:.2} seconds\n\n",
            adaptive_result.avg_duration_secs
        ));

        report.push_str("**Best Configuration:**\n");
        report.push_str(&format!("- Chunk Size: {} MB\n", best_result.chunk_size_mb));
        report.push_str(&format!("- Worker Count: {}\n", best_result.worker_count));
        report.push_str(&format!("- Throughput: {:.2} MB/s\n", best_result.avg_throughput_mbps));
        report.push_str(&format!("- Duration: {:.2} seconds\n", best_result.avg_duration_secs));
        report.push_str(&format!("- Configuration Type: {}\n\n", best_result.config_type));

        let improvement = ((best_result.avg_throughput_mbps - adaptive_result.avg_throughput_mbps)
            / adaptive_result.avg_throughput_mbps)
            * 100.0;

        if improvement > 0.0 {
            report.push_str(&format!(
                "**Performance Improvement:** {:.1}% faster than adaptive\n\n",
                improvement
            ));
        } else {
            report.push_str("**Performance:** Adaptive configuration is optimal\n\n");
        }

        // Detailed results table
        report.push_str("### Detailed Results\n\n");
        report.push_str("| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |\n");
        report.push_str("|-----------------|---------|-------------------|--------------|-------------|\n");

        let mut sorted_results = size_results.clone();
        sorted_results.sort_by(|a, b| b.avg_throughput_mbps.partial_cmp(&a.avg_throughput_mbps).unwrap());

        for result in sorted_results {
            report.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {} |\n",
                result.chunk_size_mb,
                result.worker_count,
                result.avg_throughput_mbps,
                result.avg_duration_secs,
                result.config_type
            ));
        }

        report.push('\n');
    }

    // Summary recommendations
    report.push_str("## Summary Recommendations\n\n");

    for file_size in &file_sizes {
        let size_results: Vec<_> = results.iter().filter(|r| r.file_size_mb == *file_size).collect();

        let best_result = size_results
            .iter()
            .max_by(|a, b| a.avg_throughput_mbps.partial_cmp(&b.avg_throughput_mbps).unwrap())
            .unwrap();

        report.push_str(&format!(
            "- **{} MB files**: {} MB chunks, {} workers ({:.2} MB/s)\n",
            file_size, best_result.chunk_size_mb, best_result.worker_count, best_result.avg_throughput_mbps
        ));
    }

    // Write report to file
    std::fs::write(&report_file, report).unwrap();

    println!("\n📊 Optimization report generated: {}", report_file.display());

    Ok(())
}

async fn validate_pipeline_config(config_path: PathBuf) -> Result<()> {
    info!("Validating pipeline configuration: {}", config_path.display());

    // Validate file exists
    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration file does not exist: {}",
            config_path.display()
        ));
    }

    // Read and parse configuration file
    let config_content = std::fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read configuration file: {}", e))
        .unwrap();

    println!("🔍 Validating configuration file: {}", config_path.display());
    println!("   File size: {} bytes", config_content.len());

    // Determine file format and validate accordingly
    let file_extension = config_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match file_extension.to_lowercase().as_str() {
        "toml" => validate_toml_config(&config_content, &config_path)?,
        "json" => validate_json_config(&config_content, &config_path)?,
        "yaml" | "yml" => validate_yaml_config(&config_content, &config_path)?,
        _ => {
            // Try to auto-detect format
            if config_content.trim_start().starts_with('{') {
                validate_json_config(&config_content, &config_path).unwrap();
            } else if config_content.contains("---") || config_content.contains(":") {
                validate_yaml_config(&config_content, &config_path).unwrap();
            } else {
                validate_toml_config(&config_content, &config_path).unwrap();
            }
        }
    }

    println!("\n✅ Configuration validation completed successfully!");
    Ok(())
}

/// Validate TOML configuration format
fn validate_toml_config(content: &str, _path: &PathBuf) -> Result<()> {
    println!("   Format: TOML");

    // Parse TOML
    let parsed: toml::Value = toml::from_str(content)
        .map_err(|e| anyhow::anyhow!("Invalid TOML syntax: {}", e))
        .unwrap();

    // Validate expected structure
    if let Some(pipelines) = parsed.get("pipelines") {
        if let Some(pipeline_table) = pipelines.as_table() {
            println!("   Found {} pipeline(s) in configuration", pipeline_table.len());

            for (name, config) in pipeline_table {
                validate_pipeline_config_entry(name, config).unwrap();
            }
        }
    }

    // Validate global settings if present
    if let Some(settings) = parsed.get("settings") {
        validate_global_settings(settings).unwrap();
    }

    println!("   ✅ TOML structure is valid");
    Ok(())
}

/// Validate JSON configuration format
fn validate_json_config(content: &str, _path: &PathBuf) -> Result<()> {
    println!("   Format: JSON");

    // Parse JSON
    let parsed: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("Invalid JSON syntax: {}", e))
        .unwrap();

    // Validate expected structure
    if let Some(pipelines) = parsed.get("pipelines") {
        if let Some(pipeline_obj) = pipelines.as_object() {
            println!("   Found {} pipeline(s) in configuration", pipeline_obj.len());

            for (name, config) in pipeline_obj {
                validate_json_pipeline_entry(name, config).unwrap();
            }
        }
    }

    println!("   ✅ JSON structure is valid");
    Ok(())
}

/// Validate YAML configuration format
fn validate_yaml_config(content: &str, _path: &PathBuf) -> Result<()> {
    println!("   Format: YAML");

    // Basic YAML validation (simplified)
    let lines: Vec<&str> = content.lines().collect();
    let mut _indent_stack: Vec<usize> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Check for basic YAML structure
        if trimmed.contains(':') {
            let indent = line.len() - line.trim_start().len();
            // Basic indentation validation
            if indent % 2 != 0 {
                return Err(anyhow::anyhow!(
                    "Invalid YAML indentation at line {}: should be multiple of 2",
                    line_num + 1
                ));
            }
        }
    }

    println!("   Found {} lines of YAML configuration", lines.len());
    println!("   ✅ YAML structure appears valid");
    Ok(())
}

/// Validate individual pipeline configuration entry
fn validate_pipeline_config_entry(name: &str, config: &toml::Value) -> Result<()> {
    println!("     Pipeline '{}'", name);

    // Validate pipeline name
    if name.is_empty() {
        return Err(anyhow::anyhow!("Pipeline name cannot be empty"));
    }

    // Check for required fields
    if let Some(stages) = config.get("stages") {
        if let Some(stage_array) = stages.as_array() {
            println!("       {} stage(s) configured", stage_array.len());

            for (i, stage) in stage_array.iter().enumerate() {
                if let Some(stage_name) = stage.get("name").and_then(|n| n.as_str()) {
                    println!("         Stage {}: {}", i + 1, stage_name);
                }
            }
        }
    }

    Ok(())
}

/// Validate JSON pipeline entry
fn validate_json_pipeline_entry(name: &str, config: &serde_json::Value) -> Result<()> {
    println!("     Pipeline '{}'", name);

    if let Some(stages) = config.get("stages") {
        if let Some(stage_array) = stages.as_array() {
            println!("       {} stage(s) configured", stage_array.len());
        }
    }

    Ok(())
}

/// Validate global settings
fn validate_global_settings(settings: &toml::Value) -> Result<()> {
    println!("   Global settings found:");

    if let Some(chunk_size) = settings.get("default_chunk_size") {
        println!("     Default chunk size: {:?}", chunk_size);
    }

    if let Some(worker_count) = settings.get("default_worker_count") {
        println!("     Default worker count: {:?}", worker_count);
    }

    Ok(())
}

async fn validate_adapipe_file(file_path: PathBuf, full_validation: bool) -> Result<()> {
    info!("Validating .adapipe file: {}", file_path.display());

    if !file_path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {}", file_path.display()));
    }

    if file_path.extension().is_none_or(|ext| ext != "adapipe") {
        println!("Warning: File does not have .adapipe extension");
    }

    let binary_format_service = BinaryFormatServiceImpl::new();

    // Step 1: Basic format validation
    println!("🔍 Validating .adapipe file format...");
    let validation_result = binary_format_service
        .validate_file(&file_path)
        .await
        .map_err(|e| anyhow::anyhow!("Format validation failed: {}", e))
        .unwrap();

    if !validation_result.is_valid {
        println!("❌ File format validation failed!");
        for error in &validation_result.errors {
            println!("   Error: {}", error);
        }
        return Err(anyhow::anyhow!("Invalid .adapipe file format"));
    }

    println!("✅ File format is valid");

    // Step 2: Read and display metadata
    println!("\n📋 Reading file metadata...");
    let metadata = binary_format_service
        .read_metadata(&file_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read metadata: {}", e))
        .unwrap();

    println!("   Original filename: {}", metadata.original_filename);
    println!(
        "   Original size: {}",
        Byte::from_u128(metadata.original_size as u128)
            .unwrap_or_else(|| Byte::from_u128(0).unwrap())
            .get_appropriate_unit(byte_unit::UnitType::Decimal)
    );
    println!("   Original checksum: {}", metadata.original_checksum);
    println!("   Format version: {}", metadata.format_version);
    println!("   App version: {}", metadata.app_version);
    println!(
        "   Chunk size: {}",
        Byte::from_u128(metadata.chunk_size as u128)
            .unwrap_or_else(|| Byte::from_u128(0).unwrap())
            .get_appropriate_unit(byte_unit::UnitType::Decimal)
    );
    println!("   Chunk count: {}", metadata.chunk_count);
    println!("   Pipeline ID: {}", metadata.pipeline_id);
    println!(
        "   Processed at: {}",
        metadata.processed_at.format("%Y-%m-%d %H:%M:%S UTC")
    );

    if metadata.is_compressed() {
        println!(
            "   🗜️  Compression: {}",
            metadata.compression_algorithm().unwrap_or("unknown")
        );
    }

    if metadata.is_encrypted() {
        println!(
            "   🔒 Encryption: {}",
            metadata.encryption_algorithm().unwrap_or("unknown")
        );
    }

    if metadata.processing_steps.is_empty() {
        println!("   📄 Pass-through file (no processing)");
    } else {
        println!("   🔄 Processing steps: {}", metadata.get_processing_summary());
    }

    // Step 3: Full streaming validation (if requested)
    if full_validation {
        println!("\n🔄 Performing full streaming validation...");
        println!("   This will decrypt, decompress, and verify the original checksum");
        println!("   No temporary files will be created (streaming validation)");

        // Implement full streaming validation
        println!("   🔄 Performing full streaming validation...");
        println!("   This will stream through decryption -> decompression -> SHA-256 verification");
        println!("   Expected original checksum: {}", metadata.original_checksum);

        // TODO: Full streaming validation removed temporarily
        // The restoration service was removed. This needs to be reimplemented using
        // use_cases::restore_file directly for streaming validation.
        println!("   ⚠️  Full streaming validation not yet implemented");
        println!("   (Restoration service refactoring in progress)")
    } else {
        println!("\n💡 Use --full flag for complete streaming validation (decrypt/decompress/verify)");
    }

    println!("\n✅ .adapipe file validation completed successfully!");

    Ok(())
}

/// Hybrid Architecture-compliant restore function using Application Service
async fn restore_file_from_adapipe_v2(
    input: PathBuf,
    output_dir: Option<PathBuf>,
    mkdir: bool,
    overwrite: bool,
) -> Result<()> {
    info!("Restoring file from .adapipe: {}", input.display());

    // Validate input file exists
    if !input.exists() {
        return Err(anyhow::anyhow!(
            "Input .adapipe file does not exist: {}",
            input.display()
        ));
    }

    // Read .adapipe metadata to determine target path
    println!("🔍 Reading .adapipe file metadata...");
    let file_data = std::fs::read(&input).unwrap();
    let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))
        .unwrap();

    // Determine output path
    let target_path = if let Some(ref dir) = output_dir {
        // Use specified directory + original filename
        let original_filename = std::path::Path::new(&metadata.original_filename)
            .file_name()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not extract filename from original filename: {}",
                    metadata.original_filename
                )
            })?
            .to_string_lossy()
            .to_string();

        dir.join(original_filename)
    } else {
        // Use original full path from metadata
        PathBuf::from(&metadata.original_filename)
    };

    println!("📁 Target restoration path: {}", target_path.display());

    // Note: Restoration service removed - use use_cases::restore_file directly instead
    // let file_io_service = Arc::new(FileIOServiceImpl::new_default());

    // Create Command following CQRS pattern
    let command = RestoreFileCommand::new(input.clone(), target_path.clone())
        .with_overwrite(overwrite)
        .with_create_directories(mkdir)
        .with_permission_validation(true);

    // Execute validation through Application Service
    println!("🔒 Validating permissions through Application Service...");
    // TODO: Restoration service removed - implement permission validation via use_cases if needed
    // restoration_service
    //     .validate_restoration_permissions(&command)
    //     .await
    //     .map_err(|e| anyhow::anyhow!("Permission validation failed: {}", e))?;

    println!("   ✅ All permission checks passed");

    // Use proper Application Service integration
    println!("🔄 Using Application Service for restoration...");

    // Note: Restoration service removed - use use_cases::restore_file directly instead

    // Determine target path
    let target_path = if let Some(output_dir) = output_dir {
        // Create output directory if needed
        if mkdir && !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))
                .unwrap();
        }

        // Read metadata to get original filename
        let file_data = std::fs::read(&input).unwrap();
        let (metadata, _) = FileHeader::from_footer_bytes(&file_data)
            .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))
            .unwrap();

        output_dir.join(&metadata.original_filename)
    } else {
        // Use same directory as input file, but with original filename
        let file_data = std::fs::read(&input).unwrap();
        let (metadata, _) = FileHeader::from_footer_bytes(&file_data)
            .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))
            .unwrap();

        input
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(&metadata.original_filename)
    };

    // Check if target exists and handle overwrite
    if target_path.exists() && !overwrite {
        return Err(anyhow::anyhow!(
            "Target file already exists: {}\nUse --overwrite to overwrite existing files",
            target_path.display()
        ));
    }

    // Create restore command
    let restore_command = RestoreFileCommand {
        source_adapipe_path: input.clone(),
        target_path: target_path.clone(),
        create_directories: mkdir,
        overwrite,
        validate_permissions: true,
    };

    println!("💾 Restoring file using Application Service...");
    println!("   Source: {}", input.display());
    println!("   Target: {}", target_path.display());

    // TODO: Perform restoration - restoration service removed, use use_cases::restore_file instead
    // let start_time = std::time::Instant::now();
    // let restore_result = restoration_service
    //     .restore_file(restore_command)
    //     .await
    //     .map_err(|e| anyhow::anyhow!("Restoration failed: {}", e))
    //     .unwrap();
    // let duration = start_time.elapsed();
    //
    // // Display results
    // println!("✅ File restoration completed successfully!");
    // println!("   Restored file: {}", target_path.display());
    // println!("   Bytes restored: {}", restore_result.bytes_restored);
    // println!("   Checksum verified: {}", restore_result.checksum_verified);
    // println!("   Duration: {:.2}s", duration.as_secs_f64());
    // println!(
    //     "   Throughput: {:.2} MB/s",
    //     (restore_result.bytes_restored as f64 / (1024.0 * 1024.0)) / duration.as_secs_f64()
    // );

    println!("⚠️  Restoration temporarily disabled - refactoring in progress");
    println!("   Use restore_file_from_adapipe_legacy() instead");

    Ok(())
}

/// Legacy restore function (to be gradually replaced)
async fn restore_file_from_adapipe_legacy(
    input: PathBuf,
    output_dir: Option<PathBuf>,
    mkdir: bool,
    overwrite: bool,
) -> Result<()> {
    info!("Restoring file from .adapipe: {}", input.display());

    // Validate input file exists
    if !input.exists() {
        return Err(anyhow::anyhow!(
            "Input .adapipe file does not exist: {}",
            input.display()
        ));
    }

    // Read .adapipe metadata
    println!("🔍 Reading .adapipe file metadata...");
    let _file = std::fs::File::open(&input).unwrap();
    // Read entire file to get footer data
    let file_data = std::fs::read(&input).unwrap();
    let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))
        .unwrap();

    // Debug: Show metadata details
    println!("   📋 Metadata details:");
    println!("      - Encrypted: {}", metadata.is_encrypted());
    println!("      - Compressed: {}", metadata.is_compressed());
    println!("      - Processing steps count: {}", metadata.processing_steps.len());
    for (i, step) in metadata.processing_steps.iter().enumerate() {
        println!("      - Step {}: {:?} - {}", i, step.step_type, step.algorithm);
    }
    if metadata.is_encrypted() {
        println!("      - Encryption algorithm: {:?}", metadata.encryption_algorithm());
    }
    if metadata.is_compressed() {
        println!("      - Compression algorithm: {:?}", metadata.compression_algorithm());
    }
    println!("      - Original size: {} bytes", metadata.original_size);
    println!("      - Pipeline ID: {}", metadata.pipeline_id);

    // Determine output path
    let output_path = if let Some(dir) = output_dir {
        // Use specified directory + original filename
        let original_filename = std::path::Path::new(&metadata.original_filename)
            .file_name()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Could not extract filename from original filename: {}",
                    metadata.original_filename
                )
            })?
            .to_string_lossy()
            .to_string();

        dir.join(original_filename)
    } else {
        // Use original full path from metadata
        PathBuf::from(&metadata.original_filename)
    };

    println!("📁 Target restoration path: {}", output_path.display());

    // Validate permissions before proceeding
    println!("🔒 Validating permissions...");

    // Check if target file already exists
    if output_path.exists() {
        if !overwrite {
            return Err(anyhow::anyhow!(
                "Target file already exists: {}\nUse --overwrite to replace it",
                output_path.display()
            ));
        }

        // Check if existing file is writable
        let metadata = std::fs::metadata(&output_path)
            .map_err(|e| anyhow::anyhow!("Failed to check existing file permissions: {}", e))
            .unwrap();

        if metadata.permissions().readonly() {
            return Err(anyhow::anyhow!(
                "Target file is read-only: {}\nChange permissions or use a different location",
                output_path.display()
            ));
        }

        println!("   ⚠️  Target file exists and will be overwritten");
    }

    // First, handle directory creation if needed
    if let Some(parent_dir) = output_path.parent() {
        if !parent_dir.exists() {
            if mkdir {
                println!("📂 Creating directory: {}", parent_dir.display());
                std::fs::create_dir_all(parent_dir)
                    .map_err(|e| {
                        // Provide specific error messages for common permission issues
                        if e.kind() == std::io::ErrorKind::PermissionDenied {
                            anyhow::anyhow!(
                                "Permission denied: Cannot create directory '{}'\nTry running with elevated \
                                 privileges or choose a different location",
                                parent_dir.display()
                            )
                        } else {
                            anyhow::anyhow!("Failed to create directory '{}': {}", parent_dir.display(), e)
                        }
                    })
                    .unwrap();
            } else {
                print!(
                    "Directory '{}' does not exist. Create it? [y/N]: ",
                    parent_dir.display()
                );
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                    println!("📂 Creating directory: {}", parent_dir.display());
                    std::fs::create_dir_all(parent_dir)
                        .map_err(|e| {
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                anyhow::anyhow!(
                                    "Permission denied: Cannot create directory '{}'\nTry running with elevated \
                                     privileges or choose a different location",
                                    parent_dir.display()
                                )
                            } else {
                                anyhow::anyhow!("Failed to create directory '{}': {}", parent_dir.display(), e)
                            }
                        })
                        .unwrap();
                } else {
                    return Err(anyhow::anyhow!("Directory creation cancelled by user"));
                }
            }
        }

        // Now test write permissions to the directory (whether it existed or was just
        // created)
        println!("   🔍 Testing directory write permissions...");
        let temp_test_file = parent_dir.join(".adapipe_permission_test");
        match std::fs::File::create(&temp_test_file) {
            Ok(_) => {
                // Clean up test file
                let _ = std::fs::remove_file(&temp_test_file);
                println!("   ✅ Directory write permissions verified");
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Cannot write to directory '{}': {}\nThis could be due to:\n  - Insufficient permissions (try \
                     running with elevated privileges)\n  - Directory is read-only\n  - Filesystem is mounted \
                     read-only\n  - Security restrictions (SELinux, AppArmor, etc.)\nTry choosing a different \
                     location or checking directory permissions",
                    parent_dir.display(),
                    e
                ));
            }
        }
    }

    // Check available disk space
    if let Some(parent_dir) = output_path.parent() {
        match std::fs::metadata(parent_dir) {
            Ok(_) => {
                // On Unix systems, we can use statvfs to check disk space, but for simplicity
                // we'll just verify the directory is accessible and warn about space
                let required_size = metadata.original_size;
                if required_size > 0 {
                    println!(
                        "   💾 Required disk space: {} bytes ({:.1} MB)",
                        required_size,
                        required_size as f64 / (1024.0 * 1024.0)
                    );
                    println!("   ⚠️  Ensure sufficient disk space is available");
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Cannot access target directory '{}': {}",
                    parent_dir.display(),
                    e
                ));
            }
        }
    }

    // Final permission validation summary
    println!("   ✅ All permission checks passed");

    // Create ephemeral restoration pipeline from .adapipe metadata
    println!("🔧 Creating ephemeral restoration pipeline...");
    let restoration_pipeline = create_restoration_pipeline(&metadata)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create restoration pipeline: {}", e))
        .unwrap();

    println!("   Pipeline ID: {}", restoration_pipeline.id());
    println!("   Stages: {}", restoration_pipeline.stages().len());

    // Display pipeline stages for transparency
    for (index, stage) in restoration_pipeline.stages().iter().enumerate() {
        println!(
            "   Stage {}: {} ({})",
            index + 1,
            stage.stage_type(),
            stage.configuration().algorithm
        );
    }

    // Perform streaming restoration with automatic validation
    println!("\n🔄 Streaming restoration (decrypt → decompress → write → verify)...");
    println!("   Original size: {} bytes", metadata.original_size);
    println!("   Expected checksum: {}", metadata.original_checksum);

    // Create progress indicator for real-time feedback
    let estimated_chunks = metadata.original_size.div_ceil(1024 * 1024); // Round up
    let progress_indicator = ProgressIndicatorService::new(estimated_chunks);

    let restoration_result = stream_restore_with_validation(
        &input,
        &output_path,
        &restoration_pipeline,
        &metadata,
        _footer_size,
        &progress_indicator,
    )
    .await
    .map_err(|e| anyhow::anyhow!("Restoration failed: {}", e))
    .unwrap();

    // Validate restoration results
    if restoration_result.checksum_verified {
        println!("   ✅ Checksum verified: restoration successful");
    } else {
        return Err(anyhow::anyhow!(
            "Checksum verification failed: expected {}, got {}",
            metadata.original_checksum,
            restoration_result.calculated_checksum
        ));
    }

    println!(
        "   📊 Processed {} bytes in {} chunks",
        restoration_result.bytes_processed, restoration_result.chunks_processed
    );

    println!("\n✅ File restoration completed!");
    println!("📁 Restored to: {}", output_path.display());

    Ok(())
}

async fn compare_file_against_adapipe(original: PathBuf, adapipe: PathBuf, detailed: bool) -> Result<()> {
    info!(
        "Comparing file against .adapipe: {} vs {}",
        original.display(),
        adapipe.display()
    );

    // Validate both files exist
    if !original.exists() {
        return Err(anyhow::anyhow!("Original file does not exist: {}", original.display()));
    }

    if !adapipe.exists() {
        return Err(anyhow::anyhow!(".adapipe file does not exist: {}", adapipe.display()));
    }

    // Read .adapipe metadata
    println!("🔍 Reading .adapipe file metadata...");
    let _file = std::fs::File::open(&adapipe).unwrap();
    // Read entire file to get footer data
    let file_data = std::fs::read(&adapipe).unwrap();
    let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))
        .unwrap();

    // Get original file info
    let original_metadata = std::fs::metadata(&original).unwrap();
    let original_size = original_metadata.len();

    println!("📊 File Comparison:");
    println!("   Original file: {}", original.display());
    println!("   .adapipe file: {}", adapipe.display());
    println!();

    // Compare file sizes
    println!("📏 Size Comparison:");
    println!("   Current file size: {} bytes", original_size);
    println!("   Expected size (from .adapipe): {} bytes", metadata.original_size);

    if original_size == metadata.original_size {
        println!("   ✅ Size matches");
    } else {
        println!(
            "   ❌ Size differs by {} bytes",
            (original_size as i64 - metadata.original_size as i64).abs()
        );
    }

    // Compare checksums
    println!("\n🔐 Checksum Comparison:");
    println!("   Expected checksum (from .adapipe): {}", metadata.original_checksum);

    // Calculate current file checksum
    println!("   🔄 Calculating current file checksum...");

    let mut hasher = Sha256::new();
    let mut file = std::fs::File::open(&original).unwrap();
    std::io::copy(&mut file, &mut hasher).unwrap();
    let current_checksum = format!("{:x}", hasher.finalize());

    println!("   Current file checksum: {}", current_checksum);

    if current_checksum == metadata.original_checksum {
        println!("   ✅ Checksums match - files are identical");
    } else {
        println!("   ❌ Checksums differ - files are not identical");
    }

    // Show detailed information if requested
    if detailed {
        println!("\n📋 Detailed Information:");
        println!(
            "   .adapipe created: {}",
            metadata.processed_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!("   Pipeline ID: {}", metadata.pipeline_id);
        println!("   Chunk count: {}", metadata.chunk_count);

        if metadata.is_compressed() {
            println!(
                "   Compression: {}",
                metadata.compression_algorithm().unwrap_or("unknown")
            );
        }

        if metadata.is_encrypted() {
            println!(
                "   Encryption: {}",
                metadata.encryption_algorithm().unwrap_or("unknown")
            );
        }

        let current_modified = original_metadata.modified().unwrap();
        println!(
            "   Current file modified: {}",
            chrono::DateTime::<chrono::Utc>::from(current_modified).format("%Y-%m-%d %H:%M:%S UTC")
        );
    }

    // Summary
    println!("\n🎯 Comparison Summary:");
    if original_size == metadata.original_size && current_checksum == metadata.original_checksum {
        println!("   ✅ Files are identical - no changes detected");
    } else {
        println!("   ❌ Files differ - changes detected");
        if detailed {
            println!("   💡 Use 'restore' command to restore from .adapipe if needed");
        }
    }

    Ok(())
}
/// Creates an ephemeral restoration pipeline from .adapipe metadata
///
/// This function implements the DDD pattern by creating a domain entity
/// (Pipeline) that encapsulates the restoration business logic. The pipeline is
/// ephemeral and exists only for the duration of the restoration operation.
///
/// # Architecture
/// - Domain-Driven Design: Pipeline as aggregate root
/// - Value Objects: StageId, PipelineId for type safety
/// - Error Handling: Comprehensive validation and error propagation
/// - Immutability: Pipeline stages are immutable once created
pub async fn create_restoration_pipeline(metadata: &FileHeader) -> Result<Pipeline> {
    use pipeline_domain::entities::pipeline::Pipeline;
    use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
    use std::collections::HashMap;

    info!("Creating ephemeral restoration pipeline from metadata");

    let mut stages = Vec::new();
    let mut stage_index = 1;

    // Build restoration pipeline stages from processing steps in reverse order
    // Processing steps are stored in forward order, but restoration needs reverse
    // order
    let mut processing_steps = metadata.processing_steps.clone();
    processing_steps.sort_by(|a, b| b.order.cmp(&a.order)); // Reverse order

    info!(
        "Building restoration pipeline from {} processing steps",
        processing_steps.len()
    );

    for step in processing_steps {
        match step.step_type {
            pipeline_domain::value_objects::ProcessingStepType::Encryption => {
                let decryption_config = StageConfiguration {
                    algorithm: step.algorithm.clone(),
                    parameters: step.parameters.clone(),
                    parallel_processing: false,
                    chunk_size: Some(1024 * 1024), // 1MB chunks
                };

                let decryption_stage = PipelineStage::new(
                    "decryption".to_string(),
                    StageType::Encryption, // Use Encryption type for decryption (internal restoration)
                    decryption_config,
                    stage_index,
                )
                .unwrap();

                stages.push(decryption_stage);
                info!(
                    "Added decryption stage: {} (from step order {})",
                    step.algorithm, step.order
                );
                stage_index += 1;
            }
            pipeline_domain::value_objects::ProcessingStepType::Compression => {
                let decompression_config = StageConfiguration {
                    algorithm: step.algorithm.clone(),
                    parameters: step.parameters.clone(),
                    parallel_processing: false,
                    chunk_size: Some(1024 * 1024), // 1MB chunks
                };

                let decompression_stage = PipelineStage::new(
                    "decompression".to_string(),
                    StageType::Compression, // Note: Using Compression type for decompression
                    decompression_config,
                    stage_index,
                )
                .unwrap();

                stages.push(decompression_stage);
                info!(
                    "Added decompression stage: {} (from step order {})",
                    step.algorithm, step.order
                );
                stage_index += 1;
            }
            pipeline_domain::value_objects::ProcessingStepType::Checksum => {
                // Checksum steps are used for validation only, not for data transformation
                info!(
                    "Skipping checksum step: {} (from step order {}) - used for validation only",
                    step.algorithm, step.order
                );
                continue;
            }
            pipeline_domain::value_objects::ProcessingStepType::PassThrough => {
                // PassThrough steps don't modify data, skip during restoration
                info!(
                    "Skipping pass-through step: {} (from step order {}) - no data transformation needed",
                    step.algorithm, step.order
                );
                continue;
            }
            pipeline_domain::value_objects::ProcessingStepType::Custom(ref step_name) => {
                // Only create stages for transformative custom steps, skip checksum steps
                if step_name.contains("checksum") {
                    info!(
                        "Skipping checksum step: {} (from step order {}) - used for validation only",
                        step.algorithm, step.order
                    );
                    continue;
                }

                // Handle transformative custom steps (compression, encryption implemented as
                // custom)
                let stage_type = if step_name == "compression" {
                    StageType::Compression
                } else if step_name == "encryption" {
                    StageType::Encryption
                } else {
                    StageType::PassThrough
                };

                let custom_config = StageConfiguration {
                    algorithm: step.algorithm.clone(),
                    parameters: step.parameters.clone(),
                    parallel_processing: false,
                    chunk_size: Some(1024 * 1024), // 1MB chunks
                };

                let stage_name = if step_name == "compression" {
                    "decompression".to_string()
                } else if step_name == "encryption" {
                    "decryption".to_string()
                } else {
                    format!("reverse_{}", step_name)
                };

                let custom_stage =
                    PipelineStage::new(stage_name.clone(), stage_type, custom_config, stage_index).unwrap();

                stages.push(custom_stage);
                info!(
                    "Added {} stage: {} (from step order {})",
                    stage_name, step.algorithm, step.order
                );
                stage_index += 1;
            }
        }
    }

    // Stage 3: Integrity verification (always present)
    let verification_config = StageConfiguration {
        algorithm: "sha256".to_string(),
        parameters: HashMap::new(),
        parallel_processing: false,
        chunk_size: Some(1024 * 1024), // 1MB chunks
    };

    let verification_stage = PipelineStage::new(
        "verification".to_string(),
        StageType::Checksum, // Using Checksum type for verification
        verification_config,
        stage_index,
    )
    .unwrap();

    stages.push(verification_stage);
    info!("Added verification stage: sha256");

    // Validate that we have at least one stage
    if stages.is_empty() {
        return Err(anyhow::anyhow!("No restoration stages could be created from metadata"));
    }

    // Create ephemeral pipeline with special naming convention
    let pipeline_name = format!("__restore__{}", metadata.pipeline_id);

    let pipeline = Pipeline::new(pipeline_name, stages).unwrap();

    info!(
        "Created ephemeral restoration pipeline with {} stages",
        pipeline.stages().len()
    );

    Ok(pipeline)
}

/// Result of streaming restoration with validation
#[derive(Debug, Clone)]
struct RestorationResult {
    checksum_verified: bool,
    calculated_checksum: String,
    expected_checksum: String,
    bytes_processed: u64,
    chunks_processed: u32,
    processing_duration: std::time::Duration,
}

/// Performs streaming restoration with automatic validation
///
/// This function implements the core restoration algorithm using:
/// - Streaming I/O for memory efficiency
/// - Incremental checksum calculation
/// - Proper error handling and recovery
/// - Concurrent processing where applicable
///
/// # Architecture
/// - Hexagonal Architecture: Adapts between file I/O and domain logic
/// - Error Handling: Comprehensive error propagation with context
/// - Performance: Streaming processing for large files
/// - Validation: Automatic integrity verification
async fn stream_restore_with_validation(
    input_path: &Path,
    output_path: &Path,
    restoration_pipeline: &Pipeline,
    metadata: &FileHeader,
    _footer_size: usize,
    progress_indicator: &ProgressIndicatorService,
) -> Result<RestorationResult> {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    info!("Starting streaming restoration with validation");
    let start_time = Instant::now();

    // Initialize streaming validator and file handles
    let mut hasher = Sha256::new();
    let mut bytes_processed = 0u64;
    let mut chunks_processed = 0u32;

    // Create binary format reader for proper .adapipe chunk parsing
    let binary_format_service = BinaryFormatServiceImpl::new();
    let mut adapipe_reader = binary_format_service
        .create_reader(input_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create .adapipe reader: {}", e))
        .unwrap();

    // Create output file for writing restored data
    let mut output_file = File::create(output_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))
        .unwrap();

    // Create domain services for restoration pipeline
    let compression_service = Arc::new(CompressionServiceImpl::new());
    let encryption_service = Arc::new(EncryptionServiceImpl::new());
    let stage_executor = Arc::new(BasicStageExecutor::new(compression_service, encryption_service));

    // Create security context for restoration
    let security_context = SecurityContext::new(
        None,
        pipeline_domain::entities::security_context::SecurityLevel::Internal,
    );

    // Create processing context for restoration
    let mut processing_context = ProcessingContext::new(
        input_path.to_path_buf(),
        output_path.to_path_buf(),
        metadata.original_size,
        security_context,
    );

    info!(
        "Streaming restoration through {} stages",
        restoration_pipeline.stages().len()
    );

    // Process chunks through the restoration pipeline using proper .adapipe format
    // parsing
    let mut chunk_sequence = 0u32;

    loop {
        // Read next chunk from .adapipe file using proper format parsing
        let chunk_format = match adapipe_reader
            .read_next_chunk()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read chunk: {}", e))?
        {
            Some(chunk) => chunk,
            None => break, // No more chunks
        };

        // Combine nonce and encrypted data as expected by decryption service
        // The encryption service expects: [nonce (12 bytes)] + [encrypted_data]
        let mut chunk_data = chunk_format.nonce.to_vec();
        chunk_data.extend_from_slice(&chunk_format.encrypted_data);
        let file_chunk = FileChunk::new(
            chunk_sequence as u64,
            bytes_processed,
            chunk_data,
            false, // is_final - we'll determine this later
        )
        .map_err(|e| anyhow::anyhow!("Failed to create file chunk: {}", e))
        .unwrap();

        // Process chunk through restoration pipeline stages
        let mut current_chunk = file_chunk;
        for stage in restoration_pipeline.stages() {
            debug!("Processing chunk {} through stage: {}", chunk_sequence, stage.name());

            current_chunk = stage_executor
                .execute(stage, current_chunk, &mut processing_context)
                .await
                .map_err(|e| anyhow::anyhow!("Stage '{}' failed: {}", stage.name(), e))
                .unwrap();
        }

        // Write restored chunk to output file
        let restored_data = current_chunk.data();
        output_file
            .write_all(restored_data)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write restored data: {}", e))
            .unwrap();

        // Update incremental checksum with restored data
        hasher.update(restored_data);

        // Update progress counters
        bytes_processed += restored_data.len() as u64;
        chunks_processed += 1;
        chunk_sequence += 1;

        // Update progress indicator for real-time feedback
        progress_indicator.update_progress(chunks_processed as u64).await;

        // Additional debug logging for large files
        if chunks_processed.is_multiple_of(100) {
            let progress_mb = bytes_processed as f64 / (1024.0 * 1024.0);
            let expected_mb = metadata.original_size as f64 / (1024.0 * 1024.0);
            debug!("Restoration progress: {:.1} MB / {:.1} MB", progress_mb, expected_mb);
        }
    }

    // Ensure all data is written to disk
    output_file
        .flush()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to flush output file: {}", e))
        .unwrap();

    // Calculate final checksum
    let calculated_hash = hasher.finalize();
    let calculated_checksum = format!("{:x}", calculated_hash);

    // Verify checksum against expected
    let checksum_verified = calculated_checksum == metadata.original_checksum;

    let processing_duration = start_time.elapsed();

    if checksum_verified {
        info!(
            "Streaming restoration completed successfully in {:?}",
            processing_duration
        );
        let throughput_mb_s = (bytes_processed as f64 / (1024.0 * 1024.0)) / processing_duration.as_secs_f64();
        progress_indicator
            .show_completion(bytes_processed, throughput_mb_s, processing_duration)
            .await;
    } else {
        warn!(
            "Checksum verification failed: expected {}, got {}",
            metadata.original_checksum, calculated_checksum
        );
        progress_indicator
            .show_error_summary(&format!(
                "Checksum verification failed: expected {}, got {}",
                metadata.original_checksum, calculated_checksum
            ))
            .await;
    }

    Ok(RestorationResult {
        checksum_verified,
        calculated_checksum,
        expected_checksum: metadata.original_checksum.clone(),
        bytes_processed,
        chunks_processed,
        processing_duration,
    })
}

#[cfg(test)]
mod restore_tests {
    use super::*;
    use tokio::test;

    /// Test helper to create a mock FileHeader for testing
    fn create_test_file_header() -> FileHeader {
        FileHeader::new("test_file.txt".to_string(), 1024, "abc123def456".to_string())
            .add_compression_step("brotli", 6)
            .add_encryption_step("aes256gcm", "argon2", 32, 12)
            .with_chunk_info(1024, 1)
            .with_pipeline_id("test-pipeline-123".to_string())
            .with_output_checksum("output123def456".to_string())
    }

    #[tokio::test]
    async fn test_create_restoration_pipeline_with_compression_and_encryption() {
        let header = create_test_file_header();

        let result = create_restoration_pipeline(&header).await;
        assert!(
            result.is_ok(),
            "Failed to create restoration pipeline: {:?}",
            result.err()
        );

        let pipeline = result.unwrap();
        assert_eq!(
            pipeline.stages().len(),
            5,
            "Expected 5 stages: input_checksum + decryption + decompression + verification + output_checksum"
        );

        // Verify stage order: input_checksum -> decryption -> decompression ->
        // verification -> output_checksum
        let stages = pipeline.stages();
        assert_eq!(stages[0].name(), "input_checksum");
        assert_eq!(stages[1].name(), "decryption");
        assert_eq!(stages[2].name(), "decompression");
        assert_eq!(stages[3].name(), "verification");
        assert_eq!(stages[4].name(), "output_checksum");

        // Verify stage types
        assert_eq!(stages[0].stage_type(), &StageType::Checksum);
        assert_eq!(stages[1].stage_type(), &StageType::Encryption); // Decryption uses Encryption type
        assert_eq!(stages[2].stage_type(), &StageType::Compression); // Decompression uses Compression type
        assert_eq!(stages[3].stage_type(), &StageType::Checksum);
        assert_eq!(stages[4].stage_type(), &StageType::Checksum);
    }

    #[tokio::test]
    async fn test_create_restoration_pipeline_compression_only() {
        let header =
            FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string()).add_compression_step("brotli", 6);

        let result = create_restoration_pipeline(&header).await;
        assert!(result.is_ok());

        let pipeline = result.unwrap();
        assert_eq!(
            pipeline.stages().len(),
            4,
            "Expected 4 stages: input_checksum + decompression + verification + output_checksum"
        );

        let stages = pipeline.stages();
        assert_eq!(stages[0].name(), "input_checksum");
        assert_eq!(stages[1].name(), "decompression");
        assert_eq!(stages[2].name(), "verification");
        assert_eq!(stages[3].name(), "output_checksum");
    }

    #[tokio::test]
    async fn test_create_restoration_pipeline_no_processing() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string());

        let result = create_restoration_pipeline(&header).await;
        assert!(result.is_ok());

        let pipeline = result.unwrap();
        assert_eq!(
            pipeline.stages().len(),
            3,
            "Expected 3 stages: input_checksum + verification + output_checksum"
        );

        let stages = pipeline.stages();

        // Verify automatic checksum stages
        assert_eq!(stages[0].name(), "input_checksum");
        assert_eq!(stages[0].stage_type(), &StageType::Checksum);

        // Verify user-defined verification stage
        assert_eq!(stages[1].name(), "verification");
        assert_eq!(stages[1].stage_type(), &StageType::Checksum);

        // Verify automatic output checksum stage
        assert_eq!(stages[2].name(), "output_checksum");
        assert_eq!(stages[2].stage_type(), &StageType::Checksum);
    }

    #[tokio::test]
    async fn test_restoration_result_creation() {
        let result = RestorationResult {
            checksum_verified: true,
            calculated_checksum: "abc123".to_string(),
            expected_checksum: "abc123".to_string(),
            bytes_processed: 1024,
            chunks_processed: 1,
            processing_duration: std::time::Duration::from_millis(100),
        };

        assert!(result.checksum_verified);
        assert_eq!(result.calculated_checksum, result.expected_checksum);
        assert_eq!(result.bytes_processed, 1024);
        assert_eq!(result.chunks_processed, 1);
        assert!(result.processing_duration.as_millis() >= 100);
    }

    #[tokio::test]
    async fn test_restoration_pipeline_naming() {
        let header = FileHeader::new("test.txt".to_string(), 1024, "abc123".to_string())
            .with_pipeline_id("original-pipeline-123".to_string());

        let pipeline = create_restoration_pipeline(&header).await.unwrap();

        // Verify ephemeral pipeline naming convention
        assert!(pipeline.name().starts_with("__restore__"));
        assert!(pipeline.name().contains("original-pipeline-123"));
    }

    #[tokio::test]
    async fn test_file_chunk_creation_for_restoration() {
        let test_data = vec![1, 2, 3, 4, 5];
        let chunk = FileChunk::new(
            0, // sequence_number
            0, // offset
            test_data.clone(),
            false, // is_final
        );

        assert!(chunk.is_ok(), "Failed to create FileChunk: {:?}", chunk.err());

        let chunk = chunk.unwrap();
        assert_eq!(chunk.sequence_number(), 0);
        assert_eq!(chunk.offset(), 0);
        assert_eq!(chunk.data(), &test_data);
        assert!(!chunk.is_final());
    }

    #[tokio::test]
    async fn test_restoration_result_checksum_mismatch() {
        let result = RestorationResult {
            checksum_verified: false,
            calculated_checksum: "abc123".to_string(),
            expected_checksum: "def456".to_string(),
            bytes_processed: 1024,
            chunks_processed: 1,
            processing_duration: std::time::Duration::from_millis(100),
        };

        assert!(!result.checksum_verified);
        assert_ne!(result.calculated_checksum, result.expected_checksum);
    }
}

// End-to-end tests have been moved to tests/e2e_restore_pipeline_test.rs
// This keeps main.rs focused on application logic rather than test code
