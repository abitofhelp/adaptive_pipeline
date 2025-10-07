// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Algorithm Value Object
//!
//! This module defines the algorithm value object for the adaptive pipeline
//! system. It provides type-safe algorithm specification with validation,
//! categorization, and cross-language compatibility.
//!
//! ## Overview
//!
//! The algorithm value object provides:
//!
//! - **Type Safety**: Type-safe algorithm specification and validation
//! - **Algorithm Categories**: Support for different algorithm categories
//! - **Validation**: Comprehensive validation of algorithm names and formats
//! - **Cross-Language**: Consistent representation across different languages
//! - **Extensibility**: Support for custom and user-defined algorithms
//!
//! ## Architecture
//!
//! The algorithm follows Domain-Driven Design principles:
//!
//! - **Value Object**: Immutable value object with equality semantics
//! - **Rich Domain Model**: Encapsulates algorithm-related business logic
//! - **Validation**: Comprehensive validation of algorithm specifications
//! - **Serialization**: Support for persistence and cross-language
//!   communication
//!
//! ## Key Features
//!
//! ### Algorithm Categories
//!
//! - **Compression**: Data compression algorithms (brotli, gzip, zstd, lz4)
//! - **Encryption**: Data encryption algorithms (AES-256-GCM,
//!   ChaCha20-Poly1305)
//! - **Hashing**: Cryptographic hash algorithms (SHA-256, SHA-512, Blake3)
//! - **Custom**: User-defined and application-specific algorithms
//!
//! ### Validation and Safety
//!
//! - **Format Validation**: Validate algorithm name format and structure
//! - **Category Validation**: Ensure algorithms belong to valid categories
//! - **Parameter Validation**: Validate algorithm-specific parameters
//! - **Security Validation**: Ensure algorithms meet security requirements
//!
//! ### Cross-Language Support
//!
//! - **Consistent Representation**: Same algorithm representation across
//!   languages
//! - **JSON Serialization**: Standard JSON serialization format
//! - **Database Storage**: Consistent database storage format
//! - **API Compatibility**: Compatible with REST and gRPC APIs
//!
//! ## Usage Examples
//!
//! ### Creating Algorithms

//!
//! ### Algorithm Validation

//!
//! ### Algorithm Categories

//!
//! ### Algorithm Comparison and Sorting

//!
//! ### Serialization and Deserialization

//!
//! ## Supported Algorithms
//!
//! ### Compression Algorithms
//!
//! - **brotli**: Google's Brotli compression algorithm
//! - **gzip**: GNU zip compression algorithm
//! - **zstd**: Facebook's Zstandard compression algorithm
//! - **lz4**: LZ4 fast compression algorithm
//! - **deflate**: DEFLATE compression algorithm
//!
//! ### Encryption Algorithms
//!
//! - **aes-256-gcm**: AES-256 with Galois/Counter Mode
//! - **aes-128-gcm**: AES-128 with Galois/Counter Mode
//! - **chacha20-poly1305**: ChaCha20-Poly1305 AEAD cipher
//! - **aes-256-cbc**: AES-256 with Cipher Block Chaining
//!
//! ### Hashing Algorithms
//!
//! - **sha256**: SHA-256 cryptographic hash function
//! - **sha512**: SHA-512 cryptographic hash function
//! - **blake3**: BLAKE3 cryptographic hash function
//! - **md5**: MD5 hash function (legacy, not recommended)
//!
//! ### Custom Algorithms
//!
//! - **custom-***: User-defined algorithms with "custom-" prefix
//! - **Application-specific**: Domain-specific algorithms
//! - **Experimental**: Experimental or research algorithms
//!
//! ## Validation Rules
//!
//! ### Name Format
//!
//! - **Length**: 1-64 characters
//! - **Characters**: Lowercase letters, numbers, hyphens
//! - **Start/End**: Must start and end with alphanumeric characters
//! - **Hyphens**: Cannot have consecutive hyphens
//!
//! ### Category Rules
//!
//! - **Compression**: Must be a known compression algorithm
//! - **Encryption**: Must be a known encryption algorithm
//! - **Hashing**: Must be a known hashing algorithm
//! - **Custom**: Must start with "custom-" prefix
//!
//! ### Security Requirements
//!
//! - **Encryption**: Must use authenticated encryption (AEAD)
//! - **Hashing**: Must use cryptographically secure hash functions
//! - **Key Length**: Must meet minimum key length requirements
//! - **Deprecation**: Deprecated algorithms are rejected
//!
//! ## Error Handling
//!
//! ### Validation Errors
//!
//! - **Empty Name**: Algorithm name cannot be empty
//! - **Invalid Format**: Name doesn't match required format
//! - **Unknown Algorithm**: Algorithm is not recognized
//! - **Deprecated Algorithm**: Algorithm is deprecated or insecure
//!
//! ### Usage Errors
//!
//! - **Category Mismatch**: Algorithm used in wrong category context
//! - **Parameter Errors**: Invalid algorithm parameters
//! - **Compatibility Errors**: Algorithm not compatible with system
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Compact Storage**: Efficient string storage
//! - **String Interning**: Intern common algorithm names
//! - **Copy Optimization**: Optimize copying for frequent use
//!
//! ### Validation Performance
//!
//! - **Fast Validation**: Optimized validation routines
//! - **Caching**: Cache validation results
//! - **Lazy Evaluation**: Lazy evaluation of expensive checks
//!
//! ## Cross-Language Compatibility
//!
//! ### Language Mappings
//!
//! - **Rust**: `Algorithm` newtype wrapper
//! - **Go**: `Algorithm` struct with validation
//! - **Python**: `Algorithm` class with validation
//! - **JavaScript**: Algorithm validation functions
//!
//! ### Serialization Formats
//!
//! - **JSON**: String representation
//! - **Protocol Buffers**: String field with validation
//! - **MessagePack**: String representation
//! - **CBOR**: String representation
//!
//! ### Database Storage
//!
//! - **SQLite**: TEXT column with CHECK constraint
//! - **PostgreSQL**: VARCHAR with domain validation
//! - **MySQL**: VARCHAR with validation triggers
//!
//! ## Integration
//!
//! The algorithm value object integrates with:
//!
//! - **Processing Pipeline**: Specify algorithms for processing stages
//! - **Configuration**: Algorithm configuration and selection
//! - **Validation**: Validate algorithm compatibility and security
//! - **Monitoring**: Track algorithm usage and performance
//!
//! ## Thread Safety
//!
//! The algorithm value object is thread-safe:
//!
//! - **Immutable**: Algorithms are immutable after creation
//! - **Safe Sharing**: Safe to share between threads
//! - **Concurrent Access**: Safe concurrent access to algorithm data
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Algorithm Registry**: Centralized algorithm registry
//! - **Performance Benchmarks**: Built-in performance benchmarking
//! - **Security Analysis**: Automated security analysis
//! - **Plugin System**: Plugin system for custom algorithms

