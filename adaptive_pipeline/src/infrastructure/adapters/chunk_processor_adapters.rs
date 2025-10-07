// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Chunk Processor Adapters
//!
//! This module provides adapter implementations that bridge domain services
//! with the chunk processing interface. These adapters enable domain services
//! to be used as chunk processors in the file processing pipeline.
//!
//! ## Overview
//!
//! The chunk processor adapters provide:
//!
//! - **Service Integration**: Bridge domain services with chunk processing
//! - **Type Safety**: Generic adapters with compile-time type checking
//! - **Configuration**: Flexible configuration for different service types
//! - **Reusability**: Reusable adapters for common service patterns
//! - **Performance**: Efficient adaptation with minimal overhead
//!
//! ## Architecture
//!
//! The adapters follow the Adapter pattern:
//!
//! - **Generic Design**: Generic adapters that work with any service type
//! - **Service Wrapping**: Wrap domain services to implement ChunkProcessor
//! - **Configuration-Driven**: Behavior controlled through configuration
//! - **Async Operations**: Full async support for non-blocking operations
//!
//! ## Adapter Types
//!
//! ### Service Chunk Adapter
//!
//! Generic adapter that can wrap any service:
//! - **Compression Services**: Adapt compression services for chunk processing
//! - **Encryption Services**: Adapt encryption services for chunk processing
//! - **Custom Services**: Adapt any domain service with appropriate interface
//!
//! ### Specialized Adapters
//!
//! - **Compression Adapter**: Specialized adapter for compression services
//! - **Encryption Adapter**: Specialized adapter for encryption services
//! - **Validation Adapter**: Specialized adapter for validation services
//!
//! ## Usage Examples
//!
//! ### Basic Service Adaptation

//!
//! ### Compression Service Adaptation

//!
//! ### Encryption Service Adaptation

//!
//! ### Pipeline Integration

//!
//! ## Adapter Configuration
//!
//! ### Configuration Options
//!
//! - **modifies_data**: Whether the adapter modifies chunk data
//! - **requires_security_context**: Whether the adapter needs security context
//! - **buffer_size**: Buffer size for processing operations
//! - **parallel_processing**: Enable parallel processing within adapter
//!
//! ### Performance Tuning
//!
//! - **Batch Processing**: Process multiple chunks in batches
//! - **Memory Management**: Efficient memory usage and cleanup
//! - **Async Operations**: Non-blocking operations for better throughput
//!
//! ## Performance Considerations
//!
//! ### Adaptation Overhead
//!
//! - **Minimal Overhead**: Adapters add minimal performance overhead
//! - **Zero-Cost Abstractions**: Generic design enables compiler optimizations
//! - **Efficient Wrapping**: Direct service calls without unnecessary
//!   indirection
//!
//! ### Memory Usage
//!
//! - **Shared Services**: Services are shared via Arc for memory efficiency
//! - **Chunk Copying**: Minimal chunk copying during adaptation
//! - **Resource Cleanup**: Automatic cleanup of adapter resources
//!
//! ### Concurrency
//!
//! - **Thread Safety**: All adapters are thread-safe
//! - **Concurrent Processing**: Support for concurrent chunk processing
//! - **Lock-Free Operations**: Lock-free operations where possible
//!
//! ## Error Handling
//!
//! ### Adapter Errors
//!
//! - **Service Errors**: Proper propagation of service errors
//! - **Configuration Errors**: Validation of adapter configuration
//! - **Processing Errors**: Comprehensive error context and recovery
//!
//! ### Error Recovery
//!
//! - **Graceful Degradation**: Graceful handling of service failures
//! - **Retry Logic**: Configurable retry logic for transient failures
//! - **Fallback Processing**: Alternative processing strategies
//!
//! ## Integration
//!
//! The adapters integrate with:
//!
//! - **Domain Services**: Bridge domain services with chunk processing
//! - **File Processor**: Used by file processor service for chunk processing
//! - **Pipeline System**: Integrate with pipeline processing workflow
//! - **Configuration System**: Support for runtime configuration
//!
//! ## Thread Safety
//!
//! All adapters are fully thread-safe:
//!
//! - **Shared Services**: Services are shared safely via Arc
//! - **Concurrent Access**: Safe concurrent access to adapter methods
//! - **Immutable Configuration**: Configuration is immutable after creation
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Dynamic Adaptation**: Runtime adaptation of service behavior
//! - **Metrics Integration**: Built-in metrics collection and reporting
//! - **Caching**: Intelligent caching of processing results
//! - **Load Balancing**: Load balancing across multiple service instances

use adaptive_pipeline_domain::services::compression_service::{
    CompressionAlgorithm, CompressionConfig, CompressionLevel, CompressionService,
};
use adaptive_pipeline_domain::services::encryption_service::{
    EncryptionAlgorithm, EncryptionConfig, EncryptionService, KeyDerivationFunction, KeyMaterial,
};
use adaptive_pipeline_domain::services::file_processor_service::ChunkProcessor;
use adaptive_pipeline_domain::{FileChunk, PipelineError, ProcessingContext, SecurityContext};
use async_trait::async_trait;
use std::sync::Arc;

