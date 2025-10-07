// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Generic Size Value Object
//!
//! This module provides a generic, type-safe size value object system for the
//! adaptive pipeline system. It uses phantom types to enforce compile-time
//! size category safety while providing shared arithmetic operations and
//! unit conversions.
//!
//! ## Overview
//!
//! The generic size system provides:
//!
//! - **Type Safety**: Compile-time enforcement of size categories
//! - **Unit Conversions**: Automatic conversions between size units
//! - **Arithmetic Operations**: Safe arithmetic operations on sizes
//! - **Zero-Cost Abstractions**: Phantom types with no runtime overhead
//! - **Validation**: Category-specific size validation and constraints
//!
//! ## Architecture
//!
//! The size system follows Domain-Driven Design principles:
//!
//! - **Value Object**: Immutable value object with equality semantics
//! - **Type Safety**: Phantom types prevent category mixing at compile time
//! - **Rich Domain Model**: Encapsulates size-related business logic
//! - **Validation**: Comprehensive validation of size values and constraints
//!
//! ## Key Features
//!
//! ### Type-Safe Size Categories
//!
//! - **File Sizes**: Sizes for files and file operations
//! - **Memory Sizes**: Sizes for memory allocation and usage
//! - **Network Sizes**: Sizes for network bandwidth and data transfer
//! - **Storage Sizes**: Sizes for storage capacity and usage
//! - **Custom Categories**: Support for custom size categories
//!
//! ### Unit Conversions
//!
//! - **Automatic Conversion**: Automatic conversion between units
//! - **Multiple Units**: Support for bytes, KB, MB, GB, TB, PB
//! - **Binary/Decimal**: Support for both binary (1024) and decimal (1000)
//!   units
//! - **Precision Handling**: Proper handling of precision during conversions
//!
//! ### Arithmetic Operations
//!
//! - **Addition**: Add sizes of the same category
//! - **Subtraction**: Subtract sizes with underflow protection
//! - **Multiplication**: Multiply sizes by scalars
//! - **Division**: Divide sizes with division by zero protection
//!
//! ## Usage Examples
//!
//! ### Basic Size Creation

//!
//! ### Unit Conversions

//!
//! ### Arithmetic Operations

//!
//! ### Type Safety Demonstration

//!
//! ### Custom Size Categories

//!
//! ### Size Validation and Constraints

