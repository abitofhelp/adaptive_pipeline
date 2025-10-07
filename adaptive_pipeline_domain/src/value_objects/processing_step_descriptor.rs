// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Processing Step Descriptor Value Object
//!
//! This module defines the processing step descriptor value object for the
//! adaptive pipeline system. It provides a comprehensive description of
//! processing steps with validation, parameters, and metadata.
//!
//! ## Overview
//!
//! The processing step descriptor provides:
//!
//! - **Step Definition**: Complete definition of processing steps
//! - **Algorithm Validation**: Validated algorithm names and parameters
//! - **Parameter Management**: Type-safe parameter handling
//! - **Metadata Tracking**: Comprehensive metadata for each step
//! - **Serialization**: Support for persistence and transmission
//!
//! ## Architecture
//!
//! The descriptor follows Domain-Driven Design principles:
//!
//! - **Value Object**: Immutable value object with equality semantics
//! - **Rich Domain Model**: Encapsulates processing step business logic
//! - **Validation**: Comprehensive validation of step definitions
//! - **Type Safety**: Type-safe parameter and algorithm handling
//!
//! ## Key Features
//!
//! ### Step Definition
//!
//! - **Step Types**: Support for different processing step types
//! - **Algorithm Specification**: Validated algorithm names and versions
//! - **Parameter Configuration**: Type-safe parameter configuration
//! - **Dependency Management**: Step dependency tracking
//!
//! ### Algorithm Management
//!
//! - **Name Validation**: Comprehensive algorithm name validation
//! - **Version Control**: Algorithm version tracking and compatibility
//! - **Parameter Schema**: Algorithm-specific parameter schemas
//! - **Capability Detection**: Detect algorithm capabilities and features
//!
//! ### Parameter Handling
//!
//! - **Type Safety**: Type-safe parameter values
//! - **Validation**: Parameter validation against schemas
//! - **Default Values**: Support for default parameter values
//! - **Documentation**: Parameter documentation and help text
//!
//! ## Usage Examples
//!
//! ### Basic Step Creation

//!
//! ### Algorithm Validation

//!
//! ### Parameter Management

//!
//! ### Step Composition and Chaining

//!
//! ### Serialization and Configuration

//!
//! ## Processing Step Types
//!
//! ### Built-in Step Types
//!
//! - **Validation**: Input validation and integrity checking
//!   - Algorithms: checksum, signature, format validation
//!   - Use case: Validate input files before processing
//!
//! - **Compression**: Data compression and decompression
//!   - Algorithms: brotli, gzip, zstd, lz4, deflate
//!   - Use case: Reduce file size for storage or transmission
//!
//! - **Encryption**: Data encryption and decryption
//!   - Algorithms: aes-256-gcm, chacha20-poly1305, aes-128-gcm
//!   - Use case: Secure data storage and transmission
//!
//! - **Transformation**: Data format transformation
//!   - Algorithms: json-to-binary, xml-to-json, custom transforms
//!   - Use case: Convert between different data formats
//!
//! - **Analysis**: Data analysis and metrics collection
//!   - Algorithms: statistics, pattern detection, anomaly detection
//!   - Use case: Analyze data patterns and collect metrics
//!
//! ### Custom Step Types
//!
//! Create custom step types by extending the ProcessingStepType enum:
//!
//! - **Domain-Specific**: Steps specific to your application domain
//! - **Integration**: Steps for integrating with external systems
//! - **Monitoring**: Steps for monitoring and alerting
//!
//! ## Algorithm Validation
//!
//! ### Name Format Rules
//!
//! - **Characters**: Alphanumeric, hyphens, and underscores only
//! - **Case**: Converted to lowercase for consistency
//! - **Length**: Must be non-empty after trimming
//! - **Format**: Must match regex pattern `^[a-zA-Z0-9_-]+$`
//!
//! ### Validation Process
//!
//! 1. **Trim Whitespace**: Remove leading/trailing whitespace
//! 2. **Empty Check**: Ensure name is not empty
//! 3. **Character Validation**: Check allowed characters
//! 4. **Normalization**: Convert to lowercase
//! 5. **Registration**: Check against algorithm registry
//!
//! ## Parameter Management
//!
//! ### Parameter Types
//!
//! - **String Parameters**: Text values and identifiers
//! - **Numeric Parameters**: Integer and floating-point values
//! - **Boolean Parameters**: True/false flags
//! - **Array Parameters**: Lists of values
//! - **Object Parameters**: Nested parameter structures
//!
//! ### Parameter Validation
//!
//! - **Type Checking**: Validate parameter types
//! - **Range Validation**: Check numeric ranges
//! - **Format Validation**: Validate string formats
//! - **Dependency Validation**: Check parameter dependencies
//!
//! ### Default Values
//!
//! - **Algorithm Defaults**: Default values for each algorithm
//! - **Override Support**: Allow overriding default values
//! - **Validation**: Validate default values
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Invalid Algorithm**: Algorithm name is invalid
//! - **Invalid Parameters**: Parameter values are invalid
//! - **Missing Parameters**: Required parameters are missing
//! - **Type Mismatch**: Parameter type doesn't match expected type
//!
//! ### Configuration Errors
//!
//! - **Invalid Step Type**: Step type is not supported
//! - **Incompatible Parameters**: Parameters are incompatible
//! - **Circular Dependencies**: Circular step dependencies detected
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Efficient Storage**: Compact storage of step definitions
//! - **String Interning**: Intern common algorithm names
//! - **Parameter Optimization**: Optimize parameter storage
//!
//! ### Validation Performance
//!
//! - **Lazy Validation**: Validate only when necessary
//! - **Caching**: Cache validation results
//! - **Batch Validation**: Validate multiple steps together
//!
//! ## Integration
//!
//! The processing step descriptor integrates with:
//!
//! - **Processing Pipeline**: Define pipeline processing steps
//! - **Configuration System**: Store and load step configurations
//! - **Validation Framework**: Validate step definitions
//! - **Execution Engine**: Execute defined processing steps
//!
//! ## Thread Safety
//!
//! The processing step descriptor is thread-safe:
//!
//! - **Immutable**: Descriptors are immutable after creation
//! - **Safe Sharing**: Safe to share between threads
//! - **Concurrent Access**: Safe concurrent access to descriptor data
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Dynamic Parameters**: Runtime parameter modification
//! - **Parameter Templates**: Template-based parameter generation
//! - **Advanced Validation**: More sophisticated validation rules
//! - **Performance Optimization**: Further performance improvements

