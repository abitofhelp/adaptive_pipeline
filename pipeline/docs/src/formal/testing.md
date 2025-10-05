# Software Test Plan (STP)

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

---

## 1. Introduction

### 1.1 Purpose

This Software Test Plan (STP) defines the testing strategy, approach, and organization for the Optimized Adaptive Pipeline system. It ensures the system meets all functional and non-functional requirements specified in the SRS through comprehensive, systematic testing.

**Intended Audience:**
- QA engineers implementing tests
- Developers writing testable code
- Project managers tracking test progress
- Stakeholders evaluating quality assurance

### 1.2 Scope

**Testing Coverage:**
- Unit testing of all domain, application, and infrastructure components
- Integration testing of component interactions
- End-to-end testing of complete workflows
- Architecture compliance testing
- Performance and benchmark testing
- Security testing

**Out of Scope:**
- User acceptance testing (no end users yet)
- Load testing (single-machine application)
- Penetration testing (no network exposure)
- GUI testing (CLI only)

### 1.3 Test Objectives

1. **Verify Correctness**: Ensure all requirements are met
2. **Validate Design**: Confirm architectural compliance
3. **Ensure Quality**: Maintain high code quality standards
4. **Prevent Regression**: Catch bugs before they reach production
5. **Document Behavior**: Tests serve as living documentation
6. **Enable Refactoring**: Safe code changes through comprehensive tests

### 1.4 References

- [Software Requirements Specification (SRS)](../reference/srs.md)
- [Software Design Document (SDD)](design.md)
- [Test Organization Documentation](../../../../docs/TEST_ORGANIZATION.md)
- Rust Testing Documentation: https://doc.rust-lang.org/book/ch11-00-testing.html

---

## 2. Test Strategy

### 2.1 Testing Approach

**Test-Driven Development (TDD):**
- Write tests before implementation when feasible
- Red-Green-Refactor cycle
- Tests as specification

**Behavior-Driven Development (BDD):**
- Given-When-Then structure for integration tests
- Readable test names describing behavior
- Focus on outcomes, not implementation

**Property-Based Testing:**
- Use Proptest for algorithmic correctness
- Generate random inputs to find edge cases
- Verify invariants hold for all inputs

### 2.2 Test Pyramid

```
        ┌─────────────┐
        │   E2E (11)  │  ← Few, slow, high-level
        ├─────────────┤
        │Integration  │  ← Medium count, medium speed
        │    (35)     │
        ├─────────────┤
        │   Unit      │  ← Many, fast, focused
        │    (68)     │
        └─────────────┘
```

**Rationale:**
- Most tests are fast unit tests for quick feedback
- Integration tests verify component collaboration
- E2E tests validate complete user workflows
- Architecture tests ensure design compliance

### 2.3 Test Organization (Post-Reorganization)

Following Rust best practices:

**Unit Tests:**
- Location: `#[cfg(test)]` modules within source files
- Scope: Single function/struct in isolation
- Run with: `cargo test --lib`
- Count: 68 tests
- Example: `pipeline-domain/src/entities/pipeline_stage.rs:590-747`

**Integration Tests:**
- Location: `pipeline/tests/integration/`
- Entry: `pipeline/tests/integration.rs`
- Scope: Multiple components working together
- Run with: `cargo test --test integration`
- Count: 35 tests (3 ignored pending work)

**End-to-End Tests:**
- Location: `pipeline/tests/e2e/`
- Entry: `pipeline/tests/e2e.rs`
- Scope: Complete workflows from input to output
- Run with: `cargo test --test e2e`
- Count: 11 tests

**Architecture Compliance Tests:**
- Location: `pipeline/tests/architecture_compliance_test.rs`
- Scope: Validate DDD, Clean Architecture, Hexagonal Architecture
- Run with: `cargo test --test architecture_compliance_test`
- Count: 2 tests

