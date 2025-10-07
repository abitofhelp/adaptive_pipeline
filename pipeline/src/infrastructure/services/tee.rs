// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Tee Service
//!
//! Production-ready tee (data splitting) stage for the adaptive pipeline.
//! This service provides data inspection and debugging capabilities, useful for:
//!
//! - **Debugging**: Inspecting data at specific pipeline stages
//! - **Monitoring**: Capturing samples for analysis
//! - **Validation**: Verifying transformations at intermediate stages
//! - **Audit Trails**: Recording data flow for compliance
//!
//! ## Architecture
//!
//! This implementation demonstrates the complete pattern for creating pipeline stages:
//!
//! - **Config Struct**: `TeeConfig` with typed parameters
//! - **FromParameters**: Type-safe extraction from HashMap
//! - **Service Struct**: `TeeService` implements `StageService`
//! - **Position**: `Any` (can be placed anywhere in the pipeline)
//! - **Reversibility**: Pass-through (Forward and Reverse both write + pass data)
//!
//! ## Usage
//!
//! ```rust
//! use pipeline_domain::services::StageService;
//! use pipeline::infrastructure::services::TeeService;
//!
//! let service = TeeService::new();
//! // Used automatically by pipeline when configured with StageType::Transform
//! ```
//!
//! ## Configuration Parameters
//!
//! - **output_path** (required): Path to write teed data
//!   - Example: `"/tmp/debug-output.bin"`
//!   - Data will be written to this file (appended if exists)
//!
//! - **format** (optional): Output format for teed data
//!   - `"binary"` - Raw binary output (default)
//!   - `"hex"` - Hexadecimal dump format
//!   - `"text"` - UTF-8 text (lossy conversion for non-UTF8)
//!   - Default: "binary"
//!
//! - **enabled** (optional): Whether tee is active
//!   - `"true"` - Tee is active (default)
//!   - `"false"` - Tee is disabled (pass-through only)
//!   - Allows conditional enable/disable without removing stage
//!
//! ## Performance Characteristics
//!
//! - **Throughput**: Limited by I/O speed to tee output
//! - **Overhead**: File write operation + optional format conversion
//! - **Memory**: Minimal, no buffering
//! - **Latency**: Synchronous I/O (blocking writes)

use pipeline_domain::entities::{
    Operation, ProcessingContext, StageConfiguration, StagePosition, StageType,
};
use pipeline_domain::services::{FromParameters, StageService};
use pipeline_domain::value_objects::file_chunk::FileChunk;
use pipeline_domain::PipelineError;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Output format for teed data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeeFormat {
    /// Raw binary output
    Binary,
    /// Hexadecimal dump format
    Hex,
    /// UTF-8 text (lossy conversion)
    Text,
}

/// Configuration for Tee operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeeConfig {
    /// Path to write teed data
    pub output_path: PathBuf,
    /// Output format
    pub format: TeeFormat,
    /// Whether tee is enabled
    pub enabled: bool,
}

impl Default for TeeConfig {
    fn default() -> Self {
        Self {
            output_path: PathBuf::from("/tmp/tee-output.bin"),
            format: TeeFormat::Binary,
            enabled: true,
        }
    }
}

/// Implementation of `FromParameters` for TeeConfig.
///
/// Extracts typed configuration from HashMap parameters following
/// the pattern established by other service configs.
impl FromParameters for TeeConfig {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        // Required: output_path
        let output_path = params
            .get("output_path")
            .ok_or_else(|| {
                PipelineError::MissingParameter("output_path is required for tee stage".into())
            })?
            .into();

        // Optional: format (defaults to binary)
        let format = params
            .get("format")
            .map(|s| match s.to_lowercase().as_str() {
                "binary" => Ok(TeeFormat::Binary),
                "hex" => Ok(TeeFormat::Hex),
                "text" => Ok(TeeFormat::Text),
                other => Err(PipelineError::InvalidParameter(format!(
                    "Unknown tee format: {}. Valid: binary, hex, text",
                    other
                ))),
            })
            .transpose()?
            .unwrap_or(TeeFormat::Binary);

        // Optional: enabled (defaults to true)
        let enabled = params
            .get("enabled")
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(true);

        Ok(Self {
            output_path,
            format,
            enabled,
        })
    }
}

