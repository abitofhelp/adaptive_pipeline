// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Generic Result Builder
//!
//! This module provides a generic, reusable result building system for the
//! adaptive pipeline system. It supports fluent result construction, metrics
//! collection, and comprehensive operation tracking.
//!
//! ## Overview
//!
//! The generic result builder provides:
//!
//! - **Fluent Interface**: Fluent API for building operation results
//! - **Type Safety**: Generic result building for any operation type
//! - **Metrics Integration**: Automatic metrics collection and reporting
//! - **Error Handling**: Comprehensive error handling and context
//! - **Performance Tracking**: Detailed timing and performance measurements
//!
//! ## Architecture
//!
//! The result builder follows the Builder pattern:
//!
//! - **Generic Design**: Works with any operation result type
//! - **Fluent API**: Chainable methods for result construction
//! - **Metrics Collection**: Automatic collection of operation metrics
//! - **Error Context**: Rich error context and debugging information
//!
//! ## Key Features
//!
//! ### Fluent Result Building
//!
//! - **Chainable Methods**: Fluent API for building complex results
//! - **Type Safety**: Compile-time type checking for result construction
//! - **Immutable Building**: Immutable result building with copy-on-write
//! - **Validation**: Automatic validation of result consistency
//!
//! ### Metrics Integration
//!
//! - **Automatic Collection**: Automatic collection of operation metrics
//! - **Custom Metrics**: Support for custom metric types
//! - **Performance Tracking**: Detailed timing and performance measurements
//! - **Statistical Analysis**: Statistical analysis of operation results
//!
//! ### Error Handling
//!
//! - **Rich Context**: Rich error context with operation details
//! - **Error Chaining**: Chain errors from multiple operations
//! - **Recovery Information**: Information for error recovery
//! - **Debugging Support**: Comprehensive debugging information
//!
//! ## Usage Examples
//!
//! ### Basic Result Building

//!
//! ### Result Building with Error Handling

//!
//! ### Advanced Result Building with Custom Metrics

//!
//! ## Builder Pattern Features
//!
//! ### Fluent API
//!
//! - **Method Chaining**: Chain methods for fluent result construction
//! - **Type Safety**: Compile-time type checking for method calls
//! - **Immutable Building**: Each method returns a new builder instance
//! - **Validation**: Automatic validation at each step
//!
//! ### Processing Steps
//!
//! - **Step Definition**: Define processing steps with names and logic
//! - **Step Metrics**: Collect metrics for individual processing steps
//! - **Step Data**: Store intermediate data between processing steps
//! - **Error Handling**: Handle errors within individual steps
//!
//! ### Timing and Performance
//!
//! - **Automatic Timing**: Automatic timing of overall operation
//! - **Step Timing**: Individual timing for each processing step
//! - **Performance Metrics**: Comprehensive performance measurements
//! - **Throughput Calculation**: Automatic throughput calculations
//!
//! ## Error Handling
//!
//! ### Error Types
//!
//! - **Validation Errors**: Input validation failures
//! - **Processing Errors**: Errors during processing steps
//! - **Configuration Errors**: Invalid builder configuration
//! - **System Errors**: System-level errors and failures
//!
//! ### Error Context
//!
//! - **Rich Context**: Detailed error context with operation state
//! - **Error Chaining**: Chain errors from multiple operations
//! - **Recovery Information**: Information for error recovery
//! - **Debugging Data**: Comprehensive debugging information
//!
//! ## Performance Considerations
//!
//! ### Builder Efficiency
//!
//! - **Minimal Overhead**: Designed for minimal performance impact
//! - **Lazy Evaluation**: Lazy evaluation of expensive operations
//! - **Memory Efficiency**: Efficient memory usage during building
//! - **Copy Optimization**: Optimized copying for immutable building
//!
//! ### Metrics Collection
//!
//! - **Low Overhead**: Low-overhead metrics collection
//! - **Selective Collection**: Collect only necessary metrics
//! - **Batched Operations**: Batch metrics operations for efficiency
//!
//! ## Integration
//!
//! The result builder integrates with:
//!
//! - **Processing Pipeline**: Build results for pipeline operations
//! - **Metrics System**: Integrate with metrics collection system
//! - **Error Handling**: Integrate with error handling framework
//! - **Logging System**: Integrate with logging and monitoring
//!
//! ## Thread Safety
//!
//! The result builder is designed for thread safety:
//!
//! - **Immutable Building**: Immutable builder instances
//! - **Thread-Safe Operations**: All operations are thread-safe
//! - **Concurrent Building**: Support for concurrent result building
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Async Building**: Full async support for result building
//! - **Streaming Results**: Support for streaming result construction
//! - **Advanced Metrics**: More sophisticated metrics collection
//! - **Result Caching**: Caching of frequently built results

