//! # Encryption Service
//!
//! This module provides domain-level encryption services for the adaptive
//! pipeline system. It defines the encryption service interface and related
//! types for handling data encryption and decryption operations within the
//! pipeline processing workflow.
//!
//! ## Overview
//!
//! The encryption service provides:
//!
//! - **Algorithm Support**: Multiple encryption algorithms (AES-256-GCM,
//!   ChaCha20-Poly1305)
//! - **Key Management**: Secure key generation, derivation, and storage
//! - **Authenticated Encryption**: Built-in integrity verification and
//!   authentication
//! - **Streaming Processing**: Chunk-by-chunk encryption for large files
//! - **Security Context**: Integration with security policies and access
//!   controls
//!
//! ## Architecture
//!
//! The encryption service follows Domain-Driven Design principles:
//!
//! - **Domain Service**: `EncryptionService` trait defines the contract
//! - **Configuration**: `EncryptionConfig` encapsulates encryption parameters
//! - **Algorithms**: `EncryptionAlgorithm` enum provides type-safe algorithm
//!   selection
//! - **Key Management**: `EncryptionKey` handles secure key operations
//! - **Security Context**: Integration with access control and security
//!   policies
//!
//! ## Security Features
//!
//! ### Authenticated Encryption
//!
//! All encryption algorithms provide authenticated encryption with associated
//! data (AEAD):
//! - **Confidentiality**: Data is encrypted and unreadable without the key
//! - **Integrity**: Tampering is detected through authentication tags
//! - **Authentication**: Verifies data origin and prevents forgery
//!
//! ### Key Derivation
//!
//! Secure key derivation from passwords or key material:
//! - **Argon2**: Memory-hard function resistant to GPU attacks
//! - **Scrypt**: Memory-hard function with tunable parameters
//! - **PBKDF2**: Standard key derivation with configurable iterations
//!
//! ### Memory Security
//!
//! Sensitive data is protected in memory:
//! - **Zeroization**: Keys are securely wiped from memory
//! - **Secure Storage**: Minimal exposure of sensitive material
//! - **Drop Safety**: Automatic cleanup on scope exit
//!
//! ## Usage Examples
//!
//! ### Basic Encryption

//!
//! ### Key Management

//!
//! ## Performance Considerations
//!
//! ### Algorithm Characteristics
//!
//! | Algorithm        | Speed | Security | Key Size | Nonce Size |
//! |------------------|-------|----------|----------|------------|
//! | AES-256-GCM      | Fast  | High     | 32 bytes | 12 bytes   |
//! | ChaCha20-Poly1305| Fast  | High     | 32 bytes | 12 bytes   |
//! | AES-128-GCM      | Faster| High     | 16 bytes | 12 bytes   |
//! | AES-192-GCM      | Fast  | High     | 24 bytes | 12 bytes   |
//!
//! ### Key Derivation Performance
//!
//! - **Argon2**: Slower but more secure against GPU attacks
//! - **Scrypt**: Balanced performance and security
//! - **PBKDF2**: Faster but less resistant to specialized attacks
//!
//! ## Error Handling
//!
//! The encryption service handles various error conditions:
//!
//! - **Encryption Failures**: Algorithm-specific errors
//! - **Key Derivation Errors**: Invalid parameters or insufficient entropy
//! - **Authentication Failures**: Tampering detection during decryption
//! - **Configuration Errors**: Invalid algorithms or parameters
//!
//! ## Thread Safety
//!
//! All encryption service implementations are thread-safe and can be used
//! concurrently across multiple threads. The service maintains no mutable state
//! and all operations are stateless.
//!
//! ## Integration
//!
//! The encryption service integrates with:
//!
//! - **Security Context**: Access control and security policies
//! - **Pipeline Processing**: Core pipeline stage processing
//! - **Key Management**: Secure key storage and retrieval
//! - **Audit Logging**: Security event tracking and compliance

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::services::datetime_serde;
use crate::value_objects::EncryptionBenchmark;
use crate::{FileChunk, PipelineError, ProcessingContext, SecurityContext};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Encryption algorithms supported by the adaptive pipeline system
///
/// This enum provides type-safe selection of encryption algorithms with
/// different performance characteristics and security properties. All
/// algorithms provide authenticated encryption with associated data (AEAD) for
/// both confidentiality and integrity protection.
///
/// # Algorithm Characteristics
///
/// - **AES-256-GCM**: Industry standard with 256-bit keys, excellent
///   performance
/// - **ChaCha20-Poly1305**: Modern stream cipher, constant-time implementation
/// - **AES-128-GCM**: Faster variant with 128-bit keys, still highly secure
/// - **AES-192-GCM**: Middle ground with 192-bit keys
/// - **Custom**: User-defined algorithms for specialized requirements
///
/// # Security Properties
///
/// All algorithms provide:
/// - **Confidentiality**: Data is encrypted and unreadable without the key
/// - **Integrity**: Tampering is detected through authentication tags
/// - **Authentication**: Verifies data origin and prevents forgery
/// - **Semantic Security**: Identical plaintexts produce different ciphertexts
///
/// # Examples
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    Aes128Gcm,
    Aes192Gcm,
    Custom(String),
}

