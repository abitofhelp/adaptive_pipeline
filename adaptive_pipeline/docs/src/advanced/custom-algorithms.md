# Custom Algorithms

**Version:** 0.1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter demonstrates how to implement custom compression, encryption, and hashing algorithms while integrating them seamlessly with the pipeline's existing infrastructure.

## Overview

The pipeline supports custom algorithm implementations for:

- **Compression**: Add new compression algorithms (e.g., Snappy, LZMA, custom formats)
- **Encryption**: Implement new ciphers (e.g., alternative AEADs, custom protocols)
- **Hashing**: Add new checksum algorithms (e.g., BLAKE3, xxHash, custom)

**Key Concepts:**
- **Algorithm Enum**: Type-safe algorithm identifier
- **Service Trait**: Domain interface defining algorithm operations
- **Service Implementation**: Concrete algorithm implementation
- **Configuration**: Algorithm-specific parameters and tuning

## Custom Compression Algorithm

### Step 1: Extend CompressionAlgorithm Enum

```rust
// pipeline-domain/src/value_objects/algorithm.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    Brotli,
    Gzip,
    Zstd,
    Lz4,

    // Custom algorithms
    Snappy,         // Google's Snappy
    Lzma,           // LZMA/XZ compression
    Custom(u8),     // Custom algorithm ID
}

impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing ...
            CompressionAlgorithm::Snappy => write!(f, "snappy"),
            CompressionAlgorithm::Lzma => write!(f, "lzma"),
            CompressionAlgorithm::Custom(id) => write!(f, "custom-{}", id),
        }
    }
}

impl std::str::FromStr for CompressionAlgorithm {
    type Err = PipelineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // ... existing ...
            "snappy" => Ok(CompressionAlgorithm::Snappy),
            "lzma" => Ok(CompressionAlgorithm::Lzma),
            s if s.starts_with("custom-") => {
                let id = s.strip_prefix("custom-")
                    .and_then(|id| id.parse::<u8>().ok())
                    .ok_or_else(|| PipelineError::InvalidConfiguration(
                        format!("Invalid custom ID: {}", s)
                    ))?;
                Ok(CompressionAlgorithm::Custom(id))
            }
            _ => Err(PipelineError::InvalidConfiguration(format!(
                "Unknown algorithm: {}",
                s
            ))),
        }
    }
}
```

### Step 2: Implement CompressionService

```rust
// pipeline/src/infrastructure/services/snappy_compression_service.rs

use adaptive_pipeline_domain::services::CompressionService;
use adaptive_pipeline_domain::{FileChunk, PipelineError, ProcessingContext};
use snap::raw::{Encoder, Decoder};

/// Snappy compression service implementation
///
/// Snappy is a fast compression/decompression library developed by Google.
/// It does not aim for maximum compression, or compatibility with any other
/// compression library; instead, it aims for very high speeds and reasonable
/// compression.
///
/// **Performance Characteristics:**
/// - Compression: 250-500 MB/s
/// - Decompression: 500-1000 MB/s
/// - Compression ratio: ~50-70% of original size
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
        let start = std::time::Instant::now();

        // Compress using Snappy
        let mut encoder = Encoder::new();
        let compressed = encoder
            .compress_vec(chunk.data())
            .map_err(|e| PipelineError::CompressionError(format!("Snappy: {}", e)))?;

        // Update metrics
        let duration = start.elapsed();
        context.add_bytes_processed(chunk.data().len() as u64);
        context.record_stage_duration(duration);

        // Create compressed chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            compressed,
        );

        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }

    fn decompress(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let start = std::time::Instant::now();

        // Decompress using Snappy
        let mut decoder = Decoder::new();
        let decompressed = decoder
            .decompress_vec(chunk.data())
            .map_err(|e| PipelineError::DecompressionError(format!("Snappy: {}", e)))?;

        // Update metrics
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
        // Snappy typically achieves ~50-70% compression
        (chunk.data().len() as f64 * 0.6) as usize
    }
}

impl Default for SnappyCompressionService {
    fn default() -> Self {
        Self::new()
    }
}
```

### Step 3: Register Algorithm

