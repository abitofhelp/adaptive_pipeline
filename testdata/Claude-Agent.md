# CLAUDE.md — Rust Language Rules (Merged & Optimized)

> **Purpose**: Organization-wide, project-agnostic standards for Rust. This guide defines language best practices: error handling, concurrency & cancellation, observability, testing, lint/format policy, headers/SPDX, and workspace hygiene. **Architecture (DDD/Clean/Hex, ports/adapters, layering) lives in a separate Architecture Standards doc/agent.**

---

## 0) Scope & Version

* Applies to all Rust repos in the org unless explicitly opted out.
* Toolchain: **MSRV = 1.74+**, Edition **2021** (or newer if specified by a repo).
* Architectural rules are out of scope here.

---

## 1) Required File Headers & SPDX

**Policy**

* All source code and scripts—including **tests**, **benches**, and **examples**—must carry an SPDX license header and copyright.
* Prefer short SPDX headers over long boilerplate.

**Templates**

**Rust (`.rs`)**

```rust
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc.
// Module: <crate::path::module>
// Description: <one-line purpose of this file>.
```

**Crate/module docs (optional)**

```rust
//! SPDX-License-Identifier: BSD-3-Clause
//! # <Crate Name>
//! <short crate description used by rustdoc>
```

**TOML / YAML / JSON / Markdown**

```toml
# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc.
```

```markdown
<!-- SPDX-License-Identifier: BSD-3-Clause -->
<!-- Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc. -->
```

**Python (.py)**

```python
# SPDX-License-Identifier: BSD-3-Clause
# Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc.
# Module: <package.module>
# Description: <one-line purpose of this file>.
```

toml

# SPDX-License-Identifier: BSD-3-Clause

# Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc.

````
```markdown
<!-- SPDX-License-Identifier: BSD-3-Clause -->
<!-- Copyright (c) 20XX Michael Gardner, A Bit of Help, Inc. -->
````

---

## 2) Error Handling Policy (Zero-Panic in Production)

**Principles**

* Rust has no runtime exceptions; failures are values. In production code: **no** `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`.
* Catch failures locally and return typed results: `Result<T, AppError>` (alias allowed as `AppResult<T>`).
* **anyhow/eyre scope:** **CLI boundary only** for ergonomic display. Libraries/app/infra return typed errors.
* Invariants: return typed errors for invalid input/state. `debug_assert!` allowed only for developer checks.

**Error Type**

```rust
pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    Validation(String),
    #[error("Database error: {0}")]
    Db(String),
    #[error("Cancelled")]
    Cancelled,
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    #[error("Processing failed: {0}")]
    Processing(String),
}
```

**Before → After**

```rust
// ❌
let n = s.parse::<u32>().unwrap();
// ✅
let n = s.parse::<u32>()
    .map_err(|e| AppError::Validation(format!("parse int failed for '{s}': {e}")))?;
```

---

## 3) Concurrency, Supervision & Cancellation

**Never spawn-and-forget**

* Every `tokio::spawn` must:

  1. return `AppResult<T>`,
  2. have its `JoinHandle` retained, and
  3. be awaited, mapping `JoinError` → typed error.

**Cancellation**

* Standard token: **`tokio_util::sync::CancellationToken`**.
* Long-lived tasks/functions accept a token and check it at suspension points using `tokio::select!`.
* Resource acquisition (semaphores/permits) must be cancellable.

**Blocking work**

* Use `tokio::task::spawn_blocking` for heavy CPU or sync IO/DB. Do **not** block the async scheduler.

**Optional panic containment**

* At worker/FFI/process boundaries, wrapping entry with `catch_unwind` to convert panics → `AppError::Processing("panic: …")` is acceptable. Apply surgically.

**Helpers**

```rust
use tokio::task::JoinHandle;

pub fn spawn_supervised<F, T>(name: &'static str, fut: F) -> JoinHandle<AppResult<T>>
where
    F: std::future::Future<Output = AppResult<T>> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(async move {
        let res = fut.await;
        match &res {
            Ok(_) => tracing::debug!(task = name, "task completed"),
            Err(e) => tracing::error!(task = name, error = ?e, "task failed"),
        }
        res
    })
}

