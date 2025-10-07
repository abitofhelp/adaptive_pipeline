// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # PassThrough Service
//!
//! Provides a no-op stage that passes data through unchanged.
//!
//! ## Purpose
//!
//! PassThrough stages are useful for:
//! - Pipeline structure placeholders
//! - Testing and debugging pipelines
//! - Future extension points
//! - Restoration pipeline markers
//!
//! ## Usage
//!
//! ```rust
//! use adaptive_pipeline::infrastructure::services::PassThroughService;
//! use adaptive_pipeline_domain::services::StageService;
//!
//! let service = PassThroughService::new();
//! // Data passes through completely unchanged
//! ```

use adaptive_pipeline_domain::entities::{ProcessingContext, StageConfiguration, StagePosition, StageType};
use adaptive_pipeline_domain::services::{FromParameters, StageService};
use adaptive_pipeline_domain::value_objects::FileChunk;
use adaptive_pipeline_domain::PipelineError;
use std::collections::HashMap;

/// Configuration for PassThrough stage (empty - no parameters needed)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassThroughConfig;

impl FromParameters for PassThroughConfig {
    fn from_parameters(_params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        // PassThrough has no configuration parameters
        Ok(Self)
    }
}

/// Service that passes data through without modification
///
/// This is a no-op stage useful for pipeline structure, testing, and
/// placeholders.
pub struct PassThroughService;

impl PassThroughService {
    /// Creates a new PassThroughService
    pub fn new() -> Self {
        Self
    }
}

impl Default for PassThroughService {
    fn default() -> Self {
        Self::new()
    }
}

impl StageService for PassThroughService {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        _config: &StageConfiguration,
        _context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
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
        StageType::PassThrough
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adaptive_pipeline_domain::entities::security_context::Permission;
    use adaptive_pipeline_domain::entities::{Operation, SecurityContext, SecurityLevel};

    fn create_test_chunk(data: Vec<u8>) -> FileChunk {
        FileChunk::new(0, 0, data, false).unwrap()
    }

    fn create_test_context() -> ProcessingContext {
        let security_context =
            SecurityContext::with_permissions(None, vec![Permission::Read, Permission::Write], SecurityLevel::Internal);
        ProcessingContext::new(
            std::path::PathBuf::from("/tmp/input.txt"),
            std::path::PathBuf::from("/tmp/output.adapipe"),
            1024,
            security_context,
        )
    }

    #[test]
    fn test_passthrough_service_creation() {
        let service = PassThroughService::new();
        assert_eq!(service.position(), StagePosition::Any);
        assert_eq!(service.is_reversible(), true);
        assert_eq!(service.stage_type(), StageType::PassThrough);
    }

    #[test]
    fn test_passthrough_config_from_empty_parameters() {
        let params = HashMap::new();
        let config = PassThroughConfig::from_parameters(&params).unwrap();
        assert_eq!(config, PassThroughConfig);
    }

    #[test]
    fn test_passthrough_config_ignores_parameters() {
        let mut params = HashMap::new();
        params.insert("ignored".to_string(), "value".to_string());
        let config = PassThroughConfig::from_parameters(&params).unwrap();
        assert_eq!(config, PassThroughConfig);
    }

    #[test]
    fn test_passthrough_data_unchanged() {
        let service = PassThroughService::new();
        let mut context = create_test_context();

        let test_data = b"Hello, World!".to_vec();
        let chunk = create_test_chunk(test_data.clone());

        let config = StageConfiguration {
            algorithm: "passthrough".to_string(),
            operation: Operation::Forward,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_passthrough_minimal_data() {
        let service = PassThroughService::new();
        let mut context = create_test_context();

        let test_data = b"x".to_vec();
        let chunk = create_test_chunk(test_data.clone());

        let config = StageConfiguration {
            algorithm: "passthrough".to_string(),
            operation: Operation::Forward,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_passthrough_large_data() {
        let service = PassThroughService::new();
        let mut context = create_test_context();

        let test_data = vec![42u8; 1_000_000]; // 1MB of data
        let chunk = create_test_chunk(test_data.clone());

        let config = StageConfiguration {
            algorithm: "passthrough".to_string(),
            operation: Operation::Forward,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(result.data(), test_data.as_slice());
    }

    #[test]
    fn test_passthrough_forward_and_reverse_identical() {
        let service = PassThroughService::new();
        let mut context = create_test_context();

        let test_data = b"Test data".to_vec();
        let chunk = create_test_chunk(test_data.clone());

        // Forward operation
        let config_forward = StageConfiguration {
            algorithm: "passthrough".to_string(),
            operation: Operation::Forward,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        let result_forward = service
            .process_chunk(chunk.clone(), &config_forward, &mut context)
            .unwrap();

        // Reverse operation
        let config_reverse = StageConfiguration {
            algorithm: "passthrough".to_string(),
            operation: Operation::Reverse,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        let result_reverse = service.process_chunk(chunk, &config_reverse, &mut context).unwrap();

        // Both should be identical to original
        assert_eq!(result_forward.data(), test_data.as_slice());
        assert_eq!(result_reverse.data(), test_data.as_slice());
        assert_eq!(result_forward.data(), result_reverse.data());
    }
}
