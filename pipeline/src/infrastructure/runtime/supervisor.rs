// /////////////////////////////////////////////////////////////////////////////
// Optimized Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Task Supervision Utilities
//!
//! This module provides utilities for supervised concurrent task execution
//! with proper error handling, logging, and lifecycle management.
//!
//! ## Purpose
//!
//! - Prevents spawn-and-forget anti-patterns
//! - Ensures all task errors are captured and propagated
//! - Provides structured logging for task lifecycle
//! - Facilitates graceful shutdown and error recovery
//!
//! ## Educational Value
//!
//! Demonstrates proper concurrent task management patterns:
//! - Supervised task spawning
//! - Error propagation from background tasks
//! - Structured logging for observability
//! - No silent failures

use tokio::task::JoinHandle;
use tracing::{debug, error};
use pipeline_domain::PipelineError;

/// Result type alias for application operations
pub type AppResult<T> = Result<T, PipelineError>;

/// Spawns a supervised task with automatic error logging and lifecycle tracking.
///
/// ## Purpose
///
/// Wraps `tokio::spawn` with supervision that:
/// - Logs task start (debug level)
/// - Logs task completion/failure (debug/error level)
/// - Returns a `JoinHandle` that must be awaited
/// - Ensures no silent failures
///
/// ## Educational Pattern
///
/// This prevents the **spawn-and-forget anti-pattern** by:
/// 1. Requiring the caller to await the returned handle
/// 2. Logging all task outcomes for observability
/// 3. Making task failures visible in logs immediately
///
/// ## Parameters
///
/// - `name`: Static string identifying this task (for logging)
/// - `fut`: Async function returning `AppResult<T>`
///
/// ## Returns
///
/// `JoinHandle<AppResult<T>>` that must be awaited by caller
///
/// ## Example
///
/// ```ignore
/// let handle = spawn_supervised("reader-task", async move {
///     read_data().await?;
///     Ok(())
/// });
///
/// // Later: must await the handle
/// join_supervised(handle).await?;
/// ```
pub fn spawn_supervised<F, T>(name: &'static str, fut: F) -> JoinHandle<AppResult<T>>
where
    F: std::future::Future<Output = AppResult<T>> + Send + 'static,
    T: Send + 'static,
{
    debug!(task = name, "task starting");

    tokio::spawn(async move {
        let result = fut.await;

        match &result {
            Ok(_) => debug!(task = name, "task completed successfully"),
            Err(e) => error!(task = name, error = ?e, "task failed"),
        }

        result
    })
}

/// Awaits a supervised task handle and propagates errors.
///
/// ## Purpose
///
/// Joins a spawned task and handles both:
/// - Task panics (via JoinError)
/// - Task result errors (via AppResult)
///
/// ## Educational Pattern
///
/// This demonstrates **error propagation** from background tasks:
/// - Converts task panics to typed errors
/// - Preserves original error types
/// - Ensures no error is lost
///
/// ## Parameters
///
/// - `handle`: JoinHandle from `spawn_supervised`
///
/// ## Returns
///
/// The task's result, or an error if the task panicked or failed
///
/// ## Example
///
/// ```ignore
/// let handle = spawn_supervised("worker", async { work().await });
/// let result = join_supervised(handle).await?;
/// ```
pub async fn join_supervised<T>(handle: JoinHandle<AppResult<T>>) -> AppResult<T> {
    let join_result: Result<AppResult<T>, tokio::task::JoinError> = handle.await;

    match join_result {
        Ok(task_result) => task_result,
        Err(e) => {
            if e.is_panic() {
                Err(PipelineError::internal_error(format!("task panicked: {}", e)))
            } else if e.is_cancelled() {
                Err(PipelineError::cancelled())
            } else {
                Err(PipelineError::internal_error(format!("task join failed: {}", e)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_supervised_success() {
        let handle = spawn_supervised("test-success", async {
            Ok::<i32, PipelineError>(42)
        });

        let result: AppResult<i32> = join_supervised(handle).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_spawn_supervised_error() {
        let handle = spawn_supervised("test-error", async {
            Err::<(), _>(PipelineError::validation_error("test error"))
        });

        let result: AppResult<()> = join_supervised(handle).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_supervised_panic() {
        let handle = tokio::spawn(async {
            panic!("test panic");
            #[allow(unreachable_code)]
            Ok::<(), PipelineError>(())
        });

        let result: AppResult<()> = join_supervised(handle).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("panicked"));
    }
}
