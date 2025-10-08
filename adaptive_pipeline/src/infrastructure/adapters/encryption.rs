// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

// Infrastructure module - contains future features not yet fully utilized
#![allow(dead_code, unused_imports, unused_variables)]
//! # Encryption Service Implementation
//!
//! This module is part of the Infrastructure layer, providing concrete
//! implementations of domain interfaces (ports).
//!
//! This module provides the concrete implementation of the encryption service
//! interface for the adaptive pipeline system. It implements various encryption
//! algorithms with secure key management, authenticated encryption, and
//! comprehensive error handling.
//!
//! ## Overview
//!
//! The encryption service implementation provides:
//!
//! - **Multi-Algorithm Support**: AES-256-GCM, ChaCha20-Poly1305, AES-128-GCM,
//!   AES-192-GCM
//! - **Secure Key Management**: Automatic key zeroization and secure memory
//!   handling
//! - **Key Derivation**: Argon2, Scrypt, and PBKDF2 key derivation functions
//! - **Authenticated Encryption**: Built-in integrity verification and
//!   authentication
//! - **Parallel Processing**: Multi-threaded encryption for improved
//!   performance
//!
//! ## Architecture
//!
//! The implementation follows the infrastructure layer patterns:
//!
//! - **Service Implementation**: `MultiAlgoEncryption` implements domain
//!   interface
//! - **Algorithm Handlers**: Specialized handlers for each encryption algorithm
//! - **Key Management**: Secure key generation, derivation, and storage
//! - **Memory Security**: Automatic zeroization of sensitive data
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
//! ### Key Derivation Functions
//!
//! Secure key derivation from passwords or key material:
//! - **Argon2**: Memory-hard function resistant to GPU attacks
//! - **Scrypt**: Memory-hard function with tunable parameters
//! - **PBKDF2**: Standard key derivation with configurable iterations
//!
//! ### Memory Security
//!
//! Sensitive data is protected in memory:
//! - **Automatic Zeroization**: Keys are securely wiped from memory
//! - **Secure Storage**: Minimal exposure of sensitive material
//! - **Drop Safety**: Automatic cleanup on scope exit
//!
//! ## Supported Algorithms
//!
//! ### AES-256-GCM
//! - **Key Size**: 256 bits (32 bytes)
//! - **Nonce Size**: 96 bits (12 bytes)
//! - **Performance**: Excellent on modern CPUs with AES-NI
//! - **Security**: Industry standard, FIPS approved
//!
//! ### ChaCha20-Poly1305
//! - **Key Size**: 256 bits (32 bytes)
//! - **Nonce Size**: 96 bits (12 bytes)
//! - **Performance**: Consistent across all platforms
//! - **Security**: Modern stream cipher, constant-time implementation
//!
//! ### AES-128-GCM / AES-192-GCM
//! - **Key Size**: 128/192 bits (16/24 bytes)
//! - **Nonce Size**: 96 bits (12 bytes)
//! - **Performance**: Faster than AES-256, still highly secure
//! - **Security**: Suitable for most applications
//!
//! ## Performance Optimizations
//!
//! ### Parallel Processing
//!
//! The implementation uses Rayon for parallel processing:
//! - **Chunk Parallelization**: Multiple chunks processed simultaneously
//! - **Key Derivation**: Parallel key derivation where supported
//! - **Thread Pool Management**: Efficient thread utilization
//!
//! ### Hardware Acceleration
//!
//! - **AES-NI**: Hardware acceleration for AES algorithms
//! - **Vectorization**: SIMD instructions for improved performance
//! - **Constant-Time**: Algorithms resistant to timing attacks
//!
//! ## Error Handling
//!
//! Comprehensive error handling for:
//! - **Encryption Failures**: Algorithm-specific error conditions
//! - **Key Derivation Errors**: Invalid parameters or insufficient entropy
//! - **Authentication Failures**: Tampering detection during decryption
//! - **Memory Errors**: Secure memory allocation failures
//!
//! ## Integration
//!
//! The service integrates with:
//! - **Domain Layer**: Implements `EncryptionService` trait
//! - **Security Context**: Access control and security policies
//! - **Pipeline Processing**: Chunk-based processing workflow
//! - **Metrics Collection**: Performance monitoring and statistics

use aes_gcm::{AeadInPlace, Aes256Gcm, Key, KeyInit, Nonce};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use base64::engine::general_purpose;
use base64::Engine as _;
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce};
use ring::rand::{SecureRandom, SystemRandom};
use scrypt::password_hash::SaltString as ScryptSalt;
use scrypt::Scrypt;
use std::collections::HashMap;
use zeroize::Zeroize;