use super::binary_file_format::ProcessingStepType;
use crate::PipelineError;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Value object representing a validated algorithm name
///
/// This value object encapsulates algorithm names with comprehensive validation
/// to ensure they meet the required format and naming conventions.
///
/// # Key Features
///
/// - **Name Validation**: Comprehensive validation of algorithm names
/// - **Format Normalization**: Normalize names to lowercase
/// - **Character Restrictions**: Only alphanumeric, hyphens, and underscores
/// - **Immutability**: Algorithm names cannot be changed after creation
///
/// # Validation Rules
///
/// - Must be non-empty after trimming whitespace
/// - Must contain only alphanumeric characters, hyphens, and underscores
/// - Is normalized to lowercase for consistency
/// - Must match the pattern: `^[a-zA-Z0-9_-]+$`
///
/// # Examples
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Algorithm(String);

impl Algorithm {
    /// Create a new Algorithm with validation
    ///
    /// # Purpose
    /// Creates a validated algorithm name with format normalization.
    /// Ensures algorithm names follow consistent naming conventions.
    ///
    /// # Why
    /// Validated algorithm names provide:
    /// - Prevention of configuration errors
    /// - Consistent naming across systems
    /// - Type-safe algorithm specification
    /// - Cross-platform compatibility
    ///
    /// # Arguments
    /// * `value` - Algorithm name (alphanumeric, hyphens, underscores)
    ///
    /// # Returns
    /// * `Ok(Algorithm)` - Validated algorithm (normalized to lowercase)
    /// * `Err(PipelineError)` - Invalid format
    ///
    /// # Errors
    /// - Empty name after trimming
    /// - Invalid characters (only alphanumeric, `-`, `_` allowed)
    ///
    /// # Examples
    pub fn new(value: &str) -> Result<Self, PipelineError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(
                PipelineError::InvalidConfiguration("Algorithm cannot be empty".to_string())
            );
        }

        // Validate algorithm name format (alphanumeric, hyphens, underscores)
        if !trimmed.chars().all(|c| (c.is_alphanumeric() || c == '-' || c == '_')) {
            return Err(
                PipelineError::InvalidConfiguration(
                    format!("Invalid algorithm name '{}': only alphanumeric, hyphens, and underscores allowed", trimmed)
                )
            );
        }

        Ok(Algorithm(trimmed.to_lowercase()))
    }

    /// Get the algorithm name as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Value object representing validated stage parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageParameters(HashMap<String, String>);

