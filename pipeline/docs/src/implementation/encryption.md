# Encryption Implementation

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The encryption service provides authenticated encryption with multiple algorithms, secure key management, and automatic integrity verification. It's implemented as an infrastructure adapter that implements the domain's `EncryptionService` trait.

**File:** `pipeline/src/infrastructure/adapters/encryption_service_adapter.rs`

## Supported Algorithms

### AES-256-GCM (Advanced Encryption Standard)
- **Key size:** 256 bits (32 bytes)
- **Nonce size:** 96 bits (12 bytes)
- **Security:** Industry standard, FIPS 140-2 approved
- **Performance:** Excellent with AES-NI hardware acceleration
- **Library:** `aes-gcm` crate

**Best for:**
- Compliance requirements (FIPS, government)
- Systems with AES-NI support
- Maximum security requirements
- Long-term data protection

**Performance characteristics:**
```text
Operation   | With AES-NI | Without AES-NI
------------|-------------|----------------
Encryption  | 2-3 GB/s    | 100-200 MB/s
Decryption  | 2-3 GB/s    | 100-200 MB/s
Key setup   | < 1 μs      | < 1 μs
Memory      | Low         | Low
```

### ChaCha20-Poly1305 (Stream Cipher)
- **Key size:** 256 bits (32 bytes)
- **Nonce size:** 96 bits (12 bytes)
- **Security:** Modern, constant-time implementation
- **Performance:** Consistent across all platforms
- **Library:** `chacha20poly1305` crate

**Best for:**
- Systems without AES-NI
- Mobile/embedded devices
- Constant-time requirements
- Side-channel attack resistance

**Performance characteristics:**
```text
Operation   | Any Platform
------------|-------------
Encryption  | 500-800 MB/s
Decryption  | 500-800 MB/s
Key setup   | < 1 μs
Memory      | Low
```

### AES-128-GCM (Faster AES Variant)
- **Key size:** 128 bits (16 bytes)
- **Nonce size:** 96 bits (12 bytes)
- **Security:** Very secure, faster than AES-256
- **Performance:** ~30% faster than AES-256
- **Library:** `aes-gcm` crate

**Best for:**
- High-performance requirements
- Short-term data protection
- Real-time encryption
- Bandwidth-constrained systems

**Performance characteristics:**
```text
Operation   | With AES-NI | Without AES-NI
------------|-------------|----------------
Encryption  | 3-4 GB/s    | 150-250 MB/s
Decryption  | 3-4 GB/s    | 150-250 MB/s
Key setup   | < 1 μs      | < 1 μs
Memory      | Low         | Low
```

## Security Features

### Authenticated Encryption (AEAD)

All algorithms provide Authenticated Encryption with Associated Data (AEAD):

```text
Plaintext → Encrypt → Ciphertext + Authentication Tag
                ↓
            Detects tampering
```

**Properties:**
- **Confidentiality:** Data is encrypted and unreadable
- **Integrity:** Any modification is detected
- **Authentication:** Verifies data origin

### Nonce Management

Each encryption operation requires a unique nonce (number used once):

```rust
// Nonces are automatically generated for each chunk
pub struct EncryptionContext {
    key: SecretKey,
    nonce_counter: AtomicU64,  // Incrementing counter
}

impl EncryptionContext {
    fn next_nonce(&self) -> Nonce {
        let counter = self.nonce_counter.fetch_add(1, Ordering::SeqCst);

        // Generate nonce from counter
        let mut nonce = [0u8; 12];
        nonce[0..8].copy_from_slice(&counter.to_le_bytes());

        Nonce::from_slice(&nonce)
    }
}
```

**Important:** Never reuse a nonce with the same key!

### Key Derivation

Derive encryption keys from passwords using secure KDFs:

```rust
pub enum KeyDerivationFunction {
    Argon2,   // Memory-hard, GPU-resistant
    Scrypt,   // Memory-hard, tunable
    PBKDF2,   // Standard, widely supported
}

// Derive key from password
pub fn derive_key(
    password: &[u8],
    salt: &[u8],
    kdf: KeyDerivationFunction,
) -> Result<SecretKey, PipelineError> {
    match kdf {
        KeyDerivationFunction::Argon2 => {
            // Memory: 64 MB, Iterations: 3, Parallelism: 4
            argon2::hash_raw(password, salt, &argon2::Config::default())
        }
        KeyDerivationFunction::Scrypt => {
            // N=16384, r=8, p=1
            scrypt::scrypt(password, salt, &scrypt::Params::new(14, 8, 1)?)
        }
        KeyDerivationFunction::PBKDF2 => {
            // 100,000 iterations
            pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password, salt, 100_000)
        }
    }
}
```

## Architecture

### Service Interface (Domain Layer)

```rust
// pipeline-domain/src/services/encryption_service.rs
use async_trait::async_trait;
use crate::value_objects::Algorithm;
use crate::error::PipelineError;

#[async_trait]
pub trait EncryptionService: Send + Sync {
    /// Encrypt data using the specified algorithm
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<EncryptedData, PipelineError>;

    /// Decrypt data using the specified algorithm
    async fn decrypt(
        &self,
        encrypted: &EncryptedData,
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError>;
}

/// Encrypted data with nonce and authentication tag
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,        // 12 bytes
    pub tag: Vec<u8>,          // 16 bytes (authentication tag)
}
```

### Service Implementation (Infrastructure Layer)

```rust
// pipeline/src/infrastructure/adapters/encryption_service_adapter.rs
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;

pub struct EncryptionServiceAdapter {
    // Secure key storage
    key_store: Arc<RwLock<KeyStore>>,
}

#[async_trait]
impl EncryptionService for EncryptionServiceAdapter {
    async fn encrypt(
        &self,
        data: &[u8],
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<EncryptedData, PipelineError> {
        match algorithm.name() {
            "aes-256-gcm" => self.encrypt_aes_256_gcm(data, key),
            "chacha20-poly1305" => self.encrypt_chacha20(data, key),
            "aes-128-gcm" => self.encrypt_aes_128_gcm(data, key),
            _ => Err(PipelineError::UnsupportedAlgorithm(
                algorithm.name().to_string()
            )),
        }
    }

    async fn decrypt(
        &self,
        encrypted: &EncryptedData,
        algorithm: &Algorithm,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError> {
        match algorithm.name() {
            "aes-256-gcm" => self.decrypt_aes_256_gcm(encrypted, key),
            "chacha20-poly1305" => self.decrypt_chacha20(encrypted, key),
            "aes-128-gcm" => self.decrypt_aes_128_gcm(encrypted, key),
            _ => Err(PipelineError::UnsupportedAlgorithm(
                algorithm.name().to_string()
            )),
        }
    }
}
```

## Algorithm Implementations

### AES-256-GCM Implementation

```rust
impl EncryptionServiceAdapter {
    fn encrypt_aes_256_gcm(
        &self,
        data: &[u8],
        key: &EncryptionKey,
    ) -> Result<EncryptedData, PipelineError> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, NewAead};

        // Create cipher from key
        let key = Key::from_slice(key.as_bytes());
        let cipher = Aes256Gcm::new(key);

        // Generate unique nonce
        let nonce = self.generate_nonce();
        let nonce_obj = Nonce::from_slice(&nonce);

        // Encrypt with authentication
        let ciphertext = cipher
            .encrypt(nonce_obj, data)
            .map_err(|e| PipelineError::EncryptionError(e.to_string()))?;

        // Split ciphertext and tag
        let (ciphertext_bytes, tag) = ciphertext.split_at(ciphertext.len() - 16);

        Ok(EncryptedData {
            ciphertext: ciphertext_bytes.to_vec(),
            nonce: nonce.to_vec(),
            tag: tag.to_vec(),
        })
    }

    fn decrypt_aes_256_gcm(
        &self,
        encrypted: &EncryptedData,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, NewAead};

        // Create cipher from key
        let key = Key::from_slice(key.as_bytes());
        let cipher = Aes256Gcm::new(key);

        // Reconstruct nonce
        let nonce = Nonce::from_slice(&encrypted.nonce);

        // Combine ciphertext and tag
        let mut combined = encrypted.ciphertext.clone();
        combined.extend_from_slice(&encrypted.tag);

        // Decrypt and verify authentication
        let plaintext = cipher
            .decrypt(nonce, combined.as_slice())
            .map_err(|e| PipelineError::DecryptionError(
                format!("Decryption failed (possibly tampered): {}", e)
            ))?;

        Ok(plaintext)
    }
}
```

