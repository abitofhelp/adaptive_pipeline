// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Stage Service Trait
//!
//! This module defines the unified trait interface that ALL pipeline processing
//! stages must implement, whether they are built-in (compression, encryption,
//! checksum) or custom user-defined stages.
//!
//! ## Overview
//!
//! The `StageService` trait provides a consistent contract for all processing
//! stages in the pipeline system. It handles:
//!
//! - **Chunk Processing**: Transform file chunks (forward or reverse)
//! - **Position Declaration**: Specify where stage can execute
//!   (PreBinary/PostBinary/Any)
//! - **Reversibility**: Indicate if stage supports bidirectional operations
//! - **Type Identification**: Declare the stage type for dispatch and
//!   validation
//!
//! ## Architecture
//!
//! Following Domain-Driven Design and Clean Architecture principles:
//!
//! - **Domain Layer**: Defines the `StageService` trait (this module)
//! - **Infrastructure Layer**: Implements concrete services
//! - **Dependency Inversion**: Domain defines interface; infrastructure
//!   implements
//! - **Uniform Interface**: All stages use same method signature
//!
//! ## Why One Unified Trait?
//!
//! Previous designs had separate traits for each service type
//! (CompressionService, EncryptionService, etc.). This created complexity:
//!
//! - Different method signatures for similar operations
//! - Hard to add new stage types
//! - Complex dispatch logic in StageExecutor
//! - No common interface for custom stages
//!
//! The unified approach provides:
//!
//! - ✅ **Consistency**: All stages implement same interface
//! - ✅ **Simplicity**: Single trait to understand
//! - ✅ **Extensibility**: Easy to add custom stages
//! - ✅ **Type Safety**: Rust's trait system enforces contract
//! - ✅ **Serialization**: Works with HashMap-based config (no generics
//!   nightmare)
//!
//! ## The Binary Boundary
//!
//! The `StagePosition` concept enforces architectural constraints:
//!
//! - **PreBinary**: Stages that need raw/plaintext data (PII masking, text
//!   transforms)
//! - **PostBinary**: Stages operating on compressed/encrypted data (checksums,
//!   metrics)
//! - **Any**: Position-agnostic stages (observability, tee, pass-through)
//!
//! Pipeline validation ensures PreBinary stages never execute after
//! compression/encryption, preventing bugs like "trying to find SSNs in
//! compressed gibberish."
//!
//! ## Method Parameters - The HashMap Approach
//!
//! All stages receive parameters via `StageConfiguration.parameters:
//! HashMap<String, String>`.
//!
//! **Why HashMap instead of generics?**
//!
//! - ✅ Serializable to JSON/database
//! - ✅ Stored in `.adapipe` binary format
//! - ✅ Backward compatible with existing files
//! - ✅ No generic type explosion (`Arc<dyn StageService<T>>` impossible)
//! - ✅ Services extract typed data when needed (e.g., KeyMaterial from
//!   HashMap)
//!
//! **Example:**
//!
//! ```rust,ignore
//! // Encryption service extracts KeyMaterial from parameters
//! let key_b64 = config.parameters.get("key")
//!     .ok_or_else(|| PipelineError::MissingParameter("key".into()))?;
//! let key_material = KeyMaterial::from_base64(key_b64)?;
//! ```
//!
//! ## Usage Examples
//!
//! ### Implementing a Built-in Service
//!
//! ```rust,ignore
//! use adaptive_pipeline_domain::services::stage_service::StageService;
//! use adaptive_pipeline_domain::entities::{StageType, StagePosition, StageConfiguration};
//! use adaptive_pipeline_domain::value_objects::file_chunk::FileChunk;
//! use adaptive_pipeline_domain::value_objects::processing_context::ProcessingContext;
//! use adaptive_pipeline_domain::PipelineError;
//!
//! pub struct BrotliCompressionService;
//!
//! impl StageService for BrotliCompressionService {
//!     fn process_chunk(
//!         &self,
//!         chunk: FileChunk,
//!         config: &StageConfiguration,
//!         context: &mut ProcessingContext,
//!     ) -> Result<FileChunk, PipelineError> {
//!         match config.operation {
//!             Operation::Forward => {
//!                 // Compress the chunk
//!                 let compressed = brotli::compress(&chunk.data)?;
//!                 Ok(FileChunk::new(chunk.sequence_number, compressed))
//!             }
//!             Operation::Reverse => {
//!                 // Decompress the chunk
//!                 let decompressed = brotli::decompress(&chunk.data)?;
//!                 Ok(FileChunk::new(chunk.sequence_number, decompressed))
//!             }
//!         }
//!     }
//!
//!     fn position(&self) -> StagePosition {
//!         StagePosition::PreBinary  // Compression marks the binary boundary
//!     }
//!
//!     fn is_reversible(&self) -> bool {
//!         true  // Compression can be decompressed
//!     }
//!
//!     fn stage_type(&self) -> StageType {
//!         StageType::Compression
//!     }
//! }
//! ```
//!
//! ### Implementing a Custom Service
//!
//! ```rust,ignore
//! use adaptive_pipeline_domain::services::stage_service::StageService;
//! use adaptive_pipeline_domain::entities::{StageType, StagePosition};
//!
//! pub struct PiiMaskingService;
//!
//! impl StageService for PiiMaskingService {
//!     fn process_chunk(
//!         &self,
//!         chunk: FileChunk,
//!         config: &StageConfiguration,
//!         context: &mut ProcessingContext,
//!     ) -> Result<FileChunk, PipelineError> {
//!         match config.operation {
//!             Operation::Forward => {
//!                 // Mask SSNs, credit cards, etc.
//!                 let masked = self.mask_pii(&chunk.data)?;
//!                 Ok(FileChunk::new(chunk.sequence_number, masked))
//!             }
//!             Operation::Reverse => {
//!                 Err(PipelineError::StageConfiguration(
//!                     "PII masking is not reversible".to_string()
//!                 ))
//!             }
//!         }
//!     }
//!
//!     fn position(&self) -> StagePosition {
//!         StagePosition::PreBinary  // Must see plaintext to find PII
//!     }
//!
//!     fn is_reversible(&self) -> bool {
//!         false  // Cannot unmask PII
//!     }
//!
//!     fn stage_type(&self) -> StageType {
//!         StageType::Transform
//!     }
//! }
//! ```
//!
//! ## Integration with Pipeline
//!
//! The `StageExecutor` dispatches to appropriate services based on `StageType`:
//!
//! ```rust,ignore
//! async fn execute_stage(
//!     service: Arc<dyn StageService>,
//!     stage: &PipelineStage,
//!     chunk: FileChunk,
//!     context: &mut ProcessingContext,
//! ) -> Result<FileChunk, PipelineError> {
//!     service.process_chunk(chunk, stage.configuration(), context)
//! }
//! ```
//!
//! ## Pipeline Validation
//!
//! During pipeline creation, stages are validated for correct positioning:
//!
//! ```rust,ignore
//! fn validate_stage_order(stages: &[PipelineStage]) -> Result<(), PipelineError> {
//!     let mut seen_binary_boundary = false;
//!
//!     for stage in stages {
//!         let position = /* get service position */;
//!
//!         if seen_binary_boundary && position == StagePosition::PreBinary {
//!             return Err(PipelineError::InvalidStageOrder(
//!                 format!("PreBinary stage '{}' after compression/encryption", stage.name())
//!             ));
//!         }
//!
//!         if matches!(stage.stage_type(), StageType::Compression | StageType::Encryption) {
//!             seen_binary_boundary = true;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Best Practices for Implementors
//!
//! ### Thread Safety
//! - Implement `Send + Sync` (enforced by trait bounds)
//! - Avoid mutable state; use parameters for configuration
//!
//! ### Error Handling
//! - Return descriptive `PipelineError` variants
//! - Include context in error messages for debugging
//!
//! ### Reversibility
//! - If `is_reversible() == false`, error on `Operation::Reverse`
//! - Provide clear error message explaining why it's not reversible
//!
//! ### Performance
//! - Process chunks independently (enable parallelization)
//! - Avoid unbounded buffering
//! - Update metrics in `ProcessingContext`
//!
//! ### Standards Compliance
//! - All implementations must adhere to `claude_rust.md` standards
//! - Follow Domain-Driven Design principles
//! - Include comprehensive rustdoc documentation

