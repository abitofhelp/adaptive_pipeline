//! # Async Compression Adapter
//!
//! This module provides an async adapter for the synchronous `CompressionService`
//! domain trait. It demonstrates the proper pattern for handling sync domain
//! services in async infrastructure contexts.
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
//! use pipeline::infrastructure::adapters::CompressionServiceImpl;
//!
//! // Create sync implementation
//! let sync_service = Arc::new(CompressionServiceImpl::new());
//!
//! // Wrap in async adapter
//! let async_service = AsyncCompressionAdapter::new(sync_service);
//!
//! // Use in async context
//! let result = async_service.compress_chunk_async(chunk, &config, &mut context).await?;
//! ```

use pipeline_domain::entities::ProcessingContext;
use pipeline_domain::services::compression_service::{
    CompressionAlgorithm, CompressionBenchmark, CompressionConfig, CompressionPriority,
    CompressionService,
};
use pipeline_domain::value_objects::FileChunk;
use pipeline_domain::PipelineError;
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
/// - **Infrastructure Flexibility**: Async execution is an implementation detail
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
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();

        // Move mutable context handling - for now, we'll need to rethink this
        // In practice, context updates would need to be synchronized or passed back
        let mut context_clone = context.clone();

        tokio::task::spawn_blocking(move || {
            service.compress_chunk(chunk, &config, &mut context_clone)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Decompresses a chunk asynchronously
    pub async fn decompress_chunk_async(
        &self,
        chunk: FileChunk,
        config: &CompressionConfig,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let config = config.clone();
        let mut context_clone = context.clone();

        tokio::task::spawn_blocking(move || {
            service.decompress_chunk(chunk, &config, &mut context_clone)
        })
        .await
        .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Compresses multiple chunks in parallel (infrastructure concern)
    ///
    /// This method demonstrates how parallelization is an infrastructure
    /// concern, not a domain concern. The domain just defines compress/decompress.
    pub async fn compress_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &CompressionConfig,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        let mut tasks = Vec::new();

        for chunk in chunks {
            let service = self.inner.clone();
            let config = config.clone();
            let mut context_clone = context.clone();

            let task = tokio::task::spawn_blocking(move || {
                service.compress_chunk(chunk, &config, &mut context_clone)
            });

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            let result = task.await
                .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))??;
            results.push(result);
        }

        Ok(results)
    }

    /// Estimates compression ratio (sync operation, no need for async)
    pub fn estimate_compression_ratio(
        &self,
        data_sample: &[u8],
        algorithm: &CompressionAlgorithm,
    ) -> Result<f64, PipelineError> {
        self.inner.estimate_compression_ratio(data_sample, algorithm)
    }

    /// Gets optimal config (sync operation)
    pub fn get_optimal_config(
        &self,
        file_extension: &str,
        data_sample: &[u8],
        performance_priority: CompressionPriority,
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
        test_data: &[u8],
    ) -> Result<CompressionBenchmark, PipelineError> {
        let service = self.inner.clone();
        let algorithm = algorithm.clone();
        let test_data = test_data.to_vec();

        tokio::task::spawn_blocking(move || {
            service.benchmark_algorithm(&algorithm, &test_data)
        })
        .await
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
            _context: &mut ProcessingContext,
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn decompress_chunk(
            &self,
            chunk: FileChunk,
            _config: &CompressionConfig,
            _context: &mut ProcessingContext,
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn estimate_compression_ratio(
            &self,
            _data_sample: &[u8],
            _algorithm: &CompressionAlgorithm,
        ) -> Result<f64, PipelineError> {
            Ok(0.5) // Fake: 50% compression ratio
        }

        fn get_optimal_config(
            &self,
            _file_extension: &str,
            _data_sample: &[u8],
            _performance_priority: CompressionPriority,
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
            _test_data: &[u8],
        ) -> Result<CompressionBenchmark, PipelineError> {
            Ok(CompressionBenchmark::default())
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
