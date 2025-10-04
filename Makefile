# Makefile for Optimized Adaptive Pipeline Rust Project
# Best practices for Rust development workflow

# Default shell
SHELL := /bin/bash

# Project configuration
PROJECT_NAME := optimized_adaptive_pipeline_rs
RUST_VERSION := stable
CARGO := cargo
CLIPPY_ARGS := -- -D warnings -W clippy::pedantic -W clippy::nursery -W clippy::cargo
RUSTFMT_ARGS := --edition 2021

# Environment Variables
export ADAPIPE_SQLITE_PATH := scripts/test_data/pipeline.db
export RUST_LOG := off

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
NC := \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

# Phony targets
.PHONY: help setup clean build test check lint format doc bench audit security \
        install-tools update-deps release debug run examples workspace-check \
        pipeline-check pipelinelib-check coverage flamegraph bloat pre-commit \
        docker-build docker-run ci-local install-cross-targets build-linux-x86_64 \
        build-linux-aarch64 build-macos-x86_64 build-macos-aarch64 build-windows-x86_64 \
        build-all-platforms

##@ Help
help: ## Display this help message
	@echo -e "$(CYAN)$(PROJECT_NAME) - Rust Development Makefile$(NC)"
	@echo ""
	@echo "Usage:"
	@echo -e "  make $(CYAN)<target>$(NC)"
	@echo ""
	@echo -e "$(YELLOW)Benchmarking$(NC)"
	@echo -e "  $(CYAN)bench                $(NC) Run all benchmarks"
	@echo -e "  $(CYAN)bench-baseline       $(NC) Run benchmarks and save as baseline"
	@echo -e "  $(CYAN)bench-file-io        $(NC) Run file I/O benchmarks specifically"
	@echo ""
	@echo -e "$(YELLOW)Building$(NC)"
	@echo -e "  $(CYAN)build                $(NC) Build the project in debug mode"
	@echo -e "  $(CYAN)build-all            $(NC) Build all workspace members"
	@echo -e "  $(CYAN)build-release        $(NC) Build the project in release mode"
	@echo -e "  $(CYAN)clean                $(NC) Clean build artifacts"
	@echo ""
	@echo -e "$(YELLOW)Code Quality$(NC)"
	@echo -e "  $(CYAN)check                $(NC) Run cargo check"
	@echo -e "  $(CYAN)clippy               $(NC) Run clippy linter with strict settings"
	@echo -e "  $(CYAN)format               $(NC) Format code with rustfmt"
	@echo -e "  $(CYAN)format-check         $(NC) Check code formatting"
	@echo -e "  $(CYAN)lint                 $(NC) Run clippy linter"
	@echo ""
	@echo -e "$(YELLOW)Development Workflow$(NC)"
	@echo -e "  $(CYAN)ci-local             $(NC) Run full CI pipeline locally"
	@echo -e "  $(CYAN)pre-commit           $(NC) Run pre-commit checks"
	@echo -e "  $(CYAN)watch                $(NC) Watch for changes and run tests"
	@echo -e "  $(CYAN)watch-check          $(NC) Watch for changes and run check"
	@echo ""
	@echo -e "$(YELLOW)Docker (if applicable)$(NC)"
	@echo -e "  $(CYAN)docker-build         $(NC) Build Docker image"
	@echo -e "  $(CYAN)docker-run           $(NC) Run Docker container"
	@echo ""
	@echo -e "$(YELLOW)Documentation$(NC)"
	@echo -e "  $(CYAN)doc                  $(NC) Generate documentation"
	@echo -e "  $(CYAN)doc-open             $(NC) Generate and open documentation"
	@echo ""
	@echo -e "$(YELLOW)Help$(NC)"
	@echo -e "  $(CYAN)help                 $(NC) Display this help message"
	@echo -e "  $(CYAN)pipeline-benchmark-help$(NC) Show help for pipeline benchmark subcommand"
	@echo -e "  $(CYAN)pipeline-compare-help$(NC) Show help for pipeline compare subcommand"
	@echo -e "  $(CYAN)pipeline-create-help $(NC) Show help for pipeline create subcommand"
	@echo -e "  $(CYAN)pipeline-delete-help $(NC) Show help for pipeline delete subcommand"
	@echo -e "  $(CYAN)pipeline-help        $(NC) Show help for pipeline command"
	@echo -e "  $(CYAN)pipeline-list-help   $(NC) Show help for pipeline list subcommand"
	@echo -e "  $(CYAN)pipeline-process-help$(NC) Show help for pipeline process subcommand"
	@echo -e "  $(CYAN)pipeline-restore-help$(NC) Show help for pipeline restore subcommand"
	@echo -e "  $(CYAN)pipeline-show-help   $(NC) Show help for pipeline show subcommand"
	@echo -e "  $(CYAN)pipeline-validate-help$(NC) Show help for pipeline validate subcommand"
	@echo -e "  $(CYAN)pipeline-validatefile-help$(NC) Show help for pipeline validatefile subcommand"
	@echo ""
	@echo -e "$(YELLOW)Performance Analysis$(NC)"
	@echo -e "  $(CYAN)bloat                $(NC) Analyze binary size"
	@echo -e "  $(CYAN)flamegraph           $(NC) Generate flamegraph for performance analysis"
	@echo ""
	@echo -e "$(YELLOW)Quick Commands$(NC)"
	@echo -e "  $(CYAN)all                  $(NC) Build, test, lint, and document everything"
	@echo -e "  $(CYAN)dev                  $(NC) Alias for watch (development mode)"
	@echo -e "  $(CYAN)quick-check          $(NC) Quick check (no features)"
	@echo -e "  $(CYAN)quick-test           $(NC) Quick test (no doc tests)"
	@echo ""
	@echo -e "$(YELLOW)Release Management$(NC)"
	@echo -e "  $(CYAN)release              $(NC) Build release version"
	@echo -e "  $(CYAN)release-check        $(NC) Check release build"
	@echo ""
	@echo -e "$(YELLOW)Cross-Platform Builds$(NC)"
	@echo -e "  $(CYAN)build-all-platforms  $(NC) Build for all supported platforms"
	@echo -e "  $(CYAN)build-linux-aarch64  $(NC) Build for Linux ARM64"
	@echo -e "  $(CYAN)build-linux-x86_64   $(NC) Build for Linux x86_64"
	@echo -e "  $(CYAN)build-macos-aarch64  $(NC) Build for macOS ARM64 (Apple Silicon)"
	@echo -e "  $(CYAN)build-macos-x86_64   $(NC) Build for macOS x86_64 (Intel)"
	@echo -e "  $(CYAN)build-windows-x86_64 $(NC) Build for Windows x86_64"
	@echo -e "  $(CYAN)install-cross-targets$(NC) Install cross-compilation toolchains"
	@echo ""
	@echo -e "$(YELLOW)Running$(NC)"
	@echo -e "  $(CYAN)examples             $(NC) Run example code"
	@echo -e "  $(CYAN)run                  $(NC) Run the main pipeline binary"
	@echo -e "  $(CYAN)run-create-db        $(NC) Run the database creation tool"
	@echo -e "  $(CYAN)run-release          $(NC) Run the main pipeline binary in release mode"
	@echo ""
	@echo -e "$(YELLOW)Security & Dependencies$(NC)"
	@echo -e "  $(CYAN)audit                $(NC) Run security audit"
	@echo -e "  $(CYAN)security             $(NC) Alias for audit"
	@echo -e "  $(CYAN)update-deps          $(NC) Update dependencies"
	@echo ""
	@echo -e "$(YELLOW)Setup & Installation$(NC)"
	@echo -e "  $(CYAN)install-tools        $(NC) Install development tools"
	@echo -e "  $(CYAN)setup                $(NC) Setup development environment"
	@echo ""
	@echo -e "$(YELLOW)Testing$(NC)"
	@echo -e "  $(CYAN)coverage             $(NC) Generate test coverage report"
	@echo -e "  $(CYAN)test                 $(NC) Run all tests"
	@echo -e "  $(CYAN)test-doc             $(NC) Run documentation tests"
	@echo -e "  $(CYAN)test-integration     $(NC) Run integration tests only"
	@echo -e "  $(CYAN)test-release         $(NC) Run tests in release mode"
	@echo -e "  $(CYAN)test-unit            $(NC) Run unit tests only"
	@echo -e "  $(CYAN)test-verbose         $(NC) Run tests with verbose output"
	@echo ""
	@echo -e "$(YELLOW)Utilities$(NC)"
	@echo -e "  $(CYAN)expand               $(NC) Expand macros for debugging"
	@echo -e "  $(CYAN)tree                 $(NC) Show dependency tree"
	@echo -e "  $(CYAN)version              $(NC) Show version information"
	@echo ""
	@echo -e "$(YELLOW)Workspace Management$(NC)"
	@echo -e "  $(CYAN)pipeline-check       $(NC) Check pipeline crate specifically"
	@echo -e "  $(CYAN)pipelinelib-check    $(NC) Check pipelinelib crate specifically"
	@echo -e "  $(CYAN)workspace-check      $(NC) Check all workspace members"