**Documentation Tests:**
- Location: Doc comments with ` ``` ` code blocks
- Run with: `cargo test --doc`
- Count: 25 tests

**Total Test Count: 141 tests**

---

## 3. Test Levels

### 3.1 Unit Testing

**Objectives:**
- Test individual functions and methods in isolation
- Verify domain logic correctness
- Ensure edge cases are handled
- Validate error conditions

**Coverage Goals:**
- Domain layer: 90%+ line coverage
- Application layer: 85%+ line coverage
- Infrastructure layer: 75%+ line coverage

**Example Test Structure:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation_validates_name() {
        // Given: Empty name
        let name = "";
        let stages = vec![create_test_stage()];

        // When: Creating pipeline
        let result = Pipeline::new(name.to_string(), stages);

        // Then: Should fail validation
        assert!(result.is_err());
        assert!(matches!(result, Err(PipelineError::InvalidName(_))));
    }

    #[test]
    fn test_pipeline_requires_at_least_one_stage() {
        // Given: Valid name but no stages
        let name = "test-pipeline";
        let stages = vec![];

        // When: Creating pipeline
        let result = Pipeline::new(name.to_string(), stages);

        // Then: Should fail validation
        assert!(result.is_err());
        assert!(matches!(result, Err(PipelineError::NoStages)));
    }
}
```

**Key Test Categories:**

| Category | Description | Example |
|----------|-------------|---------|
| Happy Path | Normal, expected inputs | Valid pipeline creation |
| Edge Cases | Boundary conditions | Empty strings, max values |
| Error Handling | Invalid inputs | Malformed data, constraints violated |
| Invariants | Domain rules always hold | Stage order uniqueness |

### 3.2 Integration Testing

**Objectives:**
- Test component interactions
- Verify interfaces between layers
- Validate repository operations
- Test service collaboration

**Test Structure:**

```rust
#[tokio::test]
async fn test_pipeline_repository_save_and_retrieve() {
    // Given: In-memory repository and pipeline
    let repo = Arc::new(InMemoryPipelineRepository::new());
    let pipeline = create_test_pipeline();

    // When: Saving and retrieving pipeline
    repo.save(&pipeline).await.unwrap();
    let retrieved = repo.find_by_name(&pipeline.name)
        .await
        .unwrap()
        .unwrap();

    // Then: Retrieved pipeline matches original
    assert_eq!(retrieved.name, pipeline.name);
    assert_eq!(retrieved.stages.len(), pipeline.stages.len());
}
```

**Integration Test Suites:**

1. **Application Layer Integration** (`application_integration_test.rs`)
   - Command execution
   - Use case orchestration
   - Service coordination

2. **Application Services** (`application_services_integration_test.rs`)
   - Service interactions
   - Transaction handling
   - Error propagation

3. **Domain Services** (`domain_services_test.rs`)
   - Compression service integration
   - Encryption service integration
   - Checksum service integration
   - File I/O service integration

4. **Schema Integration** (`schema_integration_test.rs`)
   - Database creation
   - Schema migrations
   - Idempotent initialization

5. **Pipeline Name Validation** (`pipeline_name_validation_tests.rs`)
   - Name normalization
   - Validation rules
   - Reserved names

### 3.3 End-to-End Testing

**Objectives:**
- Validate complete user workflows
- Test real file processing scenarios
- Verify .adapipe format correctness
- Ensure restoration matches original

**Test Structure:**

```rust
#[tokio::test]
async fn test_e2e_complete_pipeline_workflow() {
    // Given: Test input file and pipeline configuration
    let input_path = create_test_file_with_content("Hello, World!");
    let pipeline = create_secure_pipeline(); // compress + encrypt + checksum

    // When: Processing the file
    let processor = PipelineProcessor::new()?;
    let output_path = processor.process(&pipeline, &input_path).await?;

    // Then: Output file should exist and be valid .adapipe format
    assert!(output_path.exists());
    assert!(output_path.extension().unwrap() == "adapipe");

    // And: Can restore original file
    let restored_path = processor.restore(&output_path).await?;
    let restored_content = fs::read_to_string(&restored_path).await?;
    assert_eq!(restored_content, "Hello, World!");

    // Cleanup
    cleanup_test_files(&[input_path, output_path, restored_path])?;
}
```

**E2E Test Scenarios:**