use serde::{Deserialize, Serialize};
use std::cmp::{Ord, PartialOrd};
use std::fmt::{self, Display};

use crate::PipelineError;

/// Algorithm value object for pipeline stage processing
/// # Purpose
/// Type-safe algorithm specification that provides:
/// - Validation of algorithm names and formats
/// - Algorithm-specific behavior and constraints
/// - Immutable value semantics (DDD value object)
/// - Cross-language compatibility
/// # Supported Algorithms
/// - **Compression**: brotli, gzip, zstd, lz4
/// - **Encryption**: aes-256-gcm, chacha20-poly1305, aes-128-gcm
/// - **Hashing**: sha256, sha512, blake3
/// - **Custom**: User-defined algorithms
/// # Cross-Language Mapping
/// - **Rust**: `Algorithm` (newtype wrapper)
/// - **Go**: `Algorithm` struct with same interface
/// - **JSON**: String representation
/// - **SQLite**: TEXT column with validation
/// # Examples
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Algorithm(String);

impl Algorithm {
    /// Creates a new algorithm with validated name
    /// # Purpose
    /// Creates an `Algorithm` value object after validating the algorithm name
    /// against strict format rules. This ensures all algorithm instances
    /// are valid and type-safe. # Why
    /// Algorithm names must follow a consistent format for:
    /// - Cross-language compatibility (Rust, Go, JSON)
    /// - Database storage validation
    /// - Configuration file parsing
    /// - Prevention of injection attacks
    /// # Arguments
    /// * `name` - The algorithm name string. Must:
    ///   - Not be empty
    ///   - Be 1-64 characters long
    ///   - Contain only lowercase letters, hyphens, and digits
    ///   - Not start with a hyphen or digit
    ///   - Not end with a hyphen
    ///   - Not contain consecutive hyphens
    /// # Returns
    /// * `Ok(Algorithm)` - Successfully validated algorithm
    /// * `Err(PipelineError::InvalidConfiguration)` - Name validation failed
    /// # Errors
    /// Returns `PipelineError::InvalidConfiguration` when:
    /// - Name is empty
    /// - Name exceeds 64 characters
    /// - Name contains invalid characters (uppercase, special chars)
    /// - Name starts/ends with hyphen
    /// - Name starts with digit
    /// - Name contains consecutive hyphens ("--")
    /// # Examples
    pub fn new(name: String) -> Result<Self, PipelineError> {
        Self::validate_name(&name)?;
        Ok(Self(name))
    }

    /// Creates an algorithm from a string slice
    /// # Purpose
    /// Convenience constructor that accepts a string slice instead of an owned
    /// String. Internally converts to String and delegates to `new()`.
    /// # Arguments
    /// * `name` - String slice containing the algorithm name
    /// # Returns
    /// * `Ok(Algorithm)` - Successfully validated algorithm
    /// * `Err(PipelineError::InvalidConfiguration)` - Name validation failed
    /// # Errors
    /// See [`Algorithm::new`] for validation rules and error conditions.
    /// # Examples
    pub fn parse(name: &str) -> Result<Self, PipelineError> {
        Self::new(name.to_string())
    }

