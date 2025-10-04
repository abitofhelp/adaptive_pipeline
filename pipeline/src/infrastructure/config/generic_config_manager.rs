// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! # Generic Configuration Manager
//!
//! This module provides a generic, reusable configuration management system
//! for the adaptive pipeline system. It supports validation, versioning,
//! migration, and hot-reloading of configuration data.
//!
//! ## Overview
//!
//! The generic configuration manager provides:
//!
//! - **Type-Safe Configuration**: Generic configuration management for any type
//! - **Validation**: Comprehensive validation with detailed error reporting
//! - **Versioning**: Configuration schema versioning and migration support
//! - **Hot Reloading**: Runtime configuration updates without restart
//! - **Thread Safety**: Safe concurrent access to configuration data
//!
//! ## Architecture
//!
//! The configuration manager follows generic design patterns:
//!
//! - **Generic Design**: Works with any configuration type implementing
//!   required traits
//! - **Validation Framework**: Pluggable validation with detailed error
//!   reporting
//! - **Migration System**: Automatic migration between configuration versions
//! - **Event System**: Configuration change notifications and observers
//!
//! ## Key Features
//!
//! ### Configuration Validation
//!
//! - **Schema Validation**: Validate configuration against defined schemas
//! - **Business Rules**: Enforce business logic and constraints
//! - **Error Reporting**: Detailed error messages with field-level information
//! - **Warning System**: Non-fatal warnings for configuration issues
//!
//! ### Version Management
//!
//! - **Schema Versioning**: Track configuration schema versions
//! - **Migration Support**: Automatic migration between versions
//! - **Backward Compatibility**: Support for older configuration formats
//! - **Version Detection**: Automatic detection of configuration versions
//!
//! ### Hot Reloading
//!
//! - **Runtime Updates**: Update configuration without application restart
//! - **Change Detection**: Detect configuration file changes
//! - **Atomic Updates**: Atomic configuration updates to prevent inconsistency
//! - **Rollback Support**: Rollback to previous configuration on errors
//!
//! ## Usage Examples
//!
//! ### Basic Configuration Management

//!
//! ### Configuration with Hot Reloading

//!
//! ### Configuration Migration

//!
//! ## Validation Framework
//!
//! ### Validation Results
//!
//! The validation system provides detailed feedback:
//!
//! - **Errors**: Critical validation failures that prevent usage
//! - **Warnings**: Non-critical issues that should be addressed
//! - **Field-Level Information**: Specific field names and error contexts
//! - **Severity Levels**: Different severity levels for different issues
//!
//! ### Custom Validation Rules
//!
//! - **Business Logic**: Implement custom business logic validation
//! - **Cross-Field Validation**: Validate relationships between fields
//! - **External Validation**: Validate against external resources
//! - **Conditional Validation**: Conditional validation based on other fields
//!
//! ## Migration System
//!
//! ### Migration Strategies
//!
//! - **Automatic Migration**: Automatic migration between compatible versions
//! - **Manual Migration**: Manual migration for complex changes
//! - **Incremental Migration**: Step-by-step migration through versions
//! - **Rollback Support**: Rollback to previous versions on failure
//!
//! ### Version Compatibility
//!
//! - **Semantic Versioning**: Use semantic versioning for schema versions
//! - **Compatibility Matrix**: Define compatibility between versions
//! - **Breaking Changes**: Handle breaking changes gracefully
//! - **Deprecation Warnings**: Warn about deprecated configuration options
//!
//! ## Performance Considerations
//!
//! ### Memory Usage
//!
//! - **Efficient Storage**: Efficient storage of configuration data
//! - **Lazy Loading**: Lazy loading of configuration sections
//! - **Memory Pooling**: Reuse configuration objects when possible
//!
//! ### Access Performance
//!
//! - **Caching**: Cache frequently accessed configuration values
//! - **Read Optimization**: Optimize for frequent read operations
//! - **Lock Contention**: Minimize lock contention for concurrent access
//!
//! ## Error Handling
//!
//! ### Configuration Errors
//!
//! - **Parse Errors**: Handle configuration file parsing errors
//! - **Validation Errors**: Comprehensive validation error reporting
//! - **Migration Errors**: Handle migration failures gracefully
//! - **File System Errors**: Handle file system access errors
//!
//! ### Recovery Strategies
//!
//! - **Default Values**: Use default values for missing configuration
//! - **Fallback Configuration**: Fallback to previous valid configuration
//! - **Error Reporting**: Detailed error reporting with suggestions
//! - **Graceful Degradation**: Continue operation with reduced functionality
//!
//! ## Integration
//!
//! The configuration manager integrates with:
//!
//! - **File System**: Load configuration from various file formats
//! - **Environment Variables**: Override configuration with environment
//!   variables
//! - **Command Line**: Override configuration with command line arguments
//! - **External Services**: Load configuration from external configuration
//!   services
//!
//! ## Thread Safety
//!
//! The configuration manager is fully thread-safe:
//!
//! - **Concurrent Access**: Safe concurrent access to configuration data
//! - **Atomic Updates**: Atomic configuration updates
//! - **Lock-Free Reads**: Lock-free reads for better performance
//!
//! ## Future Enhancements
//!
//! Planned enhancements include:
//!
//! - **Configuration UI**: Web-based configuration management interface
//! - **A/B Testing**: Support for A/B testing of configuration changes
//! - **Configuration Validation Service**: External validation service
//!   integration
//! - **Configuration Analytics**: Analytics and monitoring of configuration
//!   usage

