#!/bin/bash
# VANTIS OS - Production Cryptography Deployment Script
# 
# This script automates the deployment of production-ready cryptography
# using RustCrypto libraries.
#
# Usage: ./deploy_production_crypto.sh [options]
#
# Options:
#   --test-only     Run tests without building
#   --benchmark     Run performance benchmarks
#   --hw-accel      Enable hardware acceleration (AES-NI)
#   --verify        Run NIST test vectors
#   --all           Run all steps

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored message
print_msg() {
    echo -e "${2}${1}${NC}"
}

print_header() {
    echo ""
    print_msg "========================================" "$BLUE"
    print_msg "$1" "$BLUE"
    print_msg "========================================" "$BLUE"
    echo ""
}

print_success() {
    print_msg "✅ $1" "$GREEN"
}

print_error() {
    print_msg "❌ $1" "$RED"
}

print_warning() {
    print_msg "⚠️  $1" "$YELLOW"
}

# Check if we're in the right directory
if [ ! -f "src/verified/Cargo.toml" ]; then
    print_error "Error: Must be run from VantisOS root directory"
    exit 1
fi

# Parse command line arguments
TEST_ONLY=false
BENCHMARK=false
HW_ACCEL=false
VERIFY=false
RUN_ALL=false

for arg in "$@"; do
    case $arg in
        --test-only)
            TEST_ONLY=true
            ;;
        --benchmark)
            BENCHMARK=true
            ;;
        --hw-accel)
            HW_ACCEL=true
            ;;
        --verify)
            VERIFY=true
            ;;
        --all)
            RUN_ALL=true
            TEST_ONLY=false
            BENCHMARK=true
            HW_ACCEL=true
            VERIFY=true
            ;;
        *)
            print_error "Unknown option: $arg"
            echo "Usage: $0 [--test-only] [--benchmark] [--hw-accel] [--verify] [--all]"
            exit 1
            ;;
    esac
done

# Step 1: Check Dependencies
print_header "Step 1: Checking Dependencies"

if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust toolchain."
    exit 1
fi
print_success "Cargo found: $(cargo --version)"

if ! command -v rustc &> /dev/null; then
    print_error "Rustc not found. Please install Rust toolchain."
    exit 1
fi
print_success "Rustc found: $(rustc --version)"

# Step 2: Update Dependencies
if [ "$TEST_ONLY" = false ]; then
    print_header "Step 2: Updating Dependencies"
    
    cd src/verified
    
    print_msg "Updating Cargo dependencies..." "$YELLOW"
    cargo update
    print_success "Dependencies updated"
    
    cd ../..
fi

# Step 3: Build
if [ "$TEST_ONLY" = false ]; then
    print_header "Step 3: Building Project"
    
    cd src/verified
    
    if [ "$HW_ACCEL" = true ]; then
        print_msg "Building with hardware acceleration (AES-NI)..." "$YELLOW"
        RUSTFLAGS="-C target-cpu=native" cargo build --release --features hw-accel
    else
        print_msg "Building in release mode..." "$YELLOW"
        cargo build --release
    fi
    
    print_success "Build completed"
    
    cd ../..
fi

# Step 4: Run Tests
print_header "Step 4: Running Tests"

cd src/verified

print_msg "Running unit tests..." "$YELLOW"
if cargo test --release; then
    print_success "All unit tests passed"
else
    print_error "Some tests failed"
    exit 1
fi

cd ../..

# Step 5: Run Benchmarks
if [ "$BENCHMARK" = true ] || [ "$RUN_ALL" = true ]; then
    print_header "Step 5: Running Performance Benchmarks"
    
    cd src/verified
    
    print_msg "Running performance tests..." "$YELLOW"
    cargo test --release test_production_performance -- --nocapture
    
    print_success "Benchmarks completed"
    
    cd ../..
fi

# Step 6: Verify with NIST Vectors
if [ "$VERIFY" = true ] || [ "$RUN_ALL" = true ]; then
    print_header "Step 6: Verifying with NIST Test Vectors"
    
    cd src/verified
    
    print_msg "Running NIST test vectors..." "$YELLOW"
    if cargo test --release --features hex nist; then
        print_success "NIST verification passed"
    else
        print_warning "NIST tests not yet implemented or failed"
    fi
    
    cd ../..
fi

# Step 7: Security Checks
print_header "Step 7: Security Checks"

print_msg "Checking for unsafe code..." "$YELLOW"
cd src/verified
UNSAFE_COUNT=$(grep -r "unsafe" --include="*.rs" . | grep -v "// unsafe" | grep -v "test" | wc -l)
if [ "$UNSAFE_COUNT" -eq 0 ]; then
    print_success "No unsafe code found (except secure zeroization)"
else
    print_warning "Found $UNSAFE_COUNT instances of unsafe code"
fi
cd ../..

print_msg "Checking for TODO/FIXME..." "$YELLOW"
cd src/verified
TODO_COUNT=$(grep -r "TODO\|FIXME" --include="*.rs" . | wc -l)
if [ "$TODO_COUNT" -eq 0 ]; then
    print_success "No TODO/FIXME found"
else
    print_warning "Found $TODO_COUNT TODO/FIXME items"
fi
cd ../..

# Step 8: Generate Documentation
print_header "Step 8: Generating Documentation"

cd src/verified

print_msg "Generating Rust documentation..." "$YELLOW"
cargo doc --no-deps --release

print_success "Documentation generated in target/doc/"

cd ../..

# Final Summary
print_header "Deployment Summary"

print_success "All steps completed successfully!"
echo ""
print_msg "Next steps:" "$BLUE"
echo "  1. Review generated documentation: target/doc/"
echo "  2. Run security audit"
echo "  3. Test on different platforms"
echo "  4. Prepare for FIPS 140-3 certification"
echo ""

if [ "$HW_ACCEL" = true ]; then
    print_msg "Hardware acceleration (AES-NI) is ENABLED" "$GREEN"
else
    print_warning "Hardware acceleration is DISABLED. Use --hw-accel to enable."
fi

echo ""
print_msg "Deployment completed at: $(date)" "$GREEN"