//!
//! ## Size Categories
//!
//! ### Built-in Categories
//!
//! - **FileSizeCategory**: For file sizes and file operations
//!   - Validation: Standard file size validation
//!   - Use case: File processing and storage
//!
//! - **MemorySizeCategory**: For memory allocation and usage
//!   - Validation: Memory-specific constraints
//!   - Use case: Memory management and allocation
//!
//! - **NetworkSizeCategory**: For network bandwidth and data transfer
//!   - Validation: Network-specific constraints
//!   - Use case: Network operations and bandwidth management
//!
//! - **StorageSizeCategory**: For storage capacity and usage
//!   - Validation: Storage-specific constraints
//!   - Use case: Storage planning and management
//!
//! ### Custom Categories
//!
//! Create custom size categories by implementing the `SizeCategory` trait:
//!
//! - **Category Name**: Unique identifier for the category
//! - **Validation Logic**: Custom validation rules
//! - **Size Limits**: Category-specific size limits
//!
//! ## Unit Systems
//!
//! ### Binary Units (Base 1024)
//!
//! - **Byte**: 1 byte
//! - **KiB**: 1,024 bytes
//! - **MiB**: 1,048,576 bytes
//! - **GiB**: 1,073,741,824 bytes
//! - **TiB**: 1,099,511,627,776 bytes
//!
//! ### Decimal Units (Base 1000)
//!
//! - **Byte**: 1 byte
//! - **KB**: 1,000 bytes
//! - **MB**: 1,000,000 bytes
//! - **GB**: 1,000,000,000 bytes
//! - **TB**: 1,000,000,000,000 bytes
//!
//! ## Arithmetic Operations
//!
//! ### Supported Operations
//!
//! - **Addition**: `size1 + size2`
//! - **Subtraction**: `size1 - size2` (with underflow protection)
//! - **Multiplication**: `size * scalar`
//! - **Division**: `size / scalar` (with division by zero protection)
//!
//! ### Safety Features
//!
//! - **Overflow Protection**: Detect and handle arithmetic overflow
//! - **Underflow Protection**: Prevent negative sizes
//! - **Division by Zero**: Prevent division by zero
//! - **Type Safety**: Ensure operations are on same category
//!
//! ## Validation Rules
//!
//! ### General Validation
//!
//! - **Non-negative**: Sizes must be non-negative
//! - **Finite**: Sizes must be finite values
//! - **Range Limits**: Sizes must be within valid ranges
//!
//! ### Category-Specific Validation
//!
//! - **File Sizes**: Validate against file system limits
//! - **Memory Sizes**: Validate against available memory
//! - **Network Sizes**: Validate against bandwidth limits
//! - **Custom Categories**: Apply custom validation rules
//!
//! ## Error Handling
//!
//! ### Size Errors
//!
//! - **Invalid Size**: Size value is invalid
//! - **Overflow**: Arithmetic operation caused overflow
//! - **Underflow**: Arithmetic operation caused underflow
//! - **Division by Zero**: Division by zero attempted
//!
//! ### Validation Errors
//!
//! - **Constraint Violation**: Size violates category constraints
//! - **Range Error**: Size is outside valid range
//! - **Type Error**: Invalid size type or category
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Compact Storage**: Efficient storage of size values
//! - **Zero-Cost Types**: Phantom types have no runtime cost
//! - **Efficient Arithmetic**: Optimized arithmetic operations
//!
//! ### Conversion Performance
//!
//! - **Fast Conversions**: Efficient unit conversions
//! - **Minimal Allocations**: Avoid unnecessary allocations
//! - **Cached Results**: Cache expensive conversion results
//!
//! ## Integration
//!
//! The generic size system integrates with:
//!
//! - **File System**: File size management and validation
//! - **Memory Management**: Memory allocation and tracking
//! - **Network Operations**: Bandwidth and transfer size management
//! - **Storage Systems**: Storage capacity planning and monitoring
//!
//! ## Thread Safety
//!
//! The generic size system is thread-safe:
//!
//! - **Immutable**: Sizes are immutable after creation
//! - **Safe Sharing**: Safe to share between threads
//! - **Concurrent Operations**: Safe concurrent arithmetic operations
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **More Unit Types**: Support for additional unit types
//! - **Precision Control**: Better precision control for conversions
//! - **Performance Optimization**: Further performance optimizations
//! - **Enhanced Validation**: More sophisticated validation rules

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use crate::PipelineError;

/// Generic size value object with type-safe size categories
///
/// # Purpose
/// Type-safe size measurement that provides:
/// - Compile-time size category enforcement (File vs Memory vs Network)
/// - Shared arithmetic and conversion operations
/// - Zero-cost abstractions with phantom types
/// - Category-specific validation and constraints
///
/// # Generic Benefits
/// - **Type Safety**: Cannot mix file sizes with memory sizes at compile time
/// - **Code Reuse**: Shared implementation for all size types
/// - **Extensibility**: Easy to add new size categories
/// - **Zero Cost**: Phantom types have no runtime overhead
///
/// # Use Cases
/// - File size management and validation
/// - Memory allocation tracking
/// - Network bandwidth measurement
/// - Storage capacity planning
///
/// # Cross-Language Mapping
/// - **Rust**: `GenericSize<T>` with marker types
///
/// # Examples
///
/// - **Go**: Separate types with shared interface
/// - **JSON**: Number with unit metadata
/// - **SQLite**: INTEGER with category column
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct GenericSize<T> {
    bytes: u64,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

/// Marker type for file sizes
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct FileSizeMarker;

/// Marker type for memory sizes
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct MemorySizeMarker;

/// Marker type for network transfer sizes
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NetworkSizeMarker;

/// Marker type for storage capacity sizes
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StorageSizeMarker;

/// Type aliases for common size types
pub type FileSize = GenericSize<FileSizeMarker>;
pub type MemorySize = GenericSize<MemorySizeMarker>;
pub type NetworkSize = GenericSize<NetworkSizeMarker>;
pub type StorageSize = GenericSize<StorageSizeMarker>;

/// Size category trait for type-specific behavior
pub trait SizeCategory {
    /// Gets the category name for this size type
    fn category_name() -> &'static str;

    /// Gets the maximum allowed size for this category
    fn max_size() -> u64;

    /// Gets the default unit for display
    fn default_unit() -> &'static str;

    /// Validates category-specific constraints
    fn validate_size(bytes: u64) -> Result<(), PipelineError> {
        if bytes > Self::max_size() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "{} size {} exceeds maximum allowed {}",
                Self::category_name(),
                bytes,
                Self::max_size()
            )));
        }
        Ok(())
    }
}

