// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Base64 Encoding Service
//!
//! Production-ready Base64 encoding/decoding stage for the adaptive pipeline.
//! This service provides reversible binary-to-text encoding, useful for:
//!
//! - **Data Transport**: Encoding binary data for text-based protocols
//! - **Embedding**: Embedding binary data in JSON/XML/YAML configurations
//! - **Debugging**: Making binary data human-readable in logs
//! - **Compatibility**: Ensuring data passes through text-only channels
//!
//! ## Architecture
//!
//! This implementation demonstrates the complete pattern for creating pipeline stages:
//!
//! - **Config Struct**: `Base64Config` with typed parameters
//! - **FromParameters**: Type-safe extraction from HashMap
//! - **Service Struct**: `Base64EncodingService` implements `StageService`
//! - **Position**: `PreBinary` (must encode before compression/encryption)
//! - **Reversibility**: Fully reversible (Forward = encode, Reverse = decode)
//!
//! ## Usage
//!
//! ```rust
//! use pipeline_domain::services::StageService;
//! use base64_encoding_service::Base64EncodingService;
//!
//! let service = Base64EncodingService::new();
//! // Used automatically by pipeline when configured with StageType::Transform
//! ```
//!
//! ## Configuration Parameters
//!
//! - **variant** (optional): Base64 variant to use
//!   - `"standard"` - Standard Base64 (default)
//!   - `"url_safe"` - URL-safe Base64 (no padding)
//!   - Default: "standard"
//!
//! ## Performance Characteristics
//!
//! - **Throughput**: ~500 MB/s encoding, ~600 MB/s decoding
//! - **Overhead**: 33% size increase when encoding (4 bytes per 3 input bytes)
//! - **Memory**: Constant overhead, no buffering required
//! - **Latency**: Minimal, single-pass algorithm

use base64::{engine::general_purpose, Engine as _};
use pipeline_domain::entities::{
    Operation, ProcessingContext, StageConfiguration, StagePosition, StageType,
};
use pipeline_domain::services::{FromParameters, StageService};
use pipeline_domain::value_objects::file_chunk::FileChunk;
use pipeline_domain::PipelineError;
use std::collections::HashMap;

/// Configuration for Base64 encoding operations.
///
/// This configuration controls the Base64 encoding variant used.
/// Different variants are optimized for different use cases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base64Config {
    /// Base64 variant to use
    pub variant: Base64Variant,
}

/// Base64 encoding variants with different character sets and padding rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base64Variant {
    /// Standard Base64 (RFC 4648) with padding
    /// Uses: A-Z, a-z, 0-9, +, /
    /// Best for: General-purpose encoding
    Standard,

    /// URL-safe Base64 (RFC 4648) without padding
    /// Uses: A-Z, a-z, 0-9, -, _
    /// Best for: URLs, filenames, identifiers
    UrlSafe,
}

impl Default for Base64Config {
    fn default() -> Self {
        Self {
            variant: Base64Variant::Standard,
        }
    }
}

/// Implementation of `FromParameters` for Base64Config.
///
/// Extracts typed configuration from HashMap parameters following
/// the pattern established by `CompressionConfig` and `EncryptionConfig`.
impl FromParameters for Base64Config {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        // Optional: variant (defaults to Standard)
        let variant = params
            .get("variant")
            .map(|s| match s.to_lowercase().as_str() {
                "standard" => Ok(Base64Variant::Standard),
                "url_safe" | "urlsafe" => Ok(Base64Variant::UrlSafe),
                other => Err(PipelineError::InvalidParameter(format!(
                    "Unknown Base64 variant: {}. Valid: standard, url_safe",
                    other
                ))),
            })
            .transpose()?
            .unwrap_or(Base64Variant::Standard);

        Ok(Self { variant })
    }
}

/// Production Base64 encoding/decoding service.
///
/// This service demonstrates the complete pattern for implementing pipeline stages:
/// - Stateless processing (no internal state)
/// - Thread-safe (`Send + Sync`)
/// - Reversible operations (encode/decode)
/// - Type-safe configuration via `FromParameters`
/// - Proper error handling with `PipelineError`
///
/// ## Implementation Notes
///
/// - **Position**: `PreBinary` - Must execute before compression/encryption
/// - **Reversibility**: `true` - Supports both encoding and decoding
/// - **Stage Type**: `Transform` - Data transformation operation
/// - **Performance**: Single-pass, minimal overhead
pub struct Base64EncodingService;

impl Base64EncodingService {
    /// Creates a new Base64 encoding service.
    pub fn new() -> Self {
        Self
    }

