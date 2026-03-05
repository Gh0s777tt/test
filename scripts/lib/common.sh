#!/bin/bash
#
# VantisOS Common Script Library
# Shared functions and configuration for all VantisOS scripts
#
# Usage: source scripts/lib/common.sh

# Ensure script is sourced, not executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "This script should be sourced, not executed directly."
    echo "Usage: source scripts/lib/common.sh"
    exit 1
fi

# ==============================================================================
# Configuration
# ==============================================================================

# Project paths
export VANTIS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export VANTIS_SCRIPTS="$VANTIS_ROOT/scripts"
export VANTIS_DOCS="$VANTIS_ROOT/docs"
export VANTIS_SRC="$VANTIS_ROOT/src"

# Version information
export VANTIS_VERSION="${VANTIS_VERSION:-1.4.1}"
export VANTIS_NAME="VantisOS"

# Build configuration
export CARGO="${CARGO:-cargo}"
export RUSTC="${RUSTC:-rustc}"
export RUSTUP="${RUSTUP:-rustup}"

# Default toolchain
export DEFAULT_TOOLCHAIN="${DEFAULT_TOOLCHAIN:-nightly-2025-10-03}"

# ==============================================================================
# Color Definitions
# ==============================================================================

export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export MAGENTA='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[1;37m'
export NC='\033[0m' # No Color

# Netflix-style colors
export NETFLIX_BLACK='\033[38;5;0m'
export NETFLIX_RED='\033[38;5;196m'
export NETFLIX_WHITE='\033[38;5;255m'

# ==============================================================================
# Logging Functions
# ==============================================================================

# Print info message
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Print success message
log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Print warning message
log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Print error message
log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Print debug message (only if DEBUG is set)
log_debug() {
    if [[ "${DEBUG:-0}" == "1" ]]; then
        echo -e "${MAGENTA}[DEBUG]${NC} $1"
    fi
}

# Print Netflix-style banner
print_banner() {
    local title="${1:-VantisOS}"
    local subtitle="${2:-}"
    
    echo ""
    echo -e "${NETFLIX_RED}========================================${NC}"
    echo -e "${NETFLIX_RED}  ${title}${NC}"
    if [[ -n "$subtitle" ]]; then
        echo -e "${NETFLIX_WHITE}  ${subtitle}${NC}"
    fi
    echo -e "${NETFLIX_RED}========================================${NC}"
    echo ""
}

# Print section header
print_section() {
    echo ""
    echo -e "${CYAN}▶ $1${NC}"
    echo "----------------------------------------"
}

# ==============================================================================
# Error Handling
# ==============================================================================

# Error handler function
error_handler() {
    local exit_code=$?
    local line_number=$1
    log_error "Script failed at line $line_number with exit code $exit_code"
    cleanup_on_exit
    exit $exit_code
}

# Setup error handling
setup_error_handling() {
    set -euo pipefail
    trap 'error_handler ${LINENO}' ERR
}

# Cleanup function (override in scripts if needed)
cleanup_on_exit() {
    log_debug "Running cleanup..."
}

# ==============================================================================
# Utility Functions
# ==============================================================================

# Check if command exists
command_exists() {
    command -v "$1" &>/dev/null
}

# Require command or exit
require_command() {
    if ! command_exists "$1"; then
        log_error "Required command '$1' not found. Please install it first."
        exit 1
    fi
}

# Check if we're in a VantisOS repository
check_repository() {
    if [[ ! -f "$VANTIS_ROOT/Cargo.toml" ]]; then
        log_error "Not in a VantisOS repository. Cargo.toml not found."
        exit 1
    fi
}

# Get current git branch
get_git_branch() {
    git -C "$VANTIS_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown"
}

# Get current git commit hash
get_git_commit() {
    git -C "$VANTIS_ROOT" rev-parse --short HEAD 2>/dev/null || echo "unknown"
}

# Check if working directory is clean
is_git_clean() {
    git -C "$VANTIS_ROOT" diff-index --quiet HEAD -- 2>/dev/null
}

# ==============================================================================
# Build Functions
# ==============================================================================

# Build the project
build_project() {
    local release="${1:-false}"
    
    print_section "Building VantisOS"
    
    if [[ "$release" == "true" ]]; then
        log_info "Building in release mode..."
        $CARGO build --release
    else
        log_info "Building in debug mode..."
        $CARGO build
    fi
    
    log_success "Build complete"
}

# Run tests
run_tests() {
    local coverage="${1:-false}"
    
    print_section "Running Tests"
    
    if [[ "$coverage" == "true" ]] && command_exists cargo-tarpaulin; then
        log_info "Running tests with coverage..."
        cargo tarpaulin --out Html
    else
        log_info "Running tests..."
        $CARGO test
    fi
    
    log_success "Tests complete"
}

# Clean build artifacts
clean_build() {
    print_section "Cleaning Build Artifacts"
    
    log_info "Running cargo clean..."
    $CARGO clean
    
    log_info "Removing target directories..."
    find "$VANTIS_ROOT" -type d -name "target" -exec rm -rf {} + 2>/dev/null || true
    
    log_success "Clean complete"
}

# ==============================================================================
# Validation Functions
# ==============================================================================

# Validate Rust code
validate_rust() {
    print_section "Validating Rust Code"
    
    log_info "Running cargo check..."
    $CARGO check
    
    log_info "Running clippy..."
    $CARGO clippy -- -D warnings
    
    log_info "Checking formatting..."
    $CARGO fmt --check
    
    log_success "Validation complete"
}

# Validate shell scripts
validate_scripts() {
    print_section "Validating Shell Scripts"
    
    local scripts=($(find "$VANTIS_SCRIPTS" -name "*.sh"))
    local errors=0
    
    for script in "${scripts[@]}"; do
        if command_exists shellcheck; then
            if ! shellcheck "$script" &>/dev/null; then
                log_warning "Issues found in $(basename "$script")"
                ((errors++))
            fi
        fi
    done
    
    if [[ $errors -eq 0 ]]; then
        log_success "All scripts pass validation"
    else
        log_warning "$errors scripts have issues"
    fi
}

# ==============================================================================
# Progress Indicators
# ==============================================================================

# Show spinner (for long operations)
show_spinner() {
    local pid=$1
    local message="${2:-Processing}"
    local spin='-\|/'
    local i=0
    
    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) % 4 ))
        printf "\r${CYAN}[${spin:$i:1}]${NC} $message..."
        sleep 0.1
    done
    
    printf "\r"
}

# ==============================================================================
# Configuration Management
# ==============================================================================

# Load environment from .env file
load_env() {
    local env_file="${1:-$VANTIS_ROOT/.env}"
    
    if [[ -f "$env_file" ]]; then
        log_info "Loading environment from $env_file"
        set -a
        source "$env_file"
        set +a
    fi
}

# Save environment to .env file
save_env() {
    local env_file="${1:-$VANTIS_ROOT/.env}"
    
    log_info "Saving environment to $env_file"
    # Implementation depends on what needs to be saved
}

# ==============================================================================
# Initialization
# ==============================================================================

# Initialize common library
init_common() {
    # Set up error handling if requested
    if [[ "${VANTIS_STRICT_MODE:-0}" == "1" ]]; then
        setup_error_handling
    fi
    
    # Export useful variables
    export VANTIS_OS_TYPE="$(uname -s)"
    export VANTIS_ARCH="$(uname -m)"
    
    log_debug "Common library initialized"
    log_debug "Repository: $VANTIS_ROOT"
    log_debug "Platform: $VANTIS_OS_TYPE ($VANTIS_ARCH)"
}

# Run initialization
init_common