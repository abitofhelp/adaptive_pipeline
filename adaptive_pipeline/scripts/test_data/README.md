<!--
Adaptive Pipeline
Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
SPDX-License-Identifier: BSD-3-Clause
See LICENSE file in the project root.
-->

# Database Management Guide

This guide explains how to create, load, and manage the SQLite database for the Adaptive Pipeline system.

## Overview

The pipeline system uses SQLite for persistent storage of pipeline configurations, processing metrics, and session data. The database follows strict Domain-Driven Design (DDD) principles and uses proper ULID format for all identifiers as defined by our value objects.

## Database Schema

The database consists of the following tables:

- **`pipelines`**: Core pipeline definitions with ULID identifiers
- **`pipeline_configuration`**: Key-value configuration for each pipeline
- **`pipeline_stages`**: Individual processing stages within pipelines
- **`stage_parameters`**: Parameters for each stage
- **`processing_metrics`**: Performance and processing statistics
- **`processing_sessions`**: Session tracking for pipeline executions
- **`file_chunks`**: Chunk-level processing information
- **`security_contexts`**: Encryption and security configurations

## Database Creation Process

### Method 1: Using the Demo Generator (Recommended)

The recommended approach uses our demo application that generates proper ULID identifiers using our domain value objects.

#### Step 1: Generate Database with Proper IDs

```bash
# Navigate to the pipeline directory
cd pipeline

# Run the database generator demo
cargo run --example generate_test_database_demo
```

This will:
- Generate proper ULID identifiers using our `PipelineId` and `StageId` value objects
- Create a SQL script at `scripts/test_data/generated_database.sql`
- Display the generated IDs for verification

Example output:
```
🔧 Generating test database with proper ID value objects...
📋 Generated Pipeline IDs:
  test-multi-stage: 01JZSKVRFN0T505E2MJC6JW0MT
  image-processing: 01JZSKVRFN3F66XKMGHDJYV84W

🔧 Generated Stage IDs for test-multi-stage:
  input_checksum: 01JZSKVRFNT2G79CPBJ0FMCPAS
  compression: 01JZSKVRFNXY50DXDYD14B63ZR
  encryption: 01JZSKVRFN0TQ0GWVVNFKH9Z74
  output_checksum: 01JZSKVRFN4YMGRQJE9HRQ1H4Z
```

#### Step 2: Create the Database

```bash
# Navigate to the project root
cd ..

# Create the database using the generated SQL
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql
```

#### Step 3: Verify Database Creation

```bash
# Check that tables were created and populated
sqlite3 pipeline/scripts/test_data/structured_pipeline.db "SELECT name FROM sqlite_master WHERE type='table';"

# Verify pipeline data
sqlite3 pipeline/scripts/test_data/structured_pipeline.db "SELECT id, name FROM pipelines;"
```

### Method 2: Manual Database Creation

If you need to create the database manually or with custom data:

#### Step 1: Delete Existing Database

```bash
rm -f pipeline/scripts/test_data/structured_pipeline.db
```

#### Step 2: Create Database Schema

```bash
# Use one of the existing schema files
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/create_structured_database.sql
```

**Important**: Ensure all IDs in the SQL file use proper ULID format (26 characters, base32 encoded).

## Database Loading and Verification

### Verify Database Connection

```bash
# Test the database connection through the application
./target/release/pipeline list
```

Expected output:
```
2025-07-10T07:31:35.423160Z  INFO pipeline: Listing available pipelines:
Found 2 pipeline(s):

Pipeline: image-processing
  ID: 01JZSKVRFN3F66XKMGHDJYV84W
  Status: Active
  Stages: 2

Pipeline: test-multi-stage
  ID: 01JZSKVRFN0T505E2MJC6JW0MT
  Status: Active
  Stages: 4
```

### Test Pipeline Processing

```bash
# Create a test file
echo "Test data for pipeline processing" > test_input.txt

# Process the file using a pipeline
./target/release/pipeline process --input test_input.txt --output test_output.adapipe --pipeline "test-multi-stage"
```

## Database File Locations

- **Production Database**: `pipeline/scripts/test_data/structured_pipeline.db`
- **Test Database**: `pipeline/scripts/test_data/test_pipeline.db`
- **Generated SQL Scripts**: `pipeline/scripts/test_data/generated_database.sql`

## ID Format Requirements

All database identifiers must follow ULID format:
- **Length**: 26 characters
- **Encoding**: Base32 (Crockford's Base32)
- **Format**: `01JZSKVRFN0T505E2MJC6JW0MT`
- **Generation**: Use our value objects (`PipelineId::new()`, `StageId::new()`)

## Troubleshooting

### Database Connection Issues

If you encounter "unable to open database file" errors:

1. **Check file permissions**:
   ```bash
   ls -la pipeline/scripts/test_data/structured_pipeline.db
   ```

2. **Verify working directory**:
   ```bash
   pwd  # Should be in project root
   ```

3. **Recreate database**:
   ```bash
   rm -f pipeline/scripts/test_data/structured_pipeline.db
   cargo run --example generate_test_database_demo
   sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql
   ```

### Invalid ID Format

If you see ID-related errors:
- Ensure all IDs are generated using our value objects
- Verify ULID format (26 characters, base32)
- Use the demo generator to create proper IDs

### Schema Mismatch

If the application fails to read pipeline data:
- Check that the database schema matches the expected structure
- Verify all required tables exist
- Ensure foreign key relationships are properly defined

## Best Practices

1. **Always use the demo generator** for creating databases with proper IDs
2. **Backup databases** before making changes
3. **Test database connectivity** after creation
4. **Use proper ULID format** for all identifiers
5. **Follow DDD principles** when modifying schema
6. **Maintain referential integrity** between tables

## Development Workflow

For development and testing:

```bash
# 1. Generate fresh database
cargo run --example generate_test_database_demo

# 2. Create database
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql

# 3. Build application
cargo build --release

# 4. Test database connection
./target/release/pipeline list

# 5. Test pipeline processing
echo "test" > test.txt
./target/release/pipeline process --input test.txt --output test.adapipe --pipeline "test-multi-stage"
```

## Database Maintenance

### Backup Database

```bash
# Create backup
cp pipeline/scripts/test_data/structured_pipeline.db pipeline/scripts/test_data/structured_pipeline.db.backup
```

### Reset Database

```bash
# Delete and recreate
rm -f pipeline/scripts/test_data/structured_pipeline.db
cargo run --example generate_test_database_demo
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql
```

### Inspect Database

```bash
# Open SQLite CLI
sqlite3 pipeline/scripts/test_data/structured_pipeline.db

# Common queries
.tables                                    # List all tables
.schema pipelines                         # Show table schema
SELECT * FROM pipelines;                  # Show all pipelines
SELECT * FROM pipeline_stages;            # Show all stages
```

This process ensures that the database is created with proper ULID identifiers that match our domain value objects, maintaining consistency with our Domain-Driven Design architecture.
