// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Pipeline Domain Events
//!
//! This module defines domain events for the pipeline processing system,
//! implementing event-driven architecture patterns for decoupled communication
//! and event sourcing.
//!
//! ## Overview
//!
//! Pipeline events capture significant business occurrences within the pipeline
//! processing domain. These events enable:
//!
//! - **Event Sourcing**: Reconstruct aggregate state from event history
//! - **Integration**: Communicate with external systems and bounded contexts
//! - **Audit Trail**: Track all significant operations for compliance
//! - **Monitoring**: Real-time system observability and alerting
//! - **Workflow Coordination**: Trigger downstream processes and reactions
//!
//! ## Event Categories
//!
//! ### Pipeline Lifecycle Events
//! Events related to pipeline management:
//! - `PipelineCreated`: New pipeline definition created
//! - `PipelineUpdated`: Pipeline configuration modified
//! - `PipelineDeleted`: Pipeline removed from system
//!
//! ### Processing Events
//! Events during file processing operations:
//! - `ProcessingStarted`: File processing initiated
//! - `ProcessingCompleted`: Processing finished successfully
//! - `ProcessingFailed`: Processing encountered errors
//! - `ProcessingPaused`: Processing temporarily suspended
//! - `ProcessingResumed`: Processing continued from pause
//! - `ProcessingCancelled`: Processing terminated by user
//!
//! ### Stage Events
//! Events for individual processing stages:
//! - `StageStarted`: Processing stage began execution
//! - `StageCompleted`: Stage finished successfully
//! - `StageFailed`: Stage encountered errors
//!
//! ### Operational Events
//! System and operational events:
//! - `ChunkProcessed`: Individual data chunk processed
//! - `MetricsUpdated`: Performance metrics updated
//! - `SecurityViolation`: Security policy violation detected
//! - `ResourceExhausted`: System resource limits reached
//!
//! ## Event Structure
//!
//! All events follow a consistent structure:
//!
//!
//! ## Usage Examples
//!
//! ### Creating Pipeline Events
//!
//!
//! ### Processing Events
//!
//!
//! ### Security Events
//!
//!
//! ## Event Sourcing Integration
//!
//! Events are designed for event sourcing patterns:
//!
//!
//! ## Serialization and Persistence
//!
//! All events support JSON serialization for persistence:
//!
//!
//! ## Event Versioning
//!
//! Events include version information for schema evolution:
//!
//! - **Version 1**: Initial event schema
//! - **Future Versions**: Backward-compatible schema changes
//! - **Migration**: Automatic handling of version differences
//!
//! ## Best Practices
//!
//! ### Event Design
//! - **Immutable**: Events should never be modified after creation
//! - **Complete**: Include all necessary information for event handlers
//! - **Focused**: Each event should represent a single business occurrence
//! - **Timestamped**: Always include accurate occurrence timestamps
//!
//! ### Event Handling
//! - **Idempotent**: Event handlers should be safe to replay
//! - **Atomic**: Handle events in atomic operations where possible
//! - **Resilient**: Handle missing or corrupted events gracefully
//! - **Ordered**: Process events in chronological order when sequence matters
//!
//! ### Performance Considerations
//! - **Batching**: Process multiple events in batches for efficiency
//! - **Async Processing**: Use asynchronous handlers for non-blocking
//!   operations
//! - **Partitioning**: Partition events by aggregate ID for parallel processing
//! - **Compression**: Compress event payloads for storage efficiency
//!
//! ## Error Handling
//!
//! Event processing includes comprehensive error handling:
//!
//! - **Validation**: Events are validated before persistence
//! - **Retry Logic**: Failed event processing can be retried
//! - **Dead Letter Queue**: Persistently failing events are quarantined
//! - **Monitoring**: Event processing failures trigger alerts
//!
//! ## Integration Patterns
//!
//! Events enable various integration patterns:
//!
//! - **Publish-Subscribe**: Broadcast events to multiple subscribers
//! - **Event Streaming**: Real-time event processing pipelines
//! - **Saga Orchestration**: Coordinate complex multi-step processes
//! - **CQRS**: Separate read and write models with event synchronization

use crate::services::datetime_serde;
use crate::{ProcessingMetrics, SecurityContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain events for pipeline processing operations
///
/// This enum represents all possible events that can occur within the pipeline
/// processing domain. Each variant contains a specific event type with its
/// associated data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    PipelineCreated(PipelineCreatedEvent),
    PipelineUpdated(PipelineUpdatedEvent),
    PipelineDeleted(PipelineDeletedEvent),
    ProcessingStarted(ProcessingStartedEvent),
    ProcessingCompleted(ProcessingCompletedEvent),
    ProcessingFailed(ProcessingFailedEvent),
    ProcessingPaused(ProcessingPausedEvent),
    ProcessingResumed(ProcessingResumedEvent),
    ProcessingCancelled(ProcessingCancelledEvent),
    StageStarted(StageStartedEvent),
    StageCompleted(StageCompletedEvent),
    StageFailed(StageFailedEvent),
    ChunkProcessed(ChunkProcessedEvent),
    MetricsUpdated(MetricsUpdatedEvent),
    SecurityViolation(SecurityViolationEvent),
    ResourceExhausted(ResourceExhaustedEvent),
}

