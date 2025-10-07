# Rust Coding Conventions for Adaptive Pipeline

This document captures Rust-specific coding conventions and architectural decisions for this project.

## ⚠️ Strict Rules (Non-Negotiable)

These rules must be followed in all new code and existing code should be migrated:

### 1. **No `mod.rs` Files** - Use Rust 2018+ Module Pattern

**NEVER use `mod.rs` files.** Always use the Rust 2018+ pattern with named module files:

✅ **REQUIRED**:
```
src/
  application/
    services.rs       ← Module declaration & re-exports
    services/
      pipeline.rs
      file_processor.rs
```

❌ **FORBIDDEN**:
```
src/
  application/
    services/
      mod.rs          ← DO NOT USE
      pipeline.rs
```

**Why this is mandatory:**
- Editor tabs show `services.rs` instead of confusing multiple `mod.rs` tabs
- Clearer hierarchy - module interface visible at parent level
- Better IDE navigation and searchability
- Official Rust recommendation since 2018 edition
- Prevents confusion when multiple `mod.rs` files are open

### 2. **No `unwrap()`, `expect()`, or `panic!()` in Production Code**

Production code (library code in `src/`) must not use:
- `.unwrap()`
- `.expect()`
- `panic!()`
- `unreachable!()`

Test code and examples may use these for test assertions.

### 3. **Error Propagation with `?` Operator**

Always use `?` operator for error propagation. Handle errors gracefully with proper error types.

## File and Module Naming

### File Naming Convention

**Drop redundant suffixes from filenames** - rely on the folder structure to provide context:

- ✅ `services/pipeline.rs` (not `pipeline_service.rs`)
- ✅ `repositories/pipeline.rs` (not `pipeline_repository.rs`)
- ✅ `errors/pipeline.rs` (not `pipeline_error.rs`)
- ✅ `entities/pipeline.rs` (not `pipeline_entity.rs`)

**Rationale**: The import path already provides full context:
```rust
use pipeline::application::services::PipelineService;
use pipeline::infrastructure::repositories::PipelineRepository;
```

### Type Naming Convention

**Keep meaningful suffixes on type names** for clarity and disambiguation:

- ✅ `struct PipelineService` - distinguishes from `Pipeline` entity
- ✅ `enum PipelineError` - immediately clear it's an error type
- ✅ `struct PipelineRepository` - signals it's a repository
- ✅ `struct Pipeline` - entity (no suffix needed, no ambiguity)
- ✅ `struct FileHeader` - value object (already clear)

**Rationale**: While the import path helps you find/import the right thing, the suffix helps readers understand what they're looking at in code:

```rust
// Clear at call site
return Err(PipelineError::invalid_config());
let service = PipelineService::new(...);
let entity = Pipeline::new(...);
```

**When to use suffixes:**
1. **Prevents conflicts**: `PipelineService` vs `Pipeline` entity
2. **Adds semantic clarity**: `PipelineError` immediately identifies error type
3. **Follows domain conventions**: `FileProcessorService` signals service layer

**When NOT to use suffixes:**
- Entities: `Pipeline` not `PipelineEntity`
- Value Objects: `FileHeader` not `FileHeaderValue`
- When there's no ambiguity or semantic benefit

## Import Organization

**Standard import order:**
```rust
// 1. External crates
use async_trait::async_trait;
use tokio::sync::Mutex;

// 2. Internal crate modules
use pipeline_domain::entities::Pipeline;
use pipeline_domain::PipelineError;

// 3. Standard library
use std::collections::HashSet;
use std::path::PathBuf;
```

**Use aliases to resolve conflicts:**
```rust
use pipeline_domain::entities::Pipeline;
use pipeline::application::services::PipelineService; // No alias needed - clear names

// Or when needed:
use pipeline_domain::entities::Pipeline as PipelineEntity;
```

## Architectural Layer Organization

Files under `pipeline/src/` and `pipeline_domain/src/` follow these conventions:

- `domain/entities/` - Core business entities (e.g., `Pipeline`)
- `domain/value_objects/` - Immutable value types (e.g., `FileHeader`)
- `domain/errors/` - Domain error types (e.g., `PipelineError`)
- `application/services/` - Application orchestration services
- `application/use_cases/` - Use case implementations
- `infrastructure/services/` - Infrastructure implementations
- `infrastructure/repositories/` - Data persistence layer

**Note**: These conventions apply to library code only, not to `/examples`, `/tools`, or `/tests` directories.

## Summary

**One Rule**: Filename matches the domain concept, folder indicates the layer, type name includes suffix when beneficial for clarity and disambiguation.

This creates a clean, predictable codebase structure regardless of architectural layer.
