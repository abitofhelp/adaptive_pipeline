# Binary File Format

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The Adaptive Pipeline uses a custom binary file format (`.adapipe`) to store processed files with complete recovery metadata and integrity verification. This format enables perfect restoration of original files while maintaining processing history and security.

**Key Features:**
- **Complete Recovery**: All metadata needed to restore original files
- **Integrity Verification**: SHA-256 checksums for both input and output
- **Processing History**: Complete record of all processing steps
- **Format Versioning**: Backward compatibility through version management
- **Security**: Supports encryption with nonce management

## File Format Specification

### Binary Layout

The `.adapipe` format uses a reverse-header design for efficient processing:

```text
┌─────────────────────────────────────────────┐
│          PROCESSED CHUNK DATA               │
│         (variable length)                   │
│  - Compressed and/or encrypted chunks       │
│  - Each chunk: [NONCE][LENGTH][DATA]        │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│          JSON HEADER                        │
│         (variable length)                   │
│  - Processing metadata                      │
│  - Recovery information                     │
│  - Checksums                                │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│      HEADER_LENGTH (4 bytes, u32 LE)        │
│  - Length of JSON header in bytes           │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│    FORMAT_VERSION (2 bytes, u16 LE)         │
│  - Current version: 1                       │
└─────────────────────────────────────────────┘
┌─────────────────────────────────────────────┐
│      MAGIC_BYTES (8 bytes)                  │
│  - "ADAPIPE\0" (0x4144415049504500)         │
└─────────────────────────────────────────────┘
```

**Why Reverse Header?**
- **Efficient Reading**: Read magic bytes and version first
- **Validation**: Quickly validate format without reading entire file
- **Streaming**: Process chunk data while reading header
- **Metadata Location**: Header location calculated from end of file

### Magic Bytes

```rust
pub const MAGIC_BYTES: [u8; 8] = [
    0x41, 0x44, 0x41, 0x50, // "ADAP"
    0x49, 0x50, 0x45, 0x00  // "IPE\0"
];
```

**Purpose:**
- Identify files in `.adapipe` format
- Prevent accidental processing of wrong file types
- Enable format detection tools

### Format Version

```rust
pub const CURRENT_FORMAT_VERSION: u16 = 1;
```

**Version History:**
- **Version 1**: Initial format with compression, encryption, checksum support

**Future Versions:**
- Version 2: Enhanced metadata, additional algorithms
- Version 3: Streaming optimizations, compression improvements

## File Header Structure

### Header Fields

The JSON header contains comprehensive metadata:

```rust
pub struct FileHeader {
    /// Application version (e.g., "0.1.0")
    pub app_version: String,

    /// File format version (1)
    pub format_version: u16,

    /// Original input filename
    pub original_filename: String,

    /// Original file size in bytes
    pub original_size: u64,

    /// SHA-256 checksum of original file
    pub original_checksum: String,

    /// SHA-256 checksum of processed file
    pub output_checksum: String,

    /// Processing steps applied (in order)
    pub processing_steps: Vec<ProcessingStep>,

    /// Chunk size used (bytes)
    pub chunk_size: u32,

    /// Number of chunks
    pub chunk_count: u32,

    /// Processing timestamp (RFC3339)
    pub processed_at: DateTime<Utc>,

    /// Pipeline ID
    pub pipeline_id: String,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
```

### Processing Steps

Each processing step records transformation details:

```rust
pub struct ProcessingStep {
    /// Step type (compression, encryption, etc.)
    pub step_type: ProcessingStepType,

    /// Algorithm used (e.g., "brotli", "aes-256-gcm")
    pub algorithm: String,

    /// Algorithm-specific parameters
    pub parameters: HashMap<String, String>,

    /// Application order (0-based)
    pub order: u32,
}

pub enum ProcessingStepType {
    Compression,
    Encryption,
    Checksum,
    PassThrough,
    Custom(String),
}
```

**Example Processing Steps:**

