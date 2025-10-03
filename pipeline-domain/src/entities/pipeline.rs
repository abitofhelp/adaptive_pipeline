// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Pipeline Entity
//!
//! The `Pipeline` entity represents the core business object for file
//! processing workflows. It orchestrates a sequence of processing stages that
//! transform input files through various operations like compression,
//! encryption, and validation.
//!
//! ## Overview
//!
//! A pipeline is a domain entity with distinct identity that maintains its
//! identity through state changes. It encapsulates the business logic for:
//!
//! - Stage ordering and validation
//! - Processing configuration management
//! - Metrics collection and reporting
//! - Integrity verification through mandatory checksum stages
//!
//! ## Architecture
//!
//! The pipeline follows Domain-Driven Design principles:
//!
//! - **Entity Identity**: Each pipeline has a unique `PipelineId`
//! - **Business Rules**: Enforces stage compatibility and ordering
//! - **Encapsulation**: Internal state is protected through controlled methods
//! - **Repository Support**: Implements traits for both generic and SQLite
//!   repositories
//!
//! ## Automatic Stage Management
//!
//! Pipelines automatically manage integrity verification by inserting mandatory
//! stages:
//!
//! 1. **Input Checksum Stage** (order: 0) - Validates input file integrity
//! 2. **User-Defined Stages** (order: 1, 2, 3...) - Custom processing logic
//! 3. **Output Checksum Stage** (order: final) - Validates output file
//!    integrity
//!
//! This ensures every pipeline maintains data integrity throughout processing.

use crate::entities::{PipelineStage, ProcessingMetrics};
use crate::services::datetime_serde;
use crate::value_objects::PipelineId;
use crate::PipelineError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import for generic repository support
// Note: These traits should be defined in domain/repositories, not infrastructure
// use crate::repositories::RepositoryEntity;
// use crate::repositories::SqliteEntity;

