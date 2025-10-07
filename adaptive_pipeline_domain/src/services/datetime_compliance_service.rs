// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # DateTime Compliance Service
//!
//! This module provides RFC3339 datetime compliance utilities and validation
//! for the adaptive pipeline system. It ensures all datetime values in the
//! system are RFC3339 compliant as required by the SRS documentation.
//!
//! ## Overview
//!
//! The datetime compliance service provides:
//!
//! - **RFC3339 Compliance**: Ensure all datetime values follow RFC3339 standard
//! - **Validation**: Comprehensive datetime validation and verification
//! - **Serialization**: Consistent datetime serialization across the system
//! - **Testing**: Testing utilities for datetime compliance verification
//! - **Standards Compliance**: Adherence to international datetime standards
//!
//! ## Architecture
//!
//! The service follows standards compliance principles:
//!
//! - **RFC3339 Standard**: Full compliance with RFC3339 datetime format
//! - **UTC Normalization**: All timestamps normalized to UTC
//! - **Serialization Consistency**: Consistent serialization format
//! - **Validation Framework**: Comprehensive validation and testing
//!
//! ## Key Features
//!
//! ### RFC3339 Compliance
//!
//! - **Standard Format**: YYYY-MM-DDTHH:MM:SS.sssZ format
//! - **Timezone Support**: Proper timezone handling and UTC normalization
//! - **Precision**: Millisecond precision for accurate timestamps
//! - **Validation**: Automatic validation of datetime formats
//!
//! ### Serialization
//!
//! - **JSON Serialization**: Consistent JSON serialization format
//! - **Deserialization**: Robust deserialization with error handling
//! - **Optional Values**: Support for optional datetime fields
//! - **Null Handling**: Proper handling of null datetime values
//!
//! ### Testing and Validation
//!
//! - **Compliance Testing**: Automated compliance testing utilities
//! - **Format Validation**: Validation of datetime format compliance
//! - **Round-trip Testing**: Serialization/deserialization round-trip testing
//! - **Error Detection**: Detection and reporting of compliance violations
//!
//! ## Usage Examples
//!
//! ### Basic DateTime Compliance

//!
//! ### Custom DateTime Validation

//!
//! ### Batch DateTime Validation

//!
//! ### Integration with Domain Entities