```json
{
  "processing_steps": [
    {
      "step_type": "Compression",
      "algorithm": "brotli",
      "parameters": {
        "level": "6"
      },
      "order": 0
    },
    {
      "step_type": "Encryption",
      "algorithm": "aes-256-gcm",
      "parameters": {
        "key_derivation": "argon2"
      },
      "order": 1
    },
    {
      "step_type": "Checksum",
      "algorithm": "sha256",
      "parameters": {},
      "order": 2
    }
  ]
}
```

## Chunk Format

### Chunk Structure

Each chunk in the processed data section follows this format:

```text
┌────────────────────────────────────┐
│   NONCE (12 bytes)                 │
│  - Unique for each chunk           │
│  - Used for encryption IV          │
└────────────────────────────────────┘
┌────────────────────────────────────┐
│   DATA_LENGTH (4 bytes, u32 LE)    │
│  - Length of encrypted data        │
└────────────────────────────────────┘
┌────────────────────────────────────┐
│   ENCRYPTED_DATA (variable)        │
│  - Compressed and encrypted        │
│  - Includes authentication tag     │
└────────────────────────────────────┘
```

**Rust Structure:**

```rust
pub struct ChunkFormat {
    /// Encryption nonce (12 bytes for AES-GCM)
    pub nonce: [u8; 12],

    /// Length of encrypted data
    pub data_length: u32,

    /// Encrypted (and possibly compressed) chunk data
    pub encrypted_data: Vec<u8>,
}
```

### Chunk Processing

**Forward Processing (Compress → Encrypt):**

```text
1. Read original chunk
2. Compress chunk data
3. Generate unique nonce
4. Encrypt compressed data
5. Write: [NONCE][LENGTH][ENCRYPTED_DATA]
```

**Reverse Processing (Decrypt → Decompress):**

```text
1. Read: [NONCE][LENGTH][ENCRYPTED_DATA]
2. Decrypt using nonce
3. Decompress decrypted data
4. Verify checksum
5. Write original chunk
```

## Creating Binary Files

### Basic File Creation

```rust
use pipeline_domain::value_objects::{FileHeader, ProcessingStep};
use std::fs::File;
use std::io::Write;

fn create_adapipe_file(
    input_data: &[u8],
    output_path: &str,
    processing_steps: Vec<ProcessingStep>,
) -> Result<(), PipelineError> {
    // Create header
    let original_checksum = calculate_sha256(input_data);
    let mut header = FileHeader::new(
        "input.txt".to_string(),
        input_data.len() as u64,
        original_checksum,
    );

    // Add processing steps
    header.processing_steps = processing_steps;
    header.chunk_count = calculate_chunk_count(input_data.len(), header.chunk_size);

    // Process chunks
    let processed_data = process_chunks(input_data, &header.processing_steps)?;

    // Calculate output checksum
    header.output_checksum = calculate_sha256(&processed_data);

    // Serialize header to JSON
    let json_header = serde_json::to_vec(&header)?;
    let header_length = json_header.len() as u32;

    // Write file in reverse order
    let mut file = File::create(output_path)?;

    // 1. Write processed data
    file.write_all(&processed_data)?;

    // 2. Write JSON header
    file.write_all(&json_header)?;

    // 3. Write header length
    file.write_all(&header_length.to_le_bytes())?;

    // 4. Write format version
    file.write_all(&CURRENT_FORMAT_VERSION.to_le_bytes())?;

    // 5. Write magic bytes
    file.write_all(&MAGIC_BYTES)?;

    Ok(())
}
```

### Adding Processing Steps

```rust
impl FileHeader {
    /// Add compression step
    pub fn add_compression_step(mut self, algorithm: &str, level: u32) -> Self {
        let mut parameters = HashMap::new();
        parameters.insert("level".to_string(), level.to_string());

        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Compression,
            algorithm: algorithm.to_string(),
            parameters,
            order: self.processing_steps.len() as u32,
        });

        self
    }

    /// Add encryption step
    pub fn add_encryption_step(
        mut self,
        algorithm: &str,
        key_derivation: &str
    ) -> Self {
        let mut parameters = HashMap::new();
        parameters.insert("key_derivation".to_string(), key_derivation.to_string());

        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Encryption,
            algorithm: algorithm.to_string(),
            parameters,
            order: self.processing_steps.len() as u32,
        });

        self
    }

    /// Add checksum step
    pub fn add_checksum_step(mut self, algorithm: &str) -> Self {
        self.processing_steps.push(ProcessingStep {
            step_type: ProcessingStepType::Checksum,
            algorithm: algorithm.to_string(),
            parameters: HashMap::new(),
            order: self.processing_steps.len() as u32,
        });

        self
    }
}
```