/// Production Tee service.
///
/// This service demonstrates the complete pattern for implementing pipeline stages:
/// - Stateless processing (no internal state)
/// - Thread-safe (`Send + Sync`)
/// - Pass-through operation (data flows unchanged)
/// - Type-safe configuration via `FromParameters`
/// - Proper error handling with `PipelineError`
///
/// ## Implementation Notes
///
/// - **Position**: `Any` - Can be placed anywhere in the pipeline
/// - **Reversibility**: `true` - Pass-through in both directions
/// - **Stage Type**: `Transform` - Data inspection operation
/// - **Performance**: Synchronous I/O (blocking writes)
pub struct TeeService;

impl TeeService {
    /// Creates a new Tee service.
    pub fn new() -> Self {
        Self
    }

    /// Writes data to the tee output in the specified format.
    fn write_tee(
        &self,
        data: &[u8],
        config: &TeeConfig,
        chunk_seq: u64,
    ) -> Result<(), PipelineError> {
        if !config.enabled {
            return Ok(());
        }

        // Format the data according to config
        let formatted = match config.format {
            TeeFormat::Binary => data.to_vec(),
            TeeFormat::Hex => {
                // Hex dump with ASCII sidebar (like hexdump -C)
                let mut output = Vec::new();
                for (i, chunk) in data.chunks(16).enumerate() {
                    let offset = i * 16;
                    let hex = chunk
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let ascii = chunk
                        .iter()
                        .map(|&b| if b.is_ascii_graphic() || b == b' ' {
                            b as char
                        } else {
                            '.'
                        })
                        .collect::<String>();
                    let line = format!("{:08x}  {:<48}  |{}|\n", offset, hex, ascii);
                    output.extend_from_slice(line.as_bytes());
                }
                output
            }
            TeeFormat::Text => String::from_utf8_lossy(data).into_owned().into_bytes(),
        };

        // Write to file (append mode)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.output_path)
            .map_err(|e| {
                PipelineError::ProcessingFailed(format!(
                    "Failed to open tee output file '{}': {}",
                    config.output_path.display(),
                    e
                ))
            })?;

        // Write chunk separator for clarity
        writeln!(
            file,
            "--- Chunk {} ({} bytes) ---",
            chunk_seq,
            data.len()
        )
        .map_err(|e| {
            PipelineError::ProcessingFailed(format!("Failed to write tee separator: {}", e))
        })?;

        file.write_all(&formatted).map_err(|e| {
            PipelineError::ProcessingFailed(format!("Failed to write tee data: {}", e))
        })?;

        writeln!(file).map_err(|e| {
            PipelineError::ProcessingFailed(format!("Failed to write tee newline: {}", e))
        })?;

        Ok(())
    }
}

impl Default for TeeService {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of `StageService` for Tee.
///
/// This demonstrates the complete pattern that all stages follow:
/// 1. Extract typed config via `FromParameters`
/// 2. Write data to tee output
/// 3. Pass data through unchanged
/// 4. Return original chunk
impl StageService for TeeService {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        _context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // Type-safe config extraction using FromParameters trait
        let tee_config = TeeConfig::from_parameters(&config.parameters)?;

        // Write data to tee output (both Forward and Reverse operations)
        tracing::debug!(
            chunk_seq = chunk.sequence_number(),
            output_path = %tee_config.output_path.display(),
            format = ?tee_config.format,
            enabled = tee_config.enabled,
            operation = %config.operation,
            "Tee operation"
        );

        self.write_tee(chunk.data(), &tee_config, chunk.sequence_number())?;

        // Pass data through unchanged
        Ok(chunk)
    }

    fn position(&self) -> StagePosition {
        // Any: Can be placed anywhere in the pipeline
        // Reason: Diagnostic stage that doesn't affect data flow
        StagePosition::Any
    }

    fn is_reversible(&self) -> bool {
        // Pass-through: Works in both directions
        true
    }