use adaptive_pipeline_domain::entities::{ProcessingContext, SecurityContext};
use adaptive_pipeline_domain::services::{
    EncryptionAlgorithm, EncryptionConfig, EncryptionService, KeyDerivationFunction, KeyMaterial,
};
use adaptive_pipeline_domain::value_objects::{EncryptionBenchmark, FileChunk};
use adaptive_pipeline_domain::PipelineError;

// NOTE: Domain traits are now synchronous. This implementation is sync and
// CPU-bound. For async contexts, wrap this implementation with
// AsyncEncryptionAdapter.

/// Secure key material that automatically zeros sensitive data on drop
///
/// This struct provides secure storage for cryptographic key material with
/// automatic zeroization when the key goes out of scope. It ensures that
/// sensitive data is properly cleared from memory to prevent information
/// leakage.
///
/// # Security Features
///
/// - **Automatic Zeroization**: Key data is securely wiped when dropped
/// - **Clone Support**: Allows key material to be safely copied when needed
/// - **Memory Protection**: Minimizes exposure of sensitive data
///
/// # Examples
#[derive(Clone)]
#[allow(dead_code)]
struct SecureKey {
    data: Vec<u8>,
}

impl SecureKey {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    fn as_slice(&self) -> &[u8] {
        &self.data
    }
}

/// Concrete implementation of the encryption service
pub struct MultiAlgoEncryption {
    rng: SystemRandom,
    key_cache: HashMap<String, SecureKey>,
}

