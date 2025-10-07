// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Infrastructure Configuration
//!
//! This module provides configuration management for the infrastructure layer,
//! handling the setup and initialization of infrastructure components such as
//! databases, external services, caches, and system resources.
//!
//! ## Overview
//!
//! Infrastructure configuration manages:
//!
//! - **Database Connections**: Connection strings, pool settings, timeouts
//! - **External Services**: API endpoints, authentication credentials
//! - **System Resources**: Thread pools, memory limits, file descriptors
//! - **Caching**: Cache providers, TTL settings, eviction policies
//! - **Security**: TLS/SSL configuration, key management
//! - **Observability**: Logging, metrics, tracing configuration
//!
//! ## Design Principles
//!
//! ### Environment-Based Configuration
//! Configuration varies by environment (development, staging, production):
//!
//!
//! ### Validation
//! All configuration is validated on load:
//!
//!
//! ## Configuration Structure
//!
//! ### Database Configuration
//!
//!
//! ### Service Configuration
//!
//!
//! ### Cache Configuration
//!
//!
//! ## Configuration Loading
//!
//! ### From Files
//!
//! Load configuration from TOML, YAML, or JSON files:
//!
//!
//! ### From Environment Variables
//!
//! Override configuration with environment variables:
//!
//!
//! ### From Multiple Sources
//!
//! Combine multiple configuration sources with precedence:
//!
//!
//! ## Secrets Management
//!
//! Secure handling of sensitive configuration:
//!
//!
//! ## Example Configuration File
//!
//! ```toml
//! # config/production.toml
//! [database]
//! url = "postgresql://localhost:5432/pipeline"
//! pool_size = 20
//! connection_timeout_seconds = 30
//! query_timeout_seconds = 60
//! enable_logging = false
//!
//! [services.compression]
//! default_algorithm = "zstd"
//! default_level = 6
//! worker_threads = 8
//!
//! [services.encryption]
//! default_algorithm = "aes256gcm"
//! key_rotation_days = 90
//!
//! [cache]
//! provider = "redis"
//! servers = ["redis://localhost:6379"]
//! default_ttl_seconds = 3600
//! max_size_mb = 1024
//!
//! [observability]
//! log_level = "info"
//! metrics_enabled = true
//! tracing_enabled = true
//! tracing_sample_rate = 0.1
//! ```
//!
//! ## Configuration Updates
//!
//! Handle configuration updates at runtime:
//!
//!
//! ## Dependency Injection
//!
//! Use configuration to initialize infrastructure components:
//!
//!
//! ## Testing
//!
//! Use test-specific configuration:

pub mod config_service;
pub mod generic_config_manager;
pub mod rayon_config;