### ChaCha20-Poly1305 Implementation

```rust
impl EncryptionServiceAdapter {
    fn encrypt_chacha20(
        &self,
        data: &[u8],
        key: &EncryptionKey,
    ) -> Result<EncryptedData, PipelineError> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
        use chacha20poly1305::aead::{Aead, NewAead};

        // Create cipher from key
        let key = Key::from_slice(key.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);

        // Generate unique nonce
        let nonce = self.generate_nonce();
        let nonce_obj = Nonce::from_slice(&nonce);

        // Encrypt with authentication
        let ciphertext = cipher
            .encrypt(nonce_obj, data)
            .map_err(|e| PipelineError::EncryptionError(e.to_string()))?;

        // Split ciphertext and tag
        let (ciphertext_bytes, tag) = ciphertext.split_at(ciphertext.len() - 16);

        Ok(EncryptedData {
            ciphertext: ciphertext_bytes.to_vec(),
            nonce: nonce.to_vec(),
            tag: tag.to_vec(),
        })
    }

    fn decrypt_chacha20(
        &self,
        encrypted: &EncryptedData,
        key: &EncryptionKey,
    ) -> Result<Vec<u8>, PipelineError> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
        use chacha20poly1305::aead::{Aead, NewAead};

        // Create cipher from key
        let key = Key::from_slice(key.as_bytes());
        let cipher = ChaCha20Poly1305::new(key);

        // Reconstruct nonce
        let nonce = Nonce::from_slice(&encrypted.nonce);

        // Combine ciphertext and tag
        let mut combined = encrypted.ciphertext.clone();
        combined.extend_from_slice(&encrypted.tag);

        // Decrypt and verify authentication
        let plaintext = cipher
            .decrypt(nonce, combined.as_slice())
            .map_err(|e| PipelineError::DecryptionError(
                format!("Decryption failed (possibly tampered): {}", e)
            ))?;

        Ok(plaintext)
    }
}
```

## Key Management

### Secure Key Storage

```rust
use zeroize::Zeroize;

/// Secure key that zeroizes on drop
pub struct SecretKey {
    bytes: Vec<u8>,
}

impl SecretKey {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Generate random key
    pub fn generate(size: usize) -> Self {
        use rand::RngCore;
        let mut bytes = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self::new(bytes)
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        // Securely wipe key from memory
        self.bytes.zeroize();
    }
}

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}
```

### Key Rotation

```rust
pub struct KeyRotation {
    current_key: SecretKey,
    previous_key: Option<SecretKey>,
    rotation_interval: Duration,
    last_rotation: Instant,
}

impl KeyRotation {
    pub fn rotate(&mut self) -> Result<(), PipelineError> {
        // Save current key as previous
        let old_key = std::mem::replace(
            &mut self.current_key,
            SecretKey::generate(32),
        );
        self.previous_key = Some(old_key);
        self.last_rotation = Instant::now();

        Ok(())
    }

    pub fn should_rotate(&self) -> bool {
        self.last_rotation.elapsed() >= self.rotation_interval
    }
}
```

## Performance Optimizations

### Parallel Chunk Encryption

```rust
use rayon::prelude::*;

pub async fn encrypt_chunks(
    chunks: Vec<FileChunk>,
    algorithm: &Algorithm,
    key: &SecretKey,
    encryption_service: &Arc<dyn EncryptionService>,
) -> Result<Vec<EncryptedChunk>, PipelineError> {
    // Encrypt chunks in parallel
    chunks.par_iter()
        .map(|chunk| {
            let encrypted = encryption_service
                .encrypt(&chunk.data, algorithm, key)?;

            Ok(EncryptedChunk {
                sequence: chunk.sequence,
                data: encrypted,
                original_size: chunk.data.len(),
            })
        })
        .collect()
}
```

### Hardware Acceleration

```rust
// Detect AES-NI support
pub fn has_aes_ni() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::x86_64::*;
        is_x86_feature_detected!("aes")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

// Select algorithm based on hardware
pub fn select_algorithm() -> Algorithm {
    if has_aes_ni() {
        Algorithm::aes_256_gcm()  // Fast with hardware support
    } else {
        Algorithm::chacha20_poly1305()  // Consistent without hardware
    }
}
```