use crate::entities::{ProcessingContext, StageConfiguration, StagePosition, StageType};
use crate::value_objects::file_chunk::FileChunk;
use crate::PipelineError;
use std::collections::HashMap;

/// Trait for converting HashMap parameters to typed configuration objects.
///
/// **This is a TRAIT, not a function.** Like Rust's standard `FromStr` trait,
/// `FromParameters` defines a contract that configuration types must implement
/// to enable type-safe construction from HashMap parameters.
///
/// ## Trait Pattern (Similar to `FromStr`)
///
/// Just as `FromStr` enables parsing from strings:
/// ```rust,ignore
/// let num: i32 = "42".parse()?; // Uses FromStr trait
/// ```
///
/// `FromParameters` enables parsing from HashMap:
/// ```rust,ignore
/// let config = CompressionConfig::from_parameters(&params)?; // Uses FromParameters trait
/// ```
///
/// ## Purpose
///
/// This trait enables type-safe extraction of stage-specific configuration from
/// the generic `StageConfiguration.parameters: HashMap<String, String>`
/// storage. It bridges the gap between serializable storage and typed domain
/// objects.
///
/// **Key Benefits:**
/// - **Type Safety**: Convert string-based HashMap to typed config with
///   validation
/// - **Reusability**: Common pattern for all service configurations
/// - **Testability**: Conversion logic is isolated and easily testable
/// - **Error Handling**: Provides clear errors for missing/invalid parameters
/// - **Contract Enforcement**: Compiler ensures all configs implement the trait
///
/// ## Why This Approach?
///
/// We use `HashMap<String, String>` in `StageConfiguration` because:
/// - ✅ Serializable to JSON/database without complex generics
/// - ✅ Stored in `.adapipe` binary format (backward compatible)
/// - ✅ Simple, uniform storage across all stage types
/// - ✅ No generic type explosion (`Arc<dyn StageService>` works)
///
/// But we need typed configs for actual processing:
/// - `CompressionConfig` (algorithm, level, dictionary, etc.)
/// - `EncryptionConfig` (algorithm, key_derivation, key_size, etc.)
///
/// `FromParameters` bridges this gap cleanly.
///
/// ## Implementation Pattern
///
/// ```rust,ignore
/// use std::collections::HashMap;
/// use adaptive_pipeline_domain::PipelineError;
/// use adaptive_pipeline_domain::services::FromParameters;
///
/// #[derive(Debug, Clone)]
/// pub struct MyStageConfig {
///     pub algorithm: String,
///     pub threshold: u32,
/// }
///
/// impl FromParameters for MyStageConfig {
///     fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
///         let algorithm = params.get("algorithm")
///             .ok_or_else(|| PipelineError::MissingParameter("algorithm".into()))?
///             .clone();
///
///         let threshold = params.get("threshold")
///             .and_then(|s| s.parse().ok())
///             .unwrap_or(100);
///
///         Ok(Self { algorithm, threshold })
///     }
/// }
/// ```
///
/// ## Usage in StageService
///
/// ```rust,ignore
/// impl StageService for MyService {
///     fn process_chunk(
///         &self,
///         chunk: FileChunk,
///         config: &StageConfiguration,
///         context: &mut ProcessingContext,
///     ) -> Result<FileChunk, PipelineError> {
///         // Type-safe extraction
///         let my_config = MyStageConfig::from_parameters(&config.parameters)?;
///
///         // Use typed config
///         self.process_with_config(chunk, &my_config, context)
///     }
/// }
/// ```
///
/// ## Error Handling
///
/// Implementations should return:
/// - `PipelineError::MissingParameter` for required missing parameters
/// - `PipelineError::InvalidParameter` for invalid values
/// - `PipelineError::ConfigurationError` for semantic errors
///
/// ## Best Practices
///
/// - Provide sensible defaults for optional parameters
/// - Validate parameter values (ranges, formats, etc.)
/// - Include context in error messages for debugging
/// - Document expected parameter keys and formats
pub trait FromParameters: Sized {
    /// Converts HashMap parameters to typed configuration.
    ///
    /// # Parameters
    ///
    /// * `params` - HashMap containing string key-value pairs from
    ///   `StageConfiguration`
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` - Successfully parsed typed configuration
    /// - `Err(PipelineError)` - Missing or invalid parameters
    ///
    /// # Errors
    ///
    /// Should return errors for:
    /// - Missing required parameters
    /// - Invalid parameter values (parse errors, out of range, etc.)
    /// - Semantic validation failures
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError>;
}

/// Unified trait that all pipeline processing stages must implement.
///
/// This trait defines the contract for all stages in the pipeline system,
/// whether built-in (compression, encryption, checksum) or custom user-defined
/// stages. It provides a consistent interface for chunk processing while
/// allowing stage-specific behavior through configuration parameters.
///
/// ## Thread Safety
///
/// All implementations must be `Send + Sync` to support concurrent processing
/// across multiple threads and async tasks. This is enforced by the trait
/// bounds.
///
/// ## Synchronous Processing
///
/// The `process_chunk` method is synchronous (not `async`). This follows domain
/// layer principles of keeping core business logic independent of async runtime
/// concerns. Infrastructure adapters handle async boundaries when needed.
///
/// ## Configuration Parameters
///
/// All stage-specific parameters are passed via `StageConfiguration.parameters`
/// as a `HashMap<String, String>`. Services extract and parse typed data as
/// needed:
///
/// - **Encryption**: Extracts `KeyMaterial` from base64-encoded key
/// - **Compression**: Extracts compression level, window size, etc.
/// - **Custom Stages**: Extract any custom parameters
///
/// This approach maintains serialization compatibility and avoids generic
/// type complexity.
///
/// ## Position Validation
///
/// The `position()` method declares where a stage can execute in the pipeline:
///
/// - **PreBinary**: Before compression/encryption (sees original data)
/// - **PostBinary**: After compression/encryption (operates on binary)
/// - **Any**: Position-agnostic (works anywhere)
///
/// The pipeline validates stage ordering during creation to prevent bugs
/// like placing text-transformation stages after compression.
///
/// ## Error Handling
///
/// All errors should be returned as `PipelineError` with descriptive messages
/// to aid debugging and error reporting. Never panic in production code.
pub trait StageService: Send + Sync {
    /// Process a file chunk according to the operation (Forward or Reverse).
    ///
    /// This is the core processing method that applies the stage's
    /// transformation to a chunk of data. Implementations should:
    ///
    /// - Check `config.operation` to determine direction (Forward/Reverse)
    /// - Extract any needed parameters from `config.parameters`
    /// - Apply the appropriate transformation
    /// - Update metrics in `ProcessingContext`
    /// - Return transformed chunk or detailed error
    ///
    /// ## Parameters
    ///
    /// * `chunk` - The input chunk to process
    /// * `config` - Stage configuration including operation, algorithm, and
    ///   parameters
    /// * `context` - Processing context for metrics and metadata
    ///
    /// ## Returns
    ///
    /// - `Ok(FileChunk)` - Successfully processed chunk with transformed data
    /// - `Err(PipelineError)` - Processing failed with descriptive error
    ///
    /// ## Errors
    ///
    /// Implementations should return errors for:
    /// - Invalid parameters in config
    /// - Unsupported operations (e.g., Reverse when not reversible)
    /// - Processing failures (compression errors, encryption failures, etc.)
    /// - Resource exhaustion or allocation failures
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// fn process_chunk(
    ///     &self,
    ///     chunk: FileChunk,
    ///     config: &StageConfiguration,
    ///     context: &mut ProcessingContext,
    /// ) -> Result<FileChunk, PipelineError> {
    ///     // Extract parameters
    ///     let level = config.parameters.get("level")
    ///         .and_then(|s| s.parse().ok())
    ///         .unwrap_or(6);
    ///
    ///     // Process based on operation
    ///     match config.operation {
    ///         Operation::Forward => self.compress(chunk, level),
    ///         Operation::Reverse => self.decompress(chunk),
    ///     }
    /// }
    /// ```
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Returns the position where this stage can execute in the pipeline.
    ///
    /// This declaration is used for pipeline validation to ensure stages are
    /// correctly ordered relative to the binary transformation boundary
    /// (compression and encryption).
    ///
    /// ## Position Types
    ///
    /// - **PreBinary**: Must execute before compression/encryption
    ///   - Examples: PII masking, text transformations, Base64 encoding
    ///   - Reason: These need to see/modify original data format
    ///
    /// - **PostBinary**: Executes after compression/encryption
    ///   - Examples: Output checksums, metrics collection
    ///   - Reason: These operate on final binary format
    ///
    /// - **Any**: Can execute at any position
    ///   - Examples: Tee stages, observability, pass-through
    ///   - Reason: Position-agnostic operations
    ///
    /// ## Validation
    ///
    /// The pipeline validates during creation that PreBinary stages don't
    /// appear after compression/encryption stages, preventing common bugs.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// fn position(&self) -> StagePosition {
    ///     StagePosition::PreBinary  // Must see plaintext
    /// }
    /// ```
    fn position(&self) -> StagePosition;