1. **Binary Format Complete Roundtrip** (`e2e_binary_format_test.rs`)
   - Process file through all stages
   - Verify .adapipe format structure
   - Restore and compare with original
   - Test large files (memory mapping)
   - Test corruption detection
   - Test version compatibility

2. **Restoration Pipeline** (`e2e_restore_pipeline_test.rs`)
   - Multi-stage restoration
   - Stage ordering (reverse of processing)
   - Real-world document restoration
   - File header roundtrip
   - Chunk processing verification

### 3.4 Architecture Compliance Testing

**Objectives:**
- Enforce architectural boundaries
- Validate design patterns
- Ensure dependency rules
- Verify SOLID principles

**Test Structure:**

```rust
#[tokio::test]
async fn test_ddd_compliance() {
    println!("Testing DDD Compliance");

    // Test domain entities in isolation
    test_domain_entity_isolation().await;

    // Test value objects for immutability
    test_value_object_patterns().await;

    // Test domain services through interfaces
    test_domain_service_interfaces().await;

    println!("✅ DDD Compliance: PASSED");
}
```

**Compliance Checks:**

1. **Domain-Driven Design (DDD)**
   - Entities tested in isolation
   - Value objects immutable
   - Domain services use interfaces
   - Aggregates maintain consistency

2. **Clean Architecture**
   - Dependency flow is inward only
   - Use cases independent
   - Infrastructure tested through abstractions

3. **Hexagonal Architecture**
   - Primary ports (driving adapters) tested
   - Secondary ports (driven adapters) mocked
   - Application core isolated

4. **Dependency Inversion Principle**
   - High-level modules depend on abstractions
   - Low-level modules implement abstractions
   - Abstractions remain stable

---

## 4. Test Automation

### 4.1 Continuous Integration

**CI Pipeline:**

```yaml
# .github/workflows/ci.yml (example)
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib
      - name: Run integration tests
        run: cargo test --test integration
      - name: Run E2E tests
        run: cargo test --test e2e
      - name: Run architecture tests
        run: cargo test --test architecture_compliance_test
      - name: Run doc tests
        run: cargo test --doc

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: cargo tarpaulin --out Xml
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
```

### 4.2 Pre-commit Hooks

**Git Hooks:**

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Run tests
cargo test --lib || exit 1

# Check formatting
cargo fmt -- --check || exit 1

# Run clippy
cargo clippy -- -D warnings || exit 1

echo "✅ All pre-commit checks passed"
```

### 4.3 Test Commands

**Quick Feedback (fast):**
```bash
cargo test --lib  # Unit tests only (~1 second)
```

**Full Test Suite:**
```bash
cargo test  # All tests (~15 seconds)
```

**Specific Test Suites:**
```bash
cargo test --test integration          # Integration tests
cargo test --test e2e                  # E2E tests
cargo test --test architecture_compliance_test  # Architecture tests
cargo test --doc                       # Documentation tests
```

**With Coverage:**
```bash
cargo tarpaulin --out Html --output-dir coverage/
```

---

## 5. Testing Tools and Frameworks

### 5.1 Core Testing Framework

**Built-in Rust Testing:**
- `#[test]` attribute for unit tests
- `#[tokio::test]` for async tests
- `assert!`, `assert_eq!`, `assert_ne!` macros
- `#[should_panic]` for error testing

**Async Testing:**
```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

### 5.2 Mocking and Test Doubles

**Mockall:**

```rust
#[automock]
#[async_trait]
pub trait PipelineRepository {
    async fn save(&self, pipeline: &Pipeline) -> Result<()>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Pipeline>>;
}

