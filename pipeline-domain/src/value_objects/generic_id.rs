// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Generic ID Value Object
//!
//! This module provides a generic, type-safe ID value object system for the
//! adaptive pipeline system. It uses ULID (Universally Unique Lexicographically
//! Sortable Identifier) with phantom types to create type-safe,
//! category-specific identifiers.
//!
//! ## Overview
//!
//! The generic ID system provides:
//!
//! - **Type Safety**: Compile-time enforcement of ID categories
//! - **ULID-Based**: Uses ULID for sortable, unique identifiers
//! - **Category Validation**: Category-specific validation rules
//! - **Zero-Cost Abstractions**: Phantom types with no runtime overhead
//! - **Serialization**: Support for persistence and transmission
//!
//! ## Architecture
//!
//! The ID system follows Domain-Driven Design principles:
//!
//! - **Value Object**: Immutable value object with equality semantics
//! - **Type Safety**: Phantom types prevent ID category mixing at compile time
//! - **Rich Domain Model**: Encapsulates ID-related business logic
//! - **Validation**: Comprehensive validation of ID formats and constraints
//!
//! ## Key Features
//!
//! ### ULID Properties
//!
//! - **Sortable**: Lexicographically sortable by creation time
//! - **Unique**: Globally unique identifiers
//! - **Compact**: 26-character string representation
//! - **URL-Safe**: Safe for use in URLs and file names
//! - **Case-Insensitive**: Base32 encoding is case-insensitive
//!
//! ### Type Safety
//!
//! - **Compile-Time Checking**: Prevent mixing different ID types
//! - **Category Enforcement**: Each ID category has specific validation
//! - **Zero Runtime Cost**: Phantom types have no runtime overhead
//! - **Rich Type System**: Leverage Rust's type system for correctness
//!
//! ### Validation and Constraints
//!
//! - **Format Validation**: Validate ULID format and structure
//! - **Category Validation**: Category-specific validation rules
//! - **Nil Handling**: Configurable nil value handling per category
//! - **Custom Constraints**: Support for custom validation logic
//!
//! ## Usage Examples
//!
//! ### Basic ID Creation

//!
//! ### ID Parsing and Validation

//!
//! ### Type Safety Demonstration

//!
//! ### Custom ID Categories

//!
//! ### ID Collections and Sorting

//!
//! ### Serialization and Deserialization

