# Custom Stages

**Version:** 0.1.0
**Date:** 2025-01-05
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Stable

This chapter provides a practical guide to creating custom pipeline stages using the unified `StageService` architecture. All stages—built-in and custom—implement the same traits and follow identical patterns.

## Quick Start

Creating a custom stage involves three simple steps:

1. **Implement `StageService` trait** - Define your stage's behavior
2. **Implement `FromParameters` trait** - Extract typed config from HashMap
3. **Register in service registry** - Add to `main.rs` stage_services HashMap

**That's it!** No enum modifications, no executor updates, no separate trait definitions.

## Learning by Example

The codebase includes three production-ready stages that demonstrate the complete pattern:

| Stage | File | Description | Complexity |
|-------|------|-------------|------------|
| **Base64** | `pipeline/src/infrastructure/services/base64_encoding_service.rs` | Binary-to-text encoding | ⭐ Simple |
| **PII Masking** | `pipeline/src/infrastructure/services/pii_masking_service.rs` | Privacy protection | ⭐⭐ Medium |
| **Tee** | `pipeline/src/infrastructure/services/tee_service.rs` | Data inspection/debugging | ⭐⭐ Medium |

**Recommendation:** Start by reading `base64_encoding_service.rs` (220 lines) for a complete, minimal example.

## The StageService Pattern

Every stage implements two traits:

### 1. FromParameters - Type-Safe Configuration

Extracts typed configuration from `HashMap<String, String>`:

```rust
use pipeline_domain::services::FromParameters;
use pipeline_domain::PipelineError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base64Config {
    pub variant: Base64Variant,
}

impl FromParameters for Base64Config {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        let variant = params
            .get("variant")
            .map(|s| match s.to_lowercase().as_str() {
                "standard" => Ok(Base64Variant::Standard),
                "url_safe" => Ok(Base64Variant::UrlSafe),
                other => Err(PipelineError::InvalidParameter(format!(
                    "Unknown Base64 variant: {}",
                    other
                ))),
            })
            .transpose()?
            .unwrap_or(Base64Variant::Standard);

        Ok(Self { variant })
    }
}
```

**Key Points:**
- Similar to Rust's `FromStr` trait
- Validates parameters early (before execution)
- Provides clear error messages
- Allows default values with `.unwrap_or()`

### 2. StageService - Core Processing Logic

Defines how your stage processes data:

```rust
use pipeline_domain::services::StageService;
use pipeline_domain::entities::{
    Operation, ProcessingContext, StageConfiguration, StagePosition, StageType,
};
use pipeline_domain::value_objects::file_chunk::FileChunk;
use pipeline_domain::PipelineError;

pub struct Base64EncodingService;

impl StageService for Base64EncodingService {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        // 1. Extract typed config
        let base64_config = Base64Config::from_parameters(&config.parameters)?;

        // 2. Process based on operation
        let processed_data = match config.operation {
            Operation::Forward => self.encode(chunk.data(), base64_config.variant),
            Operation::Reverse => self.decode(chunk.data(), base64_config.variant)?,
        };

        // 3. Return processed chunk
        chunk.with_data(processed_data)
    }

    fn position(&self) -> StagePosition {
        StagePosition::PreBinary  // Before compression/encryption
    }

    fn is_reversible(&self) -> bool {
        true  // Can encode AND decode
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }
}
```

**Key Points:**
- `process_chunk()` - The core processing logic
- `position()` - Where in pipeline (PreBinary/PostBinary/Any)
- `is_reversible()` - Whether stage supports reverse operation
- `stage_type()` - Category (Transform/Compression/Encryption/etc.)

## Stage Position (Binary Boundary)

The `position()` method enforces ordering constraints:

```
┌──────────────┬─────────────┬──────────────┐
│  PreBinary   │   Binary    │  PostBinary  │
│   Stages     │  Boundary   │    Stages    │
├──────────────┼─────────────┼──────────────┤
│              │             │              │
│ Base64       │ Compression │              │
│ PII Masking  │ Encryption  │              │
│              │             │              │
└──────────────┴─────────────┴──────────────┘
     ↑                             ↑
     └─ Sees readable data         └─ Sees binary data
```

