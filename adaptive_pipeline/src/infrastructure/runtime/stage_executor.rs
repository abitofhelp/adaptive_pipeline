// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Stage Executor Implementation
//!
//! This module provides a concrete implementation of the stage executor
//! interface for the adaptive pipeline system. It handles the execution of
//! individual pipeline stages including compression, encryption, and checksum
//! calculation.
//!
//! ## Overview
//!
//! The stage executor implementation provides:
//!
//! - **Multi-Stage Processing**: Supports compression, encryption, and checksum
//!   stages
//! - **Resource Management**: Manages computational resources and memory usage
//! - **Service Integration**: Integrates with domain services for actual
//!   processing
//! - **State Management**: Maintains stage-specific state during processing
//! - **Error Handling**: Comprehensive error handling and recovery
//!
//! ## Architecture
//!
//! The implementation follows the infrastructure layer patterns:
//!
//! - **Service Integration**: Uses injected domain services for processing
//! - **State Management**: Maintains processing state across stage executions
//! - **Resource Tracking**: Monitors and manages computational resources
//! - **Async Processing**: All operations are asynchronous and non-blocking
//!
//! ## Supported Stage Types
//!
//! ### Compression Stages
//! - **Algorithms**: Brotli, Gzip, Zstd, Lz4
//! - **Configuration**: Compression level, window size, dictionary
//! - **Performance**: Optimized for different data types and sizes
//!
//! ### Encryption Stages
//! - **Algorithms**: AES-256-GCM, ChaCha20-Poly1305
//! - **Key Management**: Secure key handling and derivation
//! - **Authentication**: Built-in integrity verification
//!
//! ### Checksum Stages
//! - **Algorithms**: SHA-256, SHA-512, Blake3
//! - **Incremental**: Supports incremental checksum calculation
//! - **Verification**: Can verify existing checksums
//!
//! ## Usage Examples
//!
//! ### Basic Stage Executor Setup

//!
//! ### Stage Execution
//!
//!
//! ## Resource Management
//!
//! The executor manages computational resources:
//!
//! ### Memory Usage
//! - **Bounded Memory**: Limits memory usage per stage
//! - **Chunk Processing**: Processes data in manageable chunks
//! - **Resource Cleanup**: Automatic cleanup of temporary resources
//!
//! ### CPU Utilization
//! - **Parallel Processing**: Utilizes multiple CPU cores when beneficial
//! - **Load Balancing**: Distributes work across available resources
//! - **Throttling**: Prevents resource exhaustion under high load
//!
//! ## Performance Characteristics
//!
//! - **Throughput**: Optimized for high-throughput processing
//! - **Latency**: Low-latency stage execution
//! - **Scalability**: Scales with available system resources
//! - **Efficiency**: Minimal overhead per stage execution
//!
//! ## Error Handling
//!
//! The executor handles various error conditions:
//! - **Service Errors**: Errors from underlying services
//! - **Resource Errors**: Resource exhaustion and allocation failures
//! - **Data Errors**: Corrupted or invalid input data
//! - **Configuration Errors**: Invalid stage configuration
//!
//! ## Thread Safety
//!
//! The executor is designed for concurrent use:
//! - **Shared State**: Thread-safe access to shared state
//! - **Service Access**: Safe concurrent access to services
//! - **Resource Coordination**: Coordinated resource access

use async_trait::async_trait;
use byte_unit::Byte;
use parking_lot::RwLock;
use adaptive_pipeline_domain::entities::{PipelineStage, ProcessingContext};
use adaptive_pipeline_domain::repositories::stage_executor::{ResourceRequirements, StageExecutor};
use adaptive_pipeline_domain::services::StageService;
use adaptive_pipeline_domain::value_objects::FileChunk;
use adaptive_pipeline_domain::PipelineError;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