## Reading Binary Files

### Basic File Reading

```rust
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn read_adapipe_file(path: &str) -> Result<FileHeader, PipelineError> {
    let mut file = File::open(path)?;

    // Read from end of file (reverse header)
    file.seek(SeekFrom::End(-8))?;

    // 1. Read and validate magic bytes
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)?;

    if magic != MAGIC_BYTES {
        return Err(PipelineError::InvalidFormat(
            "Not an .adapipe file".to_string()
        ));
    }

    // 2. Read format version
    file.seek(SeekFrom::End(-10))?;
    let mut version_bytes = [0u8; 2];
    file.read_exact(&mut version_bytes)?;
    let version = u16::from_le_bytes(version_bytes);

    if version > CURRENT_FORMAT_VERSION {
        return Err(PipelineError::UnsupportedVersion(version));
    }

    // 3. Read header length
    file.seek(SeekFrom::End(-14))?;
    let mut length_bytes = [0u8; 4];
    file.read_exact(&mut length_bytes)?;
    let header_length = u32::from_le_bytes(length_bytes) as usize;

    // 4. Read JSON header
    file.seek(SeekFrom::End(-(14 + header_length as i64)))?;
    let mut json_data = vec![0u8; header_length];
    file.read_exact(&mut json_data)?;

    // 5. Deserialize header
    let header: FileHeader = serde_json::from_slice(&json_data)?;

    Ok(header)
}
```

### Reading Chunk Data

```rust
fn read_chunks(
    file: &mut File,
    header: &FileHeader
) -> Result<Vec<ChunkFormat>, PipelineError> {
    let mut chunks = Vec::with_capacity(header.chunk_count as usize);

    // Seek to start of chunk data
    file.seek(SeekFrom::Start(0))?;

    for _ in 0..header.chunk_count {
        // Read nonce
        let mut nonce = [0u8; 12];
        file.read_exact(&mut nonce)?;

        // Read data length
        let mut length_bytes = [0u8; 4];
        file.read_exact(&mut length_bytes)?;
        let data_length = u32::from_le_bytes(length_bytes);

        // Read encrypted data
        let mut encrypted_data = vec![0u8; data_length as usize];
        file.read_exact(&mut encrypted_data)?;

        chunks.push(ChunkFormat {
            nonce,
            data_length,
            encrypted_data,
        });
    }

    Ok(chunks)
}
```

## File Recovery

### Complete Recovery Process

```rust
fn restore_original_file(
    input_path: &str,
    output_path: &str,
    password: Option<&str>,
) -> Result<(), PipelineError> {
    // 1. Read header
    let header = read_adapipe_file(input_path)?;

    // 2. Read chunks
    let mut file = File::open(input_path)?;
    let chunks = read_chunks(&mut file, &header)?;

    // 3. Process chunks in reverse order
    let mut restored_data = Vec::new();

    for chunk in chunks {
        let mut chunk_data = chunk.encrypted_data;

        // Reverse processing steps
        for step in header.processing_steps.iter().rev() {
            chunk_data = match step.step_type {
                ProcessingStepType::Encryption => {
                    decrypt_chunk(chunk_data, &chunk.nonce, &step, password)?
                }
                ProcessingStepType::Compression => {
                    decompress_chunk(chunk_data, &step)?
                }
                ProcessingStepType::Checksum => {
                    verify_chunk_checksum(&chunk_data, &step)?;
                    chunk_data
                }
                _ => chunk_data,
            };
        }

        restored_data.extend_from_slice(&chunk_data);
    }

    // 4. Verify restored data
    let restored_checksum = calculate_sha256(&restored_data);
    if restored_checksum != header.original_checksum {
        return Err(PipelineError::IntegrityError(
            "Restored data checksum mismatch".to_string()
        ));
    }

    // 5. Write restored file
    let mut output = File::create(output_path)?;
    output.write_all(&restored_data)?;

    Ok(())
}
```