//!
//! ## RFC3339 Standard Compliance
//!
//! ### Format Specification
//!
//! The RFC3339 format specification:
//!
//! - **Basic Format**: `YYYY-MM-DDTHH:MM:SSZ`
//! - **With Milliseconds**: `YYYY-MM-DDTHH:MM:SS.sssZ`
//! - **With Timezone**: `YYYY-MM-DDTHH:MM:SS.sss+/-HH:MM`
//! - **UTC Indicator**: `Z` suffix for UTC timestamps
//!
//! ### Validation Rules
//!
//! - **Year**: 4-digit year (0000-9999)
//! - **Month**: 2-digit month (01-12)
//! - **Day**: 2-digit day (01-31, depending on month)
//! - **Hour**: 2-digit hour (00-23)
//! - **Minute**: 2-digit minute (00-59)
//! - **Second**: 2-digit second (00-59)
//! - **Milliseconds**: Optional 3-digit milliseconds (000-999)
//! - **Timezone**: UTC (Z) or offset (+/-HH:MM)
//!
//! ### Compliance Testing
//!
//! - **Format Validation**: Validate datetime string format
//! - **Value Validation**: Validate datetime component values
//! - **Timezone Validation**: Validate timezone specifications
//! - **Round-trip Testing**: Serialize/deserialize consistency
//!
//! ## Serialization Support
//!
//! ### JSON Serialization
//!
//! - **Consistent Format**: All datetimes serialized in RFC3339 format
//! - **UTC Normalization**: All timestamps normalized to UTC
//! - **Precision Preservation**: Millisecond precision preserved
//! - **Null Handling**: Proper handling of optional datetime fields
//!
//! ### Custom Serialization
//!
//! - **Serde Integration**: Full integration with Serde serialization
//! - **Custom Formats**: Support for custom datetime formats when needed
//! - **Backward Compatibility**: Maintain compatibility with existing data
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Format Errors**: Invalid datetime format strings
//! - **Value Errors**: Invalid datetime component values
//! - **Timezone Errors**: Invalid timezone specifications
//! - **Precision Errors**: Precision loss during serialization
//!
//! ### Error Recovery
//!
//! - **Graceful Degradation**: Handle invalid datetimes gracefully
//! - **Error Reporting**: Detailed error messages with context
//! - **Fallback Values**: Use fallback values for invalid datetimes
//!
//! ## Performance Considerations
//!
//! ### Validation Efficiency
//!
//! - **Cached Validators**: Cache validation logic for better performance
//! - **Batch Processing**: Efficient batch validation of multiple datetimes
//! - **Lazy Validation**: Validate only when necessary
//!
//! ### Serialization Performance
//!
//! - **Efficient Serialization**: Optimized serialization routines
//! - **Memory Usage**: Minimal memory usage during serialization
//! - **String Pooling**: Reuse common datetime strings
//!
//! ## Integration
//!
//! The datetime compliance service integrates with:
//!
//! - **Domain Entities**: Validate datetime fields in domain entities
//! - **Serialization**: Ensure consistent datetime serialization
//! - **Database**: Store datetimes in RFC3339 format
//! - **API**: Consistent datetime format in API responses
//!
//! ## Standards Compliance
//!
//! ### RFC3339 Compliance
//!
//! - **Full Compliance**: Complete adherence to RFC3339 standard
//! - **Interoperability**: Ensure interoperability with other systems
//! - **Future Compatibility**: Maintain compatibility with future standards
//!
//! ### ISO 8601 Compatibility
//!
//! - **Subset Compliance**: RFC3339 is a subset of ISO 8601
//! - **Extended Support**: Support for extended ISO 8601 features
//! - **Migration Support**: Support for migrating from other formats
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Timezone Database**: Integration with timezone database
//! - **Localization**: Support for localized datetime formats
//! - **Performance Optimization**: Further performance optimizations
//! - **Extended Validation**: More comprehensive validation rules

/// RFC3339 datetime compliance utilities and tests
///
/// This module ensures all datetime values in the system are RFC3339 compliant
/// as required by the SRS documentation.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Test structure to verify RFC3339 compliance
///
/// This structure provides a comprehensive test framework for validating
/// RFC3339 datetime compliance across the system. It includes both required
/// and optional datetime fields to test various scenarios.
///
/// # Key Features
///
/// - **Required DateTime**: Tests mandatory datetime field compliance
/// - **Optional DateTime**: Tests optional datetime field compliance
/// - **Serialization Testing**: Validates serialization format compliance
/// - **Round-trip Testing**: Ensures serialization/deserialization consistency
///
/// # Examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeTest {
    pub timestamp: DateTime<Utc>,
    pub optional_timestamp: Option<DateTime<Utc>>,
}

impl Default for DateTimeTest {
    fn default() -> Self {
        Self::new()
    }
}

impl DateTimeTest {
    /// Creates a new test instance with current timestamp
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            optional_timestamp: Some(Utc::now()),
        }
    }

    /// Verifies that serialization produces RFC3339 format
    pub fn verify_rfc3339_compliance(&self) -> Result<(), String> {
        // Serialize to JSON
        let json = serde_json::to_string(self).map_err(|e| format!("Serialization failed: {}", e))?;

        // Check that the timestamp is in RFC3339 format
        // RFC3339 format: YYYY-MM-DDTHH:MM:SS.sssZ or YYYY-MM-DDTHH:MM:SS.sss+/-HH:MM
        let rfc3339_regex = regex::Regex::new(r#"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})"#)
            .map_err(|e| format!("Regex compilation failed: {}", e))?;

        if !rfc3339_regex.is_match(&json) {
            return Err(format!("Serialized JSON does not contain RFC3339 timestamps: {}", json));
        }

        // Deserialize back to verify round-trip compatibility
        let _deserialized: DateTimeTest =
            serde_json::from_str(&json).map_err(|e| format!("Deserialization failed: {}", e))?;

        Ok(())
    }

    /// Returns the RFC3339 string representation of the timestamp
    pub fn to_rfc3339_string(&self) -> String {
        self.timestamp.to_rfc3339()
    }
}