```rust
// In service factory or dependency injection

use std::sync::Arc;
use adaptive_pipeline_domain::services::CompressionService;

fn create_compression_service(
    algorithm: CompressionAlgorithm,
) -> Result<Arc<dyn CompressionService>, PipelineError> {
    match algorithm {
        CompressionAlgorithm::Brotli => Ok(Arc::new(BrotliCompressionService::new())),
        CompressionAlgorithm::Gzip => Ok(Arc::new(GzipCompressionService::new())),
        CompressionAlgorithm::Zstd => Ok(Arc::new(ZstdCompressionService::new())),
        CompressionAlgorithm::Lz4 => Ok(Arc::new(Lz4CompressionService::new())),

        // Custom algorithms
        CompressionAlgorithm::Snappy => Ok(Arc::new(SnappyCompressionService::new())),
        CompressionAlgorithm::Lzma => Ok(Arc::new(LzmaCompressionService::new())),

        CompressionAlgorithm::Custom(id) => {
            Err(PipelineError::InvalidConfiguration(format!(
                "Custom algorithm {} not registered",
                id
            )))
        }
    }
}
```

## Custom Encryption Algorithm

### Step 1: Extend EncryptionAlgorithm Enum

```rust
// pipeline-domain/src/value_objects/algorithm.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,

    // Custom algorithms
    Aes128Gcm,      // AES-128-GCM (faster, less secure)
    Twofish,        // Twofish cipher
    Custom(u8),     // Custom algorithm ID
}
```

### Step 2: Implement EncryptionService

```rust
// pipeline/src/infrastructure/services/aes128_encryption_service.rs

use adaptive_pipeline_domain::services::EncryptionService;
use adaptive_pipeline_domain::{FileChunk, PipelineError, ProcessingContext};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes128Gcm, Nonce,
};

/// AES-128-GCM encryption service
///
/// This implementation uses AES-128-GCM, which is faster than AES-256-GCM
/// but provides a lower security margin (128-bit vs 256-bit key).
///
/// **Use Cases:**
/// - Performance-critical applications
/// - Scenarios where 128-bit security is sufficient
/// - Systems without AES-NI support (software fallback)
///
/// **Performance:**
/// - Encryption: 800-1200 MB/s (with AES-NI)
/// - Encryption: 150-300 MB/s (software)
pub struct Aes128EncryptionService {
    cipher: Aes128Gcm,
}

impl Aes128EncryptionService {
    pub fn new(key: &[u8; 16]) -> Self {
        let cipher = Aes128Gcm::new(key.into());
        Self { cipher }
    }

    pub fn generate_key() -> [u8; 16] {
        use rand::RngCore;
        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        key
    }
}

impl EncryptionService for Aes128EncryptionService {
    fn encrypt(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let start = std::time::Instant::now();

        // Generate nonce (96-bit for GCM)
        let mut nonce_bytes = [0u8; 12];
        use rand::RngCore;
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt data
        let ciphertext = self.cipher
            .encrypt(nonce, chunk.data())
            .map_err(|e| PipelineError::EncryptionError(format!("AES-128-GCM: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut encrypted = nonce_bytes.to_vec();
        encrypted.extend_from_slice(&ciphertext);

        // Update metrics
        let duration = start.elapsed();
        context.add_bytes_processed(chunk.data().len() as u64);
        context.record_stage_duration(duration);

        // Create encrypted chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            encrypted,
        );

        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }

    fn decrypt(
        &self,
        chunk: FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let start = std::time::Instant::now();

        // Extract nonce from beginning
        if chunk.data().len() < 12 {
            return Err(PipelineError::DecryptionError(
                "Encrypted data too short".to_string()
            ));
        }

        let (nonce_bytes, ciphertext) = chunk.data().split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt data
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| PipelineError::DecryptionError(format!("AES-128-GCM: {}", e)))?;

        // Update metrics
        let duration = start.elapsed();
        context.add_bytes_processed(plaintext.len() as u64);
        context.record_stage_duration(duration);

        // Create decrypted chunk
        let mut result = FileChunk::new(
            chunk.sequence_number(),
            chunk.file_offset(),
            plaintext,
        );

        result.set_metadata(chunk.metadata().clone());

        Ok(result)
    }
}
```

## Custom Hashing Algorithm

### Step 1: Extend HashAlgorithm Enum

```rust
// pipeline-domain/src/value_objects/algorithm.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,

    // Custom algorithms
    XxHash,         // xxHash (extremely fast)
    Blake2b,        // BLAKE2b
    Custom(u8),     // Custom algorithm ID
}
```

### Step 2: Implement ChecksumService

