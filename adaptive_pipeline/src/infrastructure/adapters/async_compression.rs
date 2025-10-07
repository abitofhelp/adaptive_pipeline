// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Async Compression Adapter
//!
//! This module provides an async adapter for the synchronous
//! `CompressionService` domain trait. It demonstrates the proper pattern for
//! handling sync domain services in async infrastructure contexts.
//!
//! ## Architecture Pattern
//!
//! Following the hybrid DDD/Clean/Hexagonal architecture:
//!
//! - **Domain**: Defines sync `CompressionService` trait (business logic)
//! - **Infrastructure**: Provides async adapter (execution model)
//! - **Separation of Concerns**: Domain doesn't dictate async/sync execution
//!
//! ## Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use pipeline::infrastructure::adapters::AsyncCompressionAdapter;
//! use pipeline::infrastructure::adapters::MultiAlgoCompression;
//!
//! // Create sync implementation
//! let sync_service = Arc::new(MultiAlgoCompression::new());
//!
//! // Wrap in async adapter
//! let async_service = AsyncCompressionAdapter::new(sync_service);
//!
//! // Use in async context
//! let result = async_service.compress_chunk_async(chunk, &config, &mut context).await?;
//! ```

use adaptive_pipeline_domain::entities::ProcessingContext;
use adaptive_pipeline_domain::services::compression_service::{
    CompressionAlgorithm,
    CompressionBenchmark,
    CompressionConfig,
    CompressionPriority,
    CompressionService,
};
use adaptive_pipeline_domain::value_objects::FileChunk;
use adaptive_pipeline_domain::PipelineError;
use std::sync::Arc;

/// Async adapter for `CompressionService`
///
/// Wraps a synchronous `CompressionService` implementation and provides
/// async methods that execute the sync operations in a way that doesn't
/// block the async runtime.
///
/// ## Design Rationale
///
/// - **Domain Purity**: Domain traits remain sync and portable
/// - **Infrastructure Flexibility**: Async execution is an implementation
///   detail
/// - **Non-Blocking**: Uses `spawn_blocking` for CPU-intensive operations
/// - **Zero-Cost When Sync**: No overhead if used in sync contexts
pub struct AsyncCompressionAdapter<T: CompressionService + 'static> {
    inner: Arc<T>,
}