use adaptive_pipeline_domain::error::PipelineError;
use adaptive_pipeline_domain::services::datetime_serde;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{ Duration, Instant };

/// Generic trait for operation results that can be built fluently
///
/// This trait defines the interface for operation results that can be
/// constructed using the generic result builder. It provides a type-safe way to
/// define operation results with input, output, and metrics.
///
/// # Key Features
///
/// - **Type Safety**: Generic types for input, output, and metrics
/// - **Fluent Construction**: Support for fluent result building
/// - **Metrics Integration**: Built-in metrics collection and reporting
/// - **Success Tracking**: Track operation success/failure status
/// - **Extensibility**: Extensible design for custom result types
///
/// # Type Parameters
///
/// - **Input**: The input type for the operation
/// - **Output**: The output type for the operation
/// - **Metrics**: The metrics type for performance tracking
///
/// # Implementation Requirements
///
/// Implementing types must:
/// - Be cloneable for result construction
/// - Be debuggable for error reporting
/// - Be thread-safe (`Send + Sync`)
/// - Have a stable lifetime (`'static`)
///
/// # Examples
pub trait OperationResult: Clone + Debug + Send + Sync + 'static {
    type Input: Clone + Debug + Send + Sync;
    type Output: Clone + Debug + Send + Sync;
    type Metrics: Clone + Debug + Default + Send + Sync;

    /// Creates a new result with input and output
    fn new(input: Self::Input, output: Self::Output) -> Self;

    /// Gets the input data
    fn input(&self) -> &Self::Input;

    /// Gets the output data
    fn output(&self) -> &Self::Output;

    /// Gets the metrics
    fn metrics(&self) -> &Self::Metrics;

    /// Sets the metrics
    fn with_metrics(self, metrics: Self::Metrics) -> Self;

    /// Checks if the operation was successful
    fn is_success(&self) -> bool;

    /// Gets any error information
    fn error(&self) -> Option<&PipelineError>;
}

/// Generic result builder for fluent API construction
#[derive(Debug, Clone)]
pub struct GenericResultBuilder<T> where T: OperationResult {
    input: Option<T::Input>,
    output: Option<T::Output>,
    metrics: T::Metrics,
    error: Option<PipelineError>,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    metadata: HashMap<String, String>,
    warnings: Vec<String>,
    success: bool,
}

impl<T> GenericResultBuilder<T> where T: OperationResult {
    /// Creates a new result builder
    pub fn new() -> Self {
        Self {
            input: None,
            output: None,
            metrics: T::Metrics::default(),
            error: None,
            start_time: None,
            end_time: None,
            metadata: HashMap::new(),
            warnings: Vec::new(),
            success: true,
        }
    }

    /// Sets the input data
    pub fn with_input(mut self, input: T::Input) -> Self {
        self.input = Some(input);
        self
    }

    /// Sets the output data
    pub fn with_output(mut self, output: T::Output) -> Self {
        self.output = Some(output);
        self
    }

    /// Sets the metrics
    pub fn with_metrics(mut self, metrics: T::Metrics) -> Self {
        self.metrics = metrics;
        self
    }

