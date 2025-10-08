#!/bin/bash
# Create release zip files for all platforms

set -e  # Exit on error

VERSION="$1"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 1.0.2"
    exit 1
fi

cd /Users/mike/Rust/src/github.com/abitofhelp/optimized_adaptive_pipeline_rs

echo "Creating release archives for v${VERSION}..."

# macOS ARM64 (Apple Silicon)
echo "  • macOS ARM64..."
zip -j target/aarch64-apple-darwin/release/adaptive_pipeline-v${VERSION}-macos-aarch64.zip \
    target/aarch64-apple-darwin/release/adaptive_pipeline

# Linux ARM64
echo "  • Linux ARM64..."
zip -j target/aarch64-unknown-linux-gnu/release/adaptive_pipeline-v${VERSION}-linux-aarch64.zip \
    target/aarch64-unknown-linux-gnu/release/adaptive_pipeline

# macOS Intel
echo "  • macOS x86_64..."
zip -j target/x86_64-apple-darwin/release/adaptive_pipeline-v${VERSION}-macos-x86_64.zip \
    target/x86_64-apple-darwin/release/adaptive_pipeline

# Windows x86_64
echo "  • Windows x86_64..."
zip -j target/x86_64-pc-windows-gnu/release/adaptive_pipeline-v${VERSION}-windows-x86_64.zip \
    target/x86_64-pc-windows-gnu/release/adaptive_pipeline.exe

# Linux x86_64
echo "  • Linux x86_64..."
zip -j target/x86_64-unknown-linux-gnu/release/adaptive_pipeline-v${VERSION}-linux-x86_64.zip \
    target/x86_64-unknown-linux-gnu/release/adaptive_pipeline

echo ""
echo "✅ All v${VERSION} release archives created!"
echo ""
echo "Created files:"
ls -lh target/*/release/adaptive_pipeline-v${VERSION}-*.zip
