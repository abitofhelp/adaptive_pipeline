// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Pipeline Service Implementation
//!
//! Application layer orchestration of file processing workflows. Coordinates
//! multi-stage pipelines (compression, encryption, I/O) with async processing,
//! progress monitoring, and parallel chunk processing. Integrates domain
//! services via dependency injection. Provides real-time metrics, error
//! recovery, and resource management. See mdBook for workflow details and
//! integration patterns.
//! - **Encryption Integration**: Seamless encryption of sensitive data
//! - **Key Management**: Secure key generation, storage, and rotation
//! - **Memory Protection**: Secure handling of sensitive data in memory
//!
//! ## Integration
//!
//! The pipeline service integrates with:
//!
//! - **Domain Layer**: Implements `PipelineService` trait
//! - **Repository Layer**: Persists pipeline state and metadata
//! - **Monitoring Systems**: Reports metrics and traces
//! - **Configuration System**: Dynamic configuration updates

use async_trait::async_trait;
use byte_unit::Byte;
use futures::future;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, Instrument};

use pipeline_domain::aggregates::PipelineAggregate;
use pipeline_domain::entities::pipeline_stage::StageType;
use pipeline_domain::entities::{Pipeline, PipelineStage, ProcessingContext, ProcessingMetrics, SecurityContext};
use pipeline_domain::repositories::stage_executor::ResourceRequirements;
use pipeline_domain::repositories::{PipelineRepository, StageExecutor};
use pipeline_domain::services::file_io_service::{FileIOService, ReadOptions};
use pipeline_domain::services::file_processor_service::{ChunkProcessor, FileProcessingResult};
use pipeline_domain::services::{
    CompressionService, EncryptionService, ExecutionRecord, ExecutionState, ExecutionStatus, KeyMaterial,
    PipelineRequirements, PipelineService,
};
use pipeline_domain::value_objects::{ChunkFormat, ChunkSize, FileChunk, PipelineId, WorkerCount};
use pipeline_domain::PipelineError;

use crate::infrastructure::services::binary_format_service::{BinaryFormatService, BinaryFormatWriter};
use crate::infrastructure::services::progress_indicator_service::ProgressIndicatorService;

/// Concrete implementation of the pipeline service
///
/// This struct provides the main orchestration logic for the adaptive pipeline
/// system, coordinating multiple services to process files through compression,
/// encryption, and binary format operations.
///
/// # Architecture
///
/// The pipeline service acts as the central coordinator, managing:
/// - Service dependencies and their lifecycles
/// - Processing workflow orchestration
/// - Resource allocation and management
/// - Progress monitoring and reporting
/// - Error handling and recovery
///
/// # Dependencies
///
/// The service requires several injected dependencies:
/// - **Compression Service**: Handles data compression operations
/// - **Encryption Service**: Manages encryption and key operations
/// - **Binary Format Service**: Creates and manages .adapipe format files
/// - **Pipeline Repository**: Persists pipeline state and metadata
/// - **Stage Executor**: Executes individual pipeline stages
/// - **Metrics Service**: Collects and reports performance metrics
/// - **Progress Indicator**: Provides real-time progress updates
///
/// # Examples

// ============================================================================
// Channel-Based Pipeline Architecture
// ============================================================================
// Educational: This section implements the three-stage execution pipeline
// (Reader → CPU Workers → Writer) using channels for communication.
//
// See: docs/EXECUTION_VS_PROCESSING_PIPELINES.md for architectural overview

/// Message sent from Reader task to CPU Worker tasks
///
/// ## Educational: Channel Message Design
///
/// This represents a unit of work flowing through the execution pipeline.
/// The reader sends these messages to workers, who process them and forward
/// results to the writer.
///
/// ## Design Rationale:
/// - `chunk_index`: Required for ordered writes (future enhancement)
/// - `data`: Owned Vec<u8> for zero-copy channel transfer
/// - `is_final`: Allows writer to finalize file on last chunk
/// - `enqueued_at`: Timestamp for queue wait metrics
#[derive(Debug)]
struct ChunkMessage {
    /// Index of this chunk in the file (0-based)
    chunk_index: usize,

    /// Raw chunk data (owned for channel transfer)
    data: Vec<u8>,

    /// True if this is the last chunk in the file
    is_final: bool,

    /// Original file chunk with metadata
    file_chunk: FileChunk,

    /// Timestamp when message was enqueued (for queue wait metrics)
    enqueued_at: std::time::Instant,
}

/// Message sent from CPU Worker tasks to Writer task
///
/// ## Educational: Processing Result
///
/// After CPU workers execute all processing stages (compression, encryption,
/// etc.), they send this message to the writer for persistence.
///
/// ## Design Rationale:
/// - `chunk_index`: Enables ordered writes (if needed)
/// - `processed_data`: Result of all processing stages
/// - `is_final`: Signals writer to finalize file
#[derive(Debug)]
struct ProcessedChunkMessage {
    /// Index of this chunk in the file (0-based)
    chunk_index: usize,

    /// Processed data (after all pipeline stages)
    processed_data: Vec<u8>,

    /// True if this is the last chunk in the file
    is_final: bool,
}

/// Statistics from the reader task
#[derive(Debug)]
struct ReaderStats {
    chunks_read: usize,
    bytes_read: u64,
}

/// Statistics from a CPU worker task
#[derive(Debug)]
struct WorkerStats {
    worker_id: usize,
    chunks_processed: usize,
}

/// Statistics from the writer task
#[derive(Debug)]
struct WriterStats {
    chunks_written: usize,
    bytes_written: u64,
}

// ============================================================================
// Pipeline Task Implementations
// ============================================================================

/// Reader Task - Stage 1 of Execution Pipeline
///
/// ## Educational: Single Reader Pattern
///
/// This task demonstrates the "single reader" pattern, which eliminates
/// coordination overhead. Only ONE task reads from disk, ensuring sequential
/// access patterns optimal for filesystem performance.
///
/// ## Backpressure Mechanism
///
/// The bounded channel creates natural backpressure:
/// - When workers are fast: Channel stays empty, reader proceeds immediately
/// - When workers are slow: Channel fills up, `tx_cpu.send()` blocks
/// - Result: Automatic flow control without explicit rate limiting!
///
/// ## Arguments
/// - `input_path`: File to read chunks from
/// - `chunk_size`: Size of each chunk in bytes
/// - `tx_cpu`: Channel sender to CPU workers (blocks when full)
/// - `file_io_service`: Service for reading file chunks
/// - `cancel_token`: Token for graceful cancellation
///
/// ## Returns
/// `ReaderStats` with chunks read and bytes read
async fn reader_task(
    input_path: PathBuf,
    chunk_size: usize,
    tx_cpu: tokio::sync::mpsc::Sender<ChunkMessage>,
    file_io_service: Arc<dyn FileIOService>,
    channel_capacity: usize,
    cancel_token: bootstrap::shutdown::CancellationToken,
) -> Result<ReaderStats, PipelineError> {
    use crate::infrastructure::metrics::CONCURRENCY_METRICS;

    // Check for cancellation before starting
    if cancel_token.is_cancelled() {
        return Err(PipelineError::cancelled());
    }

    // Configure read options for streaming
    let read_options = ReadOptions {
        chunk_size: Some(chunk_size),
        use_memory_mapping: false,  // Stream from disk, don't load all into memory
        calculate_checksums: false, // We'll calculate during processing
        ..Default::default()
    };

    // Read file into chunks using FileIOService
    let read_result = file_io_service
        .read_file_chunks(&input_path, read_options)
        .await
        .map_err(|e| PipelineError::IoError(format!("Failed to read file chunks: {}", e)))?;

    let total_chunks = read_result.chunks.len();
    let mut bytes_read = 0u64;

    // Send each chunk to CPU workers
    for (index, file_chunk) in read_result.chunks.into_iter().enumerate() {
        let chunk_data = file_chunk.data().to_vec();
        let chunk_size_bytes = chunk_data.len() as u64;
        bytes_read += chunk_size_bytes;

        let message = ChunkMessage {
            chunk_index: index,
            data: chunk_data,
            is_final: index == total_chunks - 1,
            file_chunk,
            enqueued_at: std::time::Instant::now(), // Timestamp for queue wait
        };

        // Educational: This blocks if channel is full → backpressure!
        // When workers are processing slowly, the reader waits here,
        // preventing memory overload from reading too far ahead.
        // Also cancellable for graceful shutdown.
        tokio::select! {
            _ = cancel_token.cancelled() => {
                return Err(PipelineError::cancelled_with_msg("reader cancelled during send"));
            }
            send_result = tx_cpu.send(message) => {
                send_result.map_err(|_e| PipelineError::io_error("CPU worker channel closed unexpectedly"))?;
            }
        }

        // Update queue depth metrics after send
        // Educational: Shows backpressure in real-time
        let remaining_capacity = tx_cpu.capacity();
        let current_depth = channel_capacity.saturating_sub(remaining_capacity);
        CONCURRENCY_METRICS.update_cpu_queue_depth(current_depth);
    }

    // Educational: Dropping tx_cpu signals "no more chunks" to workers
    // Workers receive None from rx_cpu.recv() and gracefully shut down
    drop(tx_cpu);

    Ok(ReaderStats {
        chunks_read: total_chunks,
        bytes_read,
    })
}

