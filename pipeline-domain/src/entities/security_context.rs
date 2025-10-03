// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Security Context Entity
//!
//! The `SecurityContext` entity manages security-related information and access
//! control for pipeline processing operations. It enforces security policies,
//! tracks permissions, and maintains audit trails for secure data processing.
//!
//! ## Overview
//!
//! The security context provides:
//!
//! - **Access Control**: Permission-based authorization for operations
//! - **Security Levels**: Hierarchical classification of data sensitivity
//! - **Audit Tracking**: Session management and operation logging
//! - **Key Management**: Integration with encryption key infrastructure
//! - **Policy Enforcement**: Validation of security requirements
//!
//! ## Security Model
//!
//! The system implements a multi-layered security model:
//!
//! ### Permission-Based Access Control
//! Fine-grained permissions control specific operations:
//! - Read, Write, Execute for basic file operations
//! - Encrypt, Decrypt for cryptographic operations
//! - Compress, Decompress for data transformation
//! - Admin for administrative functions
//! - Custom permissions for extensibility
//!
//! ### Hierarchical Security Levels
//! Data classification from Public to TopSecret:
//! - **Public**: No restrictions, publicly accessible
//! - **Internal**: Internal use only, basic protection
//! - **Medium**: Moderate sensitivity, controlled access
//! - **Confidential**: High sensitivity, restricted access
//! - **Secret**: Very high sensitivity, need-to-know basis
//! - **TopSecret**: Maximum sensitivity, highest protection
//!
//! ### Session Management
//! Each context maintains a unique session for audit trails and tracking.

use crate::services::datetime_serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Security context entity for managing access control and security policies.
///
/// The `SecurityContext` encapsulates all security-related information needed
/// for pipeline processing operations. It enforces access control policies,
/// maintains audit information, and integrates with encryption key management.
///
/// ## Security Features
///
/// - **Permission Management**: Fine-grained access control
/// - **Security Classification**: Hierarchical data sensitivity levels
/// - **Session Tracking**: Unique session identification for audit trails
/// - **Key Integration**: Encryption key management and association
/// - **Audit Support**: Comprehensive logging and tracking capabilities
/// - **Policy Validation**: Enforcement of security requirements and
///   constraints
///
/// ## Usage Examples
///
/// ### Creating a Basic Security Context
///
///
/// ### Creating Context with Specific Permissions
///
///
/// ### Managing Permissions Dynamically
///
///
/// ### Security Level Validation
///
///
/// ### Encryption Key Management
///
///
/// ### Creating Restricted Contexts
///
///
/// ### Audit and Session Management
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    user_id: Option<String>,
    session_id: Uuid,
    permissions: Vec<Permission>,
    encryption_key_id: Option<String>,
    integrity_required: bool,
    audit_enabled: bool,
    security_level: SecurityLevel,
    metadata: HashMap<String, String>,
    #[serde(with = "datetime_serde")]
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Security permission enumeration for fine-grained access control.
///
/// Permissions define specific operations that can be performed within the
/// pipeline. The permission system supports both standard operations and custom
/// extensions for specialized use cases.
///
/// ## Standard Permissions
///
/// ### File Operations
/// - **Read**: Access to read files and data
/// - **Write**: Ability to create and modify files
/// - **Execute**: Permission to run processing operations
///
/// ### Cryptographic Operations
/// - **Encrypt**: Ability to encrypt data
/// - **Decrypt**: Ability to decrypt data
///
/// ### Data Transformation
/// - **Compress**: Permission to compress data
/// - **Decompress**: Permission to decompress data
///
/// ### Administrative
/// - **Admin**: Full administrative access (implies all other permissions)
///
/// ### Extensibility
/// - **Custom(String)**: Application-specific custom permissions
///
/// ## Permission Hierarchy
///
/// The Admin permission automatically grants all other permissions.
/// Custom permissions are evaluated independently.
///
/// ## Usage Examples
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Encrypt,
    Decrypt,
    Compress,
    Decompress,
    Custom(String),
}

/// Hierarchical security classification levels for data and operations.
///
/// Security levels provide a standardized way to classify the sensitivity of
/// data and determine appropriate protection measures. Higher levels require
/// stronger security controls and more restrictive access policies.
///
/// ## Security Level Hierarchy
///
/// Levels are ordered from lowest to highest sensitivity:
///
/// ### Public
/// - **Sensitivity**: None
/// - **Access**: Unrestricted public access
/// - **Protection**: Minimal security controls
/// - **Use Cases**: Public documentation, marketing materials
///
/// ### Internal
/// - **Sensitivity**: Low
/// - **Access**: Internal organization members
/// - **Protection**: Basic access controls
/// - **Use Cases**: Internal communications, general business data
///
/// ### Medium
/// - **Sensitivity**: Moderate
/// - **Access**: Authorized personnel only
/// - **Protection**: Standard security measures
/// - **Use Cases**: Business plans, financial reports
///
/// ### Confidential
/// - **Sensitivity**: High
/// - **Access**: Need-to-know basis
/// - **Protection**: Strong encryption and access controls
/// - **Use Cases**: Customer data, proprietary information
///
/// ### Secret
/// - **Sensitivity**: Very High
/// - **Access**: Strictly controlled
/// - **Protection**: Advanced security measures
/// - **Use Cases**: Trade secrets, sensitive personal data
///
/// ### TopSecret
/// - **Sensitivity**: Maximum
/// - **Access**: Highest clearance only
/// - **Protection**: Maximum security controls
/// - **Use Cases**: Critical infrastructure, national security data
///
/// ## Level Comparison
///
/// Security levels support comparison operations for policy enforcement:
///
///
/// ## Usage Examples
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Medium,
    Confidential,
    Secret,
    TopSecret,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: Uuid::new_v4(),
            permissions: vec![Permission::Read],
            encryption_key_id: None,
            integrity_required: false,
            audit_enabled: true,
            security_level: SecurityLevel::Internal,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }
}