/// Data Transfer Object for reconstituting a Pipeline from database storage.
///
/// This DTO represents the raw database row structure and is used by repository
/// implementations to reconstruct Pipeline entities. It separates database
/// representation from domain logic, following the Repository pattern.
///
/// # Usage
///
/// This DTO is typically created by repository implementations when fetching
/// data from the database, then passed to `Pipeline::from_database()` to
/// create a domain entity.
#[derive(Debug, Clone)]
pub struct PipelineData {
    pub id: PipelineId,
    pub name: String,
    pub archived: bool,
    pub configuration: HashMap<String, String>,
    pub metrics: ProcessingMetrics,
    pub stages: Vec<PipelineStage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Core pipeline entity representing a configurable processing workflow.
///
/// A `Pipeline` is a domain entity that orchestrates file processing through
/// an ordered sequence of stages. Each pipeline has a unique identity and
/// maintains its own configuration, metrics, and processing history.
///
/// ## Entity Characteristics
///
/// - **Identity**: Unique `PipelineId` that persists through all changes
/// - **Mutability**: Can be modified while preserving identity
/// - **Business Logic**: Enforces stage compatibility and ordering rules
/// - **Lifecycle**: Tracks creation and modification timestamps
/// - **Persistence**: Supports both generic and SQLite repository patterns
///
/// ## Automatic Integrity Verification
///
/// Every pipeline automatically includes integrity verification stages:
///
/// ```text
/// [Input Checksum] -> [User Stage 1] -> [User Stage 2] -> [Output Checksum]
///      (order: 0)       (order: 1)       (order: 2)        (order: 3)
/// ```
///
/// This ensures data integrity is maintained throughout the entire processing
/// workflow.
///
/// ## Usage Examples
///
/// ### Creating a New Pipeline
///
/// ```
/// use pipeline_domain::entities::{pipeline::Pipeline, pipeline_stage::PipelineStage};
/// use pipeline_domain::entities::pipeline_stage::{StageType, StageConfiguration};
/// use std::collections::HashMap;
///
/// // Create user-defined stages
/// let compression = PipelineStage::new(
///     "compress".to_string(),
///     StageType::Compression,
///     StageConfiguration::new("zstd".to_string(), HashMap::new(), true),
///     0,
/// ).unwrap();
///
/// let encryption = PipelineStage::new(
///     "encrypt".to_string(),
///     StageType::Encryption,
///     StageConfiguration::new("aes256gcm".to_string(), HashMap::new(), false),
///     1,
/// ).unwrap();
///
/// // Create pipeline (checksum stages added automatically)
/// let pipeline = Pipeline::new(
///     "Secure Backup".to_string(),
///     vec![compression, encryption],
/// ).unwrap();
///
/// assert_eq!(pipeline.name(), "Secure Backup");
/// // Pipeline has 4 stages: input_checksum + 2 user stages + output_checksum
/// assert_eq!(pipeline.stages().len(), 4);
/// ```
///
/// ### Modifying Pipeline Configuration
///
/// ```
/// use pipeline_domain::entities::{pipeline::Pipeline, pipeline_stage::PipelineStage};
/// use pipeline_domain::entities::pipeline_stage::{StageType, StageConfiguration};
/// use std::collections::HashMap;
///
/// let stage = PipelineStage::new(
///     "transform".to_string(),
///     StageType::Transform,
///     StageConfiguration::default(),
///     0,
/// ).unwrap();
///
/// let mut pipeline = Pipeline::new(
///     "Data Pipeline".to_string(),
///     vec![stage],
/// ).unwrap();
///
/// // Add configuration parameters
/// let mut config = HashMap::new();
/// config.insert("output_format".to_string(), "json".to_string());
/// config.insert("compression_level".to_string(), "6".to_string());
/// pipeline.update_configuration(config);
///
/// assert_eq!(pipeline.configuration().get("output_format"), Some(&"json".to_string()));
/// ```
///
/// ### Adding Stages Dynamically
///
/// ```
/// use pipeline_domain::entities::{pipeline::Pipeline, pipeline_stage::PipelineStage};
/// use pipeline_domain::entities::pipeline_stage::{StageType, StageConfiguration};
///
/// let initial_stage = PipelineStage::new(
///     "compress".to_string(),
///     StageType::Compression,
///     StageConfiguration::default(),
///     0,
/// ).unwrap();
///
/// let mut pipeline = Pipeline::new(
///     "Processing Pipeline".to_string(),
///     vec![initial_stage],
/// ).unwrap();
///
/// // Add a new encryption stage
/// let encryption_stage = PipelineStage::new(
///     "encrypt".to_string(),
///     StageType::Encryption,
///     StageConfiguration::default(),
///     0,  // order will be adjusted
/// ).unwrap();
///
/// pipeline.add_stage(encryption_stage).unwrap();
///
/// // Pipeline now has 4 stages: input_checksum + compression + encryption + output_checksum
/// assert_eq!(pipeline.stages().len(), 4);
/// ```
///
///
/// ## Business Rules and Validation
///
/// The pipeline enforces several important business rules:
///
/// ### Stage Compatibility
/// Consecutive stages must be compatible with each other:
///
///
/// ### Minimum Stage Requirement
/// Pipelines must contain at least one stage (including auto-added checksum
/// stages).
///
/// ### Stage Ordering
/// Stages are automatically reordered to maintain proper execution sequence.
///
/// ## Metrics and Monitoring
///
/// Pipelines track processing metrics for performance analysis:
///
///
/// ## Repository Integration
///
/// The pipeline supports multiple repository patterns:
///
/// ### Generic Repository
///
/// ### SQLite Repository
///
/// ## Error Handling
///
/// Pipeline operations return `Result` types with specific error variants:
///
/// - `InvalidConfiguration`: Invalid pipeline setup or parameters
/// - `IncompatibleStage`: Stages cannot be used together
/// - `InvalidInput`: Invalid data provided to pipeline methods
///
/// ## Thread Safety and Concurrency
///
/// While the pipeline entity itself is not thread-safe (following DDD
/// principles), it can be safely shared across threads when wrapped in
/// appropriate synchronization primitives like `Arc<Mutex<Pipeline>>`.
///
/// ## Performance Considerations
///
/// - Stage validation is performed during modification, not during processing
/// - Metrics collection has minimal overhead
/// - Automatic stage insertion occurs only during pipeline creation
/// - Repository operations are optimized for both read and write performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    // Identity fields (always first)
    id: PipelineId,
    name: String,