#[tokio::test]
async fn test_with_mock_repository() {
    let mut mock_repo = MockPipelineRepository::new();
    mock_repo
        .expect_save()
        .times(1)
        .returning(|_| Ok(()));

    let service = PipelineService::new(Arc::new(mock_repo));
    let result = service.create_pipeline("test").await;

    assert!(result.is_ok());
}
```

### 5.3 Property-Based Testing

**Proptest:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_chunk_size_always_valid(size in 1024usize..=100_000_000) {
        // Given: Any size within valid range
        let chunk_size = ChunkSize::new(size);

        // Then: Should always succeed
        prop_assert!(chunk_size.is_ok());
        prop_assert_eq!(chunk_size.unwrap().value(), size);
    }

    #[test]
    fn test_compression_roundtrip(
        data in prop::collection::vec(any::<u8>(), 0..10000)
    ) {
        // Given: Random byte array
        let compressed = compress(&data)?;
        let decompressed = decompress(&compressed)?;

        // Then: Should match original
        prop_assert_eq!(data, decompressed);
    }
}
```

### 5.4 Benchmarking

**Criterion:**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_compression(c: &mut Criterion) {
    let data = vec![0u8; 1_000_000]; // 1 MB

    c.bench_function("brotli_compression", |b| {
        b.iter(|| {
            let adapter = BrotliAdapter::new(6);
            adapter.compress(black_box(&data))
        })
    });

    c.bench_function("zstd_compression", |b| {
        b.iter(|| {
            let adapter = ZstdAdapter::new(3);
            adapter.compress(black_box(&data))
        })
    });
}

criterion_group!(benches, bench_compression);
criterion_main!(benches);
```

**Run Benchmarks:**
```bash
cargo bench
```

### 5.5 Code Coverage

**Cargo-tarpaulin:**

```bash
# Install
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# View report
open coverage/index.html
```

**Coverage Goals:**
- Overall: 80%+ line coverage
- Domain layer: 90%+ coverage
- Critical paths: 95%+ coverage

---

## 6. Test Data Management

### 6.1 Test Fixtures

**Fixture Organization:**

```
testdata/
├── input/
│   ├── sample.txt
│   ├── large_file.bin (10 MB)
│   └── document.pdf
├── expected/
│   ├── sample_compressed.bin
│   └── sample_encrypted.bin
└── schemas/
    └── v1_pipeline.json
```

**Fixture Helpers:**

```rust
pub fn create_test_file(content: &str) -> PathBuf {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    std::fs::write(&file_path, content).unwrap();
    file_path
}

pub fn create_test_pipeline() -> Pipeline {
    Pipeline::builder()
        .name("test-pipeline")
        .add_stage(compression_stage("compress", "zstd", 1))
        .add_stage(encryption_stage("encrypt", "aes256gcm", 2))
        .build()
        .unwrap()
}
```

### 6.2 Test Database

**In-Memory SQLite:**

```rust
async fn create_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect(":memory:")
        .await
        .unwrap();

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    pool
}
```

**Test Isolation:**
- Each test gets fresh database
- Transactions rolled back after test
- No test interdependencies

---

## 7. Performance Testing

### 7.1 Benchmark Suites

**Algorithm Benchmarks:**
- Compression algorithms (Brotli, Zstd, Gzip, LZ4)
- Encryption algorithms (AES-256-GCM, ChaCha20-Poly1305)
- Hashing algorithms (SHA-256, SHA-512, BLAKE3)

**File Size Benchmarks:**
- Small files (< 1 MB)
- Medium files (1-100 MB)
- Large files (> 100 MB)

**Concurrency Benchmarks:**
- Single-threaded vs multi-threaded
- Async vs sync I/O
- Chunk size variations

### 7.2 Performance Regression Testing

**Baseline Establishment:**
```bash
# Run benchmarks and save baseline
cargo bench -- --save-baseline main
```

**Regression Detection:**
```bash
# Compare against baseline
cargo bench -- --baseline main
```

**CI Integration:**
- Fail PR if >10% performance degradation
- Alert on >5% degradation
- Celebrate >10% improvement

---

## 8. Security Testing

### 8.1 Input Validation Testing

**Test Cases:**
- Path traversal attempts (`../../etc/passwd`)
- Command injection attempts
- SQL injection (SQLx prevents, but verify)
- Buffer overflow attempts
- Invalid UTF-8 sequences

**Example:**

```rust
#[test]
fn test_rejects_path_traversal() {
    let malicious_path = "../../etc/passwd";
    let result = validate_input_path(malicious_path);
    assert!(result.is_err());
}
```

### 8.2 Cryptographic Testing

**Test Cases:**
- Key derivation reproducibility
- Encryption/decryption roundtrips
- Authentication tag verification
- Nonce uniqueness
- Secure memory wiping

**Example:**

```rust
#[tokio::test]
async fn test_encryption_with_wrong_key_fails() {
    let data = b"secret data";
    let correct_key = generate_key();
    let wrong_key = generate_key();

    let encrypted = encrypt(data, &correct_key).await?;
    let result = decrypt(&encrypted, &wrong_key).await;

    assert!(result.is_err());
}
```

### 8.3 Dependency Security

**Cargo-audit:**

```bash
# Install
cargo install cargo-audit