    /// Indicates whether this stage supports reverse operations.
    ///
    /// This determines if the stage can be used in restoration pipelines
    /// that reverse the original processing operations.
    ///
    /// ## Reversibility
    ///
    /// - **Reversible (true)**: Supports both Forward and Reverse operations
    ///   - Examples: Compression/decompression, encryption/decryption,
    ///     encoding/decoding
    ///   - Can be included in restoration pipelines
    ///
    /// - **Non-reversible (false)**: Only supports Forward operation
    ///   - Examples: PII masking, hashing, one-way transformations
    ///   - Cannot be reversed; errors on `Operation::Reverse`
    ///   - Files processed with these stages cannot be fully restored
    ///
    /// ## Usage
    ///
    /// - Pipeline validation checks reversibility when creating restoration
    ///   pipelines
    /// - Non-reversible stages should return error for `Operation::Reverse`
    /// - Documentation should clearly state if stage is one-way
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// fn is_reversible(&self) -> bool {
    ///     false  // PII masking cannot be reversed
    /// }
    ///
    /// fn process_chunk(...) -> Result<FileChunk, PipelineError> {
    ///     match config.operation {
    ///         Operation::Forward => self.mask_pii(chunk),
    ///         Operation::Reverse => Err(PipelineError::StageConfiguration(
    ///             "PII masking is not reversible".to_string()
    ///         )),
    ///     }
    /// }
    /// ```
    fn is_reversible(&self) -> bool;

    /// Returns the type classification of this stage.
    ///
    /// The stage type is used for:
    /// - Service dispatch in StageExecutor
    /// - Binary boundary detection (Compression, Encryption)
    /// - Metrics collection and categorization
    /// - Documentation and debugging
    ///
    /// ## Stage Types
    ///
    /// - **Compression**: Data compression/decompression stages
    /// - **Encryption**: Cryptographic encryption/decryption stages
    /// - **Checksum**: Data integrity verification stages
    /// - **Transform**: Data transformation stages (custom logic)
    /// - **PassThrough**: No-op or observability stages
    ///
    /// ## Binary Boundary
    ///
    /// Compression and Encryption types mark the binary transformation
    /// boundary. After these stages, data is no longer in its original
    /// format.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// fn stage_type(&self) -> StageType {
    ///     StageType::Transform  // Custom transformation stage
    /// }
    /// ```
    fn stage_type(&self) -> StageType;
}
