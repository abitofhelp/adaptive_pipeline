//! # Stage Type Tests
//!
//! Unit tests for the StageType enum and related functionality.
//!
//! ## Test Coverage
//!
//! - **Display Formatting**: String representation of stage types
//! - **String Parsing**: Conversion from string to StageType
//! - **Validation**: Stage type validation and error handling
//!
//! ## Stage Types
//!
//! - `Compression`: Data compression stage
//! - `Encryption`: Data encryption stage
//! - `Checksum`: Data integrity verification stage
//! - `PassThrough`: No-operation stage
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test stage_type_tests
//! ```

use pipeline_domain::entities::{PipelineStage, StageConfiguration, StageType};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the Display trait implementation for StageType.
    ///
    /// Verifies that each stage type produces the correct string representation
    /// when formatted using the Display trait.
    #[test]
    fn test_stage_type_display() {
        assert_eq!(format!("{}", StageType::Compression), "compression");
        assert_eq!(format!("{}", StageType::Encryption), "encryption");
        assert_eq!(format!("{}", StageType::Checksum), "checksum");
        assert_eq!(format!("{}", StageType::PassThrough), "passthrough");
    }

    /// Tests string parsing into StageType enum values.
    ///
    /// Verifies that:
    /// - Standard stage type names parse correctly
    /// - Case sensitivity is handled appropriately
    #[test]
    fn test_stage_type_from_str() {
        assert_eq!("compression".parse::<StageType>().unwrap(), StageType::Compression);
        assert_eq!("encryption".parse::<StageType>().unwrap(), StageType::Encryption);
        assert_eq!("checksum".parse::<StageType>().unwrap(), StageType::Checksum);
        assert_eq!("passthrough".parse::<StageType>().unwrap(), StageType::PassThrough);
    }

    #[test]
    fn test_stage_type_from_str_invalid() {
        assert!("invalid".parse::<StageType>().is_err());
        assert!("".parse::<StageType>().is_err());
        // The implementation uses to_lowercase() so it's NOT case sensitive
        assert_eq!("COMPRESSION".parse::<StageType>().unwrap(), StageType::Compression);
        assert_eq!("Encryption".parse::<StageType>().unwrap(), StageType::Encryption);
    }

    #[test]
    fn test_stage_compatibility_compression() {
        let compression_stage = create_test_stage("comp1", StageType::Compression, "brotli");
        let encryption_stage = create_test_stage("enc1", StageType::Encryption, "aes256gcm");
        let checksum_stage = create_test_stage("check1", StageType::Checksum, "sha256");
        let passthrough_stage = create_test_stage("pass1", StageType::PassThrough, "passthrough");

        // Compression can be followed by encryption
        assert!(compression_stage.is_compatible_with(&encryption_stage));

        // Compression can be followed by checksum
        assert!(compression_stage.is_compatible_with(&checksum_stage));

        // Compression can be followed by passthrough
        assert!(compression_stage.is_compatible_with(&passthrough_stage));

        // Compression cannot be followed by another compression
        assert!(!compression_stage.is_compatible_with(&compression_stage));
    }

    #[test]
    fn test_stage_compatibility_encryption() {
        let compression_stage = create_test_stage("comp1", StageType::Compression, "brotli");
        let encryption_stage = create_test_stage("enc1", StageType::Encryption, "aes256gcm");
        let checksum_stage = create_test_stage("check1", StageType::Checksum, "sha256");
        let passthrough_stage = create_test_stage("pass1", StageType::PassThrough, "passthrough");

        // Encryption can be followed by checksum
        assert!(encryption_stage.is_compatible_with(&checksum_stage));

        // Encryption can be followed by passthrough
        assert!(encryption_stage.is_compatible_with(&passthrough_stage));

        // Encryption CAN be followed by compression (default _ => true)
        assert!(encryption_stage.is_compatible_with(&compression_stage));

        // Encryption cannot be followed by another encryption
        assert!(!encryption_stage.is_compatible_with(&encryption_stage));
    }

    #[test]
    fn test_stage_compatibility_checksum() {
        let compression_stage = create_test_stage("comp1", StageType::Compression, "brotli");
        let encryption_stage = create_test_stage("enc1", StageType::Encryption, "aes256gcm");
        let checksum_stage = create_test_stage("check1", StageType::Checksum, "sha256");
        let passthrough_stage = create_test_stage("pass1", StageType::PassThrough, "passthrough");

        // Checksum can be followed by passthrough
        assert!(checksum_stage.is_compatible_with(&passthrough_stage));

        // Checksum CAN be followed by compression (checksum compatible with everything)
        assert!(checksum_stage.is_compatible_with(&compression_stage));

        // Checksum CAN be followed by encryption (checksum compatible with everything)
        assert!(checksum_stage.is_compatible_with(&encryption_stage));

        // Checksum can be followed by another checksum
        assert!(checksum_stage.is_compatible_with(&checksum_stage));
    }

    #[test]
    fn test_stage_compatibility_passthrough() {
        let compression_stage = create_test_stage("comp1", StageType::Compression, "brotli");
        let encryption_stage = create_test_stage("enc1", StageType::Encryption, "aes256gcm");
        let checksum_stage = create_test_stage("check1", StageType::Checksum, "sha256");
        let passthrough_stage = create_test_stage("pass1", StageType::PassThrough, "passthrough");

        // PassThrough is compatible with everything
        assert!(passthrough_stage.is_compatible_with(&compression_stage));
        assert!(passthrough_stage.is_compatible_with(&encryption_stage));
        assert!(passthrough_stage.is_compatible_with(&checksum_stage));
        assert!(passthrough_stage.is_compatible_with(&passthrough_stage));

        // Everything is compatible with PassThrough
        assert!(compression_stage.is_compatible_with(&passthrough_stage));
        assert!(encryption_stage.is_compatible_with(&passthrough_stage));
        assert!(checksum_stage.is_compatible_with(&passthrough_stage));
    }

    #[test]
    fn test_stage_creation_with_correct_types() {
        // Test that stages are created with correct types
        let compression_stage = create_test_stage("comp", StageType::Compression, "brotli");
        assert_eq!(compression_stage.stage_type(), &StageType::Compression);

        let encryption_stage = create_test_stage("enc", StageType::Encryption, "aes256gcm");
        assert_eq!(encryption_stage.stage_type(), &StageType::Encryption);

        let checksum_stage = create_test_stage("check", StageType::Checksum, "sha256");
        assert_eq!(checksum_stage.stage_type(), &StageType::Checksum);

        let passthrough_stage = create_test_stage("pass", StageType::PassThrough, "passthrough");
        assert_eq!(passthrough_stage.stage_type(), &StageType::PassThrough);
    }

    #[test]
    fn test_stage_serialization_roundtrip() {
        let original_stage = create_test_stage("test", StageType::PassThrough, "passthrough");

        // Test that stage type is preserved through serialization/deserialization
        let stage_type_str = format!("{}", original_stage.stage_type());
        let parsed_type: StageType = stage_type_str.parse().unwrap();

        assert_eq!(parsed_type, StageType::PassThrough);
        assert_eq!(parsed_type, *original_stage.stage_type());
    }

    // Helper function to create test stages
    fn create_test_stage(name: &str, stage_type: StageType, algorithm: &str) -> PipelineStage {
        let config = StageConfiguration {
            algorithm: algorithm.to_string(),
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        };

        PipelineStage::new(name.to_string(), stage_type, config, 1).unwrap()
    }
}
