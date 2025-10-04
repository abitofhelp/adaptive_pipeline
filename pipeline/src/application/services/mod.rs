// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Application Services
//!
//! This module contains application services that orchestrate complex domain
//! operations and coordinate between domain objects, repositories, and
//! infrastructure services. Application services implement the workflow
//! coordination layer of the application.
//!
//! ## Overview
//!
//! Application services provide:
//!
//! - **Workflow Orchestration**: Coordinate complex multi-step operations
//! - **Transaction Management**: Ensure data consistency across operations
//! - **Cross-Cutting Concerns**: Handle logging, metrics, and monitoring
//! - **Domain Coordination**: Bridge between domain entities and infrastructure
//! - **Business Process Implementation**: Implement high-level business
//!   workflows
//!
//! ## Service Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │         Application Services            │
//! │  ┌─────────────┐ ┌─────────────────┐    │
//! │  │  Pipeline   │ │  Processing    │    │
//! │  │  Service    │ │   Service     │    │
//! │  └─────────────┘ └─────────────────┘    │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │            Domain Layer                 │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐    │
//! │  │Entities │ │Services │ │ Events  │    │
//! │  └─────────┘ └─────────┘ └─────────┘    │
//! └─────────────────┬───────────────────────┘
//!                   │
//! ┌─────────────────▼───────────────────────┐
//! │        Infrastructure Layer         │
//! │  ┌─────────┐ ┌─────────┐ ┌─────────┐    │
//! │  │Database │ │File I/O │ │External │    │
//! │  └─────────┘ └─────────┘ └─────────┘    │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Service Types
//!
//! ### Pipeline Management Service
//! Orchestrates pipeline lifecycle operations:

pub mod file_processor_service;
pub mod pipeline_service;
pub mod transactional_chunk_writer;

// Re-export for convenient access
pub use transactional_chunk_writer::TransactionalChunkWriter;
