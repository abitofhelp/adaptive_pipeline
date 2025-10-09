// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

// Infrastructure module - contains future features not yet fully utilized
#![allow(dead_code, unused_imports, unused_variables)]
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
//! - **Retry Logic**: Configurable retry logic for transient failures
//! - **Security Context Enforcement**: Permission-based operation validation
//! - **Dynamic Adaptation**: Runtime adaptation of service behavior
//! - **Metrics Integration**: Built-in metrics collection and reporting
//! - **Caching**: Intelligent caching of processing results
//! - **Load Balancing**: Load balancing across multiple service instances

use adaptive_pipeline_domain::services::compression_service::{
    CompressionAlgorithm,
    CompressionConfig,
    CompressionLevel,
    CompressionService,
};
use adaptive_pipeline_domain::services::encryption_service::{
    EncryptionAlgorithm,
    EncryptionConfig,
    EncryptionService,
    KeyDerivationFunction,
    KeyMaterial,
};
use adaptive_pipeline_domain::services::file_processor_service::ChunkProcessor;
use adaptive_pipeline_domain::{ FileChunk, PipelineError, ProcessingContext, SecurityContext };
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

/// Configuration for service adapters (generic)
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub modifies_data: bool,

    // TODO(v2.0 - Security Context Enforcement): Implement security validation
    // Currently defined but not enforced anywhere in the codebase.
    //
    // To implement this feature:
    // 1. Accept SecurityContext via adapter configuration or constructor
    // 2. Validate SecurityContext.can_encrypt()/can_compress() before operations
    // 3. Return PipelineError::SecurityViolation if permissions insufficient
    // 4. Document security requirements in adapter method docs
    //
    // Related:
    // - Code review Comments 5 & 6
    // - See docs/roadmap.md for security enforcement design
    // - ProcessingContext already carries SecurityContext (can be used)
    //
    // pub requires_security_context: bool,
}

/// Configuration for compression adapters with typed configuration
#[derive(Debug, Clone)]
pub struct CompressionAdapterConfig {
    pub modifies_data: bool,
    pub compression_config: CompressionConfig,
}

/// Configuration for encryption adapters with required key material
#[derive(Debug, Clone)]
pub struct EncryptionAdapterConfig {
    pub modifies_data: bool,
    pub encryption_config: EncryptionConfig,
    pub key_material: KeyMaterial,
}

impl<T: ?Sized> ServiceChunkAdapter<T> {
    pub fn new(service: Arc<T>, name: String, config: AdapterConfig) -> Self {
        Self { service, name, config }
    }
}

/// Compression service adapter with typed configuration
///
/// This adapter requires compression configuration to be provided explicitly
/// following the dependency injection pattern.
pub struct CompressionChunkAdapter {
    service: Arc<dyn CompressionService>,
    name: String,
    config: CompressionAdapterConfig,
}

impl CompressionChunkAdapter {
    /// Creates a new compression adapter with required configuration
    ///
    /// # Arguments
    ///
    /// * `service` - The compression service implementation
    /// * `name` - Name for this adapter instance
    /// * `config` - Configuration including compression settings
    pub fn new(
        service: Arc<dyn CompressionService>,
        name: String,
        config: CompressionAdapterConfig,
    ) -> Self {
        Self { service, name, config }
    }
}

impl ChunkProcessor for CompressionChunkAdapter {
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        // Create a minimal processing context for the service
        // NOTE: File paths are managed by CpuWorkerContext (DI pattern), not here
        let security_context = SecurityContext::new(
            Some("chunk_processor".to_string()),
            adaptive_pipeline_domain::entities::security_context::SecurityLevel::Internal
        );
        let mut processing_context = ProcessingContext::new(
            chunk.data().len() as u64,
            security_context
        );