pub async fn join_supervised<T>(h: JoinHandle<AppResult<T>>) -> AppResult<T> {
    h.await
        .map_err(|e| AppError::Processing(format!("task join failed: {e}")))?
}
```

**Token-aware loop**

```rust
use tokio_util::sync::CancellationToken;

async fn run_loop(mut rx: tokio::sync::mpsc::Receiver<Job>, token: CancellationToken) -> AppResult<()> {
    loop {
        tokio::select! {
            _ = token.cancelled() => break,
            msg = rx.recv() => match msg {
                Some(job) => handle(job, token.clone()).await?,
                None => break,
            }
        }
    }
    Ok(())
}
```

---

## 4) Structured Concurrency Extras

* For N-way fan-out where all results must succeed: use `FuturesUnordered` and **fail fast** on first error; cancel remaining via token.
* For small fixed joins: prefer `tokio::try_join!`.

---

## 5) Channels, Backpressure & Resource Governance

**Channels**

* Use **bounded** channels; sizes are config-driven. Treat full queues explicitly (backoff, drop policy, or error). Avoid unbounded channels in production.
* Avoid sharing one `Receiver` behind a `Mutex` across many workers. Prefer fan-out queues or single-consumer dispatcher with bounded task queue.

**Cancelable send**

```rust
tokio::select! {
    _ = token.cancelled() => return Err(AppError::Cancelled),
    r = tx.send(msg) => r.map_err(|e| AppError::Processing(format!("send failed: {e}")))?,
}
```

**Resources**

* Govern concurrency with semaphores/pools. Acquisition respects cancellation via `tokio::select!`.

---

## 6) Boundary Layers (CLI & Services)

**CLI/Main**

* `main` delegates to a `run()` that returns `AppResult<()>`.
* Map `AppError` to **deterministic exit codes** (adjust per repo):

  * `Io` → 2, `Validation` → 3, `Db` → 4, `ResourceExhausted` → 5, `Cancelled` → 130, other → 1.
* Tracing subscriber setup must not `unwrap`; on failure, print and exit code 1.
* `std::process::exit` allowed **only** at the single exit site in `main` (locally allow the lint there).

**Services/Daemons**

* Install SIGINT/SIGTERM handlers that cancel the shared token. Await supervised tasks with a **grace period**; log stragglers.

---

## 7) Observability (Tracing) & Logging

* Use `tracing` with structured fields; include error chains where helpful.
* Emit task lifecycle logs in supervision helpers (start/stop/fail), not in business logic.
* Return errors to callers; do not log-and-suppress fatal failures.

---

## 8) Testing Requirements (Concurrency/Cancellation)

**Must-haves**

* **Cancellation propagation**: start long-running work; cancel token; assert timely shutdown and JoinHandles resolution.
* **Permit release**: induce error during work and verify resource permits are released.
* **Exit code mapping**: tests for `AppError` → exit code table.
* **Deadlock guards**: wrap critical async tests in `tokio::time::timeout`.
* **Signal simulation**: unit-test the cancellation trigger (avoid OS signals in unit tests).

**Ergonomics**

* `unwrap`/`expect` allowed in tests. Production forbids them.
* Use `proptest` for invariants and parsers; add `criterion` benches for hot paths; enable profiling (`flamegraph`, `pprof-rs`) when optimizing.

---

## 9) Linting, Formatting & CI Gates

**Clippy – CI**

```bash
cargo clippy --all --all-features -- -D warnings \
  -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::todo -D clippy::unimplemented \
  -D clippy::await_holding_lock
```

**Crate-root attributes (prod crates)**

```rust
#![cfg_attr(not(test), deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::await_holding_lock,
))]
```

**clippy.toml** (org defaults)

```toml
disallowed-methods = [
  "std::option::Option::unwrap",
  "std::result::Result::unwrap",
  "std::option::Option::expect",
  "std::result::Result::expect",
  "std::process::exit",         # locally allow only at CLI exit site
]

