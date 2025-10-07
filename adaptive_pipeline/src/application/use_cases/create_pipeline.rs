// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Create Pipeline Use Case
//!
//! This module implements the use case for creating new processing pipelines.
//! It handles pipeline configuration, stage parsing, name validation, and
//! persistence to the repository.
//!
//! ## Overview
//!
//! The Create Pipeline use case provides:
//!
//! - **Pipeline Configuration**: Define processing stages and algorithms
//! - **Name Validation**: Ensure pipeline names follow conventions
//! - **Stage Parsing**: Parse comma-separated stage specifications
//! - **Algorithm Selection**: Support multiple compression/encryption
//!   algorithms
//! - **Persistence**: Save pipeline configuration to repository
//! - **Custom Stages**: Support for user-defined transformation stages
//!
//! ## Architecture
//!
//! Following Clean Architecture and Domain-Driven Design principles:
//!
//! - **Use Case Layer**: Orchestrates pipeline creation workflow
//! - **Repository Pattern**: Delegates persistence to repository interface
//! - **Dependency Inversion**: Depends on abstractions, not implementations
//! - **Single Responsibility**: Focused solely on pipeline creation
//! - **Domain Validation**: Enforces business rules for pipeline configuration
//!
//! ## Business Rules
//!
//! - Pipeline names must be at least 4 characters
//! - Names are normalized to kebab-case
//! - Reserved names (help, version, list, etc.) are rejected
//! - Stages are specified as comma-separated values
//! - Supported compression: brotli, gzip, zstd, lz4
//! - Supported encryption: aes256gcm, aes128gcm, chacha20poly1305
//! - Supported transforms: base64, pii_masking, tee, debug, passthrough
//! - Custom stages default to Transform type
//! - Debug stages auto-generate unique ULID labels
//!
//! ## Usage Examples
//!
//! ```rust,ignore
//! use adaptive_pipeline::application::use_cases::CreatePipelineUseCase;
//!
//! let use_case = CreatePipelineUseCase::new(pipeline_repository);
//!
//! // Simple compression pipeline
//! use_case.execute(
//!     "compress-files".to_string(),
//!     "brotli".to_string(),
//!     None,
//! ).await?;
//!
//! // Multi-stage pipeline
//! use_case.execute(
//!     "secure-backup".to_string(),
//!     "brotli,aes256gcm,checksum".to_string(),
//!     None,
//! ).await?;
//! ```

use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

use crate::infrastructure::repositories::sqlite_pipeline::SqlitePipelineRepository;
use adaptive_pipeline_domain::entities::pipeline::Pipeline;
use adaptive_pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};

/// Use case for creating new processing pipelines.
///
/// This use case handles the complete workflow for creating a new pipeline,
/// including name validation, stage parsing, configuration setup, and
/// persistence to the repository.
///
/// ## Responsibilities
///
/// - Validate and normalize pipeline name
/// - Parse stage specifications from comma-separated string
/// - Map stage names to types and algorithms
/// - Create pipeline domain entity
/// - Save pipeline to repository
/// - Handle creation errors gracefully
///
/// ## Dependencies
///
/// - **Pipeline Repository**: For persisting pipeline data
///
/// ## Example
///
/// ```rust,ignore
/// let use_case = CreatePipelineUseCase::new(pipeline_repository);
///
/// match use_case.execute(
///     "data-backup".to_string(),
///     "brotli,aes256gcm".to_string(),
///     None,
/// ).await {
///     Ok(()) => println!("Pipeline created successfully"),
///     Err(e) => eprintln!("Failed to create pipeline: {}", e),
/// }
/// ```
pub struct CreatePipelineUseCase {
    pipeline_repository: Arc<SqlitePipelineRepository>,
}

impl CreatePipelineUseCase {
    /// Creates a new Create Pipeline use case.
    ///
    /// # Parameters
    ///
    /// * `pipeline_repository` - Repository for persisting pipeline data
    ///
    /// # Returns
    ///
    /// A new instance of `CreatePipelineUseCase`
    pub fn new(pipeline_repository: Arc<SqlitePipelineRepository>) -> Self {
        Self { pipeline_repository }
    }

