#!/usr/bin/env bash
# VantisOS dependency analysis script.
# Generates lightweight dependency reports for src/verified.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="$REPO_ROOT/src/verified"
OUTPUT_DIR="$REPO_ROOT/analysis/dependencies"
mkdir -p "$OUTPUT_DIR"

if [[ ! -d "$SRC_DIR" ]]; then
  echo "Missing source directory: $SRC_DIR" >&2
  exit 1
fi

echo "Dependency analysis for: $SRC_DIR"
echo "Output directory: $OUTPUT_DIR"
echo

rg '^use std::' "$SRC_DIR" -g '*.rs' -N --no-heading \
  | sed 's/.*use std:://; s/;.*$//' \
  | sort | uniq -c | sort -rn > "$OUTPUT_DIR/std_dependencies.txt"

rg '^use alloc::' "$SRC_DIR" -g '*.rs' -N --no-heading \
  | sed 's/.*use alloc:://; s/;.*$//' \
  | sort | uniq -c | sort -rn > "$OUTPUT_DIR/alloc_dependencies.txt"

rg '^use core::' "$SRC_DIR" -g '*.rs' -N --no-heading \
  | sed 's/.*use core:://; s/;.*$//' \
  | sort | uniq -c | sort -rn > "$OUTPUT_DIR/core_dependencies.txt"

rg '^use [a-z_][a-z0-9_]*::' "$SRC_DIR" -g '*.rs' -N --no-heading \
  | rg -v '^.*use (std|alloc|core|crate|super)::' \
  | sed 's/.*use //; s/::.*$//' \
  | sort | uniq -c | sort -rn > "$OUTPUT_DIR/external_dependencies.txt"

rg '^use crate::' "$SRC_DIR" -g '*.rs' -N --no-heading \
  | sed 's/.*use crate:://; s/::.*$//' \
  | sort | uniq -c | sort -rn > "$OUTPUT_DIR/internal_dependencies.txt"

if [[ -f "$SRC_DIR/Cargo.toml" ]]; then
  awk '
    BEGIN { in_deps=0 }
    /^\[dependencies\]/ { in_deps=1; next }
    /^\[/ && in_deps { exit }
    in_deps && /^[a-zA-Z0-9_-]+[[:space:]]*=/ {
      gsub(/[[:space:]]*=.*/, "", $0)
      print $0
    }
  ' "$SRC_DIR/Cargo.toml" | sort > "$OUTPUT_DIR/cargo_dependencies.txt"
else
  : > "$OUTPUT_DIR/cargo_dependencies.txt"
fi

RUST_FILE_COUNT="$(rg --files "$REPO_ROOT/src" -g '*.rs' | wc -l | tr -d ' ')"
RUST_LINE_COUNT="$(rg -N '^' "$REPO_ROOT/src" -g '*.rs' | wc -l | tr -d ' ')"

cat > "$OUTPUT_DIR/summary.txt" <<EOF
Rust files under src/: $RUST_FILE_COUNT
Total Rust lines under src/: $RUST_LINE_COUNT
std dependencies: $(wc -l < "$OUTPUT_DIR/std_dependencies.txt" | tr -d ' ')
alloc dependencies: $(wc -l < "$OUTPUT_DIR/alloc_dependencies.txt" | tr -d ' ')
core dependencies: $(wc -l < "$OUTPUT_DIR/core_dependencies.txt" | tr -d ' ')
external dependencies: $(wc -l < "$OUTPUT_DIR/external_dependencies.txt" | tr -d ' ')
internal dependencies: $(wc -l < "$OUTPUT_DIR/internal_dependencies.txt" | tr -d ' ')
Cargo [dependencies] entries: $(wc -l < "$OUTPUT_DIR/cargo_dependencies.txt" | tr -d ' ')
EOF

echo "Analysis complete."
cat "$OUTPUT_DIR/summary.txt"