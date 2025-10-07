# Extending the Pipeline

**Version:** 0.1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter explains how to extend the pipeline with custom functionality, including custom stages, algorithms, and services while maintaining architectural integrity and type safety.

## Overview

The pipeline is designed for extensibility through well-defined extension points:

1. **Custom Stages**: Add new processing stages with custom logic
2. **Custom Algorithms**: Implement new compression, encryption, or hashing algorithms
3. **Custom Services**: Create new domain services for specialized operations
4. **Custom Adapters**: Add infrastructure adapters for external systems
5. **Custom Metrics**: Extend observability with custom metrics collectors

**Design Principles:**
- **Open/Closed**: Open for extension, closed for modification
- **Dependency Inversion**: Depend on abstractions (traits), not concretions
- **Single Responsibility**: Each extension has one clear purpose
- **Type Safety**: Use strong typing to prevent errors

## Extension Points

### 1. Custom Stage Types

Add new stage types by extending the `StageType` enum.

**Current stage types:**

```rust
pub enum StageType {
    Compression,    // Data compression/decompression
    Encryption,     // Data encryption/decryption
    Transform,      // Data transformation
    Checksum,       // Integrity verification
    PassThrough,    // No modification
}
```

**Adding a new stage type:**

```rust
// 1. Extend StageType enum (in pipeline-domain/src/entities/pipeline_stage.rs)
pub enum StageType {
    Compression,
    Encryption,
    Transform,
    Checksum,
    PassThrough,

    // Custom stage types
    Sanitization,   // Data sanitization (e.g., PII removal)
    Validation,     // Data validation
    Deduplication,  // Remove duplicate chunks
}

// 2. Update Display trait
impl std::fmt::Display for StageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing types ...
            StageType::Sanitization => write!(f, "sanitization"),
            StageType::Validation => write!(f, "validation"),
            StageType::Deduplication => write!(f, "deduplication"),
        }
    }
}

// 3. Update FromStr trait
impl std::str::FromStr for StageType {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ... existing types ...
            "sanitization" => Ok(StageType::Sanitization),
            "validation" => Ok(StageType::Validation),
            "deduplication" => Ok(StageType::Deduplication),
            _ => Err(PipelineError::InvalidConfiguration(format!(
                "Unknown stage type: {}",
                s
            ))),
        }
    }
}
```

### 2. Custom Algorithms

Extend algorithm enums to support new implementations.

**Example: Custom Compression Algorithm**

```rust
// 1. Extend CompressionAlgorithm enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Brotli,
    Gzip,
    Zstd,
    Lz4,

    // Custom algorithms
    Snappy,    // Google's Snappy compression
    Lzma,      // LZMA/XZ compression
    Custom(u8), // Custom algorithm identifier
}

// 2. Implement Display trait
impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing algorithms ...
            CompressionAlgorithm::Snappy => write!(f, "snappy"),
            CompressionAlgorithm::Lzma => write!(f, "lzma"),
            CompressionAlgorithm::Custom(id) => write!(f, "custom-{}", id),
        }
    }
}

// 3. Update FromStr trait
impl std::str::FromStr for CompressionAlgorithm {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ... existing algorithms ...
            "snappy" => Ok(CompressionAlgorithm::Snappy),
            "lzma" => Ok(CompressionAlgorithm::Lzma),
            s if s.starts_with("custom-") => {
                let id = s.strip_prefix("custom-")
                    .and_then(|id| id.parse::<u8>().ok())
                    .ok_or_else(|| PipelineError::InvalidConfiguration(
                        format!("Invalid custom algorithm ID: {}", s)
                    ))?;
                Ok(CompressionAlgorithm::Custom(id))
            }
            _ => Err(PipelineError::InvalidConfiguration(format!(
                "Unknown compression algorithm: {}",
                s
            ))),
        }
    }
}
```

### 3. Custom Services

Implement domain service traits for custom functionality.

