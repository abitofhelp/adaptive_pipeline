#!/bin/bash

# Initialize a test database with the proper schema
# Usage: ./init_test_db.sh [database_path]

set -e

DB_PATH="${1:-/tmp/test_pipeline.db}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCHEMA_FILE="$SCRIPT_DIR/schema.sql"

echo "Initializing test database: $DB_PATH"
echo "Using schema: $SCHEMA_FILE"

# Remove existing database if it exists
if [ -f "$DB_PATH" ]; then
    echo "Removing existing database..."
    rm "$DB_PATH"
fi

# Create new database with schema
echo "Creating database schema..."
sqlite3 "$DB_PATH" < "$SCHEMA_FILE"

echo "✅ Test database initialized successfully: $DB_PATH"
echo "Tables created:"
sqlite3 "$DB_PATH" ".tables"
