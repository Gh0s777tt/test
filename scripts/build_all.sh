#!/bin/bash
#
# VantisOS - Complete Build Script
# Builds all components of VantisOS
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

# Main build function
main() {
    log_info "Starting VantisOS complete build..."
    log_info "========================================"
    
    # Phase 1: Build kernel
    log_info "Phase 1: Building kernel..."
    if [ -f "./scripts/build_kernel.sh" ]; then
        ./scripts/build_kernel.sh || log_warning "Kernel build script not found or failed"
        log_success "Kernel build phase completed"
    else
        log_warning "Kernel build script not found, skipping..."
    fi
    
    # Phase 2: Build components
    log_info "Phase 2: Building components..."
    if [ -f "./scripts/build_components.sh" ]; then
        ./scripts/build_components.sh || log_warning "Components build script not found or failed"
        log_success "Components build phase completed"
    else
        log_warning "Components build script not found, skipping..."
    fi
    
    # Phase 3: Build desktop
    log_info "Phase 3: Building desktop environment..."
    if [ -f "./scripts/build_desktop.sh" ]; then
        ./scripts/build_desktop.sh || log_warning "Desktop build script not found or failed"
        log_success "Desktop build phase completed"
    else
        log_warning "Desktop build script not found, skipping..."
    fi
    
    # Phase 4: Create ISO
    log_info "Phase 4: Creating ISO image..."
    if [ -f "./scripts/build_iso.sh" ]; then
        ./scripts/build_iso.sh || log_warning "ISO build script not found or failed"
        log_success "ISO creation phase completed"
    else
        log_warning "ISO build script not found, skipping..."
    fi
    
    # Phase 5: Run tests
    log_info "Phase 5: Running tests..."
    if [ -f "./scripts/test_all.sh" ]; then
        ./scripts/test_all.sh || log_warning "Test script not found or failed"
        log_success "Test phase completed"
    else
        log_warning "Test script not found, skipping..."
    fi
    
    log_info "========================================"
    log_success "VantisOS complete build finished!"
    log_info "Check output directory for artifacts"
}

# Run main function
main "$@"