**Example: Custom Compression Service**

```rust
use adaptive_pipeline_domain::services::CompressionService;
use adaptive_pipeline_domain::{FileChunk, PipelineError, ProcessingContext};

/// Custom compression service using Snappy algorithm
pub struct SnappyCompressionService;

impl SnappyCompressionService {
    pub fn new() -> Self {
        Self
    }
}

impl CompressionService for SnappyCompressionService {
    fn compress(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        use snap::raw::Encoder;

        let start = std::time::Instant::now();

        // Compress data using Snappy
        let mut encoder = Encoder::new();
        let compressed = encoder
            .compress_vec(chunk.data())
            .map_err(|e| PipelineError::CompressionError(format!("Snappy error: {}", e)))?;

        // Update context
        let duration = start.elapsed();
        context.add_bytes_processed(chunk.data().len() as u64);
        context.record_stage_duration(duration);

        // Create compressed chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            compressed,
        );

        // Preserve metadata
        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }

    fn decompress(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        use snap::raw::Decoder;

        let start = std::time::Instant::now();

        // Decompress data using Snappy
        let mut decoder = Decoder::new();
        let decompressed = decoder
            .decompress_vec(chunk.data())
            .map_err(|e| PipelineError::DecompressionError(format!("Snappy error: {}", e)))?;

        // Update context
        let duration = start.elapsed();
        context.add_bytes_processed(decompressed.len() as u64);
        context.record_stage_duration(duration);

        // Create decompressed chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            decompressed,
        );

        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }

    fn estimate_compressed_size(&self, chunk: &FileChunk) -> usize {
        // Snappy typically compresses to ~50-70% of original
        (chunk.data().len() as f64 * 0.6) as usize
    }
}
```

### 4. Custom Adapters

Create infrastructure adapters for external systems.

**Example: Custom Cloud Storage Adapter**

```rust
use async_trait::async_trait;
use adaptive_pipeline_domain::services::FileIOService;
use std::path::Path;

/// Custom adapter for cloud storage (e.g., S3, Azure Blob)
pub struct CloudStorageAdapter {
    client: CloudClient,
    bucket: String,
}

impl CloudStorageAdapter {
    pub fn new(endpoint: &str, bucket: &str) -> Result<Self, PipelineError> {
        let client = CloudClient::connect(endpoint)?;
        Ok(Self {
            client,
            bucket: bucket.to_string(),
        })
    }
}

#[async_trait]
impl FileIOService for CloudStorageAdapter {
    async fn read_file_chunks(
        &self,
        path: &Path,
        options: ReadOptions,
    ) -> Result<Vec<FileChunk>, PipelineError> {
        // 1. Convert local path to cloud path
        let cloud_path = self.to_cloud_path(path)?;

        // 2. Read file from cloud storage
        let data = self.client
            .get_object(&self.bucket, &cloud_path)
            .await
            .map_err(|e| PipelineError::IOError(format!("Cloud read error: {}", e)))?;

        // 3. Chunk the data
        let chunk_size = options.chunk_size.unwrap_or(64 * 1024);
        let chunks = data
            .chunks(chunk_size)
            .enumerate()
            .map(|(seq, chunk)| {
                FileChunk::new(seq, seq * chunk_size, chunk.to_vec())
            })
            .collect();

        Ok(chunks)
    }

    async fn write_file_data(
        &self,
        path: &Path,
        data: &[u8],
        options: WriteOptions,
    ) -> Result<(), PipelineError> {
        // 1. Convert local path to cloud path
        let cloud_path = self.to_cloud_path(path)?;

        // 2. Write to cloud storage
        self.client
            .put_object(&self.bucket, &cloud_path, data)
            .await
            .map_err(|e| PipelineError::IOError(format!("Cloud write error: {}", e)))?;

        Ok(())
    }

    // ... implement other methods ...
}
```

### 5. Custom Metrics

Extend observability with custom metrics collectors.

