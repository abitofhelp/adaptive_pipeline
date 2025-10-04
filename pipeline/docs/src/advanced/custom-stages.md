# Custom Stages

**Version:** 0.1.0
**Date:** October 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a step-by-step guide to creating custom pipeline stages, from defining the stage type through implementation, testing, and integration.

## Overview

Custom stages allow you to extend the pipeline with specialized data processing operations:

- **Data Sanitization**: Remove PII, redact sensitive information
- **Data Validation**: Enforce schemas, validate formats
- **Data Transformation**: Convert formats, restructure data
- **Data Enrichment**: Add metadata, annotations, tags
- **Custom Business Logic**: Domain-specific operations

**Key Concepts:**
- **StageType**: Enum variant identifying the stage category
- **StageConfiguration**: Parameters for stage behavior
- **Service Trait**: Domain interface defining stage operations
- **Service Implementation**: Infrastructure adapter performing the work
- **Processing Context**: Shared state for metrics and metadata

## Stage Implementation Steps

### Step 1: Define Stage Type

Add a new variant to the `StageType` enum:

```rust
// pipeline-domain/src/entities/pipeline_stage.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageType {
    Compression,
    Encryption,
    Transform,
    Checksum,
    PassThrough,

    // Custom stage type
    Sanitization,  // Data sanitization
}

impl std::fmt::Display for StageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing types ...
            StageType::Sanitization => write!(f, "sanitization"),
        }
    }
}

impl std::str::FromStr for StageType {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ... existing types ...
            "sanitization" => Ok(StageType::Sanitization),
            _ => Err(PipelineError::InvalidConfiguration(format!(
                "Unknown stage type: {}",
                s
            ))),
        }
    }
}
```

### Step 2: Define Domain Service Trait

Create a trait in the domain layer:

```rust
// pipeline-domain/src/services/sanitization_service.rs

use crate::{FileChunk, PipelineError, ProcessingContext};

/// Trait for data sanitization services
///
/// This service removes or redacts sensitive information from file chunks,
/// such as PII (personally identifiable information).
pub trait SanitizationService: Send + Sync {
    /// Sanitize a file chunk by removing sensitive data
    ///
    /// # Arguments
    ///
    /// * `chunk` - File chunk to sanitize
    /// * `context` - Processing context for metrics
    ///
    /// # Returns
    ///
    /// Sanitized chunk with sensitive data removed or redacted
    fn sanitize(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Detect sensitive patterns in chunk
    ///
    /// # Returns
    ///
    /// Count of sensitive patterns found
    fn detect_sensitive_data(
        &self,
        chunk: &FileChunk,
    ) -> Result<usize, PipelineError>;
}
```

### Step 3: Implement Infrastructure Service

Create the concrete implementation:

```rust
// pipeline/src/infrastructure/services/sanitization_service_impl.rs

use pipeline_domain::services::SanitizationService;
use pipeline_domain::{FileChunk, PipelineError, ProcessingContext};
use regex::Regex;
use std::sync::LazyLock;

/// Regular expressions for detecting sensitive data
static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap());

static SSN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap());

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap());

static CREDIT_CARD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b").unwrap());

/// Sanitization service implementation using regex patterns
pub struct RegexSanitizationService {
    redaction_placeholder: String,
}

impl RegexSanitizationService {
    pub fn new() -> Self {
        Self {
            redaction_placeholder: "[REDACTED]".to_string(),
        }
    }

    pub fn with_placeholder(placeholder: impl Into<String>) -> Self {
        Self {
            redaction_placeholder: placeholder.into(),
        }
    }

    fn redact_emails(&self, text: &str) -> String {
        EMAIL_REGEX.replace_all(text, &self.redaction_placeholder).to_string()
    }

    fn redact_ssns(&self, text: &str) -> String {
        SSN_REGEX.replace_all(text, &self.redaction_placeholder).to_string()
    }

    fn redact_phones(&self, text: &str) -> String {
        PHONE_REGEX.replace_all(text, &self.redaction_placeholder).to_string()
    }

    fn redact_credit_cards(&self, text: &str) -> String {
        CREDIT_CARD_REGEX.replace_all(text, &self.redaction_placeholder).to_string()
    }
}

impl SanitizationService for RegexSanitizationService {
    fn sanitize(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let start = std::time::Instant::now();

        // Convert chunk data to string
        let text = String::from_utf8_lossy(chunk.data());

        // Apply sanitization
        let sanitized = self.redact_emails(&text);
        let sanitized = self.redact_ssns(&sanitized);
        let sanitized = self.redact_phones(&sanitized);
        let sanitized = self.redact_credit_cards(&sanitized);

        // Update context
        let duration = start.elapsed();
        context.add_bytes_processed(chunk.data().len() as u64);
        context.record_stage_duration(duration);

        // Create sanitized chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            sanitized.into_bytes(),
        );

        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }

    fn detect_sensitive_data(
        &self,
        chunk: &FileChunk,
    ) -> Result<usize, PipelineError> {
        let text = String::from_utf8_lossy(chunk.data());

        let email_count = EMAIL_REGEX.find_iter(&text).count();
        let ssn_count = SSN_REGEX.find_iter(&text).count();
        let phone_count = PHONE_REGEX.find_iter(&text).count();
        let cc_count = CREDIT_CARD_REGEX.find_iter(&text).count();

        Ok(email_count + ssn_count + phone_count + cc_count)
    }
}

impl Default for RegexSanitizationService {
    fn default() -> Self {
        Self::new()
    }
}
```

