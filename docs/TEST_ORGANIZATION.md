# Test Organization

This document describes the test organization structure for the Optimized Adaptive Pipeline project.

## Test Structure

The project follows Rust best practices for test organization:

### Unit Tests
- **Location**: Within source files using `#[cfg(test)]` modules
- **Purpose**: Test implementation details and private functions
- **Run with**: `cargo test --lib`
- **Example**: `pipeline-domain/src/entities/pipeline_stage.rs` contains unit tests for `StageType` enum and `PipelineStage` struct

### Integration Tests
- **Location**: `pipeline/tests/integration/`
- **Entry Point**: `pipeline/tests/integration.rs`
- **Purpose**: Test public API as an external consumer would
- **Run with**: `cargo test --test integration`
- **Files**:
  - `application_integration_test.rs` - Application layer component integration
  - `application_layer_integration_test.rs` - Detailed application layer tests
  - `application_services_integration_test.rs` - Application service interactions
  - `domain_services_test.rs` - Domain service integration tests
  - `minimal_application_test.rs` - Lightweight smoke tests
  - `pipeline_name_validation_tests.rs` - Pipeline name validation tests
  - `schema_integration_test.rs` - Schema integration validation

### End-to-End Tests
- **Location**: `pipeline/tests/e2e/`
- **Entry Point**: `pipeline/tests/e2e.rs`
- **Purpose**: Complete workflow testing
- **Run with**: `cargo test --test e2e`
- **Files**:
  - `e2e_binary_format_test.rs` - Binary format roundtrip tests
  - `e2e_restore_pipeline_test.rs` - Restoration pipeline tests

### Architecture Compliance Tests
- **Location**: `pipeline/tests/architecture_compliance_test.rs`
- **Purpose**: Validate DDD, Clean Architecture, and Hexagonal Architecture compliance
- **Run with**: `cargo test --test architecture_compliance_test`

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Only Unit Tests (Fast)
```bash
cargo test --lib
```

### Run Only Integration Tests
```bash
cargo test --test integration
```

### Run Only E2E Tests
```bash
cargo test --test e2e
```

### Run Architecture Compliance Tests
```bash
cargo test --test architecture_compliance_test
```

## Test Statistics

As of the latest reorganization:
- **Unit Tests**: 68 tests in source files
- **Integration Tests**: 38 tests (35 passed, 3 ignored)
- **E2E Tests**: 11 tests
- **Architecture Compliance**: 2 tests
- **Doc Tests**: 25 tests (6 pipeline + 19 pipeline-domain)
- **Total**: 141+ tests

## Benefits of This Organization

1. **Faster Feedback Loop**: Run only unit tests (`cargo test --lib`) for quick iterations
2. **Clear Test Intent**: Test location indicates scope and purpose
3. **Better Code Coverage**: Unit tests in source files improve visibility
4. **CI Optimization**: Different test types can run in parallel in CI/CD
5. **Rust Community Standards**: Follows established Rust testing conventions
