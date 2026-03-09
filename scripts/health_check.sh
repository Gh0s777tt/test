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
declare -a CHECKS_PASSED
declare -a CHECKS_FAILED
declare -a CHECKS_WARNING

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
            ((script_count++))
            if [[ -x "$script" ]]; then
                ((executable_count++))
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
    if [[ "$JSON_OUTPUT" == true ]]; then
        jq -n \
            --argjson passed "${#CHECKS_PASSED[@]}" \
            --argjson failed "${#CHECKS_FAILED[@]}" \
            --argjson warnings "${#CHECKS_WARNING[@]}" \
            --arg json_passed "$(printf '%s\n' "${CHECKS_PASSED[@]}")" \
            --arg json_failed "$(printf '%s\n' "${CHECKS_FAILED[@]}")" \
            --arg json_warnings "$(printf '%s\n' "${CHECKS_WARNING[@]}")" \
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
        echo -e "${GREEN}Passed:${NC}   ${#CHECKS_PASSED[@]}"
        echo -e "${RED}Failed:${NC}   ${#CHECKS_FAILED[@]}"
        echo -e "${YELLOW}Warnings:${NC} ${#CHECKS_WARNING[@]}"
        echo "========================================="
        
        if [[ ${#CHECKS_FAILED[@]} -eq 0 ]]; then
            echo -e "${GREEN}Repository is healthy!${NC}"
        else
            echo -e "${RED}Repository has issues that need attention.${NC}"
            echo ""
            echo "Failed checks:"
            for check in "${CHECKS_FAILED[@]}"; do
                echo "  - $check"
            done
        fi
        
        if [[ ${#CHECKS_WARNING[@]} -gt 0 ]] && [[ "$VERBOSE" == true ]]; then
            echo ""
            echo "Warnings:"
            for check in "${CHECKS_WARNING[@]}"; do
                echo "  - $check"
            done
        fi
    fi
}

check_workspace_members() {
    cd "$REPO_ROOT"
    
    if [[ ! -f "Cargo.toml" ]]; then
        log_fail "Cargo.toml not found at repository root"
        return
    fi
    
    # Extract workspace members from Cargo.toml
    local members
    members=$(grep -A 50 '^\[workspace\]' Cargo.toml | grep '"userspace/' | sed 's/.*"\(.*\)".*/\1/' || true)
    
    if [[ -z "$members" ]]; then
        log_fail "No workspace members found in Cargo.toml"
        return
    fi
    
    local total=0
    local found=0
    local missing=0
    
    while IFS= read -r member; do
        total=$((total + 1))
        if [[ -d "$member" ]] && [[ -f "$member/Cargo.toml" ]]; then
            found=$((found + 1))
        else
            missing=$((missing + 1))
            log_fail "Workspace member missing: $member"
        fi
    done <<< "$members"
    
    if [[ $missing -eq 0 ]]; then
        log_pass "All $total workspace members present with Cargo.toml"
    else
        log_fail "$missing of $total workspace members missing"
    fi
    
    # Check Cargo.lock exists
    if [[ -f "Cargo.lock" ]]; then
        log_pass "Cargo.lock present (reproducible builds)"
    else
        log_fail "Cargo.lock missing (builds not reproducible)"
    fi
}

check_version_consistency() {
    cd "$REPO_ROOT"
    
    # Get version from Cargo.toml
    local cargo_version
    cargo_version=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/' || echo "unknown")
    
    if [[ "$cargo_version" == "unknown" ]]; then
        log_fail "Cannot read version from Cargo.toml"
        return
    fi
    
    log_info "Cargo.toml version: $cargo_version"
    
    # Check CITATION.cff
    if [[ -f "CITATION.cff" ]]; then
        local cff_version
        cff_version=$(grep '^version:' CITATION.cff | awk '{print $2}' || echo "unknown")
        if [[ "$cff_version" == "$cargo_version" ]]; then
            log_pass "CITATION.cff version matches ($cff_version)"
        else
            log_fail "CITATION.cff version mismatch: $cff_version (expected $cargo_version)"
        fi
    fi
    
    # Check README.md for version references
    if [[ -f "README.md" ]]; then
        if grep -q "v${cargo_version}" README.md; then
            log_pass "README.md references current version v${cargo_version}"
        else
            log_warn "README.md may not reference current version v${cargo_version}"
        fi
    fi
    
    # Check for inflated version references
    local inflated
    inflated=$(grep -rlE "v1\.[0-9]+\.[0-9]|v[2-9]\." README.md CHANGELOG.md SECURITY.md CITATION.cff 2>/dev/null | head -5 || true)
    if [[ -n "$inflated" ]]; then
        log_warn "Possible inflated version references found in: $inflated"
    else
        log_pass "No inflated version references detected"
    fi
}

check_ci_integrity() {
    cd "$REPO_ROOT"
    
    # Check for error masking in CI workflows
    local masked
    masked=$(grep -rn '2>/dev/null.*||.*echo' .github/workflows/*.yml 2>/dev/null | grep -v '#' | head -5 || true)
    if [[ -n "$masked" ]]; then
        log_fail "Error masking detected in CI workflows (2>/dev/null || echo)"
        if [[ "$VERBOSE" == true ]]; then
            echo "$masked"
        fi
    else
        log_pass "No error masking in CI workflows"
    fi
    
    # Check for continue-on-error in critical steps
    local coe
    coe=$(grep -rn 'continue-on-error: true' .github/workflows/ci.yml .github/workflows/build.yml .github/workflows/test.yml 2>/dev/null || true)
    if [[ -n "$coe" ]]; then
        log_warn "continue-on-error found in CI workflows (may hide failures)"
    else
        log_pass "No continue-on-error in core CI workflows"
    fi
    
    # Check VantisOS/ directory doesn't exist in tracked files
    local vantis_tracked
    vantis_tracked=$(git ls-files | grep "^VantisOS/" | wc -l || echo "0")
    if [[ "$vantis_tracked" -gt 0 ]]; then
        log_fail "VantisOS/ subdirectory still tracked ($vantis_tracked files)"
    else
        log_pass "No VantisOS/ duplicate directory in tracking"
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
    check_workspace_members
    check_version_consistency
    check_ci_integrity
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