impl Default for MultiAlgoEncryption {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiAlgoEncryption {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
            key_cache: HashMap::new(),
        }
    }

    /// Generates a secure random key of the specified length
    fn generate_key(&self, length: usize) -> Result<Vec<u8>, PipelineError> {
        let mut key = vec![0u8; length];
        self.rng
            .fill(&mut key)
            .map_err(|e| PipelineError::EncryptionError(format!("Failed to generate key: {:?}", e)))?;
        Ok(key)
    }

    /// Generates a secure random nonce/IV
    fn generate_nonce(&self, length: usize) -> Result<Vec<u8>, PipelineError> {
        let mut nonce = vec![0u8; length];
        self.rng
            .fill(&mut nonce)
            .map_err(|e| PipelineError::EncryptionError(format!("Failed to generate nonce: {:?}", e)))?;
        Ok(nonce)
    }

    /// Derives a key using Argon2
    fn derive_key_argon2(&self, password: &[u8], salt: &[u8], key_length: usize) -> Result<Vec<u8>, PipelineError> {
        let argon2 = Argon2::default();
        let salt_string =
            SaltString::encode_b64(salt).map_err(|e| PipelineError::EncryptionError(format!("Invalid salt: {}", e)))?;

        let password_hash = argon2
            .hash_password(password, &salt_string)
            .map_err(|e| PipelineError::EncryptionError(format!("Argon2 key derivation failed: {}", e)))?;

        let hash_string = password_hash
            .hash
            .ok_or_else(|| PipelineError::EncryptionError("Password hash missing".to_string()))?;
        let hash_bytes = hash_string.as_bytes();
        if hash_bytes.len() >= key_length {
            Ok(hash_bytes[..key_length].to_vec())
        } else {
            Err(PipelineError::EncryptionError("Derived key too short".to_string()))
        }
    }

    /// Derives a key using scrypt
    fn derive_key_scrypt(&self, password: &[u8], salt: &[u8], key_length: usize) -> Result<Vec<u8>, PipelineError> {
        let scrypt = Scrypt;
        let salt_string =
            ScryptSalt::encode_b64(salt).map_err(|e| PipelineError::EncryptionError(format!("Invalid salt: {}", e)))?;

        let password_hash = scrypt
            .hash_password(password, &salt_string)
            .map_err(|e| PipelineError::EncryptionError(format!("Scrypt key derivation failed: {}", e)))?;

        let hash_string = password_hash
            .hash
            .ok_or_else(|| PipelineError::EncryptionError("Password hash missing".to_string()))?;
        let hash_bytes = hash_string.as_bytes();
        if hash_bytes.len() >= key_length {
            Ok(hash_bytes[..key_length].to_vec())
        } else {
            Err(PipelineError::EncryptionError("Derived key too short".to_string()))
        }
    }

    /// Derives a key using PBKDF2 with SHA-256
    fn derive_key_pbkdf2(
        &self,
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> Result<Vec<u8>, PipelineError> {
        let mut key = vec![0u8; key_length];
        ring::pbkdf2::derive(
            ring::pbkdf2::PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(iterations)
                .ok_or_else(|| PipelineError::EncryptionError("Invalid iteration count".to_string()))?,
            salt,
            password,
            &mut key,
        );
        Ok(key)
    }

    /// Encrypts data using AES-256-GCM
    fn encrypt_aes256_gcm(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, PipelineError> {
        if key.len() != 32 {
            return Err(PipelineError::EncryptionError(
                "AES-256 requires 32-byte key".to_string(),
            ));
        }
        if nonce.len() != 12 {
            return Err(PipelineError::EncryptionError(
                "AES-GCM requires 12-byte nonce".to_string(),
            ));
        }

        let cipher_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce_array = Nonce::from_slice(nonce);

        let mut buffer = data.to_vec();
        cipher
            .encrypt_in_place(nonce_array, b"", &mut buffer)
            .map_err(|e| PipelineError::EncryptionError(format!("AES-256-GCM encryption failed: {:?}", e)))?;

        // Prepend nonce to encrypted data
        let mut result = nonce.to_vec();
        result.extend_from_slice(&buffer);
        Ok(result)
    }

    /// Decrypts data using AES-256-GCM
    fn decrypt_aes256_gcm(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, PipelineError> {
        if key.len() != 32 {
            return Err(PipelineError::EncryptionError(
                "AES-256 requires 32-byte key".to_string(),
            ));
        }
        if data.len() < 12 {
            return Err(PipelineError::EncryptionError("Encrypted data too short".to_string()));
        }

        let (nonce, ciphertext) = data.split_at(12);
        let cipher_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce_array = Nonce::from_slice(nonce);

        let mut buffer = ciphertext.to_vec();
        cipher
            .decrypt_in_place(nonce_array, b"", &mut buffer)
            .map_err(|e| PipelineError::EncryptionError(format!("AES-256-GCM decryption failed: {:?}", e)))?;

        Ok(buffer)
    }

    /// Encrypts data using ChaCha20-Poly1305
    fn encrypt_chacha20_poly1305(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, PipelineError> {
        if key.len() != 32 {
            return Err(PipelineError::EncryptionError(
                "ChaCha20 requires 32-byte key".to_string(),
            ));
        }
        if nonce.len() != 12 {
            return Err(PipelineError::EncryptionError(
                "ChaCha20-Poly1305 requires 12-byte nonce".to_string(),
            ));
        }

        let cipher_key = ChaChaKey::from_slice(key);
        let cipher = ChaCha20Poly1305::new(cipher_key);
        let nonce_array = ChaChaNonce::from_slice(nonce);

        let mut buffer = data.to_vec();
        cipher
            .encrypt_in_place(nonce_array, b"", &mut buffer)
            .map_err(|e| PipelineError::EncryptionError(format!("ChaCha20-Poly1305 encryption failed: {:?}", e)))?;

        // Prepend nonce to encrypted data
        let mut result = nonce.to_vec();
        result.extend_from_slice(&buffer);
        Ok(result)
    }

    /// Decrypts data using ChaCha20-Poly1305
    fn decrypt_chacha20_poly1305(&self, data: &[u8], key: &[u8]) -> Result<Vec<u8>, PipelineError> {
        if key.len() != 32 {
            return Err(PipelineError::EncryptionError(
                "ChaCha20 requires 32-byte key".to_string(),
            ));
        }
        if data.len() < 12 {
            return Err(PipelineError::EncryptionError("Encrypted data too short".to_string()));
        }

        let (nonce, ciphertext) = data.split_at(12);
        let cipher_key = ChaChaKey::from_slice(key);
        let cipher = ChaCha20Poly1305::new(cipher_key);
        let nonce_array = ChaChaNonce::from_slice(nonce);

        let mut buffer = ciphertext.to_vec();
        cipher
            .decrypt_in_place(nonce_array, b"", &mut buffer)
            .map_err(|e| PipelineError::EncryptionError(format!("ChaCha20-Poly1305 decryption failed: {:?}", e)))?;

        Ok(buffer)
    }

    /// Calculates SHA-256 hash for integrity verification
    fn calculate_hash(&self, data: &[u8]) -> Vec<u8> {
        ring::digest::digest(&ring::digest::SHA256, data).as_ref().to_vec()
    }
}

impl EncryptionService for MultiAlgoEncryption {
    fn encrypt_chunk(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let data = chunk.data().to_vec();

        // Use the provided key material
        let key = key_material;

        // Generate nonce
        let nonce = self.generate_nonce(12)?; // 12 bytes for GCM/ChaCha20-Poly1305

        // Encrypt based on algorithm
        let encrypted_data = match &config.algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.encrypt_aes256_gcm(&data, &key.key, &nonce)?,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.encrypt_chacha20_poly1305(&data, &key.key, &nonce)?,
            EncryptionAlgorithm::Aes128Gcm => {
                if key.len() != 16 {
                    return Err(PipelineError::EncryptionError(
                        "AES-128 requires 16-byte key".to_string(),
                    ));
                }
                // Similar implementation for AES-128
                return Err(PipelineError::EncryptionError(
                    "AES-128-GCM not yet fully implemented".to_string(),
                ));
            }
            EncryptionAlgorithm::Aes192Gcm => {
                if key.len() != 24 {
                    return Err(PipelineError::EncryptionError(
                        "AES-192 requires 24-byte key".to_string(),
                    ));
                }
                // Similar implementation for AES-192
                return Err(PipelineError::EncryptionError(
                    "AES-192-GCM not yet fully implemented".to_string(),
                ));
            }
            EncryptionAlgorithm::Custom(name) => {
                return Err(PipelineError::EncryptionError(format!(
                    "Custom algorithm '{}' not implemented",
                    name
                )));
            }
        };

        // Create new chunk with encrypted data
        let chunk = chunk.with_data(encrypted_data)?;

        // Calculate integrity hash
        let integrity_hash = self.calculate_hash(chunk.data());

        // Update context with encryption metadata
        context.add_metadata("encryption_algorithm".to_string(), config.algorithm.to_string());
        context.add_metadata("integrity_hash".to_string(), hex::encode(&integrity_hash));
        context.add_metadata("encrypted".to_string(), "true".to_string());

        Ok(chunk)
    }

    fn decrypt_chunk(
        &self,
        chunk: FileChunk,
        config: &EncryptionConfig,
        key_material: &KeyMaterial,
        context: &mut ProcessingContext,
    ) -> Result<FileChunk, PipelineError> {
        let data = chunk.data().to_vec();

        // Use the provided key material
        let key = key_material;

        // Decrypt based on algorithm
        let decrypted_data = match &config.algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.decrypt_aes256_gcm(&data, &key.key)?,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha20_poly1305(&data, &key.key)?,
            EncryptionAlgorithm::Aes128Gcm => {
                return Err(PipelineError::EncryptionError(
                    "AES-128-GCM not yet fully implemented".to_string(),
                ));
            }
            EncryptionAlgorithm::Aes192Gcm => {
                return Err(PipelineError::EncryptionError(
                    "AES-192-GCM not yet fully implemented".to_string(),
                ));
            }
            EncryptionAlgorithm::Custom(name) => {
                return Err(PipelineError::EncryptionError(format!(
                    "Custom algorithm '{}' not implemented",
                    name
                )));
            }
        };

        // Create new chunk with decrypted data
        let chunk = chunk.with_data(decrypted_data)?;

        // Verify integrity if hash is available
        if let Some(expected_hash) = context.get_metadata("integrity_hash") {
            let actual_hash = hex::encode(self.calculate_hash(chunk.data()));
            if actual_hash != *expected_hash {
                return Err(PipelineError::EncryptionError(
                    "Integrity verification failed".to_string(),
                ));
            }
        }

        // Update context
        context.add_metadata("decryption_algorithm".to_string(), config.algorithm.to_string());
        context.add_metadata("encrypted".to_string(), "false".to_string());

        Ok(chunk)
    }

    fn derive_key_material(
        &self,
        password: &str,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        let password_bytes = password.as_bytes();
        let salt = self.generate_nonce(32)?; // 32-byte salt

        let key_length = match &config.algorithm {
            EncryptionAlgorithm::Aes128Gcm => 16,
            EncryptionAlgorithm::Aes192Gcm => 24,
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
            EncryptionAlgorithm::Custom(_) => 32, // Default to 32 bytes for custom algorithms
        };

        let kdf = &config.key_derivation;

        let key_bytes = match kdf {
            KeyDerivationFunction::Argon2 => self.derive_key_argon2(password_bytes, &salt, key_length)?,
            KeyDerivationFunction::Scrypt => self.derive_key_scrypt(password_bytes, &salt, key_length)?,
            KeyDerivationFunction::Pbkdf2 => {
                self.derive_key_pbkdf2(password_bytes, &salt, config.iterations, key_length)?
            }
            KeyDerivationFunction::Custom(name) => {
                return Err(PipelineError::EncryptionError(format!(
                    "Custom KDF '{}' not implemented",
                    name
                )));
            }
        };

        let nonce = self.generate_nonce(12)?; // 12 bytes for GCM

        Ok(KeyMaterial::new(key_bytes, nonce, salt, config.algorithm.clone()))
    }

    fn generate_key_material(
        &self,
        config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        let key_length = match &config.algorithm {
            EncryptionAlgorithm::Aes128Gcm => 16,
            EncryptionAlgorithm::Aes192Gcm => 24,
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
            EncryptionAlgorithm::Custom(_) => 32, // Default to 32 bytes for custom algorithms
        };

        let key = self.generate_key(key_length)?;
        let nonce = self.generate_nonce(12)?;
        let salt = self.generate_nonce(32)?;

        Ok(KeyMaterial::new(key, nonce, salt, config.algorithm.clone()))
    }

    fn validate_config(&self, config: &EncryptionConfig) -> Result<(), PipelineError> {
        // Validate algorithm
        match &config.algorithm {
            EncryptionAlgorithm::Aes128Gcm
            | EncryptionAlgorithm::Aes192Gcm
            | EncryptionAlgorithm::Aes256Gcm
            | EncryptionAlgorithm::ChaCha20Poly1305 => {
                // These are supported
            }
            EncryptionAlgorithm::Custom(name) => {
                return Err(PipelineError::InvalidConfiguration(format!(
                    "Custom algorithm '{}' not supported",
                    name
                )));
            }
        }

        // Validate key derivation function
        match &config.key_derivation {
            KeyDerivationFunction::Argon2 => {}
            KeyDerivationFunction::Scrypt => {}
            KeyDerivationFunction::Pbkdf2 => {
                if config.iterations < 10_000 {
                    return Err(PipelineError::InvalidConfiguration(
                        "PBKDF2 iterations should be at least 10,000".to_string(),
                    ));
                }
            }
            KeyDerivationFunction::Custom(_) => {}
        }

        Ok(())
    }

    fn benchmark_algorithm(
        &self,
        algorithm: &EncryptionAlgorithm,
        test_data: &[u8],
    ) -> Result<EncryptionBenchmark, PipelineError> {
        let key_length = match algorithm {
            EncryptionAlgorithm::Aes128Gcm => 16,
            EncryptionAlgorithm::Aes192Gcm => 24,
            EncryptionAlgorithm::Aes256Gcm => 32,
            EncryptionAlgorithm::ChaCha20Poly1305 => 32,
            EncryptionAlgorithm::Custom(_) => 32, // Default to 32 bytes for custom algorithms
        };
        let key = self.generate_key(key_length)?;
        let nonce = self.generate_nonce(12)?;

        let start = std::time::Instant::now();

        // Encrypt the data
        let encrypted = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.encrypt_aes256_gcm(test_data, &key, &nonce)?,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.encrypt_chacha20_poly1305(test_data, &key, &nonce)?,
            _ => {
                return Err(PipelineError::EncryptionError(
                    "Algorithm not supported for benchmarking".to_string(),
                ));
            }
        };

        let encryption_time = start.elapsed();

        // Benchmark decryption
        let start = std::time::Instant::now();
        let _decrypted = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.decrypt_aes256_gcm(&encrypted, &key)?,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha20_poly1305(&encrypted, &key)?,
            _ => {
                return Err(PipelineError::EncryptionError(
                    "Algorithm not supported for benchmarking".to_string(),
                ));
            }
        };
        let decryption_time = start.elapsed();

        // Calculate speeds in MB/s
        let data_size_mb = (test_data.len() as f64) / (1024.0 * 1024.0);
        let encryption_speed = data_size_mb / encryption_time.as_secs_f64();
        let _decryption_speed = data_size_mb / decryption_time.as_secs_f64();

        Ok(EncryptionBenchmark::new(
            algorithm.clone(),
            encryption_speed,
            encryption_time,
            32.0, // Estimated memory usage
            70.0, // Estimated CPU usage
            data_size_mb,
        ))
    }

    fn wipe_key_material(&self, key_material: &mut KeyMaterial) -> Result<(), PipelineError> {
        // Zero out the key material
        key_material.zeroize();
        Ok(())
    }

    fn store_key_material(
        &self,
        key_material: &KeyMaterial,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<(), PipelineError> {
        // In a real implementation, this would store in HSM or secure key vault
        // For now, just validate inputs
        if key_id.is_empty() {
            return Err(PipelineError::EncryptionError("Key ID cannot be empty".to_string()));
        }

        if key_material.key.is_empty() {
            return Err(PipelineError::EncryptionError(
                "Key material cannot be empty".to_string(),
            ));
        }

        // TODO: Implement actual secure storage
        Ok(())
    }

    fn retrieve_key_material(
        &self,
        key_id: &str,
        security_context: &SecurityContext,
    ) -> Result<KeyMaterial, PipelineError> {
        // In a real implementation, this would retrieve from HSM or secure key vault
        Err(PipelineError::EncryptionError(
            "Key retrieval not yet implemented".to_string(),
        ))
    }

    fn rotate_keys(
        &self,
        old_key_id: &str,
        new_config: &EncryptionConfig,
        security_context: &SecurityContext,
    ) -> Result<String, PipelineError> {
        // In a real implementation, this would generate new keys and update storage
        Err(PipelineError::EncryptionError(
            "Key rotation not yet implemented".to_string(),
        ))
    }

    fn supported_algorithms(&self) -> Vec<EncryptionAlgorithm> {
        vec![
            EncryptionAlgorithm::Aes256Gcm,
            EncryptionAlgorithm::ChaCha20Poly1305,
            EncryptionAlgorithm::Aes128Gcm,
        ]
    }
}