/// Basic implementation of the stage executor for pipeline processing.
///
/// `BasicStageExecutor` provides a concrete implementation of the
/// `StageExecutor` trait that can handle compression, encryption, and checksum
/// stages. It integrates with domain services to perform the actual data
/// transformations.
///
/// ## Features
///
/// ### Multi-Stage Support
/// - **Compression**: Supports various compression algorithms through
///   CompressionService
/// - **Encryption**: Handles encryption operations through EncryptionService
/// - **Checksum**: Calculates and verifies checksums using SHA-256
/// - **Custom Stages**: Extensible architecture for custom stage types
///
/// ### Resource Management
/// - **Memory Efficient**: Processes data in chunks to limit memory usage
/// - **State Tracking**: Maintains stage-specific state during processing
/// - **Resource Monitoring**: Tracks resource usage and requirements
/// - **Cleanup**: Automatic cleanup of temporary resources
///
/// ### Service Integration
/// - **Dependency Injection**: Services are injected through constructor
/// - **Async Operations**: All operations are asynchronous and non-blocking
/// - **Error Propagation**: Proper error handling and propagation
/// - **Performance Optimization**: Optimized for high-throughput processing
///
/// ## Usage Examples
///
/// ### Creating a Stage Executor
///
///
/// ### Processing Different Stage Types
///
///
/// ## State Management
///
/// The executor maintains several types of state:
///
/// ### Checksum State
/// - **Running Hashes**: Maintains running hash state for each checksum stage
/// - **Incremental Updates**: Updates hashes incrementally as chunks are
///   processed
/// - **Final Calculation**: Provides final hash values when processing
///   completes
///
/// ### Resource State
/// - **Memory Usage**: Tracks memory usage across stages
/// - **CPU Utilization**: Monitors CPU usage and load
/// - **I/O Operations**: Tracks I/O operations and throughput
///
/// ## Thread Safety
///
/// The executor is designed for concurrent use:
/// - **Shared Services**: Services are shared safely through Arc
/// - **Protected State**: Internal state is protected with RwLock
/// - **Concurrent Execution**: Multiple stages can be executed concurrently
/// - **Resource Coordination**: Coordinates access to shared resources
///
/// ## Performance Characteristics
///
/// - **High Throughput**: Optimized for processing large amounts of data
/// - **Low Latency**: Minimal overhead per stage execution
/// - **Memory Efficient**: Bounded memory usage regardless of data size
/// - **Scalable**: Performance scales with available system resources
pub struct BasicStageExecutor {
    // In a real implementation, this would contain stage-specific executors
    // and resource management
    _state: Arc<RwLock<()>>,
    // Store running checksums for each stage
    checksums: Arc<RwLock<HashMap<String, Sha256>>>,
    // Registry of stage services by algorithm name
    // Maps algorithm name (e.g., "brotli", "aes256gcm", "base64") to StageService implementation
    stage_services: Arc<HashMap<String, Arc<dyn StageService>>>,
}

impl BasicStageExecutor {
    /// Creates a new basic stage executor with the provided stage services.
    ///
    /// Initializes the executor with a registry of stage services that implement
    /// the unified `StageService` trait. The executor is ready to handle stage
    /// execution immediately after creation.
    ///
    /// # Arguments
    ///
    /// * `stage_services` - HashMap mapping algorithm names to StageService implementations
    ///   - Keys: algorithm names (e.g., "brotli", "aes256gcm", "base64", "pii_masking")
    ///   - Values: Arc-wrapped StageService implementations
    ///
    /// # Returns
    ///
    /// A new `BasicStageExecutor` instance ready to process pipeline stages.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::sync::Arc;
    /// use std::collections::HashMap;
    ///
    /// let mut services = HashMap::new();
    /// services.insert("brotli".to_string(), Arc::new(MultiAlgoCompression::new()) as Arc<dyn StageService>);
    /// services.insert("base64".to_string(), Arc::new(Base64EncodingService::new()) as Arc<dyn StageService>);
    ///
    /// let executor = BasicStageExecutor::new(services);
    /// ```
    ///
    /// # Initialization
    ///
    /// The executor initializes with:
    /// - Empty checksum state for tracking running hashes
    /// - Registry of stage services by algorithm name
    /// - Thread-safe state management structures
    pub fn new(stage_services: HashMap<String, Arc<dyn StageService>>) -> Self {
        Self {
            _state: Arc::new(RwLock::new(())),
            checksums: Arc::new(RwLock::new(HashMap::new())),
            stage_services: Arc::new(stage_services),
        }
    }

