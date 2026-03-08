#!/bin/bash
# Script: health_check.sh
# Purpose: Comprehensive repository health check for VantisOS
# Usage: ./scripts/health_check.sh [--verbose] [--json] [--fix]
# Requirements: bash, git, jq (optional for JSON output)
# Author: VantisOS Team
# Date: 2025-03-06
# Version: 1.0.0
# License: MIT

set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Parse arguments
VERBOSE=false
JSON_OUTPUT=false
AUTO_FIX=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --json|-j)
            JSON_OUTPUT=true
            shift
            ;;
        --fix|-f)
            AUTO_FIX=true
            shift
            ;;
        --help|-h)
            echo "Usage: $(basename "$0") [options]"
            echo ""
            echo "Options:"
            echo "  --verbose, -v    Show detailed output"
            echo "  --json, -j       Output in JSON format"
            echo "  --fix, -f        Attempt to fix issues automatically"
            echo "  --help, -h       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Health check results
declare -a CHECKS_PASSED=()
declare -a CHECKS_FAILED=()
declare -a CHECKS_WARNING=()

# Helper functions
log_pass() {
    CHECKS_PASSED+=("$1")
    if [[ "$JSON_OUTPUT" == false ]]; then
        echo -e "${GREEN}✓${NC} $1"
    fi
}

log_fail() {
    CHECKS_FAILED+=("$1")
    if [[ "$JSON_OUTPUT" == false ]]; then
        echo -e "${RED}✗${NC} $1"
    fi
}

log_warn() {
    CHECKS_WARNING+=("$1")
    if [[ "$JSON_OUTPUT" == false ]]; then
        echo -e "${YELLOW}!${NC} $1"
    fi
}

log_info() {
    if [[ "$VERBOSE" == true ]] && [[ "$JSON_OUTPUT" == false ]]; then
        echo -e "${BLUE}ℹ${NC} $1"
    fi
}

# Check functions
check_git_status() {
    cd "$REPO_ROOT"
    
    if git rev-parse --is-inside-work-tree &>/dev/null; then
        log_pass "Git repository detected"
        
        # Check for uncommitted changes
        if [[ -z $(git status --porcelain) ]]; then
            log_pass "Working directory clean"
        else
            log_warn "Uncommitted changes detected"
            if [[ "$VERBOSE" == true ]]; then
                git status --short
            fi
        fi
        
        # Check for untracked files
        local untracked
        untracked=$(git ls-files --others --exclude-standard | wc -l)
        if [[ "$untracked" -gt 0 ]]; then
            log_warn "$untracked untracked files found"
        fi
    else
        log_fail "Not a git repository"
    fi
}

check_required_files() {
    local required_files=(
        "README.md"
        "LICENSE"
        "Cargo.toml"
        ".gitignore"
        "Makefile"
    )
    
    for file in "${required_files[@]}"; do
        if [[ -f "$REPO_ROOT/$file" ]]; then
            log_pass "Required file exists: $file"
        else
            log_fail "Missing required file: $file"
        fi
    done
}

check_documentation() {
    local doc_dir="$REPO_ROOT/docs"
    
    if [[ -d "$doc_dir" ]]; then
        log_pass "Documentation directory exists"
        
        # Check for key documentation files
        local key_docs=(
            "CONTRIBUTING.md"
            "SCRIPTS_REFERENCE.md"
            "SCRIPTING_STANDARDS.md"
            "AUTOMATION_GUIDE.md"
        )
        
        for doc in "${key_docs[@]}"; do
            if [[ -f "$doc_dir/$doc" ]] || [[ -L "$REPO_ROOT/$doc" ]]; then
                log_pass "Documentation found: $doc"
            else
                log_warn "Missing documentation: $doc"
            fi
        done
    else
        log_fail "Documentation directory missing"
    fi
}

check_scripts() {
    local scripts_dir="$REPO_ROOT/scripts"
    
    if [[ -d "$scripts_dir" ]]; then
        log_pass "Scripts directory exists"
        
        # Check script permissions
        local script_count=0
        local executable_count=0
        
        while IFS= read -r -d '' script; do
            script_count=$((script_count + 1))
            if [[ -x "$script" ]]; then
                executable_count=$((executable_count + 1))
            else
                local script_name
                script_name=$(basename "$script")
                log_warn "Script not executable: $script_name"
                
                if [[ "$AUTO_FIX" == true ]]; then
                    chmod +x "$script"
                    log_info "Fixed: Made $script_name executable"
                fi
            fi
        done < <(find "$scripts_dir" -name "*.sh" -type f -print0)
        
        log_info "Found $script_count scripts, $executable_count executable"
    else
        log_fail "Scripts directory missing"
    fi
}

check_github_workflows() {
    local workflows_dir="$REPO_ROOT/.github/workflows"
    
    if [[ -d "$workflows_dir" ]]; then
        log_pass "GitHub workflows directory exists"
        
        local workflow_count
        workflow_count=$(find "$workflows_dir" -name "*.yml" -type f | wc -l)
        log_info "Found $workflow_count workflow files"
        
        # Check for common workflow files
        local common_workflows=("ci.yml" "build.yml" "test.yml")
        for workflow in "${common_workflows[@]}"; do
            if [[ -f "$workflows_dir/$workflow" ]]; then
                log_pass "Workflow exists: $workflow"
            else
                log_warn "Missing common workflow: $workflow"
            fi
        done
    else
        log_warn "GitHub workflows directory missing"
    fi
}