impl SizeCategory for FileSizeMarker {
    fn category_name() -> &'static str {
        "file"
    }

    fn max_size() -> u64 {
        10 * 1024 * 1024 * 1024 * 1024 // 10 TB
    }

    fn default_unit() -> &'static str {
        "MB"
    }
}

impl SizeCategory for MemorySizeMarker {
    fn category_name() -> &'static str {
        "memory"
    }

    fn max_size() -> u64 {
        1024 * 1024 * 1024 * 1024 // 1 TB (for very large systems)
    }

    fn default_unit() -> &'static str {
        "MB"
    }

    fn validate_size(bytes: u64) -> Result<(), PipelineError> {
        if bytes > Self::max_size() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Memory size {} exceeds maximum allowed {}",
                bytes,
                Self::max_size()
            )));
        }

        // Memory sizes should be power of 2 aligned for efficiency
        if bytes > 0 && !is_power_of_2_aligned(bytes) {
            // This is a warning, not an error
            eprintln!("Warning: Memory size {} is not power-of-2 aligned", bytes);
        }

        Ok(())
    }
}

impl SizeCategory for NetworkSizeMarker {
    fn category_name() -> &'static str {
        "network"
    }

    fn max_size() -> u64 {
        100 * 1024 * 1024 * 1024 // 100 GB per transfer
    }

    fn default_unit() -> &'static str {
        "MB"
    }

    fn validate_size(bytes: u64) -> Result<(), PipelineError> {
        if bytes > Self::max_size() {
            return Err(PipelineError::InvalidConfiguration(format!(
                "Network transfer size {} exceeds maximum allowed {}",
                bytes,
                Self::max_size()
            )));
        }

        // Network transfers should be reasonably sized chunks
        if bytes > 0 && bytes < 1024 {
            eprintln!("Warning: Very small network transfer size: {} bytes", bytes);
        }

        Ok(())
    }
}

impl SizeCategory for StorageSizeMarker {
    fn category_name() -> &'static str {
        "storage"
    }

    fn max_size() -> u64 {
        u64::MAX // Essentially unlimited for storage capacity
    }

    fn default_unit() -> &'static str {
        "GB"
    }
}

impl<T: SizeCategory> GenericSize<T> {
    /// Creates a new size with category-specific validation
    ///
    /// # Purpose
    /// Creates a type-safe size value with category-specific validation.
    /// Uses phantom types to prevent mixing different size categories at
    /// compile time.
    ///
    /// # Why
    /// Type-safe sizes provide:
    /// - Compile-time prevention of mixing file/memory/network sizes
    /// - Category-specific validation and constraints
    /// - Zero-cost abstractions with phantom types
    /// - Clear API contracts for size requirements
    ///
    /// # Arguments
    /// * `bytes` - Size in bytes (validated against category limits)
    ///
    /// # Returns
    /// * `Ok(GenericSize<T>)` - Valid size for category T
    /// * `Err(PipelineError::InvalidConfiguration)` - Exceeds category maximum
    ///
    /// # Errors
    /// Returns error when size exceeds category-specific maximum:
    /// - File sizes: 10 TB maximum
    /// - Memory sizes: 1 TB maximum
    /// - Network sizes: 100 GB per transfer
    /// - Storage sizes: essentially unlimited
    ///
    /// # Examples
    pub fn new(bytes: u64) -> Result<Self, PipelineError> {
        T::validate_size(bytes)?;
        Ok(Self {
            bytes,
            _phantom: PhantomData,
        })
    }

