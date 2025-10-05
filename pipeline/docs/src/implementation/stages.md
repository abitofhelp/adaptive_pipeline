# Stage Processing

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a comprehensive overview of the stage processing architecture in the adaptive pipeline system. Stages are the fundamental building blocks that transform data as it flows through a pipeline.

---

## Table of Contents

- [Overview](#overview)
- [Stage Types](#stage-types)
- [Stage Entity](#stage-entity)
- [Stage Configuration](#stage-configuration)
- [Stage Lifecycle](#stage-lifecycle)
- [Stage Execution Model](#stage-execution-model)
- [Stage Executor Interface](#stage-executor-interface)
- [Compatibility and Ordering](#compatibility-and-ordering)
- [Resource Management](#resource-management)
- [Usage Examples](#usage-examples)
- [Performance Considerations](#performance-considerations)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Testing Strategies](#testing-strategies)
- [Next Steps](#next-steps)

---

## Overview

**Stages** are individual processing steps within a pipeline that transform file chunks as data flows from input to output. Each stage performs a specific operation such as compression, encryption, or integrity checking.

### Key Characteristics

- **Type Safety**: Strongly-typed stage operations prevent configuration errors
- **Ordering**: Explicit ordering ensures predictable execution sequence
- **Lifecycle Management**: Stages track creation and modification timestamps
- **State Management**: Stages can be enabled/disabled without removal
- **Resource Awareness**: Stages provide resource estimation and management

### Stage Processing Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                        Pipeline                             │
│                                                             │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐           │
│  │  Stage 1   │  │  Stage 2   │  │  Stage 3   │           │
│  │ Checksum   │→ │ Compress   │→ │  Encrypt   │→ Output  │
│  │ (Order 0)  │  │ (Order 1)  │  │ (Order 2)  │           │
│  └────────────┘  └────────────┘  └────────────┘           │
│        ↑               ↑               ↑                    │
│        └───────────────┴───────────────┘                    │
│              Stage Executor                                 │
└─────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Domain-Driven Design**: Stages are domain entities with identity
2. **Separation of Concerns**: Configuration separated from execution
3. **Async-First**: All operations are asynchronous for scalability
4. **Extensibility**: New stage types can be added through configuration

---

## Stage Types

The pipeline supports five distinct stage types, each optimized for different data transformation operations.

### StageType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StageType {
    /// Compression or decompression operations
    Compression,

    /// Encryption or decryption operations
    Encryption,

    /// Data transformation operations
    Transform,

    /// Checksum calculation and verification
    Checksum,

    /// Pass-through stage that doesn't modify data
    PassThrough,
}
```

### Stage Type Details

| Stage Type | Purpose | Examples | Typical Use Case |
|------------|---------|----------|------------------|
| **Compression** | Reduce data size | Brotli, Gzip, Zstd, Lz4 | Minimize storage/bandwidth |
| **Encryption** | Secure data | AES-256-GCM, ChaCha20 | Data protection |
| **Transform** | Modify structure | Format conversion | Data reshaping |
| **Checksum** | Verify integrity | SHA-256, SHA-512, Blake3 | Data validation |
| **PassThrough** | No modification | Identity transform | Testing/debugging |

### Parsing Stage Types

Stage types support case-insensitive parsing from strings:

```rust
use pipeline_domain::entities::pipeline_stage::StageType;
use std::str::FromStr;

// Parse from lowercase
let compression = StageType::from_str("compression").unwrap();
assert_eq!(compression, StageType::Compression);

// Case-insensitive parsing
let encryption = StageType::from_str("ENCRYPTION").unwrap();
assert_eq!(encryption, StageType::Encryption);

// Display format
assert_eq!(format!("{}", StageType::Checksum), "checksum");
```

### Pattern Matching

```rust
fn describe_stage(stage_type: StageType) -> &'static str {
    match stage_type {
        StageType::Compression => "Reduces data size",
        StageType::Encryption => "Secures data",
        StageType::Transform => "Modifies data structure",
        StageType::Checksum => "Verifies data integrity",
        StageType::PassThrough => "No modification",
    }
}
```

---

## Stage Entity

The `PipelineStage` is a domain entity that encapsulates a specific data transformation operation within a pipeline.

### Entity Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    id: StageId,
    name: String,
    stage_type: StageType,
    configuration: StageConfiguration,
    enabled: bool,
    order: u32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
```

### Entity Characteristics

- **Identity**: Unique `StageId` persists through configuration changes
- **Name**: Human-readable identifier (must not be empty)
- **Type**: Strongly-typed operation (Compression, Encryption, etc.)
- **Configuration**: Algorithm-specific parameters
- **Enabled Flag**: Controls execution without removal
- **Order**: Determines execution sequence (0-based)
- **Timestamps**: Track creation and modification times

### Creating a Stage

```rust
use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("level".to_string(), "6".to_string());

let config = StageConfiguration::new("brotli".to_string(), params, true);
let stage = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,
    config,
    0  // Order: execute first
).unwrap();

assert_eq!(stage.name(), "compression");
assert_eq!(stage.stage_type(), &StageType::Compression);
assert_eq!(stage.algorithm(), "brotli");
assert!(stage.is_enabled());
```

### Modifying Stage State

```rust
let mut stage = PipelineStage::new(
    "checksum".to_string(),
    StageType::Checksum,
    StageConfiguration::default(),
    0,
).unwrap();

// Disable the stage temporarily
stage.set_enabled(false);
assert!(!stage.is_enabled());

// Update configuration
let mut new_params = HashMap::new();
new_params.insert("algorithm".to_string(), "sha512".to_string());
let new_config = StageConfiguration::new("sha512".to_string(), new_params, true);
stage.update_configuration(new_config);

// Change execution order
stage.update_order(2);
assert_eq!(stage.order(), 2);

// Re-enable the stage
stage.set_enabled(true);
```

---

## Stage Configuration

Each stage has a configuration that specifies how data should be transformed.

### Configuration Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConfiguration {
    pub algorithm: String,
    pub parameters: HashMap<String, String>,
    pub parallel_processing: bool,
    pub chunk_size: Option<usize>,
}
```

### Configuration Parameters

| Field | Type | Description | Default |
|-------|------|-------------|---------|
| `algorithm` | String | Algorithm name (e.g., "brotli", "aes256gcm") | "default" |
| `parameters` | HashMap | Algorithm-specific key-value parameters | {} |
| `parallel_processing` | bool | Enable parallel chunk processing | true |
| `chunk_size` | Option\<usize\> | Custom chunk size (1KB - 100MB) | None |

### Compression Configuration

```rust
let mut params = HashMap::new();
params.insert("level".to_string(), "9".to_string());

let config = StageConfiguration::new(
    "zstd".to_string(),
    params,
    true,  // Enable parallel processing
);
```

### Encryption Configuration

```rust
let mut params = HashMap::new();
params.insert("key_size".to_string(), "256".to_string());

let config = StageConfiguration::new(
    "aes256gcm".to_string(),
    params,
    false,  // Sequential processing for encryption
);
```

### Default Configuration

```rust
let config = StageConfiguration::default();
// algorithm: "default"
// parameters: {}
// parallel_processing: true
// chunk_size: None
```

---

## Stage Lifecycle

Stages progress through several lifecycle phases from creation to execution.

### Lifecycle Phases

```text
1. Creation
   ↓
2. Configuration
   ↓
3. Ordering
   ↓
4. Execution
   ↓
5. Monitoring
```

### 1. Creation Phase

Stages are created with initial configuration:

```rust
let stage = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,
    StageConfiguration::default(),
    0,
)?;
```

### 2. Configuration Phase

Parameters can be updated as needed:

```rust
stage.update_configuration(new_config);
// updated_at timestamp is automatically updated
```

### 3. Ordering Phase

Position in pipeline can be adjusted:

```rust
stage.update_order(1);
// Stage now executes second instead of first
```

### 4. Execution Phase

Stage processes data according to its configuration:

```rust
let executor: Arc<dyn StageExecutor> = /* ... */;
let result = executor.execute(&stage, chunk, &mut context).await?;
```

### 5. Monitoring Phase

Timestamps track when changes occur:

```rust
println!("Created: {}", stage.created_at());
println!("Last modified: {}", stage.updated_at());
```

---

## Stage Execution Model

The stage executor processes file chunks through configured stages using two primary execution modes.

### Single Chunk Processing

Process individual chunks sequentially:

```rust
async fn execute(
    &self,
    stage: &PipelineStage,
    chunk: FileChunk,
    context: &mut ProcessingContext,
) -> Result<FileChunk, PipelineError>
```

**Execution Flow:**

```text
Input Chunk → Validate → Process → Update Context → Output Chunk
```

### Parallel Processing

Process multiple chunks concurrently:

```rust
async fn execute_parallel(
    &self,
    stage: &PipelineStage,
    chunks: Vec<FileChunk>,
    context: &mut ProcessingContext,
) -> Result<Vec<FileChunk>, PipelineError>
```

**Execution Flow:**

```text
Chunks: [1, 2, 3, 4]
         ↓  ↓  ↓  ↓
      ┌────┬───┬───┬────┐
      │ T1 │T2 │T3 │ T4 │  (Parallel threads)
      └────┴───┴───┴────┘
         ↓  ↓  ↓  ↓
Results: [1, 2, 3, 4]
```

### Processing Context

The `ProcessingContext` maintains state during execution:

```rust
pub struct ProcessingContext {
    pub pipeline_id: String,
    pub stage_metrics: HashMap<String, StageMetrics>,
    pub checksums: HashMap<String, Vec<u8>>,
    // ... other context fields
}
```

---

## Stage Executor Interface

The `StageExecutor` trait defines the contract for stage execution engines.

### Trait Definition

```rust
#[async_trait]
pub trait StageExecutor: Send + Sync {
    /// Execute a stage on a single chunk
    async fn execute(
        &self,
        stage: &PipelineStage,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Execute a stage on multiple chunks in parallel
    async fn execute_parallel(
        &self,
        stage: &PipelineStage,
        chunks: Vec<FileChunk>,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Validate if a stage can be executed
    async fn can_execute(&self, stage: &PipelineStage) -> Result<bool, PipelineError>;

    /// Get supported stage types
    fn supported_stage_types(&self) -> Vec<String>;

    /// Estimate processing time for a stage
    async fn estimate_processing_time(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<std::time::Duration, PipelineError>;

    /// Get resource requirements for a stage
    async fn get_resource_requirements(
        &self,
        stage: &PipelineStage,
        data_size: u64,
    ) -> Result<ResourceRequirements, PipelineError>;
}
```

### BasicStageExecutor Implementation

The infrastructure layer provides a concrete implementation:

```rust
pub struct BasicStageExecutor {
    checksums: Arc<RwLock<HashMap<String, Sha256>>>,
    compression_service: Arc<dyn CompressionService>,
    encryption_service: Arc<dyn EncryptionService>,
}

impl BasicStageExecutor {
    pub fn new(
        compression_service: Arc<dyn CompressionService>,
        encryption_service: Arc<dyn EncryptionService>,
    ) -> Self {
        Self {
            checksums: Arc::new(RwLock::new(HashMap::new())),
            compression_service,
            encryption_service,
        }
    }
}
```

### Supported Stage Types

The `BasicStageExecutor` supports:

- **Compression**: Via `CompressionService` (Brotli, Gzip, Zstd, Lz4)
- **Encryption**: Via `EncryptionService` (AES-256-GCM, ChaCha20-Poly1305)
- **Checksum**: Via internal SHA-256 implementation

---

## Compatibility and Ordering

Stages have compatibility rules that ensure optimal pipeline performance.

### Recommended Ordering

```text
1. Input Checksum (automatic)
   ↓
2. Compression (reduces data size)
   ↓
3. Encryption (secures compressed data)
   ↓
4. Output Checksum (automatic)
```

**Rationale:**
- Compress before encrypting to reduce encrypted payload size
- Checksum before compression to detect input corruption early
- Checksum after encryption to verify output integrity

### Compatibility Matrix

```text
From \ To      | Compression | Encryption | Checksum | PassThrough | Transform
---------------|-------------|------------|----------|-------------|----------
Compression    | ❌ No       | ✅ Yes     | ✅ Yes   | ✅ Yes      | ⚠️ Rare
Encryption     | ❌ No       | ❌ No      | ✅ Yes   | ✅ Yes      | ❌ No
Checksum       | ✅ Yes      | ✅ Yes     | ✅ Yes   | ✅ Yes      | ✅ Yes
PassThrough    | ✅ Yes      | ✅ Yes     | ✅ Yes   | ✅ Yes      | ✅ Yes
Transform      | ✅ Yes      | ✅ Yes     | ✅ Yes   | ✅ Yes      | ⚠️ Depends
```

**Legend:**
- ✅ Yes: Recommended combination
- ❌ No: Not recommended (avoid duplication or inefficiency)
- ⚠️ Rare/Depends: Context-dependent

### Checking Compatibility

```rust
let compression = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,
    StageConfiguration::default(),
    0,
).unwrap();

let encryption = PipelineStage::new(
    "encryption".to_string(),
    StageType::Encryption,
    StageConfiguration::default(),
    1,
).unwrap();

// Compression should come before encryption
assert!(compression.is_compatible_with(&encryption));
```

### Compatibility Rules

The `is_compatible_with` method implements these rules:

1. **Compression → Encryption**: ✅ Compress first, then encrypt
2. **Compression → Compression**: ❌ Avoid double compression
3. **Encryption → Encryption**: ❌ Avoid double encryption
4. **Encryption → Compression**: ❌ Cannot compress encrypted data effectively
5. **PassThrough → Any**: ✅ No restrictions
6. **Checksum → Any**: ✅ Checksums compatible with everything

---

## Resource Management

Stages provide resource estimation and requirements to enable efficient execution planning.

### Resource Requirements

```rust
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub memory_bytes: u64,
    pub cpu_cores: u32,
    pub disk_space_bytes: u64,
    pub network_bandwidth_bps: Option<u64>,
    pub gpu_memory_bytes: Option<u64>,
    pub estimated_duration: std::time::Duration,
}
```

### Default Requirements

```rust
ResourceRequirements::default()
// memory_bytes: 64 MB
// cpu_cores: 1
// disk_space_bytes: 0
// network_bandwidth_bps: None
// gpu_memory_bytes: None
// estimated_duration: 1 second
```

### Custom Requirements

```rust
let requirements = ResourceRequirements::new(
    128 * 1024 * 1024,  // 128 MB memory
    4,                   // 4 CPU cores
    1024 * 1024 * 1024, // 1 GB disk space
)
.with_duration(Duration::from_secs(30))
.with_network_bandwidth(100_000_000); // 100 Mbps
```

### Estimating Resources

```rust
let executor: Arc<dyn StageExecutor> = /* ... */;
let requirements = executor.get_resource_requirements(
    &stage,
    10 * 1024 * 1024,  // 10 MB data size
).await?;

println!("Memory required: {}", Byte::from_bytes(requirements.memory_bytes));
println!("CPU cores: {}", requirements.cpu_cores);
println!("Estimated time: {:?}", requirements.estimated_duration);
```

### Scaling Requirements

```rust
let mut requirements = ResourceRequirements::default();
requirements.scale(2.0);  // Double all requirements
```

### Merging Requirements

```rust
let mut req1 = ResourceRequirements::default();
let req2 = ResourceRequirements::new(256_000_000, 2, 0);
req1.merge(&req2);  // Takes maximum of each field
```

---

## Usage Examples

### Example 1: Creating a Compression Stage

```rust
use pipeline_domain::entities::pipeline_stage::{PipelineStage, StageConfiguration, StageType};
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("level".to_string(), "9".to_string());

let config = StageConfiguration::new(
    "zstd".to_string(),
    params,
    true,  // Enable parallel processing
);

let compression_stage = PipelineStage::new(
    "fast-compression".to_string(),
    StageType::Compression,
    config,
    1,  // Execute after input checksum (order 0)
)?;

println!("Created stage: {}", compression_stage.name());
println!("Algorithm: {}", compression_stage.algorithm());
```

### Example 2: Creating an Encryption Stage

```rust
let mut params = HashMap::new();
params.insert("key_size".to_string(), "256".to_string());

let config = StageConfiguration::new(
    "aes256gcm".to_string(),
    params,
    false,  // Sequential processing for security
);

let encryption_stage = PipelineStage::new(
    "secure-encryption".to_string(),
    StageType::Encryption,
    config,
    2,  // Execute after compression
)?;
```

### Example 3: Building a Complete Pipeline

```rust
let mut stages = Vec::new();

// Stage 0: Input checksum
let checksum_in = PipelineStage::new(
    "input-checksum".to_string(),
    StageType::Checksum,
    StageConfiguration::new("sha256".to_string(), HashMap::new(), true),
    0,
)?;
stages.push(checksum_in);

// Stage 1: Compression
let mut compress_params = HashMap::new();
compress_params.insert("level".to_string(), "6".to_string());
let compression = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,
    StageConfiguration::new("brotli".to_string(), compress_params, true),
    1,
)?;
stages.push(compression);

// Stage 2: Encryption
let mut encrypt_params = HashMap::new();
encrypt_params.insert("key_size".to_string(), "256".to_string());
let encryption = PipelineStage::new(
    "encryption".to_string(),
    StageType::Encryption,
    StageConfiguration::new("aes256gcm".to_string(), encrypt_params, false),
    2,
)?;
stages.push(encryption);

// Stage 3: Output checksum
let checksum_out = PipelineStage::new(
    "output-checksum".to_string(),
    StageType::Checksum,
    StageConfiguration::new("sha256".to_string(), HashMap::new(), true),
    3,
)?;
stages.push(checksum_out);

// Validate compatibility
for i in 0..stages.len() - 1 {
    assert!(stages[i].is_compatible_with(&stages[i + 1]));
}
```

### Example 4: Executing a Stage

```rust
use pipeline_domain::repositories::stage_executor::StageExecutor;

let executor: Arc<dyn StageExecutor> = /* ... */;
let stage = /* ... */;
let chunk = FileChunk::new(0, vec![1, 2, 3, 4, 5]);
let mut context = ProcessingContext::new("pipeline-123");

// Execute single chunk
let result = executor.execute(&stage, chunk, &mut context).await?;

println!("Processed {} bytes", result.data().len());
```

### Example 5: Parallel Execution

```rust
let chunks = vec![
    FileChunk::new(0, vec![1, 2, 3]),
    FileChunk::new(1, vec![4, 5, 6]),
    FileChunk::new(2, vec![7, 8, 9]),
];

let results = executor.execute_parallel(&stage, chunks, &mut context).await?;

println!("Processed {} chunks", results.len());
```

---

## Performance Considerations

### Chunk Size Selection

Chunk size significantly impacts stage performance:

| Data Size | Recommended Chunk Size | Rationale |
|-----------|------------------------|-----------|
| < 10 MB | 1 MB | Minimize overhead |
| 10-100 MB | 2-4 MB | Balance memory/IO |
| 100 MB - 1 GB | 4-8 MB | Optimize parallelization |
| > 1 GB | 8-16 MB | Maximize throughput |

```rust
let mut config = StageConfiguration::default();
config.chunk_size = Some(4 * 1024 * 1024);  // 4 MB chunks
```

### Parallel Processing

Enable parallel processing for CPU-bound operations:

```rust
// Compression: parallel processing beneficial
let compress_config = StageConfiguration::new(
    "zstd".to_string(),
    HashMap::new(),
    true,  // Enable parallel
);

// Encryption: sequential often better for security
let encrypt_config = StageConfiguration::new(
    "aes256gcm".to_string(),
    HashMap::new(),
    false,  // Disable parallel
);
```

### Stage Ordering Impact

**Optimal:**
```text
Checksum → Compress (6:1 ratio) → Encrypt → Checksum
1 GB → 1 GB → 167 MB → 167 MB → 167 MB
```

**Suboptimal:**
```text
Checksum → Encrypt → Compress (1.1:1 ratio) → Checksum
1 GB → 1 GB → 1 GB → 909 MB → 909 MB
```

Encrypting before compression reduces compression ratio from 6:1 to 1.1:1.

### Memory Usage

Per-stage memory usage:

| Stage Type | Memory per Chunk | Notes |
|------------|------------------|-------|
| Compression | 2-3x chunk size | Compression buffers |
| Encryption | 1-1.5x chunk size | Encryption overhead |
| Checksum | ~256 bytes | Hash state only |
| PassThrough | 1x chunk size | No additional memory |

### CPU Utilization

CPU-intensive stages:

1. **Compression**: High CPU usage (especially Brotli level 9+)
2. **Encryption**: Moderate CPU usage (AES-NI acceleration helps)
3. **Checksum**: Low CPU usage (Blake3 faster than SHA-256)

---

## Best Practices

### 1. Stage Naming

Use descriptive, kebab-case names:

```rust
// ✅ Good
"input-checksum", "fast-compression", "secure-encryption"

// ❌ Bad
"stage1", "s", "MyStage"
```

### 2. Configuration Validation

Always validate configurations:

```rust
let stage = PipelineStage::new(/* ... */)?;
stage.validate()?;  // Validate before execution
```

### 3. Optimal Ordering

Follow the recommended order:

```text
1. Input Checksum
2. Compression
3. Encryption
4. Output Checksum
```

### 4. Enable/Disable vs. Remove

Prefer disabling over removing stages:

```rust
// ✅ Good: Preserve configuration
stage.set_enabled(false);

// ❌ Bad: Lose configuration
stages.retain(|s| s.name() != "compression");
```

### 5. Resource Estimation

Estimate resources before execution:

```rust
let requirements = executor.get_resource_requirements(&stage, file_size).await?;

if requirements.memory_bytes > available_memory {
    // Adjust chunk size or process sequentially
}
```

### 6. Error Handling

Handle stage-specific errors appropriately:

```rust
match executor.execute(&stage, chunk, &mut context).await {
    Ok(result) => { /* success */ },
    Err(PipelineError::CompressionFailed(msg)) => {
        // Handle compression errors
    },
    Err(PipelineError::EncryptionFailed(msg)) => {
        // Handle encryption errors
    },
    Err(e) => {
        // Handle generic errors
    },
}
```

### 7. Monitoring

Track stage execution metrics:

```rust
let start = Instant::now();
let result = executor.execute(&stage, chunk, &mut context).await?;
let duration = start.elapsed();

println!("Stage '{}' processed {} bytes in {:?}",
    stage.name(),
    result.data().len(),
    duration
);
```

### 8. Testing

Test stages in isolation:

```rust
#[tokio::test]
async fn test_compression_stage() {
    let stage = create_compression_stage();
    let executor = create_test_executor();
    let chunk = FileChunk::new(0, vec![0u8; 1024]);
    let mut context = ProcessingContext::new("test");

    let result = executor.execute(&stage, chunk, &mut context).await.unwrap();

    assert!(result.data().len() < 1024);  // Compression worked
}
```

---

## Troubleshooting

### Issue 1: Stage Validation Fails

**Symptom:**
```text
Error: InvalidConfiguration("Stage name cannot be empty")
```

**Solution:**
```rust
// Ensure stage name is not empty
let stage = PipelineStage::new(
    "my-stage".to_string(),  // ✅ Non-empty name
    stage_type,
    config,
    order,
)?;
```

### Issue 2: Incompatible Stage Order

**Symptom:**
```text
Error: IncompatibleStages("Cannot encrypt before compressing")
```

**Solution:**
```rust
// Check compatibility before adding stages
if !previous_stage.is_compatible_with(&new_stage) {
    // Reorder stages
}
```

### Issue 3: Chunk Size Validation Error

**Symptom:**
```text
Error: InvalidConfiguration("Chunk size must be between 1KB and 100MB")
```

**Solution:**
```rust
let mut config = StageConfiguration::default();
config.chunk_size = Some(4 * 1024 * 1024);  // ✅ 4 MB (valid range)
// config.chunk_size = Some(512);  // ❌ Too small (< 1KB)
// config.chunk_size = Some(200_000_000);  // ❌ Too large (> 100MB)
```

### Issue 4: Out of Memory During Execution

**Symptom:**
```text
Error: ResourceExhaustion("Insufficient memory for stage execution")
```

**Solution:**
```rust
// Reduce chunk size or disable parallel processing
let mut config = stage.configuration().clone();
config.chunk_size = Some(1 * 1024 * 1024);  // Reduce to 1 MB
config.parallel_processing = false;  // Disable parallel
stage.update_configuration(config);
```

### Issue 5: Stage Executor Not Found

**Symptom:**
```text
Error: ExecutorNotFound("No executor for stage type 'CustomStage'")
```

**Solution:**
```rust
// Check supported stage types
let supported = executor.supported_stage_types();
println!("Supported: {:?}", supported);

// Use a supported stage type
let stage = PipelineStage::new(
    "compression".to_string(),
    StageType::Compression,  // ✅ Supported type
    config,
    0,
)?;
```

### Issue 6: Performance Degradation

**Symptom:** Stage execution is slower than expected.

**Diagnosis:**
```rust
let requirements = executor.get_resource_requirements(&stage, file_size).await?;
let duration = executor.estimate_processing_time(&stage, file_size).await?;

println!("Expected duration: {:?}", duration);
println!("Memory needed: {}", Byte::from_bytes(requirements.memory_bytes));
```

**Solutions:**
- Enable parallel processing for compression stages
- Increase chunk size for large files
- Use faster algorithms (e.g., Lz4 instead of Brotli)
- Check system resource availability

---

## Testing Strategies

### Unit Tests

Test individual stage operations:

```rust
#[test]
fn test_stage_creation() {
    let stage = PipelineStage::new(
        "test-stage".to_string(),
        StageType::Compression,
        StageConfiguration::default(),
        0,
    );
    assert!(stage.is_ok());
}

#[test]
fn test_stage_validation() {
    let stage = PipelineStage::new(
        "".to_string(),  // Empty name
        StageType::Compression,
        StageConfiguration::default(),
        0,
    );
    assert!(stage.is_err());
}
```

### Integration Tests

Test stage execution with real executors:

```rust
#[tokio::test]
async fn test_compression_integration() {
    let compression_service = create_compression_service();
    let encryption_service = create_encryption_service();
    let executor = BasicStageExecutor::new(compression_service, encryption_service);

    let stage = create_compression_stage();
    let chunk = FileChunk::new(0, vec![0u8; 10000]);
    let mut context = ProcessingContext::new("test-pipeline");

    let result = executor.execute(&stage, chunk, &mut context).await.unwrap();

    assert!(result.data().len() < 10000);  // Verify compression
}
```

### Property-Based Tests

Test stage invariants:

```rust
#[quickcheck]
fn stage_order_preserved(order: u32) -> bool {
    let stage = PipelineStage::new(
        "test".to_string(),
        StageType::Checksum,
        StageConfiguration::default(),
        order,
    ).unwrap();

    stage.order() == order
}
```

### Compatibility Tests

Test stage compatibility matrix:

```rust
#[test]
fn test_compression_encryption_compatibility() {
    let compression = create_stage(StageType::Compression, 0);
    let encryption = create_stage(StageType::Encryption, 1);

    assert!(compression.is_compatible_with(&encryption));
    assert!(encryption.is_compatible_with(&create_stage(StageType::Checksum, 2)));
}
```

---

## Next Steps

After understanding stage processing fundamentals, explore specific implementations:

### Detailed Stage Implementations

1. **[Compression](compression.md)**: Deep dive into compression algorithms and performance tuning
2. **[Encryption](encryption.md)**: Encryption implementation, key management, and security considerations
3. **[Integrity Checking](integrity.md)**: Checksum algorithms and verification strategies

### Related Topics

- **[Data Persistence](persistence.md)**: How stages are persisted and retrieved from the database
- **[File I/O](file-io.md)**: File chunking and binary format for stage data
- **[Observability](observability.md)**: Monitoring stage execution and performance

### Advanced Topics

- **[Concurrency Model](../advanced/concurrency.md)**: Parallel stage execution and thread pooling
- **[Performance Optimization](../advanced/performance.md)**: Benchmarking and profiling stages
- **[Custom Stages](../advanced/custom-stages.md)**: Implementing custom stage types

---

## Summary

**Key Takeaways:**

1. **Stages** are the fundamental building blocks of pipelines, each performing a specific transformation
2. **Five stage types** are supported: Compression, Encryption, Transform, Checksum, PassThrough
3. **PipelineStage** is a domain entity with identity, configuration, and lifecycle management
4. **Stage compatibility** rules ensure optimal ordering (compress before encrypt)
5. **StageExecutor** trait provides async execution with resource estimation
6. **Resource management** enables efficient execution planning and monitoring
7. **Best practices** include proper naming, validation, and error handling

**Configuration File Reference:** `pipeline/src/domain/entities/pipeline_stage.rs`
**Executor Interface:** `pipeline-domain/src/repositories/stage_executor.rs:156`
**Executor Implementation:** `pipeline/src/infrastructure/repositories/stage_executor.rs:175`