**Example: Custom Metrics Collector**

```rust
use adaptive_pipeline::infrastructure::metrics::MetricsCollector;
use std::sync::atomic::{AtomicU64, Ordering};

/// Custom metrics collector for advanced analytics
pub struct AdvancedMetricsCollector {
    // Existing metrics
    chunks_processed: AtomicU64,
    bytes_processed: AtomicU64,

    // Custom metrics
    compression_ratio: AtomicU64,  // Stored as f64 bits
    deduplication_hits: AtomicU64,
    cache_efficiency: AtomicU64,   // Percentage * 100
}

impl AdvancedMetricsCollector {
    pub fn new() -> Self {
        Self {
            chunks_processed: AtomicU64::new(0),
            bytes_processed: AtomicU64::new(0),
            compression_ratio: AtomicU64::new(0),
            deduplication_hits: AtomicU64::new(0),
            cache_efficiency: AtomicU64::new(0),
        }
    }

    /// Record compression ratio for a chunk
    pub fn record_compression_ratio(&self, original: usize, compressed: usize) {
        let ratio = (compressed as f64 / original as f64) * 100.0;
        self.compression_ratio.store(ratio.to_bits(), Ordering::Relaxed);
    }

    /// Increment deduplication hits counter
    pub fn record_deduplication_hit(&self) {
        self.deduplication_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Update cache efficiency percentage
    pub fn update_cache_efficiency(&self, hits: u64, total: u64) {
        let efficiency = ((hits as f64 / total as f64) * 10000.0) as u64;
        self.cache_efficiency.store(efficiency, Ordering::Relaxed);
    }

    /// Get compression ratio
    pub fn compression_ratio(&self) -> f64 {
        f64::from_bits(self.compression_ratio.load(Ordering::Relaxed))
    }

    /// Get deduplication hits
    pub fn deduplication_hits(&self) -> u64 {
        self.deduplication_hits.load(Ordering::Relaxed)
    }

    /// Get cache efficiency percentage
    pub fn cache_efficiency_percent(&self) -> f64 {
        self.cache_efficiency.load(Ordering::Relaxed) as f64 / 100.0
    }
}

impl MetricsCollector for AdvancedMetricsCollector {
    fn record_chunk_processed(&self, size: usize) {
        self.chunks_processed.fetch_add(1, Ordering::Relaxed);
        self.bytes_processed.fetch_add(size as u64, Ordering::Relaxed);
    }

    fn chunks_processed(&self) -> u64 {
        self.chunks_processed.load(Ordering::Relaxed)
    }

    fn bytes_processed(&self) -> u64 {
        self.bytes_processed.load(Ordering::Relaxed)
    }
}
```

## Architecture Patterns

### Hexagonal Architecture

The pipeline uses hexagonal architecture (Ports and Adapters):

**Ports (Interfaces):**
- Domain Services (CompressionService, EncryptionService)
- Repositories (StageExecutor)
- Value Objects (StageType, CompressionAlgorithm)

**Adapters (Implementations):**
- Infrastructure Services (BrotliCompressionService, AesEncryptionService)
- Infrastructure Adapters (TokioFileIO, SQLitePipelineRepository)
- Application Services (ConcurrentPipeline, StreamingFileProcessor)

**Extension Strategy:**
1. Define domain trait (port)
2. Implement infrastructure adapter
3. Register with dependency injection container

### Dependency Injection

**Example: Registering Custom Services**