/// Key derivation functions for secure key generation from passwords or key
/// material
///
/// This enum provides type-safe selection of key derivation functions (KDFs)
/// with different security properties and performance characteristics. All
/// functions are designed to be computationally expensive to resist brute-force
/// attacks.
///
/// # Function Characteristics
///
/// - **Argon2**: Memory-hard function, winner of Password Hashing Competition
/// - **Scrypt**: Memory-hard function with tunable parameters
/// - **PBKDF2**: Standard function with configurable iterations
/// - **Custom**: User-defined functions for specialized requirements
///
/// # Security Properties
///
/// - **Argon2**: Resistant to GPU and ASIC attacks, configurable memory and
///   time costs
/// - **Scrypt**: Good resistance to hardware attacks, balanced memory/time
///   trade-offs
/// - **PBKDF2**: Widely supported, but more vulnerable to specialized hardware
///   attacks
///
/// # Performance Considerations
///
/// | Function | Speed | Memory Usage | GPU Resistance | ASIC Resistance |
/// |----------|-------|--------------|----------------|------------------|
/// | Argon2   | Slow  | High         | Excellent      | Excellent        |
/// | Scrypt   | Medium| Medium       | Good           | Good             |
/// | PBKDF2   | Fast  | Low          | Poor           | Poor             |
///
/// # Examples
///
#[derive(Debug, Clone, PartialEq)]
pub enum KeyDerivationFunction {
    /// Argon2 - Memory-hard function resistant to GPU and ASIC attacks
    /// Winner of the Password Hashing Competition, provides excellent security
    Argon2,

    /// Scrypt - Memory-hard function with tunable parameters
    /// Good balance of security and performance
    Scrypt,

    /// PBKDF2 - Standard key derivation function
    /// Widely supported but less resistant to specialized attacks
    Pbkdf2,

    /// Custom key derivation function for specialized requirements
    Custom(String),
}

/// Encryption configuration that encapsulates all parameters for encryption
/// operations
///
/// This configuration struct provides comprehensive control over encryption
/// behavior, including algorithm selection, key derivation parameters, and
/// security settings. The configuration is immutable and thread-safe.
///
/// # Configuration Parameters
///
/// - **Algorithm**: The encryption algorithm to use
/// - **Key Derivation**: Function for deriving keys from passwords
/// - **Key Size**: Size of encryption keys in bytes
/// - **Nonce Size**: Size of nonces/initialization vectors in bytes
/// - **Salt Size**: Size of salt for key derivation in bytes
/// - **Iterations**: Number of iterations for key derivation
/// - **Memory Cost**: Memory usage for memory-hard functions (optional)
/// - **Parallel Cost**: Parallelism level for key derivation (optional)
/// - **Associated Data**: Additional authenticated data (optional)
///
/// # Examples
///
///
/// # Security Considerations
///
/// - **Key Size**: Larger keys provide better security but may impact
///   performance
/// - **Iterations**: Higher iteration counts increase security but slow key
///   derivation
/// - **Memory Cost**: Higher memory usage improves resistance to attacks
/// - **Salt Size**: Larger salts prevent rainbow table attacks
/// - **Associated Data**: Additional data authenticated but not encrypted
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// The encryption algorithm to use for processing
    pub algorithm: EncryptionAlgorithm,

    /// Key derivation function for generating keys from passwords
    pub key_derivation: KeyDerivationFunction,

    /// Size of encryption keys in bytes
    pub key_size: u32,

    /// Size of nonces/initialization vectors in bytes
    pub nonce_size: u32,

    /// Size of salt for key derivation in bytes
    pub salt_size: u32,

    /// Number of iterations for key derivation functions
    pub iterations: u32,

    /// Memory cost for memory-hard functions (bytes)
    pub memory_cost: Option<u32>,

    /// Parallelism level for key derivation functions
    pub parallel_cost: Option<u32>,

    /// Additional authenticated data (not encrypted)
    pub associated_data: Option<Vec<u8>>,
}

