# Sections from Claude_Rust.md not covered by optimized language rules

## <preamble>

_(No additional content)_

## Claude-Rust.md - Code Generation and Maintenance Criteria (Hybrid DDD/Clean/Hex)

> Transformed from the Kotlin rules to Rust idioms while preserving the original intent and structure.

## 1. Architecture and Design Patterns

- **Architecture**: Follow hybrid DDD/Clean/Hexagonal architecture with strict layer boundaries
- **Dependency Management**: Apply Dependency Inversion Principle (DIP) - abstractions over implementations
- **Architectural Layer Dependencies**: Each layer can only depend on layers below it:
  - **Domain Layer** (innermost/core):
    - NO dependencies on any other layers (must be self-sufficient)
    - Only depends on Kotlin standard libraries
    - Defines ports (interfaces) but contains no implementations
    - Pure business logic and domain models only
  - **Application Layer**:
    - Can depend on: Domain layer only
    - Cannot depend on: Infrastructure or Presentation layers
    - Orchestrates domain objects and use cases
    - Defines additional ports for external services
  - **Infrastructure Layer**:
    - Can depend on: Domain and Application layers
    - Cannot depend on: Presentation layer
- **Repository Pattern**: Traits in Domain/Application; impls in Infrastructure.
- **Object Creation**: Builder pattern and smart constructors for invariants.
- **Domain Concepts**: Favor **newtypes** for value objects, **entities** with identity, **aggregates** for invariants, and **domain events** for changes.
- **Result Pattern**: Idiomatic `Result<T, E>` with custom error enums.

## 2. Code Generation Philosophy

- Must compile with `cargo build --workspace` and pass `cargo test --workspace`.
- Keep public APIs small; leverage privacy by module boundaries.
- Prefer explicit lifetimes only when necessary; avoid `'static` where inappropriate.

## 3. Rust Language Practices

- **Newtypes & Validation**:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleName(String);

impl TryFrom<String> for RuleName {
    type Error = RuleError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        if v.trim().is_empty() { return Err(RuleError::InvalidName); }
        Ok(Self(v))
    }
}
```
- **Enums / Algebraic Data Types** for closed sets.
- **Traits**: Define minimal required behavior; prefer trait objects only at boundaries.
- **Ownership**: Prefer borrowing (`&T`) over cloning; use `Arc<...>` for shared ownership across threads.

## 4. Error Handling

- **Custom Errors** with `thiserror`:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrammarError {
    #[error("parse error at {line}:{column}: {message}")]
    Parse { line: usize, column: usize, message: String },
    #[error("validation error in rule `{rule}`: {reason}")]
    Validation { rule: String, reason: String },
    #[error("generation error targeting `{target}`: {cause}")]
    Generation { target: String, cause: String },
}
```
- **Anyhow at Boundaries**: Use `anyhow::Result` in CLI/adapters; convert to/from domain errors.
- **No panics** across boundaries; use `?` for propagation; accumulate errors with custom types where needed.

## 6. Coding Standards

- **Naming**: Types/Traits `PascalCase`, functions/methods `snake_case`, constants `SCREAMING_SNAKE_CASE`, modules `snake_case`.
- **Module Organization**: One type per file when complex; otherwise cohesive modules.
- **Magic Values**: Replace with `const` or enums.

```rust
pub const MAX_RULE_ALTERNATIVES: usize = 100;
pub const DEFAULT_TIMEOUT_MS: u64 = 5_000;
```

## 8. Build & Project Management

- **Cargo Workspaces** map cleanly to layers:

```
project/
├── Cargo.toml           # [workspace]
├── domain/              # crate: domain (pure)
│   └── src/
├── application/         # crate: application (use cases, ports)
│   └── src/
├── infrastructure/      # crate: infrastructure (adapters)
│   └── src/
└── presentation/        # crate: presentation (cli/http)
    └── src/
```

- **Dependencies**: Minimal in `domain`; broader in `infrastructure` (tokio, reqwest, sqlx, etc.).
- **Features**: Gate optional integrations behind Cargo features.

## 9. Documentation

- **rustdoc**: `///` for items; crate/module docs in `lib.rs` with `//!`.
- **ADRs**: `docs/adr/` with rationale and consequences.
- **Examples**: `examples/` runnable docs; doctests kept compiling.

## 10. Performance

- **Benchmarks**: `criterion` under `benches/`.
- **Profiling**: `cargo flamegraph`, `pprof-rs`.
- **Allocations**: Prefer `String::with_capacity`, iterators over temporary `Vec`s.

## Project Scaffolding (Example)

```text
domain/src/lib.rs            // RuleName, GrammarRule, events, errors, traits
application/src/lib.rs       // usecases, ports, DTOs, mappers
infrastructure/src/lib.rs    // adapters: fs, http, db impls
presentation/src/main.rs     // CLI or HTTP server wiring
```

## Development Workflow

1. **Safety**: `git status` clean; `cargo fmt --all -- --check`.
2. **Linting**: `cargo clippy --workspace -- -D warnings`.
3. **Tests**: `cargo test --workspace`; property tests via `proptest`.
4. **No Quick Fixes**: Address root causes; keep invariants at boundaries.
5. **Generated Code**: Re-generate and review diffs before commit.
6. **Cross-Review**: Ask GPT-5/Claude before large merges.
7. **Configuration Files**: **Never** mutate user config; test with temp copies and clean up.

## Markdown Documentation

All markdown files under must include this header:

```markdown

## Title

**Version:** 1.0.0
**Date:** January 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Released

Description paragraph...
```

Perfect—here are the **final, conflict-resolved, paste-ready additions** for `claude_rust.md` based on your decisions. I’ve folded in concrete code snippets and enforcement guidance so Claude (and CI) can apply this verbatim.

---

## Additions to `claude_rust.md` (Finalized)

_(No additional content)_

## 7) Domain invariants (no panics)

**Rules**

* Domain code must not `panic!` on invalid input/state. Return a **domain error**.
* `debug_assert!` is allowed for dev checks that compile out in release. Never rely on it for logic.

```rust
pub fn set_percentage(p: u8) -> AppResult<Self> {
    debug_assert!(p <= 100, "caller bug: percentage > 100");
    if p > 100 {
        return Err(AppError::Validation(format!("percentage out of range: {p}")));
    }
    Ok(Self(p))
}
```

---

## 8) Adapter & DB rules

**Rules**

* Adapters never panic; map driver errors to `AppError`.
* Multi-statement operations use **transactions**; convert constraint/uniqueness errors to specific messages.
* Use `spawn_blocking` or an async driver for heavy queries.

---

## 13) Optional: panic containment at boundaries

Where panics are still possible (unsafe/FFI/third-party), you may guard worker entry points:

```rust
use std::panic::{catch_unwind, AssertUnwindSafe};
let res = catch_unwind(AssertUnwindSafe(|| do_risky_work()))
    .map_err(|_| AppError::Processing("panic in worker".into()))?;
```

---

## Notes on prior conflicts (now resolved)

* **`anyhow`** is **CLI-only**; app/infra use `AppError`.
* **Panics** for invariants are replaced by **typed errors** with optional **`debug_assert!`** for dev.
* **Clippy**: crate-root `deny` for panic/unwrap family + CI `-D warnings`.
* **Tests** may use `unwrap`; production cannot.
* **Cancellation** standardized on `tokio_util::CancellationToken`.
* **Join errors** bubble as `AppError::Processing(...)`; fail the run unless a documented retry policy applies.

---