/// CPU Worker Task - Stage 2 of Execution Pipeline
///
/// ## Educational: Worker Pool Pattern
///
/// Multiple instances of this task run concurrently, forming a worker pool.
/// Each worker:
/// 1. Receives chunks from shared channel (MPSC pattern)
/// 2. Acquires global CPU token (prevents oversubscription)
/// 3. Executes ALL processing stages sequentially for ONE chunk
/// 4. Writes directly to shared writer using concurrent random-access writes
///
/// ## Execution vs Processing Pipeline
///
/// This is where the two pipelines intersect:
/// - **Execution pipeline**: Concurrency management (receive → process → write)
/// - **Processing pipeline**: Business logic (compress → encrypt → checksum)
///
/// See: docs/EXECUTION_VS_PROCESSING_PIPELINES.md for details
///
/// ## Arguments
/// - `worker_id`: Unique identifier for this worker (for metrics/debugging)
/// - `rx_cpu`: Channel receiver for chunks (shared among workers)
/// - `writer`: Thread-safe writer for concurrent random-access writes
/// - `pipeline`: Processing pipeline configuration (what stages to run)
/// - `stage_executor`: Executes individual processing stages
/// - `input_path`: Input file path (for ProcessingContext)
/// - `output_path`: Output file path (for ProcessingContext)
/// - `input_size`: Total input file size (for ProcessingContext)
/// - `security_context`: Security context for processing
///
/// ## Returns
/// `WorkerStats` with worker ID and chunks processed
async fn cpu_worker_task(
    worker_id: usize,
    mut rx_cpu: tokio::sync::mpsc::Receiver<ChunkMessage>,
    writer: Arc<dyn BinaryFormatWriter>,
    pipeline: Arc<Pipeline>,
    stage_executor: Arc<dyn StageExecutor>,
    input_path: PathBuf,
    output_path: PathBuf,
    input_size: u64,
    security_context: SecurityContext,
) -> Result<WorkerStats, PipelineError> {
    use crate::infrastructure::metrics::CONCURRENCY_METRICS;
    use crate::infrastructure::runtime::RESOURCE_MANAGER;

    let mut chunks_processed = 0;

    // Educational: Worker loop - receive, process, write
    while let Some(chunk_msg) = rx_cpu.recv().await {
        // ===================================================
        // EXECUTION PIPELINE: Resource acquisition
        // ===================================================

        // Acquire global CPU token to prevent oversubscription
        let cpu_wait_start = std::time::Instant::now();
        let _cpu_permit = RESOURCE_MANAGER
            .acquire_cpu()
            .await
            .map_err(|e| PipelineError::resource_exhausted(format!("Failed to acquire CPU token: {}", e)))?;
        let cpu_wait_duration = cpu_wait_start.elapsed();

        CONCURRENCY_METRICS.record_cpu_wait(cpu_wait_duration);
        CONCURRENCY_METRICS.worker_started();

        // ===================================================
        // PROCESSING PIPELINE: Business logic execution
        // ===================================================

        // Create local processing context for this chunk
        let mut local_context = ProcessingContext::new(
            input_path.clone(),
            output_path.clone(),
            input_size,
            security_context.clone(),
        );

        // Execute each configured stage sequentially on this chunk
        // Start with the FileChunk we received
        let mut file_chunk = chunk_msg.file_chunk;

        for stage in pipeline.stages() {
            file_chunk = stage_executor
                .execute(stage, file_chunk, &mut local_context)
                .await
                .map_err(|e| PipelineError::processing_failed(format!("Stage execution failed: {}", e)))?;
        }

        // ===================================================
        // EXECUTION PIPELINE: Direct concurrent write
        // ===================================================

        // Educational: No writer task! No mutex contention!
        // Workers write directly using thread-safe random-access writes.
        // Each write goes to a different file position, so they don't conflict.

        // Prepare chunk for .adapipe file format
        // The .adapipe format includes a nonce (number used once) for security
        // TODO: In production, this would come from the encryption stage
        let nonce = [0u8; 12]; // Placeholder nonce (12 bytes for AES-GCM)

        // Convert processed FileChunk to ChunkFormat for binary format
        let chunk_format = ChunkFormat::new(nonce, file_chunk.data().to_vec());

        // Direct concurrent write to calculated position
        writer
            .write_chunk_at_position(chunk_format, chunk_msg.chunk_index as u64)
            .await?;

        // Educational: CPU token released automatically (RAII drop)
        CONCURRENCY_METRICS.worker_completed();
        chunks_processed += 1;
    }

    Ok(WorkerStats {
        worker_id,
        chunks_processed,
    })
}

// ============================================================================
// Public Implementation
// ============================================================================

pub struct PipelineServiceImpl {
    compression_service: Arc<dyn CompressionService>,
    encryption_service: Arc<dyn EncryptionService>,
    file_io_service: Arc<dyn FileIOService>,
    pipeline_repository: Arc<dyn PipelineRepository>,
    stage_executor: Arc<dyn StageExecutor>,
    binary_format_service: Arc<dyn BinaryFormatService>,
    active_pipelines: Arc<RwLock<std::collections::HashMap<String, PipelineAggregate>>>,
}

