#!/bin/bash
# Script: setup_environment.sh
# Purpose: Set up complete development environment for VantisOS
# Usage: ./scripts/dev/setup_environment.sh [options]
# Requirements: sudo access, internet connection
# Author: VantisOS Team
# Date: 2025-03-05
# Version: 1.0.0
# License: SPDX-License-Identifier: MIT

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common library
source "${SCRIPT_DIR}/../lib/common.sh" || {
    echo "Error: Failed to load common library" >&2
    exit 1
}

# Enable strict error handling
set -euo pipefail

# Configuration
INSTALL_DIR="${INSTALL_DIR:-/opt/vantisos}"
BUILD_DIR="${BUILD_DIR:-$HOME/vantisos-build}"
MIN_DISK_SPACE_GB=20
MIN_MEMORY_GB=8

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Distribution detection
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif [ -f /etc/redhat-release ]; then
        echo "rhel"
    else
        echo "unknown"
    fi
}

# Install dependencies for Debian/Ubuntu
install_debian_deps() {
    print_subheader "Installing dependencies for Debian/Ubuntu"
    
    sudo apt-get update
    
    # Build essentials
    sudo apt-get install -y build-essential git cmake make gcc g++ \
        pkg-config autoconf automake libtool
    
    # Rust toolchain
    if ! command -v rustc &> /dev/null; then
        log_info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Python
    sudo apt-get install -y python3 python3-pip python3-venv
    
    # Node.js
    if ! command -v node &> /dev/null; then
        log_info "Installing Node.js..."
        curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
        sudo apt-get install -y nodejs
    fi
    
    # QEMU for testing
    sudo apt-get install -y qemu-system qemu-utils
    
    # Additional tools
    sudo apt-get install -y curl wget jq yq shellcheck markdownlint-cli
}

# Install dependencies for Fedora/RHEL
install_rhel_deps() {
    print_subheader "Installing dependencies for Fedora/RHEL"
    
    sudo dnf groupinstall -y "Development Tools"
    sudo dnf install -y git cmake make gcc g++ pkg-config autoconf automake libtool
    
    # Rust toolchain
    if ! command -v rustc &> /dev/null; then
        log_info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Python
    sudo dnf install -y python3 python3-pip
    
    # Node.js
    if ! command -v node &> /dev/null; then
        log_info "Installing Node.js..."
        sudo dnf install -y nodejs npm
    fi
    
    # QEMU for testing
    sudo dnf install -y qemu-system-x86 qemu-img
    
    # Additional tools
    sudo dnf install -y curl wget jq shellcheck
}

# Install dependencies for Arch Linux
install_arch_deps() {
    print_subheader "Installing dependencies for Arch Linux"
    
    sudo pacman -Syu --noconfirm
    
    # Build essentials
    sudo pacman -S --noconfirm --needed base-devel git cmake make gcc \
        pkg-config autoconf automake libtool
    
    # Rust toolchain
    if ! command -v rustc &> /dev/null; then
        log_info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Python
    sudo pacman -S --noconfirm --needed python python-pip
    
    # Node.js
    if ! command -v node &> /dev/null; then
        log_info "Installing Node.js..."
        sudo pacman -S --noconfirm --needed nodejs npm
    fi
    
    # QEMU for testing
    sudo pacman -S --noconfirm --needed qemu-headless
    
    # Additional tools
    sudo pacman -S --noconfirm --needed curl wget jq shellcheck
}

# Verify system requirements
verify_requirements() {
    print_header "Verifying System Requirements"
    
    # Check disk space
    local available_gb
    available_gb=$(df -BG / | awk 'NR==2 {print $4}' | sed 's/G//')
    
    if [ "$available_gb" -lt "$MIN_DISK_SPACE_GB" ]; then
        log_warn "Low disk space: ${available_gb}GB (recommended: ${MIN_DISK_SPACE_GB}GB)"
    else
        print_success "Disk space: ${available_gb}GB"
    fi
    
    # Check memory
    local memory_gb
    memory_gb=$(free -g | awk '/^Mem:/{print $2}')
    
    if [ "$memory_gb" -lt "$MIN_MEMORY_GB" ]; then
        log_warn "Low memory: ${memory_gb}GB (recommended: ${MIN_MEMORY_GB}GB)"
    else
        print_success "Memory: ${memory_gb}GB"
    fi
    
    # Check if running as root
    if [ "$EUID" -eq 0 ]; then
        log_warn "Running as root is not recommended"
    fi
}