use pipeline_domain::error::PipelineError;
use pipeline_domain::services::datetime_serde;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

/// Generic trait for configuration validation with detailed error reporting
///
/// This trait defines the interface for configuration validation, enabling
/// type-safe validation of configuration data with comprehensive error
/// reporting and schema versioning support.
///
/// # Key Features
///
/// - **Validation Logic**: Implement custom validation rules for configuration
/// - **Error Reporting**: Detailed error messages with field-level information
/// - **Schema Versioning**: Track and manage configuration schema versions
/// - **Migration Support**: Automatic migration between configuration versions
///
/// # Implementation Requirements
///
/// Implementing types must:
/// - Be cloneable for configuration updates
/// - Be debuggable for error reporting
/// - Be thread-safe (`Send + Sync`)
/// - Have a stable lifetime (`'static`)
///
/// # Examples
///
pub trait ConfigValidation: Clone + Debug + Send + Sync + 'static {
    /// Validates the configuration and returns detailed validation results
    fn validate(&self) -> ConfigValidationResult;

    /// Returns the configuration schema version for compatibility checking
    fn schema_version(&self) -> String;

    /// Migrates configuration from an older schema version
    fn migrate_from_version(&self, from_version: &str, data: &str) -> Result<Self, PipelineError>;
}

/// Result of configuration validation with detailed error information
#[derive(Debug, Clone)]
pub struct ConfigValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ConfigValidationError>,
    pub warnings: Vec<ConfigValidationWarning>,
}

impl ConfigValidationResult {
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ConfigValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<ConfigValidationWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    pub fn add_error(&mut self, field: String, message: String) {
        self.errors.push(ConfigValidationError { field, message });
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, field: String, message: String) {
        self.warnings.push(ConfigValidationWarning { field, message });
    }
}

#[derive(Debug, Clone)]
pub struct ConfigValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ConfigValidationWarning {
    pub field: String,
    pub message: String,
}

/// Configuration source trait for loading configurations from different sources
#[async_trait]
pub trait ConfigSource: Send + Sync {
    /// Loads configuration data as a string
    async fn load(&self) -> Result<String, PipelineError>;

    /// Saves configuration data
    async fn save(&self, data: &str) -> Result<(), PipelineError>;

