// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Shutdown Coordination
//!
//! Manages graceful shutdown across application components.
//!
//! ## Design Pattern
//!
//! The shutdown coordinator provides:
//! - **Cancellation tokens** for propagating shutdown signals
//! - **Grace period** with timeout enforcement
//! - **Atomic state** for shutdown tracking
//! - **Async-aware** shutdown orchestration
//!
//! ## Usage
//!
//! ```rust
//! use adaptive_pipeline_bootstrap::shutdown::ShutdownCoordinator;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
//!
//!     // Clone token for worker tasks
//!     let token = coordinator.token();
//!
//!     // Spawn worker
//!     tokio::spawn(async move {
//!         loop {
//!             tokio::select! {
//!                 _ = token.cancelled() => {
//!                     println!("Worker shutting down gracefully");
//!                     break;
//!                 }
//!                 _ = tokio::time::sleep(Duration::from_secs(1)) => {
//!                     println!("Working...");
//!                 }
//!             }
//!         }
//!     });
//!
//!     // Later: initiate shutdown
//!     coordinator.initiate_shutdown();
//!     coordinator.wait_for_shutdown().await;
//! }
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;

/// Default grace period for graceful shutdown (in seconds)
pub const DEFAULT_GRACE_PERIOD_SECS: u64 = 5;

/// Cancellation token for signaling shutdown
///
/// Lightweight clone-able token that can be passed to async tasks.
///
/// # Examples
///
/// ```
/// use adaptive_pipeline_bootstrap::shutdown::ShutdownCoordinator;
/// use std::time::Duration;
///
/// # async fn example() {
/// let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
/// let token = coordinator.token();
///
/// // Pass token to async task
/// tokio::spawn(async move {
///     tokio::select! {
///         _ = token.cancelled() => {
///             println!("Task received shutdown signal");
///         }
///         _ = async { /* do work */ } => {
///             println!("Task completed normally");
///         }
///     }
/// });
///
/// // Later, initiate shutdown
/// coordinator.initiate_shutdown();
/// # }
/// ```
#[derive(Clone)]
pub struct CancellationToken {
    /// Shared cancellation flag
    cancelled: Arc<AtomicBool>,
    /// Notification for waiters
    notify: Arc<Notify>,
}

impl CancellationToken {
    /// Create a new cancellation token
    fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Cancel this token
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        self.notify.notify_waiters();
    }

    /// Check if cancelled (non-blocking)
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Wait for cancellation (async)
    pub async fn cancelled(&self) {
        if self.is_cancelled() {
            return;
        }
        self.notify.notified().await;
    }
}

/// Shutdown coordinator
///
/// Manages graceful shutdown with grace period and timeout enforcement.
#[derive(Clone)]
pub struct ShutdownCoordinator {
    /// Cancellation token for shutdown signal
    token: CancellationToken,

    /// Grace period before forced shutdown
    grace_period: Duration,

    /// Shutdown initiated flag
    shutdown_initiated: Arc<AtomicBool>,

    /// Notification for shutdown completion
    shutdown_complete: Arc<Notify>,
}

impl ShutdownCoordinator {
    /// Create a new shutdown coordinator
    ///
    /// # Arguments
    ///
    /// * `grace_period` - Maximum time to wait for graceful shutdown
    pub fn new(grace_period: Duration) -> Self {
        Self {
            token: CancellationToken::new(),
            grace_period,
            shutdown_initiated: Arc::new(AtomicBool::new(false)),
            shutdown_complete: Arc::new(Notify::new()),
        }
    }

    /// Get a cancellation token
    ///
    /// Tokens can be cloned and passed to async tasks for shutdown signaling.
    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Check if shutdown has been initiated
    pub fn is_shutting_down(&self) -> bool {
        self.shutdown_initiated.load(Ordering::SeqCst)
    }