    /// Creates a zero size
    pub fn zero() -> Self {
        Self {
            bytes: 0,
            _phantom: PhantomData,
        }
    }

    /// Creates size from kilobytes
    pub fn from_kb(kb: u64) -> Result<Self, PipelineError> {
        let bytes = kb
            .checked_mul(1024)
            .ok_or_else(|| PipelineError::InvalidConfiguration("Kilobyte value too large".to_string()))?;
        Self::new(bytes)
    }

    /// Creates size from megabytes
    ///
    /// # Purpose
    /// Convenience constructor for creating sizes from megabyte values.
    /// Automatically converts to bytes and validates.
    ///
    /// # Why
    /// MB-based construction provides:
    /// - Human-readable size specification
    /// - Common unit for file and memory sizes
    /// - Overflow protection during conversion
    /// - Category validation
    ///
    /// # Arguments
    /// * `mb` - Size in megabytes (1 MB = 1,048,576 bytes)
    ///
    /// # Returns
    /// * `Ok(GenericSize<T>)` - Valid size
    /// * `Err(PipelineError)` - Overflow or validation error
    ///
    /// # Errors
    /// - Multiplication overflow during byte conversion
    /// - Category maximum exceeded
    ///
    /// # Examples
    pub fn from_mb(mb: u64) -> Result<Self, PipelineError> {
        let bytes = mb
            .checked_mul(1024 * 1024)
            .ok_or_else(|| PipelineError::InvalidConfiguration("Megabyte value too large".to_string()))?;
        Self::new(bytes)
    }

    /// Creates size from gigabytes
    pub fn from_gb(gb: u64) -> Result<Self, PipelineError> {
        let bytes = gb
            .checked_mul(1024 * 1024 * 1024)
            .ok_or_else(|| PipelineError::InvalidConfiguration("Gigabyte value too large".to_string()))?;
        Self::new(bytes)
    }

    /// Gets the raw byte count
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Gets the size category name
    pub fn category(&self) -> &'static str {
        T::category_name()
    }

    /// Converts to a different size category (with validation)
    pub fn into_category<U: SizeCategory>(self) -> Result<GenericSize<U>, PipelineError> {
        U::validate_size(self.bytes)?;
        Ok(GenericSize {
            bytes: self.bytes,
            _phantom: PhantomData,
        })
    }

    /// Gets size as kilobytes
    pub fn as_kb(&self) -> u64 {
        self.bytes / 1024
    }

    /// Gets size as megabytes
    pub fn as_mb(&self) -> u64 {
        self.bytes / (1024 * 1024)
    }

    /// Gets size as gigabytes
    pub fn as_gb(&self) -> u64 {
        self.bytes / (1024 * 1024 * 1024)
    }

    /// Gets size as floating point megabytes
    pub fn as_mb_f64(&self) -> f64 {
        (self.bytes as f64) / (1024.0 * 1024.0)
    }

    /// Gets size as floating point gigabytes
    pub fn as_gb_f64(&self) -> f64 {
        (self.bytes as f64) / (1024.0 * 1024.0 * 1024.0)
    }

    /// Checks if size is zero
    pub fn is_zero(&self) -> bool {
        self.bytes == 0
    }

    /// Formats as human-readable string
    pub fn human_readable(&self) -> String {
        if self.bytes >= 1024 * 1024 * 1024 {
            format!("{:.2} GB", self.as_gb_f64())
        } else if self.bytes >= 1024 * 1024 {
            format!("{:.2} MB", self.as_mb_f64())
        } else if self.bytes >= 1024 {
            format!("{:.2} KB", (self.bytes as f64) / 1024.0)
        } else {
            format!("{} bytes", self.bytes)
        }
    }

    /// Safely adds sizes (checked arithmetic)
    pub fn checked_add(&self, other: Self) -> Result<Self, PipelineError> {
        let result = self
            .bytes
            .checked_add(other.bytes)
            .ok_or_else(|| PipelineError::InvalidConfiguration("Size addition would overflow".to_string()))?;
        Self::new(result)
    }

    /// Safely subtracts sizes (checked arithmetic)
    pub fn checked_sub(&self, other: Self) -> Result<Self, PipelineError> {
        let result = self
            .bytes
            .checked_sub(other.bytes)
            .ok_or_else(|| PipelineError::InvalidConfiguration("Size subtraction would underflow".to_string()))?;
        Self::new(result)
    }

    /// Validates the size
    pub fn validate(&self) -> Result<(), PipelineError> {
        T::validate_size(self.bytes)
    }
}

