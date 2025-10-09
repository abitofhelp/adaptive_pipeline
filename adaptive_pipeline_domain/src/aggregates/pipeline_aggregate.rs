// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! Example UUID conversion:

use crate::entities::pipeline::pipeline_id_to_uuid;
use crate::events::{
    PipelineCreatedEvent, PipelineUpdatedEvent, ProcessingCompletedEvent, ProcessingFailedEvent, ProcessingStartedEvent,
};
use crate::{Pipeline, PipelineError, PipelineEvent, ProcessingContext, ProcessingMetrics, SecurityContext};
use std::collections::HashMap;
use uuid::Uuid;
///
/// ### Managing Processing Operations
///
///
/// ### Error Handling and Recovery
///
///
/// ### Pipeline Configuration Updates
///
///
/// ## Event Management
///
/// The aggregate generates domain events for all significant state changes:
///
/// - `PipelineCreated`: When a new pipeline is created
/// - `PipelineUpdated`: When pipeline configuration changes
/// - `ProcessingStarted`: When file processing begins
/// - `ProcessingCompleted`: When processing finishes successfully
/// - `ProcessingFailed`: When processing encounters errors
///
/// ## Concurrency and Thread Safety
///
/// The aggregate is designed for single-threaded access within a transaction
/// boundary. Concurrent access should be managed through:
///
/// - Repository-level locking mechanisms
/// - Optimistic concurrency control using version numbers
/// - Event store transaction boundaries
/// - Application-level coordination
///
/// ## Performance Considerations
///
/// - **Memory Usage**: Scales with number of active processing contexts
/// - **Event Storage**: Uncommitted events held in memory until persistence
/// - **Validation Overhead**: All operations include business rule validation
/// - **Version Tracking**: Minimal overhead for optimistic concurrency control
///
/// ## Error Recovery
///
/// The aggregate provides robust error handling:
///
/// - **Validation Errors**: Prevent invalid state transitions
/// - **Processing Failures**: Tracked with detailed error information
/// - **Event Application**: Supports replay for crash recovery
/// - **State Consistency**: Maintains valid state even during failures
#[derive(Debug, Clone)]
pub struct PipelineAggregate {
    pipeline: Pipeline,
    version: u64,
    uncommitted_events: Vec<PipelineEvent>,
    active_processing_contexts: HashMap<Uuid, ProcessingContext>,
}

impl PipelineAggregate {
    /// Creates a new pipeline aggregate
    pub fn new(pipeline: Pipeline) -> Result<Self, PipelineError> {
        pipeline.validate()?;

        let mut aggregate = Self {
            pipeline: pipeline.clone(),
            version: 1,
            uncommitted_events: Vec::new(),
            active_processing_contexts: HashMap::new(),
        };

        // Raise pipeline created event
        let event = PipelineCreatedEvent::new(
            pipeline_id_to_uuid(pipeline.id()),
            pipeline.name().to_string(),
            pipeline.stages().len(),
            None, // TODO: Get from security context
        );
        aggregate.add_event(PipelineEvent::PipelineCreated(event));

        Ok(aggregate)
    }

    /// Loads aggregate from events (event sourcing)
    pub fn from_events(events: Vec<PipelineEvent>) -> Result<Self, PipelineError> {
        if events.is_empty() {
            return Err(PipelineError::InvalidConfiguration("No events provided".to_string()));
        }

        // Find the first PipelineCreated event to initialize the aggregate
        let created_event = events
            .iter()
            .find_map(|e| match e {
                PipelineEvent::PipelineCreated(event) => Some(event),
                _ => None,
            })
            .ok_or_else(|| PipelineError::InvalidConfiguration("No PipelineCreated event found".to_string()))?;

        // Create initial pipeline (this would normally be reconstructed from events)
        let pipeline = Pipeline::new(
            created_event.pipeline_name.clone(),
            Vec::new(), // Stages would be reconstructed from events
        )?;

        let mut aggregate = Self {
            pipeline,
            version: 0,
            uncommitted_events: Vec::new(),
            active_processing_contexts: HashMap::new(),
        };

        // Apply all events to reconstruct state
        for event in events {
            aggregate.apply_event(&event)?;
        }

        Ok(aggregate)
    }

    /// Gets the pipeline
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Gets the aggregate version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Gets uncommitted events
    pub fn uncommitted_events(&self) -> &[PipelineEvent] {
        &self.uncommitted_events
    }

    /// Marks events as committed
    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    /// Updates the pipeline configuration
    pub fn update_pipeline(&mut self, updated_pipeline: Pipeline) -> Result<(), PipelineError> {
        updated_pipeline.validate()?;

        // Track changes
        let mut changes = Vec::new();
        if self.pipeline.name() != updated_pipeline.name() {
            changes.push(format!(
                "Name changed from '{}' to '{}'",
                self.pipeline.name(),
                updated_pipeline.name()
            ));
        }
        if self.pipeline.stages().len() != updated_pipeline.stages().len() {
            changes.push(format!(
                "Stage count changed from {} to {}",
                self.pipeline.stages().len(),
                updated_pipeline.stages().len()
            ));
        }

        self.pipeline = updated_pipeline;

        // Raise pipeline updated event
        let event = PipelineUpdatedEvent {
            event_id: Uuid::new_v4(),
            pipeline_id: pipeline_id_to_uuid(self.pipeline.id()),
            changes,
            updated_by: None, // TODO: Get from security context
            occurred_at: chrono::Utc::now(),
            version: self.version + 1,
        };
        self.add_event(PipelineEvent::PipelineUpdated(event));

        Ok(())
    }

