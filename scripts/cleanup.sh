#!/bin/bash
# 🧹 VantisOS Repository Cleanup Script
# Removes build artifacts, temporary files, and maintains repository hygiene

set -e

echo "🧹 Starting VantisOS Repository Cleanup..."
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Get repository root
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "📁 Repository: $REPO_ROOT"
echo ""

# 1. Clean Rust build artifacts
echo "🦀 Cleaning Rust build artifacts..."
if [ -d "src/verified/target" ]; then
    SIZE_BEFORE=$(du -sh src/verified/target 2>/dev/null | cut -f1)
    rm -rf src/verified/target
    print_status "Removed src/verified/target/ ($SIZE_BEFORE)"
else
    print_warning "No target directory found"
fi

if [ -d "target" ]; then
    SIZE_BEFORE=$(du -sh target 2>/dev/null | cut -f1)
    rm -rf target
    print_status "Removed target/ ($SIZE_BEFORE)"
fi

# 2. Clean temporary test files
echo ""
echo "🧪 Cleaning temporary test files..."
TEMP_FILES=$(find . -maxdepth 1 -type f \( -name "benchmark_*.txt" -o -name "*_test_results.txt" -o -name "*.benchmark" \) 2>/dev/null)
if [ -n "$TEMP_FILES" ]; then
    echo "$TEMP_FILES" | while read file; do
        rm -f "$file"
        print_status "Removed $(basename $file)"
    done
else
    print_warning "No temporary test files found"
fi

# 3. Clean Rust backup files
echo ""
echo "📝 Cleaning Rust backup files..."
BACKUP_FILES=$(find . -type f -name "*.rs.bk" 2>/dev/null)
if [ -n "$BACKUP_FILES" ]; then
    echo "$BACKUP_FILES" | while read file; do
        rm -f "$file"
        print_status "Removed $file"
    done
else
    print_warning "No backup files found"
fi

# 4. Clean incremental compilation cache
echo ""
echo "🔄 Cleaning incremental compilation cache..."
INCREMENTAL_DIRS=$(find . -type d -name "incremental" 2>/dev/null)
if [ -n "$INCREMENTAL_DIRS" ]; then
    echo "$INCREMENTAL_DIRS" | while read dir; do
        rm -rf "$dir"
        print_status "Removed $dir"
    done
else
    print_warning "No incremental cache found"
fi

# 5. Clean long type name files
echo ""
echo "📄 Cleaning long type name files..."
LONG_TYPE_FILES=$(find . -type f -name "*.long-type-*.txt" 2>/dev/null)
if [ -n "$LONG_TYPE_FILES" ]; then
    COUNT=$(echo "$LONG_TYPE_FILES" | wc -l)
    echo "$LONG_TYPE_FILES" | xargs rm -f
    print_status "Removed $COUNT long type name files"
else
    print_warning "No long type name files found"
fi

# 6. Clean empty directories
echo ""
echo "📂 Cleaning empty directories..."
EMPTY_DIRS=$(find . -type d -empty 2>/dev/null | grep -v ".git")
if [ -n "$EMPTY_DIRS" ]; then
    echo "$EMPTY_DIRS" | while read dir; do
        rmdir "$dir" 2>/dev/null && print_status "Removed empty directory: $dir" || true
    done
else
    print_warning "No empty directories found"
fi

# 7. Show repository size
echo ""
echo "📊 Repository Statistics:"
TOTAL_SIZE=$(du -sh . 2>/dev/null | cut -f1)
echo "   Total size: $TOTAL_SIZE"

if [ -d ".git" ]; then
    GIT_SIZE=$(du -sh .git 2>/dev/null | cut -f1)
    echo "   Git size: $GIT_SIZE"
fi

SRC_SIZE=$(du -sh src 2>/dev/null | cut -f1)
echo "   Source size: $SRC_SIZE"

# 8. Check for large files
echo ""
echo "🔍 Checking for large files (>1MB)..."
LARGE_FILES=$(find . -type f -size +1M 2>/dev/null | grep -v ".git" | head -10)
if [ -n "$LARGE_FILES" ]; then
    print_warning "Found large files:"
    echo "$LARGE_FILES" | while read file; do
        SIZE=$(du -h "$file" | cut -f1)
        echo "   $SIZE - $file"
    done
else
    print_status "No large files found"
fi

# 9. Summary
echo ""
echo "✨ Cleanup Complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Run 'git status' to see changes"
echo "   2. Run 'cargo build' to rebuild if needed"
echo "   3. Run 'cargo test' to verify everything works"
echo ""