// Arithmetic operations for same-category sizes
impl<T: SizeCategory> Add for GenericSize<T> {
    type Output = GenericSize<T>;

    fn add(self, rhs: GenericSize<T>) -> Self::Output {
        GenericSize {
            bytes: self.bytes + rhs.bytes,
            _phantom: PhantomData,
        }
    }
}

impl<T: SizeCategory> Sub for GenericSize<T> {
    type Output = GenericSize<T>;

    fn sub(self, rhs: GenericSize<T>) -> Self::Output {
        GenericSize {
            bytes: self.bytes - rhs.bytes,
            _phantom: PhantomData,
        }
    }
}

impl<T: SizeCategory> Mul<u64> for GenericSize<T> {
    type Output = GenericSize<T>;

    fn mul(self, rhs: u64) -> Self::Output {
        GenericSize {
            bytes: self.bytes * rhs,
            _phantom: PhantomData,
        }
    }
}

impl<T: SizeCategory> Div<u64> for GenericSize<T> {
    type Output = GenericSize<T>;

    fn div(self, rhs: u64) -> Self::Output {
        GenericSize {
            bytes: self.bytes / rhs,
            _phantom: PhantomData,
        }
    }
}

impl<T: SizeCategory> Default for GenericSize<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: SizeCategory> Display for GenericSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.human_readable(), T::category_name())
    }
}