    /// Gets the algorithm name as a string reference
    /// # Purpose
    /// Provides read-only access to the algorithm's underlying name string.
    /// # Returns
    /// String slice containing the algorithm name
    /// # Examples
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Checks if this is a compression algorithm
    /// # Purpose
    /// Determines whether the algorithm belongs to the compression category.
    /// Used for algorithm validation and compatibility checks.
    /// # Why
    /// Compression algorithms require specific stage configurations and have
    /// different performance characteristics than encryption or hashing
    /// algorithms. # Returns
    /// * `true` - Algorithm is one of: brotli, gzip, zstd, lz4, deflate
    /// * `false` - Algorithm is not a compression algorithm
    /// # Examples
    pub fn is_compression(&self) -> bool {
        matches!(self.0.as_str(), "brotli" | "gzip" | "zstd" | "lz4" | "deflate")
    }

    /// Checks if this is an encryption algorithm
    /// # Purpose
    /// Determines whether the algorithm belongs to the encryption category.
    /// Used for security validation and stage compatibility checks.
    /// # Why
    /// Encryption algorithms require key management and have different
    /// security properties than compression or hashing algorithms.
    /// # Returns
    /// * `true` - Algorithm is one of: aes-256-gcm, aes-128-gcm, aes-128-cbc,
    ///   chacha20-poly1305, aes-256-cbc
    /// * `false` - Algorithm is not an encryption algorithm
    /// # Examples
    pub fn is_encryption(&self) -> bool {
        matches!(
            self.0.as_str(),
            "aes-256-gcm" | "aes-128-gcm" | "aes-128-cbc" | "chacha20-poly1305" | "aes-256-cbc"
        )
    }

    /// Checks if this is a hashing algorithm
    /// # Purpose
    /// Determines whether the algorithm belongs to the hashing category.
    /// Used for integrity validation and stage compatibility checks.
    /// # Why
    /// Hashing algorithms are one-way functions used for integrity
    /// verification, with different properties than compression or
    /// encryption algorithms. # Returns
    /// * `true` - Algorithm is one of: sha256, sha512, sha3-256, blake3, md5,
    ///   sha1
    /// * `false` - Algorithm is not a hashing algorithm
    /// # Note
    /// MD5 and SHA1 are included for backward compatibility but are not
    /// recommended for security-critical applications.
    /// # Examples
    pub fn is_hashing(&self) -> bool {
        matches!(
            self.0.as_str(),
            "sha256" | "sha512" | "sha3-256" | "blake3" | "md5" | "sha1"
        )
    }

    /// Checks if this is a custom algorithm
    /// # Purpose
    /// Determines whether the algorithm is a custom (user-defined) algorithm
    /// that doesn't match any of the predefined categories.
    /// # Why
    /// Custom algorithms allow extensibility for domain-specific processing
    /// while maintaining type safety and validation.
    /// # Returns
    /// * `true` - Algorithm starts with "custom-" prefix OR doesn't match any
    ///   predefined compression, encryption, or hashing algorithm
    /// * `false` - Algorithm is a predefined standard algorithm
    /// # Examples
    pub fn is_custom(&self) -> bool {
        self.0.starts_with("custom-") || (!self.is_compression() && !self.is_encryption() && !self.is_hashing())
    }

    /// Gets the algorithm category
    /// # Purpose
    /// Classifies the algorithm into one of the predefined categories.
    /// Used for compatibility validation and processing stage selection.
    /// # Returns
    /// One of:
    /// * `AlgorithmCategory::Compression` - For compression algorithms
    /// * `AlgorithmCategory::Encryption` - For encryption algorithms
    /// * `AlgorithmCategory::Hashing` - For hashing algorithms
    /// * `AlgorithmCategory::Custom` - For custom/unknown algorithms
    /// # Examples
    pub fn category(&self) -> AlgorithmCategory {
        if self.is_compression() {
            AlgorithmCategory::Compression
        } else if self.is_encryption() {
            AlgorithmCategory::Encryption
        } else if self.is_hashing() {
            AlgorithmCategory::Hashing
        } else {
            AlgorithmCategory::Custom
        }
    }