/// Base event trait
pub trait DomainEvent {
    fn event_id(&self) -> Uuid;
    fn aggregate_id(&self) -> Uuid;
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn version(&self) -> u64;
}

/// Pipeline created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineCreatedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub pipeline_name: String,
    pub stage_count: usize,
    pub created_by: Option<String>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Pipeline updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineUpdatedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub changes: Vec<String>,
    pub updated_by: Option<String>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Pipeline deleted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDeletedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub deleted_by: Option<String>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing started event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStartedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub input_path: String,
    pub output_path: String,
    pub file_size: u64,
    pub security_context: SecurityContext,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing completed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingCompletedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub metrics: ProcessingMetrics,
    pub output_size: u64,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing failed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingFailedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub error_message: String,
    pub error_code: String,
    pub stage_name: Option<String>,
    pub partial_metrics: Option<ProcessingMetrics>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing paused event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingPausedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub reason: String,
    pub checkpoint_data: Option<Vec<u8>>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing resumed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResumedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub resumed_from_checkpoint: bool,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Processing cancelled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingCancelledEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub reason: String,
    pub cancelled_by: Option<String>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Stage started event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageStartedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub stage_id: Uuid,
    pub stage_name: String,
    pub stage_type: String,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Stage completed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageCompletedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub stage_id: Uuid,
    pub stage_name: String,
    pub processing_time_ms: u64,
    pub bytes_processed: u64,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Stage failed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageFailedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub stage_id: Uuid,
    pub stage_name: String,
    pub error_message: String,
    pub error_code: String,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Chunk processed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkProcessedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub chunk_id: Uuid,
    pub chunk_sequence: u64,
    pub chunk_size: usize,
    pub stage_name: String,
    pub processing_time_ms: u64,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Metrics updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsUpdatedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub metrics: ProcessingMetrics,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Security violation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityViolationEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Option<Uuid>,
    pub violation_type: String,
    pub description: String,
    pub severity: SecurityViolationSeverity,
    pub user_id: Option<String>,
    pub source_ip: Option<String>,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Resource exhausted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceExhaustedEvent {
    pub event_id: Uuid,
    pub pipeline_id: Uuid,
    pub processing_id: Uuid,
    pub resource_type: String,
    pub current_usage: u64,
    pub limit: u64,
    pub action_taken: String,
    #[serde(with = "datetime_serde")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

/// Security violation severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// Implement DomainEvent for all events
impl DomainEvent for PipelineCreatedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn aggregate_id(&self) -> Uuid {
        self.pipeline_id
    }
    fn event_type(&self) -> &'static str {
        "PipelineCreated"
    }
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
    fn version(&self) -> u64 {
        self.version
    }
}

impl DomainEvent for ProcessingStartedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn aggregate_id(&self) -> Uuid {
        self.pipeline_id
    }
    fn event_type(&self) -> &'static str {
        "ProcessingStarted"
    }
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
    fn version(&self) -> u64 {
        self.version
    }
}

impl DomainEvent for ProcessingCompletedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn aggregate_id(&self) -> Uuid {
        self.pipeline_id
    }
    fn event_type(&self) -> &'static str {
        "ProcessingCompleted"
    }
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
    fn version(&self) -> u64 {
        self.version
    }
}

// Factory functions for creating events
impl PipelineCreatedEvent {
    pub fn new(pipeline_id: Uuid, pipeline_name: String, stage_count: usize, created_by: Option<String>) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            pipeline_id,
            pipeline_name,
            stage_count,
            created_by,
            occurred_at: chrono::Utc::now(),
            version: 1,
        }
    }
}

impl ProcessingStartedEvent {
    pub fn new(
        pipeline_id: Uuid,
        processing_id: Uuid,
        input_path: String,
        output_path: String,
        file_size: u64,
        security_context: SecurityContext,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            pipeline_id,
            processing_id,
            input_path,
            output_path,
            file_size,
            security_context,
            occurred_at: chrono::Utc::now(),
            version: 1,
        }
    }
}

impl ProcessingCompletedEvent {
    pub fn new(pipeline_id: Uuid, processing_id: Uuid, metrics: ProcessingMetrics, output_size: u64) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            pipeline_id,
            processing_id,
            metrics,
            output_size,
            occurred_at: chrono::Utc::now(),
            version: 1,
        }
    }
}

impl SecurityViolationEvent {
    pub fn new(
        pipeline_id: Uuid,
        violation_type: String,
        description: String,
        severity: SecurityViolationSeverity,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            pipeline_id,
            processing_id: None,
            violation_type,
            description,
            severity,
            user_id: None,
            source_ip: None,
            occurred_at: chrono::Utc::now(),
            version: 1,
        }
    }
}