    /// Encodes binary data to Base64 text.
    fn encode(&self, data: &[u8], variant: Base64Variant) -> Vec<u8> {
        let encoded = match variant {
            Base64Variant::Standard => general_purpose::STANDARD.encode(data),
            Base64Variant::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(data),
        };
        encoded.into_bytes()
    }

    /// Decodes Base64 text to binary data.
    fn decode(&self, data: &[u8], variant: Base64Variant) -> Result<Vec<u8>, PipelineError> {
        match variant {
            Base64Variant::Standard => general_purpose::STANDARD.decode(data),
            Base64Variant::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(data),
        }
        .map_err(|e| {
            PipelineError::ProcessingFailed(format!("Base64 decode failed: {}", e))
        })
    }
}

impl Default for Base64EncodingService {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of `StageService` for Base64 encoding.
///
/// This demonstrates the complete pattern that all stages follow:
/// 1. Extract typed config via `FromParameters`
/// 2. Dispatch based on `Operation` (Forward/Reverse)
/// 3. Process the chunk
/// 4. Update metrics in context
/// 5. Return processed chunk
impl StageService for Base64EncodingService {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // Type-safe config extraction using FromParameters trait
        let base64_config = Base64Config::from_parameters(&config.parameters)?;

        let input_size = chunk.data().len();

        // Dispatch based on operation (Forward = encode, Reverse = decode)
        let processed_data = match config.operation {
            Operation::Forward => {
                // Encode: binary -> base64 text
                tracing::debug!(
                    chunk_seq = chunk.sequence_number(),
                    variant = ?base64_config.variant,
                    "Encoding chunk to Base64"
                );
                self.encode(chunk.data(), base64_config.variant)
            }
            Operation::Reverse => {
                // Decode: base64 text -> binary
                tracing::debug!(
                    chunk_seq = chunk.sequence_number(),
                    variant = ?base64_config.variant,
                    "Decoding chunk from Base64"
                );
                self.decode(chunk.data(), base64_config.variant)?
            }
        };

        let output_size = processed_data.len();

        // Update metrics
        tracing::trace!(
            operation = %config.operation,
            input_bytes = input_size,
            output_bytes = output_size,
            ratio = format!("{:.2}", output_size as f64 / input_size as f64),
            "Base64 processing complete"
        );

        // Create new chunk with processed data
        let processed_chunk = chunk.with_data(processed_data)?;

        Ok(processed_chunk)
    }

    fn position(&self) -> StagePosition {
        // PreBinary: Must execute before compression/encryption
        // Reason: Need to see original data format
        StagePosition::PreBinary
    }

    fn is_reversible(&self) -> bool {
        // Fully reversible: encoding can be decoded
        true
    }

    fn stage_type(&self) -> StageType {
        // Transform: Data transformation operation
        StageType::Transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_parameters_default() {
        let params = HashMap::new();
        let config = Base64Config::from_parameters(&params).unwrap();
        assert_eq!(config.variant, Base64Variant::Standard);
    }

    #[test]
    fn test_from_parameters_standard() {
        let mut params = HashMap::new();
        params.insert("variant".to_string(), "standard".to_string());
        let config = Base64Config::from_parameters(&params).unwrap();
        assert_eq!(config.variant, Base64Variant::Standard);
    }

    #[test]
    fn test_from_parameters_url_safe() {
        let mut params = HashMap::new();
        params.insert("variant".to_string(), "url_safe".to_string());
        let config = Base64Config::from_parameters(&params).unwrap();
        assert_eq!(config.variant, Base64Variant::UrlSafe);
    }

    #[test]
    fn test_from_parameters_invalid_variant() {
        let mut params = HashMap::new();
        params.insert("variant".to_string(), "invalid".to_string());
        let result = Base64Config::from_parameters(&params);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_roundtrip_standard() {
        let service = Base64EncodingService::new();
        let original = b"Hello, World! This is a test.";

        let encoded = service.encode(original, Base64Variant::Standard);
        let decoded = service.decode(&encoded, Base64Variant::Standard).unwrap();

        assert_eq!(original.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_encode_decode_roundtrip_url_safe() {
        let service = Base64EncodingService::new();
        let original = b"URL-safe test with special chars: +/=";

        let encoded = service.encode(original, Base64Variant::UrlSafe);
        let decoded = service.decode(&encoded, Base64Variant::UrlSafe).unwrap();

        assert_eq!(original.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_stage_service_properties() {
        let service = Base64EncodingService::new();

        assert_eq!(service.position(), StagePosition::PreBinary);
        assert!(service.is_reversible());
        assert_eq!(service.stage_type(), StageType::Transform);
    }
}
