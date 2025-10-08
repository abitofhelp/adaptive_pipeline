<!--
Adaptive Pipeline
Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
SPDX-License-Identifier: BSD-3-Clause
See LICENSE file in the project root.
-->

# adaptive-pipeline-bootstrap

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![crates.io](https://img.shields.io/crates/v/adaptive-pipeline-bootstrap.svg)](https://crates.io/crates/adaptive-pipeline-bootstrap)
[![Documentation](https://docs.rs/adaptive-pipeline-bootstrap/badge.svg)](https://docs.rs/adaptive-pipeline-bootstrap)

**Bootstrap and platform abstraction layer for the Adaptive Pipeline** - Handles application entry points, dependency injection, signal handling, and cross-platform operations.

## 🎯 Overview

This crate sits **outside the enterprise application layers** and provides the foundational infrastructure needed to bootstrap and run Rust applications with:

- **Platform Abstraction** - Unified API for Unix and Windows
- **Signal Handling** - Graceful shutdown for SIGTERM, SIGINT, SIGHUP
- **CLI Parsing** - Secure argument validation with clap
- **Dependency Injection** - Composition root for wiring services
- **Shutdown Coordination** - CancellationToken-based graceful teardown
- **Exit Code Mapping** - Unix sysexits.h standard codes

### Design Philosophy

- **🚪 Entry Point** - Application lifecycle management
- **🌐 Cross-Platform** - Write once, run on Unix/Windows
- **🔒 Security** - Input validation, path traversal prevention
- **♻️ Reusable** - Can bootstrap any Rust CLI application
- **🧪 Testable** - Trait-based with mock implementations

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
adaptive-pipeline-bootstrap = "1.0"
```

## 🏗️ Architecture Position

Bootstrap is the outermost layer that initializes the application:

```
┌───────────────────────────────────────────┐
│    BOOTSTRAP (This Crate)                 │
│  - Entry Point                            │
│  - DI Container (Composition Root)        │
│  - Platform Abstraction                   │
│  - Signal Handling                        │
│  - Secure Arg Parsing                     │
└───────────────────────────────────────────┘
                   │
                   ▼
┌───────────────────────────────────────────┐
│         APPLICATION LAYER                 │
│  - Use Cases                              │
│  - Application Services                   │
└───────────────────────────────────────────┘
                   │
                   ▼
┌───────────────────────────────────────────┐
│           DOMAIN LAYER                    │
│  - Business Logic                         │
│  - Domain Services                        │
│  - Entities & Value Objects               │
└───────────────────────────────────────────┘
                   ▲
                   │
┌───────────────────────────────────────────┐
│       INFRASTRUCTURE LAYER                │
│  - Adapters                               │
│  - Repositories                           │
│  - External Services                      │
└───────────────────────────────────────────┘
```

## 📚 Usage Examples

### Basic CLI Bootstrapping

```rust
use bootstrap::{bootstrap_cli, result_to_exit_code};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    // Parse and validate CLI arguments
    let cli = match bootstrap::bootstrap_cli() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("CLI Error: {}", e);
            return std::process::ExitCode::from(65); // EX_DATAERR
        }
    };

    // Run application with validated config
    let result = run_application(cli).await;

    // Map result to Unix exit code
    result_to_exit_code(result)
}

async fn run_application(cli: bootstrap::ValidatedCli) -> Result<(), String> {
    println!("Running with config: {:?}", cli);
    Ok(())
}
```

### Platform Abstraction

```rust
use bootstrap::platform::create_platform;

// Get platform-specific implementation
let platform = create_platform();

// Cross-platform API
println!("Platform: {}", platform.platform_name());
println!("CPU cores: {}", platform.cpu_count());
println!("Page size: {} bytes", platform.page_size());

// Memory information
let total = platform.total_memory()?;
let available = platform.available_memory()?;
println!("Memory: {}/{} GB", available / 1_000_000_000, total / 1_000_000_000);

// Platform-specific constants
println!("Line separator: {:?}", platform.line_separator());
println!("Path separator: {:?}", platform.path_separator());
```

### Signal Handling

```rust
use bootstrap::signals::create_signal_handler;
use bootstrap::shutdown::ShutdownCoordinator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up graceful shutdown
    let coordinator = ShutdownCoordinator::new();
    let signal_handler = create_signal_handler(coordinator.token());

    // Start signal monitoring
    tokio::spawn(async move {
        signal_handler.wait_for_signal().await;
        println!("Received shutdown signal - cleaning up...");
    });

    // Run application with cancellation
    run_with_cancellation(coordinator.token()).await?;

    // Wait for graceful shutdown (with timeout)
    coordinator.wait_for_completion(30).await?;

    Ok(())
}
```

### CLI Validation

```rust
use bootstrap::cli::{parse_and_validate, ValidatedCli};

// Parse with automatic validation
let cli = parse_and_validate()?;

// All inputs are validated:
// - No SQL injection patterns
// - No path traversal (../)
// - No dangerous shell characters
// - Numbers within allowed ranges
// - Paths are sanitized

match &cli.command {
    ValidatedCommand::Process { input, output, .. } => {
        // Paths are already validated and safe
        println!("Processing {} -> {}", input.display(), output.display());
    }
    _ => {}
}
```

### Exit Code Mapping

```rust
use bootstrap::exit_code::{map_error_to_exit_code, ExitCode};

fn main() -> std::process::ExitCode {
    let result = dangerous_operation();

    match result {
        Ok(_) => ExitCode::Success.into(),
        Err(e) => {
            // Automatic mapping to Unix exit codes
            let exit_code = map_error_to_exit_code(&e);
            eprintln!("Error: {}", e);
            exit_code.into()
        }
    }
}