```rust
// pipeline/src/infrastructure/services/xxhash_checksum_service.rs

use adaptive_pipeline_domain::services::ChecksumService;
use adaptive_pipeline_domain::{FileChunk, PipelineError, ProcessingContext, Checksum};
use xxhash_rust::xxh3::Xxh3;

/// xxHash checksum service implementation
///
/// xxHash is an extremely fast non-cryptographic hash algorithm,
/// working at RAM speed limits.
///
/// **Performance:**
/// - Hashing: 10-30 GB/s (on modern CPUs)
/// - ~10-20x faster than SHA-256
/// - ~2-3x faster than BLAKE3
///
/// **Use Cases:**
/// - Data integrity verification (non-cryptographic)
/// - Deduplication
/// - Hash tables
/// - NOT for security (use SHA-256 or BLAKE3 instead)
pub struct XxHashChecksumService;

impl XxHashChecksumService {
    pub fn new() -> Self {
        Self
    }
}

impl ChecksumService for XxHashChecksumService {
    fn calculate_checksum(
        &self,
        chunk: &FileChunk,
        context: &mut ProcessingContext,
    ) -> Result<Checksum, PipelineError> {
        let start = std::time::Instant::now();

        // Calculate xxHash64
        let mut hasher = Xxh3::new();
        hasher.update(chunk.data());
        let hash = hasher.digest();

        // Convert to bytes (big-endian)
        let hash_bytes = hash.to_be_bytes();

        // Update metrics
        let duration = start.elapsed();
        context.record_stage_duration(duration);

        Ok(Checksum::new(hash_bytes.to_vec()))
    }

    fn verify_checksum(
        &self,
        chunk: &FileChunk,
        expected: &Checksum,
        context: &mut ProcessingContext,
    ) -> Result<bool, PipelineError> {
        let calculated = self.calculate_checksum(chunk, context)?;
        Ok(calculated == *expected)
    }
}

impl Default for XxHashChecksumService {
    fn default() -> Self {
        Self::new()
    }
}
```

## Algorithm Configuration

### Compression Configuration

```rust
use adaptive_pipeline_domain::entities::StageConfiguration;
use std::collections::HashMap;

// Snappy configuration (minimal parameters)
let snappy_config = StageConfiguration::new(
    "snappy".to_string(),
    HashMap::new(),  // No tuning parameters
    true,            // Parallel processing
);

// LZMA configuration (with level)
let lzma_config = StageConfiguration::new(
    "lzma".to_string(),
    HashMap::from([
        ("level".to_string(), "6".to_string()),  // 0-9
    ]),
    true,
);
```

### Encryption Configuration

```rust
// AES-128-GCM configuration
let aes128_config = StageConfiguration::new(
    "aes128gcm".to_string(),
    HashMap::from([
        ("key".to_string(), base64::encode(&key)),
    ]),
    true,
);
```

### Hashing Configuration

```rust
// xxHash configuration
let xxhash_config = StageConfiguration::new(
    "xxhash".to_string(),
    HashMap::new(),
    true,
);
```

## Performance Comparison

### Compression Algorithms

| Algorithm | Compression Speed | Decompression Speed | Ratio | Use Case |
|-----------|-------------------|---------------------|-------|----------|
| **LZ4**   | 500-700 MB/s      | 2000-3000 MB/s      | 2-3x  | Real-time, low latency |
| **Snappy**| 250-500 MB/s      | 500-1000 MB/s       | 1.5-2x| Google services, fast |
| **Zstd**  | 200-400 MB/s      | 600-800 MB/s        | 3-5x  | Modern balanced |
| **Brotli**| 50-150 MB/s       | 300-500 MB/s        | 4-8x  | Web, maximum compression |
| **LZMA**  | 10-30 MB/s        | 50-100 MB/s         | 5-10x | Archival, best ratio |

### Encryption Algorithms

| Algorithm         | Encryption Speed | Security | Hardware Support |
|-------------------|------------------|----------|------------------|
| **AES-128-GCM**   | 800-1200 MB/s    | Good     | Yes (AES-NI)     |
| **AES-256-GCM**   | 400-800 MB/s     | Excellent| Yes (AES-NI)     |
| **ChaCha20**      | 200-400 MB/s     | Excellent| No               |

### Hashing Algorithms

| Algorithm  | Throughput   | Security      | Use Case              |
|------------|--------------|---------------|-----------------------|
| **xxHash** | 10-30 GB/s   | None          | Integrity, dedup      |
| **BLAKE3** | 3-10 GB/s    | Cryptographic | General purpose       |
| **SHA-256**| 400-800 MB/s | Cryptographic | Security, signatures  |

