#!/bin/bash
# 🔍 VantisOS Repository Verification Script
# Verifies repository health, checks for issues, and validates structure

echo "🔍 Starting VantisOS Repository Verification..."
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
ERRORS=0
WARNINGS=0
CHECKS=0

# Function to print colored output
print_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((CHECKS++))
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

print_fail() {
    echo -e "${RED}✗${NC} $1"
    ((ERRORS++))
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Get repository root
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "📁 Repository: $REPO_ROOT"
echo ""

# 1. Check Git status
echo "🔧 Checking Git status..."
if git rev-parse --git-dir > /dev/null 2>&1; then
    print_pass "Git repository detected"
    
    # Check for uncommitted changes
    CHANGES=$(git status --porcelain 2>/dev/null | wc -l)
    if [ "$CHANGES" -eq 0 ]; then
        print_pass "No uncommitted changes"
    else
        print_warning "Uncommitted changes detected ($CHANGES files)"
    fi
    
    # Check current branch
    BRANCH=$(git branch --show-current)
    print_info "Current branch: $BRANCH"
else
    print_fail "Not a Git repository"
fi

# 2. Check directory structure
echo ""
echo "📂 Checking directory structure..."

REQUIRED_DIRS=(
    "src"
    "docs"
    "scripts"
    "history"
    ".github"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        print_pass "Directory exists: $dir"
    else
        print_fail "Missing directory: $dir"
    fi
done

# 3. Check essential files
echo ""
echo "📄 Checking essential files..."

REQUIRED_FILES=(
    "README.md"
    "CHANGELOG.md"
    "CONTRIBUTING.md"
    "LICENSE"
    "todo.md"
    ".gitignore"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        print_pass "File exists: $file"
    else
        print_fail "Missing file: $file"
    fi
done

# 4. Check for build artifacts
echo ""
echo "🦀 Checking for build artifacts..."

if [ -d "src/verified/target" ]; then
    SIZE=$(du -sh src/verified/target 2>/dev/null | cut -f1)
    print_warning "Build artifacts found: src/verified/target/ ($SIZE)"
    print_info "Run './scripts/cleanup.sh' to remove"
else
    print_pass "No build artifacts in src/verified/"
fi

if [ -d "target" ]; then
    SIZE=$(du -sh target 2>/dev/null | cut -f1)
    print_warning "Build artifacts found: target/ ($SIZE)"
    print_info "Run './scripts/cleanup.sh' to remove"
else
    print_pass "No build artifacts in root"
fi

# 5. Check for temporary files
echo ""
echo "🧪 Checking for temporary files..."

TEMP_FILES=$(find . -maxdepth 1 -type f \( -name "benchmark_*.txt" -o -name "*_test_results.txt" -o -name "*.benchmark" \) 2>/dev/null)
if [ -n "$TEMP_FILES" ]; then
    COUNT=$(echo "$TEMP_FILES" | wc -l)
    print_warning "Found $COUNT temporary test files"
    echo "$TEMP_FILES" | head -5
else
    print_pass "No temporary test files"
fi

# 6. Check for large files
echo ""
echo "📦 Checking for large files (>10MB)..."

LARGE_FILES=$(find . -type f -size +10M 2>/dev/null | grep -v ".git")
if [ -n "$LARGE_FILES" ]; then
    print_warning "Found large files:"
    echo "$LARGE_FILES" | while read file; do
        SIZE=$(du -h "$file" | cut -f1)
        echo "   $SIZE - $file"
    done
else
    print_pass "No large files found"
fi

# 7. Check .gitignore rules
echo ""
echo "🚫 Checking .gitignore rules..."

if [ -f ".gitignore" ]; then
    print_pass ".gitignore exists"
    
    # Check for essential patterns
    PATTERNS=(
        "target"
        "*.rs.bk"
        "node_modules"
        "*.env"
    )
    
    for pattern in "${PATTERNS[@]}"; do
        if grep -q "$pattern" .gitignore; then
            print_pass "Pattern in .gitignore: $pattern"
        else
            print_warning "Missing pattern in .gitignore: $pattern"
        fi
    done
else
    print_fail ".gitignore not found"
fi

# 8. Check Rust project
echo ""
echo "🦀 Checking Rust project..."

if [ -f "src/verified/Cargo.toml" ]; then
    print_pass "Cargo.toml found"
    
    # Try to check if project compiles (quick check)
    cd src/verified
    if cargo check --quiet 2>/dev/null; then
        print_pass "Rust project compiles"
    else
        print_warning "Rust project has compilation issues"
        print_info "Run 'cd src/verified && cargo check' for details"
    fi
    cd "$REPO_ROOT"
else
    print_fail "Cargo.toml not found"
fi

# 9. Check documentation structure
echo ""
echo "📚 Checking documentation structure..."

DOC_DIRS=(
    "docs/architecture"
    "docs/implementation"
    "docs/operations"
    "docs/development"
    "docs/api"
    "docs/security"
    "docs/translations"
)

for dir in "${DOC_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        COUNT=$(find "$dir" -name "*.md" | wc -l)
        print_pass "Documentation directory: $dir ($COUNT files)"
    else
        print_warning "Missing documentation directory: $dir"
    fi
done

# 10. Check history structure
echo ""
echo "📜 Checking history structure..."

HISTORY_DIRS=(
    "history/milestones"
    "history/sessions"
    "history/releases"
)

for dir in "${HISTORY_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        COUNT=$(find "$dir" -name "*.md" | wc -l)
        print_pass "History directory: $dir ($COUNT files)"
    else
        print_warning "Missing history directory: $dir"
    fi
done

# 11. Repository statistics
echo ""
echo "📊 Repository Statistics:"
TOTAL_SIZE=$(du -sh . 2>/dev/null | cut -f1)
echo "   Total size: $TOTAL_SIZE"

MD_COUNT=$(find . -name "*.md" -type f | wc -l)
echo "   Markdown files: $MD_COUNT"

RS_COUNT=$(find . -name "*.rs" -type f | wc -l)
echo "   Rust files: $RS_COUNT"

if [ -d ".git" ]; then
    COMMITS=$(git rev-list --count HEAD 2>/dev/null || echo "N/A")
    echo "   Total commits: $COMMITS"
fi

# 12. Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 Verification Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "   ✓ Passed checks: $CHECKS"
echo "   ⚠ Warnings: $WARNINGS"
echo "   ✗ Errors: $ERRORS"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✨ Repository is in excellent condition!${NC}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠ Repository has some warnings but is functional${NC}"
    exit 0
else
    echo -e "${RED}✗ Repository has errors that need attention${NC}"
    exit 1
fi