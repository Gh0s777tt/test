#!/bin/bash
#
# VantisOS Code Quality Check Script
# Runs comprehensive code quality checks
#
# Usage: ./scripts/dev/quality.sh [--fix] [--all]

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"

# Configuration
FIX_ISSUES="${1:-false}"
RUN_ALL="${2:-false}"

print_banner "VantisOS Code Quality Check" "Comprehensive Quality Analysis"

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNING_CHECKS=0

# ==============================================================================
# Helper Functions
# ==============================================================================

check_pass() {
    local name="$1"
    log_success "✓ $name"
    ((PASSED_CHECKS++))
    ((TOTAL_CHECKS++))
}

check_fail() {
    local name="$1"
    local details="${2:-}"
    log_error "✗ $name"
    if [[ -n "$details" ]]; then
        echo "  $details"
    fi
    ((FAILED_CHECKS++))
    ((TOTAL_CHECKS++))
}

check_warn() {
    local name="$1"
    local details="${2:-}"
    log_warning "⚠ $name"
    if [[ -n "$details" ]]; then
        echo "  $details"
    fi
    ((WARNING_CHECKS++))
    ((TOTAL_CHECKS++))
}

# ==============================================================================
# Rust Code Quality
# ==============================================================================

print_section "Rust Code Quality"

# 1. Check code formatting
log_info "Checking Rust code formatting..."
if command_exists cargo; then
    if [[ "$FIX_ISSUES" == "true" ]]; then
        log_info "Auto-fixing formatting issues..."
        cargo fmt --all
        check_pass "Code formatting (auto-fixed)"
    else
        if cargo fmt --all --check 2>/dev/null; then
            check_pass "Code formatting"
        else
            check_fail "Code formatting" "Run 'cargo fmt' to fix"
        fi
    fi
else
    check_fail "Cargo not found"
fi

# 2. Run Clippy
log_info "Running Clippy linter..."
if cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null; then
    check_pass "Clippy checks"
else
    CLIPPY_OUTPUT=$(cargo clippy --all-targets --all-features -- -D warnings 2>&1 || true)
    if [[ "$FIX_ISSUES" == "true" ]]; then
        check_warn "Clippy warnings" "Some warnings cannot be auto-fixed"
    else
        check_fail "Clippy checks" "Review and fix clippy warnings"
    fi
fi

# 3. Check for unused dependencies
log_info "Checking for unused dependencies..."
if command_exists cargo-udeps; then
    if cargo +nightly udeps 2>/dev/null; then
        check_pass "No unused dependencies"
    else
        check_warn "Unused dependencies found" "Run 'cargo udeps' for details"
    fi
else
    check_warn "cargo-udeps not installed" "Install with: cargo install cargo-udeps"
fi

# 4. Check for outdated dependencies
if [[ "$RUN_ALL" == "true" ]]; then
    log_info "Checking for outdated dependencies..."
    if command_exists cargo-outdated; then
        cargo outdated --depth 1 2>/dev/null || check_warn "Outdated dependencies check failed"
        check_pass "Outdated dependencies check"
    else
        check_warn "cargo-outdated not installed" "Install with: cargo install cargo-outdated"
    fi
fi

# ==============================================================================
# Shell Script Quality
# ==============================================================================

print_section "Shell Script Quality"