## Testing Custom Algorithms

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snappy_compression_roundtrip() {
        let service = SnappyCompressionService::new();
        let original_data = b"Hello, World! ".repeat(100);
        let chunk = FileChunk::new(0, 0, original_data.to_vec());
        let mut context = ProcessingContext::new();

        // Compress
        let compressed = service.compress(chunk.clone(), &mut context).unwrap();
        assert!(compressed.data().len() < original_data.len());

        // Decompress
        let decompressed = service.decompress(compressed, &mut context).unwrap();
        assert_eq!(decompressed.data(), &original_data);
    }

    #[test]
    fn test_aes128_encryption_roundtrip() {
        let key = Aes128EncryptionService::generate_key();
        let service = Aes128EncryptionService::new(&key);
        let original_data = b"Secret message";
        let chunk = FileChunk::new(0, 0, original_data.to_vec());
        let mut context = ProcessingContext::new();

        // Encrypt
        let encrypted = service.encrypt(chunk.clone(), &mut context).unwrap();
        assert_ne!(encrypted.data(), original_data);

        // Decrypt
        let decrypted = service.decrypt(encrypted, &mut context).unwrap();
        assert_eq!(decrypted.data(), original_data);
    }

    #[test]
    fn test_xxhash_checksum() {
        let service = XxHashChecksumService::new();
        let data = b"Test data";
        let chunk = FileChunk::new(0, 0, data.to_vec());
        let mut context = ProcessingContext::new();

        let checksum = service.calculate_checksum(&chunk, &mut context).unwrap();
        assert_eq!(checksum.bytes().len(), 8);  // 64-bit hash

        // Verify
        let valid = service.verify_checksum(&chunk, &checksum, &mut context).unwrap();
        assert!(valid);
    }
}
```

### Benchmark Custom Algorithm

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_snappy_vs_lz4(c: &mut Criterion) {
    let snappy = SnappyCompressionService::new();
    let lz4 = Lz4CompressionService::new();

    let test_data = vec![0u8; 1024 * 1024];  // 1 MB
    let chunk = FileChunk::new(0, 0, test_data);
    let mut context = ProcessingContext::new();

    let mut group = c.benchmark_group("compression");

    group.bench_function("snappy", |b| {
        b.iter(|| {
            snappy.compress(black_box(chunk.clone()), &mut context).unwrap()
        });
    });

    group.bench_function("lz4", |b| {
        b.iter(|| {
            lz4.compress(black_box(chunk.clone()), &mut context).unwrap()
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_snappy_vs_lz4);
criterion_main!(benches);
```

## Best Practices

### 1. Choose Appropriate Algorithms

```rust
// ✅ Good: Match algorithm to use case
let compression = if priority == Speed {
    CompressionAlgorithm::Snappy  // Fastest
} else if priority == Ratio {
    CompressionAlgorithm::Lzma    // Best ratio
} else {
    CompressionAlgorithm::Zstd    // Balanced
};

// ❌ Bad: Always use same algorithm
let compression = CompressionAlgorithm::Brotli;  // Slow!
```

### 2. Handle Errors Gracefully

```rust
// ✅ Good: Descriptive errors
fn compress(&self, chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    encoder.compress_vec(chunk.data())
        .map_err(|e| PipelineError::CompressionError(
            format!("Snappy compression failed: {}", e)
        ))?
}

// ❌ Bad: Generic errors
fn compress(&self, chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    encoder.compress_vec(chunk.data()).unwrap()  // Panics!
}
```

### 3. Benchmark Performance

```rust
// Always benchmark custom algorithms
#[bench]
fn bench_custom_algorithm(b: &mut Bencher) {
    let service = MyCustomService::new();
    let chunk = FileChunk::new(0, 0, vec![0u8; 1024 * 1024]);

    b.iter(|| {
        service.process(black_box(chunk.clone()))
    });
}
```

## Related Topics

- See [Extending the Pipeline](extending.md) for extension points overview
- See [Custom Stages](custom-stages.md) for stage implementation
- See [Performance Optimization](performance.md) for tuning strategies
- See [Benchmarking](benchmarking.md) for performance measurement

## Summary

Implementing custom algorithms involves:

1. **Extend Algorithm Enum**: Add variant for new algorithm
2. **Implement Service Trait**: Create concrete implementation
3. **Register Algorithm**: Add to service factory
4. **Test Thoroughly**: Unit tests, integration tests, benchmarks
5. **Document Performance**: Measure and document characteristics

**Key Takeaways:**
- Choose algorithms based on workload (speed vs ratio vs security)
- Implement complete error handling with specific error types
- Benchmark against existing algorithms
- Document performance characteristics
- Add comprehensive tests (correctness + performance)
- Consider hardware acceleration (AES-NI, SIMD)

**Algorithm Implementation Checklist:**
- [ ] Extend algorithm enum (Display + FromStr)
- [ ] Implement service trait
- [ ] Add unit tests (correctness)
- [ ] Add roundtrip tests (compress/decompress, encrypt/decrypt)
- [ ] Benchmark performance
- [ ] Compare with existing algorithms
- [ ] Document use cases and characteristics
- [ ] Register in service factory
- [ ] Update configuration documentation