    /// Initiate graceful shutdown
    ///
    /// This will:
    /// 1. Set shutdown initiated flag
    /// 2. Cancel all tokens
    /// 3. Start grace period timer
    pub fn initiate_shutdown(&self) {
        if self
            .shutdown_initiated
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            tracing::info!("Initiating graceful shutdown (grace period: {:?})", self.grace_period);
            self.token.cancel();
        }
    }

    /// Wait for shutdown to complete or timeout
    ///
    /// Returns `true` if shutdown completed within grace period,
    /// `false` if timeout occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use adaptive_pipeline_bootstrap::shutdown::ShutdownCoordinator;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
    ///
    /// // In main application loop
    /// coordinator.initiate_shutdown();
    ///
    /// // Wait for all tasks to complete
    /// if coordinator.wait_for_shutdown().await {
    ///     println!("Shutdown completed gracefully");
    /// } else {
    ///     println!("Shutdown timed out, forcing exit");
    /// }
    ///
    /// coordinator.complete_shutdown();
    /// # }
    /// ```
    pub async fn wait_for_shutdown(&self) -> bool {
        if !self.is_shutting_down() {
            tracing::warn!("wait_for_shutdown called but shutdown not initiated");
            return true;
        }

        // Race shutdown completion against timeout
        tokio::select! {
            _ = self.shutdown_complete.notified() => {
                tracing::info!("Shutdown completed gracefully");
                true
            }
            _ = tokio::time::sleep(self.grace_period) => {
                tracing::warn!("Shutdown grace period expired, forcing shutdown");
                false
            }
        }
    }

    /// Signal that shutdown is complete
    ///
    /// Call this after all cleanup is done to notify waiters.
    pub fn complete_shutdown(&self) {
        self.shutdown_complete.notify_waiters();
    }

    /// Wait for shutdown with custom timeout
    pub async fn wait_with_timeout(&self, timeout: Duration) -> bool {
        if !self.is_shutting_down() {
            return true;
        }

        tokio::select! {
            _ = self.shutdown_complete.notified() => true,
            _ = tokio::time::sleep(timeout) => false,
        }
    }
}

impl Default for ShutdownCoordinator {
    fn default() -> Self {
        Self::new(Duration::from_secs(DEFAULT_GRACE_PERIOD_SECS))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation_token_create() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
    }

    #[test]
    fn test_cancellation_token_cancel() {
        let token = CancellationToken::new();
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_cancellation_token_clone() {
        let token1 = CancellationToken::new();
        let token2 = token1.clone();

        token1.cancel();
        assert!(token2.is_cancelled());
    }

    #[tokio::test]
    async fn test_cancellation_token_cancelled_already() {
        let token = CancellationToken::new();
        token.cancel();

        // Should return immediately
        token.cancelled().await;
        assert!(token.is_cancelled());
    }

    #[tokio::test]
    async fn test_cancellation_token_cancelled_wait() {
        let token = CancellationToken::new();
        let token_clone = token.clone();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            token_clone.cancel();
        });

        token.cancelled().await;
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_shutdown_coordinator_create() {
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        assert!(!coordinator.is_shutting_down());
    }

    #[test]
    fn test_shutdown_coordinator_default() {
        let coordinator = ShutdownCoordinator::default();
        assert!(!coordinator.is_shutting_down());
    }

    #[test]
    fn test_shutdown_coordinator_initiate() {
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        coordinator.initiate_shutdown();
        assert!(coordinator.is_shutting_down());
        assert!(coordinator.token().is_cancelled());
    }

    #[test]
    fn test_shutdown_coordinator_token() {
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));
        let token = coordinator.token();

        assert!(!token.is_cancelled());

        coordinator.initiate_shutdown();
        assert!(token.is_cancelled());
    }

    #[tokio::test]
    async fn test_shutdown_coordinator_complete() {
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(5));

        coordinator.initiate_shutdown();

        let coordinator_clone = coordinator.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            coordinator_clone.complete_shutdown();
        });

        // This should complete quickly (not timeout)
        let result = tokio::time::timeout(Duration::from_millis(200), coordinator.wait_for_shutdown()).await;

        assert!(result.is_ok());
        assert!(result.unwrap()); // True = completed, not timed out
    }

    #[tokio::test]
    async fn test_shutdown_coordinator_timeout() {
        let coordinator = ShutdownCoordinator::new(Duration::from_millis(50));

        coordinator.initiate_shutdown();
        // Don't call complete_shutdown - let it timeout

        let result = coordinator.wait_for_shutdown().await;
        assert!(!result); // False = timed out
    }

    #[tokio::test]
    async fn test_shutdown_coordinator_wait_custom_timeout() {
        let coordinator = ShutdownCoordinator::new(Duration::from_secs(10));

        coordinator.initiate_shutdown();

        // Use shorter custom timeout
        let result = coordinator.wait_with_timeout(Duration::from_millis(50)).await;
        assert!(!result); // Timed out
    }
}
