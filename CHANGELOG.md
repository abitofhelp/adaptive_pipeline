# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-10-09

### Breaking Changes

This is a major release with significant architectural improvements and critical security fixes. All breaking changes are intentional and improve the codebase's security, maintainability, and design.

#### ProcessingContext API Simplified (Comment 2)

**BREAKING CHANGE**: `ProcessingContext` constructor signature changed from 4 parameters to 2.

- **Removed**: `input_path` and `output_path` fields (file-scoped concerns)
- **Rationale**: `ProcessingContext` is chunk-scoped, not file-scoped. File paths belong at the worker level via dependency injection
- **Migration**:
  ```rust
  // Before (v1.x)
  let context = ProcessingContext::new(
      input_path,
      output_path,
      file_size,
      security_context,
  );

  // After (v2.0)
  let context = ProcessingContext::new(
      file_size,
      security_context,
  );
  ```

#### Encryption Security Fix (Comment 1 - CRITICAL)

**BREAKING CHANGE**: `EncryptionChunkAdapter` now requires explicit configuration and secure key material.

- **Fixed**: CRITICAL vulnerability where encryption adapters created zero-filled keys (NO security)
- **Change**: Adapters now require explicit `EncryptionConfig` and `KeyMaterial` - will NOT generate insecure defaults
- **Migration**:
  ```rust
  // Before (v1.x - INSECURE)
  let adapter = ServiceAdapterFactory::create_encryption_adapter(service);
  // ❌ Created zero-filled keys automatically

  // After (v2.0 - SECURE)
  let encryption_config = EncryptionConfig { /* ... */ };
  let key_material = KeyMaterial { /* SECURE keys */ };
  let adapter = ServiceAdapterFactory::create_encryption_adapter(
      service,
      encryption_config,
      key_material,  // ✅ Caller MUST provide secure keys
  );
  ```

#### Dependency Injection Pattern (Comment 7)

**BREAKING CHANGE**: Adapters now require configuration via constructor instead of hardcoded defaults.

- **Before**: Configuration hardcoded inside `process_chunk()` methods
- **After**: Configuration injected via constructor (Dependency Injection pattern)
- **Benefits**: Better testability, runtime configuration, single source of truth
- **Migration**:
  ```rust
  // Before (v1.x - Hardcoded)
  let adapter = CompressionChunkAdapter::new_compression_adapter(
      service,
      Some("my-adapter".to_string())
  );

  // After (v2.0 - DI)
  let compression_config = CompressionConfig {
      algorithm: CompressionAlgorithm::Brotli,
      level: CompressionLevel::Balanced,
      // ... custom configuration
  };
  let adapter = CompressionChunkAdapter::new_compression_adapter(
      service,
      Some("my-adapter".to_string()),
      compression_config,  // ✅ Config injected explicitly
  );
  ```

### Added

- Typed configuration structs for adapters (Comment 3):
  - `CompressionAdapterConfig` with typed `compression_config` field
  - `EncryptionAdapterConfig` with typed `encryption_config` and `key_material` fields
- Security documentation for encryption adapters
- Migration guides in commit messages
- Transform stages documentation (Tee and Debug stages)
  - Tee stage: Data splitting/inspection for debugging and monitoring
  - Debug stage: Diagnostic monitoring with Prometheus metrics
  - Added to both user guide and developer guide

### Changed

- Refactored `CompressionChunkAdapter` from type alias to dedicated struct (consistency with encryption pattern)
- Refactored `EncryptionChunkAdapter` from type alias to dedicated struct (security requirements)
- Updated all `ProcessingContext::new()` call sites (18 locations across 8 files)
- Moved retry logic from current features to future enhancements section (Comment 9)
- Improved release automation script (`scripts/release.py`)
  - Real-time output streaming for better visibility during builds
  - Manual CHANGELOG workflow with validation
  - GitHub release notes extraction using awk (version-specific notes only)

### Removed

- `input_path()` and `output_path()` methods from `ProcessingContext`
- Unused `async_trait` import from `chunk_processor_adapters.rs` (Comment 4)
- Ability to create encryption adapters without secure key material
- Docker cross-compilation config files (`Cross.toml`, `.cargo/config.toml`) - using default cross behavior

### Security

- **CRITICAL FIX**: Eliminated zero-filled encryption keys vulnerability
- Enforced explicit secure key material for all encryption operations
- Added security validation at adapter construction time

### Documentation

- Updated `ProcessingContext` documentation to clarify chunk-scoped vs file-scoped design
- Added comprehensive security warnings to encryption adapter documentation
- Commented out `requires_security_context` field with TODO for v2.0 security enforcement (Comments 5 & 6)

