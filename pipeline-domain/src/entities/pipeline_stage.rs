// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! Stage configuration example:

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::value_objects::StageId;
use crate::services::datetime_serde;
use crate::PipelineError;

/// Represents the type of processing performed by a pipeline stage.
///
/// This enum categorizes stages by their primary operation, enabling
/// the pipeline to make intelligent decisions about ordering, parallelization,
/// and resource allocation.
///
/// # Examples
///
/// ## Parsing stage types from strings
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::StageType;
/// use std::str::FromStr;
///
/// // Parse from lowercase
/// let compression = StageType::from_str("compression").unwrap();
/// assert_eq!(compression, StageType::Compression);
///
/// // Case-insensitive parsing
/// let encryption = StageType::from_str("ENCRYPTION").unwrap();
/// assert_eq!(encryption, StageType::Encryption);
///
/// // Display format
/// assert_eq!(format!("{}", StageType::Checksum), "checksum");
/// ```
///
/// ## Using stage types in pattern matching
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::StageType;
///
/// fn describe_stage(stage_type: StageType) -> &'static str {
///     match stage_type {
///         StageType::Compression => "Reduces data size",
///         StageType::Encryption => "Secures data",
///         StageType::Transform => "Modifies data structure",
///         StageType::Checksum => "Verifies data integrity",
///         StageType::PassThrough => "No modification",
///     }
/// }
///
/// assert_eq!(describe_stage(StageType::Compression), "Reduces data size");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageType {
    /// Compression or decompression operations
    Compression,
    /// Encryption or decryption operations
    Encryption,
    /// Data transformation operations
    Transform,
    /// Checksum calculation and verification
    Checksum,
    /// Pass-through stage that doesn't modify data
    PassThrough,
}

impl std::fmt::Display for StageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StageType::Compression => write!(f, "compression"),
            StageType::Encryption => write!(f, "encryption"),
            StageType::Transform => write!(f, "transform"),
            StageType::Checksum => write!(f, "checksum"),
            StageType::PassThrough => write!(f, "passthrough"),
        }
    }
}

impl std::str::FromStr for StageType {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "compression" => Ok(StageType::Compression),
            "encryption" => Ok(StageType::Encryption),
            "transform" => Ok(StageType::Transform),
            "checksum" => Ok(StageType::Checksum),
            "passthrough" => Ok(StageType::PassThrough),
            _ => Err(PipelineError::InvalidConfiguration(format!("Unknown stage type: {}", s))),
        }
    }
}

///
/// ### Encryption Configuration
///
/// ### Default Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfiguration {
    pub algorithm: String,
    pub parameters: HashMap<String, String>,
    pub parallel_processing: bool,
    pub chunk_size: Option<usize>,
}

impl StageConfiguration {
    /// Creates a new stage configuration
    pub fn new(algorithm: String, parameters: HashMap<String, String>, parallel_processing: bool) -> Self {
        Self {
            algorithm,
            parameters,
            parallel_processing,
            chunk_size: None,
        }
    }
}

impl Default for StageConfiguration {
    fn default() -> Self {
        Self {
            algorithm: "default".to_string(),
            parameters: HashMap::new(),
            parallel_processing: true,
            chunk_size: None,
        }
    }
}