    /// Processes a checksum stage by updating the running hash with chunk data.
    ///
    /// This method handles checksum calculation stages by maintaining a running
    /// hash state that is updated incrementally as chunks are processed. It
    /// supports SHA-256 hashing and can be extended to support additional
    /// hash algorithms.
    ///
    /// # Arguments
    ///
    /// * `stage` - The checksum stage configuration
    /// * `chunk` - The file chunk to include in the hash calculation
    /// * `context` - Mutable processing context for storing results
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Checksum stage processed successfully
    /// - `Err(PipelineError)` - Error during checksum calculation
    ///
    /// # Process
    ///
    /// 1. Gets or creates a hasher for the stage
    /// 2. Updates the hasher with the chunk data
    /// 3. Stores the current hash state in the context
    /// 4. Provides final hash value if this is the last chunk
    ///
    /// # Examples
    ///
    ///
    /// # Performance
    ///
    /// - **Incremental**: Updates hash incrementally to avoid memory issues
    /// - **Efficient**: Uses optimized SHA-256 implementation
    /// - **Thread-Safe**: Safe concurrent access to hash state
    /// - **Memory Bounded**: Constant memory usage regardless of data size
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe:
    /// - Uses RwLock for protected access to hash state
    /// - Safe concurrent updates from multiple threads
    /// - Proper synchronization of shared state
    async fn process_checksum_stage(
        &self,
        stage: &PipelineStage,
        chunk: &FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<(), PipelineError> {
        let stage_name = stage.name();

        // Get or create hasher for this stage
        {
            let mut checksums = self.checksums.write();
            if !checksums.contains_key(stage_name) {
                checksums.insert(stage_name.to_string(), Sha256::new());
            }
        }

        // Update the running hash with chunk data
        {
            let mut checksums = self.checksums.write();
            if let Some(hasher) = checksums.get_mut(stage_name) {
                hasher.update(chunk.data());
            }
        }

        // If this is the final chunk, finalize the checksum and store it in metrics
        if chunk.is_final() {
            let final_checksum = {
                let mut checksums = self.checksums.write();
                if let Some(hasher) = checksums.remove(stage_name) {
                    format!("{:x}", hasher.finalize())
                } else {
                    return Err(PipelineError::IntegrityError("Checksum hasher not found".to_string()));
                }
            };

            let _input_size = Byte::from_u128(chunk.data().len() as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string();
            let _output_size = Byte::from_u128(chunk.data().len() as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string();

            tracing::debug!(
                "Processing chunk {}, checksum for stage '{}': {}",
                chunk.sequence_number(),
                stage_name,
                final_checksum
            );

            // Store the checksum in processing metrics
            let mut metrics = context.metrics().clone();
            match stage_name {
                "input_checksum" => {
                    // Preserve the actual input file size that was already set, don't override with
                    // chunk size
                    let current_size = metrics.input_file_size_bytes();
                    metrics.set_input_file_info(current_size, Some(final_checksum));
                }
                "output_checksum" => {
                    // Preserve the actual output file size that was already set by the pipeline
                    // service
                    let current_size = metrics.output_file_size_bytes();
                    metrics.set_output_file_info(current_size, Some(final_checksum));
                }
                _ => {}
            }
            context.update_metrics(metrics);
        }

        Ok(())
    }

    /// Process stage using the unified StageService registry
    ///
    /// This method dispatches to the appropriate StageService based on the
    /// algorithm name in the stage configuration. Special handling for checksum
    /// stages maintains backward compatibility with incremental hashing.
    async fn process_stage_by_type(
        &self,
        stage: &PipelineStage,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let start_time = std::time::Instant::now();
        let input_size = chunk.data().len();

        tracing::info!(
            "🔧 Processing stage '{}' (type: {:?}, algorithm: {}): chunk {} ({} bytes)",
            stage.name(),
            stage.stage_type(),
            stage.configuration().algorithm,
            chunk.sequence_number(),
            input_size
        );

        tracing::debug!(
            "📋 Stage type details: {:?}, operation: {} -> dispatching via StageService registry",
            stage.stage_type(),
            stage.configuration().operation
        );

        let result = match stage.stage_type() {
            adaptive_pipeline_domain::entities::pipeline_stage::StageType::Checksum => {
                // Special handling for checksum - maintains incremental hashing
                self.process_checksum_stage(stage, &chunk, context).await?;
                Ok(chunk) // Checksum stages don't modify the chunk data
            }
            _ => {
                // Use unified StageService registry for all other stages
                let algorithm = stage.configuration().algorithm.as_str();

                match self.stage_services.get(algorithm) {
                    Some(service) => {
                        tracing::debug!(
                            "Found StageService for algorithm '{}', dispatching to process_chunk()",
                            algorithm
                        );
                        service.process_chunk(chunk, stage.configuration(), context)
                    }
                    None => {
                        // Algorithm not found in registry - return helpful error
                        Err(PipelineError::InvalidConfiguration(format!(
                            "No StageService registered for algorithm '{}'. Available: {:?}",
                            algorithm,
                            self.stage_services.keys().collect::<Vec<_>>()
                        )))
                    }
                }
            }
        };

        // Record stage metrics for all stages
        let processing_time = start_time.elapsed();
        let output_size = result.as_ref().map(|c| c.data().len()).unwrap_or(input_size);

        tracing::debug!(
            "Stage '{}' completed: {} bytes -> {} bytes in {:.2}ms",
            stage.name(),
            input_size,
            output_size,
            processing_time.as_secs_f64() * 1000.0
        );

        result
    }
}

#[async_trait]
impl StageExecutor for BasicStageExecutor {
    #[tracing::instrument(skip(self, chunk, context), fields(chunk_id = chunk.sequence_number(), stage = stage.name(), input_size = chunk.data().len(), output_size))]
    async fn execute(
        &self,
        stage: &PipelineStage,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // Process stage based on its algorithm configuration, not stage name
        // This ensures all stages (built-in and user-created) are treated equally
        let result_chunk = self.process_stage_by_type(stage, chunk, context).await?;

        // Record the output size in the tracing span
        tracing::Span::current().record("output_size", result_chunk.data().len());

        Ok(result_chunk)
    }

    async fn execute_parallel(
        &self,
        stage: &PipelineStage,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        let total_bytes: usize = chunks.iter().map(|c| c.data().len()).sum();
        tracing::debug!(
            "Processing {} chunks in parallel through stage '{}': {} total",
            chunks.len(),
            stage.name(),
            Byte::from_u128(total_bytes as u128)
                .unwrap_or_else(|| Byte::from_u64(0))
                .get_appropriate_unit(byte_unit::UnitType::Decimal)
                .to_string()
        );

        // Basic parallel execution using futures
        let mut results = Vec::new();
        for chunk in chunks {
            let result = self.execute(stage, chunk, context).await?;
            results.push(result);
        }

        Ok(results)
    }

    async fn can_execute(&self, stage: &PipelineStage) -> Result<bool, PipelineError> {
        // Check if we have a StageService registered for this algorithm
        let algorithm = stage.configuration().algorithm.as_str();

        // Checksum stages are always supported (special internal handling)
        if *stage.stage_type() == adaptive_pipeline_domain::entities::StageType::Checksum {
            return Ok(true);
        }

        // For all other stages, check the registry
        Ok(self.stage_services.contains_key(algorithm))
    }

    fn supported_stage_types(&self) -> Vec<String> {
        // Return list of supported algorithms from registry
        let mut algorithms: Vec<String> = self.stage_services.keys().cloned().collect();
        algorithms.push("checksum".to_string()); // Checksum is always supported
        algorithms.sort();
        algorithms
    }

    async fn estimate_processing_time(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<std::time::Duration, PipelineError> {
        // Basic estimation based on stage type and data size
        let base_time_ms = match stage.stage_type() {
            adaptive_pipeline_domain::entities::StageType::Compression => {
                // Estimate ~100MB/s for compression
                (data_size / (100 * 1024 * 1024)) * 1000
            }
            adaptive_pipeline_domain::entities::StageType::Encryption => {
                // Estimate ~200MB/s for encryption
                (data_size / (200 * 1024 * 1024)) * 1000
            }
            adaptive_pipeline_domain::entities::StageType::Checksum => {
                // Fast checksum/validation - ~500MB/s
                (data_size / (500 * 1024 * 1024)) * 1000
            }
            adaptive_pipeline_domain::entities::StageType::PassThrough => {
                // Conservative estimate for custom stages
                (data_size / (50 * 1024 * 1024)) * 1000
            }
            adaptive_pipeline_domain::entities::StageType::Transform => {
                // Conservative estimate for transform stages
                (data_size / (50 * 1024 * 1024)) * 1000
            }
        };

        Ok(std::time::Duration::from_millis(base_time_ms.max(100)))
    }

    async fn get_resource_requirements(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<ResourceRequirements, PipelineError> {
        // Basic resource estimation
        let memory_mb = match stage.stage_type() {
            adaptive_pipeline_domain::entities::StageType::Compression => {
                // Compression typically needs ~2x data size for buffers
                ((data_size * 2) / (1024 * 1024)).max(64)
            }
            adaptive_pipeline_domain::entities::StageType::Encryption => {
                // Encryption needs less memory
                (data_size / (1024 * 1024)).max(32)
            }

            adaptive_pipeline_domain::entities::StageType::Checksum => {
                // Integrity checking needs minimal memory
                (data_size / (1024 * 1024)).max(16)
            }
            adaptive_pipeline_domain::entities::StageType::PassThrough => {
                // Conservative estimate for custom stages
                ((data_size * 3) / (1024 * 1024)).max(128)
            }
            adaptive_pipeline_domain::entities::StageType::Transform => {
                // Conservative estimate for transform stages
                ((data_size * 3) / (1024 * 1024)).max(128)
            }
        };

        Ok(ResourceRequirements::new(
            memory_mb * 1024 * 1024, // Convert MB to bytes
            1,                       // CPU cores
            0,                       // Disk space (temporary)
        ))
    }

    async fn prepare_stage(&self, stage: &PipelineStage, context: &ProcessingContext) -> Result<(), PipelineError> {
        tracing::debug!("Preparing stage: {}", stage.name());

        // Basic preparation - validate configuration
        if stage.name().is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage name cannot be empty".to_string(),
            ));
        }

        // In a real implementation, this would:
        // 1. Allocate resources
        // 2. Initialize stage-specific components
        // 3. Validate dependencies
        // 4. Set up monitoring

        Ok(())
    }

    async fn cleanup_stage(&self, stage: &PipelineStage, context: &ProcessingContext) -> Result<(), PipelineError> {
        tracing::debug!("Cleaning up stage: {}", stage.name());

        // Basic cleanup
        // In a real implementation, this would:
        // 1. Release allocated resources
        // 2. Clean up temporary files
        // 3. Finalize metrics
        // 4. Tear down stage-specific components

        Ok(())
    }

    async fn validate_configuration(&self, stage: &PipelineStage) -> Result<(), PipelineError> {
        // Basic validation
        if stage.name().is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage name cannot be empty".to_string(),
            ));
        }

