// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Async Checksum Adapter
//!
//! This module provides an async adapter for the synchronous `ChecksumService`
//! domain trait. It demonstrates the proper pattern for handling sync domain
//! services in async infrastructure contexts.
//!
//! ## Architecture Pattern
//!
//! Following the hybrid DDD/Clean/Hexagonal architecture:
//!
//! - **Domain**: Defines sync `ChecksumService` trait (business logic)
//! - **Infrastructure**: Provides async adapter (execution model)
//! - **Separation of Concerns**: Domain doesn't dictate async/sync execution
//!
//! ## Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use adaptive_pipeline::infrastructure::adapters::AsyncChecksumAdapter;
//! use adaptive_pipeline_domain::services::checksum_service::ChecksumProcessor;
//!
//! // Create sync implementation
//! let sync_service = Arc::new(ChecksumProcessor::sha256_processor(true));
//!
//! // Wrap in async adapter
//! let async_service = AsyncChecksumAdapter::new(sync_service);
//!
//! // Use in async context
//! let result = async_service.process_chunk_async(chunk, &mut context, "stage").await?;
//! ```

use adaptive_pipeline_domain::entities::ProcessingContext;
use adaptive_pipeline_domain::services::checksum_service::ChecksumService;
use adaptive_pipeline_domain::value_objects::FileChunk;
use adaptive_pipeline_domain::PipelineError;
use std::sync::Arc;

/// Async adapter for `ChecksumService`
///
/// Wraps a synchronous `ChecksumService` implementation and provides
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
pub struct AsyncChecksumAdapter<T: ChecksumService + 'static> {
    inner: Arc<T>,
}

impl<T: ChecksumService + 'static> AsyncChecksumAdapter<T> {
    /// Creates a new async adapter wrapping a sync checksum service
    pub fn new(service: Arc<T>) -> Self {
        Self { inner: service }
    }

    /// Processes a chunk asynchronously, updating the running checksum
    ///
    /// Executes the synchronous process operation in a blocking task pool
    /// to avoid blocking the async runtime.
    pub async fn process_chunk_async(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
        stage_name: &str,
    ) -> Result<FileChunk, PipelineError> {
        let service = self.inner.clone();
        let mut context_clone = context.clone();
        let stage_name = stage_name.to_string();

        tokio::task::spawn_blocking(move || service.process_chunk(chunk, &mut context_clone, &stage_name))
            .await
            .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))?
    }

    /// Processes multiple chunks in parallel (infrastructure concern)
    ///
    /// This method demonstrates how parallelization is an infrastructure
    /// concern, not a domain concern. The domain just defines process_chunk.
    pub async fn process_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
        stage_name: &str,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        let mut tasks = Vec::new();

        for chunk in chunks {
            let service = self.inner.clone();
            let mut context_clone = context.clone();
            let stage_name = stage_name.to_string();

            let task =
                tokio::task::spawn_blocking(move || service.process_chunk(chunk, &mut context_clone, &stage_name));

            tasks.push(task);
        }

        let mut results = Vec::new();
        for task in tasks {
            let result = task
                .await
                .map_err(|e| PipelineError::InternalError(format!("Task join error: {}", e)))??;
            results.push(result);
        }

        Ok(results)
    }

    /// Gets the final checksum value (sync operation)
    pub fn get_checksum(&self, context: &ProcessingContext, stage_name: &str) -> Option<String> {
        self.inner.get_checksum(context, stage_name)
    }
}

impl<T: ChecksumService + 'static> Clone for AsyncChecksumAdapter<T> {
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
    struct FakeChecksumService;

    impl ChecksumService for FakeChecksumService {
        fn process_chunk(
            &self,
            chunk: FileChunk,
            _context: &mut ProcessingContext,
            _stage_name: &str,
        ) -> Result<FileChunk, PipelineError> {
            Ok(chunk) // Fake: just return the same chunk
        }

        fn get_checksum(&self, _context: &ProcessingContext, _stage_name: &str) -> Option<String> {
            Some("fake_checksum".to_string())
        }
    }

    #[tokio::test]
    async fn test_async_adapter_pattern() {
        use adaptive_pipeline_domain::entities::{ProcessingContext, SecurityContext, SecurityLevel};
        use std::path::PathBuf;

        let sync_service = Arc::new(FakeChecksumService);
        let async_adapter = AsyncChecksumAdapter::new(sync_service);

        // Test that we can call sync methods
        let security_context = SecurityContext::new(Some("test".to_string()), SecurityLevel::Internal);
        let context = ProcessingContext::new(
            PathBuf::from("/tmp/test"),
            PathBuf::from("/tmp/test_out"),
            1024,
            security_context,
        );
        let checksum = async_adapter.get_checksum(&context, "test_stage");
        assert_eq!(checksum, Some("fake_checksum".to_string()));
    }
}
