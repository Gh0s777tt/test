#!/bin/bash
#
# VantisOS Security Audit Script
# Comprehensive security analysis and vulnerability scanning
#
# Usage: ./scripts/security/audit.sh [--full]

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"

# Configuration
FULL_AUDIT="${1:-false}"

print_banner "VantisOS Security Audit" "Comprehensive Security Analysis"

# Counters
CRITICAL_ISSUES=0
HIGH_ISSUES=0
MEDIUM_ISSUES=0
LOW_ISSUES=0
WARNINGS=0

# Output directory
REPORT_DIR="$VANTIS_ROOT/reports/security"
mkdir -p "$REPORT_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$REPORT_DIR/audit_$TIMESTAMP.md"

# ==============================================================================
# Helper Functions
# ==============================================================================

report_issue() {
    local severity="$1"
    local title="$2"
    local description="${3:-}"
    local file="${4:-}"
    
    case "$severity" in
        critical) ((CRITICAL_ISSUES++)) ;;
        high) ((HIGH_ISSUES++)) ;;
        medium) ((MEDIUM_ISSUES++)) ;;
        low) ((LOW_ISSUES++)) ;;
        warn) ((WARNINGS++)) ;;
    esac
    
    # Log to console
    local color
    case "$severity" in
        critical) color="$RED" ;;
        high) color="$MAGENTA" ;;
        medium) color="$YELLOW" ;;
        low) color="$BLUE" ;;
        warn) color="$CYAN" ;;
    esac
    
    log_${severity} "[$severity] $title"
    [[ -n "$description" ]] && echo "  $description"
    [[ -n "$file" ]] && echo "  File: $file"
    
    # Write to report
    cat >> "$REPORT_FILE" << EOF