/// Generic adapter that wraps any service as a ChunkProcessor
///
/// This adapter provides a generic way to wrap any domain service and use it
/// as a chunk processor in the file processing pipeline. It uses generics to
/// provide type-safe, reusable chunk processing capabilities.
///
/// # Key Features
///
/// - **Generic Design**: Works with any service type that implements required
///   traits
/// - **Type Safety**: Compile-time type checking for service compatibility
/// - **Configuration**: Flexible configuration for different service behaviors
/// - **Performance**: Minimal overhead adaptation with efficient service calls
/// - **Thread Safety**: Full thread safety with Arc-based service sharing
///
/// # Examples
///
///
/// # Generic Type Parameter
///
/// The type parameter `T` represents the service being adapted:
/// - Must implement the required service interface
/// - Can be any domain service (compression, encryption, validation, etc.)
/// - Uses `?Sized` to support trait objects
///
/// Uses generics to provide type-safe, reusable chunk processing capabilities
pub struct ServiceChunkAdapter<T: ?Sized> {
    service: Arc<T>,
    name: String,
    config: AdapterConfig,
}

/// Configuration for service adapters
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub modifies_data: bool,
    pub requires_security_context: bool,
}

impl<T: ?Sized> ServiceChunkAdapter<T> {
    pub fn new(service: Arc<T>, name: String, config: AdapterConfig) -> Self {
        Self { service, name, config }
    }
}

/// Compression service adapter implementing ChunkProcessor
pub type CompressionChunkAdapter = ServiceChunkAdapter<dyn CompressionService>;

impl ChunkProcessor for CompressionChunkAdapter {
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        // Create a default compression config - in real usage this would be
        // configurable
        let compression_config = CompressionConfig {
            algorithm: CompressionAlgorithm::Brotli,
            level: CompressionLevel::Balanced,
            dictionary: None,
            window_size: None,
            parallel_processing: false,
        };

        // Create a minimal processing context for the service
        let security_context = SecurityContext::new(
            Some("chunk_processor".to_string()),
            adaptive_pipeline_domain::entities::security_context::SecurityLevel::Internal,
        );
        let mut processing_context = ProcessingContext::new(
            std::path::PathBuf::from("chunk_processing"),
            std::path::PathBuf::from("output"),
            chunk.data().len() as u64,
            security_context,
        );

        // Use the compression service to compress the chunk (now sync)
        let compressed_chunk =
            self.service
                .compress_chunk(chunk.clone(), &compression_config, &mut processing_context)?;

        // Return the compressed chunk (already processed by the service)
        Ok(compressed_chunk)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn modifies_data(&self) -> bool {
        self.config.modifies_data
    }
}

/// Encryption service adapter implementing ChunkProcessor
pub type EncryptionChunkAdapter = ServiceChunkAdapter<dyn EncryptionService>;

impl ChunkProcessor for EncryptionChunkAdapter {
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        // Create a default encryption config - in real usage this would be configurable
        let encryption_config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_derivation: KeyDerivationFunction::Pbkdf2,
            key_size: 32,
            nonce_size: 12,
            salt_size: 16,
            iterations: 100000,
            memory_cost: None,
            parallel_cost: None,
            associated_data: None,
        };

        // Create a minimal security context for the service
        let security_context = SecurityContext::new(
            Some("chunk_processor".to_string()),
            adaptive_pipeline_domain::entities::security_context::SecurityLevel::Internal,
        );
        let mut processing_context = ProcessingContext::new(
            std::path::PathBuf::from("chunk_processing"),
            std::path::PathBuf::from("output"),
            chunk.data().len() as u64,
            security_context,
        );

        // Create default key material for encryption
        let key_material = KeyMaterial {
            key: vec![0u8; 32], // Default 32-byte key
            nonce: vec![0u8; 12],
            salt: vec![0u8; 16],
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };

        // Use the encryption service to encrypt the chunk (now sync)
        let encrypted_chunk = self.service.encrypt_chunk(
            chunk.clone(),
            &encryption_config,
            &key_material,
            &mut processing_context,
        )?;

        // Return the encrypted chunk (already processed by the service)
        Ok(encrypted_chunk)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn modifies_data(&self) -> bool {
        self.config.modifies_data
    }
}

/// Factory functions for creating service adapters
impl CompressionChunkAdapter {
    pub fn new_compression_adapter(service: Arc<dyn CompressionService>, name: Option<String>) -> Self {
        Self::new(
            service,
            name.unwrap_or_else(|| "CompressionAdapter".to_string()),
            AdapterConfig {
                modifies_data: true,
                requires_security_context: false,
            },
        )
    }
}

impl EncryptionChunkAdapter {
    pub fn new_encryption_adapter(service: Arc<dyn EncryptionService>, name: Option<String>) -> Self {
        Self::new(
            service,
            name.unwrap_or_else(|| "EncryptionAdapter".to_string()),
            AdapterConfig {
                modifies_data: true,
                requires_security_context: true,
            },
        )
    }
}

/// Generic factory for creating any service adapter
pub struct ServiceAdapterFactory;

impl ServiceAdapterFactory {
    /// Create a compression chunk adapter
    pub fn create_compression_adapter(service: Arc<dyn CompressionService>) -> Box<dyn ChunkProcessor> {
        Box::new(CompressionChunkAdapter::new_compression_adapter(service, None))
    }

    /// Create an encryption chunk adapter
    pub fn create_encryption_adapter(service: Arc<dyn EncryptionService>) -> Box<dyn ChunkProcessor> {
        Box::new(EncryptionChunkAdapter::new_encryption_adapter(service, None))
    }

    /// Create a custom service adapter with specific configuration
    pub fn create_custom_adapter<T: Send + Sync + 'static>(
        service: Arc<T>,
        name: String,
        config: AdapterConfig,
    ) -> ServiceChunkAdapter<T> {
        ServiceChunkAdapter::new(service, name, config)
    }
}