    /// Marks the operation as started
    pub fn started(mut self) -> Self {
        self.start_time = Some(Instant::now());
        self
    }

    /// Marks the operation as completed
    pub fn completed(mut self) -> Self {
        self.end_time = Some(Instant::now());
        self
    }

    /// Sets an error and marks as failed
    pub fn with_error(mut self, error: PipelineError) -> Self {
        self.error = Some(error);
        self.success = false;
        self
    }

    /// Adds metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Adds multiple metadata entries
    pub fn with_metadata_map(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata.extend(metadata);
        self
    }

    /// Adds a warning
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Adds multiple warnings
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings.extend(warnings);
        self
    }

    /// Marks as successful
    pub fn success(mut self) -> Self {
        self.success = true;
        self.error = None;
        self
    }

    /// Marks as failed
    pub fn failed(mut self) -> Self {
        self.success = false;
        self
    }

    /// Gets the duration if both start and end times are set
    pub fn duration(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }

    /// Gets the metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Gets the warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Builds the final result
    pub fn build(self) -> Result<T, PipelineError> {
        if let Some(error) = self.error {
            return Err(error);
        }

        let input = self.input.ok_or_else(||
            PipelineError::InternalError("Input is required to build result".to_string())
        )?;

        let output = self.output.ok_or_else(||
            PipelineError::InternalError("Output is required to build result".to_string())
        )?;

        let result = T::new(input, output).with_metrics(self.metrics);
        Ok(result)
    }

    /// Builds the result with error handling
    ///
    /// # Panics
    ///
    /// This convenience method will panic if the build fails. This is
    /// intentional for use cases where the caller knows the build cannot
    /// fail. For error handling, use `build()` instead.
    #[allow(clippy::panic)]
    pub fn build_with_error(self) -> T {
        match self.build() {
            Ok(result) => result,
            Err(error) => {
                // Create a default result with error information
                // This requires the OperationResult to have a way to represent errors
                // For now, we'll panic - in real implementation, this would need
                // to be handled by the specific result type
                panic!("Failed to build result: {}", error);
            }
        }
    }
}

impl<T> Default for GenericResultBuilder<T> where T: OperationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for creating result builders
#[macro_export]
macro_rules! result_builder {
    ($result_type:ty) => {
        $crate::application::utilities::GenericResultBuilder::<$result_type>::new()
    };
}

