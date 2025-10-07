// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Interface Layer
//!
//! The interface layer provides external interfaces for user interaction and
//! system integration. This layer handles all incoming requests and translates
//! them into application layer operations.
//!
//! ## Architecture
//!
//! The interface layer follows the Clean Architecture pattern and serves as
//! the outermost layer that:
//!
//! - Receives external requests (CLI, HTTP, etc.)
//! - Validates and sanitizes input
//! - Translates requests to application commands
//! - Formats and returns responses
//! - Handles authentication and authorization
//!
//! ## Module Structure
//!
//! ```text
//! interface/
//! ├── api/         # REST API and web interfaces
//! ├── cli/         # Command-line interface
//! └── config/      # Configuration management
//! ```
//!
//! ## Command-Line Interface (CLI)
//!
//! The CLI provides a user-friendly command-line interface for pipeline
//! operations:
//!
//! **Features:**
//! - Interactive and non-interactive modes
//! - Progress indicators and status updates
//! - Comprehensive help and documentation
//! - Configuration file support
//! - Error handling with user-friendly messages
//!
//! **Example Usage:**
//! ```bash
//! # Process a file with a specific pipeline
//! pipeline process --input input.txt --output output.adapipe --pipeline secure-backup
//!
//! # List available pipelines
//! pipeline list-pipelines
//!
//! # Restore a processed file
//! pipeline restore --input output.adapipe --output restored.txt
//!
//! # Create a new pipeline configuration
//! pipeline create-pipeline --name my-pipeline --config pipeline.toml
//! ```
//!
//! **Implementation:**
//!
//!
//!
//! ## REST API
//!
//! The REST API provides programmatic access to pipeline functionality:
//!
//! **Endpoints:**
//! - `POST /api/v1/pipelines/{id}/process` - Process a file
//! - `GET /api/v1/pipelines` - List pipelines
//! - `POST /api/v1/pipelines` - Create pipeline
//! - `GET /api/v1/pipelines/{id}/status` - Get processing status
//!
//! **Example API Usage:**
//! ```bash
//! # Process a file via REST API
//! curl -X POST http://localhost:8080/api/v1/pipelines/secure-backup/process \
//!   -H "Content-Type: application/json" \
//!   -d '{
//!     "input_path": "/path/to/input.txt",
//!     "output_path": "/path/to/output.adapipe"
//!   }'
//! ```
//!
//! **Implementation:**
//!
//!
//! ## Configuration Management
//!
//! The configuration module handles system configuration from multiple sources:
//!
//! **Configuration Sources:**
//! - Configuration files (TOML, YAML, JSON)
//! - Environment variables
//! - Command-line arguments
//! - Default values
//!
//! **Configuration Structure:**
//! ```toml
//! # pipeline.toml
//! [server]
//! host = "0.0.0.0"
//! port = 8080
//!
//! [database]
//! url = "sqlite:///pipeline.db"
//! max_connections = 10
//!
//! [security]
//! default_level = "confidential"
//! encryption_algorithm = "aes256gcm"
//!
//! [processing]
//! default_chunk_size = "1MB"
//! max_workers = 4
//! ```
//!
//! **Implementation:**
//!
//!
//! ## Input Validation
//!
//! The interface layer performs comprehensive input validation:
//!
//!
//!
//! ## Error Handling
//!
//! The interface layer translates domain errors into user-friendly messages:
//!
//!
//!
//! ## Authentication and Authorization
//!
//! The interface layer handles security concerns:
//!
//!
//!
//! ## Testing Strategy
//!
//! Interface layer components are tested with:
//!
//! - **Unit Tests**: Test individual handlers and validators
//! - **Integration Tests**: Test complete request/response flows
//! - **End-to-End Tests**: Test user workflows through actual interfaces
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use axum_test::StringestServer;
//!
//!     #[test]
//!     fn test_process_file_endpoint() {
//!         // Arrange: Set up test server
//!         let app = create_test_app()?;
//!         let server = StringestServer::new(app)?;
//!
//!         // Act: Make API request
//!         let response = server
//!             .post("/api/v1/pipelines/test/process")
//!             .json(&ProcessRequest {
//!                 input_path: "test.txt".to_string(),
//!                 output_path: "test.adapipe".to_string(),
//!             })?;
//!
//!         // Assert: Verify response
//!         response.assert_status_ok();
//!         let result: ProcessResponse = response.json();
//!         assert!(result.success);
//!         println!("API endpoint test passed");
//!     }
//! }
//! ```

//! # Interface Layer
//!
//! The interface layer provides external interfaces for interacting with the
//! pipeline processing system. It includes API endpoints, CLI commands, and
//! configuration management that allow users and external systems to access the
//! application.
//!
//! ## Overview
//!
//! The interface layer provides:
//!
//! - **API Endpoints**: HTTP/REST API for programmatic access
//! - **CLI Interface**: Command-line interface for interactive use
//! - **Configuration**: System configuration and settings management
//! - **Input Validation**: Validates and sanitizes external input
//! - **Response Formatting**: Formats responses for different interfaces
//!
//! ## Components
//!
//! ### API
//! RESTful HTTP API for external system integration:
//! - Pipeline management endpoints
//! - File processing operations
//! - Status and monitoring endpoints
//!
//! ### CLI
//! Command-line interface for direct user interaction:
//! - Interactive commands
//! - Batch processing support
//! - Configuration management
//!
//! ### Configuration
//! System configuration and settings:
//! - Application settings
//! - Pipeline configurations
//! - Environment-specific settings

pub mod adapters;