impl SecurityContext {
    /// Creates a new security context
    pub fn new(user_id: Option<String>, security_level: SecurityLevel) -> Self {
        Self {
            user_id,
            security_level,
            ..Default::default()
        }
    }

    /// Creates a security context with permissions
    pub fn with_permissions(
        user_id: Option<String>,
        permissions: Vec<Permission>,
        security_level: SecurityLevel,
    ) -> Self {
        Self {
            user_id,
            permissions,
            security_level,
            ..Default::default()
        }
    }

    /// Gets the user ID
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    /// Gets the session ID
    pub fn session_id(&self) -> Uuid {
        self.session_id
    }

    /// Gets the permissions
    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Gets the encryption key ID
    pub fn encryption_key_id(&self) -> Option<&str> {
        self.encryption_key_id.as_deref()
    }

    /// Checks if integrity is required
    pub fn integrity_required(&self) -> bool {
        self.integrity_required
    }

    /// Checks if audit is enabled
    pub fn audit_enabled(&self) -> bool {
        self.audit_enabled
    }

    /// Gets the security level
    pub fn security_level(&self) -> &SecurityLevel {
        &self.security_level
    }

    /// Gets the metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Gets the creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    /// Sets the user ID
    pub fn set_user_id(&mut self, user_id: Option<String>) {
        self.user_id = user_id;
    }

    /// Adds a permission
    pub fn add_permission(&mut self, permission: Permission) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Removes a permission
    pub fn remove_permission(&mut self, permission: &Permission) {
        self.permissions.retain(|p| p != permission);
    }

    /// Sets the encryption key ID
    pub fn set_encryption_key_id(&mut self, key_id: Option<String>) {
        self.encryption_key_id = key_id;
    }

    /// Sets integrity requirement
    pub fn set_integrity_required(&mut self, required: bool) {
        self.integrity_required = required;
    }

    /// Sets audit enablement
    pub fn set_audit_enabled(&mut self, enabled: bool) {
        self.audit_enabled = enabled;
    }

    /// Sets the security level
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;
    }

    /// Adds metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Removes metadata
    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    /// Checks if the context has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission) || self.permissions.contains(&Permission::Admin)
    }

    /// Checks if the context can perform encryption
    pub fn can_encrypt(&self) -> bool {
        self.has_permission(&Permission::Encrypt) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can perform decryption
    pub fn can_decrypt(&self) -> bool {
        self.has_permission(&Permission::Decrypt) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can perform compression
    pub fn can_compress(&self) -> bool {
        self.has_permission(&Permission::Compress) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can perform decompression
    pub fn can_decompress(&self) -> bool {
        self.has_permission(&Permission::Decompress) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can read
    pub fn can_read(&self) -> bool {
        self.has_permission(&Permission::Read) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can write
    pub fn can_write(&self) -> bool {
        self.has_permission(&Permission::Write) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the context can execute
    pub fn can_execute(&self) -> bool {
        self.has_permission(&Permission::Execute) || self.has_permission(&Permission::Admin)
    }

    /// Checks if the security level meets the minimum requirement
    pub fn meets_security_level(&self, minimum_level: &SecurityLevel) -> bool {
        self.security_level >= *minimum_level
    }

    /// Creates a restricted copy of the security context
    pub fn restrict(&self, allowed_permissions: Vec<Permission>) -> Self {
        let restricted_permissions = self
            .permissions
            .iter()
            .filter(|p| allowed_permissions.contains(p))
            .cloned()
            .collect();

        Self {
            user_id: self.user_id.clone(),
            session_id: self.session_id,
            permissions: restricted_permissions,
            encryption_key_id: self.encryption_key_id.clone(),
            integrity_required: self.integrity_required,
            audit_enabled: self.audit_enabled,
            security_level: self.security_level.clone(),
            metadata: self.metadata.clone(),
            created_at: self.created_at,
        }
    }

    /// Validates the security context
    pub fn validate(&self) -> Result<(), crate::PipelineError> {
        if self.permissions.is_empty() {
            return Err(crate::PipelineError::SecurityViolation(
                "Security context must have at least one permission".to_string(),
            ));
        }

        if self.integrity_required && self.encryption_key_id.is_none() {
            return Err(crate::PipelineError::SecurityViolation(
                "Integrity required but no encryption key specified".to_string(),
            ));
        }

        Ok(())
    }
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityLevel::Public => write!(f, "Public"),
            SecurityLevel::Internal => write!(f, "Internal"),
            SecurityLevel::Medium => write!(f, "Medium"),
            SecurityLevel::Confidential => write!(f, "Confidential"),
            SecurityLevel::Secret => write!(f, "Secret"),
            SecurityLevel::TopSecret => write!(f, "Top Secret"),
        }
    }
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Read => write!(f, "Read"),
            Permission::Write => write!(f, "Write"),
            Permission::Execute => write!(f, "Execute"),
            Permission::Admin => write!(f, "Admin"),
            Permission::Encrypt => write!(f, "Encrypt"),
            Permission::Decrypt => write!(f, "Decrypt"),
            Permission::Compress => write!(f, "Compress"),
            Permission::Decompress => write!(f, "Decompress"),
            Permission::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}