# Find all shell scripts
SHELL_SCRIPTS=($(find "$VANTIS_SCRIPTS" -name "*.sh" -type f))
if [[ ${#SHELL_SCRIPTS[@]} -eq 0 ]]; then
    log_warning "No shell scripts found"
else
    log_info "Found ${#SHELL_SCRIPTS[@]} shell scripts"
    
    # Run shellcheck on all scripts
    SHELLCHECK_ERRORS=0
    for script in "${SHELL_SCRIPTS[@]}"; do
        if command_exists shellcheck; then
            if ! shellcheck "$script" &>/dev/null; then
                ((SHELLCHECK_ERRORS++))
                echo "  Issues in: $(basename "$script")"
            fi
        else
            check_warn "Shellcheck not installed"
            break
        fi
    done
    
    if [[ $SHELLCHECK_ERRORS -eq 0 ]]; then
        check_pass "Shell script validation (${#SHELL_SCRIPTS[@]} scripts)"
    else
        check_fail "Shell script validation" "$SHELLCHECK_ERRORS scripts have issues"
    fi
fi

# Check for non-executable scripts
NON_EXEC=()
for script in "${SHELL_SCRIPTS[@]}"; do
    if [[ ! -x "$script" ]]; then
        NON_EXEC+=("$(basename "$script")")
    fi
done

if [[ ${#NON_EXEC[@]} -eq 0 ]]; then
    check_pass "All scripts are executable"
else
    check_warn "Non-executable scripts found" "${NON_EXEC[*]}"
    if [[ "$FIX_ISSUES" == "true" ]]; then
        log_info "Making scripts executable..."
        for script in "${SHELL_SCRIPTS[@]}"; do
            chmod +x "$script"
        done
        check_pass "Made scripts executable"
    fi
fi

# ==============================================================================
# Documentation Quality
# ==============================================================================

print_section "Documentation Quality"

# Check for broken links in markdown
if command_exists markdown-link-check; then
    log_info "Checking markdown links..."
    MD_FILES=($(find "$VANTIS_DOCS" -name "*.md" -type f | head -10))
    BROKEN_LINKS=0
    
    for md_file in "${MD_FILES[@]}"; do
        if ! markdown-link-check "$md_file" -q 2>/dev/null; then
            ((BROKEN_LINKS++))
        fi
    done
    
    if [[ $BROKEN_LINKS -eq 0 ]]; then
        check_pass "Markdown links validation"
    else
        check_warn "Broken markdown links" "$BROKEN_LINKS files have broken links"
    fi
else
    check_warn "markdown-link-check not installed"
fi

# Check for README in all important directories
log_info "Checking for README files..."
MISSING_README=()
for dir in "$VANTIS_DOCS"/{api,architecture,development,guides}; do
    if [[ -d "$dir" ]] && [[ ! -f "$dir/README.md" ]]; then
        MISSING_README+=("$(basename "$dir")")
    fi
done

if [[ ${#MISSING_README[@]} -eq 0 ]]; then
    check_pass "README files present"
else
    check_warn "Missing README files" "${MISSING_README[*]}"
fi

# ==============================================================================
# Security Quality
# ==============================================================================

print_section "Security Quality"

# Run cargo audit
if command_exists cargo-audit; then
    log_info "Running security audit..."
    if cargo audit 2>/dev/null; then
        check_pass "Security audit"
    else
        check_fail "Security audit" "Review cargo audit output"
    fi
else
    check_warn "cargo-audit not installed" "Install with: cargo install cargo-audit"
fi

# Check for hardcoded secrets
log_info "Checking for potential secrets..."
SECRET_PATTERNS=(
    "password"
    "api_key"
    "secret"
    "token"
    "private_key"
)

SECRETS_FOUND=0
for pattern in "${SECRET_PATTERNS[@]}"; do
    if git -C "$VANTIS_ROOT" grep -i --count "$pattern" -- "*.rs" "*.toml" "*.yaml" "*.yml" 2>/dev/null | grep -q "[1-9]"; then
        ((SECRETS_FOUND++))
    fi
done

if [[ $SECRETS_FOUND -eq 0 ]]; then
    check_pass "No obvious hardcoded secrets"
else
    check_warn "Potential secrets found" "Review code for hardcoded credentials"
fi

# ==============================================================================
# Build Quality
# ==============================================================================

print_section "Build Quality"

# Check if project builds
log_info "Checking if project compiles..."
if cargo check --all-targets 2>/dev/null; then
    check_pass "Project compilation check"
else
    check_fail "Project compilation" "Fix compilation errors"
fi

# Check if tests pass
log_info "Running tests..."
if cargo test --quiet 2>/dev/null; then
    check_pass "Test suite"
else
    check_warn "Test suite has failures" "Review test output"
fi

# ==============================================================================
# Git Quality
# ==============================================================================

print_section "Git Quality"

# Check for large files
log_info "Checking for large files..."
LARGE_FILES=$(git -C "$VANTIS_ROOT" ls-files | while read file; do
    size=$(git -C "$VANTIS_ROOT" cat-file -s ":$file" 2>/dev/null || echo 0)
    if [[ $size -gt 1048576 ]]; then  # > 1MB
        echo "$file ($((size / 1024))KB)"
    fi
done)

if [[ -z "$LARGE_FILES" ]]; then
    check_pass "No large files tracked"
else
    check_warn "Large files tracked" "$LARGE_FILES"
fi

# Check for merge conflicts
CONFLICTS=$(git -C "$VANTIS_ROOT" ls-files -u 2>/dev/null | wc -l)
if [[ $CONFLICTS -eq 0 ]]; then
    check_pass "No merge conflicts"
else
    check_fail "Merge conflicts found" "$CONFLICTS files have conflicts"
fi

# Check for uncommitted changes
if is_git_clean; then
    check_pass "Working tree clean"
else
    check_warn "Uncommitted changes" "Review git status"
fi

# ==============================================================================
# Summary
# ==============================================================================

print_banner "Quality Check Summary"

echo ""
echo "Total Checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
if [[ $WARNING_CHECKS -gt 0 ]]; then
    echo -e "${YELLOW}Warnings: $WARNING_CHECKS${NC}"
fi
if [[ $FAILED_CHECKS -gt 0 ]]; then
    echo -e "${RED}Failed: $FAILED_CHECKS${NC}"
fi
echo ""

if [[ $FAILED_CHECKS -eq 0 ]]; then
    log_success "All critical checks passed! ✨"
    exit 0
else
    log_error "Some checks failed. Please review the output above."
    
    if [[ "$FIX_ISSUES" != "true" ]]; then
        echo ""
        log_info "Run with --fix to auto-fix some issues:"
        echo "  ./scripts/dev/quality.sh --fix"
    fi
    
    exit 1
fi