### Testing

- All 348 library tests pass
- All 63 integration tests pass
- `cargo clippy` passes with no warnings
- No regressions in existing functionality

### Migration Impact

**Call Sites Updated** (18 locations across 8 files):
- `adaptive_pipeline/src/application/services/pipeline.rs` (3 locations)
- `adaptive_pipeline/src/infrastructure/adapters/async_checksum.rs` (1 test)
- `adaptive_pipeline/src/infrastructure/adapters/chunk_processor_adapters.rs` (2 locations)
- `adaptive_pipeline/src/infrastructure/services/*.rs` (4 test helpers)
- `adaptive_pipeline/src/main.rs` (2 restoration contexts)
- `adaptive_pipeline/tests/integration/domain_services_test.rs` (4 test contexts)
- `adaptive_pipeline_domain/src/aggregates/pipeline_aggregate.rs` (2 locations)

### Related Issues

- Addresses code review Comments 1, 2, 3, 4, 7, 9
- See `docs/roadmap.md` for future security enforcement design (Comments 5 & 6)

## [1.0.5] - 2025-10-07

### Added

- Add `scripts/set_versions.sh` script for automated version management
  - Takes version and optional date as arguments
  - Updates all 3 Cargo.toml files (5 version strings total)
  - Updates both documentation introduction files
  - Provides next steps guidance after version update

### Changed

- Update all Cargo.toml version numbers from 1.0.1 to 1.0.5
  - adaptive_pipeline/Cargo.toml (package version + 2 dependency versions)
  - adaptive_pipeline_domain/Cargo.toml
  - adaptive_pipeline_bootstrap/Cargo.toml
- Update documentation version to 1.0.5

### Fixed

- Move migrations directory into adaptive_pipeline crate for crates.io publishing
  - Relocate migrations/ from workspace root to adaptive_pipeline/migrations/
  - Update sqlx::migrate! path from "../migrations" to "./migrations"
  - Fixes cargo publish error where migrations were not included in package

## [1.0.4] - 2025-10-07

### Added

- Add `create-release-zips.sh` script for automated release archive creation
  - Takes version number as argument
  - Creates all 5 platform-specific zip files (macOS ARM64/Intel, Linux ARM64/x86_64, Windows x86_64)
  - Properly named archives with platform and architecture identifiers
  - Ready for GitHub release uploads

### Changed

- Update documentation version to 1.0.4

## [1.0.3] - 2025-10-07

### Changed

- Add prominent links to GitHub Pages documentation books in README.md
- Update documentation version to 1.0.3
- Improve discoverability of User Guide and Developer Guide

## [1.0.2] - 2025-10-07

### Added

