# Optimized Adaptive Pipeline - Development Roadmap

**Last Updated**: 2025-10-06
**Version**: 0.1.0

This document tracks medium and low priority enhancements and technical debt items identified during development. Items are organized by category and priority.

---

## 📋 Table of Contents

- [Configuration & CLI](#configuration--cli)
- [Security Enhancements](#security-enhancements)
- [Feature Completions](#feature-completions)
- [Architecture Improvements](#architecture-improvements)
- [Performance Optimizations](#performance-optimizations)

---

## ⚙️ Configuration & CLI

### Medium Priority

#### 1. Configuration File Loading
**Location**: `pipeline/src/main.rs:376`
**Status**: Not Implemented
**Description**: Implement configuration file loading mechanism

**Current State**:
```rust
// TODO: Load configuration
```

**Requirements**:
- Support TOML configuration files
- Environment variable overrides
- Default configuration fallback
- Validation and error reporting
- Hot reload capability (optional)

**Files to Create/Modify**:
- `pipeline/src/infrastructure/config/loader.rs`
- `pipeline/src/infrastructure/config/validator.rs`
- Configuration schema documentation

**Estimated Effort**: 4-6 hours

---

#### 2. ✅ Configurable Channel Depth (COMPLETED 2025-10-06)
**Location**: `pipeline/src/application/services/pipeline.rs:851`
**Status**: ✅ **Implemented**
**Description**: Make channel depth configurable via CLI

**Implementation**:
```rust
let channel_depth = channel_depth_override.unwrap_or(4);
debug!("Using channel depth: {}", channel_depth);
```

**Completed Features**:
- ✅ Added CLI parameter `--channel-depth` with default value 4
- ✅ Passed through ValidatedCli, ProcessFileConfig, and PipelineService
- ✅ Used in pipeline execution with proper fallback to default
- ✅ Comprehensive help documentation with educational content
- ✅ Validated and tested with multiple channel depth values

**Files Modified**:
- `bootstrap/src/cli/parser.rs` - Added --channel-depth parameter
- `bootstrap/src/cli.rs` - Added to ValidatedCli struct
- `pipeline/src/application/use_cases/process_file.rs` - Added to ProcessFileConfig
- `pipeline/src/main.rs` - Passed from CLI to config
- `pipeline-domain/src/services/pipeline_service.rs` - Added to trait
- `pipeline/src/application/services/pipeline.rs` - Implemented functionality

**Actual Effort**: ~1 hour

---

#### 3. Permission Validation for File Restoration
**Location**: `pipeline/src/main.rs` (comments)
**Status**: TODO
**Description**: Implement permission validation via domain service

**Requirements**:
- Validate file system permissions before restoration
- Check write permissions on output directory
- Prevent path traversal attacks
- Validate user has permission to create files
- Comprehensive error messages

**Files to Create/Modify**:
- `pipeline-domain/src/services/permission_service.rs`
- `pipeline/src/application/use_cases/restore_file.rs`

**Estimated Effort**: 3-4 hours

---

## 🔐 Security Enhancements

### Medium Priority

#### 4. Security Context Integration
**Location**: `pipeline-domain/src/aggregates/pipeline_aggregate.rs`
**Status**: Placeholder (None)
**Description**: Wire up actual user tracking from security context

**Current State**:
```rust
created_by: None, // TODO: Get from security context
updated_by: None, // TODO: Get from security context
```

**Requirements**:
- Extract user identity from SecurityContext
- Audit trail for pipeline creation/updates
- Store user ID in database
- Display in pipeline metadata
- Support anonymous mode for testing

**Files to Modify**:
- `pipeline-domain/src/aggregates/pipeline_aggregate.rs`
- `pipeline-domain/src/entities/security_context.rs`
- Database schema migration for user fields

**Estimated Effort**: 4-5 hours

---

#### 5. Secure Storage for Encryption Keys
**Location**: `pipeline/src/infrastructure/adapters/encryption.rs:671`
**Status**: Placeholder Implementation
**Description**: Implement actual secure key storage mechanism

**Current State**:
```rust
// TODO: Implement actual secure storage
```

**Requirements**:
- Platform-specific secure storage:
  - macOS: Keychain
  - Linux: Secret Service API / gnome-keyring
  - Windows: DPAPI
- Key derivation from master password
- Key rotation support
- Secure key deletion
- Comprehensive error handling

**Dependencies**:
- `keyring` crate (cross-platform)
- Platform-specific APIs

**Files to Create/Modify**:
- `pipeline/src/infrastructure/adapters/encryption/key_storage.rs`
- `pipeline/src/infrastructure/adapters/encryption/platform/macos.rs`
- `pipeline/src/infrastructure/adapters/encryption/platform/linux.rs`
- `pipeline/src/infrastructure/adapters/encryption/platform/windows.rs`

**Estimated Effort**: 8-12 hours (cross-platform complexity)

---

## 🚀 Feature Completions

### Medium Priority

#### 6. Full Streaming Validation
**Location**: `pipeline/src/application/use_cases/validate_file.rs`
**Status**: Partially Implemented
**Description**: Complete streaming validation implementation

**Current State**:
```rust
// TODO: Full streaming validation not yet implemented
```

**Requirements**:
- Stream file chunks without loading entire file
- Validate chunk checksums incrementally
- Validate file header and metadata
- Validate stage sequence integrity
- Memory-efficient for large files (>1GB)
- Progress reporting

**Files to Modify**:
- `pipeline/src/application/use_cases/validate_file.rs`
- `pipeline/src/infrastructure/services/binary_format.rs`

**Estimated Effort**: 6-8 hours

---

### Low Priority

#### 7. Global Shutdown Coordinator for Ctrl-C
**Location**: `pipeline/src/application/services/pipeline.rs:844`
**Status**: TODO
**Description**: Wire to global ShutdownCoordinator for graceful shutdown

**Current State**:
```rust
// TODO: Wire this to global ShutdownCoordinator for Ctrl-C handling
```

**Requirements**:
- Integrate with signal handling (SIGINT, SIGTERM)
- Graceful shutdown of processing pipeline
- Cleanup temporary files
- Save processing state
- Timeout for forced shutdown (30s default)

**Files to Create/Modify**:
- `bootstrap/src/shutdown_coordinator.rs`
- `pipeline/src/application/services/pipeline.rs`

**Estimated Effort**: 4-6 hours

---

#### 8. Async Pipeline Processing Interface
**Location**: `pipeline/src/application/services/pipeline.rs:1378`
**Status**: TODO (if needed)
**Description**: Create separate async interface if needed

**Current State**:
```rust
// TODO: If needed, create a separate async pipeline processing interface
```

**Requirements**:
- Evaluate actual need (might not be necessary)
- Design async trait interface
- Implement for existing pipeline
- Update use cases to support async
- Benchmark performance difference

**Decision Required**: Assess if current sync interface is sufficient

**Estimated Effort**: 6-10 hours (if needed)

---

#### 9. Metrics Loading in Repository
**Location**: `pipeline/src/infrastructure/repositories/sqlite_pipeline.rs:745`
**Status**: Dummy Values
**Description**: Implement actual metrics loading from database

**Current State**:
```rust
let metrics = ProcessingMetrics::new(0, 0); // TODO: Implement metrics loading
```

**Requirements**:
- Load historical processing metrics from database
- Aggregate metrics across pipeline executions
- Cache frequently accessed metrics
- Optimize query performance
- Add metrics to pipeline metadata display

**Database Changes**:
- Create `pipeline_metrics` table
- Add indexes for efficient queries
- Migration script

**Files to Modify**:
- `pipeline/src/infrastructure/repositories/sqlite_pipeline.rs`
- `pipeline/src/infrastructure/repositories/schema.rs`

**Estimated Effort**: 4-6 hours

---

## 🏗️ Architecture Improvements

### Low Priority

#### 10. Unified Stage Interface
**Location**: `pipeline-domain/src/services/checksum_service.rs:176`
**Status**: TODO
**Description**: Standardize stage interface across all services

**Requirements**:
- Create unified `StageService` trait
- Migrate all stages to use unified interface
- Consistent error handling
- Common metadata/metrics
- Backward compatibility during migration

**Affected Services**:
- ChecksumService
- CompressionService
- EncryptionService
- Base64EncodingService
- PiiMaskingService
- TeeService
- DebugService

**Files to Create/Modify**:
- `pipeline-domain/src/services/stage_service.rs` (already exists, enhance)
- All service implementations

**Estimated Effort**: 10-15 hours (major refactoring)

---

#### 11. Move Repository Traits to Domain Layer
**Location**: `pipeline-domain/src/entities/pipeline.rs:845`
**Status**: Architectural Debt
**Description**: Repository traits should be in domain/repositories, not entities

**Current State**:
```rust
// TODO: These traits should be defined in domain/repositories, not referenced
```

**Requirements**:
- Create `pipeline-domain/src/repositories/` module
- Move all repository trait definitions
- Update imports across codebase
- Follow DDD layering principles
- No breaking changes to external API

**Files to Create/Modify**:
- `pipeline-domain/src/repositories/pipeline_repository.rs`
- `pipeline-domain/src/repositories/mod.rs`
- Update all implementations and imports

**Estimated Effort**: 3-4 hours

---

## ⚡ Performance Optimizations

### Low Priority

#### 12. Chunk Index for Binary Format
**Location**: `pipeline/src/infrastructure/services/binary_format.rs:652`
**Status**: Production Optimization
**Description**: Maintain chunk index for faster seeking in large files

**Current State**:
```rust
// TODO: In production, we could maintain a chunk index for faster seeking
```

**Requirements**:
- Build in-memory chunk offset index
- Optional persistent index file (.adapipe.idx)
- Fast random access to chunks
- Minimal memory overhead
- Benchmark performance improvement

**Use Cases**:
- Large file validation (>10GB)
- Random access restoration
- Chunk-level inspection tools

**Files to Modify**:
- `pipeline/src/infrastructure/services/binary_format.rs`
- `pipeline-domain/src/value_objects/binary_file_format.rs`

**Estimated Effort**: 6-8 hours

---

## 📊 Summary

### Priority Breakdown

| Priority | Count | Total Effort (hours) |
|----------|-------|---------------------|
| Medium   | 6     | 29-36               |
| Low      | 6     | 33-49               |
| **Total**| **12**| **62-85**           |

### By Category

| Category                   | Items | Effort (hours) |
|----------------------------|-------|---------------|
| Configuration & CLI        | 3     | 9-13          |
| Security Enhancements      | 2     | 12-17         |
| Feature Completions        | 4     | 20-30         |
| Architecture Improvements  | 2     | 13-19         |
| Performance Optimizations  | 1     | 6-8           |

---

## 🎯 Recommended Implementation Order

Based on dependencies and impact:

1. **Configuration File Loading** (Medium) - Foundation for other features
2. **Configurable Channel Depth** (Medium) - Quick win, improves usability
3. **Security Context Integration** (Medium) - Important for audit trail
4. **Full Streaming Validation** (Medium) - Completes core feature
5. **Metrics Loading** (Low) - Improves observability
6. **Permission Validation** (Medium) - Security improvement
7. **Move Repository Traits** (Low) - Architectural cleanup
8. **Secure Key Storage** (Medium) - Complex but important security feature
9. **Shutdown Coordinator** (Low) - UX improvement
10. **Chunk Index** (Low) - Performance optimization
11. **Unified Stage Interface** (Low) - Large refactoring, save for later
12. **Async Interface** (Low) - Evaluate need first

---

## 📝 Notes

- All estimates are for experienced Rust developers
- Testing time is included in estimates
- Documentation updates should accompany each feature
- Consider creating issues in GitHub for tracking
- Review priorities quarterly based on user feedback

---

**Version History**:
- v0.1.0 (2025-10-06): Initial roadmap created from TODO audit