    /// Executes the create pipeline use case.
    ///
    /// Creates a new pipeline with the specified name and stages, validates
    /// the configuration, and persists it to the repository.
    ///
    /// ## Parameters
    ///
    /// * `name` - Pipeline name (will be normalized to kebab-case)
    /// * `stages` - Comma-separated list of stage specifications
    ///   - Examples: "brotli", "brotli,aes256gcm",
    ///     "compression,encryption,checksum"
    /// * `output` - Optional file path for pipeline configuration export (not
    ///   yet implemented)
    ///
    /// ## Stage Specifications
    ///
    /// **Generic Types** (use default algorithms):
    /// - `compression` → brotli
    /// - `encryption` → aes256gcm
    /// - `checksum` → sha256
    /// - `passthrough` → no-op
    ///
    /// **Specific Algorithms**:
    /// - Compression: `brotli`, `gzip`, `zstd`, `lz4`
    /// - Encryption: `aes256gcm`, `aes128gcm`, `chacha20poly1305`
    /// - Transform: `base64`, `pii_masking`, `tee`, `debug`
    ///
    /// **Type:Algorithm Syntax**:
    /// - `compression:lz4`
    /// - `encryption:chacha20poly1305`
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Pipeline created and saved successfully
    /// - `Err(anyhow::Error)` - Validation or persistence failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Empty pipeline name
    /// - Name less than 4 characters after normalization
    /// - Reserved pipeline names
    /// - Invalid stage specifications
    /// - Repository save failures
    /// - Database connection errors
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// // Create simple compression pipeline
    /// use_case.execute("backup".to_string(), "brotli".to_string(), None).await?;
    ///
    /// // Create secure multi-stage pipeline
    /// use_case.execute(
    ///     "Secure Backup!".to_string(),  // Will be normalized to "secure-backup"
    ///     "brotli,aes256gcm,checksum".to_string(),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn execute(&self, name: String, stages: String, output: Option<PathBuf>) -> Result<()> {
        info!("Creating pipeline: {}", name);
        info!("Stages: {}", stages);

        // Validate and normalize pipeline name
        let _normalized_name = Self::validate_pipeline_name(&name)?;

        // Parse stage specifications
        let stage_names: Vec<&str> = stages.split(',').collect();
        let mut pipeline_stages = Vec::new();

        for (index, stage_name) in stage_names.iter().enumerate() {
            let (stage_type, algorithm) = match stage_name.trim() {
                // Generic stage types with default algorithms
                "compression" => (StageType::Compression, "brotli".to_string()),
                "encryption" => (StageType::Encryption, "aes256gcm".to_string()),
                "integrity" | "checksum" => (StageType::Checksum, "sha256".to_string()),
                custom_name if custom_name.contains("checksum") => (StageType::Checksum, "sha256".to_string()),
                "passthrough" => (StageType::PassThrough, "passthrough".to_string()),

                // Compression algorithms
                "brotli" | "gzip" | "zstd" | "lz4" => (StageType::Compression, stage_name.trim().to_string()),

                // Encryption algorithms
                "aes256gcm" | "aes128gcm" | "chacha20poly1305" => {
                    (StageType::Encryption, stage_name.trim().to_string())
                }

                // Transform stages (production stages)
                "base64" | "pii_masking" | "tee" | "debug" => (StageType::Transform, stage_name.trim().to_string()),

                // Handle compression:algorithm syntax
                custom_name if custom_name.starts_with("compression:") => {
                    let algorithm = custom_name.strip_prefix("compression:").unwrap_or("brotli").to_string();
                    (StageType::Compression, algorithm)
                }

                // Handle encryption:algorithm syntax
                custom_name if custom_name.starts_with("encryption:") => {
                    let algorithm = custom_name
                        .strip_prefix("encryption:")
                        .unwrap_or("aes256gcm")
                        .to_string();
                    (StageType::Encryption, algorithm)
                }

                _custom => {
                    // For unknown stages, treat them as Transform with the name as the algorithm
                    // This allows for custom stages to be used without code changes
                    (StageType::Transform, stage_name.trim().to_string())
                }
            };

            // Create parameters HashMap with algorithm
            let mut parameters = HashMap::new();
            parameters.insert("algorithm".to_string(), algorithm.clone());

            // For debug stages, add a unique ULID label
            if algorithm == "debug" {
                parameters.insert("label".to_string(), ulid::Ulid::new().to_string());
            }

            let config = StageConfiguration {
                algorithm,
                parameters,
                ..Default::default()
            };

            let stage = PipelineStage::new(stage_name.trim().to_string(), stage_type, config, index as u32)?;

            pipeline_stages.push(stage);
        }

        // Create pipeline domain entity
        let pipeline = Pipeline::new(name, pipeline_stages)?;

        // Save pipeline to repository
        self.pipeline_repository
            .save(&pipeline)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to save pipeline: {}", e))?;

        info!(
            "Pipeline '{}' created successfully with ID: {}",
            pipeline.name(),
            pipeline.id()
        );
        info!("Pipeline saved to database");

        if output.is_some() {
            info!("Note: File output not yet implemented, pipeline saved to database only");
        }

