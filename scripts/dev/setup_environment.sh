#!/bin/bash
# Script: setup_environment.sh
# Purpose: Complete development environment setup for VantisOS
# Usage: ./scripts/dev/setup_environment.sh [--full] [--skip-packages]
# Requirements: bash, sudo (for package installation)
# Author: VantisOS Team
# Date: 2025-03-06
# Version: 1.0.0
# License: MIT

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/../lib/common.sh"

# Configuration
FULL_INSTALL=false
SKIP_PACKAGES=false
VERBOSE=false

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --full|-f)
            FULL_INSTALL=true
            shift
            ;;
        --skip-packages|-s)
            SKIP_PACKAGES=true
            shift
            ;;
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --help|-h)
            echo "Usage: $(basename "$0") [options]"
            echo ""
            echo "Options:"
            echo "  --full, -f          Install all optional dependencies (QEMU, Docker, etc.)"
            echo "  --skip-packages, -s Skip package installation, only configure environment"
            echo "  --verbose, -v       Enable verbose output"
            echo "  --help, -h          Show this help message"
            echo ""
            echo "This script sets up the complete development environment for VantisOS."
            echo "It will install build tools, development dependencies, and configure the system."
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

print_header "VantisOS Development Environment Setup"

# Detect Linux distribution
detect_distro() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        echo "$ID"
    elif [[ -f /etc/redhat-release ]]; then
        echo "rhel"
    else
        echo "unknown"
    fi
}

DISTRO=$(detect_distro)
log_info "Detected distribution: $DISTRO"

# Function to install packages on Debian/Ubuntu
install_debian_deps() {
    local packages=(
        build-essential
        git
        cmake
        make
        ninja-build
        pkg-config
        libssl-dev
        libclang-dev
        curl
        wget
        jq
        yamllint
        shellcheck
        python3
        python3-pip
        python3-venv
        nodejs
        npm
    )

    if [[ "$FULL_INSTALL" == true ]]; then
        packages+=(
            qemu-system-x86
            qemu-kvm
            libvirt-daemon-system
            libvirt-clients
            virtinst
            bridge-utils
            vagrant
            docker.io
            docker-compose
            containerd
        )
    fi

    log_info "Updating package lists..."
    sudo apt-get update -qq

    log_info "Installing ${#packages[@]} packages..."
    sudo apt-get install -y -qq "${packages[@]}"

    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
}

# Function to install packages on Fedora/RHEL
install_fedora_deps() {
    local packages=(
        gcc
        gcc-c++
        git
        cmake
        make
        ninja-build
        pkg-config
        openssl-devel
        clang-devel
        curl
        wget
        jq
        yamllint
        ShellCheck
        python3
        python3-pip
        nodejs
        npm
    )

    if [[ "$FULL_INSTALL" == true ]]; then
        packages+=(
            qemu-kvm
            libvirt
            virt-install
            bridge-utils
            vagrant
            docker
            docker-compose
            containerd
        )
    fi

    log_info "Installing ${#packages[@]} packages..."
    sudo dnf install -y -q "${packages[@]}"

    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
}

# Function to install packages on Arch Linux
install_arch_deps() {
    local packages=(
        base-devel
        git
        cmake
        make
        ninja
        pkg-config
        openssl
        clang
        curl
        wget
        jq
        yamllint
        shellcheck
        python
        python-pip
        nodejs
        npm
    )

    if [[ "$FULL_INSTALL" == true ]]; then
        packages+=(
            qemu
            libvirt
            virt-manager
            vagrant
            docker
            docker-compose
            containerd
        )
    fi

    log_info "Installing ${#packages[@]} packages..."
    sudo pacman -S --noconfirm --needed "${packages[@]}"

    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
}

# Install Python packages
install_python_tools() {
    log_info "Installing Python development tools..."
    pip3 install --quiet --user \
        pre-commit \
        black \
        isort \
        flake8 \
        mypy \
        pytest \
        pytest-cov \
        sphinx \
        sphinx-rtd-theme \
        mkdocs \
        mkdocs-material \
        mkdocs-mermaid2-plugin
}

# Install Node.js packages
install_node_tools() {
    log_info "Installing Node.js development tools..."
    npm install --global --quiet \
        prettier \
        eslint \
        markdownlint-cli \
        @commitlint/cli \
        @commitlint/config-conventional
}

# Configure Git
configure_git() {
    log_info "Configuring Git..."

    if ! git config --global user.name &> /dev/null; then
        echo -e "${YELLOW}Git user name not configured${NC}"
        read -p "Enter your Git user name: " git_name
        git config --global user.name "$git_name"
    fi

    if ! git config --global user.email &> /dev/null; then
        echo -e "${YELLOW}Git user email not configured${NC}"
        read -p "Enter your Git user email: " git_email
        git config --global user.email "$git_email"
    fi

    # Set useful defaults
    git config --global init.defaultBranch main
    git config --global core.autocrlf input
    git config --global pull.rebase false
}

# Install pre-commit hooks
install_pre_commit() {
    if [[ -f .pre-commit-config.yaml ]]; then
        log_info "Installing pre-commit hooks..."
        pre-commit install
    else
        log_warn ".pre-commit-config.yaml not found, skipping pre-commit setup"
    fi
}

# Create development directories
create_dev_directories() {
    log_info "Creating development directories..."
    mkdir -p build
    mkdir -p logs
    mkdir -p temp
    mkdir -p output
}

# Print summary
print_summary() {
    print_header "Setup Complete"

    echo -e "${GREEN}✓${NC} Development environment configured successfully"
    echo ""
    echo "Next steps:"
    echo "  1. Source the Rust environment: source ~/.cargo/env"
    echo "  2. Build the project: make build"
    echo "  3. Run tests: make test"
    echo "  4. View available commands: make help"
    echo ""

    if [[ "$FULL_INSTALL" == true ]]; then
        echo "Additional tools installed:"
        echo "  - QEMU/KVM for virtualization"
        echo "  - Docker for containerization"
        echo "  - Vagrant for VM management"
        echo ""
    fi

    echo "For more information, see:"
    echo "  - docs/AUTOMATION_GUIDE.md"
    echo "  - docs/SCRIPTING_STANDARDS.md"
}

# Main execution
main() {
    # Install packages
    if [[ "$SKIP_PACKAGES" == false ]]; then
        case $DISTRO in
            ubuntu|debian|pop|linuxmint|kali)
                install_debian_deps
                ;;
            fedora|rhel|centos)
                install_fedora_deps
                ;;
            arch|manjaro|endeavouros)
                install_arch_deps
                ;;
            *)
                log_error "Unsupported distribution: $DISTRO"
                log_info "Please install dependencies manually for your system"
                exit 1
                ;;
        esac

        install_python_tools
        install_node_tools
    else
        log_info "Skipping package installation (--skip-packages flag set)"
    fi

    # Configure environment
    configure_git
    create_dev_directories
    install_pre_commit

    # Print summary
    print_summary

    log_info "Setup completed successfully!"
}

# Run main function
main