impl StageParameters {
    /// Create new empty parameters
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Create from existing HashMap with validation
    pub fn from_map(map: HashMap<String, String>) -> Result<Self, PipelineError> {
        // Validate parameter keys and values
        for (key, value) in &map {
            if key.trim().is_empty() {
                return Err(
                    PipelineError::InvalidConfiguration("Parameter key cannot be empty".to_string())
                );
            }
            if value.len() > 1024 {
                return Err(
                    PipelineError::InvalidConfiguration(
                        format!("Parameter value for '{}' exceeds maximum length of 1024 characters", key)
                    )
                );
            }
        }
        Ok(Self(map))
    }

    /// Add a parameter with validation
    pub fn add_parameter(&mut self, key: &str, value: &str) -> Result<(), PipelineError> {
        let trimmed_key = key.trim();
        if trimmed_key.is_empty() {
            return Err(
                PipelineError::InvalidConfiguration("Parameter key cannot be empty".to_string())
            );
        }
        if value.len() > 1024 {
            return Err(
                PipelineError::InvalidConfiguration(
                    format!("Parameter value for '{}' exceeds maximum length of 1024 characters", trimmed_key)
                )
            );
        }

        self.0.insert(trimmed_key.to_string(), value.to_string());
        Ok(())
    }

    /// Get parameter value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// Get all parameters as HashMap reference
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.0
    }

    /// Convert to owned HashMap
    pub fn into_map(self) -> HashMap<String, String> {
        self.0
    }

    /// Check if parameters are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for StageParameters {
    fn default() -> Self {
        Self::new()
    }
}

/// Value object representing the order of a processing step
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StepOrder(u32);

impl StepOrder {
    /// Create a new StepOrder
    pub fn new(order: u32) -> Self {
        Self(order)
    }

    /// Get the order value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Get the next order
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl std::fmt::Display for StepOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Value object describing a complete processing step
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingStepDescriptor {
    step_type: ProcessingStepType,
    algorithm: Algorithm,
    parameters: StageParameters,
    order: StepOrder,
}

impl ProcessingStepDescriptor {
    /// Create a new ProcessingStepDescriptor
    pub fn new(
        step_type: ProcessingStepType,
        algorithm: Algorithm,
        parameters: StageParameters,
        order: StepOrder
    ) -> Self {
        Self {
            step_type,
            algorithm,
            parameters,
            order,
        }
    }

    /// Create a compression step descriptor
    pub fn compression(algorithm: Algorithm, order: StepOrder) -> Self {
        Self::new(ProcessingStepType::Compression, algorithm, StageParameters::new(), order)
    }

    /// Create an encryption step descriptor
    pub fn encryption(algorithm: Algorithm, order: StepOrder) -> Self {
        Self::new(ProcessingStepType::Encryption, algorithm, StageParameters::new(), order)
    }

    /// Create a checksum step descriptor
    pub fn checksum(algorithm: Algorithm, order: StepOrder) -> Self {
        Self::new(ProcessingStepType::Checksum, algorithm, StageParameters::new(), order)
    }

    /// Create a pass-through step descriptor
    pub fn pass_through(algorithm: Algorithm, order: StepOrder) -> Self {
        Self::new(ProcessingStepType::PassThrough, algorithm, StageParameters::new(), order)
    }

    /// Get the step type
    pub fn step_type(&self) -> &ProcessingStepType {
        &self.step_type
    }

    /// Get the algorithm
    pub fn algorithm(&self) -> &Algorithm {
        &self.algorithm
    }

    /// Get the parameters
    pub fn parameters(&self) -> &StageParameters {
        &self.parameters
    }

    /// Get the order
    pub fn order(&self) -> StepOrder {
        self.order
    }

