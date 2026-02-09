#!/bin/bash

# Script to analyze all branches in the repository
# This will check each branch for cleanup needs

echo "🔍 VantisOS Branch Analysis Tool"
echo "================================"
echo ""

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"
echo ""

# Get all remote branches
echo "📋 Fetching branch list..."
BRANCHES=$(git branch -r | grep -v HEAD | sed 's/origin\///' | sort -u)

# Create analysis report
REPORT_FILE="BRANCH_ANALYSIS_REPORT.md"
echo "# VantisOS Branch Analysis Report" > $REPORT_FILE
echo "" >> $REPORT_FILE
echo "**Generated**: $(date)" >> $REPORT_FILE
echo "**Current Branch**: $CURRENT_BRANCH" >> $REPORT_FILE
echo "" >> $REPORT_FILE

# Function to analyze a branch
analyze_branch() {
    local branch=$1
    echo "---" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "## Branch: \`$branch\`" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    
    # Checkout branch (suppress output)
    git checkout $branch 2>/dev/null 1>/dev/null
    
    if [ $? -ne 0 ]; then
        echo "❌ **Status**: Could not checkout" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return
    fi
    
    # Count root markdown files
    ROOT_MD_COUNT=$(find . -maxdepth 1 -name "*.md" | wc -l)
    
    # Check for build artifacts
    BUILD_SIZE=0
    if [ -d "src/verified/target" ]; then
        BUILD_SIZE=$(du -sm src/verified/target 2>/dev/null | cut -f1)
    fi
    
    # Check for docs directory
    HAS_DOCS_DIR="❌"
    if [ -d "docs" ]; then
        HAS_DOCS_DIR="✅"
    fi
    
    # Check for history directory
    HAS_HISTORY_DIR="❌"
    if [ -d "history" ]; then
        HAS_HISTORY_DIR="✅"
    fi
    
    # Check for scripts directory
    HAS_SCRIPTS_DIR="❌"
    if [ -d "scripts" ]; then
        HAS_SCRIPTS_DIR="✅"
    fi
    
    # Count total files
    TOTAL_FILES=$(find . -type f | wc -l)
    
    # Last commit info
    LAST_COMMIT=$(git log -1 --format="%h - %s (%ar)" 2>/dev/null)
    
    # Determine cleanup priority
    PRIORITY="🟢 LOW"
    NEEDS_CLEANUP="No"
    
    if [ $ROOT_MD_COUNT -gt 15 ] || [ $BUILD_SIZE -gt 100 ]; then
        PRIORITY="🔴 HIGH"
        NEEDS_CLEANUP="Yes"
    elif [ $ROOT_MD_COUNT -gt 10 ] || [ $BUILD_SIZE -gt 50 ]; then
        PRIORITY="🟡 MEDIUM"
        NEEDS_CLEANUP="Maybe"
    fi
    
    # Write analysis
    echo "**Last Commit**: $LAST_COMMIT" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "### Metrics" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "| Metric | Value |" >> $REPORT_FILE
    echo "|--------|-------|" >> $REPORT_FILE
    echo "| Root .md files | $ROOT_MD_COUNT |" >> $REPORT_FILE
    echo "| Build artifacts | ${BUILD_SIZE} MB |" >> $REPORT_FILE
    echo "| Total files | $TOTAL_FILES |" >> $REPORT_FILE
    echo "| Has docs/ | $HAS_DOCS_DIR |" >> $REPORT_FILE
    echo "| Has history/ | $HAS_HISTORY_DIR |" >> $REPORT_FILE
    echo "| Has scripts/ | $HAS_SCRIPTS_DIR |" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "### Assessment" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "- **Priority**: $PRIORITY" >> $REPORT_FILE
    echo "- **Needs Cleanup**: $NEEDS_CLEANUP" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
}

# Analyze each branch
echo "🔄 Analyzing branches..."
echo ""

BRANCH_COUNT=0
for branch in $BRANCHES; do
    BRANCH_COUNT=$((BRANCH_COUNT + 1))
    echo "[$BRANCH_COUNT] Analyzing: $branch"
    analyze_branch $branch
done

# Return to original branch
echo ""
echo "🔙 Returning to original branch: $CURRENT_BRANCH"
git checkout $CURRENT_BRANCH 2>/dev/null 1>/dev/null

# Summary
echo "" >> $REPORT_FILE
echo "---" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "## Summary" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "**Total Branches Analyzed**: $BRANCH_COUNT" >> $REPORT_FILE
echo "" >> $REPORT_FILE

# Count priorities
HIGH_PRIORITY=$(grep "🔴 HIGH" $REPORT_FILE | wc -l)
MEDIUM_PRIORITY=$(grep "🟡 MEDIUM" $REPORT_FILE | wc -l)
LOW_PRIORITY=$(grep "🟢 LOW" $REPORT_FILE | wc -l)

echo "### Priority Distribution" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "- 🔴 **High Priority**: $HIGH_PRIORITY branches" >> $REPORT_FILE
echo "- 🟡 **Medium Priority**: $MEDIUM_PRIORITY branches" >> $REPORT_FILE
echo "- 🟢 **Low Priority**: $LOW_PRIORITY branches" >> $REPORT_FILE
echo "" >> $REPORT_FILE

echo ""
echo "✅ Analysis complete!"
echo "📄 Report saved to: $REPORT_FILE"
echo ""
echo "Summary:"
echo "  - Total branches: $BRANCH_COUNT"
echo "  - High priority: $HIGH_PRIORITY"
echo "  - Medium priority: $MEDIUM_PRIORITY"
echo "  - Low priority: $LOW_PRIORITY"