#!/usr/bin/env bash
# VantisOS Minimal Build Script
# For environments with limited disk space (< 1GB free)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
VERIFIED_DIR="${REPO_ROOT}/src/verified"
REPORT_PATH="${REPO_ROOT}/BUILD_REPORT.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Main
main() {
    if [[ -t 1 ]] && [[ -n "${TERM:-}" ]]; then
        clear
    fi
    print_header "VantisOS Minimal Build"
    echo ""
    print_info "This script will:"
    echo "  1. Compile VantisOS verified components"
    echo "  2. Run tests"
    echo "  3. Generate build report"
    echo ""
    
    # Check disk space
    available=$(df -BG /workspace | awk 'NR==2 {print $4}' | sed 's/G//')
    if [ "$available" -lt 1 ]; then
        print_error "Insufficient disk space: ${available}GB available"
        print_info "Need at least 1GB free"
        exit 1
    fi
    print_success "Disk space OK: ${available}GB available"
    echo ""
    
    # Navigate to verified directory
    cd "${VERIFIED_DIR}"
    
    # Step 1: Build
    print_header "Step 1: Building Components"
    print_info "Compiling VantisOS verified components..."
    if cargo build --release 2>&1 | tee build.log; then
        print_success "Build completed successfully"
    else
        print_error "Build failed. Check build.log for details"
        exit 1
    fi
    echo ""
    
    # Step 2: Tests
    print_header "Step 2: Running Tests"
    print_info "Running test suite..."
    if cargo test --release 2>&1 | tee test.log; then
        print_success "Tests passed"
    else
        print_error "Some tests failed. Check test.log for details"
    fi
    echo ""
    
    # Step 3: Report
    print_header "Step 3: Generating Report"
    
    cat > "${REPORT_PATH}" << EOF
# VantisOS Minimal Build Report
## Generated: $(date)

## Build Environment
- **OS**: $(uname -s)
- **Kernel**: $(uname -r)
- **Rust**: $(rustc --version)
- **Cargo**: $(cargo --version)

## Build Results

### Compiled Components
EOF
    
    # List compiled artifacts
    if [ -d "target/release" ]; then
        echo "" >> "${REPORT_PATH}"
        echo '```' >> "${REPORT_PATH}"
        ls -lh target/release/*.rlib 2>/dev/null | head -20 >> "${REPORT_PATH}" || echo "No .rlib files found"
        echo '```' >> "${REPORT_PATH}"
    fi
    
    cat >> "${REPORT_PATH}" << 'EOF'

### Test Results
EOF
    
    # Add test summary
    if [ -f "test.log" ]; then
        echo "" >> "${REPORT_PATH}"
        echo '```' >> "${REPORT_PATH}"
        grep -E "(test result:|running)" test.log | tail -20 >> "${REPORT_PATH}"
        echo '```' >> "${REPORT_PATH}"
    fi
    
    local build_date
    build_date="$(date)"

    cat >> "${REPORT_PATH}" << 'EOF'

## Next Steps

### For Full Build
1. Setup environment with 20GB+ disk space
2. Use scripts/start_full_build.sh
3. Follow FULL_BUILD_PLAN.md

### Resources
- FULL_BUILD_PLAN.md - Complete build plan
- BUILD_OPTIONS_SUMMARY.md - All build options
- REALISTIC_BUILD_OPTIONS.md - Current situation analysis

---
EOF
    printf "**Build Date**: %s\n" "${build_date}" >> "${REPORT_PATH}"
    printf "**Status**: Minimal build completed\n" >> "${REPORT_PATH}"
    
    print_success "Report generated: ${REPORT_PATH}"
    echo ""
    
    # Summary
    print_header "Build Summary"
    echo ""
    print_success "Minimal build completed!"
    echo ""
    print_info "Results:"
    echo "  - Build log: src/verified/build.log"
    echo "  - Test log: src/verified/test.log"
    echo "  - Report: BUILD_REPORT.md"
    echo ""
    print_info "Next steps:"
    echo "  1. Review BUILD_REPORT.md"
    echo "  2. Check REALISTIC_BUILD_OPTIONS.md for full build"
    echo "  3. Setup larger environment for complete system"
    echo ""
}

main "$@"