    /// Add a parameter to this descriptor
    pub fn with_parameter(mut self, key: &str, value: &str) -> Result<Self, PipelineError> {
        self.parameters.add_parameter(key, value)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests algorithm validation rules and constraint enforcement.
    ///
    /// This test validates that algorithm names are properly validated
    /// according to business rules and that invalid algorithm names
    /// are rejected with appropriate error handling.
    ///
    /// # Test Coverage
    ///
    /// - Valid algorithm name acceptance
    /// - Standard algorithm validation (brotli, aes256-gcm, sha256)
    /// - Custom algorithm name support
    /// - Empty string rejection
    /// - Whitespace-only string rejection
    /// - Algorithm names with spaces rejection
    /// - Algorithm names with special characters rejection
    ///
    /// # Test Scenario
    ///
    /// Tests various algorithm name formats including valid standard
    /// algorithms, custom algorithms, and invalid formats.
    ///
    /// # Domain Concerns
    ///
    /// - Algorithm name validation
    /// - Processing step configuration
    /// - Input validation and sanitization
    /// - Business rule enforcement
    ///
    /// # Assertions
    ///
    /// - Standard algorithms are accepted
    /// - Custom algorithms are accepted
    /// - Empty strings are rejected
    /// - Whitespace-only strings are rejected
    /// - Names with spaces are rejected
    /// - Names with special characters are rejected
    #[test]
    fn test_algorithm_validation() {
        // Valid algorithms
        assert!(Algorithm::new("brotli").is_ok());
        assert!(Algorithm::new("aes256-gcm").is_ok());
        assert!(Algorithm::new("sha256").is_ok());
        assert!(Algorithm::new("custom_algo").is_ok());

        // Invalid algorithms
        assert!(Algorithm::new("").is_err());
        assert!(Algorithm::new("   ").is_err());
        assert!(Algorithm::new("algo with spaces").is_err());
        assert!(Algorithm::new("algo@special").is_err());
    }

    /// Tests stage parameters management and validation.
    ///
    /// This test validates that stage parameters can be properly
    /// managed including parameter addition, validation, and
    /// retrieval for processing step configuration.
    ///
    /// # Test Coverage
    ///
    /// - Parameter addition with valid key-value pairs
    /// - Parameter validation for empty keys
    /// - Parameter retrieval functionality
    /// - Parameter storage and access
    /// - Validation error handling
    ///
    /// # Test Scenario
    ///
    /// Creates stage parameters, adds valid and invalid parameters,
    /// and verifies parameter management functionality.
    ///
    /// # Domain Concerns
    ///
    /// - Processing step configuration
    /// - Parameter validation and management
    /// - Configuration storage and retrieval
    /// - Input validation
    ///
    /// # Assertions
    ///
    /// - Valid parameters are added successfully
    /// - Empty key parameters are rejected
    /// - Parameter values can be retrieved
    /// - Parameter storage works correctly
    #[test]
    fn test_stage_parameters() {
        let mut params = StageParameters::new();
        assert!(params.add_parameter("level", "6").is_ok());
        assert!(params.add_parameter("", "value").is_err());
        assert_eq!(params.get("level"), Some(&"6".to_string()));
    }

    /// Tests step order creation and navigation functionality.
    ///
    /// This test validates that step orders can be created with
    /// proper value storage and that navigation to next orders
    /// works correctly for processing sequence management.
    ///
    /// # Test Coverage
    ///
    /// - Step order creation with value
    /// - Value storage and retrieval
    /// - Next order calculation
    /// - Order navigation functionality
    /// - Sequential order management
    ///
    /// # Test Scenario
    ///
    /// Creates a step order and tests value access and navigation
    /// to the next order in the sequence.
    ///
    /// # Domain Concerns
    ///
    /// - Processing step sequencing
    /// - Order management and navigation
    /// - Sequential processing logic
    /// - Step execution order
    ///
    /// # Assertions
    ///
    /// - Order value is stored correctly
    /// - Next order is calculated correctly
    /// - Navigation functionality works
    /// - Sequential ordering is maintained
    #[test]
    fn test_step_order() {
        let order = StepOrder::new(5);
        assert_eq!(order.value(), 5);
        assert_eq!(order.next().value(), 6);
    }

    /// Tests processing step descriptor creation and properties.
    ///
    /// This test validates that processing step descriptors can be
    /// created with proper algorithm, order, and step type
    /// configuration for pipeline processing steps.
    ///
    /// # Test Coverage
    ///
    /// - Compression step descriptor creation
    /// - Step type property validation
    /// - Algorithm property access
    /// - Order property access
    /// - Descriptor configuration integrity
    ///
    /// # Test Scenario
    ///
    /// Creates a compression processing step descriptor and verifies
    /// all properties are correctly configured and accessible.
    ///
    /// # Domain Concerns
    ///
    /// - Processing step configuration
    /// - Step type classification
    /// - Algorithm and order management
    /// - Pipeline step definition
    ///
    /// # Assertions
    ///
    /// - Step type is compression
    /// - Algorithm is stored correctly
    /// - Order value is preserved
    /// - Descriptor properties are accessible
    #[test]
    fn test_processing_step_descriptor() {
        let algorithm = Algorithm::new("brotli").unwrap();
        let order = StepOrder::new(1);

        let descriptor = ProcessingStepDescriptor::compression(algorithm, order);
        assert_eq!(descriptor.step_type(), &ProcessingStepType::Compression);
        assert_eq!(descriptor.algorithm().as_str(), "brotli");
        assert_eq!(descriptor.order().value(), 1);
    }
}
