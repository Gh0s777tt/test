#!/bin/bash
#
# VantisOS Local CI Simulation Script
# Simulates CI pipeline locally before pushing
#
# Usage: ./scripts/dev/local-ci.sh [--quick]

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"

# Configuration
QUICK_MODE="${1:-false}"

print_banner "VantisOS Local CI" "Simulating CI Pipeline"

# Counters
TOTAL_STEPS=0
PASSED_STEPS=0
FAILED_STEPS=0

# Output directory
LOG_DIR="$VANTIS_ROOT/logs/ci"
mkdir -p "$LOG_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="$LOG_DIR/ci_$TIMESTAMP.log"

# ==============================================================================
# Helper Functions
# ==============================================================================

run_step() {
    local name="$1"
    local cmd="$2"
    local required="${3:-true}"
    
    ((TOTAL_STEPS++))
    
    print_section "$name"
    log_info "Running: $cmd"
    
    local output
    local exit_code
    
    if output=$(eval "$cmd" 2>&1); then
        log_success "$name passed"
        ((PASSED_STEPS++))
        echo "[PASS] $name" >> "$LOG_FILE"
    else
        if [[ "$required" == "true" ]]; then
            log_error "$name failed"
            ((FAILED_STEPS++))
            echo "[FAIL] $name" >> "$LOG_FILE"
        else
            log_warning "$name failed (non-critical)"
            echo "[WARN] $name" >> "$LOG_FILE"
        fi
        echo "$output" >> "$LOG_FILE"
    fi
}

# ==============================================================================
# CI Pipeline Steps
# ==============================================================================

print_section "Starting CI Pipeline Simulation"
echo "Timestamp: $TIMESTAMP"
echo "Log file: $LOG_FILE"
echo ""

# Step 1: Code Formatting
run_step "Code Formatting" "cargo fmt --all --check" "true"

# Step 2: Linting
run_step "Clippy Linting" "cargo clippy --all-targets --all-features -- -D warnings" "true"

# Step 3: Build (Debug)
run_step "Debug Build" "cargo build" "true"

# Step 4: Build (Release) - Skip in quick mode
if [[ "$QUICK_MODE" != "true" ]]; then
    run_step "Release Build" "cargo build --release" "false"
fi

# Step 5: Tests
run_step "Unit Tests" "cargo test --lib" "true"

# Step 6: Integration Tests - Skip in quick mode
if [[ "$QUICK_MODE" != "true" ]]; then
    run_step "Integration Tests" "cargo test --test integration" "false"
fi

# Step 7: Documentation
run_step "Documentation Build" "cargo doc --no-deps" "false"

# Step 8: Security Audit
run_step "Security Audit" "cargo audit" "false"

# Step 9: Shell Script Validation
run_step "Shell Script Linting" "find scripts -name '*.sh' -exec shellcheck {} +" "false"

# Step 10: Pre-commit (if available)
if command_exists pre-commit && [[ -f "$VANTIS_ROOT/.pre-commit-config.yaml" ]]; then
    run_step "Pre-commit Hooks" "pre-commit run --all-files" "false"
fi

# Step 11: Dependencies Check
run_step "Dependency Check" "cargo tree --depth 1" "false"

# ==============================================================================
# Summary
# ==============================================================================

print_banner "CI Pipeline Summary"

echo ""
echo "Total Steps: $TOTAL_STEPS"
echo -e "${GREEN}Passed: $PASSED_STEPS${NC}"
if [[ $FAILED_STEPS -gt 0 ]]; then
    echo -e "${RED}Failed: $FAILED_STEPS${NC}"
fi
echo ""
echo "Log saved to: $LOG_FILE"
echo ""

if [[ $FAILED_STEPS -eq 0 ]]; then
    log_success "All CI checks passed! Ready to push. ✨"
    exit 0
else
    log_error "$FAILED_STEPS CI checks failed. Fix issues before pushing."
    exit 1
fi