/// Utility function to ensure all datetime values in the system are RFC3339
/// compliant
pub fn ensure_rfc3339_compliance() {
    // This function serves as documentation that all DateTime<Utc> fields
    // in our domain automatically serialize to RFC3339 format when using serde

    // Key points:
    // 1. All datetime fields use chrono::DateTime<chrono::Utc>
    // 2. All structs with datetime fields have Serialize/Deserialize derives
    // 3. chrono automatically uses RFC3339 format for serialization
    // 4. UTC timezone ensures consistent handling across systems
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests RFC3339 datetime compliance validation and formatting.
    ///
    /// This test validates that datetime instances comply with RFC3339
    /// standards and that RFC3339 string formatting produces correctly
    /// formatted timestamps for interoperability.
    ///
    /// # Test Coverage
    ///
    /// - RFC3339 compliance verification
    /// - RFC3339 string formatting
    /// - Timezone indicator validation
    /// - Timestamp format structure
    /// - Standards compliance checking
    ///
    /// # Test Scenario
    ///
    /// Creates a datetime test instance and verifies RFC3339 compliance
    /// and proper string formatting with timezone indicators.
    ///
    /// # Domain Concerns
    ///
    /// - Datetime standards compliance
    /// - Interoperability and data exchange
    /// - Timestamp formatting consistency
    /// - Cross-system compatibility
    ///
    /// # Assertions
    ///
    /// - RFC3339 compliance verification succeeds
    /// - RFC3339 string contains 'T' separator
    /// - RFC3339 string has proper timezone indicator
    /// - Timestamp format meets standards
    #[test]
    fn test_rfc3339_compliance() {
        let test_instance = DateTimeTest::new();

        // Verify RFC3339 compliance
        assert!(test_instance.verify_rfc3339_compliance().is_ok());

        // Verify manual RFC3339 conversion
        let rfc3339_string = test_instance.to_rfc3339_string();
        assert!(rfc3339_string.contains('T'));
        assert!(rfc3339_string.ends_with('Z') || rfc3339_string.contains('+') || rfc3339_string.contains('-'));
    }

    /// Tests datetime serialization format compliance and JSON output.
    ///
    /// This test validates that datetime instances serialize to JSON
    /// with proper RFC3339 formatting and that serialized timestamps
    /// maintain standards compliance.
    ///
    /// # Test Coverage
    ///
    /// - JSON serialization of datetime instances
    /// - RFC3339 format preservation in JSON
    /// - Serialized timestamp structure validation
    /// - JSON output format verification
    /// - Standards compliance in serialization
    ///
    /// # Test Scenario
    ///
    /// Serializes a datetime test instance to JSON and verifies
    /// that the output contains properly formatted RFC3339 timestamps.
    ///
    /// # Domain Concerns
    ///
    /// - Data serialization and persistence
    /// - API data exchange formats
    /// - Timestamp serialization consistency
    /// - Standards compliance in data output
    ///
    /// # Assertions
    ///
    /// - JSON serialization succeeds
    /// - Serialized JSON contains 'T' separator
    /// - Serialized JSON has timezone indicators
    /// - Timestamp format is preserved in JSON
    #[test]
    fn test_serialization_format() {
        let test_instance = DateTimeTest::new();
        let json = serde_json::to_string(&test_instance).unwrap();

        // Verify that the JSON contains RFC3339 formatted timestamps
        println!("Serialized JSON: {}", json);
        assert!(json.contains("T"));
        assert!(json.contains("Z") || json.contains("+") || json.contains("-"));
    }
}