pipeline-benchmark-help: ## Show help for pipeline benchmark subcommand
	@echo -e "$(CYAN)Pipeline Benchmark Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- benchmark --help

pipeline-compare-help: ## Show help for pipeline compare subcommand
	@echo -e "$(CYAN)Pipeline Compare Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- compare --help

pipeline-create-help: ## Show help for pipeline create subcommand
	@echo -e "$(CYAN)Pipeline Create Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- create --help

pipeline-delete-help: ## Show help for pipeline delete subcommand
	@echo -e "$(CYAN)Pipeline Delete Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- delete --help

pipeline-help: ## Show help for pipeline command (alias for help-pipeline)
	@echo -e "$(CYAN)Pipeline Executable Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- --help

pipeline-list-help: ## Show help for pipeline list subcommand
	@echo -e "$(CYAN)Pipeline List Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- list --help

pipeline-process-help: ## Show help for pipeline process subcommand
	@echo -e "$(CYAN)Pipeline Process Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- process --help

pipeline-restore-help: ## Show help for pipeline restore subcommand
	@echo -e "$(CYAN)Pipeline Restore Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- restore --help

pipeline-show-help: ## Show help for pipeline show subcommand
	@echo -e "$(CYAN)Pipeline Show Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- show --help

pipeline-validate-help: ## Show help for pipeline validate subcommand
	@echo -e "$(CYAN)Pipeline Validate Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- validate --help

