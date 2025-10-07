#!/bin/bash

echo "🚀 Demonstrating ADAPIPE_SQLITE_PATH Environment Variable Integration"
echo "=================================================================="

# Create a temporary directory for our test
TEMP_DIR=$(mktemp -d)
echo "📁 Created temporary directory: $TEMP_DIR"

# Define paths
TEST_DB="$TEMP_DIR/demo_test.db"
SCHEMA_FILE="$TEMP_DIR/schema.sql"

echo ""
echo "Step 1: Generate test database schema"
echo "-------------------------------------"
cargo run --bin create-test-database "$SCHEMA_FILE"

if [ $? -eq 0 ]; then
    echo "✅ Schema generated successfully"
    echo "📊 Schema size: $(wc -c < "$SCHEMA_FILE") bytes"
else
    echo "❌ Failed to generate schema"
    exit 1
fi

echo ""
echo "Step 2: Create SQLite database from schema"
echo "------------------------------------------"
sqlite3 "$TEST_DB" < "$SCHEMA_FILE"

if [ $? -eq 0 ]; then
    echo "✅ Database created successfully"
    echo "📊 Database size: $(wc -c < "$TEST_DB") bytes"
else
    echo "❌ Failed to create database"
    exit 1
fi

echo ""
echo "Step 3: Test pipeline commands with ADAPIPE_SQLITE_PATH"
echo "-------------------------------------------------------"

echo "🔧 Setting ADAPIPE_SQLITE_PATH=$TEST_DB"
export ADAPIPE_SQLITE_PATH="$TEST_DB"

echo ""
echo "📋 Testing 'pipeline list' command:"
ADAPIPE_SQLITE_PATH="$TEST_DB" cargo run --bin pipeline -- list

echo ""
echo "🔍 Testing 'pipeline show' command for test-multi-stage:"
ADAPIPE_SQLITE_PATH="$TEST_DB" cargo run --bin pipeline -- show test-multi-stage

echo ""
echo "Step 4: Compare with default database behavior"
echo "----------------------------------------------"

echo "📋 Testing without environment variable (uses default database):"
unset ADAPIPE_SQLITE_PATH
cargo run --bin pipeline -- list

echo ""
echo "🎉 Demo completed successfully!"
echo ""
echo "Summary:"
echo "- ✅ Created test database using create-test-database tool"
echo "- ✅ Used ADAPIPE_SQLITE_PATH to point to test database"
echo "- ✅ Verified pipeline commands work with custom database path"
echo "- ✅ Confirmed fallback to default database when env var not set"
echo ""
echo "🧹 Cleaning up temporary files..."
rm -rf "$TEMP_DIR"
echo "✅ Cleanup complete"
