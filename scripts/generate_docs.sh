#!/bin/bash
# Generate Documentation
# Generuje dokumentację projektu

set -e

echo "📚 VantisOS - Generate Documentation"
echo "====================================="

DOCS_DIR="docs"
OUTPUT_DIR="target/doc"

echo "Generating Rust documentation..."
cargo doc --workspace --no-deps

echo ""
echo "Generating API documentation..."
if [[ -d "docs/api" ]]; then
    echo "API docs already present in docs/api/"
fi

echo ""
echo "Generating architecture documentation..."
if [[ -f "docs/architecture/ARCHITECTURE.md" ]]; then
    echo "Architecture docs already present"
fi

echo ""
echo "✅ Documentation generated!"
echo "Open: target/doc/vantis/index.html"