### Step 4: Register Stage in Pipeline

Add the stage to pipeline configuration:

```rust
use pipeline_domain::entities::{PipelineStage, StageType, StageConfiguration};
use std::collections::HashMap;

// Create sanitization stage
let stage = PipelineStage::new(
    StageType::Sanitization,
    "pii-removal",
    StageConfiguration::new(
        "regex".to_string(),
        HashMap::from([
            ("placeholder".to_string(), "[REDACTED]".to_string()),
        ]),
        true,  // Parallel processing enabled
    ),
);

// Add to pipeline
pipeline.add_stage(stage)?;
```

### Step 5: Integrate with Stage Executor

Update the stage executor to handle the new stage type:

```rust
// pipeline/src/infrastructure/execution/stage_executor_impl.rs

impl StageExecutor for StageExecutorImpl {
    async fn execute(
        &self,
        stage: &PipelineStage,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        match stage.stage_type() {
            // ... existing types ...

            StageType::Sanitization => {
                let service = self.sanitization_service
                    .as_ref()
                    .ok_or_else(|| PipelineError::ServiceNotConfigured(
                        "SanitizationService not configured".to_string()
                    ))?;

                service.sanitize(chunk, context)
            }
        }
    }
}
```

## Complete Example: Data Validation Stage

Here's a complete example implementing a data validation stage:

```rust
// 1. Add StageType variant
pub enum StageType {
    // ... existing ...
    Validation,
}

// 2. Define domain trait
pub trait ValidationService: Send + Sync {
    fn validate(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;
}

// 3. Implement infrastructure service
pub struct JsonSchemaValidationService {
    schema: serde_json::Value,
}

impl JsonSchemaValidationService {
    pub fn new(schema: serde_json::Value) -> Self {
        Self { schema }
    }
}

impl ValidationService for JsonSchemaValidationService {
    fn validate(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        use jsonschema::JSONSchema;

        let start = std::time::Instant::now();

        // Parse chunk data as JSON
        let data: serde_json::Value = serde_json::from_slice(chunk.data())
            .map_err(|e| PipelineError::ValidationError(format!("Invalid JSON: {}", e)))?;

        // Validate against schema
        let compiled_schema = JSONSchema::compile(&self.schema)
            .map_err(|e| PipelineError::ValidationError(format!("Invalid schema: {}", e)))?;

        if let Err(errors) = compiled_schema.validate(&data) {
            let error_messages: Vec<String> = errors
                .map(|e| e.to_string())
                .collect();

            return Err(PipelineError::ValidationError(format!(
                "Validation failed: {}",
                error_messages.join(", ")
            )));
        }

        // Update context
        let duration = start.elapsed();
        context.add_bytes_processed(chunk.data().len() as u64);
        context.record_stage_duration(duration);

        Ok(chunk)  // Return unchanged if valid
    }
}

// 4. Usage in pipeline
let schema = serde_json::json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "age": { "type": "number", "minimum": 0 }
    },
    "required": ["name", "age"]
});

let validation_service = Arc::new(JsonSchemaValidationService::new(schema));

let stage = PipelineStage::new(
    StageType::Validation,
    "json-schema",
    StageConfiguration::new(
        "jsonschema".to_string(),
        HashMap::new(),
        false,  // Sequential validation
    ),
);

pipeline.add_stage(stage)?;
```