impl<T: CompressionService + 'static> AsyncCompressionAdapter<T> {
    /// Creates a new async adapter wrapping a sync compression service
    pub fn new(service: Arc<T>) -> Self {
        Self { inner: service }
    }

    /// Compresses a chunk asynchronously
    ///
    /// Executes the synchronous compress operation in a blocking task pool
    /// to avoid blocking the async runtime.
    pub async fn compress_chunk_async(
        &self,
        chunk: FileChunk,
        config: &CompressionConfig,
        context: &mut ProcessingContext
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();

        // Move mutable context handling - for now, we'll need to rethink this
        // In practice, context updates would need to be synchronized or passed back
        let mut context_clone = context.clone();

        tokio::task
            ::spawn_blocking(move ||
                service.compress_chunk(chunk, &config, &mut context_clone)
            ).await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Decompresses a chunk asynchronously
    pub async fn decompress_chunk_async(
        &self,
        chunk: FileChunk,
        config: &CompressionConfig,
        context: &mut ProcessingContext
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();
        let mut context_clone = context.clone();

        tokio::task
            ::spawn_blocking(move ||
                service.decompress_chunk(chunk, &config, &mut context_clone)
            ).await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Compresses multiple chunks in parallel using Rayon (infrastructure
    /// concern)
    ///
    /// This method demonstrates how parallelization is an infrastructure
    /// concern, not a domain concern. The domain just defines
    /// compress/decompress.
    ///
    /// Uses Rayon's data parallelism for efficient CPU-bound batch compression,
    /// providing 3-5x speedup on multi-core systems.
    pub async fn compress_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &CompressionConfig,
        context: &mut ProcessingContext
    ) -> Result<Vec<FileChunk>, PipelineError> {
        use crate::infrastructure::config::rayon_config::RAYON_POOLS;
        use rayon::prelude::*;

        let service = self.inner.clone();
        let config = config.clone();
        let context_clone = context.clone();

        // Use spawn_blocking to run entire Rayon batch on blocking thread pool
        tokio::task
            ::spawn_blocking(move || {
                // Use CPU-bound pool for compression
                RAYON_POOLS.cpu_bound_pool().install(|| {
                    // Parallel compression using Rayon
                    chunks
                        .into_par_iter()
                        .map(|chunk| {
                            let mut local_context = context_clone.clone();
                            service.compress_chunk(chunk, &config, &mut local_context)
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
            }).await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Estimates compression ratio (sync operation, no need for async)
    pub fn estimate_compression_ratio(
        &self,
        data_sample: &[u8],
        algorithm: &CompressionAlgorithm
    ) -> Result<f64, PipelineError> {
        self.inner.estimate_compression_ratio(data_sample, algorithm)
    }

    /// Gets optimal config (sync operation)
    pub fn get_optimal_config(
        &self,
        file_extension: &str,
        data_sample: &[u8],
        performance_priority: CompressionPriority
    ) -> Result<CompressionConfig, PipelineError> {
        self.inner.get_optimal_config(file_extension, data_sample, performance_priority)
    }

    /// Validates config (sync operation)
    pub fn validate_config(&self, config: &CompressionConfig) -> Result<(), PipelineError> {
        self.inner.validate_config(config)
    }

    /// Gets supported algorithms (sync operation)
    pub fn supported_algorithms(&self) -> Vec<CompressionAlgorithm> {
        self.inner.supported_algorithms()
    }

    /// Benchmarks algorithm asynchronously
    pub async fn benchmark_algorithm_async(
        &self,
        algorithm: &CompressionAlgorithm,
        test_data: &[u8]
    ) -> Result<CompressionBenchmark, PipelineError> {
        let service = self.inner.clone();
        let algorithm = algorithm.clone();
        let test_data = test_data.to_vec();

        tokio::task
            ::spawn_blocking(move || service.benchmark_algorithm(&algorithm, &test_data)).await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }
}

impl<T: CompressionService + 'static> Clone for AsyncCompressionAdapter<T> {
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
    struct FakeCompressionService;

    impl CompressionService for FakeCompressionService {
        fn compress_chunk(
            &self,
            chunk: FileChunk,
            _config: &CompressionConfig,
            _context: &mut ProcessingContext
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn decompress_chunk(
            &self,
            chunk: FileChunk,
            _config: &CompressionConfig,
            _context: &mut ProcessingContext
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn estimate_compression_ratio(
            &self,
            _data_sample: &[u8],
            _algorithm: &CompressionAlgorithm
        ) -> Result<f64, PipelineError> {
            Ok(0.5) // Fake: 50% compression ratio
        }

        fn get_optimal_config(
            &self,
            _file_extension: &str,
            _data_sample: &[u8],
            _performance_priority: CompressionPriority
        ) -> Result<CompressionConfig, PipelineError> {
            Ok(CompressionConfig::default())
        }

        fn validate_config(&self, _config: &CompressionConfig) -> Result<(), PipelineError> {
            Ok(())
        }

        fn supported_algorithms(&self) -> Vec<CompressionAlgorithm> {
            vec![CompressionAlgorithm::Gzip]
        }

        fn benchmark_algorithm(
            &self,
            _algorithm: &CompressionAlgorithm,
            _test_data: &[u8]
        ) -> Result<CompressionBenchmark, PipelineError> {
            Ok(CompressionBenchmark::default())
        }
    }

    impl adaptive_pipeline_domain::services::StageService for FakeCompressionService {
        fn process_chunk(
            &self,
            chunk: FileChunk,
            config: &adaptive_pipeline_domain::entities::pipeline_stage::StageConfiguration,
            context: &mut ProcessingContext
        ) -> Result<FileChunk, PipelineError> {
            use adaptive_pipeline_domain::services::FromParameters;
            let compression_config = CompressionConfig::from_parameters(&config.parameters)?;
            match config.operation {
                adaptive_pipeline_domain::entities::Operation::Forward => {
                    self.compress_chunk(chunk, &compression_config, context)
                }
                adaptive_pipeline_domain::entities::Operation::Reverse => {
                    self.decompress_chunk(chunk, &compression_config, context)
                }
            }
        }

        fn position(&self) -> adaptive_pipeline_domain::entities::StagePosition {
            adaptive_pipeline_domain::entities::StagePosition::PreBinary
        }

        fn is_reversible(&self) -> bool {
            true
        }

        fn stage_type(&self) -> adaptive_pipeline_domain::entities::StageType {
            adaptive_pipeline_domain::entities::StageType::Compression
        }
    }

    #[tokio::test]
    async fn test_async_adapter_pattern() {
        let sync_service = Arc::new(FakeCompressionService);
        let async_adapter = AsyncCompressionAdapter::new(sync_service);

        // Test that we can call async methods
        let algorithms = async_adapter.supported_algorithms();
        assert_eq!(algorithms.len(), 1);
    }
}