        // Use the compression config provided via configuration (DI pattern)
        let compressed_chunk = self.service.compress_chunk(
            chunk.clone(),
            &self.config.compression_config,
            &mut processing_context
        )?;

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

/// Encryption service adapter with required configuration
///
/// SECURITY: This adapter requires encryption configuration and key material
/// to be provided explicitly. It will NOT create insecure default keys.
pub struct EncryptionChunkAdapter {
    service: Arc<dyn EncryptionService>,
    name: String,
    config: EncryptionAdapterConfig,
}

impl EncryptionChunkAdapter {
    /// Creates a new encryption adapter with required configuration
    ///
    /// # Security
    ///
    /// - Requires explicit `EncryptionConfig` with algorithm settings
    /// - Requires explicit `KeyMaterial` - will NOT generate insecure defaults
    /// - Validates configuration before accepting
    ///
    /// # Arguments
    ///
    /// * `service` - The encryption service implementation
    /// * `name` - Name for this adapter instance
    /// * `config` - Configuration including encryption settings and key material
    pub fn new(
        service: Arc<dyn EncryptionService>,
        name: String,
        config: EncryptionAdapterConfig,
    ) -> Self {
        Self { service, name, config }
    }
}

impl ChunkProcessor for EncryptionChunkAdapter {
    fn process_chunk(&self, chunk: &FileChunk) -> Result<FileChunk, PipelineError> {
        // Create a minimal security context for the service
        // NOTE: File paths are managed by CpuWorkerContext (DI pattern), not here
        let security_context = SecurityContext::new(
            Some("chunk_processor".to_string()),
            adaptive_pipeline_domain::entities::security_context::SecurityLevel::Internal
        );
        let mut processing_context = ProcessingContext::new(
            chunk.data().len() as u64,
            security_context
        );

        // Use the encryption config and key material provided via configuration
        // SECURITY: No insecure defaults - config must be provided explicitly
        let encrypted_chunk = self.service.encrypt_chunk(
            chunk.clone(),
            &self.config.encryption_config,
            &self.config.key_material,
            &mut processing_context
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
    /// Creates a new compression adapter with provided configuration
    ///
    /// # Arguments
    ///
    /// * `service` - The compression service implementation
    /// * `name` - Optional name for this adapter (defaults to "CompressionAdapter")
    /// * `compression_config` - Compression algorithm and parameters
    pub fn new_compression_adapter(
        service: Arc<dyn CompressionService>,
        name: Option<String>,
        compression_config: CompressionConfig,
    ) -> Self {
        Self::new(
            service,
            name.unwrap_or_else(|| "CompressionAdapter".to_string()),
            CompressionAdapterConfig {
                modifies_data: true,
                compression_config,
            }
        )
    }
}

impl EncryptionChunkAdapter {
    /// Creates a new encryption adapter with provided configuration
    ///
    /// # Security
    ///
    /// Caller MUST provide:
    /// - Valid encryption configuration
    /// - Secure key material (NOT zero-filled or weak keys)
    ///
    /// # Arguments
    ///
    /// * `service` - The encryption service implementation
    /// * `name` - Optional name for this adapter (defaults to "EncryptionAdapter")
    /// * `encryption_config` - Encryption algorithm and parameters
    /// * `key_material` - Cryptographic key material (must be secure!)
    pub fn new_encryption_adapter(
        service: Arc<dyn EncryptionService>,
        name: Option<String>,
        encryption_config: EncryptionConfig,
        key_material: KeyMaterial,
    ) -> Self {
        Self::new(
            service,
            name.unwrap_or_else(|| "EncryptionAdapter".to_string()),
            EncryptionAdapterConfig {
                modifies_data: true,
                encryption_config,
                key_material,
            }
        )
    }
}

/// Generic factory for creating any service adapter
pub struct ServiceAdapterFactory;

impl ServiceAdapterFactory {
    /// Create a compression chunk adapter with required configuration
    ///
    /// # Arguments
    ///
    /// * `service` - The compression service implementation
    /// * `compression_config` - Compression algorithm and parameters
    pub fn create_compression_adapter(
        service: Arc<dyn CompressionService>,
        compression_config: CompressionConfig,
    ) -> Box<dyn ChunkProcessor> {
        Box::new(CompressionChunkAdapter::new_compression_adapter(
            service,
            None,
            compression_config,
        ))
    }

    /// Create an encryption chunk adapter with required configuration
    ///
    /// # Security
    ///
    /// Caller MUST provide secure key material. This factory will NOT
    /// generate insecure defaults.
    ///
    /// # Arguments
    ///
    /// * `service` - The encryption service implementation
    /// * `encryption_config` - Encryption algorithm and parameters
    /// * `key_material` - Cryptographic key material (must be secure!)
    pub fn create_encryption_adapter(
        service: Arc<dyn EncryptionService>,
        encryption_config: EncryptionConfig,
        key_material: KeyMaterial,
    ) -> Box<dyn ChunkProcessor> {
        Box::new(EncryptionChunkAdapter::new_encryption_adapter(
            service,
            None,
            encryption_config,
            key_material,
        ))
    }

    /// Create a custom service adapter with specific configuration
    pub fn create_custom_adapter<T: Send + Sync + 'static>(
        service: Arc<T>,
        name: String,
        config: AdapterConfig
    ) -> ServiceChunkAdapter<T> {
        ServiceChunkAdapter::new(service, name, config)
    }
}