**PreBinary:**
- Executes BEFORE compression/encryption
- Sees readable, original data format
- Examples: Base64, PII Masking, format conversion

**PostBinary:**
- Executes AFTER compression/encryption
- Sees final binary output
- Examples: checksumming, digital signatures

**Any:**
- Can execute anywhere in pipeline
- Typically diagnostic/observability stages
- Examples: Tee (data inspection), metrics

## Complete Example: Custom ROT13 Stage

Here's a minimal custom stage (rot13 cipher):

```rust
// pipeline/src/infrastructure/services/rot13_service.rs

use pipeline_domain::entities::{
    Operation, ProcessingContext, StageConfiguration, StagePosition, StageType,
};
use pipeline_domain::services::{FromParameters, StageService};
use pipeline_domain::value_objects::file_chunk::FileChunk;
use pipeline_domain::PipelineError;
use std::collections::HashMap;

/// Configuration for ROT13 cipher (no parameters needed)
#[derive(Debug, Clone, Default)]
pub struct Rot13Config;

impl FromParameters for Rot13Config {
    fn from_parameters(_params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        Ok(Self)  // No parameters to parse
    }
}

/// ROT13 cipher service (simple character rotation)
pub struct Rot13Service;

impl Rot13Service {
    pub fn new() -> Self {
        Self
    }

    fn rot13_byte(b: u8) -> u8 {
        match b {
            b'A'..=b'Z' => ((b - b'A' + 13) % 26) + b'A',
            b'a'..=b'z' => ((b - b'a' + 13) % 26) + b'a',
            _ => b,
        }
    }
}

impl StageService for Rot13Service {
    fn process_chunk(
        &self,
        chunk: FileChunk,
        config: &StageConfiguration,
        _context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let _ = Rot13Config::from_parameters(&config.parameters)?;

        // ROT13 is self-inverse: same operation for Forward and Reverse
        let processed: Vec<u8> = chunk.data()
            .iter()
            .map(|&b| Self::rot13_byte(b))
            .collect();

        chunk.with_data(processed)
    }

    fn position(&self) -> StagePosition {
        StagePosition::PreBinary  // Must execute before encryption
    }

    fn is_reversible(&self) -> bool {
        true  // ROT13 is self-inverse
    }

    fn stage_type(&self) -> StageType {
        StageType::Transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_roundtrip() {
        let service = Rot13Service::new();
        let chunk = FileChunk::new(0, 0, b"Hello, World!".to_vec(), false).unwrap();
        let config = StageConfiguration::default();
        let mut context = ProcessingContext::default();

        // Encode
        let encoded = service.process_chunk(chunk, &config, &mut context).unwrap();
        assert_eq!(encoded.data(), b"Uryyb, Jbeyq!");

        // Decode (same operation)
        let decoded = service.process_chunk(encoded, &config, &mut context).unwrap();
        assert_eq!(decoded.data(), b"Hello, World!");
    }
}
```

## Registration and Usage

### 1. Export from Module

```rust
// pipeline/src/infrastructure/services/mod.rs

pub mod base64_encoding_service;
pub mod pii_masking_service;
pub mod tee_service;
pub mod rot13_service;  // Add your service

pub use base64_encoding_service::Base64EncodingService;
pub use pii_masking_service::PiiMaskingService;
pub use tee_service::TeeService;
pub use rot13_service::Rot13Service;  // Export
```

### 2. Register in main.rs

```rust
// pipeline/src/main.rs

use pipeline::infrastructure::services::{
    Base64EncodingService, PiiMaskingService, TeeService, Rot13Service,
};

// Build stage service registry
let mut stage_services: HashMap<String, Arc<dyn StageService>> = HashMap::new();

// Register built-in stages
stage_services.insert("brotli".to_string(), compression_service.clone());
stage_services.insert("aes256gcm".to_string(), encryption_service.clone());

// Register production transform stages
stage_services.insert("base64".to_string(), Arc::new(Base64EncodingService::new()));
stage_services.insert("pii_masking".to_string(), Arc::new(PiiMaskingService::new()));
stage_services.insert("tee".to_string(), Arc::new(TeeService::new()));

// Register your custom stage
stage_services.insert("rot13".to_string(), Arc::new(Rot13Service::new()));

// Create stage executor with registry
let stage_executor = Arc::new(BasicStageExecutor::new(stage_services));
```