pipeline-validatefile-help: ## Show help for pipeline validatefile subcommand
	@echo -e "$(CYAN)Pipeline ValidateFile Command Help:$(NC)"
	@echo ""
	@cargo run --bin pipeline -- validatefile --help

##@ Setup & Installation
setup: install-tools ## Setup development environment
	@echo -e "$(GREEN)Setting up development environment...$(NC)"
	@rustup update $(RUST_VERSION)
	@rustup default $(RUST_VERSION)
	@rustup component add clippy rustfmt rust-src
	@echo -e "$(GREEN)✓ Development environment ready!$(NC)"

install-tools: ## Install development tools
	@echo -e "$(BLUE)Installing development tools...$(NC)"
	@cargo install cargo-audit cargo-outdated cargo-bloat cargo-flamegraph
	@cargo install cargo-tarpaulin cargo-watch cargo-expand
	@echo -e "$(GREEN)✓ Tools installed!$(NC)"

##@ Building
build: ## Build the project in debug mode
	@echo -e "$(BLUE)Building project...$(NC)"
	@$(CARGO) build

build-release: ## Build the project in release mode
	@echo -e "$(BLUE)Building project (release)...$(NC)"
	@$(CARGO) build --release

build-all: ## Build all workspace members
	@echo -e "$(BLUE)Building all workspace members...$(NC)"
	@$(CARGO) build --workspace

clean: ## Clean build artifacts
	@echo -e "$(YELLOW)Cleaning build artifacts...$(NC)"
	@$(CARGO) clean
	@rm -rf target/
	@echo -e "$(GREEN)✓ Clean complete!$(NC)"

##@ Testing
test: ## Run all tests
	@echo -e "$(BLUE)Running tests...$(NC)"
	@$(CARGO) test --workspace

test-verbose: ## Run tests with verbose output
	@echo -e "$(BLUE)Running tests (verbose)...$(NC)"
	@$(CARGO) test --workspace -- --nocapture

test-release: ## Run tests in release mode
	@echo -e "$(BLUE)Running tests (release)...$(NC)"
	@$(CARGO) test --workspace --release

test-doc: ## Run documentation tests
	@echo -e "$(BLUE)Running documentation tests...$(NC)"
	@$(CARGO) test --workspace --doc

test-integration: ## Run integration tests only
	@echo -e "$(BLUE)Running integration tests...$(NC)"
	@$(CARGO) test --workspace --test '*'

test-unit: ## Run unit tests only
	@echo -e "$(BLUE)Running unit tests...$(NC)"
	@$(CARGO) test --workspace --lib

coverage: ## Generate test coverage report
	@echo -e "$(BLUE)Generating coverage report...$(NC)"
	@$(CARGO) tarpaulin --workspace --out Html --output-dir target/coverage
	@echo -e "$(GREEN)✓ Coverage report generated in target/coverage/$(NC)"