    fn stage_type(&self) -> StageType {
        // Transform: Data inspection operation
        StageType::Transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pipeline_domain::entities::{SecurityContext, SecurityLevel};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_from_parameters_minimal() {
        let mut params = HashMap::new();
        params.insert("output_path".to_string(), "/tmp/test.bin".to_string());
        let config = TeeConfig::from_parameters(&params).unwrap();
        assert_eq!(config.output_path, PathBuf::from("/tmp/test.bin"));
        assert_eq!(config.format, TeeFormat::Binary);
        assert!(config.enabled);
    }

    #[test]
    fn test_from_parameters_full() {
        let mut params = HashMap::new();
        params.insert("output_path".to_string(), "/tmp/test.hex".to_string());
        params.insert("format".to_string(), "hex".to_string());
        params.insert("enabled".to_string(), "false".to_string());
        let config = TeeConfig::from_parameters(&params).unwrap();
        assert_eq!(config.output_path, PathBuf::from("/tmp/test.hex"));
        assert_eq!(config.format, TeeFormat::Hex);
        assert!(!config.enabled);
    }

    #[test]
    fn test_from_parameters_missing_output_path() {
        let params = HashMap::new();
        let result = TeeConfig::from_parameters(&params);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("output_path"));
    }

    #[test]
    fn test_from_parameters_invalid_format() {
        let mut params = HashMap::new();
        params.insert("output_path".to_string(), "/tmp/test.bin".to_string());
        params.insert("format".to_string(), "invalid".to_string());
        let result = TeeConfig::from_parameters(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_tee_binary_format() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("tee-binary.bin");

        let service = TeeService::new();
        let config = TeeConfig {
            output_path: output_path.clone(),
            format: TeeFormat::Binary,
            enabled: true,
        };

        let test_data = b"Hello, World!";
        service.write_tee(test_data, &config, 0).unwrap();

        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Chunk 0"));
        assert!(contents.contains("Hello, World!"));
    }

    #[test]
    fn test_tee_hex_format() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("tee-hex.txt");

        let service = TeeService::new();
        let config = TeeConfig {
            output_path: output_path.clone(),
            format: TeeFormat::Hex,
            enabled: true,
        };

        let test_data = b"Hello, World!";
        service.write_tee(test_data, &config, 0).unwrap();

        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Chunk 0"));
        assert!(contents.contains("48 65 6c 6c 6f")); // "Hello" in hex
    }

    #[test]
    fn test_tee_text_format() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("tee-text.txt");

        let service = TeeService::new();
        let config = TeeConfig {
            output_path: output_path.clone(),
            format: TeeFormat::Text,
            enabled: true,
        };

        let test_data = b"Hello, World!";
        service.write_tee(test_data, &config, 0).unwrap();

        let contents = fs::read_to_string(&output_path).unwrap();
        assert!(contents.contains("Chunk 0"));
        assert!(contents.contains("Hello, World!"));
    }

    #[test]
    fn test_tee_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("tee-disabled.bin");

        let service = TeeService::new();
        let config = TeeConfig {
            output_path: output_path.clone(),
            format: TeeFormat::Binary,
            enabled: false,
        };

        service.write_tee(b"test", &config, 0).unwrap();

        // File should not exist when disabled
        assert!(!output_path.exists());
    }

    #[test]
    fn test_process_chunk_pass_through() {
        use pipeline_domain::entities::pipeline_stage::StageConfiguration;

        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("tee-process.bin");

        let service = TeeService::new();
        let original_data = b"Test data for pass-through".to_vec();
        let chunk = FileChunk::new(0, 0, original_data.clone(), false).unwrap();

        let mut params = HashMap::new();
        params.insert("output_path".to_string(), output_path.display().to_string());
        params.insert("format".to_string(), "binary".to_string());

        let config = StageConfiguration {
            algorithm: "tee".to_string(),
            operation: Operation::Forward,
            parameters: params,
            parallel_processing: false,
            chunk_size: None,
        };

        let mut context = ProcessingContext::new(
            PathBuf::from("/tmp/input"),
            PathBuf::from("/tmp/output"),
            100,
            SecurityContext::new(None, SecurityLevel::Public),
        );

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();

        // Data should pass through unchanged
        assert_eq!(result.data(), &original_data);

        // Tee file should exist
        assert!(output_path.exists());
    }

    #[test]
    fn test_stage_service_properties() {
        let service = TeeService::new();

        assert_eq!(service.position(), StagePosition::Any);
        assert!(service.is_reversible());
        assert_eq!(service.stage_type(), StageType::Transform);
    }
}