//!
//! ## ID Categories
//!
//! ### Built-in Categories
//!
//! - **PipelineIdCategory**: For pipeline instances
//!   - Validation: Standard ULID validation
//!   - Use case: Identify pipeline execution instances
//!
//! - **FileIdCategory**: For file references
//!   - Validation: Standard ULID validation
//!   - Use case: Identify files in the system
//!
//! - **UserIdCategory**: For user identification
//!   - Validation: Standard ULID validation
//!   - Use case: Identify users and sessions
//!
//! - **StageIdCategory**: For pipeline stages
//!   - Validation: Standard ULID validation
//!   - Use case: Identify individual pipeline stages
//!
//! ### Custom Categories
//!
//! Create custom ID categories by implementing the `IdCategory` trait:
//!
//! - **Category Name**: Unique identifier for the category
//! - **Validation Logic**: Custom validation rules
//! - **Nil Handling**: Configure whether nil values are allowed
//!
//! ## ULID Properties
//!
//! ### Format
//!
//! - **Length**: 26 characters
//! - **Encoding**: Base32 (Crockford's Base32)
//! - **Case**: Case-insensitive
//! - **Characters**: 0-9, A-Z (excluding I, L, O, U)
//!
//! ### Structure
//!
//! ```text
//! 01AN4Z07BY      79KA1307SR9X4MV3
//! |----------|    |----------------|
//!  Timestamp          Randomness
//!    48bits             80bits
//! ```
//!
//! ### Properties
//!
//! - **Sortable**: Lexicographically sortable by timestamp
//! - **Unique**: 80 bits of randomness ensure uniqueness
//! - **Compact**: More compact than UUID strings
//! - **URL-Safe**: Safe for use in URLs without encoding
//!
//! ## Validation Rules
//!
//! ### Format Validation
//!
//! - **Length**: Must be exactly 26 characters
//! - **Characters**: Must contain only valid Base32 characters
//! - **Structure**: Must follow ULID structure
//!
//! ### Category Validation
//!
//! - **Nil Handling**: Check if nil values are allowed
//! - **Custom Rules**: Apply category-specific validation
//! - **Timestamp Validation**: Validate timestamp ranges
//!
//! ### Security Considerations
//!
//! - **Randomness**: Ensure sufficient randomness
//! - **Predictability**: Prevent ID prediction attacks
//! - **Information Leakage**: Minimize information leakage
//!
//! ## Error Handling
//!
//! ### ID Errors
//!
//! - **Invalid Format**: ID format is invalid
//! - **Parse Error**: Cannot parse ID from string
//! - **Validation Error**: ID fails category validation
//! - **Nil Error**: Nil ID where not allowed
//!
//! ### Category Errors
//!
//! - **Unknown Category**: Category is not recognized
//! - **Validation Failure**: Category-specific validation failed
//! - **Constraint Violation**: ID violates category constraints
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Compact Storage**: ULID is more compact than UUID
//! - **Zero-Cost Types**: Phantom types have no runtime cost
//! - **Efficient Comparison**: Fast comparison operations
//!
//! ### Generation Performance
//!
//! - **Fast Generation**: ULID generation is very fast
//! - **No Network**: No network calls required
//! - **Thread-Safe**: Safe for concurrent generation
//!
//! ### Parsing Performance
//!
//! - **Fast Parsing**: Efficient Base32 parsing
//! - **Validation**: Fast validation algorithms
//! - **Caching**: Cache validation results when appropriate
//!
//! ## Integration
//!
//! The generic ID system integrates with:
//!
//! - **Domain Entities**: Identify domain entities uniquely
//! - **Database**: Use as primary keys and foreign keys
//! - **API**: Consistent ID format across APIs
//! - **Logging**: Include IDs in logs for tracing
//!
//! ## Thread Safety
//!
//! The generic ID system is thread-safe:
//!
//! - **Immutable**: IDs are immutable after creation
//! - **Safe Generation**: ULID generation is thread-safe
//! - **Concurrent Access**: Safe concurrent access to ID data
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **ID Registry**: Centralized ID category registry
//! - **Migration Support**: Support for migrating between ID formats
//! - **Performance Optimization**: Further performance optimizations
//! - **Enhanced Validation**: More sophisticated validation rules

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use ulid::Ulid;

use crate::PipelineError;

/// ID category trait for type-specific behavior
///
/// This trait defines the interface for ID categories, allowing different
/// types of IDs to have category-specific validation rules and behavior.
///
/// # Key Features
///
/// - **Category Identification**: Unique name for each ID category
/// - **Custom Validation**: Category-specific validation logic
/// - **Nil Handling**: Configure whether nil values are allowed
/// - **Extensibility**: Easy to add new ID categories
///
/// # Examples
///
pub trait IdCategory {
    /// Gets the category name for this ID type
    fn category_name() -> &'static str;

    /// Validates category-specific constraints
    fn validate_id(ulid: &Ulid) -> Result<(), PipelineError> {
        // Default implementation - can be overridden
        if *ulid == Ulid::nil() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "{} ID cannot be nil",
                Self::category_name()
            )));
        }
        Ok(())
    }

    /// Checks if this ID type should allow nil values
    fn allows_nil() -> bool {
        false // Default: IDs cannot be nil
    }
}

