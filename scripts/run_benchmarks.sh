#!/usr/bin/env bash
# Run benchmark suite for src/verified.
# Usage:
#   ./scripts/run_benchmarks.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT/src/verified"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required but not installed." >&2
  exit 1
fi

echo "== Running VantisOS benchmarks =="

BENCHES=(
  "scheduler_benchmark:scheduler_baseline"
  "filesystem_benchmark:filesystem_baseline"
)

for item in "${BENCHES[@]}"; do
  bench="${item%%:*}"
  baseline="${item##*:}"
  echo
  echo "Running benchmark: $bench (baseline: $baseline)"
  cargo bench --bench "$bench" -- --save-baseline "$baseline"
done

echo
echo "Benchmarks finished."
echo "Reports: src/verified/target/criterion/"