check_pre_commit() {
    local pre_commit_config="$REPO_ROOT/.pre-commit-config.yaml"
    
    if [[ -f "$pre_commit_config" ]]; then
        log_pass "Pre-commit configuration exists"
        
        # Check if pre-commit is installed
        if command -v pre-commit &>/dev/null; then
            log_pass "Pre-commit is installed"
            
            # Check if hooks are installed
            if [[ -f "$REPO_ROOT/.git/hooks/pre-commit" ]]; then
                log_pass "Pre-commit hooks are installed"
            else
                log_warn "Pre-commit hooks not installed"
                
                if [[ "$AUTO_FIX" == true ]]; then
                    cd "$REPO_ROOT"
                    pre-commit install
                    log_info "Fixed: Installed pre-commit hooks"
                fi
            fi
        else
            log_warn "Pre-commit not installed (pip install pre-commit)"
        fi
    else
        log_warn "Pre-commit configuration missing"
    fi
}

check_code_quality() {
    cd "$REPO_ROOT"
    
    # Check Rust formatting
    if command -v cargo &>/dev/null; then
        log_pass "Rust/Cargo detected"
        
        if command -v rustfmt &>/dev/null; then
            log_pass "rustfmt available"
        else
            log_warn "rustfmt not available (rustup component add rustfmt)"
        fi
        
        if command -v cargo-clippy &>/dev/null; then
            log_pass "clippy available"
        else
            log_warn "clippy not available (rustup component add clippy)"
        fi
    else
        log_warn "Rust/Cargo not detected"
    fi
    
    # Check shell script linting
    if command -v shellcheck &>/dev/null; then
        log_pass "shellcheck available"
    else
        log_warn "shellcheck not installed (apt install shellcheck)"
    fi
}

check_security() {
    cd "$REPO_ROOT"
    
    # Check for security policy
    if [[ -f "SECURITY.md" ]] || [[ -f "docs/security/SECURITY.md" ]]; then
        log_pass "Security policy exists"
    else
        log_warn "Security policy missing"
    fi
    
    # Check for CodeQL or similar
    if [[ -f ".github/workflows/codeql.yml" ]]; then
        log_pass "CodeQL workflow exists"
    else
        log_info "Consider adding CodeQL for security scanning"
    fi
    
    # Check for dependency scanning
    if [[ -f ".github/workflows/dependency-validation.yml" ]] || \
       [[ -f ".github/workflows/dependency-review.yml" ]]; then
        log_pass "Dependency validation workflow exists"
    else
        log_warn "Dependency validation workflow missing"
    fi
}

check_dependencies() {
    cd "$REPO_ROOT"
    
    # Rust dependencies
    if [[ -f "Cargo.lock" ]]; then
        log_pass "Cargo.lock exists (Rust dependencies locked)"
    fi
    
    # Check for outdated dependencies (if cargo-outdated is available)
    if command -v cargo-outdated &>/dev/null; then
        log_info "cargo-outdated available for checking outdated dependencies"
    fi
    
    # Check for audit tools
    if command -v cargo-audit &>/dev/null; then
        log_pass "cargo-audit available for security auditing"
    else
        log_info "Install cargo-audit for dependency security auditing"
    fi
}

print_summary() {
    # Handle empty arrays with defaults
    local passed_count=${#CHECKS_PASSED[@]}
    local failed_count=${#CHECKS_FAILED[@]}
    local warning_count=${#CHECKS_WARNING[@]}
    
    if [[ "$JSON_OUTPUT" == true ]]; then
        local json_passed=""
        local json_failed=""
        local json_warnings=""
        
        [[ ${#CHECKS_PASSED[@]} -gt 0 ]] && json_passed="$(printf '%s\n' "${CHECKS_PASSED[@]}")"
        [[ ${#CHECKS_FAILED[@]} -gt 0 ]] && json_failed="$(printf '%s\n' "${CHECKS_FAILED[@]}")"
        [[ ${#CHECKS_WARNING[@]} -gt 0 ]] && json_warnings="$(printf '%s\n' "${CHECKS_WARNING[@]}")"
        
        jq -n \
            --argjson passed "$passed_count" \
            --argjson failed "$failed_count" \
            --argjson warnings "$warning_count" \
            --arg json_passed "$json_passed" \
            --arg json_failed "$json_failed" \
            --arg json_warnings "$json_warnings" \
            '{
                summary: {
                    passed: $passed,
                    failed: $failed,
                    warnings: $warnings,
                    total: ($passed + $failed + $warnings)
                },
                checks: {
                    passed: ($json_passed | split("\n") | map(select(length > 0))),
                    failed: ($json_failed | split("\n") | map(select(length > 0))),
                    warnings: ($json_warnings | split("\n") | map(select(length > 0)))
                }
            }'
    else
        echo ""
        echo "========================================="
        echo "         REPOSITORY HEALTH SUMMARY"
        echo "========================================="
        echo -e "${GREEN}Passed:${NC}   $passed_count"
        echo -e "${RED}Failed:${NC}   $failed_count"
        echo -e "${YELLOW}Warnings:${NC} $warning_count"
        echo "========================================="
        
        if [[ $failed_count -eq 0 ]]; then
            echo -e "${GREEN}Repository is healthy!${NC}"
        else
            echo -e "${RED}Repository has issues that need attention.${NC}"
            echo ""
            echo "Failed checks:"
            for check in "${CHECKS_FAILED[@]}"; do
                echo "  - $check"
            done
        fi
        
        if [[ $warning_count -gt 0 ]] && [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Warnings:"
            for check in "${CHECKS_WARNING[@]}"; do
                echo "  - $check"
            done
        fi
    fi
}

# Main execution
main() {
    if [[ "$JSON_OUTPUT" == false ]]; then
        echo ""
        echo "VantisOS Repository Health Check"
        echo "================================="
    fi
    
    check_git_status
    check_required_files
    check_documentation
    check_scripts
    check_github_workflows
    check_pre_commit
    check_code_quality
    check_security
    check_dependencies
    
    print_summary
}

# Run main function
main