/// Key material for encryption/decryption operations with secure memory
/// management
///
/// This struct contains all cryptographic material needed for encryption and
/// decryption operations. It implements secure memory management through the
/// `Zeroize` trait to ensure sensitive data is properly cleared from memory
/// when no longer needed.
///
/// # Security Features
///
/// - **Automatic Zeroization**: Keys are securely wiped from memory on drop
/// - **Expiration Support**: Keys can have expiration times for security
///   policies
/// - **Algorithm Binding**: Keys are bound to specific algorithms
/// - **Timestamp Tracking**: Creation time tracking for audit and compliance
///
/// # Key Material Components
///
/// - **Key**: The actual encryption/decryption key
/// - **Nonce**: Unique number used once per encryption operation
/// - **Salt**: Random data used in key derivation
/// - **Algorithm**: The encryption algorithm this key is for
/// - **Created At**: When the key material was generated
/// - **Expires At**: Optional expiration time for key rotation
///
/// # Examples
///
///
/// # Memory Security
///
/// The key material implements `Zeroize` to ensure sensitive data is securely
/// cleared from memory:
///
///
/// # Serialization
///
/// Key material can be serialized for storage, but care must be taken to:
/// - Encrypt serialized key material
/// - Use secure storage mechanisms
/// - Implement proper access controls
/// - Follow key management best practices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMaterial {
    /// The encryption/decryption key (sensitive data)
    pub key: Vec<u8>,

    /// Nonce/initialization vector for encryption operations
    pub nonce: Vec<u8>,

    /// Salt used in key derivation (if applicable)
    pub salt: Vec<u8>,

    /// The encryption algorithm this key material is for
    pub algorithm: EncryptionAlgorithm,

    /// When this key material was created (RFC3339 format)
    #[serde(with = "datetime_serde")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Optional expiration time for key rotation (RFC3339 format)
    #[serde(with = "datetime_serde::optional")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Zeroize for KeyMaterial {
    fn zeroize(&mut self) {
        self.key.zeroize();
        self.nonce.zeroize();
        self.salt.zeroize();
    }
}

impl ZeroizeOnDrop for KeyMaterial {}

impl KeyMaterial {
    pub fn len(&self) -> usize {
        self.key.len()
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
    }

    pub fn new(key: Vec<u8>, nonce: Vec<u8>, salt: Vec<u8>, algorithm: EncryptionAlgorithm) -> Self {
        Self {
            key,
            nonce,
            salt,
            algorithm,
            created_at: chrono::Utc::now(),
            expires_at: None,
        }
    }
}