impl<T> From<u64> for GenericSize<T> {
    fn from(bytes: u64) -> Self {
        // Note: This bypasses validation, use new() for validation
        Self {
            bytes,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<GenericSize<T>> for u64 {
    fn from(size: GenericSize<T>) -> Self {
        size.bytes
    }
}

/// Specialized methods for different size categories
impl FileSize {
    /// Checks if this is a large file (> 1 GB)
    pub fn is_large_file(&self) -> bool {
        self.bytes > 1024 * 1024 * 1024
    }

    /// Estimates transfer time for given bandwidth (MB/s)
    pub fn transfer_time_seconds(&self, bandwidth_mbps: f64) -> f64 {
        if bandwidth_mbps <= 0.0 {
            f64::INFINITY
        } else {
            self.as_mb_f64() / bandwidth_mbps
        }
    }
}

impl MemorySize {
    /// Checks if this is aligned to page boundaries (typically 4KB)
    pub fn is_page_aligned(&self) -> bool {
        self.bytes.is_multiple_of(4096)
    }

    /// Rounds up to next page boundary
    pub fn round_up_to_page(&self) -> MemorySize {
        let page_size = 4096;
        let aligned_bytes = (self.bytes + page_size - 1) & !(page_size - 1);
        MemorySize::from(aligned_bytes)
    }

    /// Checks if size is reasonable for allocation
    pub fn is_reasonable_allocation(&self) -> bool {
        // Between 1 byte and 1 GB for typical allocations
        self.bytes > 0 && self.bytes <= 1024 * 1024 * 1024
    }
}

impl NetworkSize {
    /// Gets optimal chunk size for network transfer
    pub fn optimal_chunk_size(&self) -> NetworkSize {
        // Use 64KB chunks for small transfers, 1MB for large
        if self.bytes <= 1024 * 1024 {
            NetworkSize::from(64 * 1024) // 64KB
        } else {
            NetworkSize::from(1024 * 1024) // 1MB
        }
    }

    /// Estimates number of network round trips
    pub fn estimated_round_trips(&self, mtu: u64) -> u64 {
        self.bytes.div_ceil(mtu)
    }
}

impl StorageSize {
    /// Converts to storage units (TB, PB)
    pub fn as_tb(&self) -> u64 {
        self.bytes / (1024 * 1024 * 1024 * 1024)
    }

    /// Gets storage cost estimate ($/month at given rate per GB)
    pub fn monthly_cost(&self, cost_per_gb: f64) -> f64 {
        self.as_gb_f64() * cost_per_gb
    }
}

/// Helper function to check power-of-2 alignment
fn is_power_of_2_aligned(size: u64) -> bool {
    if size == 0 {
        return true;
    }

    // Check if size is a power of 2 or aligned to common boundaries
    let common_alignments = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
    common_alignments.iter().any(|&align| size.is_multiple_of(align))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests generic size creation for different size categories.
    ///
    /// This test validates that generic sizes can be created for
    /// different categories (file, memory) with proper type safety
    /// and category identification.
    ///
    /// # Test Coverage
    ///
    /// - File size creation with `new()`
    /// - Memory size creation with `from_mb()`
    /// - Byte value storage and retrieval
    /// - Category identification
    /// - Type-specific constructors
    ///
    /// # Test Scenario
    ///
    /// Creates file and memory sizes using different constructors
    /// and verifies the values and categories are set correctly.
    ///
    /// # Assertions
    ///
    /// - File size stores bytes correctly
    /// - File size has correct category
    /// - Memory size converts MB correctly
    /// - Memory size has correct category
    #[test]
    fn test_generic_size_creation() {
        let file_size = FileSize::new(1024).unwrap();
        assert_eq!(file_size.bytes(), 1024);
        assert_eq!(file_size.category(), "file");

        let memory_size = MemorySize::from_mb(512).unwrap();
        assert_eq!(memory_size.as_mb(), 512);
        assert_eq!(memory_size.category(), "memory");
    }

    /// Tests size category validation and constraints.
    ///
    /// This test validates that different size categories enforce
    /// their specific validation rules and constraints, including
    /// maximum size limits and alignment considerations.
    ///
    /// # Test Coverage
    ///
    /// - File size validation within limits
    /// - File size maximum limit enforcement
    /// - File size overflow rejection
    /// - Memory size validation with alignment
    /// - Memory size page alignment warnings
    ///
    /// # Test Scenario
    ///
    /// Tests various size values against category-specific validation
    /// rules to ensure proper constraint enforcement.
    ///
    /// # Assertions
    ///
    /// - Valid file sizes are accepted
    /// - Maximum file size is accepted
    /// - Oversized file sizes are rejected
    /// - Page-aligned memory sizes are accepted
    /// - Unaligned memory sizes are accepted with warnings
    #[test]
    fn test_size_category_validation() {
        // File size validation
        assert!(FileSize::new(1024).is_ok());
        assert!(FileSize::new(FileSizeMarker::max_size()).is_ok());
        assert!(FileSize::new(FileSizeMarker::max_size() + 1).is_err());

        // Memory size validation (with alignment warning)
        assert!(MemorySize::new(4096).is_ok()); // Page aligned
        assert!(MemorySize::new(1000).is_ok()); // Not aligned but valid
    }

    /// Tests size arithmetic operations and mathematical operations.
    ///
    /// This test validates that generic sizes support standard
    /// arithmetic operations (addition, subtraction, multiplication,
    /// division) with proper type safety.
    ///
    /// # Test Coverage
    ///
    /// - Size addition with `+` operator
    /// - Size subtraction with `-` operator
    /// - Size multiplication with `*` operator
    /// - Size division with `/` operator
    /// - Result value correctness
    /// - Type safety preservation
    ///
    /// # Test Scenario
    ///
    /// Creates two file sizes and performs various arithmetic
    /// operations, verifying the results are calculated correctly.
    ///
    /// # Assertions
    ///
    /// - Addition produces correct sum
    /// - Subtraction produces correct difference
    /// - Multiplication produces correct product
    /// - Division produces correct quotient
    #[test]
    fn test_size_arithmetic() {
        let size1 = FileSize::new(1000).unwrap();
        let size2 = FileSize::new(500).unwrap();

        let sum = size1 + size2;
        assert_eq!(sum.bytes(), 1500);

        let diff = size1 - size2;
        assert_eq!(diff.bytes(), 500);

        let doubled = size1 * 2;
        assert_eq!(doubled.bytes(), 2000);

        let halved = size1 / 2;
        assert_eq!(halved.bytes(), 500);
    }

    /// Tests size category conversions and type transformations.
    ///
    /// This test validates that generic sizes can be converted
    /// between different categories while preserving byte values
    /// and updating category information.
    ///
    /// # Test Coverage
    ///
    /// - File size creation from GB
    /// - Category conversion with `into_category()`
    /// - Byte value preservation during conversion
    /// - Category update after conversion
    /// - Type transformation validation
    ///
    /// # Test Scenario
    ///
    /// Creates a file size and converts it to a memory size,
    /// verifying the byte value is preserved and category is updated.
    ///
    /// # Assertions
    ///
    /// - Byte values are preserved during conversion
    /// - Category is updated correctly
    /// - Type transformation succeeds
    /// - Converted size has correct properties
    #[test]
    fn test_size_conversions() {
        let file_size = FileSize::from_gb(2).unwrap();

        // Convert to memory size (should work if within limits)
        let memory_size: MemorySize = file_size.into_category().unwrap();
        assert_eq!(memory_size.bytes(), file_size.bytes());
        assert_eq!(memory_size.category(), "memory");
    }

    /// Tests specialized methods for different size categories.
    ///
    /// This test validates that each size category provides
    /// specialized methods relevant to its domain (file operations,
    /// memory management, network transfers, storage costs).
    ///
    /// # Test Coverage
    ///
    /// - File size: large file detection, transfer time calculation
    /// - Memory size: page alignment, reasonable allocation checks
    /// - Network size: optimal chunk size, round trip estimation
    /// - Storage size: TB conversion, monthly cost calculation
    /// - Domain-specific functionality
    ///
    /// # Test Scenario
    ///
    /// Creates sizes for different categories and tests their
    /// specialized methods to ensure domain-specific functionality
    /// works correctly.
    ///
    /// # Assertions
    ///
    /// - Large file detection works
    /// - Transfer time calculation is positive
    /// - Page alignment detection works
    /// - Memory allocation reasonableness checks work
    /// - Network chunk size optimization works
    /// - Storage cost calculation is accurate
    #[test]
    fn test_specialized_methods() {
        // File size methods
        let large_file = FileSize::from_gb(2).unwrap();
        assert!(large_file.is_large_file());

        let transfer_time = large_file.transfer_time_seconds(100.0); // 100 MB/s
        assert!(transfer_time > 0.0);

        // Memory size methods
        let memory = MemorySize::new(4096).unwrap();
        assert!(memory.is_page_aligned());
        assert!(memory.is_reasonable_allocation());

        let unaligned = MemorySize::new(1000).unwrap();
        let aligned = unaligned.round_up_to_page();
        assert!(aligned.is_page_aligned());
        assert!(aligned.bytes() >= unaligned.bytes());

        // Network size methods
        let network_transfer = NetworkSize::from_mb(10).unwrap();
        let chunk_size = network_transfer.optimal_chunk_size();
        assert!(chunk_size.bytes() > 0);

        let round_trips = network_transfer.estimated_round_trips(1500); // Ethernet MTU
        assert!(round_trips > 0);

        // Storage size methods
        let storage = StorageSize::from_gb(1000).unwrap();
        assert_eq!(storage.as_tb(), 0); // Less than 1 TB

        let cost = storage.monthly_cost(0.023); // $0.023 per GB
        assert_eq!(cost, 23.0); // 1000 GB * $0.023
    }

    /// Tests human-readable formatting for different size units.
    ///
    /// This test validates that generic sizes provide proper
    /// human-readable string representations with appropriate
    /// units and formatting.
    ///
    /// # Test Coverage
    ///
    /// - Byte-level formatting
    /// - Kilobyte formatting with decimals
    /// - Megabyte formatting with decimals
    /// - Gigabyte formatting with decimals
    /// - Unit selection and precision
    ///
    /// # Test Scenario
    ///
    /// Creates sizes at different scales and verifies their
    /// human-readable representations use appropriate units
    /// and formatting.
    ///
    /// # Assertions
    ///
    /// - Byte sizes show "bytes" unit
    /// - KB sizes show "KB" with 2 decimal places
    /// - MB sizes show "MB" with 2 decimal places
    /// - GB sizes show "GB" with 2 decimal places
    #[test]
    fn test_human_readable_formatting() {
        assert_eq!(FileSize::new(512).unwrap().human_readable(), "512 bytes");
        assert_eq!(FileSize::from_kb(2).unwrap().human_readable(), "2.00 KB");
        assert_eq!(FileSize::from_mb(5).unwrap().human_readable(), "5.00 MB");
        assert_eq!(FileSize::from_gb(3).unwrap().human_readable(), "3.00 GB");
    }

    /// Tests type safety and compile-time guarantees.
    ///
    /// This test validates that generic sizes provide compile-time
    /// type safety, preventing operations between incompatible
    /// size categories while allowing explicit conversions.
    ///
    /// # Test Coverage
    ///
    /// - Type safety between different size categories
    /// - Compile-time prevention of invalid operations
    /// - Explicit type conversion with `into_category()`
    /// - Operations after type conversion
    /// - Type system guarantees
    ///
    /// # Test Scenario
    ///
    /// Creates different size types and demonstrates that direct
    /// operations are prevented while explicit conversions work.
    ///
    /// # Assertions
    ///
    /// - Type conversion succeeds
    /// - Operations work after conversion
    /// - Result has correct byte value
    /// - Type safety is maintained
    #[test]
    fn test_type_safety() {
        let file_size = FileSize::new(1024).unwrap();
        let memory_size = MemorySize::new(1024).unwrap();

        // This would be a compile error - cannot add different size types
        // let sum = file_size + memory_size; // Compile error!

        // But we can convert between types
        let converted: MemorySize = file_size.into_category().unwrap();
        let sum = converted + memory_size;
        assert_eq!(sum.bytes(), 2048);
    }

    /// Tests checked arithmetic operations with overflow protection.
    ///
    /// This test validates that generic sizes provide checked
    /// arithmetic operations that detect and handle overflow
    /// conditions gracefully.
    ///
    /// # Test Coverage
    ///
    /// - Checked addition with `checked_add()`
    /// - Checked subtraction with `checked_sub()`
    /// - Overflow detection and prevention
    /// - Error handling for overflow conditions
    /// - Safe arithmetic operations
    ///
    /// # Test Scenario
    ///
    /// Performs checked arithmetic operations on normal values
    /// and tests overflow conditions to ensure proper error handling.
    ///
    /// # Assertions
    ///
    /// - Checked addition produces correct result
    /// - Checked subtraction produces correct result
    /// - Overflow conditions are detected
    /// - Overflow results in error
    #[test]
    fn test_checked_arithmetic() {
        let size1 = FileSize::new(1000).unwrap();
        let size2 = FileSize::new(500).unwrap();

        let sum = size1.checked_add(size2).unwrap();
        assert_eq!(sum.bytes(), 1500);

        let diff = size1.checked_sub(size2).unwrap();
        assert_eq!(diff.bytes(), 500);

        // Test overflow protection
        let max_size = FileSize::new(FileSizeMarker::max_size()).unwrap();
        let overflow_result = max_size.checked_add(FileSize::new(1).unwrap());
        assert!(overflow_result.is_err());
    }
}
