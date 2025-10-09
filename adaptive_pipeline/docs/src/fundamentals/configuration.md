# Configuration Basics

**Version:** 1.0
**Date:** October 08, 2025
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The pipeline system provides flexible configuration through command-line options, environment variables, and configuration files. This chapter covers the basics of configuring your pipelines.

## Command-Line Interface

The pipeline CLI provides several commands for managing and running pipelines.

### Basic Commands

#### Process a File
```bash
pipeline process \
  --input /path/to/input.txt \
  --output /path/to/output.bin \
  --pipeline my-pipeline
```

#### Create a Pipeline
```bash
pipeline create \
  --name my-pipeline \
  --stages compression,encryption,integrity
```

#### List Pipelines
```bash
pipeline list
```

#### Show Pipeline Details
```bash
pipeline show my-pipeline
```

#### Delete a Pipeline
```bash
pipeline delete my-pipeline --force
```

### Performance Options

#### CPU Threads
Control the number of worker threads for CPU-bound operations (compression, encryption):

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline \
  --cpu-threads 8
```

**Default:** Number of CPU cores - 1 (reserves one core for I/O)

**Tips:**
- Too high: CPU thrashing, context switching overhead
- Too low: Underutilized cores, slower processing
- Monitor CPU saturation metrics to tune

#### I/O Threads
Control the number of concurrent I/O operations:

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline \
  --io-threads 24
```

**Default:** Device-specific (NVMe: 24, SSD: 12, HDD: 4)

**Storage Type Detection:**
```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline \
  --storage-type nvme  # or ssd, hdd
```

#### Channel Depth
Control backpressure in the pipeline stages:

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline \
  --channel-depth 8
```

**Default:** 4

**Tips:**
- Lower values: Less memory, may cause pipeline stalls
- Higher values: More buffering, higher memory usage
- Optimal value depends on chunk processing time and I/O latency

#### Chunk Size
Configure the size of file chunks for parallel processing:

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline \
  --chunk-size-mb 10
```

**Default:** Automatically determined based on file size and available resources

### Global Options

#### Verbose Logging
Enable detailed logging output:

```bash
pipeline --verbose process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline
```

#### Configuration File
Use a custom configuration file:

```bash
pipeline --config /path/to/config.toml process \
  --input file.txt \
  --output file.bin \
  --pipeline my-pipeline
```

## Configuration Files

Configuration files use TOML format and allow you to save pipeline settings for reuse.

### Basic Configuration
```toml
[pipeline]
name = "my-pipeline"
stages = ["compression", "encryption", "integrity"]

[performance]
cpu_threads = 8
io_threads = 24
channel_depth = 4

[processing]
chunk_size_mb = 10
```

### Algorithm Configuration
```toml
[stages.compression]
algorithm = "zstd"

[stages.encryption]
algorithm = "aes-256-gcm"
key_file = "/path/to/keyfile"

[stages.integrity]
algorithm = "sha256"
```

### Complete Example
```toml
# Pipeline configuration example
[pipeline]
name = "secure-archival"
description = "High compression with encryption for archival"

[stages.compression]
algorithm = "brotli"
level = 11  # Maximum compression

[stages.encryption]
algorithm = "aes-256-gcm"
key_derivation = "argon2"

[stages.integrity]
algorithm = "blake3"

[performance]
cpu_threads = 16
io_threads = 24
channel_depth = 8
storage_type = "nvme"

[processing]
chunk_size_mb = 64
parallel_workers = 16
```

### Using Configuration Files
```bash
# Use a configuration file
pipeline --config secure-archival.toml process \
  --input large-dataset.tar \
  --output large-dataset.bin

# Override configuration file settings
pipeline --config secure-archival.toml \
  --cpu-threads 8 \
  process --input file.txt --output file.bin
```

## Environment Variables

Environment variables provide another way to configure the pipeline:

```bash
# Set performance defaults
export PIPELINE_CPU_THREADS=8
export PIPELINE_IO_THREADS=24
export PIPELINE_CHANNEL_DEPTH=8

# Set default chunk size
export PIPELINE_CHUNK_SIZE_MB=10

# Enable verbose logging
export PIPELINE_VERBOSE=true

# Run pipeline
pipeline process --input file.txt --output file.bin --pipeline my-pipeline
```

## Configuration Priority

When the same setting is configured in multiple places, the following priority applies (highest to lowest):

1. **Command-line arguments** - Explicit flags like `--cpu-threads`
2. **Environment variables** - `PIPELINE_*` variables
3. **Configuration file** - Settings from `--config` file
4. **Default values** - Built-in intelligent defaults

Example:
```bash
# Config file says cpu_threads = 8
# Environment says PIPELINE_CPU_THREADS=12
# Command line says --cpu-threads=16

# Result: Uses 16 (command-line wins)
```

## Performance Tuning Guidelines

### For Maximum Speed
- Use LZ4 compression
- Use ChaCha20-Poly1305 encryption
- Increase CPU threads to match cores
- Use large chunks (32-64 MB)
- Higher channel depth (8-16)

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline speed-pipeline \
  --cpu-threads 16 \
  --chunk-size-mb 64 \
  --channel-depth 16
```

### For Maximum Compression
- Use Brotli compression
- Smaller chunks for better compression ratio
- More CPU threads for parallel compression

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline compression-pipeline \
  --cpu-threads 16 \
  --chunk-size-mb 4
```

### For Resource-Constrained Systems
- Reduce CPU and I/O threads
- Smaller chunks
- Lower channel depth

```bash
pipeline process \
  --input file.txt \
  --output file.bin \
  --pipeline minimal-pipeline \
  --cpu-threads 2 \
  --io-threads 4 \
  --chunk-size-mb 2 \
  --channel-depth 2
```

## Next Steps

Now that you understand configuration, you're ready to:

- [Run Your First Pipeline](first-run.md) - Step-by-step tutorial
- [Learn About Stages](stages.md) - Deep dive into pipeline stages
- [Explore Architecture](../architecture/overview.md) - Understand the system design
