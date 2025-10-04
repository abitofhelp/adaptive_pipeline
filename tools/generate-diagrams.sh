#!/bin/bash
# Generate SVG diagrams from PlantUML sources

set -e  # Exit on error

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PLANTUML_JAR="${PROJECT_ROOT}/tools/plantuml.jar"

if [ ! -f "$PLANTUML_JAR" ]; then
    echo "Error: PlantUML JAR not found at $PLANTUML_JAR"
    exit 1
fi

echo "Generating diagrams..."

# Generate main docs diagrams
if [ -d "${PROJECT_ROOT}/docs/diagrams" ]; then
    echo "  Processing docs/diagrams/*.puml"
    java -jar "$PLANTUML_JAR" -tsvg "${PROJECT_ROOT}/docs/diagrams/*.puml" 2>/dev/null || true
fi

# Generate pipeline docs diagrams
if [ -d "${PROJECT_ROOT}/pipeline/docs/diagrams" ]; then
    echo "  Processing pipeline/docs/diagrams/*.puml"
    java -jar "$PLANTUML_JAR" -tsvg "${PROJECT_ROOT}/pipeline/docs/diagrams/*.puml" 2>/dev/null || true
fi

echo "Done! Diagrams generated as SVG files."