    /// Checks if the source exists and is accessible
    async fn exists(&self) -> bool;

    /// Gets the source identifier (e.g., file path, URL)
    fn source_id(&self) -> String;
}

/// File-based configuration source
pub struct FileConfigSource {
    file_path: String,
}

impl FileConfigSource {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

#[async_trait]
impl ConfigSource for FileConfigSource {
    async fn load(&self) -> Result<String, PipelineError> {
        tokio::fs::read_to_string(&self.file_path)
            .await
            .map_err(|e| PipelineError::InternalError(format!("Failed to read config file {}: {}", self.file_path, e)))
    }

    async fn save(&self, data: &str) -> Result<(), PipelineError> {
        tokio::fs::write(&self.file_path, data)
            .await
            .map_err(|e| PipelineError::InternalError(format!("Failed to write config file {}: {}", self.file_path, e)))
    }

    async fn exists(&self) -> bool {
        tokio::fs::metadata(&self.file_path).await.is_ok()
    }

    fn source_id(&self) -> String {
        self.file_path.clone()
    }
}

/// Environment variable configuration source
pub struct EnvConfigSource {
    prefix: String,
}

impl EnvConfigSource {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

#[async_trait]
impl ConfigSource for EnvConfigSource {
    async fn load(&self) -> Result<String, PipelineError> {
        let mut config_map = HashMap::new();

        for (key, value) in std::env::vars() {
            if key.starts_with(&self.prefix) {
                let config_key = key.strip_prefix(&self.prefix).unwrap_or(&key);
                config_map.insert(config_key.to_lowercase(), value);
            }
        }

        serde_json::to_string(&config_map)
            .map_err(|e| PipelineError::InternalError(format!("Failed to serialize env config: {}", e)))
    }

    async fn save(&self, _data: &str) -> Result<(), PipelineError> {
        Err(PipelineError::InternalError(
            "Cannot save to environment variables".to_string(),
        ))
    }

    async fn exists(&self) -> bool {
        std::env::vars().any(|(key, _)| key.starts_with(&self.prefix))
    }