```rust
use std::sync::Arc;

/// Application dependencies container
pub struct AppDependencies {
    compression_service: Arc<dyn CompressionService>,
    encryption_service: Arc<dyn EncryptionService>,
    file_io_service: Arc<dyn FileIOService>,
}

impl AppDependencies {
    pub fn new_with_custom_compression(
        compression: Arc<dyn CompressionService>,
    ) -> Self {
        Self {
            compression_service: compression,
            encryption_service: Arc::new(DefaultEncryptionService::new()),
            file_io_service: Arc::new(DefaultFileIOService::new()),
        }
    }

    pub fn new_with_cloud_storage(
        cloud_adapter: Arc<CloudStorageAdapter>,
    ) -> Self {
        Self {
            compression_service: Arc::new(DefaultCompressionService::new()),
            encryption_service: Arc::new(DefaultEncryptionService::new()),
            file_io_service: cloud_adapter,
        }
    }
}
```

## Best Practices

### 1. Follow Domain-Driven Design

```rust
// ✅ Good: Domain trait in domain layer
// pipeline-domain/src/services/my_service.rs
pub trait MyService: Send + Sync {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, PipelineError>;
}

// ✅ Good: Infrastructure implementation in infrastructure layer
// pipeline/src/infrastructure/services/my_custom_service.rs
// Note: Use technology-based names (e.g., TokioFileIO, BrotliCompression)
// rather than "Impl" suffix for actual implementations
pub struct MyCustomService {
    config: MyConfig,
}

impl MyService for MyCustomService {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, PipelineError> {
        // Implementation details
    }
}

// ❌ Bad: Implementation in domain layer
// pipeline-domain/src/services/my_service.rs
pub struct MyCustomService { /* ... */ }  // Wrong layer!
```

### 2. Use Type-Safe Configuration

```rust
// ✅ Good: Strongly typed configuration
pub struct MyStageConfig {
    algorithm: MyAlgorithm,
    level: u8,
    parallel: bool,
}

impl MyStageConfig {
    pub fn new(algorithm: MyAlgorithm, level: u8) -> Result<Self, PipelineError> {
        if level > 10 {
            return Err(PipelineError::InvalidConfiguration(
                "Level must be 0-10".to_string()
            ));
        }
        Ok(Self { algorithm, level, parallel: true })
    }
}

// ❌ Bad: Stringly-typed configuration
pub struct MyStageConfig {
    algorithm: String,  // Could be anything!
    level: String,      // Not validated!
}
```

### 3. Implement Proper Error Handling

```rust
// ✅ Good: Specific error types
impl MyService for MyCustomService {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, PipelineError> {
        self.validate_input(data)
            .map_err(|e| PipelineError::ValidationError(format!("Invalid input: {}", e)))?;

        self.do_processing(data)
            .map_err(|e| PipelineError::ProcessingError(format!("Processing failed: {}", e)))?;

        Ok(result)
    }
}

// ❌ Bad: Generic errors
impl MyService for MyServiceImpl {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>, PipelineError> {
        // Just returns "something went wrong" - not helpful!
        Ok(self.do_processing(data).unwrap())
    }
}
```

