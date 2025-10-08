// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Pipeline Name Validation Tests
//!
//! Tests for pipeline name validation and normalization functionality.
//!
//! ## Test Coverage
//!
//! - Pipeline name normalization to kebab-case
//! - Special character handling and replacement
//! - Edge cases and boundary conditions
//! - Validation rules and constraints
//!
//! ## Name Normalization Rules
//!
//! - Convert to lowercase
//! - Replace separators with hyphens
//! - Handle special characters appropriately
//! - Ensure consistent kebab-case format
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test pipeline_name_validation_tests
//! ```

// Pipeline name validation functions for testing
// Note: These functions are copied here for testing purposes

/// Normalizes pipeline name to kebab-case standard.
///
/// Converts pipeline names to a consistent kebab-case format by:
/// - Converting to lowercase
/// - Replacing various separators and special characters with hyphens
/// - Ensuring consistent naming conventions
fn normalize_pipeline_name(name: &str) -> String {
    name.to_lowercase()
        // Replace common separators with hyphens
        .replace(
            [
                ' ', '_', '.', '/', '\\', ':', ';', ',', '|', '&', '+', '=', '!', '?', '*', '%', '#', '@', '$', '^',
                '(', ')', '[', ']', '{', '}', '<', '>', '"', '\'', '`', '~',
            ],
            "-",
        )
        // Remove any remaining non-alphanumeric, non-hyphen characters
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        // Clean up multiple consecutive hyphens
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Validates pipeline name according to kebab-case naming conventions
fn validate_pipeline_name(name: &str) -> anyhow::Result<String> {
    // Check for empty name
    if name.is_empty() {
        return Err(anyhow::anyhow!("Pipeline name cannot be empty"));
    }

    // Normalize to kebab-case
    let normalized = normalize_pipeline_name(name);

    // Check minimum length after normalization
    if normalized.len() < 4 {
        return Err(anyhow::anyhow!(
            "Pipeline name '{}' is too short. Minimum length is 4 characters after normalization.",
            name
        ));
    }

    // Check for reserved names
    let reserved_names = [
        "help", "list", "show", "create", "delete", "process", "restore", "validate",
    ];
    if reserved_names.contains(&normalized.as_str()) {
        return Err(anyhow::anyhow!(
            "Pipeline name '{}' is reserved. Please choose a different name.",
            name
        ));
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_pipeline_name_basic() {
        assert_eq!(normalize_pipeline_name("hello-world"), "hello-world");
        assert_eq!(normalize_pipeline_name("HelloWorld"), "helloworld");
        assert_eq!(normalize_pipeline_name("hello_world"), "hello-world");
        assert_eq!(normalize_pipeline_name("hello world"), "hello-world");
    }

    #[test]
    fn test_normalize_pipeline_name_special_characters() {
        assert_eq!(normalize_pipeline_name("hello/world\\test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello:world;test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello,world|test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello&world+test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello=world(test)"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello[world]test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello{world}test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello<world>test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello\"world'test"), "hello-world-test");
        assert_eq!(normalize_pipeline_name("hello`world~test"), "hello-world-test");
    }

    #[test]
    fn test_normalize_pipeline_name_consecutive_separators() {
        assert_eq!(normalize_pipeline_name("hello---world"), "hello-world");
        assert_eq!(normalize_pipeline_name("hello___world"), "hello-world");
        assert_eq!(normalize_pipeline_name("hello   world"), "hello-world");
        assert_eq!(normalize_pipeline_name("hello-_-world"), "hello-world");
    }

    #[test]
    fn test_normalize_pipeline_name_edge_cases() {
        assert_eq!(normalize_pipeline_name(""), "");
        assert_eq!(normalize_pipeline_name("a"), "a");
        assert_eq!(normalize_pipeline_name("123"), "123");
        assert_eq!(normalize_pipeline_name("test123"), "test123");
        assert_eq!(normalize_pipeline_name("123test"), "123test");
    }

    #[test]
    fn test_normalize_pipeline_name_unicode_and_invalid() {
        assert_eq!(normalize_pipeline_name("héllo-wörld"), "hllo-wrld"); // Non-ASCII removed
        assert_eq!(normalize_pipeline_name("hello@world#test"), "hello-world-test"); // @ and # replaced with hyphens
        assert_eq!(normalize_pipeline_name("hello$world%test"), "hello-world-test"); // $ and % replaced with hyphens
        assert_eq!(normalize_pipeline_name("hello^world*test"), "hello-world-test");
        // ^ and * replaced with hyphens
    }

    #[test]
    fn test_validate_pipeline_name_success() {
        assert_eq!(validate_pipeline_name("test-pipeline").unwrap(), "test-pipeline");
        assert_eq!(
            validate_pipeline_name("my-awesome-pipeline").unwrap(),
            "my-awesome-pipeline"
        );
        assert_eq!(validate_pipeline_name("pipeline123").unwrap(), "pipeline123");
        assert_eq!(
            validate_pipeline_name("data-processing-v2").unwrap(),
            "data-processing-v2"
        );
    }

    #[test]
    fn test_validate_pipeline_name_normalization() {
        assert_eq!(validate_pipeline_name("Test Pipeline").unwrap(), "test-pipeline");
        assert_eq!(
            validate_pipeline_name("My_Awesome_Pipeline").unwrap(),
            "my-awesome-pipeline"
        );
        assert_eq!(
            validate_pipeline_name("DATA/PROCESSING\\V2").unwrap(),
            "data-processing-v2"
        );
    }

    #[test]
    fn test_validate_pipeline_name_empty() {
        let result = validate_pipeline_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_pipeline_name_too_short() {
        let result = validate_pipeline_name("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));

        let result = validate_pipeline_name("a");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));

        // Test normalization that results in too short
        let result = validate_pipeline_name("a-b");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }

    #[test]
    fn test_validate_pipeline_name_reserved_names() {
        let reserved_names = vec![
            "help", "list", "show", "create", "delete", "process", "restore", "validate",
        ];

        for name in reserved_names {
            let result = validate_pipeline_name(name);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("reserved"));

            // Test case-insensitive
            let result = validate_pipeline_name(&name.to_uppercase());
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("reserved"));
        }
    }

    #[test]
    fn test_validate_pipeline_name_minimum_length_boundary() {
        // Exactly 4 characters should pass
        assert_eq!(validate_pipeline_name("test").unwrap(), "test");
        assert_eq!(validate_pipeline_name("abcd").unwrap(), "abcd");
        assert_eq!(validate_pipeline_name("1234").unwrap(), "1234");

        // 3 characters should fail
        let result = validate_pipeline_name("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pipeline_name_complex_normalization() {
        // Test that complex input gets properly normalized and validated
        assert_eq!(
            validate_pipeline_name("My Super Complex Pipeline Name!!!").unwrap(),
            "my-super-complex-pipeline-name"
        );

        assert_eq!(
            validate_pipeline_name("data_processing/v2.1@production").unwrap(),
            "data-processing-v2-1-production" // @ replaced with hyphen
        );

        // Test that normalization can result in reserved name
        let result = validate_pipeline_name("HELP!!!");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("reserved"));
    }

    #[test]
    fn test_validate_pipeline_name_edge_cases_after_normalization() {
        // Test names that become empty after normalization
        let result = validate_pipeline_name("!!!");
        assert!(result.is_err());

        let result = validate_pipeline_name("@#$%");
        assert!(result.is_err());

        // Test names that become too short after normalization
        let result = validate_pipeline_name("a!@#b");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }
}