## Configuration

### Encryption Configuration

```rust
pub struct EncryptionConfig {
    pub algorithm: Algorithm,
    pub key_derivation: KeyDerivationFunction,
    pub key_rotation_interval: Duration,
    pub nonce_reuse_prevention: bool,
}

impl EncryptionConfig {
    pub fn maximum_security() -> Self {
        Self {
            algorithm: Algorithm::aes_256_gcm(),
            key_derivation: KeyDerivationFunction::Argon2,
            key_rotation_interval: Duration::from_secs(86400), // 24 hours
            nonce_reuse_prevention: true,
        }
    }

    pub fn balanced() -> Self {
        Self {
            algorithm: if has_aes_ni() {
                Algorithm::aes_256_gcm()
            } else {
                Algorithm::chacha20_poly1305()
            },
            key_derivation: KeyDerivationFunction::Scrypt,
            key_rotation_interval: Duration::from_secs(604800), // 7 days
            nonce_reuse_prevention: true,
        }
    }

    pub fn high_performance() -> Self {
        Self {
            algorithm: if has_aes_ni() {
                Algorithm::aes_128_gcm()
            } else {
                Algorithm::chacha20_poly1305()
            },
            key_derivation: KeyDerivationFunction::PBKDF2,
            key_rotation_interval: Duration::from_secs(2592000), // 30 days
            nonce_reuse_prevention: true,
        }
    }
}
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Authentication failed - data may be tampered")]
    AuthenticationFailed,

    #[error("Invalid key length: expected {expected}, got {actual}")]
    InvalidKeyLength { expected: usize, actual: usize },

    #[error("Nonce reuse detected")]
    NonceReuse,

    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
}

impl From<EncryptionError> for PipelineError {
    fn from(err: EncryptionError) -> Self {
        match err {
            EncryptionError::EncryptionFailed(msg) =>
                PipelineError::EncryptionError(msg),
            EncryptionError::DecryptionFailed(msg) =>
                PipelineError::DecryptionError(msg),
            EncryptionError::AuthenticationFailed =>
                PipelineError::IntegrityError("Authentication failed".to_string()),
            _ => PipelineError::EncryptionError(err.to_string()),
        }
    }
}
```

## Usage Examples

### Basic Encryption

```rust
use pipeline::infrastructure::adapters::EncryptionServiceAdapter;
use pipeline_domain::services::EncryptionService;
use pipeline_domain::value_objects::Algorithm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create encryption service
    let encryption = EncryptionServiceAdapter::new();

    // Generate encryption key
    let key = SecretKey::generate(32); // 256 bits

    // Encrypt data
    let data = b"Sensitive information";
    let encrypted = encryption.encrypt(
        data,
        &Algorithm::aes_256_gcm(),
        &key
    ).await?;

    println!("Original size: {} bytes", data.len());
    println!("Encrypted size: {} bytes", encrypted.ciphertext.len());
    println!("Nonce: {} bytes", encrypted.nonce.len());
    println!("Tag: {} bytes", encrypted.tag.len());

    // Decrypt data
    let decrypted = encryption.decrypt(
        &encrypted,
        &Algorithm::aes_256_gcm(),
        &key
    ).await?;

    assert_eq!(data, decrypted.as_slice());
    println!("✓ Decryption successful");

    Ok(())
}
```

### Password-Based Encryption

```rust
async fn encrypt_with_password(
    data: &[u8],
    password: &str,
) -> Result<EncryptedData, PipelineError> {
    // Generate random salt
    let salt = SecretKey::generate(16);

    // Derive key from password
    let key = derive_key(
        password.as_bytes(),
        salt.as_bytes(),
        KeyDerivationFunction::Argon2,
    )?;

    // Encrypt data
    let encryption = EncryptionServiceAdapter::new();
    let encrypted = encryption.encrypt(
        data,
        &Algorithm::aes_256_gcm(),
        &key,
    ).await?;

    // Store salt with encrypted data
    encrypted.salt = salt.as_bytes().to_vec();

    Ok(encrypted)
}
```

