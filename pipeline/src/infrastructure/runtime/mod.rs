// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Runtime Infrastructure
//!
//! This module provides runtime infrastructure for resource management,
//! concurrency control, and system-level coordination.
//!
//! ## Modules
//!
//! - **resource_manager**: Global resource governance (CPU, I/O, memory)
//! - **supervisor**: Supervised task spawning with error handling and logging
//!
//! ## Educational Purpose
//!
//! This module demonstrates enterprise patterns for:
//! - Centralized resource control
//! - System-wide coordination
//! - Prevention of resource oversubscription
//! - Supervised concurrent task execution

pub mod resource_manager;
pub mod supervisor;

// Re-export commonly used types
pub use resource_manager::{
    GlobalResourceManager,
    ResourceConfig,
    StorageType,
    RESOURCE_MANAGER,
    init_resource_manager,
    resource_manager,
};

pub use supervisor::{
    spawn_supervised,
    join_supervised,
    AppResult,
};
