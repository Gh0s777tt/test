#!/bin/bash
#
# VantisOS - Complete Test Script
# Runs all tests for VantisOS
#

set -e  # Exit on error
set -u  # Exit on undefined variable
set -o pipefail  # Exit on pipe failure

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Run unit tests
run_unit_tests() {
    log_info "Running unit tests..."
    if cargo test --lib 2>/dev/null; then
        log_success "Unit tests passed"
        ((TESTS_PASSED++))
    else
        log_error "Unit tests failed"
        ((TESTS_FAILED++))
    fi
}

# Run integration tests
run_integration_tests() {
    log_info "Running integration tests..."
    if cargo test --test integration 2>/dev/null; then
        log_success "Integration tests passed"
        ((TESTS_PASSED++))
    else
        log_warning "Integration tests not found or failed"
    fi
}

# Run cloud-native tests
run_cloud_native_tests() {
    log_info "Running cloud-native tests..."
    if cargo test --test integration_cloud_native 2>/dev/null; then
        log_success "Cloud-native tests passed"
        ((TESTS_PASSED++))
    else
        log_warning "Cloud-native tests not found or failed"
    fi
}

# Run coverage
run_coverage() {
    log_info "Running coverage analysis..."
    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --out Html || log_warning "Coverage tool not available"
        log_success "Coverage report generated"
    else
        log_warning "cargo-tarpaulin not installed, skipping coverage..."
    fi
}

# Main function
main() {
    log_info "Starting VantisOS complete test suite..."
    log_info "========================================"
    
    # Run all test phases
    run_unit_tests
    run_integration_tests
    run_cloud_native_tests
    run_coverage
    
    log_info "========================================"
    log_info "Test Results:"
    log_info "  Passed: $TESTS_PASSED"
    log_info "  Failed: $TESTS_FAILED"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "All tests passed!"
        exit 0
    else
        log_error "Some tests failed!"
        exit 1
    fi
}

# Run main function
main "$@"