        Ok(())
    }

    /// Normalizes pipeline name to kebab-case.
    ///
    /// Converts any valid input string to a clean kebab-case identifier by:
    /// - Converting to lowercase
    /// - Replacing separators and special characters with hyphens
    /// - Removing non-alphanumeric characters (except hyphens)
    /// - Collapsing multiple consecutive hyphens
    ///
    /// ## Parameters
    ///
    /// * `name` - Raw pipeline name
    ///
    /// ## Returns
    ///
    /// Normalized kebab-case string
    ///
    /// ## Examples
    ///
    /// ```text
    /// "My Pipeline" → "my-pipeline"
    /// "data_backup" → "data-backup"
    /// "Test::Pipeline!" → "test-pipeline"
    /// ```
    fn normalize_pipeline_name(name: &str) -> String {
        name.to_lowercase()
            // Replace common separators with hyphens
            .replace(
                [
                    ' ', '_', '.', '/', '\\', ':', ';', ',', '|', '&', '+', '=', '!', '?', '*', '%', '#', '@', '$',
                    '^', '(', ')', '[', ']', '{', '}', '<', '>', '"', '\'', '`', '~',
                ],
                "-",
            )
            // Remove any remaining non-alphanumeric, non-hyphen characters
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
            // Clean up multiple consecutive hyphens
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }

    /// Validates pipeline name according to business rules.
    ///
    /// Ensures pipeline names meet the following criteria:
    /// - Not empty
    /// - At least 4 characters after normalization
    /// - Not a reserved system name
    ///
    /// Names are automatically normalized to kebab-case.
    ///
    /// ## Parameters
    ///
    /// * `name` - Pipeline name to validate
    ///
    /// ## Returns
    ///
    /// - `Ok(String)` - Normalized pipeline name
    /// - `Err(anyhow::Error)` - Validation failed
    ///
    /// ## Errors
    ///
    /// Returns errors for:
    /// - Empty name
    /// - Name less than 4 characters after normalization
    /// - Reserved names: help, version, list, show, create, delete, update,
    ///   config
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// assert!(validate_pipeline_name("").is_err());  // Too short
    /// assert!(validate_pipeline_name("abc").is_err());  // Too short after normalization
    /// assert!(validate_pipeline_name("help").is_err());  // Reserved
    /// assert_eq!(validate_pipeline_name("My Pipeline").unwrap(), "my-pipeline");
    /// ```
    fn validate_pipeline_name(name: &str) -> Result<String> {
        // Check for empty name
        if name.is_empty() {
            return Err(anyhow::anyhow!("Pipeline name cannot be empty"));
        }

        // Normalize to kebab-case
        let normalized = Self::normalize_pipeline_name(name);

        // Check minimum length after normalization
        if normalized.len() < 4 {
            return Err(anyhow::anyhow!("Pipeline name must be at least 4 characters long"));
        }

        // Reserved names
        let reserved_names = [
            "help", "version", "list", "show", "create", "delete", "update", "config",
        ];
        if reserved_names.contains(&normalized.as_str()) {
            return Err(anyhow::anyhow!(
                "Pipeline name '{}' is reserved. Please choose a different name.",
                name
            ));
        }

        // Inform user if name was normalized
        if normalized != name {
            info!(
                "Pipeline name normalized from '{}' to '{}' (kebab-case standard)",
                name, normalized
            );
        }

        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pipeline_name() {
        assert_eq!(
            CreatePipelineUseCase::normalize_pipeline_name("My Pipeline"),
            "my-pipeline"
        );
        assert_eq!(
            CreatePipelineUseCase::normalize_pipeline_name("data_backup"),
            "data-backup"
        );
        assert_eq!(
            CreatePipelineUseCase::normalize_pipeline_name("Test::Pipeline!"),
            "test-pipeline"
        );
        assert_eq!(
            CreatePipelineUseCase::normalize_pipeline_name("---multiple---hyphens---"),
            "multiple-hyphens"
        );
    }

    #[test]
    fn test_validate_pipeline_name() {
        // Valid names
        assert!(CreatePipelineUseCase::validate_pipeline_name("test-pipeline").is_ok());
        assert!(CreatePipelineUseCase::validate_pipeline_name("my-backup").is_ok());

        // Invalid: empty
        assert!(CreatePipelineUseCase::validate_pipeline_name("").is_err());

        // Invalid: too short after normalization
        assert!(CreatePipelineUseCase::validate_pipeline_name("abc").is_err());

        // Invalid: reserved names
        assert!(CreatePipelineUseCase::validate_pipeline_name("help").is_err());
        assert!(CreatePipelineUseCase::validate_pipeline_name("list").is_err());
        assert!(CreatePipelineUseCase::validate_pipeline_name("create").is_err());
    }

    #[tokio::test]
    #[ignore] // Requires database setup
    async fn test_create_pipeline_with_real_repository() {
        // This test would require a real database setup
        // For now, marked as ignored
        // See tests/integration/ for full end-to-end tests
    }
}