##@ Code Quality
check: ## Run cargo check
	@echo -e "$(BLUE)Running cargo check...$(NC)"
	@$(CARGO) check --workspace

lint: clippy ## Run clippy linter
clippy: ## Run clippy linter with strict settings
	@echo -e "$(BLUE)Running clippy...$(NC)"
	@$(CARGO) clippy --workspace --all-targets --all-features $(CLIPPY_ARGS)

format: ## Format code with rustfmt
	@echo -e "$(BLUE)Formatting code...$(NC)"
	@$(CARGO) fmt --all $(RUSTFMT_ARGS)

format-check: ## Check code formatting
	@echo -e "$(BLUE)Checking code formatting...$(NC)"
	@$(CARGO) fmt --all $(RUSTFMT_ARGS) -- --check

##@ Documentation
doc: ## Generate documentation
	@echo -e "$(BLUE)Generating documentation...$(NC)"
	@$(CARGO) doc --workspace --no-deps --document-private-items

doc-open: ## Generate and open documentation
	@echo -e "$(BLUE)Generating and opening documentation...$(NC)"
	@$(CARGO) doc --workspace --no-deps --document-private-items --open

##@ Benchmarking
bench: ## Run all benchmarks
	@echo -e "$(BLUE)Running benchmarks...$(NC)"
	@$(CARGO) bench --workspace

bench-file-io: ## Run file I/O benchmarks specifically
	@echo -e "$(BLUE)Running file I/O benchmarks...$(NC)"
	@$(CARGO) bench --bench file_io_benchmark

bench-baseline: ## Run benchmarks and save as baseline
	@echo -e "$(BLUE)Running benchmarks (baseline)...$(NC)"
	@$(CARGO) bench --workspace -- --save-baseline main

##@ Security & Dependencies
audit: ## Run security audit
	@echo -e "$(BLUE)Running security audit...$(NC)"
	@$(CARGO) audit

security: audit ## Alias for audit

update-deps: ## Update dependencies
	@echo -e "$(BLUE)Updating dependencies...$(NC)"
	@$(CARGO) update
	@$(CARGO) outdated --workspace

##@ Performance Analysis
flamegraph: ## Generate flamegraph for performance analysis
	@echo -e "$(BLUE)Generating flamegraph...$(NC)"
	@$(CARGO) flamegraph --bin pipeline

bloat: ## Analyze binary size
	@echo -e "$(BLUE)Analyzing binary size...$(NC)"
	@$(CARGO) bloat --release --crates

##@ Workspace Management
workspace-check: ## Check all workspace members
	@echo -e "$(BLUE)Checking workspace...$(NC)"
	@$(CARGO) check --workspace --all-targets --all-features

pipeline-check: ## Check pipeline crate specifically
	@echo -e "$(BLUE)Checking pipeline crate...$(NC)"
	@cd pipeline && $(CARGO) check --all-targets --all-features

pipelinelib-check: ## Check pipelinelib crate specifically
	@echo -e "$(BLUE)Checking pipelinelib crate...$(NC)"
	@cd pipelinelib && $(CARGO) check --all-targets --all-features

##@ Running
run: ## Run the main pipeline binary
	@echo -e "$(BLUE)Running pipeline...$(NC)"
	@$(CARGO) run --bin pipeline

run-release: ## Run the main pipeline binary in release mode
	@echo -e "$(BLUE)Running pipeline (release)...$(NC)"
	@$(CARGO) run --release --bin pipeline

run-create-db: ## Run the database creation tool
	@echo -e "$(BLUE)Running database creation tool...$(NC)"
	@$(CARGO) run --bin create-test-database

examples: ## Run example code
	@echo -e "$(BLUE)Running examples...$(NC)"
	@$(CARGO) run --example basic_usage 2>/dev/null || echo -e "$(YELLOW)No examples found$(NC)"

##@ Development Workflow
watch: ## Watch for changes and run tests
	@echo -e "$(BLUE)Watching for changes...$(NC)"
	@cargo watch -x "test --workspace"

watch-check: ## Watch for changes and run check
	@echo -e "$(BLUE)Watching for changes (check)...$(NC)"
	@cargo watch -x "check --workspace"

pre-commit: format lint test ## Run pre-commit checks
	@echo -e "$(GREEN)✓ Pre-commit checks passed!$(NC)"

ci-local: clean build test lint doc audit ## Run full CI pipeline locally
	@echo -e "$(GREEN)✓ Local CI pipeline completed!$(NC)"