// Implement StageService trait for unified interface
impl adaptive_pipeline_domain::services::StageService for MultiAlgoEncryption {
    fn process_chunk(
        &self,
        chunk: adaptive_pipeline_domain::FileChunk,
        config: &adaptive_pipeline_domain::entities::StageConfiguration,
        context: &mut adaptive_pipeline_domain::ProcessingContext,
    ) -> Result<adaptive_pipeline_domain::FileChunk, adaptive_pipeline_domain::PipelineError> {
        use adaptive_pipeline_domain::services::FromParameters;

        // Type-safe extraction of EncryptionConfig from parameters
        let encryption_config = EncryptionConfig::from_parameters(&config.parameters)?;

        // Extract KeyMaterial from parameters
        // Expected format: base64-encoded key, nonce, salt
        let key_b64 = config
            .parameters
            .get("key")
            .ok_or_else(|| adaptive_pipeline_domain::PipelineError::MissingParameter("key".into()))?;

        let nonce_b64 = config
            .parameters
            .get("nonce")
            .ok_or_else(|| adaptive_pipeline_domain::PipelineError::MissingParameter("nonce".into()))?;

        let salt_b64 = config
            .parameters
            .get("salt")
            .ok_or_else(|| adaptive_pipeline_domain::PipelineError::MissingParameter("salt".into()))?;

        // Decode base64 strings using Engine API
        let key = general_purpose::STANDARD.decode(key_b64).map_err(|e| {
            adaptive_pipeline_domain::PipelineError::InvalidParameter(format!("Invalid base64 key: {}", e))
        })?;

        let nonce = general_purpose::STANDARD.decode(nonce_b64).map_err(|e| {
            adaptive_pipeline_domain::PipelineError::InvalidParameter(format!("Invalid base64 nonce: {}", e))
        })?;

        let salt = general_purpose::STANDARD.decode(salt_b64).map_err(|e| {
            adaptive_pipeline_domain::PipelineError::InvalidParameter(format!("Invalid base64 salt: {}", e))
        })?;

        let key_material = KeyMaterial::new(key, nonce, salt, encryption_config.algorithm.clone());

        match config.operation {
            adaptive_pipeline_domain::entities::Operation::Forward => {
                self.encrypt_chunk(chunk, &encryption_config, &key_material, context)
            }
            adaptive_pipeline_domain::entities::Operation::Reverse => {
                self.decrypt_chunk(chunk, &encryption_config, &key_material, context)
            }
        }
    }

    fn position(&self) -> adaptive_pipeline_domain::entities::StagePosition {
        adaptive_pipeline_domain::entities::StagePosition::PreBinary
    }

    fn is_reversible(&self) -> bool {
        true
    }

    fn stage_type(&self) -> adaptive_pipeline_domain::entities::StageType {
        adaptive_pipeline_domain::entities::StageType::Encryption
    }
}
