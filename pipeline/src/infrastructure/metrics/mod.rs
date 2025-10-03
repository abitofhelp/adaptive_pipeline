// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////


//! Metrics Module
//!
//! This module is part of the Infrastructure layer, providing metrics
//! collection and monitoring capabilities following the Hexagonal Architecture
//! pattern.

pub mod concurrency_metrics;
pub mod generic_metrics_collector;
pub mod metrics_endpoint;
pub mod metrics_observer;
pub mod metrics_service;

pub use concurrency_metrics::*;
pub use generic_metrics_collector::*;
pub use metrics_endpoint::*;
pub use metrics_observer::*;
pub use metrics_service::*;
