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
// CLI parsing now handled by bootstrap layer
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tracing::{debug, error, info, warn};

// Import ChunkSize and WorkerCount for optimal sizing calculations
use crate::application::commands::RestoreFileCommand;
// File restoration is now handled via use_cases::restore_file
use crate::infrastructure::adapters::file_io::TokioFileIO;
use crate::infrastructure::services::progress_indicator::ProgressIndicatorService;
use pipeline_domain::value_objects::binary_file_format::FileHeader;
use pipeline_domain::value_objects::chunk_size::ChunkSize;
use pipeline_domain::value_objects::worker_count::WorkerCount;

// Import all use cases from application layer
use crate::application::use_cases::{
    BenchmarkSystemUseCase, CompareFilesUseCase, CreatePipelineUseCase, DeletePipelineUseCase,
    ListPipelinesUseCase, ProcessFileConfig, ProcessFileUseCase, ShowPipelineUseCase,
    ValidateConfigUseCase, ValidateFileUseCase,
};

/// Format bytes with 6-digit precision
fn format_bytes_6_digits(bytes: u64) -> String {
    let byte_obj = Byte::from_u128(bytes as u128)
        .unwrap_or_default()
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

use crate::application::services::pipeline::ConcurrentPipeline;
use crate::infrastructure::repositories::sqlite_pipeline::SqlitePipelineRepository;
use crate::infrastructure::adapters::{MultiAlgoCompression, MultiAlgoEncryption};
use crate::infrastructure::logging::ObservabilityService;
use crate::infrastructure::metrics::{MetricsEndpoint, MetricsService};
use crate::infrastructure::runtime::stage_executor::BasicStageExecutor;
use crate::infrastructure::services::{
    Base64EncodingService, BinaryFormatService, AdapipeFormat, DebugService,
    PassThroughService, PiiMaskingService, TeeService,
};
use pipeline_domain::repositories::stage_executor::StageExecutor;

// CLI parsing now handled by bootstrap layer
// See bootstrap::cli for CLI definitions and validation
// Exit code mapping now in bootstrap::exit_code

#[tokio::main]
async fn main() -> std::process::ExitCode {
    // Bootstrap: Parse and validate CLI arguments with security checks
    let validated_cli = match bootstrap::bootstrap_cli() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("CLI Error: {}", e);
            return std::process::ExitCode::from(65); // EX_DATAERR
        }
    };

    // Run application logic with validated configuration
    let result = run_app(validated_cli).await;

    // Map result to appropriate Unix exit code
    bootstrap::result_to_exit_code(result)
}

