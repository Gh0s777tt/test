#!/bin/bash
#
# VantisOS Development Environment Setup Script
# Sets up complete development environment for VantisOS
#
# Usage: ./scripts/dev/setup.sh [--full]

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"

# Configuration
FULL_SETUP="${1:-false}"

print_banner "VantisOS Development Setup" "Environment Configuration Tool"

# ==============================================================================
# Prerequisites Check
# ==============================================================================

print_section "Checking Prerequisites"

MISSING_CMDS=()

# Check for basic commands
for cmd in git curl wget; do
    if ! command_exists "$cmd"; then
        MISSING_CMDS+=("$cmd")
    fi
done

if [[ ${#MISSING_CMDS[@]} -gt 0 ]]; then
    log_error "Missing required commands: ${MISSING_CMDS[*]}"
    log_info "Please install them and run this script again."
    exit 1
fi

log_success "All prerequisites installed"

# ==============================================================================
# Rust Installation
# ==============================================================================

print_section "Rust Installation"

if command_exists rustup; then
    log_info "Rust is already installed"
    rustup --version
    RUST_VERSION=$(rustc --version)
    log_info "Current version: $RUST_VERSION"
else
    log_info "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    log_success "Rust installed successfully"
fi

# Install default toolchain
log_info "Installing VantisOS toolchain: $DEFAULT_TOOLCHAIN"
rustup toolchain install $DEFAULT_TOOLCHAIN || log_warning "Toolchain installation failed, using stable"
rustup default stable

# ==============================================================================
# Rust Components
# ==============================================================================

print_section "Installing Rust Components"

# Essential components
COMPONENTS=(
    "rustfmt"
    "clippy"
    "rust-src"
    "rust-analyzer"
)

for component in "${COMPONENTS[@]}"; do
    log_info "Installing $component..."
    rustup component add "$component" 2>/dev/null || log_warning "Failed to add $component"
done

log_success "Rust components installed"

# ==============================================================================
# Development Tools
# ==============================================================================

print_section "Installing Development Tools"

# Cargo tools (best effort)
CARGO_TOOLS=(
    "cargo-edit"           # For cargo upgrade/upgrade
    "cargo-watch"          # For continuous compilation
    "cargo-outdated"       # Check for outdated dependencies
    "cargo-audit"          # Security audit
    "cargo-binstall"       # Fast binary installation
)

for tool in "${CARGO_TOOLS[@]}"; do
    log_info "Installing $tool..."
    if cargo install "$tool" 2>/dev/null; then
        log_success "Installed $tool"
    else
        log_warning "Failed to install $tool"
    fi
done

# Try installing coverage tool
if ! command_exists cargo-tarpaulin; then
    log_info "Installing cargo-tarpaulin for code coverage..."
    cargo install cargo-tarpaulin 2>/dev/null || log_warning "Failed to install cargo-tarpaulin"
fi

# ==============================================================================
# Pre-commit Hooks
# ==============================================================================

print_section "Setting Up Pre-commit Hooks"

if [[ "$FULL_SETUP" == "true" ]]; then
    if command_exists pre-commit; then
        log_info "Installing pre-commit hooks..."
        pre-commit install 2>/dev/null || log_warning "Pre-commit installation failed"
    else
        log_info "Installing pre-commit..."
        pip3 install pre-commit 2>/dev/null || log_warning "Failed to install pre-commit"
        
        if [[ -f "$VANTIS_ROOT/.pre-commit-config.yaml" ]]; then
            pre-commit install 2>/dev/null || log_warning "Pre-commit installation failed"
        fi
    fi
else
    log_info "Skipping pre-commit hooks (use --full for full setup)"
fi

# ==============================================================================
# Shell Tools
# ==============================================================================

print_section "Shell Development Tools"

# Shellcheck for bash script validation
if ! command_exists shellcheck; then
    log_info "Installing shellcheck..."
    
    if [[ "$VANTIS_OS_TYPE" == "Linux" ]]; then
        # Try various package managers
        if command_exists apt-get; then
            sudo apt-get install -y shellcheck 2>/dev/null || log_warning "Failed to install shellcheck via apt"
        elif command_exists dnf; then
            sudo dnf install -y shellcheck 2>/dev/null || log_warning "Failed to install shellcheck via dnf"
        elif command_exists pacman; then
            sudo pacman -S --noconfirm shellcheck 2>/dev/null || log_warning "Failed to install shellcheck via pacman"
        fi
    fi
    
    # Fallback to direct download
    if ! command_exists shellcheck; then
        log_info "Downloading shellcheck binary..."
        SC_VERSION="v0.9.0"
        wget -qO- "https://github.com/koalaman/shellcheck/releases/download/${SC_VERSION}/shellcheck-${SC_VERSION}.linux.x86_64.tar.xz" | tar xJv
        sudo mv shellcheck-${SC_VERSION}/shellcheck /usr/local/bin/
        rm -rf shellcheck-${SC_VERSION}
    fi
    
    if command_exists shellcheck; then
        log_success "Shellcheck installed"
    else
        log_warning "Failed to install shellcheck"
    fi
else
    log_success "Shellcheck already installed"
fi

# ==============================================================================
# Documentation Tools
# ==============================================================================

print_section "Documentation Tools"

if command_exists mdbook; then
    log_info "mdbook is already installed"
else
    log_info "Installing mdbook..."
    cargo install mdbook 2>/dev/null || log_warning "Failed to install mdbook"
fi

# ==============================================================================
# Git Configuration
# ==============================================================================

print_section "Git Configuration"

# Setup git hooks directory
if [[ -d "$VANTIS_ROOT/.git/hooks" ]]; then
    log_info "Git hooks directory exists"
else
    log_info "Creating .git/hooks directory..."
    mkdir -p "$VANTIS_ROOT/.git/hooks"
fi

# Set up git attributes for line endings
if ! git config core.autocrlf &>/dev/null; then
    log_info "Setting git autocrlf..."
    if [[ "$VANTIS_OS_TYPE" == "Linux" ]]; then
        git config --global core.autocrlf input
    else
        git config --global core.autocrlf true
    fi
fi

# ==============================================================================
# Environment Setup
# ==============================================================================

print_section "Environment Setup"

# Create .env file if it doesn't exist
if [[ ! -f "$VANTIS_ROOT/.env" ]]; then
    log_info "Creating .env file..."
    cat > "$VANTIS_ROOT/.env" << 'EOF'
# VantisOS Development Environment

# Rust Toolchain
DEFAULT_TOOLCHAIN=stable

# Build Configuration
RUSTFLAGS="-D warnings"

# Testing
CARGO_TEST_FLAGS="-- --test-threads=1"

# Development
DEBUG=0
VANTIS_STRICT_MODE=1
EOF
    log_success ".env file created"
else
    log_info ".env file already exists"
fi

# ==============================================================================
# Verification
# ==============================================================================

print_section "Verifying Installation"

# Check Rust installation
log_info "Verifying Rust installation..."
if command_exists rustc; then
    RUST_VERSION=$(rustc --version)
    log_success "Rust: $RUST_VERSION"
else
    log_error "Rust not found"
    exit 1
fi

# Check Cargo installation
if command_exists cargo; then
    CARGO_VERSION=$(cargo --version)
    log_success "Cargo: $CARGO_VERSION"
fi

# Check project builds
log_info "Verifying project configuration..."
if cd "$VANTIS_ROOT" && cargo check 2>/dev/null; then
    log_success "Project configuration is valid"
else
    log_warning "Project check failed (may need dependency installation)"
fi

# ==============================================================================
# Summary
# ==============================================================================

print_banner "Setup Complete"

echo ""
log_success "VantisOS development environment is ready!"
echo ""
log_info "Next steps:"
echo "  1. Run: cargo build          # Build the project"
echo "  2. Run: cargo test           # Run tests"
echo "  3. Run: ./scripts/dev/quality.sh  # Check code quality"
echo ""
log_info "Useful commands:"
echo "  - cargo fmt              # Format code"
echo "  - cargo clippy           # Lint code"
echo "  - cargo watch -x build   # Watch for changes"
echo "  - cargo doc --no-deps --open  # View documentation"
echo ""

if [[ "$FULL_SETUP" != "true" ]]; then
    log_info "For full setup including pre-commit hooks, run:"
    echo "  ./scripts/dev/setup.sh --full"
fi

log_success "Happy hacking! 🚀"