### $severity: $title
${description:+**Description:** $description}
${file:+**File:** \`$file\`}

EOF
}

# ==============================================================================
# Initialize Report
# ==============================================================================

cat > "$REPORT_FILE" << EOF
# VantisOS Security Audit Report

**Date:** $(date)
**Auditor:** Automated Security Audit
**Version:** $VANTIS_VERSION

---

## Summary

| Severity | Count |
|----------|-------|
| Critical | CRITICAL_PLACEHOLDER |
| High | HIGH_PLACEHOLDER |
| Medium | MEDIUM_PLACEHOLDER |
| Low | LOW_PLACEHOLDER |
| Warnings | WARNINGS_PLACEHOLDER |

---

## Findings

EOF

# ==============================================================================
# Dependency Security Audit
# ==============================================================================

print_section "Dependency Security Audit"

if command_exists cargo-audit; then
    log_info "Running cargo audit..."
    
    if cargo audit 2>/dev/null; then
        log_success "No known vulnerabilities in dependencies"
    else
        # Parse audit output
        AUDIT_OUTPUT=$(cargo audit 2>&1 || true)
        
        # Count vulnerability IDs
        VULN_COUNT=$(echo "$AUDIT_OUTPUT" | grep -c "ID:" || true)
        
        if [[ $VULN_COUNT -gt 0 ]]; then
            report_issue "high" "Vulnerable dependencies found" "$VULN_COUNT vulnerabilities detected in Cargo dependencies"
            echo "$AUDIT_OUTPUT" | grep -A 5 "ID:" | head -30
        fi
    fi
else
    log_warning "cargo-audit not installed"
    echo "  Install with: cargo install cargo-audit"
fi

# Check for unmaintained dependencies
if command_exists cargo-crev; then
    log_info "Checking dependency trust..."
    cargo crev verify 2>/dev/null || log_warning "Dependency verification had issues"
fi

# ==============================================================================
# Code Security Analysis
# ==============================================================================

print_section "Code Security Analysis"

# Check for unsafe code blocks
log_info "Searching for unsafe code blocks..."
UNSAFE_COUNT=$(grep -r "unsafe" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null | wc -l || echo "0")

if [[ $UNSAFE_COUNT -gt 0 ]]; then
    log_info "Found $UNSAFE_COUNT potential unsafe code blocks"
    
    if [[ "$FULL_AUDIT" == "true" ]]; then
        log_info "Unsafe code locations:"
        grep -rn "unsafe" "$VANTIS_ROOT/src" --include="*.rs" | head -20
    fi
    
    if [[ $UNSAFE_COUNT -gt 100 ]]; then
        report_issue "medium" "High usage of unsafe code" "Found $UNSAFE_COUNT unsafe blocks - review for safety"
    else
        report_issue "low" "Unsafe code usage" "Found $UNSAFE_COUNT unsafe blocks"
    fi
else
    log_success "No unsafe code blocks found"
fi

# Check for panicking functions
log_info "Checking for panic-prone code..."
PANIC_COUNT=$(grep -rn "\.unwrap()\|\.expect(\|panic!\|todo!\|unimplemented!" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null | wc -l || echo "0")

if [[ $PANIC_COUNT -gt 50 ]]; then
    report_issue "medium" "High panic potential" "Found $PANIC_COUNT potential panic points (unwrap, expect, panic!)"
elif [[ $PANIC_COUNT -gt 0 ]]; then
    report_issue "low" "Panic-prone code" "Found $PANIC_COUNT potential panic points"
fi

# ==============================================================================
# Hardcoded Secrets Detection
# ==============================================================================

print_section "Secrets Detection"

log_info "Searching for hardcoded secrets..."
SECRET_PATTERNS=(
    "password\s*=\s*&quot;[^&quot;]+&quot;"
    "api_key\s*=\s*&quot;[^&quot;]+&quot;"
    "secret\s*=\s*&quot;[^&quot;]+&quot;"
    "token\s*=\s*&quot;[^&quot;]+&quot;"
    "private_key\s*=\s*&quot;[^&quot;]+&quot;"
    "-----BEGIN.*PRIVATE KEY-----"
    "aws_access_key_id\s*=\s*&quot;[^&quot;]+&quot;"
    "aws_secret_access_key\s*=\s*&quot;[^&quot;]+&quot;"
)

SECRETS_FOUND=0
for pattern in "${SECRET_PATTERNS[@]}"; do
    MATCHES=$(grep -rnE "$pattern" "$VANTIS_ROOT/src" --include="*.rs" --include="*.toml" --include="*.yaml" --include="*.yml" 2>/dev/null || true)
    
    if [[ -n "$MATCHES" ]]; then
        SECRETS_FOUND=$((SECRETS_FOUND + $(echo "$MATCHES" | wc -l)))
        echo "$MATCHES" | head -5
    fi
done

if [[ $SECRETS_FOUND -gt 0 ]]; then
    report_issue "critical" "Potential hardcoded secrets found" "$SECRETS_FOUND matches found - review immediately"
else
    log_success "No hardcoded secrets detected"
fi

# ==============================================================================
# Permission & Access Control
# ==============================================================================

print_section "Permission Analysis"

# Check for world-writable files
log_info "Checking for world-writable files..."
WORLD_WRITABLE=$(find "$VANTIS_ROOT" -type f -perm /o+w ! -path "*/.git/*" 2>/dev/null | head -20)

if [[ -n "$WORLD_WRITABLE" ]]; then
    WW_COUNT=$(echo "$WORLD_WRITABLE" | wc -l)
    report_issue "high" "World-writable files found" "$WW_COUNT files are world-writable"
    echo "$WORLD_WRITABLE"
else
    log_success "No world-writable files found"
fi

# Check for sensitive files with wrong permissions
log_info "Checking sensitive file permissions..."
SENSITIVE_FILES=(
    "$HOME/.ssh/id_rsa"
    "$HOME/.ssh/id_ed25519"
    "$HOME/.gnupg"
)

for file in "${SENSITIVE_FILES[@]}"; do
    if [[ -e "$file" ]]; then
        perms=$(stat -c "%a" "$file" 2>/dev/null)
        if [[ "$perms" != "600" ]] && [[ "$perms" != "700" ]]; then
            report_issue "high" "Sensitive file has wrong permissions" "File: $file, Permissions: $perms"
        fi
    fi
done

# ==============================================================================
# Network Security
# ==============================================================================

print_section "Network Security Analysis"

# Check for insecure HTTP URLs in code
log_info "Checking for insecure URLs..."
HTTP_URLS=$(grep -rn "http://" "$VANTIS_ROOT/src" --include="*.rs" --include="*.toml" 2>/dev/null | grep -v "localhost" | grep -v "127.0.0.1" | grep -v "example.com" || true)

if [[ -n "$HTTP_URLS" ]]; then
    HTTP_COUNT=$(echo "$HTTP_URLS" | wc -l)
    report_issue "medium" "Insecure HTTP URLs in code" "$HTTP_COUNT non-localhost HTTP URLs found"
    echo "$HTTP_URLS" | head -10
else
    log_success "No insecure HTTP URLs found"
fi

# Check for network binding
log_info "Checking network bindings..."
BIND_ANY=$(grep -rn "0.0.0.0\|::" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null | grep -i "bind\|listen" || true)

if [[ -n "$BIND_ANY" ]]; then
    report_issue "medium" "Binding to all interfaces" "Code may bind to all network interfaces"
    echo "$BIND_ANY" | head -5
fi

# ==============================================================================
# Input Validation
# ==============================================================================

print_section "Input Validation Analysis"

# Check for SQL injection potential
log_info "Checking for SQL injection potential..."
SQL_PATTERNS=(
    "format!.*SELECT"
    "format!.*INSERT"
    "format!.*UPDATE"
    "format!.*DELETE"
)

for pattern in "${SQL_PATTERNS[@]}"; do
    MATCHES=$(grep -rnE "$pattern" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null || true)
    if [[ -n "$MATCHES" ]]; then
        report_issue "high" "Potential SQL injection" "Dynamic SQL query construction found"
        echo "$MATCHES"
    fi
done

# Check for command injection potential
log_info "Checking for command injection..."
CMD_INJ=$(grep -rn "std::process::Command\|Command::new" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null | head -20 || true)

if [[ -n "$CMD_INJ" ]]; then
    CMD_COUNT=$(echo "$CMD_INJ" | wc -l)
    report_issue "low" "External command execution" "$CMD_COUNT locations execute external commands - verify input sanitization"
fi

# ==============================================================================
# Cryptographic Analysis
# ==============================================================================

print_section "Cryptographic Analysis"

# Check for weak crypto algorithms
log_info "Checking for weak cryptography..."
WEAK_CRYPTO=(
    "md5"
    "sha1"
    "des"
    "rc4"
    "ecb"
)

for algo in "${WEAK_CRYPTO[@]}"; do
    MATCHES=$(grep -rni "$algo" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null | grep -v "// " | grep -v "test" | head -5 || true)
    if [[ -n "$MATCHES" ]]; then
        report_issue "medium" "Weak cryptographic algorithm" "Potential use of $algo detected"
        echo "$MATCHES"
    fi
done

# Check for proper random number generation
log_info "Checking random number generation..."
WEAK_RNG=$(grep -rn "rand::thread_rng\|Math.random\|rand()\|srand" "$VANTIS_ROOT/src" --include="*.rs" 2>/dev/null || true)

if [[ -n "$WEAK_RNG" ]]; then
    report_issue "low" "Random number generation" "Review RNG usage for security context"
fi

# ==============================================================================
# Git Security
# ==============================================================================

print_section "Git Security"

# Check for secrets in git history
if [[ "$FULL_AUDIT" == "true" ]]; then
    log_info "Scanning git history for secrets..."
    if command_exists gitleaks; then
        if gitleaks detect --source "$VANTIS_ROOT" --no-git 2>/dev/null; then
            log_success "No secrets leaked in git history"
        else
            report_issue "high" "Secrets detected in git history" "Run gitleaks for details"
        fi
    else
        log_warning "gitleaks not installed - skipping git history scan"
    fi
fi

# Check .gitignore
if [[ -f "$VANTIS_ROOT/.gitignore" ]]; then
    log_info "Checking .gitignore for sensitive patterns..."
    SENSITIVE_IGNORED=(
        "*.pem"
        "*.key"
        "*.env"
        ".env*"
        "secrets"
    )
    
    MISSING_IGNORE=()
    for pattern in "${SENSITIVE_IGNORED[@]}"; do
        if ! grep -q "$pattern" "$VANTIS_ROOT/.gitignore" 2>/dev/null; then
            MISSING_IGNORE+=("$pattern")
        fi
    done
    
    if [[ ${#MISSING_IGNORE[@]} -gt 0 ]]; then
        report_issue "medium" "Missing .gitignore patterns" "Consider adding: ${MISSING_IGNORE[*]}"
    fi
fi

# ==============================================================================
# Container Security (if Docker present)
# ==============================================================================

if [[ -f "$VANTIS_ROOT/Dockerfile" ]] || [[ -d "$VANTIS_ROOT/docker" ]]; then
    print_section "Container Security"
    
    if command_exists docker; then
        # Check for latest image usage
        LATEST_USAGE=$(grep -n "FROM.*:latest" "$VANTIS_ROOT"/Dockerfile "$VANTIS_ROOT"/docker/* 2>/dev/null || true)
        
        if [[ -n "$LATEST_USAGE" ]]; then
            report_issue "medium" "Using :latest tag" "Pin specific versions for reproducibility"
            echo "$LATEST_USAGE"
        fi
        
        # Check for USER directive
        if ! grep -q "USER" "$VANTIS_ROOT"/Dockerfile 2>/dev/null; then
            report_issue "low" "No USER directive in Dockerfile" "Consider running as non-root user"
        fi
    fi
fi

# ==============================================================================
# Summary Report
# ==============================================================================

print_banner "Security Audit Summary"

echo ""
echo -e "${RED}Critical Issues: $CRITICAL_ISSUES${NC}"
echo -e "${MAGENTA}High Issues: $HIGH_ISSUES${NC}"
echo -e "${YELLOW}Medium Issues: $MEDIUM_ISSUES${NC}"
echo -e "${BLUE}Low Issues: $LOW_ISSUES${NC}"
echo -e "${CYAN}Warnings: $WARNINGS${NC}"
echo ""

# Update report with counts
sed -i "s/CRITICAL_PLACEHOLDER/$CRITICAL_ISSUES/g" "$REPORT_FILE"
sed -i "s/HIGH_PLACEHOLDER/$HIGH_ISSUES/g" "$REPORT_FILE"
sed -i "s/MEDIUM_PLACEHOLDER/$MEDIUM_ISSUES/g" "$REPORT_FILE"
sed -i "s/LOW_PLACEHOLDER/$LOW_ISSUES/g" "$REPORT_FILE"
sed -i "s/WARNINGS_PLACEHOLDER/$WARNINGS/g" "$REPORT_FILE"

# Add recommendations
cat >> "$REPORT_FILE" << EOF

---

## Recommendations

1. **Immediate Actions Required:**
   - Review and address all critical and high-severity issues
   - Remove or rotate any exposed secrets
   - Update vulnerable dependencies

2. **Short-term Improvements:**
   - Add missing .gitignore patterns
   - Review and reduce unsafe code usage
   - Implement input validation

3. **Long-term Security:**
   - Implement automated security scanning in CI/CD
   - Regular dependency audits
   - Security code review process

---

**Report saved to:** \`$REPORT_FILE\`
EOF

log_info "Report saved to: $REPORT_FILE"

# Exit code based on findings
if [[ $CRITICAL_ISSUES -gt 0 ]]; then
    log_error "CRITICAL SECURITY ISSUES FOUND - IMMEDIATE ACTION REQUIRED"
    exit 2
elif [[ $HIGH_ISSUES -gt 0 ]]; then
    log_error "High severity security issues found"
    exit 1
elif [[ $MEDIUM_ISSUES -gt 0 ]]; then
    log_warning "Medium severity security issues found"
    exit 0
else
    log_success "No critical security issues found! ✨"
    exit 0
fi