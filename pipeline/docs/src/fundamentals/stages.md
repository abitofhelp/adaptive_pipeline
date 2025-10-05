# Pipeline Stages

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## What is a Stage?

A **pipeline stage** is a single processing operation that transforms data in a specific way. Each stage performs one well-defined task, like compressing data, encrypting it, or verifying its integrity.

Think of stages like workstations on an assembly line. Each workstation has specialized tools and performs one specific operation. The product moves from one workstation to the next until it's complete.

## Stage Types

Our pipeline supports three main categories of stages:

### 1. Compression Stages

Compression stages reduce the size of your data. This is useful for:
- Saving disk space
- Reducing network bandwidth
- Faster file transfers
- Lower storage costs

**Available Compression Algorithms:**

- **Brotli** - Best compression ratio, slower speed
  - Best for: Text files, web content, logs
  - Performance: Excellent compression, moderate speed
  - Memory: Higher memory usage

- **Gzip** - General-purpose compression
  - Best for: General files, wide compatibility
  - Performance: Good balance of speed and ratio
  - Memory: Moderate memory usage

- **Zstandard (zstd)** - Modern, fast compression
  - Best for: Large files, real-time compression
  - Performance: Excellent speed and ratio
  - Memory: Efficient memory usage

- **LZ4** - Extremely fast compression
  - Best for: Real-time applications, live data streams
  - Performance: Fastest compression, moderate ratio
  - Memory: Low memory usage

### 2. Encryption Stages

Encryption stages protect your data by making it unreadable without the correct key. This is essential for:
- Protecting sensitive information
- Compliance with security regulations
- Secure data transmission
- Privacy protection

**Available Encryption Algorithms:**

- **AES-256-GCM** - Industry standard encryption
  - Key Size: 256 bits (32 bytes)
  - Security: FIPS approved, very strong
  - Performance: Excellent with AES-NI hardware support
  - Authentication: Built-in integrity verification

- **ChaCha20-Poly1305** - Modern stream cipher
  - Key Size: 256 bits (32 bytes)
  - Security: Strong, constant-time implementation
  - Performance: Consistent across all platforms
  - Authentication: Built-in integrity verification

- **AES-128-GCM** - Faster AES variant
  - Key Size: 128 bits (16 bytes)
  - Security: Still very secure, slightly faster
  - Performance: Faster than AES-256
  - Authentication: Built-in integrity verification

### 3. Integrity Verification Stages

Integrity stages ensure your data hasn't been corrupted or tampered with. They create a unique "fingerprint" of your data called a checksum or hash.

**Available Hashing Algorithms:**

- **SHA-256** - Industry standard hashing
  - Output: 256 bits (32 bytes)
  - Security: Cryptographically secure
  - Performance: Good balance
  - Use Case: General integrity verification

- **SHA-512** - Stronger SHA variant
  - Output: 512 bits (64 bytes)
  - Security: Stronger than SHA-256
  - Performance: Good on 64-bit systems
  - Use Case: High-security applications

- **BLAKE3** - Modern, high-performance hashing
  - Output: 256 bits (32 bytes)
  - Security: Strong security properties
  - Performance: Very fast
  - Use Case: High-performance applications

## Stage Configuration

Each stage has a configuration that specifies how it should process data:

```rust
use pipeline_domain::{PipelineStage, Algorithm};

// Example: Compression stage
let compression_stage = PipelineStage::new(
    "compress".to_string(),
    Algorithm::zstd(),
    1, // stage order
)?;

// Example: Encryption stage
let encryption_stage = PipelineStage::new(
    "encrypt".to_string(),
    Algorithm::aes_256_gcm(),
    2, // stage order
)?;

// Example: Integrity verification stage
let integrity_stage = PipelineStage::new(
    "verify".to_string(),
    Algorithm::sha256(),
    3, // stage order
)?;
```

## Stage Execution Order

Stages execute in the order you define them. The output of one stage becomes the input to the next stage.

**Recommended Order for Processing:**
1. Compress (reduce size first)
2. Encrypt (protect compressed data)
3. Verify integrity (create checksum of encrypted data)

**For Restoration (reverse order):**
1. Verify integrity (check encrypted data)
2. Decrypt (recover compressed data)
3. Decompress (restore original file)

```text
Processing Pipeline:
Input File → Compress → Encrypt → Verify → Output File

Restoration Pipeline:
Input File → Verify → Decrypt → Decompress → Output File
```

## Combining Stages

You can combine stages in different ways depending on your needs:

### Maximum Security
```rust
vec![
    PipelineStage::new("compress", Algorithm::brotli(), 1)?,
    PipelineStage::new("encrypt", Algorithm::aes_256_gcm(), 2)?,
    PipelineStage::new("verify", Algorithm::blake3(), 3)?,
]
```

### Maximum Speed
```rust
vec![
    PipelineStage::new("compress", Algorithm::lz4(), 1)?,
    PipelineStage::new("encrypt", Algorithm::chacha20_poly1305(), 2)?,
]
```

### Balanced Approach
```rust
vec![
    PipelineStage::new("compress", Algorithm::zstd(), 1)?,
    PipelineStage::new("encrypt", Algorithm::aes_256_gcm(), 2)?,
    PipelineStage::new("verify", Algorithm::sha256(), 3)?,
]
```

## Parallel Processing

Stages process file chunks in parallel for better performance:

```text
File Split into Chunks:
┌──────┬──────┬──────┬──────┐
│Chunk1│Chunk2│Chunk3│Chunk4│
└──┬───┴──┬───┴──┬───┴──┬───┘
   │      │      │      │
   ▼      ▼      ▼      ▼
   ┌──────┬──────┬──────┬──────┐
   │Stage1│Stage1│Stage1│Stage1│ (Parallel)
   └──┬───┴──┬───┴──┬───┴──┬───┘
      ▼      ▼      ▼      ▼
   ┌──────┬──────┬──────┬──────┐
   │Stage2│Stage2│Stage2│Stage2│ (Parallel)
   └──┬───┴──┬───┴──┬───┴──┬───┘
      │      │      │      │
      ▼      ▼      ▼      ▼
   Combined Output File
```

This parallel processing allows the pipeline to utilize multiple CPU cores for faster throughput.

## Stage Validation

The pipeline validates stages at creation time:

- **Algorithm compatibility**: Ensures compression algorithms are only used in compression stages
- **Stage order**: Verifies stages have unique, sequential order numbers
- **Configuration validity**: Checks all stage parameters are valid
- **Dependency checks**: Ensures restoration pipelines match processing pipelines

```rust
// This will fail - wrong algorithm for stage type
PipelineStage::new(
    "compress",
    Algorithm::aes_256_gcm(), // Encryption algorithm!
    1
) // ❌ Error: Algorithm not compatible with stage type
```

## Extending with Custom Stages

The pipeline can be easily extended through custom stages to meet your specific requirements. You can create custom stages that implement your own processing logic, integrate third-party tools, or add specialized transformations.

For detailed information on implementing custom stages, see [Custom Stages](../advanced/custom-stages.md) in the Advanced Topics section.

## Next Steps

Now that you understand pipeline stages, you can learn about:

- [Configuration](configuration.md) - How to configure pipelines and stages
- [Your First Pipeline](first-run.md) - Run your first pipeline
- [Architecture Overview](../architecture/overview.md) - Deeper dive into the architecture
- [Custom Stages](../advanced/custom-stages.md) - Create your own custom stage implementations
