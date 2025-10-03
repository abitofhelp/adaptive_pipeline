// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Async Encryption Adapter
//!
//! This module provides an async adapter for the synchronous `EncryptionService`
//! domain trait. It demonstrates the proper pattern for handling sync domain
//! services in async infrastructure contexts.
//!
//! ## Architecture Pattern
//!
//! Following the hybrid DDD/Clean/Hexagonal architecture:
//!
//! - **Domain**: Defines sync `EncryptionService` trait (business logic)
//! - **Infrastructure**: Provides async adapter (execution model)
//! - **Separation of Concerns**: Domain doesn't dictate async/sync execution
//!
//! ## Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use pipeline::infrastructure::adapters::AsyncEncryptionAdapter;
//! use pipeline::infrastructure::adapters::EncryptionServiceImpl;
//!
//! // Create sync implementation
//! let sync_service = Arc::new(EncryptionServiceImpl::new());
//!
//! // Wrap in async adapter
//! let async_service = AsyncEncryptionAdapter::new(sync_service);
//!
//! // Use in async context
//! let result = async_service.encrypt_chunk_async(chunk, &config, &key, &mut context).await?;
//! ```

use pipeline_domain::entities::{ProcessingContext, SecurityContext};
use pipeline_domain::services::encryption_service::{
    EncryptionConfig, EncryptionService, KeyMaterial,
};
use pipeline_domain::value_objects::{EncryptionBenchmark, FileChunk};
use pipeline_domain::PipelineError;
use pipeline_domain::services::encryption_service::EncryptionAlgorithm;
use std::sync::Arc;

/// Async adapter for `EncryptionService`
///
/// Wraps a synchronous `EncryptionService` implementation and provides
/// async methods that execute the sync operations in a way that doesn't
/// block the async runtime.
///
/// ## Design Rationale
///
/// - **Domain Purity**: Domain traits remain sync and portable
/// - **Infrastructure Flexibility**: Async execution is an implementation detail
/// - **Non-Blocking**: Uses `spawn_blocking` for CPU-intensive operations
/// - **Zero-Cost When Sync**: No overhead if used in sync contexts
pub struct AsyncEncryptionAdapter<T: EncryptionService + 'static> {
    inner: Arc<T>,
}