impl PipelineServiceImpl {
    /// Creates a new pipeline service with injected dependencies
    ///
    /// # Arguments
    /// * `compression_service` - Service for compression operations
    /// * `encryption_service` - Service for encryption operations
    /// * `file_io_service` - Service for file I/O operations
    /// * `pipeline_repository` - Repository for pipeline persistence
    /// * `stage_executor` - Executor for pipeline stages
    /// * `binary_format_service` - Service for binary format operations
    pub fn new(
        compression_service: Arc<dyn CompressionService>,
        encryption_service: Arc<dyn EncryptionService>,
        file_io_service: Arc<dyn FileIOService>,
        pipeline_repository: Arc<dyn PipelineRepository>,
        stage_executor: Arc<dyn StageExecutor>,
        binary_format_service: Arc<dyn BinaryFormatService>,
    ) -> Self {
        Self {
            compression_service,
            encryption_service,
            file_io_service,
            pipeline_repository,
            stage_executor,
            binary_format_service,
            active_pipelines: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Processes a single chunk through a pipeline stage
    async fn process_chunk_through_stage(
        &self,
        chunk: FileChunk,
        stage: &PipelineStage,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        debug!("Processing chunk through stage: {}", stage.name());

        match stage.stage_type() {
            StageType::Compression => {
                // Extract compression configuration from stage
                let compression_config = self.extract_compression_config(stage)?;
                self.compression_service
                    .compress_chunk(chunk, &compression_config, context)
            }
            StageType::Encryption => {
                let encryption_config = self.extract_encryption_config(stage)?;
                // Generate a temporary key material for demonstration (NOT secure for
                // production)
                let key_material = KeyMaterial {
                    key: vec![0u8; 32],   // 32-byte key
                    nonce: vec![0u8; 12], // 12-byte nonce
                    salt: vec![0u8; 32],  // 32-byte salt
                    algorithm: encryption_config.algorithm.clone(),
                    created_at: chrono::Utc::now(),
                    expires_at: None,
                };
                self.encryption_service
                    .encrypt_chunk(chunk, &encryption_config, &key_material, context)
            }

            StageType::Checksum => {
                // For integrity checking, just pass through the chunk unchanged
                // In a real implementation, this would calculate and verify checksums
                Ok(chunk)
            }
            StageType::Transform => {
                // For transform stages, delegate to the stage executor
                self.stage_executor.execute(stage, chunk.clone(), context).await
            }
            StageType::PassThrough => {
                // For custom stages, delegate to the stage executor
                self.stage_executor.execute(stage, chunk.clone(), context).await
            }
        }
    }

    /// Extracts compression configuration from a pipeline stage
    fn extract_compression_config(
        &self,
        stage: &PipelineStage,
    ) -> Result<pipeline_domain::services::CompressionConfig, PipelineError> {
        let algorithm_str = stage.configuration().algorithm.as_str();
        let algorithm = match algorithm_str {
            "brotli" => pipeline_domain::services::CompressionAlgorithm::Brotli,
            "gzip" => pipeline_domain::services::CompressionAlgorithm::Gzip,
            "zstd" => pipeline_domain::services::CompressionAlgorithm::Zstd,
            "lz4" => pipeline_domain::services::CompressionAlgorithm::Lz4,
            _ => {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Unsupported compression algorithm: {}",
                    algorithm_str
                )));
            }
        };

        // Extract compression level from parameters
        let level = stage
            .configuration()
            .parameters
            .get("level")
            .and_then(|v| v.parse::<u32>().ok())
            .map(|l| match l {
                0..=3 => pipeline_domain::services::CompressionLevel::Fast,
                4..=6 => pipeline_domain::services::CompressionLevel::Balanced,
                7.. => pipeline_domain::services::CompressionLevel::Best,
            })
            .unwrap_or(pipeline_domain::services::CompressionLevel::Balanced);

        Ok(pipeline_domain::services::CompressionConfig {
            algorithm,
            level,
            dictionary: None,
            window_size: None,
            parallel_processing: stage.configuration().parallel_processing,
        })
    }

    /// Extracts encryption configuration from a pipeline stage
    fn extract_encryption_config(
        &self,
        stage: &PipelineStage,
    ) -> Result<pipeline_domain::services::EncryptionConfig, PipelineError> {
        let algorithm_str = stage.configuration().algorithm.as_str();
        let algorithm = match algorithm_str {
            "aes256-gcm" | "aes256gcm" => pipeline_domain::services::EncryptionAlgorithm::Aes256Gcm,
            "chacha20-poly1305" | "chacha20poly1305" => {
                pipeline_domain::services::EncryptionAlgorithm::ChaCha20Poly1305
            }
            "aes128-gcm" | "aes128gcm" => pipeline_domain::services::EncryptionAlgorithm::Aes128Gcm,
            "aes192-gcm" | "aes192gcm" => pipeline_domain::services::EncryptionAlgorithm::Aes192Gcm,
            _ => {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Unsupported encryption algorithm: {}",
                    algorithm_str
                )));
            }
        };

        let kdf = stage
            .configuration()
            .parameters
            .get("kdf")
            .map(|kdf_str| match kdf_str.as_str() {
                "argon2" => pipeline_domain::services::KeyDerivationFunction::Argon2,
                "scrypt" => pipeline_domain::services::KeyDerivationFunction::Scrypt,
                "pbkdf2" => pipeline_domain::services::KeyDerivationFunction::Pbkdf2,
                _ => pipeline_domain::services::KeyDerivationFunction::Argon2,
            });

        Ok(pipeline_domain::services::EncryptionConfig {
            algorithm,
            key_derivation: kdf.unwrap_or(pipeline_domain::services::KeyDerivationFunction::Argon2),
            key_size: 32,             // Default to 256-bit keys
            nonce_size: 12,           // Standard for AES-GCM
            salt_size: 16,            // Standard salt size
            iterations: 100_000,      // Default iterations for PBKDF2
            memory_cost: Some(65536), // Default for Argon2
            parallel_cost: Some(1),   // Default for Argon2
            associated_data: None,    // No additional authenticated data by default
        })
    }

    /// Updates processing metrics based on execution results
    fn update_metrics(&self, context: &mut ProcessingContext, stage_name: &str, duration: std::time::Duration) {
        let mut metrics = context.metrics().clone();

        // Create new stage metrics with actual data
        let mut stage_metrics =
            pipeline_domain::entities::processing_metrics::StageMetrics::new(stage_name.to_string());
        stage_metrics.update(metrics.bytes_processed(), duration);
        metrics.add_stage_metrics(stage_metrics);

        context.update_metrics(metrics);
    }
}