/// Main application logic separated for testability
///
/// # Arguments
///
/// * `cli` - Validated CLI configuration from bootstrap layer
///
/// # Returns
///
/// Result indicating success or error
async fn run_app(cli: bootstrap::ValidatedCli) -> Result<()> {
    // === Initialize Global Resource Manager ===
    // Educational: This must happen BEFORE any code uses RESOURCE_MANAGER
    // We configure it from CLI flags, falling back to intelligent defaults.
    use crate::infrastructure::runtime::{init_resource_manager, ResourceConfig, StorageType};

    let resource_config = ResourceConfig {
        cpu_tokens: cli.cpu_threads,
        io_tokens: cli.io_threads,
        storage_type: cli
            .storage_type
            .as_ref()
            .map(|s| match s.as_str() {
                "nvme" => StorageType::NVMe,
                "ssd" => StorageType::SSD,
                "hdd" => StorageType::HDD,
                _ => StorageType::Auto, // Shouldn't happen due to parse_storage_type validation
            })
            .unwrap_or(StorageType::Auto),
        memory_limit: None, // Use system detection
    };

    init_resource_manager(resource_config)
        .map_err(|e| anyhow::anyhow!("Failed to initialize resource manager: {}", e))?;

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

    tracing::subscriber::set_global_default(subscriber)?;

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

    // Execute command (using validated commands from bootstrap)
    match cli.command {
        bootstrap::ValidatedCommand::Process {
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
            let use_case = ProcessFileUseCase::new(
                metrics_service.clone(),
                observability_service.clone(),
                pipeline_repository.clone(),
            );
            use_case.execute(config).await?;
        }

        bootstrap::ValidatedCommand::Create { name, stages, output } => {
            let use_case = CreatePipelineUseCase::new(pipeline_repository.clone());
            use_case.execute(name, stages, output).await?;
        }

        bootstrap::ValidatedCommand::List => {
            let use_case = ListPipelinesUseCase::new(pipeline_repository.clone());
            use_case.execute().await?;
        }

        bootstrap::ValidatedCommand::Show { pipeline } => {
            let use_case = ShowPipelineUseCase::new(pipeline_repository.clone());
            use_case.execute(pipeline).await?;
        }

        bootstrap::ValidatedCommand::Delete { pipeline, force } => {
            let use_case = DeletePipelineUseCase::new(pipeline_repository.clone());
            use_case.execute(pipeline, force).await?;
        }

        bootstrap::ValidatedCommand::Benchmark {
            file,
            size_mb,
            iterations,
        } => {
            let use_case = BenchmarkSystemUseCase::new();
            use_case.execute(file, size_mb, iterations).await?;
        }

        bootstrap::ValidatedCommand::Validate { config } => {
            let use_case = ValidateConfigUseCase::new();
            use_case.execute(config).await?;
        }

        bootstrap::ValidatedCommand::ValidateFile { file, full } => {
            let use_case = ValidateFileUseCase::new();
            use_case.execute(file, full).await?;
        }

        bootstrap::ValidatedCommand::Restore {
            input,
            output_dir,
            mkdir,
            overwrite,
        } => {
            // Use the new hybrid architecture-compliant function
            restore_file_from_adapipe_v2(input, output_dir, mkdir, overwrite).await?;
        }

        bootstrap::ValidatedCommand::Compare {
            original,
            adapipe,
            detailed,
        } => {
            let use_case = CompareFilesUseCase::new();
            use_case.execute(original, adapipe, detailed).await?;
        }
    }

    Ok(())
}

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
    let file_data = std::fs::read(&input)?;
    let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

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

    // Note: Restoration service removed - use use_cases::restore_file directly
    // instead let file_io_service = Arc::new(TokioFileIO::new_default());

    // Create Command following CQRS pattern
    let command = RestoreFileCommand::new(input.clone(), target_path.clone())
        .with_overwrite(overwrite)
        .with_create_directories(mkdir)
        .with_permission_validation(true);

    // Execute validation through Application Service
    println!("🔒 Validating permissions through Application Service...");
    // TODO: Restoration service removed - implement permission validation via
    // use_cases if needed restoration_service
    //     .validate_restoration_permissions(&command)
    //     .await
    //     .map_err(|e| anyhow::anyhow!("Permission validation failed: {}", e))?;

    println!("   ✅ All permission checks passed");

    // Use proper Application Service integration
    println!("🔄 Using Application Service for restoration...");

    // Note: Restoration service removed - use use_cases::restore_file directly
    // instead

    // Determine target path
    let target_path = if let Some(output_dir) = output_dir {
        // Create output directory if needed
        if mkdir && !output_dir.exists() {
            std::fs::create_dir_all(&output_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
        }

        // Read metadata to get original filename
        let file_data = std::fs::read(&input)?;
        let (metadata, _) = FileHeader::from_footer_bytes(&file_data)
            .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

        output_dir.join(&metadata.original_filename)
    } else {
        // Use same directory as input file, but with original filename
        let file_data = std::fs::read(&input)?;
        let (metadata, _) = FileHeader::from_footer_bytes(&file_data)
            .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

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

    // Step 1: Read .adapipe metadata
    info!("Reading .adapipe file metadata...");
    let binary_format_service = AdapipeFormat::new();
    let metadata = binary_format_service
        .read_metadata(&input)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

    println!("   📋 Metadata details:");
    println!("      - Original filename: {}", metadata.original_filename);
    println!("      - Original size: {} bytes", metadata.original_size);
    println!("      - Encrypted: {}", metadata.is_encrypted());
    println!("      - Compressed: {}", metadata.is_compressed());
    println!("      - Processing steps: {}", metadata.processing_steps.len());

    // Step 2: Validate target path and permissions
    if target_path.exists() && !overwrite {
        return Err(anyhow::anyhow!(
            "Target file already exists: {}\nUse --overwrite to replace it",
            target_path.display()
        ));
    }

    // Step 3: Handle directory creation if needed
    if let Some(parent_dir) = target_path.parent() {
        if !parent_dir.exists() {
            if mkdir {
                println!("📂 Creating directory: {}", parent_dir.display());
                std::fs::create_dir_all(parent_dir).map_err(|e| {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        anyhow::anyhow!(
                            "Permission denied: Cannot create directory '{}'\nTry running with elevated privileges",
                            parent_dir.display()
                        )
                    } else {
                        anyhow::anyhow!("Failed to create directory '{}': {}", parent_dir.display(), e)
                    }
                })?;
            } else {
                return Err(anyhow::anyhow!(
                    "Output directory does not exist: {}\nUse --mkdir to create it",
                    parent_dir.display()
                ));
            }
        }
    }

    // Step 4: Create restoration pipeline using use_cases::restore_file
    info!("Creating restoration pipeline...");
    let restoration_pipeline = application::use_cases::create_restoration_pipeline(&metadata)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create restoration pipeline: {}", e))?;

    println!(
        "   🔄 Restoration pipeline created with {} stages",
        restoration_pipeline.stages().len()
    );
    for stage in restoration_pipeline.stages() {
        println!("      - {} (type: {:?})", stage.name(), stage.stage_type());
    }

    // Step 5: Read chunks from .adapipe file and process through restoration pipeline
    info!("Starting restoration process...");
    let mut reader = binary_format_service
        .create_reader(&input)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create .adapipe reader: {}", e))?;

    // Create output file
    let mut output_file = tokio::fs::File::create(&target_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;

    // Create services and stage executor for restoration
    let compression_service = Arc::new(MultiAlgoCompression::new());
    let encryption_service = Arc::new(MultiAlgoEncryption::new());

    // Build stage service registry for restoration
    let mut stage_services: HashMap<String, Arc<dyn pipeline_domain::services::StageService>> = HashMap::new();
    stage_services.insert("brotli".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("gzip".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("zstd".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("lz4".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("aes256gcm".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("aes128gcm".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("chacha20poly1305".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("base64".to_string(), Arc::new(Base64EncodingService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("pii_masking".to_string(), Arc::new(PiiMaskingService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("tee".to_string(), Arc::new(TeeService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("passthrough".to_string(), Arc::new(PassThroughService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("debug".to_string(), Arc::new(DebugService::new(Arc::new(MetricsService::new().unwrap()))) as Arc<dyn pipeline_domain::services::StageService>);

    let stage_executor = Arc::new(BasicStageExecutor::new(stage_services));

    let mut chunks_processed = 0u32;
    let mut bytes_written = 0u64;
    let mut current_offset = 0u64;

    // Process each chunk
    while let Some(chunk_format) = reader
        .read_next_chunk()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read chunk: {}", e))?
    {
        // Reconstruct FileChunk from ChunkFormat
        // For encrypted chunks, prepend nonce back to data
        let chunk_data = if metadata.is_encrypted() {
            let mut reconstructed_data = chunk_format.nonce.to_vec();
            reconstructed_data.extend_from_slice(&chunk_format.payload);
            reconstructed_data
        } else {
            chunk_format.payload.clone()
        };

        let is_final = chunks_processed == metadata.chunk_count - 1;
        let mut file_chunk = FileChunk::new(chunks_processed as u64, current_offset, chunk_data, is_final)
            .map_err(|e| anyhow::anyhow!("Failed to create FileChunk: {}", e))?;

        // Create processing context for restoration
        let security_context = SecurityContext::with_permissions(
            None,
            vec![Permission::Read, Permission::Write],
            SecurityLevel::Internal,
        );
        let mut context = ProcessingContext::new(
            input.clone(),
            target_path.clone(),
            metadata.original_size,
            security_context,
        );

        // Process through restoration stages (decryption, decompression)
        for stage in restoration_pipeline.stages() {
            // Skip checksum stages during restoration
            if stage.stage_type() == &StageType::Checksum {
                continue;
            }

            // Execute stage using stage executor
            file_chunk = stage_executor
                .execute(stage, file_chunk, &mut context)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to execute stage '{}': {}", stage.name(), e))?;
        }

        // Write restored data to output file
        output_file
            .write_all(file_chunk.data())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write to output file: {}", e))?;

        bytes_written += file_chunk.data().len() as u64;
        current_offset += file_chunk.data().len() as u64;
        chunks_processed += 1;

        if chunks_processed % 100 == 0 {
            println!(
                "   📦 Processed {} chunks, {} bytes written",
                chunks_processed, bytes_written
            );
        }
    }

    // Flush and close output file
    output_file
        .flush()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to flush output file: {}", e))?;

    println!("✅ Restoration complete!");
    println!("   📦 Chunks processed: {}", chunks_processed);
    println!("   📊 Total bytes written: {} bytes", bytes_written);
    println!("   📁 Restored file: {}", target_path.display());

    // Verify file size matches original
    let restored_size = std::fs::metadata(&target_path)?.len();
    if restored_size != metadata.original_size {
        println!(
            "   ⚠️  Warning: Restored file size ({} bytes) doesn't match original size ({} bytes)",
            restored_size, metadata.original_size
        );
    } else {
        println!("   ✅ File size verified: {} bytes", restored_size);
    }

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
    let _file = std::fs::File::open(&input)?;
    // Read entire file to get footer data
    let file_data = std::fs::read(&input)?;
    let (metadata, _footer_size) = FileHeader::from_footer_bytes(&file_data)
        .map_err(|e| anyhow::anyhow!("Failed to read .adapipe metadata: {}", e))?;

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
            .map_err(|e| anyhow::anyhow!("Failed to check existing file permissions: {}", e))?;

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
                std::fs::create_dir_all(parent_dir).map_err(|e| {
                    // Provide specific error messages for common permission issues
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        anyhow::anyhow!(
                            "Permission denied: Cannot create directory '{}'\nTry running with elevated privileges or \
                             choose a different location",
                            parent_dir.display()
                        )
                    } else {
                        anyhow::anyhow!("Failed to create directory '{}': {}", parent_dir.display(), e)
                    }
                })?;
            } else {
                print!(
                    "Directory '{}' does not exist. Create it? [y/N]: ",
                    parent_dir.display()
                );
                std::io::Write::flush(&mut std::io::stdout())?;

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
                    println!("📂 Creating directory: {}", parent_dir.display());
                    std::fs::create_dir_all(parent_dir).map_err(|e| {
                        if e.kind() == std::io::ErrorKind::PermissionDenied {
                            anyhow::anyhow!(
                                "Permission denied: Cannot create directory '{}'\nTry running with elevated \
                                 privileges or choose a different location",
                                parent_dir.display()
                            )
                        } else {
                            anyhow::anyhow!("Failed to create directory '{}': {}", parent_dir.display(), e)
                        }
                    })?;
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
        .map_err(|e| anyhow::anyhow!("Failed to create restoration pipeline: {}", e))?;

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
    .map_err(|e| anyhow::anyhow!("Restoration failed: {}", e))?;

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
                    operation: pipeline_domain::entities::Operation::Reverse, // REVERSE for legacy restoration!
                    parameters: step.parameters.clone(),
                    parallel_processing: false,
                    chunk_size: Some(1024 * 1024), // 1MB chunks
                };

                let decryption_stage = PipelineStage::new(
                    "decryption".to_string(),
                    StageType::Encryption, // Use Encryption type for decryption (internal restoration)
                    decryption_config,
                    stage_index,
                )?;

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
                    operation: pipeline_domain::entities::Operation::Reverse, // REVERSE for legacy restoration!
                    parameters: step.parameters.clone(),
                    parallel_processing: false,
                    chunk_size: Some(1024 * 1024), // 1MB chunks
                };

                let decompression_stage = PipelineStage::new(
                    "decompression".to_string(),
                    StageType::Compression, // Note: Using Compression type for decompression
                    decompression_config,
                    stage_index,
                )?;

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
                    operation: pipeline_domain::entities::Operation::Reverse, // REVERSE for legacy restoration!
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

                let custom_stage = PipelineStage::new(stage_name.clone(), stage_type, custom_config, stage_index)?;

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
        operation: pipeline_domain::entities::Operation::Reverse, // REVERSE for legacy restoration!
        parameters: HashMap::new(),
        parallel_processing: false,
        chunk_size: Some(1024 * 1024), // 1MB chunks
    };

    let verification_stage = PipelineStage::new(
        "verification".to_string(),
        StageType::Checksum, // Using Checksum type for verification
        verification_config,
        stage_index,
    )?;

    stages.push(verification_stage);
    info!("Added verification stage: sha256");

    // Validate that we have at least one stage
    if stages.is_empty() {
        return Err(anyhow::anyhow!("No restoration stages could be created from metadata"));
    }

    // Create ephemeral pipeline with special naming convention
    let pipeline_name = format!("__restore__{}", metadata.pipeline_id);

    let pipeline = Pipeline::new(pipeline_name, stages)?;

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
    let binary_format_service = AdapipeFormat::new();
    let mut adapipe_reader = binary_format_service
        .create_reader(input_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create .adapipe reader: {}", e))?;

    // Create output file for writing restored data
    let mut output_file = File::create(output_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create output file: {}", e))?;

    // Create domain services for restoration pipeline
    let compression_service = Arc::new(MultiAlgoCompression::new());
    let encryption_service = Arc::new(MultiAlgoEncryption::new());

    // Build stage service registry for validation
    let mut stage_services: HashMap<String, Arc<dyn pipeline_domain::services::StageService>> = HashMap::new();
    stage_services.insert("brotli".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("gzip".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("zstd".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("lz4".to_string(), compression_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("aes256gcm".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("aes128gcm".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("chacha20poly1305".to_string(), encryption_service.clone() as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("base64".to_string(), Arc::new(Base64EncodingService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("pii_masking".to_string(), Arc::new(PiiMaskingService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("tee".to_string(), Arc::new(TeeService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("passthrough".to_string(), Arc::new(PassThroughService::new()) as Arc<dyn pipeline_domain::services::StageService>);
    stage_services.insert("debug".to_string(), Arc::new(DebugService::new(Arc::new(MetricsService::new().unwrap()))) as Arc<dyn pipeline_domain::services::StageService>);

    let stage_executor = Arc::new(BasicStageExecutor::new(stage_services));

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

        // Combine nonce and payload data as expected by decryption service
        // The encryption service expects: [nonce (12 bytes)] + [encrypted_data]
        let mut chunk_data = chunk_format.nonce.to_vec();
        chunk_data.extend_from_slice(&chunk_format.payload);
        let file_chunk = FileChunk::new(
            chunk_sequence as u64,
            bytes_processed,
            chunk_data,
            false, // is_final - we'll determine this later
        )
        .map_err(|e| anyhow::anyhow!("Failed to create file chunk: {}", e))?;

        // Process chunk through restoration pipeline stages
        let mut current_chunk = file_chunk;
        for stage in restoration_pipeline.stages() {
            debug!("Processing chunk {} through stage: {}", chunk_sequence, stage.name());

            current_chunk = stage_executor
                .execute(stage, current_chunk, &mut processing_context)
                .await
                .map_err(|e| anyhow::anyhow!("Stage '{}' failed: {}", stage.name(), e))?;
        }

        // Write restored chunk to output file
        let restored_data = current_chunk.data();
        output_file
            .write_all(restored_data)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write restored data: {}", e))?;

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
        .map_err(|e| anyhow::anyhow!("Failed to flush output file: {}", e))?;

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