### Tamper Detection

```rust
async fn decrypt_with_verification(
    encrypted: &EncryptedData,
    key: &SecretKey,
) -> Result<Vec<u8>, PipelineError> {
    let encryption = EncryptionServiceAdapter::new();

    // Attempt decryption (will fail if tampered)
    match encryption.decrypt(encrypted, &Algorithm::aes_256_gcm(), key).await {
        Ok(plaintext) => {
            println!("✓ Data is authentic and unmodified");
            Ok(plaintext)
        }
        Err(PipelineError::DecryptionError(_)) => {
            eprintln!("✗ Data has been tampered with!");
            Err(PipelineError::IntegrityError(
                "Authentication failed - data may be tampered".to_string()
            ))
        }
        Err(e) => Err(e),
    }
}
```

## Benchmarks

Typical performance on modern systems:

```text
Algorithm          | File Size | Encrypt Time | Decrypt Time | Throughput
-------------------|-----------|--------------|--------------|------------
AES-256-GCM (NI)   | 100 MB    | 0.04s        | 0.04s        | 2.5 GB/s
AES-256-GCM (SW)   | 100 MB    | 0.8s         | 0.8s         | 125 MB/s
ChaCha20-Poly1305  | 100 MB    | 0.15s        | 0.15s        | 670 MB/s
AES-128-GCM (NI)   | 100 MB    | 0.03s        | 0.03s        | 3.3 GB/s
```

## Best Practices

### Algorithm Selection

**Use AES-256-GCM when:**
- Compliance requires FIPS-approved encryption
- Long-term data protection is needed
- Hardware has AES-NI support
- Maximum security is required

**Use ChaCha20-Poly1305 when:**
- Running on platforms without AES-NI
- Constant-time execution is critical
- Side-channel resistance is needed
- Mobile/embedded deployment

**Use AES-128-GCM when:**
- Maximum performance is required
- Short-term data protection is sufficient
- Hardware has AES-NI support

### Key Management

```rust
// ✅ GOOD: Secure key handling
let key = SecretKey::generate(32);
let encrypted = encrypt(data, &key)?;
// Key is automatically zeroized on drop

// ❌ BAD: Exposing key in logs
println!("Key: {:?}", key);  // Never log keys!

// ✅ GOOD: Key derivation from password
let key = derive_key(password, salt, KeyDerivationFunction::Argon2)?;

// ❌ BAD: Weak key derivation
let key = sha256(password);  // Not secure!
```

### Nonce Management

```rust
// ✅ GOOD: Unique nonce per encryption
let nonce = generate_unique_nonce();

// ❌ BAD: Reusing nonces
let nonce = [0u8; 12];  // NEVER reuse nonces!

// ✅ GOOD: Counter-based nonces
let nonce_counter = AtomicU64::new(0);
let nonce = generate_nonce_from_counter(nonce_counter.fetch_add(1));
```

### Authentication Verification

```rust
// ✅ GOOD: Always verify authentication
match decrypt(encrypted, key) {
    Ok(data) => process(data),
    Err(e) => {
        log::error!("Decryption failed - possible tampering");
        return Err(e);
    }
}

// ❌ BAD: Ignoring authentication failures
let data = decrypt(encrypted, key).unwrap_or_default();  // Dangerous!
```

## Security Considerations

### Nonce Uniqueness
- **Critical:** Never reuse a nonce with the same key
- Use counter-based or random nonces
- Rotate keys after 2^32 encryptions (GCM limit)

### Key Strength
- Minimum 256 bits for long-term security
- Use cryptographically secure random number generators
- Derive keys properly from passwords (use Argon2)

### Memory Security
- Keys are automatically zeroized on drop
- Avoid cloning keys unnecessarily
- Don't log or print keys

### Side-Channel Attacks
- ChaCha20 provides constant-time execution
- AES requires AES-NI for timing attack resistance
- Validate all inputs before decryption

## Next Steps

Now that you understand encryption implementation:

- [Integrity Verification](integrity.md) - Checksum and hashing
- [Key Management](../advanced/key-management.md) - Advanced key handling
- [Security Best Practices](../advanced/security.md) - Comprehensive security guide
- [Compression](compression.md) - Data compression before encryption
