# Database Setup Quick Reference

## TL;DR - Quick Setup

```bash
# 1. Generate database with proper ULID IDs
cd pipeline
cargo run --example generate_test_database_demo

# 2. Create the database
cd ..
sqlite3 pipeline/scripts/test_data/structured_pipeline.db < pipeline/scripts/test_data/generated_database.sql

# 3. Build and test
cargo build --release
./target/release/pipeline list
```

## Why This Process?

The database setup process uses our **demo generator** to ensure:

1. **Proper ULID Format**: All IDs are generated using our domain value objects (`PipelineId::new()`, `StageId::new()`)
2. **DDD Compliance**: Maintains Domain-Driven Design principles
3. **Consistency**: Single source of truth for ID generation
4. **Production-Grade**: Uses the same ID generation logic as the main application

## Generated Pipelines

The setup creates two test pipelines:

### `test-multi-stage`
- **Stages**: input_checksum → compression → encryption → output_checksum
- **Use Case**: Full pipeline testing with all processing stages
- **Good for**: Testing atomic counters, progress tracking, multi-stage processing

### `image-processing`
- **Stages**: input_validation → image_compression
- **Use Case**: Simpler pipeline for basic testing
- **Good for**: Quick validation, basic functionality testing

## Verification Commands

```bash
# List available pipelines
./target/release/pipeline list

# Test processing with small file
echo "test data" > test.txt
./target/release/pipeline process --input test.txt --output test.adapipe --pipeline "test-multi-stage"

# Check database directly
sqlite3 pipeline/scripts/test_data/structured_pipeline.db "SELECT id, name FROM pipelines;"
```

## Troubleshooting

**Database connection errors?**
- Ensure you're in the project root directory
- Verify the database file exists: `ls -la pipeline/scripts/test_data/structured_pipeline.db`
- Recreate using the steps above

**Invalid ID format errors?**
- Always use the demo generator - never create IDs manually
- IDs must be 26-character ULID format (e.g., `01JZSKVRFN0T505E2MJC6JW0MT`)

## Full Documentation

See `pipeline/scripts/test_data/README.md` for complete documentation including:
- Detailed schema information
- Manual database creation methods
- Advanced troubleshooting
- Development workflow
- Database maintenance procedures
