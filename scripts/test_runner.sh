#!/usr/bin/env bash
#
# Advanced Test Runner for VantisOS
# Comprehensive testing automation with coverage, property-based testing, and fuzzing
#
# Usage: ./scripts/test_runner.sh [OPTIONS]
#
# Options:
#   --unit              Run unit tests only
#   --integration       Run integration tests only
#   --fuzz              Run fuzz tests
#   --coverage          Generate code coverage report
#   --property-based    Run property-based tests
#   --benchmark         Run benchmarks
#   --all               Run all tests (default)
#   --verbose           Enable verbose output
#   --fail-fast         Stop on first test failure
#   --json              Output results in JSON format
#   --help              Show this help message

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
RUN_UNIT=true
RUN_INTEGRATION=true
RUN_FUZZ=false
RUN_COVERAGE=false
RUN_PROPERTY=false
RUN_BENCHMARK=false
VERBOSE=false
FAIL_FAST=false
JSON_OUTPUT=false

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_RESULTS_DIR="$PROJECT_ROOT/test-results"
COVERAGE_DIR="$PROJECT_ROOT/target/tarpaulin"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --unit)
            RUN_INTEGRATION=false
            RUN_FUZZ=false
            shift
            ;;
        --integration)
            RUN_UNIT=false
            RUN_FUZZ=false
            shift
            ;;
        --fuzz)
            RUN_UNIT=false
            RUN_INTEGRATION=false
            RUN_FUZZ=true
            shift
            ;;
        --coverage)
            RUN_COVERAGE=true
            shift
            ;;
        --property-based)
            RUN_PROPERTY=true
            shift
            ;;
        --benchmark)
            RUN_BENCHMARK=true
            shift
            ;;
        --all)
            RUN_UNIT=true
            RUN_INTEGRATION=true
            RUN_FUZZ=true
            RUN_COVERAGE=true
            RUN_PROPERTY=true
            RUN_BENCHMARK=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --fail-fast)
            FAIL_FAST=true
            shift
            ;;
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --help)
            echo "Advanced Test Runner for VantisOS"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --unit              Run unit tests only"
            echo "  --integration       Run integration tests only"
            echo "  --fuzz              Run fuzz tests"
            echo "  --coverage          Generate code coverage report"
            echo "  --property-based    Run property-based tests"
            echo "  --benchmark         Run benchmarks"
            echo "  --all               Run all tests (default)"
            echo "  --verbose           Enable verbose output"
            echo "  --fail-fast         Stop on first test failure"
            echo "  --json              Output results in JSON format"
            echo "  --help              Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Create necessary directories
mkdir -p "$TEST_RESULTS_DIR"
mkdir -p "$COVERAGE_DIR"

# Logging functions
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

# Print banner
print_banner() {
    echo ""
    echo "=========================================="
    echo "  VantisOS Advanced Test Runner v0.4.1"
    echo "=========================================="
    echo ""
}

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("cargo")
    fi
    
    if [ "$RUN_COVERAGE" = true ] && ! command -v cargo-tarpaulin &> /dev/null; then
        missing_deps+=("cargo-tarpaulin")
    fi
    
    if [ "$RUN_PROPERTY" = true ] && ! cargo list | grep -q "proptest"; then
        missing_deps+=("proptest")
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        log_error "Missing dependencies: ${missing_deps[*]}"
        log_info "Install missing dependencies with:"
        for dep in "${missing_deps[@]}"; do
            echo "  cargo install $dep"
        done
        exit 1
    fi
    
    log_success "All dependencies installed"
}

# Run unit tests
run_unit_tests() {
    log_info "Running unit tests..."
    
    local cargo_args=""
    if [ "$VERBOSE" = true ]; then
        cargo_args="-- --nocapture"
    fi
    if [ "$FAIL_FAST" = true ]; then
        cargo_args="$cargo_args --fail-fast"
    fi
    
    cd "$PROJECT_ROOT"
    if cargo test --lib --bins $cargo_args 2>&1 | tee "$TEST_RESULTS_DIR/unit_tests.log"; then
        log_success "Unit tests passed"
        return 0
    else
        log_error "Unit tests failed"
        return 1
    fi
}

# Run integration tests
run_integration_tests() {
    log_info "Running integration tests..."
    
    local cargo_args=""
    if [ "$VERBOSE" = true ]; then
        cargo_args="-- --nocapture"
    fi
    if [ "$FAIL_FAST" = true ]; then
        cargo_args="$cargo_args --fail-fast"
    fi
    
    cd "$PROJECT_ROOT"
    if cargo test --test '*' $cargo_args 2>&1 | tee "$TEST_RESULTS_DIR/integration_tests.log"; then
        log_success "Integration tests passed"
        return 0
    else
        log_error "Integration tests failed"
        return 1
    fi
}

# Run fuzz tests
run_fuzz_tests() {
    log_info "Running fuzz tests..."
    
    if [ ! -d "$PROJECT_ROOT/fuzz" ]; then
        log_warning "No fuzz tests found, skipping"
        return 0
    fi
    
    cd "$PROJECT_ROOT/fuzz"
    if cargo fuzz run -- -max_total_time=300 2>&1 | tee "$TEST_RESULTS_DIR/fuzz_tests.log"; then
        log_success "Fuzz tests completed"
        return 0
    else
        log_error "Fuzz tests failed"
        return 1
    fi
}