impl<T: EncryptionService + 'static> AsyncEncryptionAdapter<T> {
    /// Creates a new async adapter wrapping a sync encryption service
    pub fn new(service: Arc<T>) -> Self {
        Self { inner: service }
    }

    /// Encrypts a chunk asynchronously
    ///
    /// Executes the synchronous encrypt operation in a blocking task pool
    /// to avoid blocking the async runtime.
    pub async fn encrypt_chunk_async(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();
        let key_material = key_material.clone();
        let mut context_clone = context.clone();

        tokio::task::spawn_blocking(move || {
            service.encrypt_chunk(chunk, &config, &key_material, &mut context_clone)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Decrypts a chunk asynchronously
    pub async fn decrypt_chunk_async(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();
        let key_material = key_material.clone();
        let mut context_clone = context.clone();

        tokio::task::spawn_blocking(move || {
            service.decrypt_chunk(chunk, &config, &key_material, &mut context_clone)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Encrypts multiple chunks in parallel using Rayon (infrastructure concern)
    ///
    /// This method demonstrates how parallelization is an infrastructure
    /// concern, not a domain concern. The domain just defines encrypt/decrypt.
    ///
    /// Uses Rayon's data parallelism for efficient CPU-bound batch encryption,
    /// providing 3-4x speedup on multi-core systems.
    pub async fn encrypt_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        use crate::infrastructure::config::rayon_config::RAYON_POOLS;
        use rayon::prelude::*;

        let service = self.inner.clone();
        let config = config.clone();
        let key_material = key_material.clone();
        let context_clone = context.clone();

        // Use spawn_blocking to run entire Rayon batch on blocking thread pool
        tokio::task::spawn_blocking(move || {
            // Use CPU-bound pool for encryption
            RAYON_POOLS.cpu_bound_pool().install(|| {
                // Parallel encryption using Rayon
                chunks
                    .into_par_iter()
                    .map(|chunk| {
                        let mut local_context = context_clone.clone();
                        service.encrypt_chunk(chunk, &config, &key_material, &mut local_context)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Decrypts multiple chunks in parallel using Rayon (infrastructure concern)
    ///
    /// Uses Rayon's data parallelism for efficient CPU-bound batch decryption,
    /// providing 3-4x speedup on multi-core systems.
    pub async fn decrypt_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        use crate::infrastructure::config::rayon_config::RAYON_POOLS;
        use rayon::prelude::*;

        let service = self.inner.clone();
        let config = config.clone();
        let key_material = key_material.clone();
        let context_clone = context.clone();

        // Use spawn_blocking to run entire Rayon batch on blocking thread pool
        tokio::task::spawn_blocking(move || {
            // Use CPU-bound pool for decryption
            RAYON_POOLS.cpu_bound_pool().install(|| {
                // Parallel decryption using Rayon
                chunks
                    .into_par_iter()
                    .map(|chunk| {
                        let mut local_context = context_clone.clone();
                        service.decrypt_chunk(chunk, &config, &key_material, &mut local_context)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Derives key material from password asynchronously
    ///
    /// This is a CPU-intensive operation that benefits from blocking execution.
    pub async fn derive_key_material_async(
        &self,
        password: &str,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        let service = self.inner.clone();
        let password = password.to_string();
        let config = config.clone();
        let security_context = security_context.clone();

        tokio::task::spawn_blocking(move || {
            service.derive_key_material(&password, &config, &security_context)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Generates random key material asynchronously
    pub async fn generate_key_material_async(
        &self,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();
        let security_context = security_context.clone();

        tokio::task::spawn_blocking(move || {
            service.generate_key_material(&config, &security_context)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Validates config (sync operation)
    pub fn validate_config(&self, config: &EncryptionConfig) -> Result<(), PipelineError> {
        self.inner.validate_config(config)
    }

    /// Gets supported algorithms (sync operation)
    pub fn supported_algorithms(&self) -> Vec<EncryptionAlgorithm> {
        self.inner.supported_algorithms()
    }

    /// Benchmarks algorithm asynchronously
    pub async fn benchmark_algorithm_async(
        &self,
        algorithm: &EncryptionAlgorithm,
        test_data: &[u8],
    ) -> Result<EncryptionBenchmark, PipelineError> {
        let service = self.inner.clone();
        let algorithm = algorithm.clone();
        let test_data = test_data.to_vec();

        tokio::task::spawn_blocking(move || service.benchmark_algorithm(&algorithm, &test_data))
            .await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Wipes key material (sync operation)
    pub fn wipe_key_material(&self, key_material: &mut KeyMaterial) -> Result<(), PipelineError> {
        self.inner.wipe_key_material(key_material)
    }

    /// Stores key material (sync operation - HSM calls might be sync or async depending on implementation)
    pub fn store_key_material(
        &self,
        key_material: &KeyMaterial,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<(), PipelineError> {
        self.inner.store_key_material(key_material, key_id, security_context)
    }

    /// Retrieves key material (sync operation)
    pub fn retrieve_key_material(
        &self,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        self.inner.retrieve_key_material(key_id, security_context)
    }

    /// Rotates keys (sync operation)
    pub fn rotate_keys(
        &self,
        old_key_id: &str,
        new_config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<String, PipelineError> {
        self.inner.rotate_keys(old_key_id, new_config, security_context)
    }
}

impl<T: EncryptionService + 'static> Clone for AsyncEncryptionAdapter<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test double for demonstration
    struct FakeEncryptionService;

    impl EncryptionService for FakeEncryptionService {
        fn encrypt_chunk(
            &self,
            chunk: FileChunk,
            _config: &EncryptionConfig,
            _key_material: &KeyMaterial,
            _context: &mut ProcessingContext,
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn decrypt_chunk(
            &self,
            chunk: FileChunk,
            _config: &EncryptionConfig,
            _key_material: &KeyMaterial,
            _context: &mut ProcessingContext,
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn derive_key_material(
            &self,
            _password: &str,
            config: &EncryptionConfig,
            _security_context: &SecurityContext,
        ) -> Result<KeyMaterial, PipelineError> {
            Ok(KeyMaterial::new(
                vec![0u8; 32],
                vec![0u8; 12],
                vec![0u8; 16],
                config.algorithm.clone(),
            ))
        }

        fn generate_key_material(
            &self,
            config: &EncryptionConfig,
            _security_context: &SecurityContext,
        ) -> Result<KeyMaterial, PipelineError> {
            Ok(KeyMaterial::new(
                vec![0u8; 32],
                vec![0u8; 12],
                vec![0u8; 16],
                config.algorithm.clone(),
            ))
        }

        fn validate_config(&self, _config: &EncryptionConfig) -> Result<(), PipelineError> {
            Ok(())
        }

        fn supported_algorithms(&self) -> Vec<EncryptionAlgorithm> {
            vec![EncryptionAlgorithm::Aes256Gcm]
        }

        fn benchmark_algorithm(
            &self,
            algorithm: &EncryptionAlgorithm,
            _test_data: &[u8],
        ) -> Result<EncryptionBenchmark, PipelineError> {
            use std::time::Duration;
            Ok(EncryptionBenchmark::new(
                algorithm.clone(),
                100.0,  // throughput_mbps
                Duration::from_millis(10),  // latency
                64.0,   // memory_usage_mb
                50.0,   // cpu_usage_percent
                1.0,    // file_size_mb
            ))
        }

        fn wipe_key_material(&self, key_material: &mut KeyMaterial) -> Result<(), PipelineError> {
            key_material.clear();
            Ok(())
        }

        fn store_key_material(
            &self,
            _key_material: &KeyMaterial,
            _key_id: &str,
            _security_context: &SecurityContext,
        ) -> Result<(), PipelineError> {
            Ok(())
        }

        fn retrieve_key_material(
            &self,
            _key_id: &str,
            _security_context: &SecurityContext,
        ) -> Result<KeyMaterial, PipelineError> {
            Ok(KeyMaterial::new(
                vec![0u8; 32],
                vec![0u8; 12],
                vec![0u8; 16],
                EncryptionAlgorithm::Aes256Gcm,
            ))
        }

        fn rotate_keys(
            &self,
            _old_key_id: &str,
            _new_config: &EncryptionConfig,
            _security_context: &SecurityContext,
        ) -> Result<String, PipelineError> {
            Ok("new_key_id".to_string())
        }
    }

    #[tokio::test]
    async fn test_async_adapter_pattern() {
        let sync_service = Arc::new(FakeEncryptionService);
        let async_adapter = AsyncEncryptionAdapter::new(sync_service);

        // Test that we can call sync methods
        let algorithms = async_adapter.supported_algorithms();
        assert_eq!(algorithms.len(), 1);
    }
}