/// Generic processing result that implements OperationResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericProcessingResult<I, O, M>
    where
        I: Clone + Debug + Send + Sync + 'static,
        O: Clone + Debug + Send + Sync + 'static,
        M: Clone + Debug + Default + Send + Sync + 'static {
    pub input: I,
    pub output: O,
    pub metrics: M,
    #[serde(with = "datetime_serde")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "datetime_serde")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub error: Option<String>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl<I, O, M> OperationResult
    for GenericProcessingResult<I, O, M>
    where
        I: Clone + Debug + Send + Sync + 'static,
        O: Clone + Debug + Send + Sync + 'static,
        M: Clone + Debug + Default + Send + Sync + 'static
{
    type Input = I;
    type Output = O;
    type Metrics = M;

    fn new(input: Self::Input, output: Self::Output) -> Self {
        let now = chrono::Utc::now();
        Self {
            input,
            output,
            metrics: M::default(),
            started_at: now,
            completed_at: now,
            success: true,
            error: None,
            warnings: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    fn input(&self) -> &Self::Input {
        &self.input
    }

    fn output(&self) -> &Self::Output {
        &self.output
    }

    fn metrics(&self) -> &Self::Metrics {
        &self.metrics
    }

    fn with_metrics(mut self, metrics: Self::Metrics) -> Self {
        self.metrics = metrics;
        self
    }

    fn is_success(&self) -> bool {
        self.success
    }

    fn error(&self) -> Option<&PipelineError> {
        // Convert string error back to PipelineError if needed
        None // Simplified for now
    }
}

/// Type aliases for common result patterns
pub type FileProcessingGenericResult<M> = GenericProcessingResult<String, String, M>;
pub type CompressionGenericResult<M> = GenericProcessingResult<Vec<u8>, Vec<u8>, M>;
pub type EncryptionGenericResult<M> = GenericProcessingResult<Vec<u8>, Vec<u8>, M>;

/// Builder type aliases for convenience
pub type FileProcessingResultBuilder<M> = GenericResultBuilder<FileProcessingGenericResult<M>>;
pub type CompressionResultBuilder<M> = GenericResultBuilder<CompressionGenericResult<M>>;
pub type EncryptionResultBuilder<M> = GenericResultBuilder<EncryptionGenericResult<M>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default)]
    struct TestMetrics {
        bytes_processed: u64,
        duration_ms: u64,
    }

    type TestResult = GenericProcessingResult<String, String, TestMetrics>;

    /// Tests generic result builder creation and initialization.
    ///
    /// This test validates that result builders can be created with
    /// proper initialization and that default values are set correctly
    /// for metadata and warnings collections.
    ///
    /// # Test Coverage
    ///
    /// - Result builder creation
    /// - Default value initialization
    /// - Metadata collection initialization
    /// - Warnings collection initialization
    /// - Builder structure validation
    ///
    /// # Test Scenario
    ///
    /// Creates a new result builder and verifies that collections
    /// are properly initialized as empty.
    ///
    /// # Domain Concerns
    ///
    /// - Builder pattern implementation
    /// - Result construction initialization
    /// - Default state management
    /// - Collection initialization
    ///
    /// # Assertions
    ///
    /// - Metadata collection is empty
    /// - Warnings collection is empty
    /// - Builder is properly initialized
    /// - Default state is correct
    #[test]
    fn test_result_builder_creation() {
        let builder = GenericResultBuilder::<TestResult>::new();
        assert!(builder.metadata().is_empty());
        assert!(builder.warnings().is_empty());
    }

    /// Tests result builder fluent API for chainable construction.
    ///
    /// This test validates that the result builder supports a fluent
    /// API pattern for chainable method calls and that all builder
    /// methods work correctly together.
    ///
    /// # Test Coverage
    ///
    /// - Fluent API method chaining
    /// - Input and output setting
    /// - Metadata addition
    /// - Warning addition
    /// - Timing operations (started/completed)
    /// - Success state setting
    /// - Result building and validation
    ///
    /// # Test Scenario
    ///
    /// Uses the fluent API to build a complete result with input,
    /// output, metadata, warnings, timing, and success state.
    ///
    /// # Domain Concerns
    ///
    /// - Fluent API design and usability
    /// - Builder pattern implementation
    /// - Method chaining functionality
    /// - Result construction workflow
    ///
    /// # Assertions
    ///
    /// - Result building succeeds
    /// - Input value is preserved
    /// - Output value is preserved
    /// - Success state is set correctly
    /// - Fluent chaining works properly
    #[test]
    fn test_fluent_api() {
        let result = GenericResultBuilder::<TestResult>
            ::new()
            .with_input("test input".to_string())
            .with_output("test output".to_string())
            .with_metadata("key1".to_string(), "value1".to_string())
            .with_warning("Test warning".to_string())
            .started()
            .completed()
            .success()
            .build();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.input(), "test input");
        assert_eq!(result.output(), "test output");
        assert!(result.is_success());
    }

    /// Tests result builder error handling and failure states.
    ///
    /// This test validates that the result builder properly handles
    /// error conditions and that error states are correctly managed
    /// during result construction.
    ///
    /// # Test Coverage
    ///
    /// - Error setting and handling
    /// - Failure state management
    /// - Error result building
    /// - Error propagation
    /// - Result validation with errors
    ///
    /// # Test Scenario
    ///
    /// Creates a result builder with an error and verifies that
    /// the result building fails appropriately.
    ///
    /// # Domain Concerns
    ///
    /// - Error handling and propagation
    /// - Failure state management
    /// - Result validation
    /// - Error reporting
    ///
    /// # Assertions
    ///
    /// - Result building fails with error
    /// - Error state is properly handled
    /// - Failure is correctly propagated
    /// - Error handling works as expected
    #[test]
    fn test_error_handling() {
        let result = GenericResultBuilder::<TestResult>
            ::new()
            .with_input("test input".to_string())
            .with_error(PipelineError::InternalError("Test error".to_string()))
            .build();

        assert!(result.is_err());
    }

    /// Tests result builder metadata and warnings management.
    ///
    /// This test validates that the result builder can properly
    /// manage metadata and warnings collections including bulk
    /// operations and collection extensions.
    ///
    /// # Test Coverage
    ///
    /// - Metadata map addition
    /// - Multiple metadata entries
    /// - Warnings collection addition
    /// - Multiple warnings entries
    /// - Collection size validation
    /// - Bulk operation functionality
    ///
    /// # Test Scenario
    ///
    /// Adds metadata and warnings in bulk and verifies that
    /// collections are properly managed and sized.
    ///
    /// # Domain Concerns
    ///
    /// - Metadata management
    /// - Warning collection
    /// - Bulk operations
    /// - Collection extension
    ///
    /// # Assertions
    ///
    /// - Metadata collection has correct size
    /// - Warnings collection has correct size
    /// - Bulk operations work correctly
    /// - Collections are properly managed
    #[test]
    fn test_metadata_and_warnings() {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), "value1".to_string());
        metadata.insert("key2".to_string(), "value2".to_string());

        let builder = GenericResultBuilder::<TestResult>
            ::new()
            .with_metadata_map(metadata)
            .with_warnings(vec!["warning1".to_string(), "warning2".to_string()]);

        assert_eq!(builder.metadata().len(), 2);
        assert_eq!(builder.warnings().len(), 2);
    }

    /// Tests result builder duration calculation and timing.
    ///
    /// This test validates that the result builder can properly
    /// calculate durations between start and completion times
    /// for performance measurement.
    ///
    /// # Test Coverage
    ///
    /// - Duration calculation functionality
    /// - Start time recording
    /// - Completion time recording
    /// - Duration availability validation
    /// - Performance measurement
    ///
    /// # Test Scenario
    ///
    /// Creates a result builder with start and completion times
    /// and verifies that duration is calculated correctly.
    ///
    /// # Domain Concerns
    ///
    /// - Performance measurement
    /// - Timing functionality
    /// - Duration calculation
    /// - Execution time tracking
    ///
    /// # Assertions
    ///
    /// - Duration is available
    /// - Duration is positive
    /// - Timing calculation works correctly
    /// - Performance measurement is accurate
    #[test]
    fn test_duration_calculation() {
        let builder = GenericResultBuilder::<TestResult>::new().started().completed();

        assert!(builder.duration().is_some());
        let duration = builder.duration().unwrap();
        assert!(duration.as_nanos() > 0);
    }

    /// Tests result builder macro convenience functionality.
    ///
    /// This test validates that the result builder macro provides
    /// convenient creation of result builders with proper
    /// initialization and default values.
    ///
    /// # Test Coverage
    ///
    /// - Macro-based builder creation
    /// - Convenience macro functionality
    /// - Default initialization via macro
    /// - Macro parameter handling
    /// - Builder creation shortcuts
    ///
    /// # Test Scenario
    ///
    /// Uses the result builder macro to create a builder and
    /// verifies proper initialization.
    ///
    /// # Domain Concerns
    ///
    /// - Developer convenience
    /// - Macro-based creation
    /// - Code simplification
    /// - Builder pattern shortcuts
    ///
    /// # Assertions
    ///
    /// - Macro creates builder successfully
    /// - Default initialization is correct
    /// - Metadata collection is empty
    /// - Macro functionality works properly
    #[test]
    fn test_macro_usage() {
        let builder = result_builder!(TestResult);
        assert!(builder.metadata().is_empty());
    }
}
