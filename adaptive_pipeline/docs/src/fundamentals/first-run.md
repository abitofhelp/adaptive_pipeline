# Running Your First Pipeline

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Prerequisites

Before running your first pipeline, ensure you have:

- **Pipeline binary** - Built and available in your PATH
  ```bash
  cargo build --release
  cp target/release/pipeline /usr/local/bin/  # or add to PATH
  ```

- **Test file** - A sample file to process
  ```bash
  echo "Hello, Pipeline World!" > test.txt
  ```

- **Permissions** - Read/write access to input and output directories

## Quick Start (5 minutes)

Let's run a simple compression and encryption pipeline in 3 steps:

### Step 1: Create a Pipeline

```bash
pipeline create \
  --name my-first-pipeline \
  --stages compression,encryption
```

You should see output like:
```
✓ Created pipeline: my-first-pipeline
  Stages: compression (zstd), encryption (aes-256-gcm)
```

### Step 2: Process a File

```bash
pipeline process \
  --input test.txt \
  --output test.bin \
  --pipeline my-first-pipeline
```

You should see progress output:
```
Processing: test.txt
Pipeline: my-first-pipeline
  Stage 1/2: Compression (zstd)... ✓
  Stage 2/2: Encryption (aes-256-gcm)... ✓
Output: test.bin (24 bytes)
Time: 0.05s
```

### Step 3: Restore the File

```bash
pipeline restore \
  --input test.bin \
  --output restored.txt
```

Verify the restoration:
```bash
diff test.txt restored.txt
# No output = files are identical ✓
```

## Detailed Walkthrough

Let's explore each step in more detail.

### Creating Pipelines

#### Basic Pipeline
```bash
pipeline create \
  --name basic \
  --stages compression
```

This creates a simple compression-only pipeline using default settings (zstd compression).

#### Secure Pipeline
```bash
pipeline create \
  --name secure \
  --stages compression,encryption,integrity
```

This creates a complete security pipeline with:
- Compression (reduces size)
- Encryption (protects data)
- Integrity verification (detects tampering)

#### Save Pipeline Configuration
```bash
pipeline create \
  --name archival \
  --stages compression,encryption \
  --output archival-pipeline.toml
```

This saves the pipeline configuration to a file for reuse.

### Processing Files

#### Basic Processing
```bash
# Process a file
pipeline process \
  --input large-file.log \
  --output large-file.bin \
  --pipeline secure
```

#### With Performance Options
```bash
# Process with custom settings
pipeline process \
  --input large-file.log \
  --output large-file.bin \
  --pipeline secure \
  --cpu-threads 8 \
  --chunk-size-mb 32
```

#### With Verbose Logging
```bash
# See detailed progress
pipeline --verbose process \
  --input large-file.log \
  --output large-file.bin \
  --pipeline secure
```

### Restoring Files

The pipeline automatically detects the processing stages from the output file's metadata:

```bash
# Restore automatically reverses all stages
pipeline restore \
  --input large-file.bin \
  --output restored-file.log
```

The system will:
1. Read metadata from the file header
2. Apply stages in reverse order
3. Verify integrity if available
4. Restore original file

### Managing Pipelines

#### List All Pipelines
```bash
pipeline list
```

Output:
```
Available Pipelines:
  - my-first-pipeline (compression, encryption)
  - secure (compression, encryption, integrity)
  - archival (compression, encryption)
```

#### Show Pipeline Details
```bash
pipeline show secure
```

Output:
```
Pipeline: secure
  Stage 1: Compression (zstd)
  Stage 2: Encryption (aes-256-gcm)
  Stage 3: Integrity (sha256)
Created: 2025-01-04 10:30:00
```

#### Delete a Pipeline
```bash
pipeline delete my-first-pipeline --force
```

## Understanding Output

### Successful Processing

When processing completes successfully:
```
Processing: test.txt
Pipeline: my-first-pipeline
  Stage 1/2: Compression (zstd)... ✓
  Stage 2/2: Encryption (aes-256-gcm)... ✓

Statistics:
  Input size:  1,024 KB
  Output size: 512 KB
  Compression ratio: 50%
  Processing time: 0.15s
  Throughput: 6.8 MB/s

Output: test.bin
```

### Performance Metrics

With `--verbose` flag, you'll see detailed metrics:
```
Pipeline Execution Metrics:
  Chunks processed: 64
  Parallel workers: 8
  Average chunk time: 2.3ms
  CPU utilization: 87%
  I/O wait: 3%

Stage Breakdown:
  Compression: 0.08s (53%)
  Encryption: 0.05s (33%)
  I/O: 0.02s (14%)
```

### Error Messages

#### File Not Found
```
Error: Input file not found: test.txt
  Check the file path and try again
```

#### Permission Denied
```
Error: Permission denied: /protected/output.bin
  Ensure you have write access to the output directory
```

#### Invalid Pipeline
```
Error: Pipeline not found: nonexistent
  Use 'pipeline list' to see available pipelines
```

## Common Scenarios

### Scenario 1: Compress Large Log Files
```bash
# Create compression pipeline
pipeline create --name logs --stages compression

# Process log files
pipeline process \
  --input app.log \
  --output app.log.bin \
  --pipeline logs \
  --chunk-size-mb 64

# Compression ratio is typically 70-90% for text logs
```

### Scenario 2: Secure Sensitive Files
```bash
# Create secure pipeline with all protections
pipeline create --name sensitive --stages compression,encryption,integrity

# Process sensitive file
pipeline process \
  --input customer-data.csv \
  --output customer-data.bin \
  --pipeline sensitive

# File is now compressed, encrypted, and tamper-evident
```

### Scenario 3: High-Performance Batch Processing
```bash
# Process multiple files with optimized settings
for file in data/*.csv; do
  pipeline process \
    --input "$file" \
    --output "processed/$(basename $file).bin" \
    --pipeline fast \
    --cpu-threads 16 \
    --chunk-size-mb 128 \
    --channel-depth 16
done
```

### Scenario 4: Restore and Verify
```bash
# Restore file
pipeline restore \
  --input customer-data.bin \
  --output customer-data-restored.csv

# Verify restoration
sha256sum customer-data.csv customer-data-restored.csv
# Both checksums should match
```

## Testing Your Pipeline

### Create Test Data
```bash
# Create a test file
dd if=/dev/urandom of=test-10mb.bin bs=1M count=10

# Calculate original checksum
sha256sum test-10mb.bin > original.sha256
```

### Process and Restore
```bash
# Process the file
pipeline process \
  --input test-10mb.bin \
  --output test-10mb.processed \
  --pipeline my-first-pipeline

# Restore the file
pipeline restore \
  --input test-10mb.processed \
  --output test-10mb.restored
```

### Verify Integrity
```bash
# Verify restored file matches original
sha256sum -c original.sha256
# Should output: test-10mb.bin: OK
```

## Next Steps

Congratulations! You've run your first pipeline. Now you can:

- **Explore Advanced Features**
  - [Architecture Overview](../architecture/overview.md) - Understand the system design
  - [Implementation Details](../implementation/compression.md) - Learn about algorithms
  - [Performance Tuning](../advanced/performance.md) - Optimize for your use case

- **Learn More About Configuration**
  - [Configuration Guide](configuration.md) - Detailed configuration options
  - [Stage Types](stages.md) - Available processing stages

- **Build Custom Pipelines**
  - Experiment with different stage combinations
  - Test different algorithms for your workload
  - Benchmark performance with your data