    /// Starts processing a file
    pub fn start_processing(
        &mut self,
        input_path: String,
        output_path: String,
        file_size: u64,
        security_context: SecurityContext,
    ) -> Result<Uuid, PipelineError> {
        // Validate security context
        security_context.validate()?;

        // Create processing context
        let processing_id = Uuid::new_v4();
        let context = ProcessingContext::new(
            file_size,
            security_context.clone(),
        );

        self.active_processing_contexts.insert(processing_id, context);

        // Raise processing started event
        let event = ProcessingStartedEvent::new(
            pipeline_id_to_uuid(self.pipeline.id()),
            processing_id,
            input_path,
            output_path,
            file_size,
            security_context,
        );
        self.add_event(PipelineEvent::ProcessingStarted(event));

        Ok(processing_id)
    }

    /// Completes processing
    pub fn complete_processing(
        &mut self,
        processing_id: Uuid,
        metrics: ProcessingMetrics,
        output_size: u64,
    ) -> Result<(), PipelineError> {
        if !self.active_processing_contexts.contains_key(&processing_id) {
            return Err(PipelineError::InvalidConfiguration(
                "Processing context not found".to_string(),
            ));
        }

        // Remove from active contexts
        self.active_processing_contexts.remove(&processing_id);

        // Raise processing completed event
        let event = ProcessingCompletedEvent::new(
            pipeline_id_to_uuid(self.pipeline.id()),
            processing_id,
            metrics,
            output_size,
        );
        self.add_event(PipelineEvent::ProcessingCompleted(event));

        Ok(())
    }

    /// Fails processing
    pub fn fail_processing(
        &mut self,
        processing_id: Uuid,
        error_message: String,
        error_code: String,
        stage_name: Option<String>,
        partial_metrics: Option<ProcessingMetrics>,
    ) -> Result<(), PipelineError> {
        if !self.active_processing_contexts.contains_key(&processing_id) {
            return Err(PipelineError::InvalidConfiguration(
                "Processing context not found".to_string(),
            ));
        }

        // Remove from active contexts
        self.active_processing_contexts.remove(&processing_id);

        // Raise processing failed event
        let event = ProcessingFailedEvent {
            event_id: Uuid::new_v4(),
            pipeline_id: pipeline_id_to_uuid(self.pipeline.id()),
            processing_id,
            error_message,
            error_code,
            stage_name,
            partial_metrics,
            occurred_at: chrono::Utc::now(),
            version: self.version + 1,
        };
        self.add_event(PipelineEvent::ProcessingFailed(event));

        Ok(())
    }

    /// Gets active processing contexts
    pub fn active_processing_contexts(&self) -> &HashMap<Uuid, ProcessingContext> {
        &self.active_processing_contexts
    }

    /// Gets a specific processing context
    pub fn get_processing_context(&self, processing_id: Uuid) -> Option<&ProcessingContext> {
        self.active_processing_contexts.get(&processing_id)
    }

    /// Updates a processing context
    pub fn update_processing_context(
        &mut self,
        processing_id: Uuid,
        context: ProcessingContext,
    ) -> Result<(), PipelineError> {
        if !self.active_processing_contexts.contains_key(&processing_id) {
            return Err(PipelineError::InvalidConfiguration(
                "Processing context not found".to_string(),
            ));
        }

        self.active_processing_contexts.insert(processing_id, context);
        Ok(())
    }

    /// Validates the aggregate state
    pub fn validate(&self) -> Result<(), PipelineError> {
        self.pipeline.validate()?;

        // Validate all active processing contexts
        for context in self.active_processing_contexts.values() {
            context.security_context().validate()?;
        }

        Ok(())
    }

    /// Adds an event to uncommitted events
    fn add_event(&mut self, event: PipelineEvent) {
        self.version += 1;
        self.uncommitted_events.push(event);
    }

    /// Applies an event to the aggregate state
    fn apply_event(&mut self, event: &PipelineEvent) -> Result<(), PipelineError> {
        match event {
            PipelineEvent::PipelineCreated(_) => {
                // Pipeline already created in constructor
                self.version += 1;
            }
            PipelineEvent::PipelineUpdated(_) => {
                // Pipeline updates would be applied here
                self.version += 1;
            }
            PipelineEvent::ProcessingStarted(event) => {
                let context = ProcessingContext::new(
                    event.file_size,
                    event.security_context.clone(),
                );
                self.active_processing_contexts.insert(event.processing_id, context);
                self.version += 1;
            }
            PipelineEvent::ProcessingCompleted(event) => {
                self.active_processing_contexts.remove(&event.processing_id);
                self.version += 1;
            }
            PipelineEvent::ProcessingFailed(event) => {
                self.active_processing_contexts.remove(&event.processing_id);
                self.version += 1;
            }
            _ => {
                // Handle other events as needed
                self.version += 1;
            }
        }

        Ok(())
    }

    /// Gets the aggregate ID
    pub fn id(&self) -> Uuid {
        pipeline_id_to_uuid(self.pipeline.id())
    }

    /// Checks if the aggregate has uncommitted events
    pub fn has_uncommitted_events(&self) -> bool {
        !self.uncommitted_events.is_empty()
    }

    /// Gets the number of active processing contexts
    pub fn active_processing_count(&self) -> usize {
        self.active_processing_contexts.len()
    }

    /// Checks if processing is active
    pub fn is_processing_active(&self) -> bool {
        !self.active_processing_contexts.is_empty()
    }
}