    // Core business fields (alphabetical within group)
    archived: bool,
    configuration: HashMap<String, String>,
    metrics: ProcessingMetrics,
    stages: Vec<PipelineStage>,

    // Metadata fields (always last)
    #[serde(with = "datetime_serde")]
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Pipeline {
    /// Creates the mandatory input checksum stage
    ///
    /// This stage is automatically prepended to every pipeline to ensure
    /// input file integrity verification.
    fn create_input_checksum_stage() -> Result<PipelineStage, PipelineError> {
        PipelineStage::new(
            "input_checksum".to_string(),
            crate::entities::pipeline_stage::StageType::Checksum,
            crate::entities::pipeline_stage::StageConfiguration::new(
                "sha256".to_string(),
                HashMap::new(),
                false, // not parallel
            ),
            0, // order: first
        )
    }

    /// Creates the mandatory output checksum stage
    ///
    /// This stage is automatically appended to every pipeline to ensure
    /// output file integrity verification.
    ///
    /// # Arguments
    ///
    /// * `order` - The order position for this stage (should be last)
    fn create_output_checksum_stage(order: u32) -> Result<PipelineStage, PipelineError> {
        PipelineStage::new(
            "output_checksum".to_string(),
            crate::entities::pipeline_stage::StageType::Checksum,
            crate::entities::pipeline_stage::StageConfiguration::new(
                "sha256".to_string(),
                HashMap::new(),
                false, // not parallel
            ),
            order, // order: last
        )
    }
    /// Creates a new pipeline with the given name and user-defined stages.
    ///
    /// # Automatic Stage Insertion
    ///
    /// **IMPORTANT**: This constructor automatically inserts mandatory checksum
    /// stages:
    /// - `input_checksum` stage is prepended (order: 0)
    /// - User-defined stages follow (order: 1, 2, 3...)
    /// - `output_checksum` stage is appended (order: final)
    ///
    /// This ensures the database reflects the complete processing pipeline that
    /// actually gets executed, maintaining the "database as single source
    /// of truth" principle.
    ///
    /// # Example
    ///
    /// ```
    /// use pipeline_domain::entities::{pipeline::Pipeline, pipeline_stage::PipelineStage};
    /// use pipeline_domain::entities::pipeline_stage::{StageType, StageConfiguration};
    ///
    /// let compression = PipelineStage::new(
    ///     "compress".to_string(),
    ///     StageType::Compression,
    ///     StageConfiguration::default(),
    ///     0,
    /// ).unwrap();
    ///
    /// let pipeline = Pipeline::new("My Pipeline".to_string(), vec![compression]).unwrap();
    ///
    /// // Verify automatic checksum stages were added
    /// assert_eq!(pipeline.stages().len(), 3); // input_checksum + user stage + output_checksum
    /// assert_eq!(pipeline.name(), "My Pipeline");
    /// ```
    ///
    /// # Arguments
    ///
    /// * `name` - Pipeline name (must not be empty)
    /// * `stages` - User-defined processing stages (must not be empty)
    ///
    /// # Returns
    ///
    /// Returns a `Pipeline` with automatic checksum stages inserted, or
    /// `PipelineError` if validation fails.
    ///
    /// # Errors
    ///
    /// * `InvalidConfiguration` - If name is empty or no user stages provided
    pub fn new(name: String, user_stages: Vec<PipelineStage>) -> Result<Self, PipelineError> {
        if name.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline name cannot be empty".to_string(),
            ));
        }

        if user_stages.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline must have at least one user-defined stage".to_string(),
            ));
        }

        let now = chrono::Utc::now();

        // Calculate total stages before consuming user_stages vector
        let user_stage_count = user_stages.len();

        // Build complete pipeline stages: input_checksum + user_stages +
        // output_checksum
        let mut complete_stages = Vec::with_capacity(user_stage_count + 2);

        // 1. Create and add input_checksum stage (order: 0)
        let input_checksum_stage = Self::create_input_checksum_stage().unwrap();
        complete_stages.push(input_checksum_stage);

        // 2. Add user stages with proper order (starting from 1)
        for (index, stage) in user_stages.into_iter().enumerate() {
            // Create new stage with adjusted order to account for input_checksum at
            // position 0
            let user_stage = PipelineStage::new(
                stage.name().to_string(),
                *stage.stage_type(),
                stage.configuration().clone(),
                (index + 1) as u32, // order: 1, 2, 3...
            )
            .unwrap();
            complete_stages.push(user_stage);
        }

        // 3. Create and add output_checksum stage (order: last)
        let output_checksum_stage = Self::create_output_checksum_stage((user_stage_count + 1) as u32).unwrap();
        complete_stages.push(output_checksum_stage);

        Ok(Pipeline {
            // Identity fields
            id: PipelineId::new(),
            name,

            // Core business fields (alphabetical)
            archived: false,
            configuration: HashMap::new(),
            metrics: ProcessingMetrics::default(),
            stages: complete_stages, // Complete pipeline with mandatory stages

            // Metadata fields
            created_at: now,
            updated_at: now,
        })
    }

    /// Gets the unique identifier for this pipeline
    ///
    /// The pipeline ID is immutable and persists throughout the entity's lifetime.
    /// This ID is used for database lookups, API references, and maintaining
    /// entity identity across system boundaries.
    ///
    /// # Returns
    ///
    /// A reference to the pipeline's unique identifier
    ///
    /// # Examples
    ///
    pub fn id(&self) -> &PipelineId {
        &self.id
    }

    /// Gets the human-readable name of the pipeline
    ///
    /// The name is used for display purposes, logging, and user identification.
    /// Unlike the ID, the name can be duplicated across different pipelines.
    ///
    /// # Returns
    ///
    /// The pipeline name as a string slice
    ///
    /// # Examples
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the ordered list of processing stages in this pipeline
    ///
    /// Returns all stages including automatically-inserted checksum stages.
    /// Stages are returned in execution order (order: 0, 1, 2, ...).
    ///
    /// # Why This Returns a Slice
    ///
    /// Returning a slice (`&[PipelineStage]`) instead of a Vec provides:
    /// - **No cloning**: Efficient access without copying data
    /// - **Immutability**: Prevents external modification of stages
    /// - **Standard interface**: Works with all slice methods and iterators
    ///
    /// # Returns
    ///
    /// An immutable slice containing all pipeline stages in execution order
    ///
    /// # Examples
    ///
    pub fn stages(&self) -> &[PipelineStage] {
        &self.stages
    }

    /// Gets the pipeline configuration parameters
    ///
    /// Configuration parameters are key-value pairs that control pipeline
    /// behavior. Common parameters include:
    /// - `max_workers`: Number of concurrent worker threads
    /// - `chunk_size`: Size of data chunks for processing
    /// - `timeout`: Processing timeout in seconds
    /// - `buffer_size`: I/O buffer size in bytes
    ///
    /// # Returns
    ///
    /// An immutable reference to the configuration HashMap
    ///
    /// # Examples
    ///
    pub fn configuration(&self) -> &HashMap<String, String> {
        &self.configuration
    }

    /// Gets the current processing metrics for this pipeline
    ///
    /// Metrics track performance and execution statistics including:
    /// - Bytes processed and throughput
    /// - Processing duration and timestamps
    /// - Error and warning counts
    /// - Stage-specific metrics
    ///
    /// # Returns
    ///
    /// An immutable reference to the pipeline's metrics
    ///
    /// # Examples
    ///
    pub fn metrics(&self) -> &ProcessingMetrics {
        &self.metrics
    }

    /// Gets the timestamp when this pipeline was created
    ///
    /// The creation timestamp is set when the pipeline entity is first
    /// constructed and never changes. It's useful for auditing, sorting,
    /// and determining pipeline age.
    ///
    /// # Returns
    ///
    /// Reference to the UTC creation timestamp
    ///
    /// # Examples
    ///
    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    /// Gets the timestamp of the last modification to this pipeline
    ///
    /// The updated timestamp changes whenever the pipeline is modified,
    /// including configuration updates, stage additions/removals, or
    /// metrics updates.
    ///
    /// # Why Track Updates?
    ///
    /// Tracking update times enables:
    /// - **Optimistic locking**: Detect concurrent modifications
    /// - **Audit trails**: Know when changes occurred
    /// - **Cache invalidation**: Know when cached data is stale
    /// - **Sorting**: Order pipelines by recency
    ///
    /// # Returns
    ///
    /// Reference to the UTC timestamp of the last update
    ///
    /// # Examples
    ///
    pub fn updated_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.updated_at
    }

    /// Gets the current status of the pipeline
    ///
    /// Returns a simple status string indicating whether the pipeline is
    /// currently active or archived. This is a basic status indicator;
    /// detailed operational status (running, idle, failed) should be
    /// obtained from monitoring systems like Prometheus/Grafana.
    ///
    /// # Why Simple Status?
    ///
    /// This returns a static status because:
    /// - **Domain purity**: Pipeline entity shouldn't know about runtime state
    /// - **Separation of concerns**: Operational status belongs in monitoring
    /// - **Simplicity**: Avoid mixing persistent state with transient state
    ///
    /// # Returns
    ///
    /// - `"Active"` if the pipeline is available for use
    /// - `"Archived"` if the pipeline has been soft-deleted
    ///
    /// # Examples
    ///
    ///
    /// # Note
    ///
    /// For real-time operational status (running, idle, error states),
    /// query your monitoring system (Prometheus/Grafana) instead.
    pub fn status(&self) -> &'static str {
        if self.archived {
            "Archived"
        } else {
            "Active"
        }
    }

    /// Checks if the pipeline is archived (soft-deleted)
    ///
    /// Archived pipelines are not physically deleted but are hidden from
    /// normal queries and cannot be executed. Archiving provides:
    /// - **Reversibility**: Can be restored if needed
    /// - **Audit trail**: Maintains history of deleted pipelines
    /// - **Data integrity**: Preserves foreign key relationships
    ///
    /// # Returns
    ///
    /// - `true` if the pipeline is archived
    /// - `false` if the pipeline is active
    ///
    /// # Examples
    ///
    pub fn archived(&self) -> bool {
        self.archived
    }

    /// Updates the complete pipeline configuration
    ///
    /// Replaces the entire configuration HashMap with new values. Any previous
    /// configuration is discarded. For updating individual keys, retrieve the
    /// configuration, modify it, and pass it back.
    ///
    /// # Why Replace Instead of Merge?
    ///
    /// Complete replacement provides:
    /// - **Clear semantics**: No ambiguity about what happens to old values
    /// - **Simplicity**: No complex merge logic needed
    /// - **Explicit control**: Caller decides exact final state
    /// - **Immutability pattern**: Aligns with functional programming principles
    ///
    /// # Arguments
    ///
    /// * `config` - The new configuration HashMap to set. Common keys include:
    ///   - `max_workers`: Number of worker threads
    ///   - `chunk_size`: Processing chunk size in bytes
    ///   - `timeout`: Timeout in seconds
    ///   - `buffer_size`: I/O buffer size
    ///
    /// # Side Effects
    ///
    /// - Replaces all configuration values
    /// - Updates the `updated_at` timestamp
    ///
    /// # Examples
    ///
    ///
    /// ## Updating Individual Keys
    ///
    pub fn update_configuration(&mut self, config: HashMap<String, String>) {
        self.configuration = config;
        self.updated_at = chrono::Utc::now();
    }

    /// Adds a new processing stage to the pipeline
    ///
    /// Appends a stage to the end of the pipeline's stage sequence. The new stage
    /// must be compatible with the last existing stage according to compatibility
    /// rules (e.g., compression should precede encryption).
    ///
    /// # Why Compatibility Checking?
    ///
    /// Stage compatibility ensures:
    /// - **Correct ordering**: Stages execute in a logical sequence
    /// - **Data integrity**: Each stage can process the previous stage's output
    /// - **Performance**: Optimal stage ordering (compress before encrypt)
    /// - **Correctness**: Prevents invalid combinations (e.g., double compression)
    ///
    /// # Arguments
    ///
    /// * `stage` - The pipeline stage to add. Must be compatible with the current
    ///   last stage.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the stage was added successfully
    /// - `Err(PipelineError::IncompatibleStage)` if stage is incompatible
    ///
    /// # Errors
    ///
    /// Returns `IncompatibleStage` if the new stage is not compatible with
    /// the last stage in the pipeline. For example:
    /// - Attempting to add encryption before compression
    /// - Adding duplicate stage types
    /// - Incompatible algorithm combinations
    ///
    /// # Side Effects
    ///
    /// - Appends stage to the pipeline's stage list
    /// - Updates the `updated_at` timestamp
    ///
    /// # Examples
    ///
    pub fn add_stage(&mut self, stage: PipelineStage) -> Result<(), PipelineError> {
        // Validate stage compatibility
        if let Some(last_stage) = self.stages.last() {
            if !last_stage.is_compatible_with(&stage) {
                return Err(PipelineError::IncompatibleStage(format!(
                    "Stage {} is not compatible with {}",
                    stage.name(),
                    last_stage.name()
                )));
            }
        }

        self.stages.push(stage);
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Removes a processing stage from the pipeline by its index position
    ///
    /// Removes the stage at the specified index and returns it. The pipeline
    /// must always have at least one stage remaining after removal.
    ///
    /// # Why Index-Based Removal?
    ///
    /// Index-based removal provides:
    /// - **Precision**: Remove exact stage by position
    /// - **Simplicity**: No need to search by name or ID
    /// - **Efficiency**: O(n) removal where n is stages after index
    /// - **Flexibility**: Works even with duplicate stage names
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based position of the stage to remove (0 = first stage)
    ///
    /// # Returns
    ///
    /// - `Ok(PipelineStage)` - The removed stage
    /// - `Err(PipelineError)` - If index is invalid or removal would leave pipeline empty
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - `InvalidConfiguration`: Index is out of bounds (>= stage count)
    /// - `InvalidConfiguration`: Removing the last remaining stage
    ///
    /// # Side Effects
    ///
    /// - Removes stage from the pipeline
    /// - Shifts subsequent stages down by one position
    /// - Updates the `updated_at` timestamp
    ///
    /// # Examples
    ///
    pub fn remove_stage(&mut self, index: usize) -> Result<PipelineStage, PipelineError> {
        if index >= self.stages.len() {
            return Err(PipelineError::InvalidConfiguration(
                "Stage index out of bounds".to_string(),
            ));
        }

        if self.stages.len() == 1 {
            return Err(PipelineError::InvalidConfiguration(
                "Cannot remove the last stage".to_string(),
            ));
        }

        self.updated_at = chrono::Utc::now();
        Ok(self.stages.remove(index))
    }

    /// Updates the pipeline's processing metrics with new values
    ///
    /// Replaces the entire metrics object with new performance data. This is
    /// typically called after processing completes to record final statistics.
    ///
    /// # Why Replace Metrics?
    ///
    /// Complete replacement instead of incremental updates provides:
    /// - **Atomicity**: Metrics represent a single processing run
    /// - **Clarity**: No confusion about partial vs. complete metrics
    /// - **Simplicity**: No merge logic needed
    /// - **Immutability**: Aligns with functional programming patterns
    ///
    /// # Arguments
    ///
    /// * `metrics` - New processing metrics to replace current metrics. Should
    ///   contain complete statistics from a processing run including:
    ///   - Bytes processed and throughput
    ///   - Processing duration
    ///   - Error and warning counts
    ///   - Stage-specific metrics
    ///
    /// # Side Effects
    ///
    /// - Replaces current metrics completely
    /// - Updates the `updated_at` timestamp
    ///
    /// # Examples
    ///
    pub fn update_metrics(&mut self, metrics: ProcessingMetrics) {
        self.metrics = metrics;
        self.updated_at = chrono::Utc::now();
    }

    /// Validates the complete pipeline configuration for correctness
    ///
    /// Performs comprehensive validation of the pipeline's configuration including
    /// stage count, stage compatibility, and ordering rules. This should be called
    /// before attempting to execute the pipeline.
    ///
    /// # What is Validated?
    ///
    /// - **Stage Count**: Pipeline must have at least one stage
    /// - **Stage Compatibility**: Each stage must be compatible with the next
    /// - **Stage Ordering**: Stages must be in correct execution order
    /// - **Configuration Completeness**: All required configuration present
    ///
    /// # Why Validate?
    ///
    /// Validation prevents:
    /// - **Runtime errors**: Catch issues before execution starts
    /// - **Data corruption**: Ensure stages can process each other's output
    /// - **Resource waste**: Don't start processing with invalid configuration
    /// - **Poor UX**: Provide clear error messages upfront
    ///
    /// # Returns
    ///
    /// - `Ok(())` if pipeline configuration is valid
    /// - `Err(PipelineError)` with details if validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `InvalidConfiguration`: No stages present in pipeline
    /// - `IncompatibleStage`: Adjacent stages are incompatible
    ///
    /// # Examples
    ///
    ///
    /// # Implementation Note
    ///
    /// Validation uses a sliding window to check pairwise stage compatibility,
    /// which is O(n) where n is the number of stages.
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.stages.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline must have at least one stage".to_string(),
            ));
        }

        // Validate stage sequence
        for window in self.stages.windows(2) {
            if !window[0].is_compatible_with(&window[1]) {
                return Err(PipelineError::IncompatibleStage(format!(
                    "Stages {} and {} are not compatible",
                    window[0].name(),
                    window[1].name()
                )));
            }
        }

        Ok(())
    }

    /// Creates a pipeline from database data (for repository use).
    ///
    /// # Arguments
    ///
    /// * `data` - A `PipelineData` DTO containing all fields from the database
    ///
    /// # Returns
    ///
    /// Returns `Ok(Pipeline)` if the data is valid, or `Err(PipelineError)` if validation fails.
    ///
    /// # Errors
    ///
    /// * `PipelineError::InvalidConfiguration` - If name is empty or no stages provided
    pub fn from_database(data: PipelineData) -> Result<Self, PipelineError> {
        if data.name.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline name cannot be empty".to_string(),
            ));
        }

        if data.stages.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Pipeline must have at least one stage".to_string(),
            ));
        }

        Ok(Pipeline {
            id: data.id,
            name: data.name,
            archived: data.archived,
            configuration: data.configuration,
            metrics: data.metrics,
            stages: data.stages,
            created_at: data.created_at,
            updated_at: data.updated_at,
        })
    }
}