### 3. Use in Pipeline Configuration

```rust
use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
use pipeline_domain::entities::Operation;
use std::collections::HashMap;

// Create ROT13 stage
let rot13_stage = PipelineStage::new(
    "rot13_encode".to_string(),
    StageType::Transform,
    StageConfiguration {
        algorithm: "rot13".to_string(),  // Matches registry key
        operation: Operation::Forward,
        parameters: HashMap::new(),       // No parameters needed
        parallel_processing: false,
        chunk_size: None,
    },
    1,  // Order
).unwrap();

// Add to pipeline
pipeline.add_stage(rot13_stage)?;
```

## Design Patterns and Best Practices

### Pattern 1: Parameterless Stages

For stages without configuration:

```rust
#[derive(Debug, Clone, Default)]
pub struct MyConfig;

impl FromParameters for MyConfig {
    fn from_parameters(_params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        Ok(Self)  // Always succeeds, no validation needed
    }
}
```

### Pattern 2: Optional Parameters with Defaults

```rust
impl FromParameters for TeeConfig {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        // Required parameter
        let output_path = params
            .get("output_path")
            .ok_or_else(|| PipelineError::MissingParameter("output_path".into()))?;

        // Optional with default
        let format = params
            .get("format")
            .map(|s| TeeFormat::from_str(s))
            .transpose()?
            .unwrap_or(TeeFormat::Binary);

        Ok(Self { output_path: output_path.into(), format })
    }
}
```

### Pattern 3: Non-Reversible Stages

For one-way operations:

```rust
impl StageService for PiiMaskingService {
    fn process_chunk(...) -> Result<FileChunk, PipelineError> {
        match config.operation {
            Operation::Forward => {
                // Mask PII
                self.mask_data(chunk.data(), &pii_config)?
            }
            Operation::Reverse => {
                // Cannot unmask - data is destroyed
                return Err(PipelineError::ProcessingFailed(
                    "PII masking is not reversible".to_string()
                ));
            }
        }
    }

    fn is_reversible(&self) -> bool {
        false  // Important: declares non-reversibility
    }
}
```

### Pattern 4: Lazy Static Resources

For expensive initialization (regex, schemas, etc.):

```rust
use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b")
        .expect("Invalid email regex")
});

impl StageService for PiiMaskingService {
    fn process_chunk(...) -> Result<FileChunk, PipelineError> {
        // Regex compiled once, reused for all chunks
        let masked = EMAIL_REGEX.replace_all(&text, "***");
        // ...
    }
}
```

## Testing Your Stage

### Unit Tests

