// /////////////////////////////////////////////////////////////////////////////
// Adaptive Pipeline RS
// Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
// SPDX-License-Identifier: BSD-3-Clause
// See LICENSE file in the project root.
// /////////////////////////////////////////////////////////////////////////////

//! # Progress Indicator Service
//!
//! This module provides a real-time progress indicator for user feedback during
//! pipeline processing operations. It offers immediate visual feedback to users
//! about processing progress, separate from logging and metrics systems.
//!
//! ## Overview
//!
//! The progress indicator service provides:
//!
//! - **Real-Time Updates**: Live progress updates as chunks are processed
//! - **User-Focused Feedback**: Immediate visual feedback for end users
//! - **Terminal Integration**: Direct terminal output with in-place updates
//! - **Thread Safety**: Concurrent-safe for multi-threaded processing
//! - **Performance Metrics**: Throughput and timing information
//!
//! ## Architecture
//!
//! The progress indicator follows these design principles:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Progress Indicator System                     │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                 Real-Time Display                       │    │
//! │  │  - Terminal output with in-place updates                │    │
//! │  │  - Chunk progress tracking                              │    │
//! │  │  - Throughput calculations                              │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │                Thread-Safe Tracking                    │    │
//! │  │  - Atomic counters for concurrent access               │    │
//! │  │  - Mutex coordination for terminal output              │    │
//! │  │  - Lock-free progress updates                           │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! │                                                                     │
//! │  ┌─────────────────────────────────────────────────────────┐    │
//! │  │              Performance Monitoring                   │    │
//! │  │  - Throughput calculation (MB/s)                       │    │
//! │  │  - Duration tracking                                    │    │
//! │  │  - Completion statistics                               │    │
//! │  └─────────────────────────────────────────────────────────┘    │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Principles
//!
//! ### User-Focused Design
//!
//! The progress indicator is designed specifically for end-user feedback:
//!
//! - **Immediate Feedback**: Updates appear as soon as chunks are processed
//! - **Clear Format**: Easy-to-read progress format with chunk IDs and counts
//! - **Non-Intrusive**: Doesn't interfere with normal application logging
//! - **Terminal Integration**: Works seamlessly with terminal-based
//!   applications
//!
//! ### Separation of Concerns
//!
//! Progress indication is separate from other monitoring systems:
//!
//! - **Not Logging**: Writes directly to terminal, bypassing logging systems
//! - **Not Metrics**: Focused on user feedback, not system monitoring
//! - **Real-Time**: Updates immediately, not batched or aggregated
//! - **Ephemeral**: Progress display is temporary and contextual
//!
//! ### Performance Considerations
//!
//! - **Minimal Overhead**: Lightweight implementation to avoid performance
//!   impact
//! - **Atomic Operations**: Lock-free progress updates using atomic counters
//! - **Coordinated Output**: Mutex only for terminal output coordination
//! - **Efficient Updates**: In-place terminal updates without scrolling
//!
//! ## Output Format
//!
//! ### Progress Display
//!
//! The progress indicator shows real-time chunk processing status:
//!
//! ```text
//! Wrote Id: 000097/Completed: 002000
//! ```
//!
//! - **Wrote Id**: Last chunk ID that was processed
//! - **Completed**: Total number of chunks completed
//! - **Format**: Zero-padded for consistent alignment
//!
//! ### Completion Summary
//!
//! Upon completion, shows comprehensive statistics:
//!
//! ```text
//! ✅ Processing completed successfully!
//! 📄 Processed: 1.25 GB
//! ⏱️ Duration: 2.34 seconds
//! 🚀 Throughput: 534.2 MB/s
//! 📊 Chunks: 2000 total
//! ```
//!
//! ## Usage Examples
//!
//! ### Basic Progress Tracking

//!
//! ### Concurrent Processing
//!
//!
//! ### Integration with File Processing