/// Domain service interface for encryption operations
#[async_trait]
pub trait EncryptionService: Send + Sync {
    /// Encrypts a file chunk
    async fn encrypt_chunk(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Decrypts a file chunk
    async fn decrypt_chunk(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError>;

    /// Encrypts multiple chunks in parallel
    async fn encrypt_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Decrypts multiple chunks in parallel
    async fn decrypt_chunks_parallel(
        &self,
        chunks: Vec<FileChunk>,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<Vec<FileChunk>, PipelineError>;

    /// Derives key material from password
    async fn derive_key_material(
        &self,
        password: &str,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError>;

    /// Generates random key material
    async fn generate_key_material(
        &self,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError>;

    /// Validates encryption configuration
    async fn validate_config(&self, config: &EncryptionConfig) -> Result<(), PipelineError>;

    /// Gets supported algorithms
    fn supported_algorithms(&self) -> Vec<EncryptionAlgorithm>;

    /// Benchmarks encryption performance
    async fn benchmark_algorithm(
        &self,
        algorithm: &EncryptionAlgorithm,
        test_data: &[u8],
    ) -> Result<EncryptionBenchmark, PipelineError>;

    /// Securely wipes key material from memory
    async fn wipe_key_material(&self, key_material: &mut KeyMaterial) -> Result<(), PipelineError>;

    /// Stores key material securely (HSM integration)
    async fn store_key_material(
        &self,
        key_material: &KeyMaterial,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<(), PipelineError>;

    /// Retrieves key material securely (HSM integration)
    async fn retrieve_key_material(
        &self,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError>;

    /// Rotates encryption keys
    async fn rotate_keys(
        &self,
        old_key_id: &str,
        new_config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<String, PipelineError>;
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_derivation: KeyDerivationFunction::Argon2,
            key_size: 32,   // 256 bits
            nonce_size: 12, // 96 bits for GCM
            salt_size: 16,  // 128 bits
            iterations: 100_000,
            memory_cost: Some(65536), // 64MB for Argon2
            parallel_cost: Some(1),
            associated_data: None,
        }
    }
}

impl EncryptionConfig {
    /// Creates a new encryption configuration
    pub fn new(algorithm: EncryptionAlgorithm) -> Self {
        Self {
            algorithm,
            ..Default::default()
        }
    }

    /// Sets key derivation function
    pub fn with_key_derivation(mut self, kdf: KeyDerivationFunction) -> Self {
        self.key_derivation = kdf;
        self
    }

    /// Sets key size
    pub fn with_key_size(mut self, size: u32) -> Self {
        self.key_size = size;
        self
    }

    /// Sets iterations
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    /// Sets memory cost (for Argon2)
    pub fn with_memory_cost(mut self, cost: u32) -> Self {
        self.memory_cost = Some(cost);
        self
    }

    /// Sets parallel cost (for Argon2)
    pub fn with_parallel_cost(mut self, cost: u32) -> Self {
        self.parallel_cost = Some(cost);
        self
    }

    /// Sets associated data
    pub fn with_associated_data(mut self, data: Vec<u8>) -> Self {
        self.associated_data = Some(data);
        self
    }

    /// Creates a high-security configuration
    pub fn high_security() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_derivation: KeyDerivationFunction::Argon2,
            key_size: 32,
            nonce_size: 12,
            salt_size: 32,              // Larger salt
            iterations: 1_000_000,      // More iterations
            memory_cost: Some(1048576), // 1GB for Argon2
            parallel_cost: Some(4),
            associated_data: None,
        }
    }

    /// Creates a performance-optimized configuration
    pub fn performance_optimized() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            key_derivation: KeyDerivationFunction::Argon2,
            key_size: 32,
            nonce_size: 12,
            salt_size: 16,
            iterations: 10_000,      // Fewer iterations
            memory_cost: Some(8192), // 8MB for Argon2
            parallel_cost: Some(1),
            associated_data: None,
        }
    }
}

impl KeyMaterial {
    /// Sets expiration time
    pub fn with_expiration(mut self, expires_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Checks if key material is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Securely clears key material
    pub fn clear(&mut self) {
        // Zero out sensitive data
        self.key.fill(0);
        self.nonce.fill(0);
        self.salt.fill(0);

        // Clear vectors
        self.key.clear();
        self.nonce.clear();
        self.salt.clear();

        // Shrink to free memory
        self.key.shrink_to_fit();
        self.nonce.shrink_to_fit();
        self.salt.shrink_to_fit();
    }

    /// Gets key size in bytes
    pub fn key_size(&self) -> usize {
        self.key.len()
    }

    /// Gets nonce size in bytes
    pub fn nonce_size(&self) -> usize {
        self.nonce.len()
    }

    /// Gets salt size in bytes
    pub fn salt_size(&self) -> usize {
        self.salt.len()
    }
}

impl Drop for KeyMaterial {
    fn drop(&mut self) {
        self.clear();
    }
}

impl std::fmt::Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES-256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            EncryptionAlgorithm::Aes128Gcm => write!(f, "AES-128-GCM"),
            EncryptionAlgorithm::Aes192Gcm => write!(f, "AES-192-GCM"),
            EncryptionAlgorithm::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

impl std::fmt::Display for KeyDerivationFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyDerivationFunction::Argon2 => write!(f, "Argon2"),
            KeyDerivationFunction::Scrypt => write!(f, "scrypt"),
            KeyDerivationFunction::Pbkdf2 => write!(f, "PBKDF2"),
            KeyDerivationFunction::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}