### 4. Add Comprehensive Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ✅ Good: Unit tests for service
    #[test]
    fn test_custom_compression_small_data() {
        let service = SnappyCompressionService::new();
        let chunk = FileChunk::new(0, 0, vec![0u8; 1024]);
        let mut context = ProcessingContext::new();

        let result = service.compress(chunk, &mut context).unwrap();

        assert!(result.data().len() < 1024);
        assert_eq!(context.bytes_processed(), 1024);
    }

    // ✅ Good: Integration test
    #[tokio::test]
    async fn test_custom_cloud_adapter_roundtrip() {
        let adapter = CloudStorageAdapter::new("http://localhost:9000", "test").unwrap();
        let test_data = b"Hello, World!";

        adapter.write_file_data(Path::new("test.txt"), test_data, Default::default())
            .await
            .unwrap();

        let chunks = adapter.read_file_chunks(Path::new("test.txt"), Default::default())
            .await
            .unwrap();

        assert_eq!(chunks[0].data(), test_data);
    }

    // ✅ Good: Error case testing
    #[test]
    fn test_custom_compression_invalid_data() {
        let service = SnappyCompressionService::new();
        let corrupt_chunk = FileChunk::new(0, 0, vec![0xFF; 10]);
        let mut context = ProcessingContext::new();

        let result = service.decompress(corrupt_chunk, &mut context);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PipelineError::DecompressionError(_)));
    }
}
```

### 5. Document Extension Points

```rust
/// Custom sanitization service for PII removal
///
/// This service implements data sanitization by removing personally
/// identifiable information (PII) from file chunks.
///
/// # Examples
///
/// ```
/// use adaptive_pipeline::infrastructure::services::SanitizationService;
///
/// let service = SanitizationService::new();
/// let sanitized = service.sanitize(chunk, &mut context)?;
/// ```
///
/// # Performance
///
/// - Throughput: ~200 MB/s (regex-based)
/// - Memory: O(chunk_size)
/// - Thread-safe: Yes
///
/// # Algorithms Supported
///
/// - Email addresses (regex: `\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b`)
/// - Phone numbers (various formats)
/// - Social Security Numbers (US)
/// - Credit card numbers (Luhn validation)
pub struct SanitizationService {
    patterns: Vec<Regex>,
}
```

## Integration Examples

### Example 1: Custom Deduplication Stage

**Complete implementation:**

```rust
// 1. Add to StageType enum
pub enum StageType {
    // ... existing types ...
    Deduplication,
}

// 2. Define domain service
pub trait DeduplicationService: Send + Sync {
    fn deduplicate(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<Option<FileChunk>, PipelineError>;
}

// 3. Implement infrastructure service
pub struct BloomFilterDeduplicationService {
    bloom_filter: Arc<Mutex<BloomFilter>>,
}

impl DeduplicationService for BloomFilterDeduplicationService {
    fn deduplicate(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<Option<FileChunk>, PipelineError> {
        use blake3::hash;

        // Calculate chunk hash
        let hash = hash(chunk.data());

        // Check if chunk seen before
        let mut filter = self.bloom_filter.lock().unwrap();
        if filter.contains(&hash) {
            // Duplicate found
            context.increment_deduplication_hits();
            return Ok(None);
        }

        // New chunk, add to filter
        filter.insert(&hash);
        Ok(Some(chunk))
    }
}

// 4. Register in pipeline configuration
let mut pipeline = Pipeline::new();
pipeline.add_stage(PipelineStage::new(
    StageType::Deduplication,
    "dedup",
    StageConfiguration::new(
        "bloom-filter".to_string(),
        HashMap::new(),
        false,  // Not parallel (shared bloom filter)
    ),
));
```

## Related Topics

- See [Custom Stages](custom-stages.md) for detailed stage implementation guide
- See [Custom Algorithms](custom-algorithms.md) for algorithm implementation patterns
- See [Architecture](../architecture/layers.md) for layered architecture principles
- See [Ports and Adapters](../architecture/ports-adapters.md) for hexagonal architecture

## Summary

The pipeline provides multiple extension points:

1. **Custom Stages**: Extend StageType enum and implement processing logic
2. **Custom Algorithms**: Add new compression, encryption, or hashing algorithms
3. **Custom Services**: Implement domain service traits for specialized operations
4. **Custom Adapters**: Create infrastructure adapters for external systems
5. **Custom Metrics**: Extend observability with custom metrics collectors

**Key Takeaways:**
- Follow hexagonal architecture (domain → application → infrastructure)
- Use dependency injection for loose coupling
- Implement domain traits (ports) in infrastructure layer (adapters)
- Maintain type safety with strong typing and validation
- Add comprehensive tests for all custom implementations
- Document extension points and usage examples

**Extension Checklist:**
- [ ] Define domain trait (if new concept)
- [ ] Implement infrastructure adapter
- [ ] Add unit tests for adapter
- [ ] Add integration tests for end-to-end workflow
- [ ] Document configuration options
- [ ] Update architectural diagrams (if significant)
- [ ] Consider performance impact (benchmark)
- [ ] Verify thread safety (Send + Sync)
