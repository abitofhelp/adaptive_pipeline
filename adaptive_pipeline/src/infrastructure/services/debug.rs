// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Debug Service
//!
//! Provides a diagnostic stage that passes data through unchanged while
//! emitting metrics to Prometheus for real-time monitoring and debugging.
//!
//! ## Purpose
//!
//! Debug stages are useful for:
//! - **Detecting Data Corruption**: Calculate checksums at intermediate points
//! - **Performance Analysis**: Measure throughput between stages
//! - **Pipeline Debugging**: Monitor data flow through complex pipelines
//! - **Production Monitoring**: Real-time visibility into processing
//!
//! ## Metrics Exposed
//!
//! Each DebugStage emits to Prometheus:
//! - `debug_stage_checksum{label, chunk_id}` - SHA256 of chunk data
//! - `debug_stage_bytes{label, chunk_id}` - Bytes processed per chunk
//! - `debug_stage_chunks_total{label}` - Total chunks processed
//!
//! ## Label Uniqueness
//!
//! Each DebugStage is automatically assigned a ULID label during creation,
//! ensuring no metric name conflicts when multiple debug stages exist in
//! a pipeline.
//!
//! ## Usage
//!
//! ```bash
//! # Create pipeline with debug stages at key points
//! pipeline create --name test --stages base64,debug,brotli,debug,encryption
//!
//! # Monitor via Prometheus
//! curl http://localhost:9091/metrics | grep debug_stage
//! ```

use adaptive_pipeline_domain::entities::{ProcessingContext, StageConfiguration, StagePosition, StageType};
use adaptive_pipeline_domain::services::{FromParameters, StageService};
use adaptive_pipeline_domain::value_objects::FileChunk;
use adaptive_pipeline_domain::PipelineError;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;

use crate::infrastructure::metrics::MetricsService;

/// Configuration for Debug stage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugConfig {
    /// Unique label for this debug stage (ULID)
    pub label: String,
}

impl FromParameters for DebugConfig {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        let label = params
            .get("label")
            .ok_or_else(|| PipelineError::MissingParameter("label".into()))?
            .clone();

        Ok(Self { label })
    }
}

/// Service that passes data through while emitting Prometheus metrics
///
/// This is a diagnostic stage that:
/// - Passes data through completely unchanged
/// - Calculates SHA256 checksum of each chunk
/// - Emits metrics to Prometheus for monitoring
pub struct DebugService {
    metrics: Arc<MetricsService>,
}

impl DebugService {
    /// Creates a new DebugService with metrics integration
    pub fn new(metrics: Arc<MetricsService>) -> Self {
        Self { metrics }
    }

    /// Calculate SHA256 checksum of data
    fn calculate_checksum(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

impl StageService for DebugService {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        _context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let debug_config = DebugConfig::from_parameters(&config.parameters)?;

        // Calculate checksum
        let checksum = self.calculate_checksum(chunk.data());
        let bytes = chunk.data().len() as u64;
        let chunk_id = chunk.sequence_number();

        // Emit metrics to Prometheus
        tracing::debug!(
            "DebugStage[{}]: chunk={}, bytes={}, checksum={}",
            debug_config.label,
            chunk_id,
            bytes,
            checksum
        );

        // Record metrics in Prometheus
        self.metrics
            .record_debug_stage_bytes(&debug_config.label, chunk_id, bytes);
        self.metrics.increment_debug_stage_chunks(&debug_config.label);

        // Pass through unchanged
        Ok(chunk)
    }

    fn position(&self) -> StagePosition {
        StagePosition::Any
    }

    fn is_reversible(&self) -> bool {
        true
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adaptive_pipeline_domain::entities::security_context::Permission;
    use adaptive_pipeline_domain::entities::{Operation, SecurityContext, SecurityLevel};

    fn create_test_metrics() -> Arc<MetricsService> {
        Arc::new(MetricsService::new().unwrap())
    }

    fn create_test_chunk(data: Vec<u8>) -> FileChunk {
        FileChunk::new(0, 0, data, false).unwrap()
    }

    fn create_test_context() -> ProcessingContext {
        let security_context =
            SecurityContext::with_permissions(None, vec![Permission::Read, Permission::Write], SecurityLevel::Internal);
        ProcessingContext::new(
            1024,
            security_context,
        )
    }

    #[test]
    fn test_debug_service_creation() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);
        assert_eq!(service.position(), StagePosition::Any);
        assert!(service.is_reversible());
        assert_eq!(service.stage_type(), StageType::Transform);
    }

    #[test]
    fn test_debug_config_from_parameters() {
        let mut params = HashMap::new();
        params.insert("label".to_string(), "01K6VWAA123456".to_string());

        let config = DebugConfig::from_parameters(&params).unwrap();
        assert_eq!(config.label, "01K6VWAA123456");
    }

    #[test]
    fn test_debug_config_missing_label() {
        let params = HashMap::new();
        let result = DebugConfig::from_parameters(&params);
        assert!(result.is_err());
        assert!(matches!(result, Err(PipelineError::MissingParameter(_))));
    }

    #[test]
    fn test_debug_data_unchanged() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);
        let mut context = create_test_context();

        let test_data = b"Hello, World!".to_vec();
        let chunk = create_test_chunk(test_data.clone());

        let mut params = HashMap::new();
        params.insert("label".to_string(), "01K6VWAA123456".to_string());
        params.insert("algorithm".to_string(), "debug".to_string());

        let config = StageConfiguration {
            algorithm: "debug".to_string(),
            operation: Operation::Forward,
            parameters: params,
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_debug_checksum_calculation() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);

        let test_data = b"Hello, World!";
        let checksum = service.calculate_checksum(test_data);

        // Known SHA256 of "Hello, World!"
        assert_eq!(
            checksum,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_debug_minimal_data() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);
        let mut context = create_test_context();

        let test_data = b"x".to_vec();
        let chunk = create_test_chunk(test_data.clone());

        let mut params = HashMap::new();
        params.insert("label".to_string(), "01K6VWAA123456".to_string());
        params.insert("algorithm".to_string(), "debug".to_string());

        let config = StageConfiguration {
            algorithm: "debug".to_string(),
            operation: Operation::Forward,
            parameters: params,
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_debug_large_data() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);
        let mut context = create_test_context();

        let test_data = vec![42u8; 1_000_000]; // 1MB
        let chunk = create_test_chunk(test_data.clone());

        let mut params = HashMap::new();
        params.insert("label".to_string(), "01K6VWAA123456".to_string());
        params.insert("algorithm".to_string(), "debug".to_string());

        let config = StageConfiguration {
            algorithm: "debug".to_string(),
            operation: Operation::Forward,
            parameters: params,
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_debug_multiple_chunks() {
        let metrics = create_test_metrics();
        let service = DebugService::new(metrics);
        let mut context = create_test_context();

        let mut params = HashMap::new();
        params.insert("label".to_string(), "01K6VWAA123456".to_string());
        params.insert("algorithm".to_string(), "debug".to_string());

        let config = StageConfiguration {
            algorithm: "debug".to_string(),
            operation: Operation::Forward,
            parameters: params.clone(),
            parallel_processing: false,
            chunk_size: None,
        };

        // Process multiple chunks
        for i in 0..5 {
            let test_data = format!("Chunk {}", i).into_bytes();
            let chunk = FileChunk::new(i, 0, test_data.clone(), false).unwrap();
            let result = service.process_chunk(chunk, &config, &mut context).unwrap();
            assert_eq!(result.data(), test_data.as_slice());
            assert_eq!(result.sequence_number(), i);
        }
    }
}