### Processing Step Reversal

```rust
fn reverse_processing_step(
    data: Vec<u8>,
    step: &ProcessingStep,
    password: Option<&str>,
) -> Result<Vec<u8>, PipelineError> {
    match step.step_type {
        ProcessingStepType::Compression => {
            // Decompress
            match step.algorithm.as_str() {
                "brotli" => decompress_brotli(data),
                "gzip" => decompress_gzip(data),
                "zstd" => decompress_zstd(data),
                "lz4" => decompress_lz4(data),
                _ => Err(PipelineError::UnsupportedAlgorithm(
                    step.algorithm.clone()
                )),
            }
        }
        ProcessingStepType::Encryption => {
            // Decrypt
            let password = password.ok_or(PipelineError::MissingPassword)?;
            match step.algorithm.as_str() {
                "aes-256-gcm" => decrypt_aes_256_gcm(data, password, step),
                "chacha20-poly1305" => decrypt_chacha20(data, password, step),
                _ => Err(PipelineError::UnsupportedAlgorithm(
                    step.algorithm.clone()
                )),
            }
        }
        ProcessingStepType::Checksum => {
            // Verify checksum (no transformation)
            verify_checksum(&data, step)?;
            Ok(data)
        }
        _ => Ok(data),
    }
}
```

## Integrity Verification

### File Validation

```rust
fn validate_adapipe_file(path: &str) -> Result<ValidationReport, PipelineError> {
    let mut report = ValidationReport::new();

    // 1. Read and validate header
    let header = match read_adapipe_file(path) {
        Ok(h) => {
            report.add_check("Header format", true, "Valid");
            h
        }
        Err(e) => {
            report.add_check("Header format", false, &e.to_string());
            return Ok(report);
        }
    };

    // 2. Validate format version
    if header.format_version <= CURRENT_FORMAT_VERSION {
        report.add_check("Format version", true, &format!("v{}", header.format_version));
    } else {
        report.add_check(
            "Format version",
            false,
            &format!("Unsupported: v{}", header.format_version)
        );
    }

    // 3. Validate processing steps
    for (i, step) in header.processing_steps.iter().enumerate() {
        let is_supported = match step.step_type {
            ProcessingStepType::Compression => {
                matches!(step.algorithm.as_str(), "brotli" | "gzip" | "zstd" | "lz4")
            }
            ProcessingStepType::Encryption => {
                matches!(step.algorithm.as_str(), "aes-256-gcm" | "chacha20-poly1305")
            }
            _ => true,
        };

        report.add_check(
            &format!("Step {} ({:?})", i, step.step_type),
            is_supported,
            &step.algorithm
        );
    }

    // 4. Verify output checksum
    let mut file = File::open(path)?;
    let data_length = file.metadata()?.len() - 14 - header.json_size() as u64;
    let mut processed_data = vec![0u8; data_length as usize];
    file.read_exact(&mut processed_data)?;

    let calculated_checksum = calculate_sha256(&processed_data);
    let checksums_match = calculated_checksum == header.output_checksum;

    report.add_check(
        "Output checksum",
        checksums_match,
        if checksums_match { "Valid" } else { "Mismatch" }
    );

    Ok(report)
}

pub struct ValidationReport {
    checks: Vec<(String, bool, String)>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    pub fn add_check(&mut self, name: &str, passed: bool, message: &str) {
        self.checks.push((name.to_string(), passed, message.to_string()));
    }

    pub fn is_valid(&self) -> bool {
        self.checks.iter().all(|(_, passed, _)| *passed)
    }
}
```

### Checksum Verification