# Check for vulnerabilities
cargo audit

# CI integration
cargo audit --deny warnings
```

**Cargo-deny:**

```bash
# Check licenses and security
cargo deny check
```

---

## 9. Test Metrics and Reporting

### 9.1 Test Metrics

**Key Metrics:**
- Test count: 141 tests
- Test pass rate: Target 100%
- Code coverage: Target 80%+
- Test execution time: < 20 seconds for full suite
- Benchmark performance: Track trends

**Tracking:**
```bash
# Test count
cargo test -- --list | wc -l

# Coverage
cargo tarpaulin --out Json | jq '.coverage'

# Execution time
time cargo test
```

### 9.2 Test Reporting

**Console Output:**
```
running 141 tests
test unit::test_pipeline_creation ... ok
test integration::test_repository_save ... ok
test e2e::test_complete_workflow ... ok

test result: ok. 141 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.34s
```

**Coverage Report:**
```
|| Tested/Total Lines:
|| src/domain/entities/pipeline.rs: 95/100 (95%)
|| src/domain/services/compression.rs: 87/95 (91.6%)
|| Overall: 2847/3421 (83.2%)
```

**Benchmark Report:**
```
brotli_compression      time:   [45.2 ms 46.1 ms 47.0 ms]
                        change: [-2.3% +0.1% +2.5%] (p = 0.91 > 0.05)
                        No change in performance detected.

zstd_compression        time:   [12.5 ms 12.7 ms 12.9 ms]
                        change: [-8.2% -6.5% -4.8%] (p = 0.00 < 0.05)
                        Performance has improved.
```

---

## 10. Test Maintenance

### 10.1 Test Review Process

**Code Review Checklist:**
- [ ] Tests cover new functionality
- [ ] Tests follow naming conventions
- [ ] Tests are independent and isolated
- [ ] Test data is appropriate
- [ ] Assertions are clear and specific
- [ ] Edge cases are tested
- [ ] Error conditions are tested

### 10.2 Test Refactoring

**When to Refactor Tests:**
- Duplicate test logic (extract helpers)
- Brittle tests (too coupled to implementation)
- Slow tests (optimize or move to integration)
- Unclear test names (rename for clarity)

**Test Helpers:**

```rust
// Instead of duplicating this in every test:
#[test]
fn test_something() {
    let stage = PipelineStage::new(
        "compress".to_string(),
        StageType::Compression,
        StageConfiguration {
            algorithm: "zstd".to_string(),
            parameters: HashMap::new(),
            parallel_processing: false,
            chunk_size: None,
        },
        1
    ).unwrap();
    // ...
}

// Extract helper:
fn create_compression_stage(name: &str, order: usize) -> PipelineStage {
    PipelineStage::compression(name, "zstd", order).unwrap()
}

#[test]
fn test_something() {
    let stage = create_compression_stage("compress", 1);
    // ...
}
```

### 10.3 Flaky Test Prevention

**Common Causes:**
- Time-dependent tests
- Filesystem race conditions
- Nondeterministic ordering
- Shared state between tests

**Solutions:**
- Mock time with `mockall`
- Use unique temp directories
- Sort collections before assertions
- Ensure test isolation

---

## 11. Test Schedule

### 11.1 Development Workflow

**During Development:**
```bash
# Quick check (unit tests only)
cargo test --lib

# Before commit
cargo test && cargo clippy
```

**Before Push:**
```bash
# Full test suite
cargo test