/// Core pipeline stage entity representing a single processing step.
///
/// A `PipelineStage` is a domain entity that encapsulates a specific data
/// transformation operation within a pipeline. Each stage has a unique
/// identity, maintains its own configuration, and can be enabled/disabled
/// independently.
///
/// ## Entity Characteristics
///
/// - **Identity**: Unique `StageId` that persists through configuration changes
/// - **Type Safety**: Strongly typed stage operations prevent configuration
///   errors
/// - **Ordering**: Explicit ordering ensures predictable execution sequence
/// - **Lifecycle**: Tracks creation and modification timestamps
/// - **State Management**: Can be enabled/disabled without removal
///
/// ## Stage Lifecycle
///
/// 1. **Creation**: Stage is created with initial configuration
/// 2. **Configuration**: Parameters can be updated as needed
/// 3. **Ordering**: Position in pipeline can be adjusted
/// 4. **Execution**: Stage processes data according to its configuration
/// 5. **Monitoring**: Timestamps track when changes occur
///
/// ## Usage Examples
///
/// ### Creating a Compression Stage
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
/// use std::collections::HashMap;
///
/// let mut params = HashMap::new();
/// params.insert("level".to_string(), "6".to_string());
///
/// let config = StageConfiguration::new("brotli".to_string(), params, true);
/// let stage = PipelineStage::new(
///     "compression".to_string(),
///     StageType::Compression,
///     config,
///     0,
/// ).unwrap();
///
/// assert_eq!(stage.name(), "compression");
/// assert_eq!(stage.stage_type(), &StageType::Compression);
/// assert_eq!(stage.algorithm(), "brotli");
/// assert!(stage.is_enabled());
/// ```
///
/// ### Creating an Encryption Stage
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
/// use std::collections::HashMap;
///
/// let mut params = HashMap::new();
/// params.insert("key_size".to_string(), "256".to_string());
///
/// let config = StageConfiguration::new("aes256gcm".to_string(), params, false);
/// let stage = PipelineStage::new(
///     "encryption".to_string(),
///     StageType::Encryption,
///     config,
///     1,
/// ).unwrap();
///
/// assert_eq!(stage.algorithm(), "aes256gcm");
/// assert_eq!(stage.order(), 1);
/// ```
///
/// ### Modifying Stage Configuration
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
/// use std::collections::HashMap;
///
/// let config = StageConfiguration::default();
/// let mut stage = PipelineStage::new(
///     "transform".to_string(),
///     StageType::Transform,
///     config,
///     0,
/// ).unwrap();
///
/// // Update configuration
/// let mut new_params = HashMap::new();
/// new_params.insert("format".to_string(), "json".to_string());
/// let new_config = StageConfiguration::new("transform".to_string(), new_params, true);
/// stage.update_configuration(new_config);
///
/// assert_eq!(stage.algorithm(), "transform");
/// ```
///
/// ### Stage Compatibility Checking
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
///
/// let compression = PipelineStage::new(
///     "compression".to_string(),
///     StageType::Compression,
///     StageConfiguration::default(),
///     0,
/// ).unwrap();
///
/// let encryption = PipelineStage::new(
///     "encryption".to_string(),
///     StageType::Encryption,
///     StageConfiguration::default(),
///     1,
/// ).unwrap();
///
/// // Compression should come before encryption
/// assert!(compression.is_compatible_with(&encryption));
/// ```
///
/// ### Enabling and Disabling Stages
///
/// ```
/// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
///
/// let mut stage = PipelineStage::new(
///     "checksum".to_string(),
///     StageType::Checksum,
///     StageConfiguration::default(),
///     0,
/// ).unwrap();
///
/// assert!(stage.is_enabled());
///
/// // Disable the stage
/// stage.set_enabled(false);
/// assert!(!stage.is_enabled());
///
/// // Re-enable the stage
/// stage.set_enabled(true);
/// assert!(stage.is_enabled());
/// ```
///
/// ## Stage Compatibility Rules
///
/// The stage compatibility system ensures optimal pipeline performance:
///
/// ### Recommended Ordering
/// 1. **Input Checksum** (automatic)
/// 2. **Compression** (reduces data size)
/// 3. **Encryption** (secures compressed data)
/// 4. **Output Checksum** (automatic)
///
/// ### Compatibility Matrix
/// ```text
/// From \ To      | Compression | Encryption | Checksum | PassThrough
/// ----------------|-------------|------------|----------|------------
/// Compression     | ❌ No       | ✅ Yes     | ✅ Yes   | ✅ Yes
/// Encryption      | ❌ No       | ❌ No      | ✅ Yes   | ✅ Yes
/// Checksum        | ✅ Yes      | ✅ Yes     | ✅ Yes   | ✅ Yes
/// PassThrough     | ✅ Yes      | ✅ Yes     | ✅ Yes   | ✅ Yes
/// ```
///
/// ## Validation and Error Handling
///
/// Stages perform validation during creation and modification:
///
///
/// ## Performance Considerations
///
/// - Stage creation and modification are lightweight operations
/// - Compatibility checking is performed in constant time
/// - Configuration updates only affect the specific stage
/// - Parallel processing settings can significantly impact performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    id: StageId,
    name: String,
    stage_type: StageType,
    configuration: StageConfiguration,
    enabled: bool,
    order: u32,
    #[serde(with = "datetime_serde")]
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl PipelineStage {
    /// Creates a new pipeline stage with the specified configuration
    ///
    /// Constructs a new stage entity with a unique identifier and timestamps.
    /// The stage is created in an enabled state by default.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable stage identifier (must not be empty)
    /// * `stage_type` - Type of processing operation (Compression, Encryption, etc.)
    /// * `configuration` - Algorithm and parameter configuration for the stage
    /// * `order` - Execution order position in the pipeline (0-based)
    ///
    /// # Returns
    ///
    /// * `Ok(PipelineStage)` - Successfully created stage
    /// * `Err(PipelineError::InvalidConfiguration)` - If name is empty
    ///
    /// # Errors
    ///
    /// Returns `InvalidConfiguration` if the stage name is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageType, StageConfiguration};
    /// use std::collections::HashMap;
    ///
    /// // Create a stage successfully
    /// let mut params = HashMap::new();
    /// params.insert("level".to_string(), "9".to_string());
    /// let config = StageConfiguration::new("zstd".to_string(), params, true);
    ///
    /// let stage = PipelineStage::new(
    ///     "my-compression-stage".to_string(),
    ///     StageType::Compression,
    ///     config,
    ///     0,
    /// ).unwrap();
    ///
    /// assert_eq!(stage.name(), "my-compression-stage");
    ///
    /// // Empty name returns an error
    /// let result = PipelineStage::new(
    ///     "".to_string(),
    ///     StageType::Compression,
    ///     StageConfiguration::default(),
    ///     0,
    /// );
    /// assert!(result.is_err());
    /// ```
    pub fn new(
        name: String,
        stage_type: StageType,
        configuration: StageConfiguration,
        order: u32,
    ) -> Result<Self, PipelineError> {
        if name.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage name cannot be empty".to_string(),
            ));
        }

        let now = chrono::Utc::now();

        Ok(PipelineStage {
            id: StageId::new(),
            name,
            stage_type,
            configuration,
            enabled: true,
            order,
            created_at: now,
            updated_at: now,
        })
    }

    /// Gets the unique identifier for this stage
    ///
    /// # Returns
    ///
    /// Reference to the stage's unique identifier
    pub fn id(&self) -> &StageId {
        &self.id
    }

    /// Gets the human-readable name of the stage
    ///
    /// # Returns
    ///
    /// The stage name as a string slice
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the processing operation type for this stage
    ///
    /// # Returns
    ///
    /// Reference to the stage type (Compression, Encryption, Checksum, or PassThrough)
    pub fn stage_type(&self) -> &StageType {
        &self.stage_type
    }

    /// Gets the complete configuration for this stage
    ///
    /// Includes algorithm selection, parameters, and processing options.
    ///
    /// # Returns
    ///
    /// Reference to the stage's configuration
    pub fn configuration(&self) -> &StageConfiguration {
        &self.configuration
    }

    /// Gets the algorithm name from the stage configuration
    ///
    /// Convenience method for accessing the algorithm without going through
    /// the configuration object. Useful for test framework compatibility.
    ///
    /// # Returns
    ///
    /// The algorithm name as a string slice
    pub fn algorithm(&self) -> &str {
        &self.configuration.algorithm
    }

    /// Checks whether the stage is currently enabled for execution
    ///
    /// Disabled stages are skipped during pipeline execution.
    ///
    /// # Returns
    ///
    /// `true` if enabled, `false` if disabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Gets the execution order position of this stage
    ///
    /// Lower numbers execute first. Order determines the sequence of
    /// processing operations in the pipeline.
    ///
    /// # Returns
    ///
    /// The stage's order position (0-based)
    pub fn order(&self) -> u32 {
        self.order
    }

    /// Gets the timestamp when this stage was created
    ///
    /// # Returns
    ///
    /// Reference to the UTC creation timestamp
    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    /// Gets the timestamp of the last modification to this stage
    ///
    /// Updated whenever configuration, enabled state, or order changes.
    ///
    /// # Returns
    ///
    /// Reference to the UTC timestamp of the last update
    pub fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.updated_at
    }

    /// Enables or disables the stage for execution
    ///
    /// Disabled stages are skipped during pipeline execution without being removed.
    /// This allows temporary deactivation while preserving stage configuration.
    ///
    /// # Arguments
    ///
    /// * `enabled` - `true` to enable execution, `false` to disable
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.updated_at = chrono::Utc::now();
    }

    /// Updates the complete stage configuration
    ///
    /// Replaces the entire configuration including algorithm, parameters,
    /// and processing options.
    ///
    /// # Arguments
    ///
    /// * `configuration` - New configuration to apply to the stage
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn update_configuration(&mut self, configuration: StageConfiguration) {
        self.configuration = configuration;
        self.updated_at = chrono::Utc::now();
    }

    /// Updates the execution order position of this stage
    ///
    /// Changes where this stage executes in the pipeline sequence.
    /// Lower order values execute first.
    ///
    /// # Arguments
    ///
    /// * `order` - New order position (0-based)
    ///
    /// # Side Effects
    ///
    /// Updates the `updated_at` timestamp
    pub fn update_order(&mut self, order: u32) {
        self.order = order;
        self.updated_at = chrono::Utc::now();
    }

    /// Checks if this stage is compatible with another stage
    pub fn is_compatible_with(&self, other: &PipelineStage) -> bool {
        match (&self.stage_type, &other.stage_type) {
            // Compression should come before encryption
            (StageType::Compression, StageType::Encryption) => true,

            (StageType::Encryption, StageType::PassThrough) => true,

            // PassThrough stages are compatible with everything
            (StageType::PassThrough, _) => true,
            (_, StageType::PassThrough) => true,

            // Checksum stages are compatible with everything (for verification)
            (StageType::Checksum, _) => true,
            (_, StageType::Checksum) => true,

            // Same type stages are not compatible (avoid duplication)
            (StageType::Compression, StageType::Compression) => false,
            (StageType::Encryption, StageType::Encryption) => false,

            // Other combinations
            _ => true,
        }
    }

    /// Validates the stage configuration
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.name.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage name cannot be empty".to_string(),
            ));
        }

        if self.configuration.algorithm.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage algorithm cannot be empty".to_string(),
            ));
        }

        // Validate chunk size if specified
        if let Some(chunk_size) = self.configuration.chunk_size {
            if !(1024..=100 * 1024 * 1024).contains(&chunk_size) {
                return Err(PipelineError::InvalidConfiguration(
                    "Chunk size must be between 1KB and 100MB".to_string(),
                ));
            }
        }

        Ok(())
    }
}
