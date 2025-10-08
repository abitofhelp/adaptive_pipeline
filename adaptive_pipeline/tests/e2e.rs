// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! End-to-End Tests
//!
//! This module aggregates all E2E tests for the pipeline application.

// Shared test helpers
mod common;

#[path = "e2e/e2e_binary_format_test.rs"]
mod e2e_binary_format_test;

#[path = "e2e/e2e_restore_pipeline_test.rs"]
mod e2e_restore_pipeline_test;

#[path = "e2e/e2e_use_cases_test.rs"]
mod e2e_use_cases_test;