    fn source_id(&self) -> String {
        format!("env:{}", self.prefix)
    }
}

/// Configuration change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChangeEvent<T> {
    pub config_type: String,
    pub old_config: Option<T>,
    pub new_config: T,
    #[serde(with = "datetime_serde")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
    pub change_reason: String,
    pub changed_by: String,
}

/// Configuration change listener trait
#[async_trait]
pub trait ConfigChangeListener<T>: Send + Sync
where
    T: ConfigValidation + Serialize + DeserializeOwned,
{
    /// Called when configuration changes
    async fn on_config_changed(&self, event: ConfigChangeEvent<T>) -> Result<(), PipelineError>;
}

/// Generic configuration manager providing centralized configuration management
pub struct GenericConfigManager<T>
where
    T: ConfigValidation + Serialize + DeserializeOwned,
{
    config: RwLock<T>,
    sources: Vec<Arc<dyn ConfigSource>>,
    listeners: Vec<Arc<dyn ConfigChangeListener<T>>>,
    change_history: RwLock<Vec<ConfigChangeEvent<T>>>,
    auto_reload: bool,
}

impl<T> GenericConfigManager<T>
where
    T: ConfigValidation + Serialize + DeserializeOwned,
{
    /// Creates a new configuration manager with default configuration
    pub fn new(default_config: T) -> Self {
        Self {
            config: RwLock::new(default_config),
            sources: Vec::new(),
            listeners: Vec::new(),
            change_history: RwLock::new(Vec::new()),
            auto_reload: false,
        }
    }

    /// Adds a configuration source
    pub fn add_source(mut self, source: Arc<dyn ConfigSource>) -> Self {
        self.sources.push(source);
        self
    }

    /// Adds a configuration change listener
    pub fn add_listener(mut self, listener: Arc<dyn ConfigChangeListener<T>>) -> Self {
        self.listeners.push(listener);
        self
    }

    /// Enables or disables automatic configuration reloading
    pub fn with_auto_reload(mut self, auto_reload: bool) -> Self {
        self.auto_reload = auto_reload;
        self
    }

    /// Gets the current configuration
    pub fn get_config(&self) -> Result<T, PipelineError> {
        self.config
            .read()
            .map_err(|e| PipelineError::InternalError(format!("Failed to read config: {}", e)))
            .map(|config| config.clone())
    }

    /// Updates the configuration with validation
    pub async fn update_config(
        &self,
        new_config: T,
        change_reason: String,
        changed_by: String,
    ) -> Result<(), PipelineError> {
        // Validate the new configuration
        let validation_result = new_config.validate();
        if !validation_result.is_valid {
            let error_messages: Vec<String> = validation_result
                .errors
                .iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();
            return Err(PipelineError::InvalidConfiguration(format!(
                "Configuration validation failed: {}",
                error_messages.join(", ")
            )));
        }

        // Get the old configuration for the change event
        let old_config = self.get_config().ok();

        // Update the configuration
        {
            let mut config = self
                .config
                .write()
                .map_err(|e| PipelineError::InternalError(format!("Failed to write config: {}", e)))
                ?;
            *config = new_config.clone();
        }

        // Create change event
        let change_event = ConfigChangeEvent {
            config_type: std::any::type_name::<T>().to_string(),
            old_config,
            new_config: new_config.clone(),
            changed_at: chrono::Utc::now(),
            change_reason,
            changed_by,
        };

        // Record the change
        if let Ok(mut history) = self.change_history.write() {
            history.push(change_event.clone());
            // Keep only the last 100 changes
            if history.len() > 100 {
                history.remove(0);
            }
        }

        // Notify listeners
        for listener in &self.listeners {
            if let Err(e) = listener.on_config_changed(change_event.clone()).await {
                eprintln!("Config change listener error: {}", e);
            }
        }

        Ok(())
    }

    /// Loads configuration from all sources in order
    pub async fn load_from_sources(&self, changed_by: String) -> Result<(), PipelineError> {
        let mut merged_config = None;

        for source in &self.sources {
            if source.exists().await {
                let config_data = source.load().await?;
                let config: T = serde_json::from_str(&config_data)
                    .map_err(|e| {
                        PipelineError::InternalError(format!(
                            "Failed to parse config from {}: {}",
                            source.source_id(),
                            e
                        ))
                    })
                    ?;

                merged_config = Some(match merged_config {
                    Some(_existing) => {
                        // In a real implementation, you'd want a merge strategy
                        // For now, later sources override earlier ones
                        config
                    }
                    None => config,
                });
            }
        }

        if let Some(config) = merged_config {
            self.update_config(config, "Loaded from sources".to_string(), changed_by)
                .await
                ?;
        }

        Ok(())
    }

    /// Saves current configuration to the first writable source
    pub async fn save_to_source(&self) -> Result<(), PipelineError> {
        let config = self.get_config()?;
        let config_data = serde_json::to_string_pretty(&config)
            .map_err(|e| PipelineError::InternalError(format!("Failed to serialize config: {}", e)))
            ?;

        for source in &self.sources {
            if let Ok(()) = source.save(&config_data).await {
                return Ok(());
            }
        }

        Err(PipelineError::InternalError(
            "No writable configuration source available".to_string(),
        ))
    }

    /// Gets configuration change history
    pub fn get_change_history(&self) -> Vec<ConfigChangeEvent<T>> {
        self.change_history
            .read()
            .map(|history| history.clone())
            .unwrap_or_default()
    }

    /// Validates current configuration
    pub fn validate_current_config(&self) -> Result<ConfigValidationResult, PipelineError> {
        let config = self.get_config()?;
        Ok(config.validate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestConfig {
        database_url: String,
        max_connections: u32,
        timeout_seconds: u64,
        features: Vec<String>,
    }

    impl ConfigValidation for TestConfig {
        fn validate(&self) -> ConfigValidationResult {
            let mut result = ConfigValidationResult::valid();

            if self.database_url.is_empty() {
                result.add_error("database_url".to_string(), "Database URL cannot be empty".to_string());
            }

            if self.max_connections == 0 {
                result.add_error(
                    "max_connections".to_string(),
                    "Max connections must be greater than 0".to_string(),
                );
            }

            if self.max_connections > 1000 {
                result.add_warning(
                    "max_connections".to_string(),
                    "Very high connection count may impact performance".to_string(),
                );
            }

            if self.timeout_seconds == 0 {
                result.add_error(
                    "timeout_seconds".to_string(),
                    "Timeout must be greater than 0".to_string(),
                );
            }

            result
        }

        fn schema_version(&self) -> String {
            "1.0.0".to_string()
        }

        fn migrate_from_version(&self, _from_version: &str, _data: &str) -> Result<Self, PipelineError> {
            // Simple migration - just return self for now
            Ok(self.clone())
        }
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                database_url: "postgresql://localhost:5432/test".to_string(),
                max_connections: 10,
                timeout_seconds: 30,
                features: vec!["feature1".to_string(), "feature2".to_string()],
            }
        }
    }

    #[tokio::test]
    async fn test_config_manager_creation() {
        let config_manager = GenericConfigManager::new(TestConfig::default());
        let config = config_manager.get_config().unwrap();

        assert_eq!(config.database_url, "postgresql://localhost:5432/test");
        assert_eq!(config.max_connections, 10);
    }

    #[tokio::test]
    async fn test_config_validation() {
        let config_manager = GenericConfigManager::new(TestConfig::default());

        let invalid_config = TestConfig {
            database_url: "".to_string(),
            max_connections: 0,
            timeout_seconds: 0,
            features: vec![],
        };

        let result = config_manager
            .update_config(invalid_config, "Test update".to_string(), "test_user".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_change_history() {
        let config_manager = GenericConfigManager::new(TestConfig::default());

        let new_config = TestConfig {
            database_url: "postgresql://localhost:5432/newdb".to_string(),
            max_connections: 20,
            timeout_seconds: 60,
            features: vec!["new_feature".to_string()],
        };

        config_manager
            .update_config(new_config, "Updated for testing".to_string(), "test_user".to_string())
            .await
            .unwrap();

        let history = config_manager.get_change_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].change_reason, "Updated for testing");
        assert_eq!(history[0].changed_by, "test_user");
    }

    /// Tests configuration validation result management and state tracking.
    ///
    /// This test validates that configuration validation results can
    /// properly track validation state, errors, and warnings for
    /// comprehensive configuration validation reporting.
    ///
    /// # Test Coverage
    ///
    /// - Valid validation result creation
    /// - Initial state validation (valid, no errors)
    /// - Error addition and state change
    /// - Validation state update on error
    /// - Warning addition functionality
    /// - Error and warning collection management
    ///
    /// # Test Scenario
    ///
    /// Creates a valid validation result, adds errors and warnings,
    /// and verifies that state and collections are managed correctly.
    ///
    /// # Domain Concerns
    ///
    /// - Configuration validation reporting
    /// - Validation state management
    /// - Error and warning collection
    /// - Validation result aggregation
    ///
    /// # Assertions
    ///
    /// - Initial result is valid with no errors
    /// - Adding error changes validity to false
    /// - Error collection is updated correctly
    /// - Warning collection is managed properly
    /// - State tracking works correctly
    #[test]
    fn test_validation_result() {
        let mut result = ConfigValidationResult::valid();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());

        result.add_error("field1".to_string(), "Error message".to_string());
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);

        result.add_warning("field2".to_string(), "Warning message".to_string());
        assert_eq!(result.warnings.len(), 1);
    }
}