# Install pre-commit hooks
install_precommit() {
    print_subheader "Installing Pre-commit Hooks"
    
    if command -v pre-commit &> /dev/null; then
        log_info "Pre-commit already installed"
    else
        pip3 install pre-commit
    fi
    
    if [ -f .pre-commit-config.yaml ]; then
        pre-commit install
        print_success "Pre-commit hooks installed"
    else
        log_warn "No .pre-commit-config.yaml found"
    fi
}

# Configure git
configure_git() {
    print_subheader "Configuring Git"
    
    if [ -z "$(git config --get user.name)" ]; then
        log_info "Please configure your git user.name and user.email"
        echo "Example:"
        echo "  git config --global user.name 'Your Name'"
        echo "  git config --global user.email 'your.email@example.com'"
    else
        print_success "Git configured for: $(git config --get user.name)"
    fi
}

# Create build directories
create_directories() {
    print_subheader "Creating Build Directories"
    
    ensure_dir "$BUILD_DIR"
    ensure_dir "$BUILD_DIR/cache"
    ensure_dir "$BUILD_DIR/output"
    ensure_dir "$BUILD_DIR/logs"
    
    print_success "Build directories created at $BUILD_DIR"
}

# Set up environment variables
setup_env_vars() {
    print_subheader "Setting up Environment Variables"
    
    local env_file="$HOME/.vantisos_env"
    
    cat > "$env_file" << EOF
# VantisOS Environment Variables
export VANTISOS_BUILD_DIR="$BUILD_DIR"
export VANTISOS_INSTALL_DIR="$INSTALL_DIR"
export VANTISOS_CACHE_DIR="$BUILD_DIR/cache"
export VANTISOS_OUTPUT_DIR="$BUILD_DIR/output"
export VANTISOS_LOG_DIR="$BUILD_DIR/logs"

# Add to PATH
export PATH="\$PATH:$INSTALL_DIR/bin"

# Rust configuration
export CARGO_TARGET_DIR="$BUILD_DIR/target"
EOF

    print_success "Environment variables saved to $env_file"
    log_info "Add 'source ~/.vantisos_env' to your .bashrc or .zshrc"
}

# Print summary
print_summary() {
    print_header "Setup Complete"
    
    echo -e "${GREEN}VantisOS development environment has been set up!${NC}"
    echo ""
    echo "Installed tools:"
    command -v rustc &> /dev/null && echo "  ✓ Rust $(rustc --version 2>/dev/null)"
    command -v node &> /dev/null && echo "  ✓ Node.js $(node --version 2>/dev/null)"
    command -v python3 &> /dev/null && echo "  ✓ Python $(python3 --version 2>/dev/null)"
    command -v qemu-system-x86_64 &> /dev/null && echo "  ✓ QEMU $(qemu-system-x86_64 --version | head -1)"
    command -v git &> /dev/null && echo "  ✓ Git $(git --version)"
    echo ""
    echo "Build directory: $BUILD_DIR"
    echo "Install directory: $INSTALL_DIR"
    echo ""
    echo "Next steps:"
    echo "  1. source ~/.vantisos_env"
    echo "  2. Run: ./scripts/build_all.sh"
    echo "  3. Test: ./scripts/test_all.sh"
    echo ""
    echo "For more information, see docs/guides/INSTALLATION.md"
}

# Main function
main() {
    local distro
    
    print_header "VantisOS Development Environment Setup"
    
    # Verify requirements
    verify_requirements
    
    # Detect distribution
    distro=$(detect_distro)
    log_info "Detected distribution: $distro"
    
    # Install dependencies based on distribution
    case $distro in
        ubuntu|debian|linuxmint|pop)
            install_debian_deps
            ;;
        fedora|rhel|centos|rocky|almalinux)
            install_rhel_deps
            ;;
        arch|manjaro|endeavouros)
            install_arch_deps
            ;;
        *)
            log_error "Unsupported distribution: $distro"
            log_info "Please install dependencies manually"
            log_info "Required: build-essential, git, cmake, rust, python3, nodejs, qemu"
            exit 1
            ;;
    esac
    
    # Install pre-commit hooks
    install_precommit
    
    # Configure git
    configure_git
    
    # Create directories
    create_directories
    
    # Set up environment variables
    setup_env_vars
    
    # Print summary
    print_summary
}

# Run main function
main "$@"