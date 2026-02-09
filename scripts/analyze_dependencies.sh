#!/bin/bash
# VantisOS Dependency Analysis Script
# Analyzes POSIX and external dependencies in the codebase

set -e

REPO_ROOT="/workspace/VantisOS"
OUTPUT_DIR="$REPO_ROOT/analysis"
mkdir -p "$OUTPUT_DIR"

echo "🔍 VantisOS Dependency Analysis"
echo "================================"
echo ""

# 1. Analyze std library usage
echo "1. Analyzing std library dependencies..."
grep -rh "^use std::" "$REPO_ROOT/src/verified" 2>/dev/null | \
    sed 's/use std:://g' | \
    sed 's/;.*//g' | \
    sort | uniq -c | sort -rn > "$OUTPUT_DIR/std_dependencies.txt"

echo "   Found $(wc -l < "$OUTPUT_DIR/std_dependencies.txt") unique std imports"

# 2. Analyze alloc library usage
echo "2. Analyzing alloc library dependencies..."
grep -rh "^use alloc::" "$REPO_ROOT/src/verified" 2>/dev/null | \
    sed 's/use alloc:://g' | \
    sed 's/;.*//g' | \
    sort | uniq -c | sort -rn > "$OUTPUT_DIR/alloc_dependencies.txt"

echo "   Found $(wc -l < "$OUTPUT_DIR/alloc_dependencies.txt") unique alloc imports"

# 3. Analyze core library usage
echo "3. Analyzing core library dependencies..."
grep -rh "^use core::" "$REPO_ROOT/src/verified" 2>/dev/null | \
    sed 's/use core:://g' | \
    sed 's/;.*//g' | \
    sort | uniq -c | sort -rn > "$OUTPUT_DIR/core_dependencies.txt"

echo "   Found $(wc -l < "$OUTPUT_DIR/core_dependencies.txt") unique core imports"

# 4. Analyze external crate dependencies
echo "4. Analyzing external crate dependencies..."
grep -rh "^use [a-z_]*::" "$REPO_ROOT/src/verified" 2>/dev/null | \
    grep -v "^use std::" | \
    grep -v "^use alloc::" | \
    grep -v "^use core::" | \
    grep -v "^use super::" | \
    grep -v "^use crate::" | \
    sed 's/use //g' | \
    sed 's/::.*//g' | \
    sort | uniq -c | sort -rn > "$OUTPUT_DIR/external_dependencies.txt"

echo "   Found $(wc -l < "$OUTPUT_DIR/external_dependencies.txt") unique external crates"

# 5. Analyze internal module dependencies
echo "5. Analyzing internal module dependencies..."
grep -rh "^use crate::" "$REPO_ROOT/src/verified" 2>/dev/null | \
    sed 's/use crate:://g' | \
    sed 's/::.*//g' | \
    sort | uniq -c | sort -rn > "$OUTPUT_DIR/internal_dependencies.txt"

echo "   Found $(wc -l < "$OUTPUT_DIR/internal_dependencies.txt") unique internal modules"

# 6. Count files by type
echo "6. Analyzing file types..."
echo "   Rust files: $(find "$REPO_ROOT/src" -name "*.rs" | wc -l)"
echo "   Total lines: $(find "$REPO_ROOT/src" -name "*.rs" -exec cat {} \; | wc -l)"

# 7. Analyze Cargo dependencies
echo "7. Analyzing Cargo.toml dependencies..."
if [ -f "$REPO_ROOT/src/verified/Cargo.toml" ]; then
    grep -A 100 "^\[dependencies\]" "$REPO_ROOT/src/verified/Cargo.toml" | \
        grep "^[a-z]" | \
        sed 's/ =.*//' | \
        sort > "$OUTPUT_DIR/cargo_dependencies.txt"
    echo "   Found $(wc -l < "$OUTPUT_DIR/cargo_dependencies.txt") Cargo dependencies"
fi

# 8. Summary
echo ""
echo "✅ Analysis complete!"
echo "   Results saved to: $OUTPUT_DIR/"
echo ""
echo "Summary:"
echo "  - std dependencies: $(wc -l < "$OUTPUT_DIR/std_dependencies.txt")"
echo "  - alloc dependencies: $(wc -l < "$OUTPUT_DIR/alloc_dependencies.txt")"
echo "  - core dependencies: $(wc -l < "$OUTPUT_DIR/core_dependencies.txt")"
echo "  - external crates: $(wc -l < "$OUTPUT_DIR/external_dependencies.txt")"
echo "  - internal modules: $(wc -l < "$OUTPUT_DIR/internal_dependencies.txt")"
echo "  - Cargo dependencies: $(wc -l < "$OUTPUT_DIR/cargo_dependencies.txt")"