# File I/O

**Version:** 0.1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

This chapter provides a comprehensive overview of the file input/output architecture in the adaptive pipeline system. Learn how file chunks, type-safe paths, and streaming I/O work together to enable efficient, memory-safe file processing.

---

## Table of Contents

- [Overview](#overview)
- [File I/O Architecture](#file-io-architecture)
- [FileChunk Value Object](#filechunk-value-object)
- [FilePath Value Object](#filepath-value-object)
- [ChunkSize Value Object](#chunksize-value-object)
- [FileIOService Interface](#fileioservice-interface)
- [File Reading](#file-reading)
- [File Writing](#file-writing)
- [Memory Management](#memory-management)
- [Error Handling](#error-handling)
- [Performance Optimization](#performance-optimization)
- [Usage Examples](#usage-examples)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Testing Strategies](#testing-strategies)
- [Next Steps](#next-steps)

---

## Overview

**File I/O** in the adaptive pipeline system is designed for efficient, memory-safe processing of files of any size through chunking, streaming, and intelligent memory management. The system uses immutable value objects and async I/O for optimal performance.

### Key Features

- **Chunked Processing**: Files split into manageable chunks for parallel processing
- **Streaming I/O**: Process files without loading entirely into memory
- **Type-Safe Paths**: Compile-time path category enforcement
- **Immutable Chunks**: Thread-safe, corruption-proof file chunks
- **Validated Sizes**: Chunk sizes validated at creation
- **Async Operations**: Non-blocking I/O for better concurrency

### File I/O Stack

```text
┌──────────────────────────────────────────────────────────┐
│                  Application Layer                        │
│  ┌────────────────────────────────────────────────┐      │
│  │   File Processor Service                       │      │
│  │   - Orchestrates chunking and processing       │      │
│  └────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────┘
                         ↓ uses
┌──────────────────────────────────────────────────────────┐
│                    Domain Layer                           │
│  ┌────────────────────────────────────────────────┐      │
│  │   FileIOService (Trait)                        │      │
│  │   - read_file_chunks(), write_file_chunks()    │      │
│  └────────────────────────────────────────────────┘      │
│  ┌────────────┬───────────┬──────────────┐              │
│  │ FileChunk  │ FilePath  │  ChunkSize   │              │
│  │ (immutable)│(type-safe)│(validated)   │              │
│  └────────────┴───────────┴──────────────┘              │
└──────────────────────────────────────────────────────────┘
                         ↓ implements
┌──────────────────────────────────────────────────────────┐
│              Infrastructure Layer                         │
│  ┌────────────────────────────────────────────────┐      │
│  │   Async File I/O Implementation                │      │
│  │   - tokio::fs for async operations             │      │
│  │   - Streaming, chunking, buffering             │      │
│  └────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────┘
                         ↓ reads/writes
┌──────────────────────────────────────────────────────────┐
│                  File System                              │
│  - Input files, output files, temporary files            │
└──────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Immutability**: File chunks cannot be modified after creation
2. **Streaming**: Process files without loading entirely into memory
3. **Type Safety**: Compile-time path category enforcement
4. **Async-First**: Non-blocking I/O for better concurrency
5. **Memory Efficiency**: Bounded memory usage regardless of file size

---

## File I/O Architecture

The file I/O layer uses value objects and async services to provide efficient, safe file processing.

### Architectural Components

```text
┌─────────────────────────────────────────────────────────────┐
│ Value Objects (Domain)                                      │
│  ┌────────────────┬────────────────┬─────────────────┐     │
│  │  FileChunk     │  FilePath<T>   │   ChunkSize     │     │
│  │  - Immutable   │  - Type-safe   │   - Validated   │     │
│  │  - UUID ID     │  - Category    │   - 1B-512MB    │     │
│  │  - Sequence #  │  - Validated   │   - Default 1MB │     │
│  └────────────────┴────────────────┴─────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Service Interface (Domain)                                  │
│  ┌──────────────────────────────────────────────────┐      │
│  │  FileIOService (async trait)                     │      │
│  │  - read_file_chunks()                            │      │
│  │  - write_file_chunks()                           │      │
│  │  - stream_chunks()                               │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ Implementation (Infrastructure)                             │
│  ┌──────────────────────────────────────────────────┐      │
│  │  Async File I/O                                  │      │
│  │  - tokio::fs::File                               │      │
│  │  - Buffering, streaming                          │      │
│  │  - Memory mapping (large files)                  │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Processing Flow

**File → Chunks → Processing → Chunks → File:**

```text
Input File (e.g., 100MB)
        ↓
Split into chunks (1MB each)
        ↓
┌─────────┬─────────┬─────────┬─────────┐
│ Chunk 0 │ Chunk 1 │ Chunk 2 │   ...   │
│ (1MB)   │ (1MB)   │ (1MB)   │ (1MB)   │
└─────────┴─────────┴─────────┴─────────┘
        ↓ parallel processing
┌─────────┬─────────┬─────────┬─────────┐
│Processed│Processed│Processed│   ...   │
│ Chunk 0 │ Chunk 1 │ Chunk 2 │ (1MB)   │
└─────────┴─────────┴─────────┴─────────┘
        ↓
Reassemble chunks
        ↓
Output File (100MB)
```

---

## FileChunk Value Object

`FileChunk` is an immutable value object representing a portion of a file for processing.

### FileChunk Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileChunk {
    id: Uuid,                           // Unique identifier
    sequence_number: u64,               // Order in file (0-based)
    offset: u64,                        // Byte offset in original file
    size: ChunkSize,                    // Validated chunk size
    data: Vec<u8>,                      // Actual chunk data
    checksum: Option<String>,           // Optional SHA-256 checksum
    is_final: bool,                     // Last chunk flag
    created_at: DateTime<Utc>,          // Creation timestamp
}
```

### Key Characteristics

| Feature | Description |
|---------|-------------|
| **Immutability** | Once created, chunks cannot be modified |
| **Unique Identity** | Each chunk has a UUID for tracking |
| **Sequence Ordering** | Maintains position for reassembly |
| **Integrity** | Optional checksums for validation |
| **Thread Safety** | Fully thread-safe due to immutability |

### Creating Chunks

```rust
use adaptive_pipeline_domain::FileChunk;

// Basic chunk creation
let data = vec![1, 2, 3, 4, 5];
let chunk = FileChunk::new(
    0,        // sequence_number
    0,        // offset
    data,     // data
    false,    // is_final
)?;

println!("Chunk ID: {}", chunk.id());
println!("Size: {} bytes", chunk.size().bytes());
```

### Chunk with Checksum

```rust
// Create chunk with checksum
let data = vec![1, 2, 3, 4, 5];
let chunk = FileChunk::new(0, 0, data, false)?
    .with_calculated_checksum();

// Verify checksum
if let Some(checksum) = chunk.checksum() {
    println!("Checksum: {}", checksum);
}

// Verify data integrity
assert!(chunk.verify_checksum());
```

### Chunk Properties

```rust
// Access chunk properties
println!("ID: {}", chunk.id());
println!("Sequence: {}", chunk.sequence_number());
println!("Offset: {}", chunk.offset());
println!("Size: {} bytes", chunk.size().bytes());
println!("Is final: {}", chunk.is_final());
println!("Created: {}", chunk.created_at());

// Access data
let data: &[u8] = chunk.data();
```

---

## FilePath Value Object

`FilePath<T>` is a type-safe, validated file path with compile-time category enforcement.

### Path Categories

```rust
// Type-safe path categories
pub struct InputPath;      // For input files
pub struct OutputPath;     // For output files
pub struct TempPath;       // For temporary files
pub struct LogPath;        // For log files

// Usage with phantom types
let input: FilePath<InputPath> = FilePath::new("./input.dat")?;
let output: FilePath<OutputPath> = FilePath::new("./output.dat")?;

// ✅ Type system prevents mixing categories
// ❌ Cannot assign input path to output variable
// let wrong: FilePath<OutputPath> = input;  // Compile error!
```

### Path Validation

```rust
use adaptive_pipeline_domain::value_objects::{FilePath, InputPath};

// Create and validate path
let path = FilePath::<InputPath>::new("/path/to/file.dat")?;

// Path properties
println!("Path: {}", path.as_str());
println!("Exists: {}", path.exists());
println!("Is file: {}", path.is_file());
println!("Is dir: {}", path.is_dir());
println!("Is absolute: {}", path.is_absolute());

// Convert to std::path::Path
let std_path: &Path = path.as_path();
```

### Path Operations

```rust
// Get file name
let file_name = path.file_name();

// Get parent directory
let parent = path.parent();

// Get file extension
let extension = path.extension();

// Convert to string
let path_str = path.to_string();
```

---

## ChunkSize Value Object

`ChunkSize` represents a validated chunk size within system bounds.

### Size Constraints

```rust
// Chunk size constants
ChunkSize::MIN_SIZE  // 1 byte
ChunkSize::MAX_SIZE  // 512 MB
ChunkSize::DEFAULT   // 1 MB
```

### Creating Chunk Sizes

```rust
use adaptive_pipeline_domain::ChunkSize;

// From bytes
let size = ChunkSize::new(1024 * 1024)?;  // 1 MB
assert_eq!(size.bytes(), 1_048_576);

// From kilobytes
let size_kb = ChunkSize::from_kb(512)?;  // 512 KB
assert_eq!(size_kb.kilobytes(), 512.0);

// From megabytes
let size_mb = ChunkSize::from_mb(16)?;  // 16 MB
assert_eq!(size_mb.megabytes(), 16.0);

// Default (1 MB)
let default_size = ChunkSize::default();
assert_eq!(default_size.megabytes(), 1.0);
```

### Size Validation

```rust
// ✅ Valid sizes
let valid = ChunkSize::new(64 * 1024)?;  // 64 KB

// ❌ Invalid: too small
let too_small = ChunkSize::new(0);  // Error: must be ≥ 1 byte
assert!(too_small.is_err());

// ❌ Invalid: too large
let too_large = ChunkSize::new(600 * 1024 * 1024);  // Error: must be ≤ 512 MB
assert!(too_large.is_err());
```

### Optimal Sizing

```rust
// Calculate optimal chunk size for file
let file_size = 100 * 1024 * 1024;  // 100 MB
let optimal = ChunkSize::optimal_for_file_size(file_size);

println!("Optimal chunk size: {} MB", optimal.megabytes());

// Size conversions
let size = ChunkSize::from_mb(4)?;
println!("Bytes: {}", size.bytes());
println!("Kilobytes: {}", size.kilobytes());
println!("Megabytes: {}", size.megabytes());
```

---

## FileIOService Interface

`FileIOService` is an async trait defining file I/O operations.

### Service Interface

```rust
#[async_trait]
pub trait FileIOService: Send + Sync {
    /// Read file into chunks
    async fn read_file_chunks(
        &self,
        path: &Path,
        chunk_size: ChunkSize,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Write chunks to file
    async fn write_file_chunks(
        &self,
        path: &Path,
        chunks: Vec<FileChunk>,
    ) -> Result<(), PipelineError>;

    /// Stream chunks for processing
    async fn stream_chunks(
        &self,
        path: &Path,
        chunk_size: ChunkSize,
    ) -> Result<impl Stream<Item = Result<FileChunk, PipelineError>>, PipelineError>;
}
```

### Why Async?

The service is async because file I/O is **I/O-bound**, not CPU-bound:

**Benefits:**
- **Non-Blocking**: Doesn't block the async runtime
- **Concurrent**: Multiple files can be processed concurrently
- **tokio Integration**: Natural integration with tokio::fs
- **Performance**: Better throughput for I/O operations

**Classification:**
- This is an **infrastructure port**, not a domain service
- Domain services (compression, encryption) are CPU-bound and sync
- Infrastructure ports (file I/O, network, database) are I/O-bound and async

---

## File Reading

File reading uses streaming and chunking for memory-efficient processing.

### Reading Small Files

```rust
use adaptive_pipeline_domain::FileIOService;

// Read entire file into chunks
let service: Arc<dyn FileIOService> = /* ... */;
let chunks = service.read_file_chunks(
    Path::new("./input.dat"),
    ChunkSize::from_mb(1)?,
).await?;

println!("Read {} chunks", chunks.len());
for chunk in chunks {
    println!("Chunk {}: {} bytes", chunk.sequence_number(), chunk.size().bytes());
}
```

### Streaming Large Files

```rust
use tokio_stream::StreamExt;

// Stream chunks for memory efficiency
let mut stream = service.stream_chunks(
    Path::new("./large_file.dat"),
    ChunkSize::from_mb(4)?,
).await?;

while let Some(chunk_result) = stream.next().await {
    let chunk = chunk_result?;

    // Process chunk without loading entire file
    process_chunk(chunk).await?;
}
```

### Reading with Metadata

```rust
use std::fs::metadata;

// Get file metadata first
let metadata = metadata("./input.dat")?;
let file_size = metadata.len();

// Choose optimal chunk size
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

// Read with optimal settings
let chunks = service.read_file_chunks(
    Path::new("./input.dat"),
    chunk_size,
).await?;
```

---

## File Writing

File writing assembles processed chunks back into complete files.

### Writing Chunks to File

```rust
// Write chunks to output file
let processed_chunks: Vec<FileChunk> = /* ... */;

service.write_file_chunks(
    Path::new("./output.dat"),
    processed_chunks,
).await?;

println!("File written successfully");
```

### Streaming Write

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Stream chunks directly to file
let mut file = File::create("./output.dat").await?;

for chunk in processed_chunks {
    file.write_all(chunk.data()).await?;
}

file.flush().await?;
println!("File written: {} bytes", file.metadata().await?.len());
```

### Atomic Writes

```rust
use tempfile::NamedTempFile;

// Write to temporary file first
let temp_file = NamedTempFile::new()?;
service.write_file_chunks(
    temp_file.path(),
    chunks,
).await?;

// Atomically move to final location
temp_file.persist("./output.dat")?;
```

---

## Memory Management

The system uses several strategies to manage memory efficiently.

### Bounded Memory Usage

```text
File Size: 1 GB
Chunk Size: 4 MB
Memory Usage: ~4 MB (single chunk in memory at a time)

Without chunking: 1 GB in memory
With chunking: 4 MB in memory (250x reduction!)
```

### Memory Mapping for Large Files

```rust
// Automatically uses memory mapping for files > threshold
let config = FileIOConfig {
    chunk_size: ChunkSize::from_mb(4)?,
    use_memory_mapping: true,
    memory_mapping_threshold: 100 * 1024 * 1024,  // 100 MB
    ..Default::default()
};

// Files > 100 MB use memory mapping
let chunks = service.read_file_chunks_with_config(
    Path::new("./large_file.dat"),
    &config,
).await?;
```

### Streaming vs. Buffering

**Streaming (Memory-Efficient):**
```rust
// Process one chunk at a time
let mut stream = service.stream_chunks(path, chunk_size).await?;
while let Some(chunk) = stream.next().await {
    process_chunk(chunk?).await?;
}
// Peak memory: 1 chunk size
```

**Buffering (Performance):**
```rust
// Load all chunks into memory
let chunks = service.read_file_chunks(path, chunk_size).await?;
process_all_chunks(chunks).await?;
// Peak memory: all chunks
```

---

## Error Handling

The file I/O system handles various error conditions.

### Common Errors

```rust
use adaptive_pipeline_domain::PipelineError;

match service.read_file_chunks(path, chunk_size).await {
    Ok(chunks) => { /* success */ },
    Err(PipelineError::FileNotFound(path)) => {
        eprintln!("File not found: {}", path);
    },
    Err(PipelineError::PermissionDenied(path)) => {
        eprintln!("Permission denied: {}", path);
    },
    Err(PipelineError::InsufficientDiskSpace) => {
        eprintln!("Not enough disk space");
    },
    Err(PipelineError::InvalidChunk(msg)) => {
        eprintln!("Invalid chunk: {}", msg);
    },
    Err(e) => {
        eprintln!("I/O error: {}", e);
    },
}
```

### Retry Logic

```rust
use tokio::time::{sleep, Duration};

async fn read_with_retry(
    service: &dyn FileIOService,
    path: &Path,
    chunk_size: ChunkSize,
    max_retries: u32,
) -> Result<Vec<FileChunk>, PipelineError> {
    let mut retries = 0;

    loop {
        match service.read_file_chunks(path, chunk_size).await {
            Ok(chunks) => return Ok(chunks),
            Err(e) if retries < max_retries => {
                retries += 1;
                warn!("Read failed (attempt {}/{}): {}", retries, max_retries, e);
                sleep(Duration::from_secs(1 << retries)).await;  // Exponential backoff
            },
            Err(e) => return Err(e),
        }
    }
}
```

### Partial Reads

```rust
// Handle partial reads gracefully
async fn read_available_chunks(
    service: &dyn FileIOService,
    path: &Path,
    chunk_size: ChunkSize,
) -> Result<Vec<FileChunk>, PipelineError> {
    let mut chunks = Vec::new();
    let mut stream = service.stream_chunks(path, chunk_size).await?;

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => chunks.push(chunk),
            Err(e) => {
                warn!("Chunk read error: {}, stopping", e);
                break;  // Return partial results
            },
        }
    }

    Ok(chunks)
}
```

---

## Performance Optimization

Several strategies optimize file I/O performance.

### Optimal Chunk Size Selection

```rust
// Chunk size recommendations
fn optimal_chunk_size(file_size: u64) -> ChunkSize {
    match file_size {
        0..=10_485_760 => ChunkSize::from_mb(1).unwrap(),          // < 10 MB: 1 MB chunks
        10_485_761..=104_857_600 => ChunkSize::from_mb(4).unwrap(), // 10-100 MB: 4 MB chunks
        104_857_601..=1_073_741_824 => ChunkSize::from_mb(8).unwrap(), // 100 MB - 1 GB: 8 MB chunks
        _ => ChunkSize::from_mb(16).unwrap(),                       // > 1 GB: 16 MB chunks
    }
}
```

### Parallel Processing

```rust
use futures::future::try_join_all;

// Process chunks in parallel
let futures: Vec<_> = chunks.into_iter()
    .map(|chunk| async move {
        process_chunk(chunk).await
    })
    .collect();

let results = try_join_all(futures).await?;
```

### Buffered I/O

```rust
use tokio::io::BufReader;

// Use buffered reading for better performance
let file = File::open(path).await?;
let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);  // 8 MB buffer

// Read chunks with buffering
while let Some(chunk) = read_chunk(&mut reader).await? {
    process_chunk(chunk).await?;
}
```

### Performance Benchmarks

| Operation | Small Files (< 10 MB) | Medium Files (100 MB) | Large Files (> 1 GB) |
|-----------|----------------------|----------------------|---------------------|
| **Read** | ~50 MB/s | ~200 MB/s | ~300 MB/s |
| **Write** | ~40 MB/s | ~180 MB/s | ~280 MB/s |
| **Stream** | ~45 MB/s | ~190 MB/s | ~290 MB/s |

*Benchmarks on SSD with 4 MB chunks*

---

## Usage Examples

### Example 1: Basic File Processing

```rust
use adaptive_pipeline_domain::{FileIOService, ChunkSize};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service: Arc<dyn FileIOService> = /* ... */;

    // Read file
    let chunks = service.read_file_chunks(
        Path::new("./input.dat"),
        ChunkSize::from_mb(1)?,
    ).await?;

    println!("Read {} chunks", chunks.len());

    // Process chunks
    let processed: Vec<_> = chunks.into_iter()
        .map(|chunk| process_chunk(chunk))
        .collect();

    // Write output
    service.write_file_chunks(
        Path::new("./output.dat"),
        processed,
    ).await?;

    println!("Processing complete!");
    Ok(())
}

fn process_chunk(chunk: FileChunk) -> FileChunk {
    // Transform chunk data
    let transformed_data = chunk.data().to_vec();
    FileChunk::new(
        chunk.sequence_number(),
        chunk.offset(),
        transformed_data,
        chunk.is_final(),
    ).unwrap()
}
```

### Example 2: Streaming Large Files

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service: Arc<dyn FileIOService> = /* ... */;

    // Stream large file
    let mut stream = service.stream_chunks(
        Path::new("./large_file.dat"),
        ChunkSize::from_mb(8)?,
    ).await?;

    let mut processed_chunks = Vec::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        // Process chunk in streaming fashion
        let processed = process_chunk(chunk);
        processed_chunks.push(processed);

        // Optional: write chunks as they're processed
        // write_chunk_to_file(&processed).await?;
    }

    println!("Processed {} chunks", processed_chunks.len());
    Ok(())
}
```

### Example 3: Parallel Chunk Processing

```rust
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service: Arc<dyn FileIOService> = /* ... */;

    // Read all chunks
    let chunks = service.read_file_chunks(
        Path::new("./input.dat"),
        ChunkSize::from_mb(4)?,
    ).await?;

    // Process chunks in parallel
    let futures = chunks.into_iter().map(|chunk| {
        tokio::spawn(async move {
            process_chunk_async(chunk).await
        })
    });

    let results = try_join_all(futures).await?;
    let processed: Vec<_> = results.into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // Write results
    service.write_file_chunks(
        Path::new("./output.dat"),
        processed,
    ).await?;

    Ok(())
}

async fn process_chunk_async(chunk: FileChunk) -> Result<FileChunk, PipelineError> {
    // Async processing
    tokio::time::sleep(Duration::from_millis(10)).await;
    Ok(process_chunk(chunk))
}
```

### Example 4: Error Handling and Retry

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service: Arc<dyn FileIOService> = /* ... */;
    let path = Path::new("./input.dat");
    let chunk_size = ChunkSize::from_mb(1)?;

    // Retry on failure
    let chunks = read_with_retry(&*service, path, chunk_size, 3).await?;

    println!("Successfully read {} chunks", chunks.len());
    Ok(())
}

async fn read_with_retry(
    service: &dyn FileIOService,
    path: &Path,
    chunk_size: ChunkSize,
    max_retries: u32,
) -> Result<Vec<FileChunk>, PipelineError> {
    for attempt in 1..=max_retries {
        match service.read_file_chunks(path, chunk_size).await {
            Ok(chunks) => return Ok(chunks),
            Err(e) if attempt < max_retries => {
                eprintln!("Attempt {}/{} failed: {}", attempt, max_retries, e);
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
            },
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

---

## Best Practices

### 1. Choose Appropriate Chunk Sizes

```rust
// ✅ Good: Optimize chunk size for file
let file_size = metadata(path)?.len();
let chunk_size = ChunkSize::optimal_for_file_size(file_size);

// ❌ Bad: Fixed chunk size for all files
let chunk_size = ChunkSize::from_mb(1)?;  // May be suboptimal
```

### 2. Use Streaming for Large Files

```rust
// ✅ Good: Stream large files
let mut stream = service.stream_chunks(path, chunk_size).await?;
while let Some(chunk) = stream.next().await {
    process_chunk(chunk?).await?;
}

// ❌ Bad: Load entire large file into memory
let chunks = service.read_file_chunks(path, chunk_size).await?;
// Entire file in memory!
```

### 3. Validate Chunk Integrity

```rust
// ✅ Good: Verify checksums
for chunk in chunks {
    if !chunk.verify_checksum() {
        return Err(PipelineError::ChecK sumMismatch);
    }
    process_chunk(chunk)?;
}
```

### 4. Handle Errors Gracefully

```rust
// ✅ Good: Specific error handling
match service.read_file_chunks(path, chunk_size).await {
    Ok(chunks) => process(chunks),
    Err(PipelineError::FileNotFound(_)) => handle_missing_file(),
    Err(PipelineError::PermissionDenied(_)) => handle_permissions(),
    Err(e) => handle_generic_error(e),
}
```

### 5. Use Type-Safe Paths

```rust
// ✅ Good: Type-safe paths
let input: FilePath<InputPath> = FilePath::new("./input.dat")?;
let output: FilePath<OutputPath> = FilePath::new("./output.dat")?;

// ❌ Bad: Raw strings
let input = "./input.dat";
let output = "./output.dat";
```

### 6. Clean Up Temporary Files

```rust
// ✅ Good: Automatic cleanup with tempfile
let temp = NamedTempFile::new()?;
write_chunks(temp.path(), chunks).await?;
// Automatically deleted when dropped

// Or explicit cleanup
temp.close()?;
```

### 7. Monitor Memory Usage

```rust
// ✅ Good: Track memory usage
let chunks_in_memory = chunks.len();
let memory_used = chunks_in_memory * chunk_size.bytes();
if memory_used > max_memory {
    // Switch to streaming
}
```

---

## Troubleshooting

### Issue 1: Out of Memory

**Symptom:**
```text
Error: Out of memory
```

**Solutions:**

```rust
// 1. Reduce chunk size
let chunk_size = ChunkSize::from_mb(1)?;  // Smaller chunks

// 2. Use streaming instead of buffering
let mut stream = service.stream_chunks(path, chunk_size).await?;

// 3. Process chunks one at a time
while let Some(chunk) = stream.next().await {
    process_chunk(chunk?).await?;
    // Chunk dropped, memory freed
}
```

### Issue 2: Slow File I/O

**Diagnosis:**

```rust
let start = Instant::now();
let chunks = service.read_file_chunks(path, chunk_size).await?;
let duration = start.elapsed();
println!("Read took: {:?}", duration);
```

**Solutions:**

```rust
// 1. Increase chunk size
let chunk_size = ChunkSize::from_mb(8)?;  // Larger chunks = fewer I/O ops

// 2. Use memory mapping for large files
let config = FileIOConfig {
    use_memory_mapping: true,
    memory_mapping_threshold: 50 * 1024 * 1024,  // 50 MB
    ..Default::default()
};

// 3. Use buffered I/O
let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
```

### Issue 3: Checksum Mismatch

**Symptom:**
```text
Error: Checksum mismatch for chunk 42
```

**Solutions:**

```rust
// 1. Verify during read
let chunk = chunk.with_calculated_checksum();
if !chunk.verify_checksum() {
    // Re-read chunk
}

// 2. Log and skip corrupted chunks
match chunk.verify_checksum() {
    true => process_chunk(chunk),
    false => {
        warn!("Chunk {} corrupted, skipping", chunk.sequence_number());
        continue;
    },
}
```

### Issue 4: File Permission Errors

**Symptom:**
```text
Error: Permission denied: ./output.dat
```

**Solutions:**

```rust
// 1. Check permissions before writing
use std::fs;
let metadata = fs::metadata(parent_dir)?;
if metadata.permissions().readonly() {
    return Err("Output directory is read-only".into());
}

// 2. Use appropriate path categories
let output: FilePath<OutputPath> = FilePath::new("./output.dat")?;
output.ensure_writable()?;
```

---

## Testing Strategies

### Unit Testing with Mock Chunks

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let chunk = FileChunk::new(0, 0, data.clone(), false).unwrap();

        assert_eq!(chunk.sequence_number(), 0);
        assert_eq!(chunk.data(), &data);
        assert!(!chunk.is_final());
    }

    #[test]
    fn test_chunk_checksum() {
        let data = vec![1, 2, 3, 4, 5];
        let chunk = FileChunk::new(0, 0, data, false)
            .unwrap()
            .with_calculated_checksum();

        assert!(chunk.checksum().is_some());
        assert!(chunk.verify_checksum());
    }
}
```

### Integration Testing with Files

```rust
#[tokio::test]
async fn test_file_round_trip() {
    let service: Arc<dyn FileIOService> = create_test_service();

    // Create test data
    let test_data = vec![0u8; 10 * 1024 * 1024];  // 10 MB
    let input_path = temp_dir().join("test_input.dat");
    std::fs::write(&input_path, &test_data).unwrap();

    // Read chunks
    let chunks = service.read_file_chunks(
        &input_path,
        ChunkSize::from_mb(1).unwrap(),
    ).await.unwrap();

    assert_eq!(chunks.len(), 10);  // 10 x 1MB chunks

    // Write chunks
    let output_path = temp_dir().join("test_output.dat");
    service.write_file_chunks(&output_path, chunks).await.unwrap();

    // Verify
    let output_data = std::fs::read(&output_path).unwrap();
    assert_eq!(test_data, output_data);
}
```

### Streaming Tests

```rust
#[tokio::test]
async fn test_streaming() {
    use tokio_stream::StreamExt;

    let service: Arc<dyn FileIOService> = create_test_service();
    let path = create_test_file(100 * 1024 * 1024);  // 100 MB

    let mut stream = service.stream_chunks(
        &path,
        ChunkSize::from_mb(4).unwrap(),
    ).await.unwrap();

    let mut chunk_count = 0;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.unwrap();
        assert!(chunk.size().bytes() <= 4 * 1024 * 1024);
        chunk_count += 1;
    }

    assert_eq!(chunk_count, 25);  // 100 MB / 4 MB = 25 chunks
}
```

---

## Next Steps

After understanding file I/O fundamentals, explore specific implementations:

### Detailed File I/O Topics

1. **[Chunking Strategy](chunking.md)**: Deep dive into chunking algorithms and optimization
2. **[Binary Format](binary-format.md)**: File format specification and serialization

### Related Topics

- **[Stage Processing](stages.md)**: How stages process file chunks
- **[Integrity Checking](integrity.md)**: Checksums and verification
- **[Performance Optimization](../advanced/performance.md)**: I/O performance tuning

### Advanced Topics

- **[Concurrency Model](../advanced/concurrency.md)**: Parallel file processing
- **[Extending the Pipeline](../advanced/extending.md)**: Custom file formats and I/O

---

## Summary

**Key Takeaways:**

1. **FileChunk** is an immutable value object representing file data portions
2. **FilePath<T>** provides type-safe, validated file paths with phantom types
3. **ChunkSize** validates chunk sizes within 1 byte to 512 MB bounds
4. **FileIOService** defines async I/O operations for streaming and chunking
5. **Streaming** enables memory-efficient processing of files of any size
6. **Memory Management** uses bounded buffers and optional memory mapping
7. **Error Handling** provides retry logic and graceful degradation

**Architecture File References:**
- **FileChunk:** `pipeline-domain/src/value_objects/file_chunk.rs:176`
- **FilePath:** `pipeline-domain/src/value_objects/file_path.rs:1`
- **ChunkSize:** `pipeline-domain/src/value_objects/chunk_size.rs:1`
- **FileIOService:** `pipeline-domain/src/services/file_io_service.rs:185`
