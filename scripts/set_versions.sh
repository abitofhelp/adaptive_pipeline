#!/bin/bash
# Set version numbers across all project files
# Usage: ./scripts/set_versions.sh <version> [date]
# Example: ./scripts/set_versions.sh 1.0.4 "October 7, 2025"

set -e  # Exit on error

VERSION="$1"
DATE="${2:-$(date '+%B %d, %Y')}"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version> [date]"
    echo "Example: $0 1.0.4 \"October 7, 2025\""
    exit 1
fi

# Validate version format (semantic versioning)
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "Error: Version must be in format X.Y.Z (e.g., 1.0.4)"
    exit 1
fi

cd /Users/mike/Rust/src/github.com/abitofhelp/optimized_adaptive_pipeline_rs

echo "Setting version to v${VERSION} (${DATE})..."
echo ""

# Update Cargo.toml files
echo "Updating Cargo.toml files..."

# adaptive_pipeline/Cargo.toml - main version
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" adaptive_pipeline/Cargo.toml

# adaptive_pipeline/Cargo.toml - dependency versions
sed -i '' "s/adaptive-pipeline-domain = { path = \"\.\.\/adaptive_pipeline_domain\", version = \".*\" }/adaptive-pipeline-domain = { path = \"..\/adaptive_pipeline_domain\", version = \"${VERSION}\" }/" adaptive_pipeline/Cargo.toml
sed -i '' "s/adaptive-pipeline-bootstrap = { path = \"\.\.\/adaptive_pipeline_bootstrap\", version = \".*\" }/adaptive-pipeline-bootstrap = { path = \"..\/adaptive_pipeline_bootstrap\", version = \"${VERSION}\" }/" adaptive_pipeline/Cargo.toml

# adaptive_pipeline_domain/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" adaptive_pipeline_domain/Cargo.toml

# adaptive_pipeline_bootstrap/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" adaptive_pipeline_bootstrap/Cargo.toml

echo "  ✓ adaptive_pipeline/Cargo.toml"
echo "  ✓ adaptive_pipeline_domain/Cargo.toml"
echo "  ✓ adaptive_pipeline_bootstrap/Cargo.toml"
echo ""

# Update documentation files
echo "Updating documentation files..."

# User guide introduction
sed -i '' "s/^\*\*Version:\*\* .*/\*\*Version:\*\* ${VERSION}/" docs/src/introduction.md
sed -i '' "s/^\*\*Date:\*\* .*/\*\*Date:\*\* ${DATE}/" docs/src/introduction.md
echo "  ✓ docs/src/introduction.md"

# Developer guide introduction
sed -i '' "s/^\*\*Version:\*\* .*/\*\*Version:\*\* ${VERSION}/" adaptive_pipeline/docs/src/introduction.md
sed -i '' "s/^\*\*Date:\*\* .*/\*\*Date:\*\* ${DATE}/" adaptive_pipeline/docs/src/introduction.md
echo "  ✓ adaptive_pipeline/docs/src/introduction.md"

# Update all documentation markdown files with version headers
# This catches files that were missed in previous releases (0.1.0 -> current)
find adaptive_pipeline/docs/src -name "*.md" -type f -exec sed -i '' "s/^\*\*Version:\*\* [0-9]\+\.[0-9]\+\.[0-9]\+/\*\*Version:\*\* ${VERSION}/" {} \;
echo "  ✓ All adaptive_pipeline/docs/src/**/*.md files (version headers)"

# Update roadmap
sed -i '' "s/^\*\*Version\*\*: [0-9]\+\.[0-9]\+\.[0-9]\+/\*\*Version\*\*: ${VERSION}/" docs/roadmap.md
echo "  ✓ docs/roadmap.md"

echo ""

echo "✅ Version updated to v${VERSION} (${DATE})"
echo ""
echo "Files modified:"
echo "  • adaptive_pipeline/Cargo.toml (3 version strings)"
echo "  • adaptive_pipeline_domain/Cargo.toml"
echo "  • adaptive_pipeline_bootstrap/Cargo.toml"
echo "  • docs/src/introduction.md (version + date)"
echo "  • adaptive_pipeline/docs/src/introduction.md (version + date)"
echo "  • adaptive_pipeline/docs/src/**/*.md (~30 files with version headers)"
echo "  • docs/roadmap.md (version)"
echo ""
echo "Note: This script now comprehensively updates ALL documentation files,"
echo "      catching any that were missed in previous releases (0.1.0 -> current)."
echo ""
echo "Next steps:"
echo "  1. Build the project: cargo build --release"
echo "  2. Verify version: ./target/release/adaptive_pipeline --version"
echo "  3. Update CHANGELOG.md if needed"
echo "  4. Commit: git add -A && git commit -m 'chore: bump version to v${VERSION}'"
echo "  5. Tag: git tag -a v${VERSION} -m 'Release v${VERSION}'"
echo "  6. Push: git push && git push --tags"
