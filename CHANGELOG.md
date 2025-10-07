# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/abitofhelp/optimized_adaptive_pipeline_rs/releases/tag/pipeline-domain-v0.1.0) - 2025-10-07

### Bug Fixes

- resolve test compilation errors and update documentation

### Documentation

- update book for first release - modern Rust patterns and accurate API names

### Features

- add pipeline stage ordering validation (PreBinary before PostBinary)
- implement unified stage service architecture with generic FromParameters trait

### Refactor

- Convert domain to sync, achieve Clean Architecture compliance

### Refactoring

- complete Rust 2018+ module pattern migration across all layers
- standardize Rust naming conventions across infrastructure layer
- reorganize tests following Rust best practices and update documentation
- streamline domain and application layer documentation

### Tests

- rewrite e2e binary format tests with real components and fix isolation
- add comprehensive cancellation and exit code tests

### Added

### Changed

### Fixed

### Removed

### Security