# Check formatting
cargo fmt -- --check

# Lint
cargo clippy -- -D warnings
```

**Before Release:**
```bash
# All tests
cargo test

# Benchmarks
cargo bench

# Coverage
cargo tarpaulin

# Security audit
cargo audit
```

### 11.2 CI/CD Integration

**On Every Commit:**
- Unit tests
- Integration tests
- Clippy linting
- Format checking

**On Pull Request:**
- Full test suite
- Coverage report
- Benchmark comparison
- Security audit

**On Release:**
- Full test suite
- Performance benchmarks
- Security scan
- Documentation build

---

## 12. Test Deliverables

### 12.1 Test Documentation

- [x] Test Organization Guide (`docs/TEST_ORGANIZATION.md`)
- [x] Architecture Compliance Tests (`tests/architecture_compliance_test.rs`)
- [x] This Software Test Plan

### 12.2 Test Code

- [x] 68 unit tests in source files
- [x] 35 integration tests in `tests/integration/`
- [x] 11 E2E tests in `tests/e2e/`
- [x] 2 architecture compliance tests
- [x] 25 documentation tests
- [ ] Benchmark suite (TODO)
- [ ] Property-based tests (TODO)

### 12.3 Test Reports

**Generated Artifacts:**
- Test execution report (console output)
- Coverage report (HTML, XML)
- Benchmark report (HTML, JSON)
- Security audit report

---

## 13. Risks and Mitigation

### 13.1 Testing Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Flaky tests | Medium | Low | Test isolation, deterministic behavior |
| Slow tests | Medium | Medium | Optimize, parallelize, tiered testing |
| Low coverage | High | Low | Coverage tracking, CI enforcement |
| Missing edge cases | High | Medium | Property-based testing, code review |
| Test maintenance burden | Medium | High | Helper functions, clear conventions |

### 13.2 Mitigation Strategies

**Flaky Tests:**
- Run tests multiple times in CI
- Investigate and fix immediately
- Use deterministic test data

**Slow Tests:**
- Profile test execution
- Optimize slow tests
- Move to higher test level if appropriate

**Low Coverage:**
- Track coverage in CI
- Require minimum coverage for PRs
- Review uncovered code paths

---

## 14. Conclusion

This Software Test Plan establishes a comprehensive testing strategy for the Optimized Adaptive Pipeline system. Key highlights:

- **141 tests** across all levels (unit, integration, E2E, architecture, doc)
- **Organized structure** following Rust best practices
- **Automated CI/CD** integration for continuous quality
- **High coverage goals** (80%+ overall, 90%+ domain layer)
- **Multiple testing approaches** (TDD, BDD, property-based)
- **Performance monitoring** through benchmarks
- **Security validation** through audits and crypto testing

The testing strategy ensures the system meets all requirements, maintains high quality, and remains maintainable as it evolves.

---

## Appendix A: Test Command Reference

```bash
# Run all tests
cargo test

# Run specific test levels
cargo test --lib                    # Unit tests only
cargo test --test integration       # Integration tests
cargo test --test e2e              # E2E tests
cargo test --test architecture_compliance_test  # Architecture tests
cargo test --doc                    # Doc tests

# Run specific test
cargo test test_pipeline_creation

# Run tests matching pattern
cargo test pipeline

# Show test output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test

# Run tests serially
cargo test -- --test-threads=1

# Generate coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench

# Security audit
cargo audit
```

---

## Appendix B: Test Naming Conventions

**Unit Tests:**
- `test_<function>_<scenario>_<expected_result>`
- Example: `test_pipeline_creation_with_empty_name_fails`

**Integration Tests:**
- `test_<component>_<interaction>_<expected_result>`
- Example: `test_repository_save_and_retrieve_pipeline`

**E2E Tests:**
- `test_e2e_<workflow>_<scenario>`
- Example: `test_e2e_complete_pipeline_roundtrip`

**Property Tests:**
- `test_<property>_holds_for_all_<inputs>`
- Example: `test_compression_roundtrip_succeeds_for_all_data`
