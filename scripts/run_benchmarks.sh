#!/usr/bin/env bash
# Run benchmark suite for src/verified.
# Usage:
#   ./scripts/run_benchmarks.sh            # scheduler + filesystem (legacy default)
#   ./scripts/run_benchmarks.sh --syscall  # syscall-focused suite (Day 7)
#   ./scripts/run_benchmarks.sh --all      # run all benchmark groups

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT/src/verified"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required but not installed." >&2
  exit 1
fi

MODE="legacy"
if [[ "${1:-}" == "--syscall" ]]; then
  MODE="syscall"
elif [[ "${1:-}" == "--all" ]]; then
  MODE="all"
elif [[ -n "${1:-}" ]]; then
  echo "Unknown option: $1" >&2
  echo "Usage: ./scripts/run_benchmarks.sh [--syscall|--all]" >&2
  exit 1
fi

echo "== Running VantisOS benchmarks =="

LEGACY_BENCHES=(
  "scheduler_benchmark:scheduler_baseline"
  "filesystem_benchmark:filesystem_baseline"
)

SYSCALL_BENCHES=(
  "performance_baseline:performance_baseline_day7"
  "syscall_performance_simple:syscall_simple_day7"
  "path_lookup_cache_benchmark:path_lookup_cache_day7"
  "fd_allocator_benchmark:fd_allocator_day7"
)

BENCHES=()
if [[ "$MODE" == "legacy" || "$MODE" == "all" ]]; then
  BENCHES+=("${LEGACY_BENCHES[@]}")
fi
if [[ "$MODE" == "syscall" || "$MODE" == "all" ]]; then
  BENCHES+=("${SYSCALL_BENCHES[@]}")
fi

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