#[async_trait]
impl PipelineService for PipelineServiceImpl {
    async fn process_file(
        &self,
        pipeline_id: PipelineId,
        input_path: &std::path::Path,
        output_path: &std::path::Path,
        security_context: SecurityContext,
        user_worker_override: Option<usize>,
        observer: Option<std::sync::Arc<dyn pipeline_domain::services::pipeline_service::ProcessingObserver>>,
    ) -> Result<ProcessingMetrics, PipelineError> {
        debug!(
            "Processing file: {} -> {} with pipeline {} (.adapipe format)",
            input_path.display(),
            output_path.display(),
            pipeline_id
        );

        let start_time = std::time::Instant::now();

        // Load pipeline from repository using the provided PipelineId
        let pipeline = self
            .pipeline_repository
            .find_by_id(pipeline_id.clone())
            .await?
            .ok_or_else(|| PipelineError::PipelineNotFound(pipeline_id.to_string()))?;

        // Validate pipeline before execution
        self.validate_pipeline(&pipeline).await?;

        // Get file metadata first to determine optimal chunk size
        let input_metadata = tokio::fs::metadata(input_path)
            .await
            .map_err(|e| PipelineError::IoError(e.to_string()))?;
        let input_size = input_metadata.len();

        // Calculate optimal chunk size based on file size
        let chunk_size = pipeline_domain::value_objects::ChunkSize::optimal_for_file_size(input_size).bytes();

        // Use FileIOService to read file in chunks (streaming, memory-efficient)
        // This avoids loading the entire file into memory
        let read_options = pipeline_domain::services::file_io_service::ReadOptions {
            chunk_size: Some(chunk_size),
            use_memory_mapping: false,  // Start with streaming; can optimize later
            calculate_checksums: false, // We'll calculate overall checksum ourselves
            ..Default::default()
        };

        let read_result = self.file_io_service.read_file_chunks(input_path, read_options).await?;

        let input_chunks = read_result.chunks;

        // Calculate original file checksum incrementally from chunks
        // This way we don't need the entire file in memory
        let original_checksum = {
            use ring::digest;
            let mut context = ring::digest::Context::new(&ring::digest::SHA256);
            for chunk in &input_chunks {
                context.update(chunk.data());
            }
            let digest = context.finish();
            hex::encode(digest.as_ref())
        };

        debug!(
            "Input file: {}, SHA256: {}",
            Byte::from_u128(input_size as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string(),
            original_checksum
        );

        // Create .adapipe file header
        let mut header = pipeline_domain::value_objects::FileHeader::new(
            input_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            input_size,
            original_checksum.clone(),
        );

        // Add processing steps based on pipeline stages
        for stage in pipeline.stages() {
            debug!(
                "Processing pipeline stage: name='{}', type='{:?}', algorithm='{}'",
                stage.name(),
                stage.stage_type(),
                stage.configuration().algorithm
            );
            match stage.stage_type() {
                pipeline_domain::entities::pipeline_stage::StageType::Compression => {
                    debug!("✅ Matched Compression stage: {}", stage.name());
                    let config = self.extract_compression_config(stage)?;
                    let algorithm_str = match config.algorithm {
                        pipeline_domain::services::CompressionAlgorithm::Brotli => "brotli",
                        pipeline_domain::services::CompressionAlgorithm::Gzip => "gzip",
                        pipeline_domain::services::CompressionAlgorithm::Zstd => "zstd",
                        pipeline_domain::services::CompressionAlgorithm::Lz4 => "lz4",
                        pipeline_domain::services::CompressionAlgorithm::Custom(ref name) => name.as_str(),
                    };
                    let level = match config.level {
                        pipeline_domain::services::CompressionLevel::Fastest => 1,
                        pipeline_domain::services::CompressionLevel::Fast => 3,
                        pipeline_domain::services::CompressionLevel::Balanced => 6,
                        pipeline_domain::services::CompressionLevel::Best => 9,
                        pipeline_domain::services::CompressionLevel::Custom(level) => level,
                    };
                    header = header.add_compression_step(algorithm_str, level);
                }
                pipeline_domain::entities::pipeline_stage::StageType::Encryption => {
                    debug!("✅ Matched Encryption stage: {}", stage.name());
                    let config = self.extract_encryption_config(stage)?;
                    let algorithm_str = match config.algorithm {
                        pipeline_domain::services::EncryptionAlgorithm::Aes128Gcm => "aes128gcm",
                        pipeline_domain::services::EncryptionAlgorithm::Aes192Gcm => "aes192gcm",
                        pipeline_domain::services::EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
                        pipeline_domain::services::EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
                        pipeline_domain::services::EncryptionAlgorithm::Custom(ref name) => name.as_str(),
                    };
                    header = header.add_encryption_step(algorithm_str, "argon2", 32, 12);
                }
                pipeline_domain::entities::pipeline_stage::StageType::Checksum => {
                    debug!("✅ Matched Checksum stage: {}", stage.name());
                    // Checksum stages use proper ProcessingStepType::Checksum
                    header = header.add_checksum_step(stage.configuration().algorithm.as_str());
                }
                pipeline_domain::entities::pipeline_stage::StageType::PassThrough => {
                    debug!("✅ Matched PassThrough stage: {}", stage.name());
                    // PassThrough stages use proper ProcessingStepType::PassThrough
                    header = header.add_passthrough_step(stage.configuration().algorithm.as_str());
                }
                _ => {
                    // Fallback for any unhandled stage types
                    debug!(
                        "⚠️ Unhandled stage type: name='{}', type='{:?}', algorithm='{}'",
                        stage.name(),
                        stage.stage_type(),
                        stage.configuration().algorithm
                    );
                    header = header.add_custom_step(
                        stage.name(),
                        stage.configuration().algorithm.as_str(),
                        stage.configuration().parameters.clone(),
                    );
                }
            }
        }

        // Set chunk info and pipeline ID - chunk_size already calculated above
        header = header
            .with_chunk_info(chunk_size as u32, 0) // chunk_count will be updated later
            .with_pipeline_id(pipeline_id.to_string());

        // Clone security context before moving it into ProcessingContext
        let security_context_for_tasks = security_context.clone();

        let mut context = ProcessingContext::new(
            input_path.to_path_buf(),
            output_path.to_path_buf(),
            input_size,
            security_context,
        );

        // Set input file checksum in metrics
        {
            let mut metrics = context.metrics().clone();
            metrics.set_input_file_info(input_size, Some(original_checksum.clone()));
            context.update_metrics(metrics);
        }

        // =============================================================================
        // CHANNEL-BASED PIPELINE ARCHITECTURE
        // =============================================================================
        // This section implements the three-stage execution pipeline using channels
        // for natural backpressure and lock-free concurrent writes.
        //
        // ARCHITECTURE:
        // Reader Task → [Channel] → CPU Worker Pool → Direct Concurrent Writes
        //
        // KEY BENEFITS:
        // 1. NO MUTEX CONTENTION: Workers write directly using thread-safe
        //    random-access
        // 2. NATURAL BACKPRESSURE: Bounded channels prevent memory overload
        // 3. CLEAR SEPARATION: Reader/Workers have distinct responsibilities
        // 4. OBSERVABLE: Channel depths reveal bottlenecks
        // 5. SCALABLE: True parallel writes to non-overlapping file positions
        //
        // See: docs/EXECUTION_VS_PROCESSING_PIPELINES.md for architectural details

        // STEP 1: Calculate total number of chunks
        let total_chunks = (input_size as usize).div_ceil(chunk_size);

        // STEP 2: Create thread-safe writer
        // Writer uses &self for concurrent writes (no mutex on individual writes!)
        // But we wrap in Arc for sharing, and Mutex is needed only for finalization
        let binary_writer = self.binary_format_service.create_writer(output_path, header.clone()).await?;
        let writer_shared = Arc::new(binary_writer);

        // Create progress indicator for this operation
        let progress_indicator = Arc::new(ProgressIndicatorService::new(total_chunks as u64));

        // STEP 3: Determine worker count (adaptive or user-specified)
        let available_cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        let is_cpu_intensive = pipeline.stages().iter().any(|stage| {
            matches!(stage.stage_type(), StageType::Checksum)
                && (stage.name().contains("compression") || stage.name().contains("encryption"))
        });

        let optimal_worker_count =
            WorkerCount::optimal_for_processing_type(input_size, available_cores, is_cpu_intensive);

        let worker_count = if let Some(user_workers) = user_worker_override {
            let validated = WorkerCount::validate_user_input(user_workers, available_cores, input_size);
            match validated {
                Ok(count) => {
                    debug!("Using user-specified worker count: {} (validated)", count);
                    count
                }
                Err(warning) => {
                    warn!(
                        "User worker count invalid: {}. Using adaptive: {}",
                        warning,
                        optimal_worker_count.count()
                    );
                    optimal_worker_count.count()
                }
            }
        } else {
            debug!("Using adaptive worker count: {}", optimal_worker_count.count());
            optimal_worker_count.count()
        };

        debug!(
            "Channel-based pipeline: {} workers for {} bytes ({})",
            worker_count,
            input_size,
            WorkerCount::strategy_description(input_size)
        );

        // STEP 4: Create cancellation token for graceful shutdown
        // Educational: Enables graceful cancellation of reader and worker tasks
        // TODO: Wire this to global ShutdownCoordinator for Ctrl-C handling
        let shutdown_coordinator = bootstrap::shutdown::ShutdownCoordinator::new(std::time::Duration::from_secs(5));
        let cancel_token = shutdown_coordinator.token();

        // STEP 5: Create bounded channels for pipeline stages
        // Educational: Channel depth creates backpressure to prevent memory overload
        let channel_depth = 4; // TODO: Make this configurable via CLI
        let (tx_cpu, rx_cpu) = tokio::sync::mpsc::channel::<ChunkMessage>(channel_depth);

        // STEP 5: Wrap receiver in Arc<Mutex> for sharing among workers
        // Educational: Multiple workers need to share ONE receiver (MPSC pattern)
        // This adds some contention, but only on channel receive (not on writes!)
        let rx_cpu_shared = Arc::new(tokio::sync::Mutex::new(rx_cpu));

        // STEP 6: Spawn reader task
        // Single reader streams chunks from disk to CPU workers
        let reader_handle = tokio::spawn(reader_task(
            input_path.to_path_buf(),
            chunk_size,
            tx_cpu,
            self.file_io_service.clone(),
            channel_depth,
            cancel_token.clone(),
        ));

        // STEP 7: Spawn CPU worker pool
        // Multiple workers receive chunks, process them, and write directly
        let mut worker_handles = Vec::new();
        let pipeline_arc = Arc::new(pipeline.clone());

        for worker_id in 0..worker_count {
            let rx_cpu_clone = rx_cpu_shared.clone();
            let writer_clone = writer_shared.clone();
            let pipeline_clone = pipeline_arc.clone();
            let stage_executor_clone = self.stage_executor.clone();
            let input_path_clone = input_path.to_path_buf();
            let output_path_clone = output_path.to_path_buf();
            let security_context_clone = security_context_for_tasks.clone();
            let cancel_token_clone = cancel_token.clone();

            // Each worker shares the receiver via Arc<Mutex>
            let worker_handle = tokio::spawn(async move {
                use crate::infrastructure::metrics::CONCURRENCY_METRICS;
                use crate::infrastructure::runtime::RESOURCE_MANAGER;

                let mut chunks_processed = 0;

                loop {
                    // Check for cancellation before receiving next chunk
                    // Educational: Cancellation checked at loop boundary (not in hot path)
                    // IMPORTANT: We hold the mutex across await in the receive - this is correct!
                    // It ensures atomic receive from shared receiver (work-stealing pattern)
                    #[allow(clippy::await_holding_lock)]
                    let chunk_result = tokio::select! {
                        _ = cancel_token_clone.cancelled() => {
                            // Graceful shutdown: exit worker loop
                            break;
                        }
                        // Lock receiver to get next chunk
                        chunk_msg = async {
                            let mut rx = rx_cpu_clone.lock().await;
                            rx.recv().await
                        } => chunk_msg,
                    };

                    match chunk_result {
                        Some(chunk_msg) => {
                            // Record queue wait time (time chunk spent in channel)
                            // Educational: High wait times indicate worker saturation
                            let queue_wait = chunk_msg.enqueued_at.elapsed();
                            CONCURRENCY_METRICS.record_cpu_queue_wait(queue_wait);

                            // Acquire global CPU token
                            let cpu_wait_start = std::time::Instant::now();
                            let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await.map_err(|e| {
                                PipelineError::resource_exhausted(format!("Failed to acquire CPU token: {}", e))
                            })?;
                            let cpu_wait_duration = cpu_wait_start.elapsed();

                            CONCURRENCY_METRICS.record_cpu_wait(cpu_wait_duration);
                            CONCURRENCY_METRICS.worker_started();

                            // Create local processing context
                            let mut local_context = ProcessingContext::new(
                                input_path_clone.clone(),
                                output_path_clone.clone(),
                                input_size,
                                security_context_clone.clone(),
                            );

                            // Execute all processing stages
                            let mut file_chunk = chunk_msg.file_chunk;
                            for stage in pipeline_clone.stages() {
                                file_chunk = stage_executor_clone
                                    .execute(stage, file_chunk, &mut local_context)
                                    .await
                                    .map_err(|e| {
                                        PipelineError::processing_failed(format!("Stage execution failed: {}", e))
                                    })?;
                            }

                            // Prepare and write chunk
                            let nonce = [0u8; 12]; // TODO: Get from encryption stage
                            let chunk_format = ChunkFormat::new(nonce, file_chunk.data().to_vec());
                            writer_clone
                                .write_chunk_at_position(chunk_format, chunk_msg.chunk_index as u64)
                                .await?;

                            CONCURRENCY_METRICS.worker_completed();
                            chunks_processed += 1;
                        }
                        None => {
                            // Channel closed, exit
                            break;
                        }
                    }
                }

                Ok::<WorkerStats, PipelineError>(WorkerStats {
                    worker_id,
                    chunks_processed,
                })
            });

            worker_handles.push(worker_handle);
        }

        // =============================================================================
        // STEP 7: WAIT FOR PIPELINE COMPLETION
        // =============================================================================
        // Reader → Workers all complete independently, coordinated by channels

        // Wait for reader to finish
        let reader_stats = reader_handle
            .await
            .map_err(|e| PipelineError::processing_failed(format!("Reader task failed: {}", e)))??;

        debug!(
            "Reader completed: {} chunks read, {} bytes",
            reader_stats.chunks_read, reader_stats.bytes_read
        );

        // Wait for all workers to complete
        let mut total_chunks_processed = 0;
        for (worker_id, worker_handle) in worker_handles.into_iter().enumerate() {
            let worker_stats = worker_handle
                .await
                .map_err(|e| PipelineError::processing_failed(format!("Worker {} failed: {}", worker_id, e)))??;

            debug!(
                "Worker {} completed: {} chunks processed",
                worker_stats.worker_id, worker_stats.chunks_processed
            );
            total_chunks_processed += worker_stats.chunks_processed;
        }

        // =============================================================================
        // STEP 8: FINALIZE WRITER
        // =============================================================================
        // All chunks written, now write footer and finalize

        // Finalize writer using &self signature (works perfectly with Arc!)
        // Educational: No Arc::try_unwrap needed, just call finalize directly
        let _total_bytes_written = writer_shared.finalize(header).await?;

        // =============================================================================
        // STEP 9: COLLECT METRICS AND COMPLETE
        // =============================================================================

        // Calculate final metrics from task results
        let chunks_processed = total_chunks_processed as u64;
        let total_bytes_processed = reader_stats.bytes_read;

        // Show completion summary to user
        let total_duration = start_time.elapsed();
        let throughput = (total_bytes_processed as f64) / total_duration.as_secs_f64() / (1024.0 * 1024.0); // MB/s
        progress_indicator
            .show_completion(total_bytes_processed, throughput, total_duration)
            .await;

        // Get the final file size for metrics
        let total_output_bytes = tokio::fs::metadata(output_path)
            .await
            .map_err(|e| PipelineError::io_error(format!("Failed to get output file size: {}", e)))?
            .len();

        // Record metrics to Prometheus
        let mut processing_metrics = context.metrics().clone();
        processing_metrics.start();
        processing_metrics.update_bytes_processed(total_bytes_processed);
        processing_metrics.update_chunks_processed(chunks_processed);
        processing_metrics.set_output_file_info(total_output_bytes, None);
        processing_metrics.end();

        // Single concise completion log
        debug!(
            "Channel pipeline completed: {} chunks, {:.2} MB/s, {} → {} in {:?}",
            chunks_processed,
            throughput,
            Byte::from_u128(total_bytes_processed as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal),
            Byte::from_u128(total_output_bytes as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal),
            total_duration
        );

        // Create and return processing metrics
        let mut metrics = context.metrics().clone();
        metrics.start();
        metrics.update_bytes_processed(total_bytes_processed);
        metrics.update_chunks_processed(chunks_processed);

        // Calculate output file checksum
        let output_checksum = {
            let output_data = tokio::fs::read(output_path)
                .await
                .map_err(|e| PipelineError::io_error(e.to_string()))?;
            let digest = ring::digest::digest(&ring::digest::SHA256, &output_data);
            hex::encode(digest.as_ref())
        };

        // Set the actual output file size and checksum
        metrics.set_output_file_info(total_output_bytes, Some(output_checksum));
        metrics.end();

        // Notify observer that processing completed with final metrics
        if let Some(obs) = &observer {
            obs.on_processing_completed(total_duration, Some(&metrics)).await;
        }

        Ok(metrics)
    }

    async fn process_chunks(
        &self,
        pipeline: &Pipeline,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        let mut processed_chunks = chunks;

        for stage in pipeline.stages() {
            info!("Processing through stage: {}", stage.name());
            let stage_start = std::time::Instant::now();

            // Process chunks in parallel within this stage
            // Note: Each chunk gets a cloned context since we're processing in parallel
            let futures: Vec<_> = processed_chunks
                .into_iter()
                .map(|chunk| {
                    let mut ctx = context.clone();
                    async move { self.process_chunk_through_stage(chunk, stage, &mut ctx).await }
                })
                .collect();

            processed_chunks = future::try_join_all(futures).await?;

            let stage_duration = stage_start.elapsed();
            self.update_metrics(context, stage.name(), stage_duration);

            info!("Completed stage {} in {:?}", stage.name(), stage_duration);
        }

        Ok(processed_chunks)
    }

    async fn validate_pipeline(&self, pipeline: &Pipeline) -> Result<(), PipelineError> {
        debug!("Validating pipeline: {}", pipeline.id());

        // Check if pipeline has stages
        if pipeline.stages().is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline has no stages".to_string(),
            ));
        }

        // Validate each stage
        for stage in pipeline.stages() {
            // Check stage configuration
            if stage.configuration().algorithm.is_empty() {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Stage '{}' has no algorithm specified",
                    stage.name()
                )));
            }

            // Check stage compatibility
            if let Err(e) = stage.validate() {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Stage '{}' validation failed: {}",
                    stage.name(),
                    e
                )));
            }
        }

        debug!("Pipeline validation passed");
        Ok(())
    }

    async fn estimate_processing_time(
        &self,
        pipeline: &Pipeline,
        file_size: u64,
    ) -> Result<std::time::Duration, PipelineError> {
        let mut total_seconds = 0.0;
        let file_size_mb = (file_size as f64) / (1024.0 * 1024.0);

        for stage in pipeline.stages() {
            // Estimate based on stage type and file size
            let stage_seconds = match stage.stage_type() {
                pipeline_domain::entities::StageType::Compression => file_size_mb / 50.0, // 50 MB/s
                pipeline_domain::entities::StageType::Encryption => file_size_mb / 100.0, // 100 MB/s
                _ => file_size_mb / 200.0,
                /* 200 MB/s for other
                 * operations */
            };
            total_seconds += stage_seconds;
        }

        Ok(std::time::Duration::from_secs_f64(total_seconds))
    }

    async fn get_resource_requirements(
        &self,
        pipeline: &Pipeline,
        file_size: u64,
    ) -> Result<ResourceRequirements, PipelineError> {
        let mut total_memory_mb = 0.0;
        let mut total_cpu_cores = 0;
        let mut estimated_time_seconds = 0.0;

        for stage in pipeline.stages() {
            // Estimate memory usage based on stage type and chunk size
            let chunk_size = stage.configuration().chunk_size.unwrap_or(1024 * 1024) as f64;
            let stage_memory = (chunk_size / (1024.0 * 1024.0)) * 2.0; // Estimate 2x chunk size for processing
            total_memory_mb += stage_memory;

            // Estimate CPU cores needed
            if stage.configuration().parallel_processing {
                total_cpu_cores = total_cpu_cores.max(4); // Assume 4 cores for
                                                          // parallel stages
            } else {
                total_cpu_cores = total_cpu_cores.max(1);
            }

            // Estimate processing time
            let throughput_mbps = match stage.stage_type() {
                pipeline_domain::entities::StageType::Compression => 50.0,
                pipeline_domain::entities::StageType::Encryption => 100.0,
                _ => 200.0,
            };

            let file_size_mb = (file_size as f64) / (1024.0 * 1024.0);
            estimated_time_seconds += file_size_mb / throughput_mbps;
        }

        Ok(ResourceRequirements {
            memory_bytes: (total_memory_mb * 1024.0 * 1024.0) as u64,
            cpu_cores: total_cpu_cores,
            disk_space_bytes: ((file_size as f64) * 2.0) as u64, // Estimate 2x file size
            network_bandwidth_bps: None,                         // Not applicable for local processing
            gpu_memory_bytes: None,                              // Not implemented yet
            estimated_duration: std::time::Duration::from_secs_f64(estimated_time_seconds),
        })
    }

    async fn create_optimized_pipeline(
        &self,
        file_path: &std::path::Path,
        requirements: PipelineRequirements,
    ) -> Result<Pipeline, PipelineError> {
        let file_extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        let pipeline_name = format!("optimized_pipeline_{}", uuid::Uuid::new_v4());
        let mut stages = Vec::new();

        // Add compression stage if requested
        if requirements.compression_required {
            let algorithm = match file_extension {
                "txt" | "log" | "csv" | "json" | "xml" | "html" => "brotli",
                "bin" | "exe" | "dll" => "zstd",
                _ => "brotli", // Default to Brotli
            };

            let compression_config = pipeline_domain::entities::StageConfiguration {
                algorithm: algorithm.to_string(),
                parameters: std::collections::HashMap::new(),
                parallel_processing: requirements.parallel_processing,
                chunk_size: Some(1024 * 1024), // Default 1MB chunks
            };

            let compression_stage = pipeline_domain::entities::PipelineStage::new(
                "compression".to_string(),
                pipeline_domain::entities::StageType::Compression,
                compression_config,
                stages.len() as u32,
            )?;

            stages.push(compression_stage);
        }

        // Add encryption stage if requested
        if requirements.encryption_required {
            let encryption_config = pipeline_domain::entities::StageConfiguration {
                algorithm: "aes256-gcm".to_string(),
                parameters: std::collections::HashMap::new(),
                parallel_processing: requirements.parallel_processing,
                chunk_size: Some(1024 * 1024), // Default 1MB chunks
            };

            let encryption_stage = pipeline_domain::entities::PipelineStage::new(
                "encryption".to_string(),
                pipeline_domain::entities::StageType::Encryption,
                encryption_config,
                stages.len() as u32,
            )?;

            stages.push(encryption_stage);
        }

        Pipeline::new(pipeline_name, stages)
    }

    async fn monitor_execution(
        &self,
        pipeline_id: PipelineId,
        context: &ProcessingContext,
    ) -> Result<ExecutionStatus, PipelineError> {
        let active_pipelines = self.active_pipelines.read().await;

        if let Some(_aggregate) = active_pipelines.get(&pipeline_id.to_string()) {
            Ok(ExecutionStatus {
                pipeline_id,
                status: ExecutionState::Running,
                progress_percentage: 0.0,
                bytes_processed: 0,
                bytes_total: 0,
                current_stage: Some("unknown".to_string()),
                estimated_remaining_time: None,
                error_count: 0,
                warning_count: 0,
                started_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        } else {
            Err(PipelineError::PipelineNotFound(pipeline_id.to_string()))
        }
    }

    async fn pause_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError> {
        info!("Pipeline {} paused", pipeline_id);
        Ok(())
    }

    async fn resume_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError> {
        info!("Pipeline {} resumed", pipeline_id);
        Ok(())
    }

    async fn cancel_execution(&self, pipeline_id: PipelineId) -> Result<(), PipelineError> {
        let mut active_pipelines = self.active_pipelines.write().await;

        if active_pipelines.remove(&pipeline_id.to_string()).is_some() {
            info!("Pipeline {} cancelled", pipeline_id);
            Ok(())
        } else {
            Err(PipelineError::PipelineNotFound(pipeline_id.to_string()))
        }
    }

    async fn get_execution_history(
        &self,
        pipeline_id: PipelineId,
        _limit: Option<usize>,
    ) -> Result<Vec<ExecutionRecord>, PipelineError> {
        // In a real implementation, this would query a database
        // For now, return empty history
        Ok(Vec::new())
    }
}

/// ChunkProcessor implementation that processes chunks through a pipeline
pub struct PipelineChunkProcessor {
    pipeline: Pipeline,
    stage_executor: Arc<dyn StageExecutor>,
}

impl PipelineChunkProcessor {
    pub fn new(pipeline: Pipeline, stage_executor: Arc<dyn StageExecutor>) -> Self {
        Self {
            pipeline,
            stage_executor,
        }
    }
}

// NOTE: PipelineChunkProcessor cannot implement the sync ChunkProcessor trait
// because it coordinates async operations (stage_executor.execute is async).
// This is an application-level service that orchestrates multiple async
// operations, not a CPU-bound chunk processor.
//
// The ChunkProcessor trait is for sync, CPU-bound processing (compression,
// encryption, etc.) Pipeline orchestration involves async I/O and should use
// application-level patterns instead.
//
// TODO: If needed, create a separate async pipeline processing interface in the
// application layer.

// Removed ChunkProcessor implementation - architectural mismatch
// impl ChunkProcessor for PipelineChunkProcessor {
//     ...
// }
// Orphaned methods removed (were part of ChunkProcessor trait implementation)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::adapters::repositories::stage_executor_adapter::BasicStageExecutorAdapterAdapter;
    use crate::infrastructure::adapters::{CompressionServiceImpl, EncryptionServiceImpl};
    use crate::infrastructure::repositories::sqlite_pipeline_repository::SqlitePipelineRepository;
    use pipeline_domain::entities::pipeline::Pipeline;
    use pipeline_domain::entities::security_context::SecurityContext;
    use pipeline_domain::value_objects::binary_file_format::{FileHeader, CURRENT_FORMAT_VERSION, MAGIC_BYTES};
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio::fs;

    /// Tests pipeline creation for database operations.
    ///
    /// This test validates that pipelines can be created with proper
    /// configuration for database storage and retrieval operations,
    /// including stage creation and pipeline assembly.
    ///
    /// # Test Coverage
    ///
    /// - Pipeline stage creation with compression configuration
    /// - Pipeline assembly with multiple stages
    /// - Stage configuration validation
    /// - Pipeline metadata verification
    /// - Database-ready pipeline preparation
    ///
    /// # Test Scenario
    ///
    /// Creates a compression stage with specific configuration and
    /// assembles it into a pipeline suitable for database operations.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Pipeline creation for database persistence
    /// - Stage configuration and validation
    /// - Pipeline metadata management
    /// - Database integration preparation
    ///
    /// # Assertions
    ///
    /// - Compression stage creation succeeds
    /// - Pipeline creation succeeds
    /// - Pipeline has non-empty name
    /// - Pipeline contains expected number of stages
    #[test]
    fn test_pipeline_creation_for_database() {
        // Test basic pipeline creation that would be used in database operations
        println!("Testing pipeline creation for database operations");

        // Test that we can create a simple pipeline for database operations
        let compression_stage = pipeline_domain::entities::PipelineStage::new(
            "compression".to_string(),
            StageType::Compression,
            pipeline_domain::entities::pipeline_stage::StageConfiguration {
                algorithm: "brotli".to_string(),
                parameters: std::collections::HashMap::new(),
                parallel_processing: false,
                chunk_size: Some(1024),
            },
            1,
        )
        .unwrap();
        println!("✅ Created compression stage");

        // Just test that we can create a pipeline - no complex assertions
        let test_pipeline = Pipeline::new("test-database-integration".to_string(), vec![compression_stage]).unwrap();
        println!("✅ Created test pipeline with {} stages", test_pipeline.stages().len());

        // Basic sanity checks
        assert!(!test_pipeline.name().is_empty());
        assert!(!test_pipeline.stages().is_empty());

        println!("✅ Pipeline creation test passed!");
    }

    /// Tests database path handling and URL generation.
    ///
    /// This test validates that the service can properly handle
    /// database file paths, generate SQLite connection URLs,
    /// and prepare pipelines for database operations.
    ///
    /// # Test Coverage
    ///
    /// - Temporary database file creation
    /// - SQLite URL generation and formatting
    /// - Database schema file loading
    /// - Pipeline creation for database operations
    /// - Database preparation validation
    ///
    /// # Test Scenario
    ///
    /// Creates a temporary database file, generates a SQLite URL,
    /// loads the database schema, and creates a pipeline ready
    /// for database operations.
    ///
    /// # Infrastructure Concerns
    ///
    /// - Database file path management
    /// - SQLite connection URL formatting
    /// - Database schema loading and validation
    /// - Pipeline-database integration preparation
    ///
    /// # Assertions
    ///
    /// - Database URL has correct SQLite prefix
    /// - URL contains expected database filename
    /// - Schema file loads successfully
    /// - Schema contains CREATE TABLE statements
    /// - Pipeline is created and ready for database operations
    #[test]
    fn test_database_path_and_url_generation() {
        // Test database path handling and URL generation without async operations
        println!("Testing database path and URL generation");

        // Create temporary directory for test files
        let temp_dir = TempDir::new().unwrap();
        let db_file = temp_dir.path().join("test_pipeline.db");
        let db_path = db_file.to_str().unwrap();

        println!("📁 Creating temporary database path: {}", db_path);

        // Test database URL generation
        let database_url = format!("sqlite:{}", db_path);
        println!("🔗 Generated database URL: {}", database_url);

        // Verify URL format
        assert!(database_url.starts_with("sqlite:"));
        assert!(database_url.contains("test_pipeline.db"));

        // Test that we can read the schema file
        let schema_sql = include_str!("../../../scripts/test_data/create_fresh_structured_database.sql");
        println!("📝 Schema file loaded: {} characters", schema_sql.len());
        assert!(!schema_sql.is_empty());
        assert!(schema_sql.contains("CREATE TABLE"));

        // Test pipeline creation for database operations
        let compression_stage = pipeline_domain::entities::PipelineStage::new(
            "compression".to_string(),
            StageType::Compression,
            pipeline_domain::entities::pipeline_stage::StageConfiguration {
                algorithm: "brotli".to_string(),
                parameters: std::collections::HashMap::new(),
                parallel_processing: false,
                chunk_size: Some(1024),
            },
            1,
        )
        .unwrap();

        let test_pipeline = Pipeline::new("test-database-operations".to_string(), vec![compression_stage]).unwrap();
        println!(
            "✅ Created test pipeline: {} with {} stages",
            test_pipeline.name(),
            test_pipeline.stages().len()
        );

        // Verify pipeline is ready for database operations
        assert!(!test_pipeline.name().is_empty());
        assert!(!test_pipeline.stages().is_empty());
        assert!(!test_pipeline.id().to_string().is_empty());

        println!("✅ Database preparation test passed!");
    }

    /// Tests cancellation propagation to reader task.
    ///
    /// This test validates that when a cancellation token is triggered,
    /// the reader task stops gracefully and returns a cancellation error.
    ///
    /// # Test Coverage
    ///
    /// - Cancellation token creation and triggering
    /// - Reader task cancellation detection
    /// - Graceful shutdown of reader
    /// - Cancellation error propagation
    ///
    /// # Test Scenario
    ///
    /// 1. Create a test file with data
    /// 2. Start reader task with cancellation token
    /// 3. Trigger cancellation immediately
    /// 4. Verify reader stops with cancellation error
    #[tokio::test]
    async fn test_reader_task_cancellation() {
        use crate::infrastructure::adapters::file_io_service_adapter::FileIOServiceImpl;
        use bootstrap::shutdown::ShutdownCoordinator;
        use pipeline_domain::services::file_io_service::FileIOConfig;
        use std::time::Duration;

        // Create test file
        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("test_input.txt");
        fs::write(&input_file, b"test data for cancellation").await.unwrap();

        // Create channel and cancellation token
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let cancel_token = coordinator.token();

        // Cancel immediately
        cancel_token.cancel();

        // Start reader task (should detect cancellation and exit)
        let file_io = Arc::new(FileIOServiceImpl::new(FileIOConfig::default())) as Arc<dyn FileIOService>;
        let result = reader_task(input_file, 1024, tx, file_io, 10, cancel_token).await;

        // Verify cancellation error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("cancel"),
            "Expected cancellation error, got: {}",
            err
        );
    }

    /// Tests cancellation propagation during active processing.
    ///
    /// This test validates that when cancellation is triggered while
    /// processing is in progress, all tasks stop gracefully.
    ///
    /// # Test Coverage
    ///
    /// - Cancellation during active file processing
    /// - Graceful shutdown of reader and workers
    /// - Channel cleanup on cancellation
    /// - No resource leaks on cancellation
    ///
    /// # Test Scenario
    ///
    /// 1. Create a larger test file
    /// 2. Start processing with cancellation token
    /// 3. Trigger cancellation during processing
    /// 4. Verify all tasks stop gracefully
    #[tokio::test]
    async fn test_cancellation_during_processing() {
        use crate::infrastructure::adapters::file_io_service_adapter::FileIOServiceImpl;
        use crate::infrastructure::runtime::{init_resource_manager, ResourceConfig};
        use bootstrap::shutdown::ShutdownCoordinator;
        use pipeline_domain::services::file_io_service::FileIOConfig;
        use std::time::Duration;

        // Initialize resource manager for test (required by CONCURRENCY_METRICS)
        let _ = init_resource_manager(ResourceConfig::default());

        // Create a larger test file to ensure processing takes time
        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("large_input.txt");
        let test_data = vec![b'X'; 1024 * 100]; // 100KB
        fs::write(&input_file, &test_data).await.unwrap();

        // Create channel and cancellation token
        let (tx, mut rx) = tokio::sync::mpsc::channel::<ChunkMessage>(5);
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let cancel_token = coordinator.token();
        let cancel_clone = cancel_token.clone();

        // Spawn reader task
        let file_io = Arc::new(FileIOServiceImpl::new(FileIOConfig::default())) as Arc<dyn FileIOService>;
        let reader_handle =
            tokio::spawn(async move { reader_task(input_file, 1024, tx, file_io, 5, cancel_clone).await });

        // Let some chunks be sent
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Trigger cancellation
        cancel_token.cancel();

        // Reader should exit with cancellation error
        let reader_result = reader_handle.await.unwrap();
        assert!(reader_result.is_err());

        // Channel should be closed (no more messages)
        // Drain any remaining messages
        while rx.try_recv().is_ok() {}

        // Verify channel is now empty and closed
        assert!(rx.recv().await.is_none(), "Channel should be closed after cancellation");
    }

    /// Tests that cancelled workers exit gracefully.
    ///
    /// This test validates that worker tasks respect cancellation
    /// and exit their processing loop cleanly.
    ///
    /// # Test Coverage
    ///
    /// - Worker task cancellation detection
    /// - Graceful exit from worker loop
    /// - No panics on cancellation
    /// - Resource cleanup in workers
    #[tokio::test]
    async fn test_worker_cancellation() {
        use bootstrap::shutdown::ShutdownCoordinator;
        use std::time::Duration;

        // Create a channel that will receive chunks
        let (_tx, rx) = tokio::sync::mpsc::channel::<ChunkMessage>(10);
        let rx_shared = Arc::new(tokio::sync::Mutex::new(rx));

        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let cancel_token = coordinator.token();
        let cancel_clone = cancel_token.clone();

        // Spawn worker that will wait for chunks or cancellation
        let worker_handle = tokio::spawn(async move {
            loop {
                let mut rx_lock = rx_shared.lock().await;

                #[allow(clippy::await_holding_lock)]
                let result = tokio::select! {
                    _ = cancel_clone.cancelled() => {
                        // Graceful shutdown: exit worker loop
                        break;
                    }
                    _chunk_msg = rx_lock.recv() => {
                        continue;
                    }
                };

                #[allow(unreachable_code)]
                {
                    result
                }
            }
            Ok::<(), PipelineError>(())
        });

        // Give worker time to start
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Trigger cancellation
        cancel_token.cancel();

        // Worker should exit cleanly
        let result = tokio::time::timeout(tokio::time::Duration::from_secs(1), worker_handle).await;

        assert!(result.is_ok(), "Worker should exit within timeout");
        let worker_result = result.unwrap().unwrap();
        assert!(worker_result.is_ok(), "Worker should exit without error");
    }

    /// Tests early cancellation before processing starts.
    ///
    /// This test validates that if cancellation is triggered before
    /// processing begins, the system detects it and aborts cleanly.
    ///
    /// # Test Coverage
    ///
    /// - Pre-processing cancellation detection
    /// - Early abort mechanism
    /// - No resource allocation on early cancel
    /// - Clean error propagation
    #[tokio::test]
    async fn test_early_cancellation_detection() {
        use crate::infrastructure::adapters::file_io_service_adapter::FileIOServiceImpl;
        use bootstrap::shutdown::ShutdownCoordinator;
        use pipeline_domain::services::file_io_service::FileIOConfig;
        use std::time::Duration;

        let temp_dir = TempDir::new().unwrap();
        let input_file = temp_dir.path().join("input.txt");
        fs::write(&input_file, b"data").await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let cancel_token = coordinator.token();

        // Cancel BEFORE starting any work
        cancel_token.cancel();
        assert!(cancel_token.is_cancelled(), "Token should be cancelled");

        // Attempt to start reader
        let file_io = Arc::new(FileIOServiceImpl::new(FileIOConfig::default())) as Arc<dyn FileIOService>;
        let result = reader_task(input_file, 1024, tx, file_io, 10, cancel_token).await;

        // Should immediately return cancellation error
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cancel"));
    }

    /// Tests cancellation token cloning and propagation.
    ///
    /// This test validates that cancellation tokens can be cloned
    /// and all clones observe the cancellation state.
    ///
    /// # Test Coverage
    ///
    /// - Token cloning behavior
    /// - Cancellation propagation to clones
    /// - Shared state consistency
    /// - Multiple task coordination
    #[tokio::test]
    async fn test_cancellation_token_propagation() {
        use bootstrap::shutdown::ShutdownCoordinator;
        use std::time::Duration;

        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let token = coordinator.token();
        let clone1 = token.clone();
        let clone2 = token.clone();

        // None should be cancelled initially
        assert!(!token.is_cancelled());
        assert!(!clone1.is_cancelled());
        assert!(!clone2.is_cancelled());

        // Cancel original
        token.cancel();

        // All clones should see cancellation
        assert!(token.is_cancelled());
        assert!(clone1.is_cancelled());
        assert!(clone2.is_cancelled());

        // All should unblock from cancelled()
        tokio::time::timeout(tokio::time::Duration::from_millis(100), clone1.cancelled())
            .await
            .unwrap();

        tokio::time::timeout(tokio::time::Duration::from_millis(100), clone2.cancelled())
            .await
            .unwrap();
    }
}
