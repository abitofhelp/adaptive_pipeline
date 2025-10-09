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

### Changed

- Refactored `CompressionChunkAdapter` from type alias to dedicated struct (consistency with encryption pattern)
- Refactored `EncryptionChunkAdapter` from type alias to dedicated struct (security requirements)
- Updated all `ProcessingContext::new()` call sites (18 locations across 8 files)
- Moved retry logic from current features to future enhancements section (Comment 9)

### Removed

- `input_path()` and `output_path()` methods from `ProcessingContext`
- Unused `async_trait` import from `chunk_processor_adapters.rs` (Comment 4)
- Ability to create encryption adapters without secure key material

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
