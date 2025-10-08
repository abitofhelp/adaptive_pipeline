# Pipeline Developer Guide

**Version:** 1.0.2
**Date:** October 7, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner
**Status:** Draft

## Welcome

This is the comprehensive technical guide for the Adaptive Pipeline. Whether you're learning advanced Rust patterns, contributing to the project, or using the pipeline in production, this guide provides the depth you need.

## How to Use This Guide

This guide follows a **progressive disclosure** approach - each section builds on previous ones:

### Start Here: Fundamentals

If you're new to the pipeline, start with **Fundamentals**. This section introduces core concepts in an accessible way:

- What pipelines do and why they're useful
- Key terminology and concepts
- How stages work together
- Basic configuration
- Running your first pipeline

**Time commitment:** 30-45 minutes

### Building Understanding: Architecture

Once you understand the basics, explore the **Architecture** section. This explains *how* the pipeline is designed:

- Layered architecture (Domain, Application, Infrastructure)
- Domain-Driven Design concepts
- Design patterns in use (Repository, Service, Adapter, Observer)
- Dependency management

This section bridges the gap between basic usage and implementation details.

**Time commitment:** 1-2 hours

### Going Deeper: Implementation

The **Implementation** section covers how specific features work:

- Stage processing details
- Compression and encryption
- Data persistence and schema management
- File I/O and chunking
- Metrics and observability

Perfect for contributors or those adapting the pipeline for specific needs.

**Time commitment:** 2-3 hours

### Expert Level: Advanced Topics

For optimization and extension, the **Advanced Topics** section covers:

- Concurrency model and thread pooling
- Performance optimization techniques
- Creating custom stages and algorithms

**Time commitment:** 2-4 hours depending on depth

### Reference: Formal Documentation

The **Formal Documentation** section contains:

- Software Requirements Specification (SRS)
- Software Design Document (SDD)
- Test Strategy (STP)

These are comprehensive reference documents.

## Documentation Scope

Following our **"reasonable" principle**, this guide focuses on:

✅ **What you need to know** to use, contribute to, or extend the pipeline
✅ **Why decisions were made** with just enough context
✅ **How to accomplish tasks** with practical examples
✅ **Advanced Rust patterns** demonstrated in real code

We intentionally **do not** include:

❌ Rust language tutorials (see [The Rust Book](https://doc.rust-lang.org/book/))
❌ General programming concepts
❌ Third-party library documentation (links provided instead)
❌ Exhaustive algorithm details (high-level explanations with references)

## Learning Path Recommendations

### I want to use the pipeline

→ Read [Fundamentals](fundamentals/what-is-a-pipeline.md)
→ Skip to [Implementation](implementation/stages.md) for specific features

### I want to learn advanced Rust patterns

→ Focus on Architecture section (patterns)
→ Review Implementation for real-world examples
→ Study Advanced Topics for concurrency/performance

### I'm building something similar

→ Read Architecture + Implementation
→ Study formal documentation (SRS/SDD)
→ Review source code with this guide as reference

## Conventions Used

Throughout this guide:

- **Code examples** are complete and runnable unless marked otherwise
- **File paths** use format `module/file.rs:line` for source references
- **Diagrams** are in PlantUML (SVG rendered in book)
- **Callouts** highlight important information:

> **Note:** Additional helpful information

> **Warning:** Important caveats or gotchas

> **Example:** Practical code demonstration

## Quick Links

- [User Guide](../index.html) - Getting started and quick reference
- [GitHub Repository](https://github.com/abitofhelp/optimized_adaptive_pipeline_rs)
- [API Documentation](https://docs.rs/adaptive-pipeline)

## Ready to Start?

Choose your path:

- **New users:** [What is a Pipeline?](fundamentals/what-is-a-pipeline.md)
- **Developers:** [Architecture Overview](architecture/overview.md)
- **Specific feature:** Use search (press 's') or browse [table of contents](#)

Let's dive in!