## Testing Custom Stages

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitization_redacts_emails() {
        let service = RegexSanitizationService::new();
        let test_data = b"Contact: user@example.com for details";

        let chunk = FileChunk::new(0, 0, test_data.to_vec());
        let mut context = ProcessingContext::new();

        let result = service.sanitize(chunk, &mut context).unwrap();

        let sanitized_text = String::from_utf8(result.data().to_vec()).unwrap();
        assert!(sanitized_text.contains("[REDACTED]"));
        assert!(!sanitized_text.contains("user@example.com"));
    }

    #[test]
    fn test_sanitization_detects_multiple_patterns() {
        let service = RegexSanitizationService::new();
        let test_data = b"Email: test@example.com, SSN: 123-45-6789, Phone: 555-123-4567";

        let chunk = FileChunk::new(0, 0, test_data.to_vec());

        let count = service.detect_sensitive_data(&chunk).unwrap();
        assert_eq!(count, 3);  // Email + SSN + Phone
    }

    #[test]
    fn test_validation_rejects_invalid_json() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });

        let service = JsonSchemaValidationService::new(schema);
        let invalid_data = b"{ \"age\": 25 }";  // Missing required "name"

        let chunk = FileChunk::new(0, 0, invalid_data.to_vec());
        let mut context = ProcessingContext::new();

        let result = service.validate(chunk, &mut context);
        assert!(result.is_err());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_custom_stage_in_pipeline() {
    // Create pipeline with custom stage
    let mut pipeline = Pipeline::new();

    let sanitization_stage = PipelineStage::new(
        StageType::Sanitization,
        "pii-removal",
        StageConfiguration::default(),
    );

    pipeline.add_stage(sanitization_stage).unwrap();

    // Process test data
    let test_data = b"User: john@example.com, SSN: 123-45-6789";
    let result = pipeline.process(test_data).await.unwrap();

    // Verify sanitization
    let output = String::from_utf8(result).unwrap();
    assert!(!output.contains("john@example.com"));
    assert!(!output.contains("123-45-6789"));
    assert!(output.contains("[REDACTED]"));
}
```

## Best Practices

### 1. Stateless Services

```rust
// ✅ Good: Stateless service (thread-safe)
pub struct MyService {
    config: MyConfig,  // Immutable configuration
}

impl MyService for MyServiceImpl {
    fn process(&self, chunk: FileChunk, context: &mut ProcessingContext)
        -> Result<FileChunk, PipelineError>
    {
        // No mutable state - safe for concurrent use
    }
}

// ❌ Bad: Stateful service (not thread-safe)
pub struct MyService {
    processed_count: usize,  // Mutable state without synchronization!
}
```

### 2. Proper Error Handling

```rust
// ✅ Good: Specific error types
fn validate(&self, chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    let data = serde_json::from_slice(chunk.data())
        .map_err(|e| PipelineError::ValidationError(format!("Invalid JSON: {}", e)))?;

    // ...
}

// ❌ Bad: Generic errors
fn validate(&self, chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    let data = serde_json::from_slice(chunk.data()).unwrap();  // Panics!
    // ...
}
```

### 3. Update Processing Context

```rust
// ✅ Good: Track metrics
fn process(&self, chunk: FileChunk, context: &mut ProcessingContext)
    -> Result<FileChunk, PipelineError>
{
    let start = std::time::Instant::now();

    // ... do work ...

    context.add_bytes_processed(chunk.data().len() as u64);
    context.record_stage_duration(start.elapsed());

    Ok(result)
}

// ❌ Bad: No metrics
fn process(&self, chunk: FileChunk, context: &mut ProcessingContext)
    -> Result<FileChunk, PipelineError>
{
    // ... do work ...
    Ok(result)  // No metrics recorded!
}
```

### 4. Preserve Chunk Metadata

```rust
// ✅ Good: Preserve metadata
let mut result = FileChunk::new(
    chunk.sequence_number(),
    chunk.file_offset(),
    processed_data,
);
result.set_metadata(chunk.metadata().clone());

// ❌ Bad: Lose metadata
let result = FileChunk::new(0, 0, processed_data);  // Lost sequence info!
```

## Related Topics

- See [Extending the Pipeline](extending.md) for overview of extension points
- See [Custom Algorithms](custom-algorithms.md) for algorithm implementation
- See [Architecture](../architecture/layers.md) for layered architecture principles

## Summary

Creating custom stages involves:

1. **Define StageType**: Add enum variant for stage category
2. **Define Service Trait**: Create domain interface in `pipeline-domain`
3. **Implement Service**: Build infrastructure adapter in `pipeline`
4. **Register Stage**: Add to pipeline configuration
5. **Integrate Executor**: Update stage executor to handle new type
6. **Test Thoroughly**: Unit and integration tests

**Key Takeaways:**
- Keep services stateless for thread safety
- Use specific error types for better diagnostics
- Update processing context with metrics
- Preserve chunk metadata through transformations
- Add comprehensive tests (unit + integration)
- Document configuration options and behavior

**Stage Development Checklist:**
- [ ] Define StageType enum variant
- [ ] Create domain service trait
- [ ] Implement infrastructure service
- [ ] Add unit tests for service
- [ ] Register in pipeline configuration
- [ ] Update stage executor
- [ ] Add integration tests
- [ ] Document usage and configuration
- [ ] Benchmark performance (if applicable)