Test your service in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pipeline_domain::entities::{SecurityContext, SecurityLevel};
    use std::path::PathBuf;

    #[test]
    fn test_config_parsing() {
        let mut params = HashMap::new();
        params.insert("variant".to_string(), "url_safe".to_string());

        let config = Base64Config::from_parameters(&params).unwrap();
        assert_eq!(config.variant, Base64Variant::UrlSafe);
    }

    #[test]
    fn test_forward_operation() {
        let service = Base64EncodingService::new();
        let chunk = FileChunk::new(0, 0, b"Hello, World!".to_vec(), false).unwrap();
        let config = StageConfiguration::default();
        let mut context = ProcessingContext::new(
            PathBuf::from("/tmp/in"),
            PathBuf::from("/tmp/out"),
            100,
            SecurityContext::new(None, SecurityLevel::Public),
        );

        let result = service.process_chunk(chunk, &config, &mut context).unwrap();

        // Base64 of "Hello, World!" is "SGVsbG8sIFdvcmxkIQ=="
        assert_eq!(result.data(), b"SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_roundtrip() {
        let service = Base64EncodingService::new();
        let original = b"Test data";
        let chunk = FileChunk::new(0, 0, original.to_vec(), false).unwrap();

        let mut config = StageConfiguration::default();
        let mut context = ProcessingContext::default();

        // Encode
        config.operation = Operation::Forward;
        let encoded = service.process_chunk(chunk, &config, &mut context).unwrap();

        // Decode
        config.operation = Operation::Reverse;
        let decoded = service.process_chunk(encoded, &config, &mut context).unwrap();

        assert_eq!(decoded.data(), original);
    }
}
```

### Integration Tests

Test stage within pipeline:

```rust
#[tokio::test]
async fn test_rot13_in_pipeline() {
    // Build stage service registry
    let mut stage_services = HashMap::new();
    stage_services.insert("rot13".to_string(),
        Arc::new(Rot13Service::new()) as Arc<dyn StageService>);

    let stage_executor = Arc::new(BasicStageExecutor::new(stage_services));

    // Create pipeline with ROT13 stage
    let rot13_stage = PipelineStage::new(
        "rot13".to_string(),
        StageType::Transform,
        StageConfiguration {
            algorithm: "rot13".to_string(),
            operation: Operation::Forward,
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        },
        1,
    ).unwrap();

    let pipeline = Pipeline::new("test".to_string(), vec![rot13_stage]).unwrap();

    // Execute
    let chunk = FileChunk::new(0, 0, b"Hello!".to_vec(), false).unwrap();
    let mut context = ProcessingContext::default();

    let result = stage_executor
        .execute(pipeline.stages()[1], chunk, &mut context)  // [1] skips input_checksum
        .await
        .unwrap();

    assert_eq!(result.data(), b"Uryyb!");
}
```

## Real-World Examples

### Production Stages in Codebase

Study these files for complete, tested implementations:

1. **base64_encoding_service.rs** (220 lines)
   - Simple reversible transformation
   - Multiple config variants
   - Clean error handling
   - Complete test coverage

2. **pii_masking_service.rs** (309 lines)
   - Non-reversible transformation
   - Multiple regex patterns
   - Lazy static initialization
   - Complex configuration

3. **tee_service.rs** (380 lines)
   - Pass-through with side effects
   - File I/O operations
   - Multiple output formats
   - Position::Any usage

## Common Pitfalls

### ❌ Don't: Modify Domain Enums

```rust
// WRONG: Don't add to StageType enum
pub enum StageType {
    Transform,
    MyCustomStage,  // ❌ Not needed!
}
```

**Correct:** Just use `StageType::Transform` and register with a unique algorithm name.

### ❌ Don't: Create Separate Traits

```rust
// WRONG: Don't create custom trait
pub trait MyCustomService: Send + Sync {
    fn process(...);
}
```

**Correct:** Implement `StageService` directly. All stages use the same trait.

### ❌ Don't: Manual Executor Updates

```rust
// WRONG: Don't modify stage executor
impl StageExecutor for BasicStageExecutor {
    async fn execute(...) {
        match algorithm {
            "my_stage" => // manual dispatch ❌
        }
    }
}
```

**Correct:** Just register in `stage_services` HashMap. Executor uses the registry.

## Summary

Creating custom stages with the unified architecture:

**Three Steps:**
1. Implement `StageService` trait (process_chunk, position, is_reversible, stage_type)
2. Implement `FromParameters` trait (type-safe config extraction)
3. Register in `main.rs` stage_services HashMap

**Key Benefits:**
- No enum modifications needed
- No executor changes required
- Type-safe configuration
- Automatic validation (ordering, parameters)
- Same pattern for all stages

**Learn More:**
- Read production stages: `base64_encoding_service.rs`, `pii_masking_service.rs`, `tee_service.rs`
- See `pipeline-domain/src/services/stage_service.rs` for trait definitions
- See `pipeline-domain/src/entities/pipeline_stage.rs` for StagePosition documentation

**Quick Reference:**
```rust
// Minimal custom stage template
pub struct MyService;

impl StageService for MyService {
    fn process_chunk(...) -> Result<FileChunk, PipelineError> {
        let config = MyConfig::from_parameters(&config.parameters)?;
        // ... process data ...
        chunk.with_data(processed_data)
    }
    fn position(&self) -> StagePosition { StagePosition::PreBinary }
    fn is_reversible(&self) -> bool { true }
    fn stage_type(&self) -> StageType { StageType::Transform }
}

impl FromParameters for MyConfig {
    fn from_parameters(params: &HashMap<String, String>) -> Result<Self, PipelineError> {
        // ... parse parameters ...
        Ok(Self { /* config fields */ })
    }
}
```