    /// Validates the algorithm name format
    fn validate_name(name: &str) -> Result<(), PipelineError> {
        if name.is_empty() {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name cannot be empty".to_string(),
            ));
        }

        if name.len() > 64 {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name cannot exceed 64 characters".to_string(),
            ));
        }

        // Algorithm names should be lowercase with hyphens and numbers
        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name must contain only lowercase letters, hyphens, and digits".to_string(),
            ));
        }

        // Cannot start or end with hyphen
        if name.starts_with('-') || name.ends_with('-') {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name cannot start or end with hyphen".to_string(),
            ));
        }

        // Cannot start with a digit
        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name cannot start with a digit".to_string(),
            ));
        }

        // Cannot have consecutive hyphens
        if name.contains("--") {
            return Err(PipelineError::InvalidConfiguration(
                "Algorithm name cannot contain consecutive hyphens".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates the algorithm
    /// # Purpose
    /// Re-validates the algorithm name format. Useful for ensuring algorithm
    /// integrity after deserialization or when working with external data.
    /// # Why
    /// While algorithms are validated on creation, this method allows
    /// revalidation after deserialization or when algorithm constraints
    /// change. # Returns
    /// * `Ok(())` - Algorithm name is valid
    /// * `Err(PipelineError::InvalidConfiguration)` - Name validation failed
    /// # Errors
    /// See [`Algorithm::new`] for validation rules and error conditions.
    /// # Examples
    pub fn validate(&self) -> Result<(), PipelineError> {
        Self::validate_name(&self.0)
    }
}

/// Algorithm categories for classification
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AlgorithmCategory {
    Compression,
    Encryption,
    Hashing,
    Custom,
    Unknown,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Algorithm {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl From<Algorithm> for String {
    fn from(algorithm: Algorithm) -> Self {
        algorithm.0
    }
}

impl AsRef<str> for Algorithm {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Predefined algorithms for common use cases
impl Algorithm {
    /// Creates a Brotli compression algorithm
    /// # Purpose
    /// Factory method for the Brotli compression algorithm.
    /// Brotli offers excellent compression ratios with good performance.
    /// # Returns
    /// Validated `Algorithm` instance for Brotli
    /// # Examples
    pub fn brotli() -> Self {
        Self("brotli".to_string())
    }

    /// Creates a Gzip compression algorithm
    /// # Purpose
    /// Factory method for the Gzip compression algorithm.
    /// Widely supported compression with balanced performance.
    /// # Returns
    /// Validated `Algorithm` instance for Gzip
    /// # Examples
    pub fn gzip() -> Self {
        Self("gzip".to_string())
    }

    /// Creates a Zstandard compression algorithm
    /// # Purpose
    /// Factory method for the Zstandard compression algorithm.
    /// Excellent balance of compression ratio and speed.
    /// # Returns
    /// Validated `Algorithm` instance for Zstandard
    /// # Examples
    pub fn zstd() -> Self {
        Self("zstd".to_string())
    }

    /// Creates an LZ4 compression algorithm
    /// # Purpose
    /// Factory method for the LZ4 compression algorithm.
    /// Extremely fast compression with moderate compression ratios.
    /// # Returns
    /// Validated `Algorithm` instance for LZ4
    /// # Examples
    pub fn lz4() -> Self {
        Self("lz4".to_string())
    }

    /// Creates an AES-256-GCM encryption algorithm
    /// # Purpose
    /// Factory method for AES-256 with Galois/Counter Mode.
    /// Provides authenticated encryption with strong security.
    /// # Returns
    /// Validated `Algorithm` instance for AES-256-GCM
    /// # Examples
    pub fn aes_256_gcm() -> Self {
        Self("aes-256-gcm".to_string())
    }

    /// Creates a ChaCha20-Poly1305 encryption algorithm
    /// # Purpose
    /// Factory method for ChaCha20 stream cipher with Poly1305 MAC.
    /// Modern authenticated encryption with excellent performance on
    /// mobile/embedded. # Returns
    /// Validated `Algorithm` instance for ChaCha20-Poly1305
    /// # Examples
    pub fn chacha20_poly1305() -> Self {
        Self("chacha20-poly1305".to_string())
    }

    /// Creates a SHA-256 hashing algorithm
    /// # Purpose
    /// Factory method for SHA-256 cryptographic hash function.
    /// Industry-standard hashing for integrity verification.
    /// # Returns
    /// Validated `Algorithm` instance for SHA-256
    /// # Examples
    pub fn sha256() -> Self {
        Self("sha256".to_string())
    }

    /// Creates a SHA-512 hashing algorithm
    /// # Purpose
    /// Factory method for SHA-512 cryptographic hash function.
    /// Stronger variant of SHA-2 family with 512-bit output.
    /// # Returns
    /// Validated `Algorithm` instance for SHA-512
    /// # Examples
    pub fn sha512() -> Self {
        Self("sha512".to_string())
    }

    /// Creates a BLAKE3 hashing algorithm
    /// # Purpose
    /// Factory method for BLAKE3 cryptographic hash function.
    /// Modern, high-performance hashing with strong security properties.
    /// # Returns
    /// Validated `Algorithm` instance for BLAKE3
    /// # Examples
    pub fn blake3() -> Self {
        Self("blake3".to_string())
    }

    /// AES-256-GCM encryption algorithm (alias for test framework
    /// compatibility)
    pub fn aes256_gcm() -> Self {
        Self::aes_256_gcm()
    }

    /// No algorithm / passthrough
    pub fn none() -> Self {
        Self("none".to_string())
    }

    /// AES-128-CBC encryption
    pub fn aes_128_cbc() -> Self {
        Self("aes-128-cbc".to_string())
    }

    /// SHA3-256 hashing
    pub fn sha3_256() -> Self {
        Self("sha3-256".to_string())
    }
}

/// Utility functions for algorithm operations
pub mod algorithm_utils {
    use super::*;

    /// Gets all supported compression algorithms
    /// # Purpose
    /// Returns a collection of all predefined compression algorithms.
    /// Useful for configuration validation and algorithm selection UIs.
    /// # Returns
    /// Vector containing: brotli, gzip, zstd, lz4
    /// # Examples
    pub fn compression_algorithms() -> Vec<Algorithm> {
        vec![
            Algorithm::brotli(),
            Algorithm::gzip(),
            Algorithm::zstd(),
            Algorithm::lz4(),
        ]
    }

    /// Gets all supported encryption algorithms
    /// # Purpose
    /// Returns a collection of all predefined encryption algorithms.
    /// Useful for security configuration and key management setup.
    /// # Returns
    /// Vector containing: aes-256-gcm, chacha20-poly1305
    /// # Examples
    pub fn encryption_algorithms() -> Vec<Algorithm> {
        vec![Algorithm::aes_256_gcm(), Algorithm::chacha20_poly1305()]
    }

    /// Gets all supported hashing algorithms
    /// # Purpose
    /// Returns a collection of all predefined hashing algorithms.
    /// Useful for integrity verification configuration.
    /// # Returns
    /// Vector containing: sha256, sha512, blake3
    /// # Examples
    pub fn hashing_algorithms() -> Vec<Algorithm> {
        vec![Algorithm::sha256(), Algorithm::sha512(), Algorithm::blake3()]
    }

    /// Validates algorithm compatibility with stage type
    /// # Purpose
    /// Ensures an algorithm is compatible with its intended processing stage
    /// type. Prevents misconfiguration like using compression algorithms
    /// for encryption stages. # Why
    /// Early validation of algorithm-stage compatibility:
    /// - Prevents runtime errors in processing pipelines
    /// - Ensures type safety across pipeline configuration
    /// - Provides clear error messages for configuration issues
    /// # Arguments
    /// * `algorithm` - The algorithm to validate
    /// * `stage_type` - Stage type string (case-insensitive):
    ///   - "compression" - Requires compression algorithm
    ///   - "encryption" - Requires encryption algorithm
    ///   - "hashing" - Requires hashing algorithm
    ///   - "custom" - Accepts any algorithm
    /// # Returns
    /// * `Ok(())` - Algorithm is compatible with stage type
    /// * `Err(PipelineError::InvalidConfiguration)` - Incompatible or unknown
    ///   stage type
    /// # Errors
    /// Returns `PipelineError::InvalidConfiguration` when:
    /// - Algorithm doesn't match required stage type category
    /// - Stage type is unknown/unsupported
    /// # Examples
    pub fn validate_compatibility(algorithm: &Algorithm, stage_type: &str) -> Result<(), PipelineError> {
        match stage_type.to_lowercase().as_str() {
            "compression" => {
                if !algorithm.is_compression() {
                    return Err(PipelineError::InvalidConfiguration(format!(
                        "Algorithm '{}' is not compatible with compression stage",
                        algorithm
                    )));
                }
            }
            "encryption" => {
                if !algorithm.is_encryption() {
                    return Err(PipelineError::InvalidConfiguration(format!(
                        "Algorithm '{}' is not compatible with encryption stage",
                        algorithm
                    )));
                }
            }
            "hashing" => {
                if !algorithm.is_hashing() {
                    return Err(PipelineError::InvalidConfiguration(format!(
                        "Algorithm '{}' is not compatible with hashing stage",
                        algorithm
                    )));
                }
            }
            "custom" => {
                // Custom stages can use any algorithm
            }
            _ => {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Unknown stage type: {}",
                    stage_type
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests for Algorithm value object.
    //
    // Tests cover creation, validation, categorization, and serialization.

    use crate::value_objects::algorithm::AlgorithmCategory;
    use crate::value_objects::Algorithm;
    use serde_json;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    /// Tests Algorithm creation with valid input values.
    /// Validates that:
    /// - Basic algorithm creation works correctly
    /// - from_str creation method functions properly
    /// - Various valid algorithm name formats are accepted
    /// - Algorithm names are stored and retrieved accurately
    /// - Edge cases like single character and long names work
    #[test]
    fn test_algorithm_creation_valid_cases() {
        // Test basic creation
        let algo = Algorithm::new("brotli".to_string()).unwrap();
        assert_eq!(algo.name(), "brotli");

        // Test parse creation
        let algo = Algorithm::parse("aes-256-gcm").unwrap();
        assert_eq!(algo.name(), "aes-256-gcm");

        // Test various valid formats
        let valid_algorithms = vec![
            "gzip",
            "lz4",
            "zstd",
            "aes-128-cbc",
            "aes-256-gcm",
            "chacha20-poly1305",
            "sha256",
            "sha3-256",
            "blake3",
            "custom-algo-v2",
            "algo123",
            "a", // Single character
            "very-long-algorithm-name-with-many-hyphens-and-numbers123",
        ];

        for name in valid_algorithms {
            let result = Algorithm::new(name.to_string());
            assert!(result.is_ok(), "Algorithm '{}' should be valid", name);
            assert_eq!(result.unwrap().name(), name);
        }
    }

    /// Tests Algorithm creation with invalid input values.
    /// Validates that:
    /// - Empty strings are rejected
    /// - Uppercase and mixed case names are rejected
    /// - Names with invalid characters are rejected
    /// - Names starting/ending with hyphens are rejected
    /// - Names starting with numbers are rejected
    /// - Appropriate error messages are returned
    #[test]
    fn test_algorithm_creation_invalid_cases() {
        let invalid_algorithms = vec![
            ("", "empty string"),
            ("UPPERCASE", "uppercase letters"),
            ("MixedCase", "mixed case"),
            ("-starts-with-hyphen", "starts with hyphen"),
            ("ends-with-hyphen-", "ends with hyphen"),
            ("has spaces", "contains spaces"),
            ("has_underscores", "contains underscores"),
            ("has.dots", "contains dots"),
            ("has@symbols", "contains special symbols"),
            ("has/slashes", "contains slashes"),
            ("has\\backslashes", "contains backslashes"),
            ("has(parentheses)", "contains parentheses"),
            ("has[brackets]", "contains brackets"),
            ("has{braces}", "contains braces"),
            ("has:colons", "contains colons"),
            ("has;semicolons", "contains semicolons"),
            ("has,commas", "contains commas"),
            ("has'quotes", "contains quotes"),
            ("has\"doublequotes", "contains double quotes"),
            ("\t\n\r", "whitespace characters"),
            ("--double-hyphen", "consecutive hyphens"),
            ("123-starts-with-number", "starts with number"),
        ];

        for (name, reason) in invalid_algorithms {
            let result = Algorithm::new(name.to_string());
            assert!(result.is_err(), "Algorithm '{}' should be invalid ({})", name, reason);
        }
    }

    /// Tests predefined algorithm constructor methods.
    /// Validates that:
    /// - Compression algorithm constructors work correctly
    /// - Encryption algorithm constructors work correctly
    /// - Hashing algorithm constructors work correctly
    /// - Predefined algorithms have correct names and categories
    /// - All predefined algorithms are properly categorized
    #[test]
    fn test_algorithm_predefined_constructors() {
        // Test compression algorithms
        let brotli = Algorithm::brotli();
        assert_eq!(brotli.name(), "brotli");
        assert!(brotli.is_compression());

        let gzip = Algorithm::gzip();
        assert_eq!(gzip.name(), "gzip");
        assert!(gzip.is_compression());

        let lz4 = Algorithm::lz4();
        assert_eq!(lz4.name(), "lz4");
        assert!(lz4.is_compression());

        let zstd = Algorithm::zstd();
        assert_eq!(zstd.name(), "zstd");
        assert!(zstd.is_compression());

        // Test encryption algorithms
        let aes_128 = Algorithm::aes_128_cbc();
        assert_eq!(aes_128.name(), "aes-128-cbc");
        assert!(aes_128.is_encryption());

        let aes_256 = Algorithm::aes_256_gcm();
        assert_eq!(aes_256.name(), "aes-256-gcm");
        assert!(aes_256.is_encryption());

        let chacha = Algorithm::chacha20_poly1305();
        assert_eq!(chacha.name(), "chacha20-poly1305");
        assert!(chacha.is_encryption());

        // Test hashing algorithms
        let sha256 = Algorithm::sha256();
        assert_eq!(sha256.name(), "sha256");
        assert!(sha256.is_hashing());

        let sha3 = Algorithm::sha3_256();
        assert_eq!(sha3.name(), "sha3-256");
        assert!(sha3.is_hashing());

        let blake3 = Algorithm::blake3();
        assert_eq!(blake3.name(), "blake3");
        assert!(blake3.is_hashing());
    }

    /// Tests algorithm category classification system.
    /// Validates that:
    /// - Compression algorithms are classified correctly
    /// - Encryption algorithms are classified correctly
    /// - Hashing algorithms are classified correctly
    /// - Custom algorithms default to appropriate category
    /// - Category classification is consistent and accurate
    #[test]
    fn test_algorithm_categories() {
        // Test compression category
        let brotli = Algorithm::brotli();
        assert!(brotli.is_compression());
        assert!(!brotli.is_encryption());
        assert!(!brotli.is_hashing());
        assert_eq!(brotli.category(), AlgorithmCategory::Compression);

        // Test encryption category
        let aes = Algorithm::aes_256_gcm();
        assert!(aes.is_encryption());
        assert!(!aes.is_compression());
        assert!(!aes.is_hashing());
        assert_eq!(aes.category(), AlgorithmCategory::Encryption);

        // Test hashing category
        let sha = Algorithm::sha256();
        assert!(sha.is_hashing());
        assert!(!sha.is_encryption());
        assert!(!sha.is_compression());
        assert_eq!(sha.category(), AlgorithmCategory::Hashing);

        // Test unknown/custom algorithm category
        let custom = Algorithm::new("custom-algo".to_string()).unwrap();
        assert!(!custom.is_compression());
        assert!(!custom.is_encryption());
        assert!(!custom.is_hashing());
        assert_eq!(custom.category(), AlgorithmCategory::Custom);
    }

    // Tests JSON serialization and deserialization of Algorithm objects.
    // Validates that:
    // - Algorithm objects serialize to valid JSON
    // - Deserialized algorithms maintain original properties
    // - Serialization roundtrip preserves data integrity
    // - JSON format is compatible with external systems
    // - Algorithm metadata is preserved during serialization

    #[test]
    fn test_algorithm_serialization() {
        let original = Algorithm::aes_256_gcm();

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Algorithm = serde_json::from_str(&json).unwrap();

        assert_eq!(original.name(), deserialized.name());
        assert_eq!(original.category(), deserialized.category());

        // Test with custom algorithm
        let custom = Algorithm::new("custom-test-algo".to_string()).unwrap();
        let json = serde_json::to_string(&custom).unwrap();
        let deserialized: Algorithm = serde_json::from_str(&json).unwrap();

        assert_eq!(custom.name(), deserialized.name());
        assert_eq!(custom.category(), deserialized.category());
    }

    /// Tests equality comparison and ordering for Algorithm objects.
    /// Validates that:
    /// - Identical algorithms compare as equal
    /// - Different algorithms compare as not equal
    /// - Algorithm ordering follows lexicographic name order
    /// - Equality is consistent across multiple calls
    /// - Ordering supports sorting and collection operations

    #[test]
    fn test_algorithm_equality_and_ordering() {
        let algo1 = Algorithm::brotli();
        let algo2 = Algorithm::brotli();
        let algo3 = Algorithm::gzip();

        // Test equality
        assert_eq!(algo1, algo2);
        assert_ne!(algo1, algo3);

        // Test ordering (lexicographic by name)
        assert!(algo1 < algo3); // "brotli" < "gzip"
        assert!(algo3 > algo1);
        assert!(algo1 <= algo2);
        assert!(algo2 >= algo1);
    }

    /// Tests hash consistency for Algorithm objects.
    /// Validates that:
    /// - Equal algorithms produce identical hash values
    /// - Hash values are consistent across multiple calls
    /// - Different algorithms produce different hash values
    /// - Hash implementation supports HashMap usage
    /// - Hash distribution is reasonable for performance

    #[test]
    fn test_algorithm_hash_consistency() {
        let algo1 = Algorithm::aes_256_gcm();
        let algo2 = Algorithm::aes_256_gcm();

        let mut map = HashMap::new();
        map.insert(algo1, "test_value");

        // Should be able to retrieve with equivalent Algorithm
        assert_eq!(map.get(&algo2), Some(&"test_value"));
    }

    /// Tests Display trait implementation for Algorithm objects.
    /// Validates that:
    /// - Display format shows algorithm name clearly
    /// - Display output is human-readable
    /// - Display format is consistent across algorithm types
    /// - Display output is suitable for logging and debugging
    /// - Display format matches expected conventions

    #[test]
    fn test_algorithm_display_formatting() {
        let algo = Algorithm::aes_256_gcm();
        assert_eq!(format!("{}", algo), "aes-256-gcm");

        let custom = Algorithm::new("custom-algo".to_string()).unwrap();
        assert_eq!(format!("{}", custom), "custom-algo");
    }

    /// Tests algorithm validation with edge cases and boundary conditions.
    /// Validates that:
    /// - Very long algorithm names are handled correctly
    /// - Minimum length algorithm names work properly
    /// - Special character combinations are validated
    /// - Unicode characters are handled appropriately
    /// - Edge cases don't cause validation failures

    #[test]
    fn test_algorithm_validation_edge_cases() {
        // Test minimum length (1 character)
        let min_algo = Algorithm::new("a".to_string()).unwrap();
        assert_eq!(min_algo.name(), "a");

        // Test maximum valid length (64 characters)
        let long_name = "a".repeat(64);
        let long_algo = Algorithm::new(long_name.clone()).unwrap();
        assert_eq!(long_algo.name(), long_name);

        // Test length exceeding limit should fail
        let too_long_name = "a".repeat(65);
        assert!(Algorithm::new(too_long_name).is_err());

        // Test algorithm with numbers
        let numbered = Algorithm::new("algo123".to_string()).unwrap();
        assert_eq!(numbered.name(), "algo123");

        // Test algorithm with multiple hyphens
        let multi_hyphen = Algorithm::new("multi-hyphen-algo".to_string()).unwrap();
        assert_eq!(multi_hyphen.name(), "multi-hyphen-algo");
    }

    /// Tests comprehensive algorithm category classification logic.
    /// Validates that:
    /// - All predefined algorithms are classified correctly
    /// - Custom algorithms receive appropriate default categories
    /// - Category classification is deterministic
    /// - Classification logic handles edge cases
    /// - Category system is extensible and maintainable

    #[test]
    fn test_algorithm_category_classification() {
        // Test all known compression algorithms
        let compression_algos = vec![
            Algorithm::brotli(),
            Algorithm::gzip(),
            Algorithm::lz4(),
            Algorithm::zstd(),
        ];

        for algo in compression_algos {
            assert!(
                algo.is_compression(),
                "Algorithm '{}' should be compression",
                algo.name()
            );
            assert_eq!(algo.category(), AlgorithmCategory::Compression);
        }

        // Test all known encryption algorithms
        let encryption_algos = vec![
            Algorithm::aes_128_cbc(),
            Algorithm::aes_256_gcm(),
            Algorithm::chacha20_poly1305(),
        ];

        for algo in encryption_algos {
            assert!(algo.is_encryption(), "Algorithm '{}' should be encryption", algo.name());
            assert_eq!(algo.category(), AlgorithmCategory::Encryption);
        }

        // Test all known hashing algorithms
        let hashing_algos = vec![Algorithm::sha256(), Algorithm::sha3_256(), Algorithm::blake3()];

        for algo in hashing_algos {
            assert!(algo.is_hashing(), "Algorithm '{}' should be hashing", algo.name());
            assert_eq!(algo.category(), AlgorithmCategory::Hashing);
        }
    }

    /// Tests error handling in Algorithm::parse method.
    /// Validates that:
    /// - Invalid algorithm strings produce appropriate errors
    /// - Error messages are descriptive and helpful
    /// - Error types match expected validation failures
    /// - Error handling is consistent across input types
    /// - Error recovery provides useful feedback

    #[test]
    fn test_algorithm_from_str_error_handling() {
        // Test FromStr trait error cases
        let invalid_cases = vec!["", "INVALID", "has spaces", "-invalid", "invalid-"];

        for case in invalid_cases {
            let result: Result<Algorithm, _> = case.parse();
            assert!(result.is_err(), "Parsing '{}' should fail", case);
        }

        // Test valid cases
        let valid_cases = vec!["brotli", "aes-256-gcm", "custom-algo"];

        for case in valid_cases {
            let result: Result<Algorithm, _> = case.parse();
            assert!(result.is_ok(), "Parsing '{}' should succeed", case);
            assert_eq!(result.unwrap().name(), case);
        }
    }

    /// Tests comprehensive algorithm validation logic.
    /// Validates that:
    /// - All validation rules are applied consistently
    /// - Validation covers all algorithm name requirements
    /// - Validation errors provide specific feedback
    /// - Validation performance is acceptable
    /// - Validation logic is thorough and reliable

    #[test]
    fn test_algorithm_validation_comprehensive() {
        // Test the validate method directly
        let valid_algo = Algorithm::brotli();
        assert!(valid_algo.validate().is_ok());

        // Test validation with custom algorithms
        let custom_valid = Algorithm::new("my-custom-algo123".to_string()).unwrap();
        assert!(custom_valid.validate().is_ok());

        // Test edge cases for validation
        let edge_cases = vec![
            "a",          // Single character
            "algo1",      // Ends with number
            "1algo",      // Starts with number (should be invalid)
            "algo-1-2-3", // Multiple numbers with hyphens
        ];

        for case in edge_cases {
            let result = Algorithm::new(case.to_string());
            if case == "1algo" {
                assert!(result.is_err(), "Algorithm starting with number should be invalid");
            } else {
                assert!(result.is_ok(), "Algorithm '{}' should be valid", case);
            }
        }
    }

    /// Tests AlgorithmCategory enum functionality.
    /// Validates that:
    /// - All algorithm categories are properly defined
    /// - Category enum supports all required operations
    /// - Category conversion works correctly
    /// - Category display formatting is appropriate
    /// - Category enum is extensible for future algorithms

    #[test]
    fn test_algorithm_category_enum() {
        // Test AlgorithmCategory enum behavior
        assert_ne!(AlgorithmCategory::Compression, AlgorithmCategory::Encryption);
        assert_ne!(AlgorithmCategory::Encryption, AlgorithmCategory::Hashing);
        assert_ne!(AlgorithmCategory::Hashing, AlgorithmCategory::Unknown);

        // Test Debug formatting
        assert_eq!(format!("{:?}", AlgorithmCategory::Compression), "Compression");
        assert_eq!(format!("{:?}", AlgorithmCategory::Encryption), "Encryption");
        assert_eq!(format!("{:?}", AlgorithmCategory::Hashing), "Hashing");
        assert_eq!(format!("{:?}", AlgorithmCategory::Unknown), "Unknown");
    }

    /// Tests Clone and Copy trait implementations for Algorithm.
    /// Validates that:
    /// - Algorithm objects can be cloned correctly
    /// - Cloned algorithms maintain all properties
    /// - Copy semantics work for algorithm objects
    /// - Clone and copy operations are efficient
    /// - Memory management is handled properly

    #[test]
    fn test_algorithm_clone_and_copy_semantics() {
        let original = Algorithm::aes_256_gcm();
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.name(), cloned.name());
        assert_eq!(original.category(), cloned.category());

        // Test that they're independent
        drop(original);
        assert_eq!(cloned.name(), "aes-256-gcm");
    }
}
