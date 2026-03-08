#!/bin/bash
# VANTIS OS Benchmark Runner
# Runs comprehensive benchmarks and generates reports

set -e

echo "🚀 VANTIS OS Benchmark Suite"
echo "=============================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Navigate to verified directory
cd "$(dirname "$0")/../src/verified"

echo -e "${BLUE}📊 Running Scheduler Benchmarks...${NC}"
echo ""
cargo bench --bench scheduler_benchmark -- --save-baseline scheduler_baseline

echo ""
echo -e "${BLUE}📊 Running Filesystem Benchmarks...${NC}"
echo ""
cargo bench --bench filesystem_benchmark -- --save-baseline filesystem_baseline

echo ""
echo -e "${GREEN}✅ Benchmarks Complete!${NC}"
echo ""
echo "📈 Results saved to: target/criterion/"
echo ""
echo "To view HTML reports:"
echo "  - Scheduler: target/criterion/scheduler_decision/report/index.html"
echo "  - Filesystem: target/criterion/block_allocation/report/index.html"
echo ""
echo "To compare with baseline:"
echo "  cargo bench --bench scheduler_benchmark -- --baseline scheduler_baseline"
echo "  cargo bench --bench filesystem_benchmark -- --baseline filesystem_baseline"