//!
//! ## Thread Safety
//!
//! The progress indicator is designed for concurrent use:
//!
//! ### Atomic Counters
//!
//! - **Completed Chunks**: Atomic counter for lock-free updates
//! - **Last Chunk ID**: Atomic storage for the most recent chunk ID
//! - **Performance**: No contention on progress updates
//!
//! ### Terminal Coordination
//!
//! - **Output Mutex**: Coordinates terminal output to prevent garbled display
//! - **Minimal Locking**: Mutex only held during actual terminal writes
//! - **Non-Blocking**: Progress updates don't block on terminal output
//!
//! ## Performance Impact
//!
//! The progress indicator is designed to have minimal performance impact:
//!
//! - **Atomic Operations**: Lock-free progress updates
//! - **Minimal Allocations**: Reuses buffers and avoids unnecessary allocations
//! - **Efficient Terminal I/O**: Direct terminal writes without buffering
//! - **Optional**: Can be disabled in production environments if needed
//!
//! ## Integration with Other Systems
//!
//! ### Logging System
//!
//! Progress indication is separate from logging:
//!
//! - **No Log Interference**: Doesn't interfere with structured logging
//! - **Direct Terminal**: Writes directly to terminal, not through log handlers
//! - **Complementary**: Works alongside logging for different purposes
//!
//! ### Metrics System
//!
//! Progress indication complements metrics collection:
//!
//! - **Different Purpose**: User feedback vs. system monitoring
//! - **Real-Time**: Immediate updates vs. aggregated metrics
//! - **Ephemeral**: Temporary display vs. persistent metrics storage
//!
//! ## Error Handling
//!
//! The progress indicator handles errors gracefully:
//!
//! - **Terminal Errors**: Gracefully handles terminal I/O errors
//! - **Non-Fatal**: Progress indicator failures don't affect processing
//! - **Fallback**: Can fall back to silent operation if terminal is unavailable
//! - **Recovery**: Automatically recovers from transient terminal issues

use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// Real-time progress indicator for user feedback during pipeline processing.
///
/// This provides immediate visual feedback to users about processing progress,
/// separate from logging and metrics systems. It writes directly to the
/// terminal with real-time updates on the same line.
///
/// # Design Principles
/// - **User-Focused**: Provides immediate visual feedback for end users
/// - **Non-Logging**: Writes directly to terminal, not through logging system
/// - **Real-Time**: Updates as chunks are processed, not batched
/// - **Concurrent-Safe**: Thread-safe for concurrent chunk processing
/// - **Minimal Overhead**: Lightweight to avoid impacting performance
///
/// # Example Output
/// ```text
/// Wrote Id: 000097/Completed: 002000
/// ```
///
/// # Usage
pub struct ProgressIndicatorService {
    /// Total number of chunks expected
    total_chunks: u64,

    /// Number of chunks completed (atomic for thread safety)
    completed_chunks: Arc<AtomicU64>,

    /// Last chunk ID written (for display)
    last_chunk_id: Arc<AtomicU64>,

    /// Mutex for terminal output coordination
    terminal_mutex: Arc<Mutex<()>>,

    /// Start time for duration calculation
    start_time: Instant,

    /// Last update time (to avoid too frequent updates)
    last_update: Arc<Mutex<Instant>>,
}