disallowed-macros = [
  "panic",
  "todo",
  "unimplemented",
]
```

**rustfmt**

```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Max"
newline_style = "Unix"
hard_tabs = false
reorder_imports = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
format_macro_bodies = true
wrap_comments = true
comment_width = 100
use_field_init_shorthand = true
```

**Headers check (CI)** – quick guard

```bash
# scripts/check_spdx.sh
set -euo pipefail
# Cover Rust sources, config, docs, and common scripts. Extend as needed per repo.
FILES=$(git ls-files \
  '*.rs' '*.toml' '*.md' '*.yml' '*.yaml' '*.json' '*.sh' '*.py' \
  | grep -vE '(^|/)target/')
MISSING=0
for f in $FILES; do
  if ! head -n 5 "$f" | grep -Eq 'SPDX-License-Identifier:'; then
    echo "SPDX header missing: $f"; MISSING=1
  fi
done
exit $MISSING
```

---

## 10) Workspace & Cargo Hygiene

* Maintain `rust-version` (MSRV) in each crate’s `Cargo.toml`.
* Fill package metadata: `license`, `repository`, `readme`, `keywords`, `categories`, `description`.
* Prefer `workspace = true` dependencies for shared versions; avoid duplicate-version drift.
* Gate optional integrations behind features.

**Cargo.toml exemplar**

```toml
[package]
name = "…"
version = "…"
edition = "2021"
rust-version = "1.74"
license = "BSD-3-Clause"
repository = "https://github.com/…"
readme = "README.md"
keywords = ["…"]
categories = ["…"]
description = "One-line, rustdoc-ready description."
```

---

## 11) Module & Naming Conventions

* Types/Traits = `PascalCase`; functions/methods = `snake_case`; consts = `SCREAMING_SNAKE_CASE`; modules = `snake_case`.
* Prefer one complex type per file; otherwise keep cohesive modules.
* Replace magic values with `const` or enums.
* Prefer borrowing over cloning; use `Arc<T>` only for cross-task/thread sharing.
* Prefer validated newtypes (`TryFrom`) for value objects.

---

## 12) TTY / Console Output Safety

* Decouple user-facing output from critical tasks.
* Buffer writes and flush on newline or interval.
* On shutdown, drain pending messages; cancellation must not be blocked by TTY stalls.

**Patterns**

* Threaded writer: `std::sync::mpsc` + `BufWriter`.
* Async writer: `tokio::sync::mpsc` + `tokio::io::BufWriter` + periodic flush.

---

## 13) Developer Checklists

**PR Checklist**

* [ ] No banned calls in prod (`unwrap/expect/panic/todo/unimplemented`).
* [ ] All spawned tasks supervised & awaited; `JoinError` mapped.
* [ ] Long-lived fns accept `CancellationToken` and use `tokio::select!`.
* [ ] Bounded channels; send/recv are cancel-aware; no scheduler blocking.
* [ ] CLI/main: single exit path; deterministic codes; safe tracing init.
* [ ] Tests: cancellation, exit codes, resource release, timeouts.
* [ ] CI: clippy/rustfmt/tests pass; SPDX header check passes.
* [ ] Cargo metadata/MSRV set; feature flags documented.

**Definition of Done**

* [ ] Conforms to this CLAUDE.md.
* [ ] Observability on error paths (structured `tracing`).
* [ ] Config documented for bounds/timeouts/parallelism.

---

## 14) Appendix: Markdown Front-Matter (Docs)

Recommended at the top of top-level docs:

```
Version: x.y.z
Date: <Month YYYY>
SPDX-License-Identifier: BSD-3-Clause
License File: See LICENSE in repo root
Copyright: © 20XX Michael Gardner, A Bit of Help, Inc.
Authors: <names>
Status: Draft/Released
```

---

**End of language-focused Rust rules.**