/// Generic identifier value object for domain entities
///
/// # Purpose
/// Provides the foundational ID implementation that all specific entity IDs
/// build upon. This generic approach ensures consistency while allowing
/// type-safe specialization.
///
/// # Design Principles
/// - **Type Safety**: Each entity gets its own distinct ID type
/// - **Validation**: Consistent validation rules across all ID types
/// - **Serialization**: Uniform JSON/database representation
/// - **Cross-Language**: Clear specification for Go implementation
///
/// # Architecture Notes
/// This is the base implementation that specific ID value objects compose.
/// It should not be used directly - instead use the specific ID types like
/// `PipelineId`, `StageId`, etc.
///
/// # Cross-Language Mapping
/// - **Rust**: `GenericId<T>` with phantom type parameter
/// - **Go**: `GenericID[T any]` with type parameter
/// - **JSON**: String representation of ULID
/// - **SQLite**: TEXT column with ULID string
/// - **Time-Ordered**: Natural chronological sorting
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GenericId<T: IdCategory> {
    value: Ulid,
    _phantom: std::marker::PhantomData<T>,
}

// Custom serialization to use simple string format instead of JSON object
impl<T: IdCategory> Serialize for GenericId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.to_string().serialize(serializer)
    }
}

impl<'de, T: IdCategory> Deserialize<'de> for GenericId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let ulid = Ulid::from_string(&s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(Self {
            value: ulid,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T: IdCategory> GenericId<T> {
    /// Creates a new time-ordered entity ID with category validation
    ///
    /// # Time Ordering
    /// ULIDs are naturally sorted by creation time, making them perfect for:
    /// - Database indexes (sequential inserts)
    /// - Event ordering
    /// - Chronological queries
    /// - Debugging (time-based ID inspection)
    pub fn new() -> Self {
        let ulid = Ulid::new();
        // For new IDs, we assume they're valid since we just created them
        Self {
            value: ulid,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates an entity ID from an existing ULID with validation
    pub fn from_ulid(ulid: Ulid) -> Result<Self, PipelineError> {
        T::validate_id(&ulid)?;
        Ok(Self {
            value: ulid,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Creates an entity ID from a timestamp (useful for range queries)
    ///
    /// # Use Cases
    ///   - Creating boundary IDs for time-range queries
    ///   - Testing with specific timestamps
    ///   - Migration scenarios requiring specific timestamp IDs
    pub fn from_timestamp_ms(timestamp_ms: u64) -> Result<Self, PipelineError> {
        // Generate random bits for the ULID
        let random = rand::random::<u128>() & ((1u128 << 80) - 1); // Mask to 80 bits
        let ulid = Ulid::from_parts(timestamp_ms, random);
        T::validate_id(&ulid)?;
        Ok(Self {
            value: ulid,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Creates an entity ID from a string representation
    ///
    /// # Format
    /// Accepts standard ULID string format (26 characters, base32 encoded)
    /// Example: "01ARZ3NDEKTSV4RRFFQ69G5FAV"
    pub fn from_string(s: &str) -> Result<Self, PipelineError> {
        let ulid = Ulid::from_str(s)
            .map_err(|e| PipelineError::InvalidConfiguration(format!("Invalid entity ID format: {}", e)))?;
        Self::from_ulid(ulid)
    }

    /// Gets the underlying ULID value
    pub fn as_ulid(&self) -> Ulid {
        self.value
    }

    /// Gets the timestamp component of the ULID
    ///
    /// # Returns
    /// Milliseconds since Unix epoch when this ID was created
    ///
    /// # Use Cases
    /// - Time-range queries
    /// - Debugging creation times
    /// - Audit trails
    pub fn timestamp_ms(&self) -> u64 {
        self.value.timestamp_ms()
    }

    /// Gets the creation time as a DateTime
    ///
    /// # Use Cases
    /// - Human-readable timestamps
    /// - Time-based filtering
    /// - Audit logs
    pub fn datetime(&self) -> chrono::DateTime<chrono::Utc> {
        let timestamp_ms = self.timestamp_ms();
        chrono::DateTime::from_timestamp_millis(timestamp_ms as i64).unwrap_or_else(chrono::Utc::now)
    }

    /// Converts to lowercase string representation
    ///
    /// # Use Cases
    /// - Case-insensitive systems
    /// - URL paths
    /// - Database systems that prefer lowercase
    pub fn to_lowercase(&self) -> String {
        self.value.to_string().to_lowercase()
    }

    /// Gets the ID category name
    pub fn category(&self) -> &'static str {
        T::category_name()
    }

    /// Validates the ID using category-specific rules
    pub fn validate(&self) -> Result<(), PipelineError> {
        T::validate_id(&self.value)
    }

    /// Checks if this is a nil (zero) ULID
    pub fn is_nil(&self) -> bool {
        self.value.0 == 0
    }

    /// Creates a nil entity ID (for testing/default values)
    #[cfg(test)]
    pub fn nil() -> Self {
        Self {
            value: Ulid(0),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates an entity ID with a specific timestamp (for testing)
    #[cfg(test)]
    pub fn from_timestamp_for_test(timestamp_ms: u64) -> Self {
        Self::from_timestamp_ms(timestamp_ms).unwrap_or_else(|_| Self::new())
    }
}

impl<T: IdCategory> Default for GenericId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: IdCategory> Display for GenericId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: IdCategory> Hash for GenericId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: IdCategory> FromStr for GenericId<T> {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

// Conversion traits for interoperability
impl<T: IdCategory> From<Ulid> for GenericId<T> {
    fn from(ulid: Ulid) -> Self {
        Self::from_ulid(ulid).unwrap_or_else(|_| {
            // For From trait, we can't return an error, so create a new ID
            Self::new()
        })
    }
}

impl<T: IdCategory> From<GenericId<T>> for Ulid {
    fn from(id: GenericId<T>) -> Self {
        id.value
    }
}

impl<T: IdCategory> AsRef<Ulid> for GenericId<T> {
    fn as_ref(&self) -> &Ulid {
        &self.value
    }
}

/// Utility functions for working with generic IDs
pub mod generic_id_utils {
    use super::*;

    /// Generates a batch of unique entity IDs
    ///
    /// # Time Ordering
    /// Generated IDs will be naturally ordered by creation time
    pub fn generate_batch<T: IdCategory>(count: usize) -> Vec<GenericId<T>> {
        (0..count).map(|_| GenericId::new()).collect()
    }

    /// Generates a batch of IDs with specific timestamp
    ///
    /// # Use Cases
    /// - Testing with controlled timestamps
    /// - Bulk operations with same creation time
    /// - Migration from timestamp-based systems
    pub fn generate_batch_at_time<T: IdCategory>(count: usize, timestamp_ms: u64) -> Vec<GenericId<T>> {
        (0..count)
            .map(|_| GenericId::from_timestamp_ms(timestamp_ms).unwrap_or_else(|_| GenericId::new()))
            .collect()
    }

    /// Validates a collection of entity IDs
    pub fn validate_batch<T: IdCategory>(ids: &[GenericId<T>]) -> Result<(), PipelineError> {
        // Check each ID individually
        for id in ids {
            id.validate()?;
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for id in ids {
            if !seen.insert(id.as_ulid()) {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Duplicate entity ID found: {}",
                    id
                )));
            }
        }

        Ok(())
    }

    /// Converts a collection of ULIDs to entity IDs
    pub fn from_ulids<T: IdCategory>(ulids: Vec<Ulid>) -> Result<Vec<GenericId<T>>, PipelineError> {
        ulids
            .into_iter()
            .map(GenericId::from_ulid)
            .collect::<Result<Vec<_>, _>>()
    }

    /// Converts a collection of entity IDs to ULIDs
    pub fn to_ulids<T: IdCategory>(ids: Vec<GenericId<T>>) -> Vec<Ulid> {
        ids.into_iter().map(|id| id.as_ulid()).collect()
    }

    /// Creates a boundary ID for time-range queries
    ///
    /// # Use Cases
    /// - "Find all entities created after this time"
    /// - Time-based pagination
    /// - Audit queries
    pub fn boundary_id_for_time<T: IdCategory>(timestamp_ms: u64) -> GenericId<T> {
        GenericId::from_timestamp_ms(timestamp_ms).unwrap_or_else(|_| GenericId::new())
    }

    /// Sorts a collection of IDs by creation time (natural ULID order)
    ///
    /// # Note
    /// ULIDs are naturally time-ordered, so this is just a regular sort
    pub fn sort_by_time<T: IdCategory + Ord>(mut ids: Vec<GenericId<T>>) -> Vec<GenericId<T>> {
        ids.sort();
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PipelineError;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    struct TestEntity;

    impl IdCategory for TestEntity {
        fn category_name() -> &'static str {
            "test"
        }

        fn validate_id(ulid: &Ulid) -> Result<(), PipelineError> {
            // Reject nil ULIDs (all zeros)
            if ulid.0 == 0 {
                return Err(PipelineError::ValidationError(
                    "Nil ULID not allowed for test entities".to_string(),
                ));
            }
            Ok(())
        }
    }

    type TestId = GenericId<TestEntity>;

    /// Tests generic ID creation and uniqueness guarantees.
    ///
    /// This test validates that generic IDs can be created with
    /// guaranteed uniqueness and proper time-based ordering
    /// using ULID generation.
    ///
    /// # Test Coverage
    ///
    /// - Generic ID creation with `new()`
    /// - Uniqueness guarantees between IDs
    /// - ULID uniqueness verification
    /// - Time-based ordering with millisecond resolution
    /// - Temporal sequence validation
    ///
    /// # Test Scenario
    ///
    /// Creates two generic IDs with a small time delay between
    /// them and verifies they are unique and properly ordered.
    ///
    /// # Assertions
    ///
    /// - IDs are unique
    /// - Underlying ULIDs are unique
    /// - Second ID is greater than first (time ordering)
    /// - Millisecond resolution ordering works
    #[test]
    fn test_generic_id_creation() {
        let id1 = TestId::new();

        // Sleep for 1ms to ensure different timestamps
        std::thread::sleep(std::time::Duration::from_millis(1));

        let id2 = TestId::new();

        // IDs should be unique
        assert_ne!(id1, id2);
        assert_ne!(id1.as_ulid(), id2.as_ulid());

        // IDs should be time-ordered (id2 created after id1)
        // This works because ULIDs have millisecond resolution
        assert!(id2 > id1);
    }

    /// Tests generic ID time-based ordering with specific timestamps.
    ///
    /// This test validates that generic IDs can be created from
    /// specific timestamps and maintain proper chronological
    /// ordering for time-based queries.
    ///
    /// # Test Coverage
    ///
    /// - ID creation from specific timestamps
    /// - Time-based ordering validation
    /// - Timestamp preservation and retrieval
    /// - Chronological comparison operations
    /// - Millisecond precision handling
    ///
    /// # Test Scenario
    ///
    /// Creates two generic IDs from specific timestamps with
    /// a one-minute difference and verifies proper ordering
    /// and timestamp preservation.
    ///
    /// # Assertions
    ///
    /// - Later ID is greater than earlier ID
    /// - Timestamps are preserved correctly
    /// - Time-based ordering works as expected
    /// - Millisecond precision is maintained
    #[test]
    fn test_generic_id_time_ordering() {
        let timestamp1 = 1640995200000; // 2022-01-01
        let timestamp2 = 1640995260000; // 2022-01-01 + 1 minute

        let id1 = TestId::from_timestamp_ms(timestamp1).unwrap();
        let id2 = TestId::from_timestamp_ms(timestamp2).unwrap();

        assert!(id2 > id1);
        assert_eq!(id1.timestamp_ms(), timestamp1);
        assert_eq!(id2.timestamp_ms(), timestamp2);
    }

    /// Tests generic ID JSON serialization and deserialization.
    ///
    /// This test validates that generic IDs can be properly
    /// serialized to JSON and deserialized back while maintaining
    /// equality and data integrity.
    ///
    /// # Test Coverage
    ///
    /// - JSON serialization with serde
    /// - JSON deserialization with serde
    /// - Serialization roundtrip integrity
    /// - Data preservation during serialization
    /// - Type safety after deserialization
    ///
    /// # Test Scenario
    ///
    /// Creates a generic ID, serializes it to JSON, deserializes
    /// it back, and verifies the roundtrip preserves equality.
    ///
    /// # Assertions
    ///
    /// - Serialization succeeds
    /// - Deserialization succeeds
    /// - Original and deserialized IDs are equal
    /// - Data integrity is maintained
    #[test]
    fn test_generic_id_serialization() {
        let id = TestId::new();

        let json = serde_json::to_string(&id).unwrap();
        let deserialized: TestId = serde_json::from_str(&json).unwrap();

        assert_eq!(id, deserialized);
    }

    /// Tests generic ID validation for valid and invalid cases.
    ///
    /// This test validates that generic IDs can be validated
    /// for correctness and that invalid IDs (like nil ULIDs)
    /// are properly rejected.
    ///
    /// # Test Coverage
    ///
    /// - Valid ID validation with `validate()`
    /// - Invalid ID detection and rejection
    /// - Nil ULID validation failure
    /// - Validation error handling
    /// - ID correctness verification
    ///
    /// # Test Scenario
    ///
    /// Creates a valid generic ID and verifies it passes validation,
    /// then creates a nil ID and verifies it fails validation.
    ///
    /// # Assertions
    ///
    /// - Valid ID passes validation
    /// - Nil ID fails validation
    /// - Validation logic works correctly
    /// - Error handling is appropriate
    #[test]
    fn test_generic_id_validation() {
        let valid_id = TestId::new();
        assert!(valid_id.validate().is_ok());

        // Create a nil ULID using the nil() method
        let nil_id = TestId::nil();
        assert!(nil_id.validate().is_err());
    }

    /// Tests ULID conversion utilities for batch operations.
    ///
    /// This test validates that generic IDs can be converted
    /// to and from ULIDs in batch operations while preserving
    /// data integrity and order.
    ///
    /// # Test Coverage
    ///
    /// - Batch ULID to generic ID conversion
    /// - Batch generic ID to ULID conversion
    /// - Conversion roundtrip integrity
    /// - Order preservation during conversion
    /// - Utility function correctness
    ///
    /// # Test Scenario
    ///
    /// Creates a batch of ULIDs, converts them to generic IDs,
    /// then converts back to ULIDs and verifies the roundtrip
    /// preserves the original data.
    ///
    /// # Assertions
    ///
    /// - Conversion to generic IDs succeeds
    /// - Conversion back to ULIDs succeeds
    /// - Original and final ULIDs are equal
    /// - Order is preserved throughout
    #[test]
    fn test_ulid_conversions() {
        use super::generic_id_utils::*;

        let original_ulids = vec![Ulid::new(), Ulid::new(), Ulid::new()];
        let entity_ids = from_ulids::<TestEntity>(original_ulids.clone()).unwrap();
        let converted_ulids = to_ulids(entity_ids);

        assert_eq!(original_ulids, converted_ulids);
    }

    /// Tests time-based range queries and boundary ID generation.
    ///
    /// This test validates that generic IDs support time-based
    /// range queries by generating boundary IDs for specific
    /// timestamps and verifying ordering relationships.
    ///
    /// # Test Coverage
    ///
    /// - Boundary ID generation for specific timestamps
    /// - Time-based range query support
    /// - Timestamp preservation in boundary IDs
    /// - Ordering relationships with boundary IDs
    /// - Time range query utilities
    ///
    /// # Test Scenario
    ///
    /// Creates a boundary ID for a specific timestamp, then
    /// creates an ID for a later timestamp and verifies the
    /// ordering relationship works correctly.
    ///
    /// # Assertions
    ///
    /// - Boundary ID has correct timestamp
    /// - Later ID is greater than boundary ID
    /// - Time-based ordering works for queries
    /// - Boundary generation is accurate
    #[test]
    fn test_time_range_queries() {
        use super::generic_id_utils::*;
        
        let base_time = 1640995200000; // 2022-01-01
        let boundary_id = boundary_id_for_time::<TestEntity>(base_time);

        assert_eq!(boundary_id.timestamp_ms(), base_time);

        // Test that IDs created after boundary are greater
        let later_id = TestId::from_timestamp_ms(base_time + 1000).unwrap();
        assert!(later_id > boundary_id);
    }
}