// Maps error messages to sysexits.h codes:
// - "file not found" -> EX_NOINPUT (66)
// - "invalid data" -> EX_DATAERR (65)
// - "I/O error" -> EX_IOERR (74)
```

## 🔧 Module Overview

### Platform Abstraction (`platform`)

Provides unified cross-platform API:

```rust
pub trait Platform: Send + Sync {
    // System Information
    fn page_size(&self) -> usize;
    fn cpu_count(&self) -> usize;
    fn total_memory(&self) -> Result<u64, PlatformError>;
    fn available_memory(&self) -> Result<u64, PlatformError>;

    // Platform Constants
    fn line_separator(&self) -> &'static str;
    fn path_separator(&self) -> char;
    fn platform_name(&self) -> &'static str;
    fn temp_dir(&self) -> PathBuf;

    // Security & Permissions
    fn is_elevated(&self) -> bool;
    fn set_permissions(&self, path: &Path, mode: u32) -> Result<(), PlatformError>;
    fn is_executable(&self, path: &Path) -> bool;

    // File Operations
    async fn sync_file(&self, file: &tokio::fs::File) -> Result<(), PlatformError>;
}
```

**Implementations:**
- `UnixPlatform` - Uses libc, /proc, /sys
- `WindowsPlatform` - Uses winapi, with stubs for non-Windows

### Signal Handling (`signals`)

Cross-platform graceful shutdown:

```rust
pub trait SignalHandler: Send + Sync {
    async fn wait_for_signal(&self);
    fn shutdown_requested(&self) -> bool;
}

// Unix: SIGTERM, SIGINT, SIGHUP
// Windows: Ctrl+C, Ctrl+Break
let handler = create_signal_handler(cancellation_token);
```

### CLI Parsing (`cli`)

Secure argument validation with clap:

```rust
pub struct ValidatedCli {
    pub command: ValidatedCommand,
    pub verbose: bool,
    pub config: Option<PathBuf>,
    pub cpu_threads: Option<usize>,
    pub io_threads: Option<usize>,
    pub storage_type: Option<String>,
    pub channel_depth: usize,
}

// Security validations:
// - SQL injection prevention
// - Path traversal checks
// - Shell command injection blocking
// - Numeric range validation
```

### Shutdown Coordination (`shutdown`)

CancellationToken-based coordination:

```rust
pub struct ShutdownCoordinator {
    token: CancellationToken,
    completed: Arc<AtomicBool>,
}

impl ShutdownCoordinator {
    pub fn new() -> Self;
    pub fn token(&self) -> CancellationToken;
    pub fn initiate(&self);
    pub async fn wait_for_completion(&self, timeout_secs: u64) -> Result<()>;
    pub fn mark_complete(&self);
}
```

### Exit Codes (`exit_code`)

Unix sysexits.h standard codes:

```rust
pub enum ExitCode {
    Success = 0,
    Error = 1,
    DataErr = 65,      // Invalid data
    NoInput = 66,      // File not found
    Software = 70,     // Internal error
    IoErr = 74,        // I/O error
}

// Automatic error message mapping
let code = map_error_to_exit_code("file not found");
assert_eq!(code, ExitCode::NoInput);
```

## 🎯 Key Features

### Cross-Platform Compatibility

**Compile-time platform selection:**
```rust
#[cfg(unix)]
pub use unix::UnixPlatform;

#[cfg(windows)]
pub use windows::WindowsPlatform;
```

**Runtime platform detection:**
```rust
let platform = create_platform();
if platform.platform_name() == "macos" {
    // macOS-specific logic
}
```

### Security Validations

All CLI inputs are validated for security:

```rust
// ❌ Rejected patterns:
// - "../../../etc/passwd"     (path traversal)
// - "'; DROP TABLE users; --" (SQL injection)
// - "$(rm -rf /)"            (shell injection)
// - "\x00\x01\x02"           (binary data)

// ✅ Accepted inputs:
// - "/valid/path/to/file.txt"
// - "my-pipeline-name"
// - "8"  (numeric validation)
```

### Graceful Shutdown

**Multi-stage shutdown:**
```
1. Signal received (SIGTERM/SIGINT)
2. CancellationToken triggered
3. Tasks check token and cleanup
4. Coordinator waits for completion
5. Timeout enforcement (default: 30s)
6. Process exits cleanly
```

## 🧪 Testing

Bootstrap includes testable abstractions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = create_platform();
        assert!(platform.cpu_count() >= 1);
        assert!(platform.page_size() >= 512);
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let coordinator = ShutdownCoordinator::new();
        let token = coordinator.token();

        coordinator.initiate();
        assert!(token.is_cancelled());
    }
}
```

## 📊 Dependencies

Minimal dependencies for cross-platform support:

- **tokio** - Async runtime
- **async-trait** - Async trait support
- **thiserror / anyhow** - Error handling
- **clap** - CLI argument parsing
- **tracing** - Structured logging

**Platform-specific:**
- Unix: `libc`
- Windows: `winapi`

## 🔗 Related Crates

- **[adaptive-pipeline](../adaptive-pipeline)** - Application layer and CLI
- **[adaptive-pipeline-domain](../adaptive-pipeline-domain)** - Pure business logic

## 📄 License

BSD 3-Clause License - see [LICENSE](../LICENSE) file for details.

## 🤝 Contributing

Contributions should focus on:
- ✅ Cross-platform compatibility
- ✅ Security hardening
- ✅ Signal handling improvements
- ✅ Platform abstraction enhancements
- ❌ Not business logic (belongs in domain)
- ❌ Not application logic (belongs in application layer)

---

**Cross-Platform Foundation | Secure by Default | Production-Ready**