```rust
fn verify_file_integrity(path: &str) -> Result<bool, PipelineError> {
    let header = read_adapipe_file(path)?;

    // Calculate actual checksum
    let mut file = File::open(path)?;
    let data_length = file.metadata()?.len() - 14 - header.json_size() as u64;
    let mut data = vec![0u8; data_length as usize];
    file.read_exact(&mut data)?;

    let calculated = calculate_sha256(&data);

    // Compare with stored checksum
    Ok(calculated == header.output_checksum)
}

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
```

## Version Management

### Format Versioning

```rust
pub fn check_format_compatibility(version: u16) -> Result<(), PipelineError> {
    match version {
        1 => Ok(()), // Current version
        v if v < CURRENT_FORMAT_VERSION => {
            // Older version - attempt migration
            migrate_format(v, CURRENT_FORMAT_VERSION)
        }
        v => Err(PipelineError::UnsupportedVersion(v)),
    }
}
```

### Format Migration

```rust
fn migrate_format(from: u16, to: u16) -> Result<(), PipelineError> {
    match (from, to) {
        (1, 2) => {
            // Migration from v1 to v2
            // Add new fields with defaults
            Ok(())
        }
        _ => Err(PipelineError::MigrationUnsupported(from, to)),
    }
}
```

### Backward Compatibility

```rust
fn read_any_version(path: &str) -> Result<FileHeader, PipelineError> {
    let version = read_format_version(path)?;

    match version {
        1 => read_v1_format(path),
        2 => read_v2_format(path),
        v => Err(PipelineError::UnsupportedVersion(v)),
    }
}
```

## Best Practices

### File Creation

**Always set checksums:**

```rust
// ✅ GOOD: Set both checksums
let original_checksum = calculate_sha256(&input_data);
let header = FileHeader::new(filename, size, original_checksum);
// ... process data ...
header.output_checksum = calculate_sha256(&processed_data);
```

**Record all processing steps:**

```rust
// ✅ GOOD: Record every transformation
header = header
    .add_compression_step("brotli", 6)
    .add_encryption_step("aes-256-gcm", "argon2")
    .add_checksum_step("sha256");
```

### File Reading

**Always validate format:**

```rust
// ✅ GOOD: Validate before processing
let header = read_adapipe_file(path)?;

if header.format_version > CURRENT_FORMAT_VERSION {
    return Err(PipelineError::UnsupportedVersion(
        header.format_version
    ));
}
```

**Verify checksums:**

```rust
// ✅ GOOD: Verify integrity
let restored_checksum = calculate_sha256(&restored_data);
if restored_checksum != header.original_checksum {
    return Err(PipelineError::IntegrityError(
        "Checksum mismatch".to_string()
    ));
}
```

### Error Handling

**Handle all error cases:**

```rust
match read_adapipe_file(path) {
    Ok(header) => process_file(header),
    Err(PipelineError::InvalidFormat(msg)) => {
        eprintln!("Not a valid .adapipe file: {}", msg);
    }
    Err(PipelineError::UnsupportedVersion(v)) => {
        eprintln!("Unsupported format version: {}", v);
    }
    Err(e) => {
        eprintln!("Error reading file: {}", e);
    }
}
```

## Security Considerations

### Nonce Management

**Never reuse nonces:**

```rust
// ✅ GOOD: Generate unique nonce per chunk
fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}
```

### Key Derivation

**Use strong key derivation:**

```rust
// ✅ GOOD: Argon2 for password-based encryption
fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    use argon2::{Argon2, PasswordHasher};

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt)
        .unwrap();

    hash.hash.unwrap().as_bytes().to_vec()
}
```

### Integrity Protection

**Verify at every step:**

```rust
// ✅ GOOD: Verify after each transformation
fn process_with_verification(
    data: Vec<u8>,
    step: &ProcessingStep
) -> Result<Vec<u8>, PipelineError> {
    let processed = apply_transformation(data, step)?;
    verify_transformation(&processed, step)?;
    Ok(processed)
}
```

## Next Steps

Now that you understand the binary file format:

- [Chunking Strategy](chunking.md) - Efficient chunk processing
- [File I/O](file-io.md) - File reading and writing patterns
- [Integrity Verification](integrity.md) - Checksum algorithms
- [Encryption](encryption.md) - Encryption implementation details