##@ Release Management
release: ## Build release version
	@echo -e "$(BLUE)Building release...$(NC)"
	@$(CARGO) build --release --workspace

release-check: ## Check release build
	@echo -e "$(BLUE)Checking release build...$(NC)"
	@$(CARGO) check --release --workspace

##@ Cross-Platform Builds
install-cross-targets: ## Install cross-compilation toolchains
	@echo -e "$(BLUE)Installing cross-compilation targets...$(NC)"
	@rustup target add x86_64-unknown-linux-gnu
	@rustup target add aarch64-unknown-linux-gnu
	@rustup target add x86_64-apple-darwin
	@rustup target add aarch64-apple-darwin
	@rustup target add x86_64-pc-windows-msvc
	@echo -e "$(GREEN)✓ Cross-compilation targets installed!$(NC)"

build-linux-x86_64: ## Build for Linux x86_64
	@echo -e "$(BLUE)Building for Linux x86_64...$(NC)"
	@$(CARGO) build --release --target x86_64-unknown-linux-gnu
	@echo -e "$(GREEN)✓ Build complete: target/x86_64-unknown-linux-gnu/release/$(NC)"

build-linux-aarch64: ## Build for Linux ARM64
	@echo -e "$(BLUE)Building for Linux ARM64...$(NC)"
	@$(CARGO) build --release --target aarch64-unknown-linux-gnu
	@echo -e "$(GREEN)✓ Build complete: target/aarch64-unknown-linux-gnu/release/$(NC)"

build-macos-x86_64: ## Build for macOS x86_64 (Intel)
	@echo -e "$(BLUE)Building for macOS x86_64...$(NC)"
	@$(CARGO) build --release --target x86_64-apple-darwin
	@echo -e "$(GREEN)✓ Build complete: target/x86_64-apple-darwin/release/$(NC)"

build-macos-aarch64: ## Build for macOS ARM64 (Apple Silicon)
	@echo -e "$(BLUE)Building for macOS ARM64...$(NC)"
	@$(CARGO) build --release --target aarch64-apple-darwin
	@echo -e "$(GREEN)✓ Build complete: target/aarch64-apple-darwin/release/$(NC)"

build-windows-x86_64: ## Build for Windows x86_64
	@echo -e "$(BLUE)Building for Windows x86_64...$(NC)"
	@$(CARGO) build --release --target x86_64-pc-windows-msvc
	@echo -e "$(GREEN)✓ Build complete: target/x86_64-pc-windows-msvc/release/$(NC)"

build-all-platforms: ## Build for all supported platforms
	@echo -e "$(CYAN)Building for all platforms...$(NC)"
	@$(MAKE) build-linux-x86_64
	@$(MAKE) build-linux-aarch64
	@$(MAKE) build-macos-x86_64
	@$(MAKE) build-macos-aarch64
	@$(MAKE) build-windows-x86_64
	@echo -e "$(GREEN)✓ All platform builds complete!$(NC)"

##@ Docker (if applicable)
docker-build: ## Build Docker image
	@echo -e "$(BLUE)Building Docker image...$(NC)"
	@docker build -t $(PROJECT_NAME) .

docker-run: ## Run Docker container
	@echo -e "$(BLUE)Running Docker container...$(NC)"
	@docker run --rm -it $(PROJECT_NAME)

##@ Utilities
expand: ## Expand macros for debugging
	@echo -e "$(BLUE)Expanding macros...$(NC)"
	@$(CARGO) expand

tree: ## Show dependency tree
	@echo -e "$(BLUE)Showing dependency tree...$(NC)"
	@$(CARGO) tree --workspace

version: ## Show version information
	@echo -e "$(CYAN)Project: $(PROJECT_NAME)$(NC)"
	@echo -e "$(CYAN)Rust version:$(NC) $$(rustc --version)"
	@echo -e "$(CYAN)Cargo version:$(NC) $$(cargo --version)"
	@echo -e "$(CYAN)Clippy version:$(NC) $$(cargo clippy --version)"

##@ Quick Commands
quick-test: ## Quick test (no doc tests)
	@$(CARGO) test --workspace --lib --bins

quick-check: ## Quick check (no features)
	@$(CARGO) check --workspace

dev: watch ## Alias for watch (development mode)

all: clean build test lint doc ## Build, test, lint, and document everything
	@echo -e "$(GREEN)✓ All tasks completed!$(NC)"