- Add copyright headers to all 9 README.md files
- Add cross-navigation links between user guide and developer guide
- Add dual-book deployment to GitHub Pages (user guide at root, developer guide at /developer/)
- Add documentation URL to all Cargo.toml files (https://abitofhelp.github.io/adaptive_pipeline/)

### Changed

- Update documentation version to 1.0.2 with October 7, 2025 publication date
- Improve gitignore pattern for mdBook build directories (add both `book/` and `**/book/`)
- Fix API documentation links to use docs.rs instead of local paths
- Deploy both user guide and developer guide in single unified GitHub Pages site

### Fixed

- Remove mdBook build artifacts from version control (158 HTML/CSS/JS files)
- Fix deploy-docs workflow to build both documentation books

## [1.0.1] - 2025-10-08

### Added

- Add README.md (initial commit)
- Add bootstrap module: Entry point, platform abstraction, and signal handling
- Add bootstrap module: Entry point, platform abstraction, and signal handling
- Add bootstrap module: Entry point, platform abstraction, and signal handling
- Add global resource manager with CLI configuration and concurrency metric
- Add queue depth metrics integration to channel-based pipeline
- Add cross-platform build targets to Makefile
- Add comprehensive cancellation and exit code tests
- Add comprehensive CLI reference and sync version from Cargo.toml
- Add automatic database schema management with sqlx migrations
- docs: add comprehensive compression implementation chapter
- docs: add comprehensive integrity verification implementation chapter
- docs: add advanced topics chapters for concurrency, threading, and resources
- Add comprehensive formal documentation and fix rustdoc warnings
- Add comprehensive custom stages feature documentation and cleanup TODOs
- Add pipeline stage ordering validation (PreBinary before PostBinary)
- Add comprehensive E2E tests for all Application Layer use cases
- Added .gitattributes to improve lang detection at gh.
- Add missing Write trait import for Windows build

### Changed

- Initial commit: Adaptive pipeline processing system
- Fix clippy warnings and improve code quality
- Convert domain to sync, achieve Clean Architecture compliance
- Complete architecture refactoring to 100% - DIP violations fixed
- Change project license from MIT to BSD 3-Clause
- Optimize async concurrency and fix blocking I/O in pipeline services
- Implement Rayon-based parallel processing for CPU-bound operations
- Fix critical memory bug: Replace full-file read with streaming I/O
- Design channel-based pipeline with concurrent random-access writes
- Refactor StreamingBinaryWriter for concurrent random-access writes
- Implement reader_task and cpu_worker_task for channel-based pipeline
- Refactor BinaryFormatWriter::finalize to use &self for Arc compatibility
- Delete .claude directory
- Updated to reflect refactoring.
- Move CLI parsing and validation to bootstrap layer
- Integrate bootstrap CLI layer into main.rs and complete cleanup
- Purging out of date documentation
- Apply automated code formatting and linting across codebase
- Move schema integration example to tests directory
- Set up documentation tooling and infrastructure
- Create comprehensive documentation audit and inventory
- docs: complete documentation audit and create book structures
- docs: create placeholder chapter files with standard headers
- docs: create PlantUML architecture diagrams
- docs: write architecture overview chapter with diagrams
- docs: write fundamentals and repository pattern chapters
- docs: complete fundamentals chapters with comprehensive content
- docs: complete fundamentals and start architecture chapters
- docs: complete fundamentals and architecture sections
- ⏺ I've completed the repository implementation chapter (~967 lines).
- I've completed the binary format implementation chapter (~838 lines).
- Completed observability overview chapter (~808 lines). This comprehensive chapter ties together metrics, logging,   and health monitoring, explaining the three pillars of observability, the ObservabilityService architecture, alert    thresholds, health scoring, usage patterns, Prometheus/Grafana integration, and troubleshooting. Build   successful.
- docs: complete implementation section with 13 comprehensive chapters
- Streamline infrastructure code documentation
- Streamline domain and application layer documentation
- Streamline test documentation and remove excessive comments
- Reorganize tests following Rust best practices and update documentation
- , ready to commit! Here's the commit message:
- Added a feature description for custom stages.
- Implement unified stage service architecture with generic FromParameters trait
- Update StageExecutor to use unified StageService registry pattern
- Rewrite custom stages guide for unified StageService architecture
- refactor: migrate CLI command logic from main.rs to Application Layer use cases
- Move TransactionalChunkWriter to infrastructure layer and implement BinaryFormatWriter trait
- Move TransactionalChunkWriter to infrastructure layer and implement BinaryFormatWriter trait
- Merge branch 'feature/unified-stage-service-with-generics'
- Standardize file naming by removing redundant suffixes across codebase
- Standardize Rust naming conventions across infrastructure layer
- Merge branch 'feature/unified-stage-service-with-generics'
- Complete Rust 2018+ module pattern migration across all layers
- Merge branch 'feature/unified-stage-service-with-generics'
- Merge branch 'feature/unified-stage-service-with-generics'
- Update book for first release - modern Rust patterns and accurate API names
- Deleted symlinked file
- Updated CHANGELOG.md for v1.0.0.
- Bump version to 1.0.0 for first production release
- Bump version to 1.0.1
- Release notes for v1.0.1
- Removed "work in progress" message in README.md.
- Prepare workspace for v1.0.1 release with crates.io publishing
- Update .gitattributes for renamed directory structure
- Trigger linguist reindex
- All "too many arguments" warnings resolved using consistent context struct pattern!
- Prepare v1.0.1 release with GPT-5 audit recommendations
- Clean up clippy warnings and add license headers for v1.0.1
- Clean up clippy warnings, add license headers, and configure cross-compilation
- Trigger CI workflows with updated configuration
- CHANGELOG.md

### Fixed

- Resolve test compilation errors and update documentation
- Fix code examples and technical accuracy in documentation
- Fix code examples and technical accuracy in documentation
- fix: replace BufferedBinaryWriter with StreamingBinaryWriter for concurrent processing
- Extract encryption nonce from encrypted data instead of hardcoding zeros
- Rewrite e2e binary format tests with real components and fix isolation
- Updated module removing files moved.
- Fix high-priority test issues and clean up obsolete test code
- Resolve compilation errors in unix platform implementation
- Resolve e2e test failures and import path inconsistencies
- Protect C string literals from rustfmt and enable locked builds
- Configure clippy linting for CI and development workflows
- Make rustfmt configuration stable-compatible for CI
- Use $HOME instead of ~ in GitHub Actions mdbook installation
- Resolve CI failures - doctests, formatting, and workflow improvements

### Removed

- Removed release-plz; using git-cliff