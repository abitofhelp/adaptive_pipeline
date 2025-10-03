//! # System Signal Handling
//!
//! Cross-platform signal handling for graceful shutdown.
//!
//! ## Supported Signals
//!
//! - **SIGTERM** (15) - Graceful shutdown request
//! - **SIGINT** (2) - User interrupt (Ctrl+C)
//! - **SIGHUP** (1) - Hangup (terminal closed)
//!
//! ## Design Pattern
//!
//! The signal handler provides:
//! - **Async signal handling** via tokio
//! - **Trait abstraction** for testing
//! - **Callback-based** shutdown initiation
//! - **Platform-specific** implementations (Unix vs Windows)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bootstrap::signals::{SystemSignals, UnixSignalHandler};
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicBool, Ordering};
//!
//! #[tokio::main]
//! async fn main() {
//!     let shutdown_flag = Arc::new(AtomicBool::new(false));
//!     let flag_clone = shutdown_flag.clone();
//!
//!     let signal_handler = UnixSignalHandler::new();
//!
//!     // Install signal handlers
//!     tokio::spawn(async move {
//!         let callback = Box::new(move || {
//!             flag_clone.store(true, Ordering::SeqCst);
//!         });
//!         signal_handler.wait_for_signal(callback).await;
//!     });
//!
//!     // Main application loop
//!     while !shutdown_flag.load(Ordering::SeqCst) {
//!         // Application work...
//!         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//!     }
//! }
//! ```

use std::future::Future;
use std::pin::Pin;

/// Callback type for shutdown notification
pub type ShutdownCallback = Box<dyn FnOnce() + Send + 'static>;

/// System signal handling trait
///
/// Abstracts platform-specific signal handling for graceful shutdown.
pub trait SystemSignals: Send + Sync {
    /// Wait for a shutdown signal and invoke the callback
    ///
    /// This method blocks until one of the shutdown signals is received:
    /// - SIGTERM
    /// - SIGINT
    /// - SIGHUP (Unix only)
    ///
    /// When a signal is received, the provided callback is invoked to
    /// initiate graceful shutdown.
    fn wait_for_signal(&self, on_shutdown: ShutdownCallback) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

/// Unix signal handler implementation
///
/// Handles SIGTERM, SIGINT, and SIGHUP using tokio::signal.
#[cfg(unix)]
pub struct UnixSignalHandler;

#[cfg(unix)]
impl UnixSignalHandler {
    /// Create a new Unix signal handler
    pub fn new() -> Self {
        Self
    }
}

#[cfg(unix)]
impl Default for UnixSignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(unix)]
impl SystemSignals for UnixSignalHandler {
    fn wait_for_signal(&self, on_shutdown: ShutdownCallback) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sigterm = signal(SignalKind::terminate())
                .expect("Failed to register SIGTERM handler");
            let mut sigint = signal(SignalKind::interrupt())
                .expect("Failed to register SIGINT handler");
            let mut sighup = signal(SignalKind::hangup())
                .expect("Failed to register SIGHUP handler");

            tokio::select! {
                _ = sigterm.recv() => {
                    tracing::info!("Received SIGTERM, initiating graceful shutdown");
                }
                _ = sigint.recv() => {
                    tracing::info!("Received SIGINT (Ctrl+C), initiating graceful shutdown");
                }
                _ = sighup.recv() => {
                    tracing::info!("Received SIGHUP, initiating graceful shutdown");
                }
            }

            on_shutdown();
        })
    }
}

/// Windows signal handler implementation
///
/// Handles Ctrl+C and Ctrl+Break on Windows.
#[cfg(windows)]
pub struct WindowsSignalHandler;

#[cfg(windows)]
impl WindowsSignalHandler {
    /// Create a new Windows signal handler
    pub fn new() -> Self {
        Self
    }
}

#[cfg(windows)]
impl Default for WindowsSignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl SystemSignals for WindowsSignalHandler {
    fn wait_for_signal(&self, on_shutdown: ShutdownCallback) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            // On Windows, tokio provides ctrl_c signal
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to register Ctrl+C handler");

            tracing::info!("Received Ctrl+C, initiating graceful shutdown");
            on_shutdown();
        })
    }
}

/// No-op signal handler for testing
///
/// Never receives signals, allowing tests to control shutdown explicitly.
pub struct NoOpSignalHandler;

impl NoOpSignalHandler {
    /// Create a new no-op signal handler
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoOpSignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemSignals for NoOpSignalHandler {
    fn wait_for_signal(&self, _on_shutdown: ShutdownCallback) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        // Never completes - perfect for testing
        Box::pin(async move {
            std::future::pending::<()>().await;
        })
    }
}

/// Create platform-specific signal handler
///
/// Returns the appropriate signal handler for the current platform:
/// - Unix: `UnixSignalHandler`
/// - Windows: `WindowsSignalHandler`
pub fn create_signal_handler() -> Box<dyn SystemSignals> {
    #[cfg(unix)]
    {
        Box::new(UnixSignalHandler::new())
    }

    #[cfg(windows)]
    {
        Box::new(WindowsSignalHandler::new())
    }

    #[cfg(not(any(unix, windows)))]
    {
        compile_error!("Unsupported platform for signal handling");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_noop_signal_handler() {
        let handler = NoOpSignalHandler::new();
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        // Start waiting for signal (will never complete)
        let callback = Box::new(move || {
            called_clone.store(true, Ordering::SeqCst);
        });
        let wait_future = handler.wait_for_signal(callback);

        // Race the signal wait against a timeout
        tokio::select! {
            _ = wait_future => {
                panic!("NoOp handler should never complete");
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Expected - timeout wins
            }
        }

        // Callback should not have been called
        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_create_signal_handler() {
        // Just verify it doesn't panic
        let _handler = create_signal_handler();
    }

    #[cfg(unix)]
    #[test]
    fn test_unix_signal_handler_creation() {
        let _handler = UnixSignalHandler::new();
        let _handler = UnixSignalHandler::default();
    }

    #[cfg(windows)]
    #[test]
    fn test_windows_signal_handler_creation() {
        let _handler = WindowsSignalHandler::new();
        let _handler = WindowsSignalHandler::default();
    }
}