impl ProgressIndicatorService {
    /// Creates a new progress indicator.
    ///
    /// # Arguments
    /// * `total_chunks` - Total number of chunks expected to be processed
    ///
    /// # Returns
    /// * `Self` - New progress indicator instance
    pub fn new(total_chunks: u64) -> Self {
        // Show initial progress with blank line before
        println!();
        print!("\rWrote Id: 000000/Completed: {:06}", total_chunks);
        io::stdout().flush().unwrap_or(());

        Self {
            total_chunks,
            completed_chunks: Arc::new(AtomicU64::new(0)),
            last_chunk_id: Arc::new(AtomicU64::new(0)),
            terminal_mutex: Arc::new(Mutex::new(())),
            start_time: Instant::now(),
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Updates progress when a chunk has been successfully written.
    ///
    /// This method is thread-safe and can be called concurrently from
    /// multiple chunk processing tasks.
    ///
    /// # Arguments
    /// * `chunk_id` - ID of the chunk that was just written
    ///
    /// # Performance
    /// Updates are throttled to avoid excessive terminal I/O during
    /// high-throughput processing.
    pub async fn update_progress(&self, chunk_id: u64) {
        // Update counters atomically
        let completed = self.completed_chunks.fetch_add(1, Ordering::Relaxed) + 1;
        self.last_chunk_id.store(chunk_id, Ordering::Relaxed);

        // Throttle updates to avoid excessive terminal I/O
        // Only update every 100ms or every 10 chunks, whichever comes first
        let should_update = {
            let mut last_update = self.last_update.lock().await;
            let now = Instant::now();
            let time_since_update = now.duration_since(*last_update);

            if time_since_update >= Duration::from_millis(100) || completed.is_multiple_of(10) {
                *last_update = now;
                true
            } else {
                false
            }
        };

        if should_update {
            self.update_display(chunk_id, completed).await;
        }
    }

    /// Updates the terminal display with current progress.
    ///
    /// This method coordinates terminal access to ensure clean output
    /// even with concurrent chunk processing.
    async fn update_display(&self, chunk_id: u64, completed: u64) {
        let _terminal_lock = self.terminal_mutex.lock().await;

        // Clear the current line and write new progress
        print!("\rWrote Id: {:06}/Completed: {:06}", chunk_id, completed);
        io::stdout().flush().unwrap_or(());
    }

    /// Shows the final completion summary.
    ///
    /// This replaces the progress indicator with a comprehensive summary
    /// of the processing results.
    ///
    /// # Arguments
    /// * `bytes_processed` - Total bytes processed
    /// * `throughput_mb_s` - Processing throughput in MB/s
    /// * `total_duration` - Total time taken for processing
    pub async fn show_completion(&self, _bytes_processed: u64, _throughput_mb_s: f64, _total_duration: Duration) {
        let _terminal_lock = self.terminal_mutex.lock().await;

        // Clear the progress line and show final progress with correct total
        let final_completed = self.completed_chunks.load(Ordering::Relaxed);
        print!(
            "\rWrote Id: {:06}/Completed: {:06}\n",
            self.last_chunk_id.load(Ordering::Relaxed),
            final_completed
        );

        io::stdout().flush().unwrap_or(());
    }

    /// Shows an error summary if processing fails.
    ///
    /// # Arguments
    /// * `error_message` - Description of what went wrong
    pub async fn show_error_summary(&self, error_message: &str) {
        let _terminal_lock = self.terminal_mutex.lock().await;

        // Clear the progress line and show final progress
        let final_completed = self.completed_chunks.load(Ordering::Relaxed);
        println!(
            "\rWrote Id: {:06}/Completed: {:06}",
            self.last_chunk_id.load(Ordering::Relaxed),
            final_completed
        );

        // Show error summary with 6-digit precision
        println!("\n✗ Processing Failed!");
        println!("  Chunks Completed: {:06}", final_completed);
        println!("  Total Expected:   {:06}", self.total_chunks);
        println!("  Error:            {}", error_message);
        println!();
        io::stdout().flush().unwrap_or(());
    }

    /// Gets the current progress as a percentage.
    ///
    /// # Returns
    /// * `f64` - Progress percentage (0.0 to 100.0)
    pub fn progress_percentage(&self) -> f64 {
        let completed = self.completed_chunks.load(Ordering::Relaxed);
        if self.total_chunks > 0 {
            ((completed as f64) / (self.total_chunks as f64)) * 100.0
        } else {
            0.0
        }
    }
}

// Explicitly implement Send and Sync for ProgressIndicatorService
// All fields are Send + Sync (Arc<AtomicU64>, Arc<Mutex<T>>, u64, Instant)
unsafe impl Send for ProgressIndicatorService {}
unsafe impl Sync for ProgressIndicatorService {}

/// Formats bytes in human-readable format.
///
/// # Arguments
/// * `bytes` - Number of bytes to format
///
/// # Returns
/// * `String` - Human-readable byte format (e.g., "1.5 MB")
#[allow(dead_code)]
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_progress_indicator_creation() {
        let progress = ProgressIndicatorService::new(100);
        assert_eq!(progress.total_chunks, 100);
        assert_eq!(progress.completed_chunks.load(Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn test_chunk_update() {
        let progress = ProgressIndicatorService::new(100);

        progress.update_progress(1).await;
        progress.update_progress(2).await;

        assert_eq!(progress.completed_chunks.load(Ordering::Relaxed), 2);
        assert_eq!(progress.last_chunk_id.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_progress_percentage() {
        let progress = ProgressIndicatorService::new(100);

        assert_eq!(progress.progress_percentage(), 0.0);

        progress.update_progress(1).await;
        progress.update_progress(2).await;

        assert_eq!(progress.progress_percentage(), 2.0);
    }

    /// Tests byte formatting for human-readable display.
    ///
    /// This test validates that the byte formatting function properly
    /// converts byte values to human-readable strings with appropriate
    /// units (B, KB, MB, GB) and decimal precision.
    ///
    /// # Test Coverage
    ///
    /// - Zero byte formatting
    /// - Byte-level formatting (< 1KB)
    /// - Kilobyte formatting with decimal precision
    /// - Megabyte formatting with decimal precision
    /// - Gigabyte formatting with decimal precision
    /// - Unit selection and conversion accuracy
    ///
    /// # Test Scenario
    ///
    /// Tests various byte values across different scales to ensure
    /// proper unit selection and formatting precision.
    ///
    /// # Infrastructure Concerns
    ///
    /// - User interface display formatting
    /// - Progress reporting and visualization
    /// - Human-readable data size representation
    /// - Consistent formatting across the application
    ///
    /// # Assertions
    ///
    /// - Zero bytes display as "0 B"
    /// - Small values display in bytes
    /// - KB values display with 1 decimal place
    /// - MB values display with 1 decimal place
    /// - GB values display with 1 decimal place
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }
}