// Implementation for generic repository support
// This allows Pipeline to be used with the generic InMemoryRepository<T>
// TODO: These traits should be defined in domain/repositories, not referenced from infrastructure
/*
impl RepositoryEntity for Pipeline {
    type Id = PipelineId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

// Implementation for SQLite repository support
// This allows Pipeline to be used with the SQLite repository through adapters
impl SqliteEntity for Pipeline {
    type Id = PipelineId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn table_name() -> &'static str {
        "pipelines"
    }

    fn table_schema() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS pipelines (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            archived BOOLEAN NOT NULL DEFAULT false,
            UNIQUE(name)
        )
        "#
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn id_to_string(&self) -> String {
        self.id.to_string()
    }

    fn id_to_string_static(id: &Self::Id) -> String {
        id.to_string()
    }

    fn id_from_string(s: &str) -> Result<Self::Id, PipelineError> {
        PipelineId::from_string(s)
    }
}
*/

/// Helper function to convert PipelineId to Uuid
///
/// This is used primarily for event sourcing where events use Uuid
/// while the domain entities use PipelineId.
pub fn pipeline_id_to_uuid(pipeline_id: &PipelineId) -> uuid::Uuid {
    let ulid = pipeline_id.as_ulid();
    uuid::Uuid::from_u128(ulid.0)
}