# Run code coverage
run_coverage() {
    log_info "Generating code coverage report..."
    
    cd "$PROJECT_ROOT"
    
    # Check if tarpaulin is installed
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log_warning "cargo-tarpaulin not found, skipping coverage"
        return 0
    fi
    
    # Run tarpaulin
    if cargo tarpaulin --out Html --out Lcov --out Xml --output-dir "$COVERAGE_DIR" \
        --line-coverage --branch-coverage --timeout 300 \
        --exclude-files "tests/**,examples/**,benches/**" 2>&1 | tee "$TEST_RESULTS_DIR/coverage.log"; then
        log_success "Code coverage report generated: $COVERAGE_DIR/index.html"
        return 0
    else
        log_error "Code coverage generation failed"
        return 1
    fi
}

# Run property-based tests
run_property_tests() {
    log_info "Running property-based tests..."
    
    cd "$PROJECT_ROOT"
    
    # Run proptest tests
    if cargo test --features proptest 2>&1 | tee "$TEST_RESULTS_DIR/property_tests.log"; then
        log_success "Property-based tests passed"
        return 0
    else
        log_error "Property-based tests failed"
        return 1
    fi
}

# Run benchmarks
run_benchmarks() {
    log_info "Running benchmarks..."
    
    cd "$PROJECT_ROOT"
    
    if [ ! -d "benches" ]; then
        log_warning "No benchmarks found, skipping"
        return 0
    fi
    
    if cargo bench 2>&1 | tee "$TEST_RESULTS_DIR/benchmarks.log"; then
        log_success "Benchmarks completed"
        return 0
    else
        log_error "Benchmarks failed"
        return 1
    fi
}

# Generate test summary
generate_summary() {
    log_info "Generating test summary..."
    
    local summary_file="$TEST_RESULTS_DIR/test_summary.txt"
    
    echo "========================================" > "$summary_file"
    echo "VantisOS Test Summary" >> "$summary_file"
    echo "========================================" >> "$summary_file"
    echo "Date: $(date)" >> "$summary_file"
    echo "Repository: $(git -C "$PROJECT_ROOT" remote get-url origin 2>/dev/null || echo 'N/A')" >> "$summary_file"
    echo "Commit: $(git -C "$PROJECT_ROOT" rev-parse HEAD 2>/dev/null || echo 'N/A')" >> "$summary_file"
    echo "" >> "$summary_file"
    
    # Test results
    if [ -f "$TEST_RESULTS_DIR/unit_tests.log" ]; then
        echo "Unit Tests: $(grep -c 'test result' "$TEST_RESULTS_DIR/unit_tests.log" || echo '0')" >> "$summary_file"
    fi
    
    if [ -f "$TEST_RESULTS_DIR/coverage.log" ]; then
        echo "Coverage Report: $COVERAGE_DIR/index.html" >> "$summary_file"
    fi
    
    echo "" >> "$summary_file"
    echo "Test Results Directory: $TEST_RESULTS_DIR" >> "$summary_file"
    
    log_success "Test summary generated: $summary_file"
    
    if [ "$JSON_OUTPUT" = true ]; then
        local json_file="$TEST_RESULTS_DIR/test_results.json"
        echo '{"timestamp":"'$(date -u +%Y-%m-%dT%H:%M:%SZ)'","unit_tests":"'$([ -f "$TEST_RESULTS_DIR/unit_tests.log" ] && echo "completed" || echo "skipped")'","integration_tests":"'$([ -f "$TEST_RESULTS_DIR/integration_tests.log" ] && echo "completed" || echo "skipped")'","fuzz_tests":"'$([ -f "$TEST_RESULTS_DIR/fuzz_tests.log" ] && echo "completed" || echo "skipped")'","coverage":"'$([ -f "$TEST_RESULTS_DIR/coverage.log" ] && echo "generated" || echo "skipped")'","property_tests":"'$([ -f "$TEST_RESULTS_DIR/property_tests.log" ] && echo "completed" || echo "skipped")'","benchmarks":"'$([ -f "$TEST_RESULTS_DIR/benchmarks.log" ] && echo "completed" || echo "skipped")'"}' > "$json_file"
        log_success "JSON results generated: $json_file"
    fi
}

# Main execution
main() {
    print_banner
    check_dependencies
    
    local start_time=$(date +%s)
    local exit_code=0
    
    # Run tests based on flags
    if [ "$RUN_UNIT" = true ]; then
        run_unit_tests || exit_code=1
    fi
    
    if [ "$RUN_INTEGRATION" = true ] && [ $exit_code -eq 0 ]; then
        run_integration_tests || exit_code=1
    fi
    
    if [ "$RUN_FUZZ" = true ] && [ $exit_code -eq 0 ]; then
        run_fuzz_tests || exit_code=1
    fi
    
    if [ "$RUN_COVERAGE" = true ] && [ $exit_code -eq 0 ]; then
        run_coverage || exit_code=1
    fi
    
    if [ "$RUN_PROPERTY" = true ] && [ $exit_code -eq 0 ]; then
        run_property_tests || exit_code=1
    fi
    
    if [ "$RUN_BENCHMARK" = true ] && [ $exit_code -eq 0 ]; then
        run_benchmarks || exit_code=1
    fi
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # Generate summary
    generate_summary
    
    echo ""
    if [ $exit_code -eq 0 ]; then
        log_success "All tests passed successfully!"
        echo "Duration: ${duration}s"
    else
        log_error "Some tests failed. Check logs in $TEST_RESULTS_DIR"
        echo "Duration: ${duration}s"
    fi
    
    exit $exit_code
}

# Run main function
main "$@"