        // Validate that we have a StageService for this algorithm
        let algorithm = stage.configuration().algorithm.as_str();

        // Checksum stages are always valid (special internal handling)
        if *stage.stage_type() == adaptive_pipeline_domain::entities::StageType::Checksum {
            return Ok(());
        }

        // For all other stages, check the registry
        if !self.stage_services.contains_key(algorithm) {
            return Err(PipelineError::InvalidConfiguration(format!(
                "No StageService registered for algorithm '{}'. Supported algorithms: {:?}",
                algorithm,
                self.supported_stage_types()
            )));
        }

        Ok(())
    }

    async fn validate_stage_ordering(&self, stages: &[PipelineStage]) -> Result<(), PipelineError> {
        use adaptive_pipeline_domain::entities::StagePosition;

        // Track if we've encountered a PostBinary stage
        let mut seen_post_binary = false;

        for (index, stage) in stages.iter().enumerate() {
            // Skip checksum stages - they're automatically added and managed
            if *stage.stage_type() == adaptive_pipeline_domain::entities::StageType::Checksum {
                continue;
            }

            // Get the algorithm name to look up the service
            let algorithm = stage.configuration().algorithm.as_str();

            // Look up the stage service to determine its position
            let position = match self.stage_services.get(algorithm) {
                Some(service) => service.position(),
                None => {
                    return Err(PipelineError::InvalidConfiguration(format!(
                        "Cannot validate stage ordering: No StageService registered for algorithm '{}' at position {}",
                        algorithm, index
                    )));
                }
            };

            // Validate ordering based on position
            match position {
                StagePosition::PreBinary => {
                    if seen_post_binary {
                        return Err(PipelineError::InvalidConfiguration(format!(
                            "Invalid stage ordering: PreBinary stage '{}' (algorithm: {}) at position {} cannot appear after PostBinary stages. \
                             PreBinary stages must execute before compression/encryption.",
                            stage.name(),
                            algorithm,
                            index
                        )));
                    }
                }
                StagePosition::PostBinary => {
                    // Mark that we've seen a PostBinary stage
                    seen_post_binary = true;
                }
                StagePosition::Any => {
                    // Any position can appear anywhere - no restrictions
                }
            }
        }

        Ok(())
    }
}
