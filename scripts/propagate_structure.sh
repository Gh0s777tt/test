#!/bin/bash

# Script to propagate the organized structure to active feature branches
# This will apply the same docs/, history/, scripts/ structure to key branches

echo "🔄 VantisOS Structure Propagation Tool"
echo "======================================"
echo ""

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"
echo ""

# Define target branches (active feature branches)
TARGET_BRANCHES=(
    "feature/developer-guide-v2"
    "feature/developer-onboarding-guide"
    "feature/formal-verification-pipeline"
    "feature/formal-verification-v2"
    "master"
)

# Create propagation report
REPORT_FILE="STRUCTURE_PROPAGATION_REPORT.md"
echo "# VantisOS Structure Propagation Report" > $REPORT_FILE
echo "" >> $REPORT_FILE
echo "**Generated**: $(date)" >> $REPORT_FILE
echo "**Source Branch**: $CURRENT_BRANCH" >> $REPORT_FILE
echo "" >> $REPORT_FILE

# Function to propagate structure to a branch
propagate_to_branch() {
    local branch=$1
    echo "---" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    echo "## Branch: \`$branch\`" >> $REPORT_FILE
    echo "" >> $REPORT_FILE
    
    # Checkout branch
    echo "  Checking out $branch..."
    git checkout $branch 2>/dev/null 1>/dev/null
    
    if [ $? -ne 0 ]; then
        echo "❌ **Status**: Could not checkout" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return
    fi
    
    # Check what needs to be done
    NEEDS_DOCS=false
    NEEDS_HISTORY=false
    NEEDS_SCRIPTS=false
    
    if [ ! -d "docs" ]; then
        NEEDS_DOCS=true
    fi
    
    if [ ! -d "history" ]; then
        NEEDS_HISTORY=true
    fi
    
    if [ ! -d "scripts" ]; then
        NEEDS_SCRIPTS=true
    fi
    
    # If everything exists, skip
    if [ "$NEEDS_DOCS" = false ] && [ "$NEEDS_HISTORY" = false ] && [ "$NEEDS_SCRIPTS" = false ]; then
        echo "✅ **Status**: Already has organized structure" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return
    fi
    
    # Merge from 0.4.1 to get the structure
    echo "  Merging structure from 0.4.1..."
    git merge 0.4.1 --no-commit --no-ff 2>/dev/null 1>/dev/null
    
    if [ $? -ne 0 ]; then
        # If merge fails, try cherry-picking the cleanup commits
        git merge --abort 2>/dev/null
        echo "⚠️ **Status**: Merge conflict - manual intervention needed" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return
    fi
    
    # Commit the merge
    git commit -m "chore: Propagate organized structure from 0.4.1

- Add docs/ directory with organized documentation
- Add history/ directory for historical records
- Add scripts/ directory for maintenance tools
- Maintain clean root directory structure" 2>/dev/null 1>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ **Status**: Structure propagated successfully" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        echo "**Changes Applied**:" >> $REPORT_FILE
        if [ "$NEEDS_DOCS" = true ]; then
            echo "- ✅ Added docs/ directory" >> $REPORT_FILE
        fi
        if [ "$NEEDS_HISTORY" = true ]; then
            echo "- ✅ Added history/ directory" >> $REPORT_FILE
        fi
        if [ "$NEEDS_SCRIPTS" = true ]; then
            echo "- ✅ Added scripts/ directory" >> $REPORT_FILE
        fi
        echo "" >> $REPORT_FILE
    else
        echo "❌ **Status**: Commit failed" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
    fi
}

# Propagate to each target branch
echo "🔄 Propagating structure to target branches..."
echo ""

BRANCH_COUNT=0
SUCCESS_COUNT=0
for branch in "${TARGET_BRANCHES[@]}"; do
    BRANCH_COUNT=$((BRANCH_COUNT + 1))
    echo "[$BRANCH_COUNT/${#TARGET_BRANCHES[@]}] Processing: $branch"
    propagate_to_branch $branch
    
    # Check if successful
    if grep -q "✅ \*\*Status\*\*: Structure propagated successfully" $REPORT_FILE | tail -1; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    fi
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
echo "**Total Branches Processed**: $BRANCH_COUNT" >> $REPORT_FILE
echo "**Successfully Updated**: $SUCCESS_COUNT" >> $REPORT_FILE
echo "**Already Organized**: $((BRANCH_COUNT - SUCCESS_COUNT))" >> $REPORT_FILE
echo "" >> $REPORT_FILE

echo ""
echo "✅ Propagation complete!"
echo "📄 Report saved to: $REPORT_FILE"
echo ""
echo "Summary:"
echo "  - Total branches: $BRANCH_COUNT"
echo "  - Successfully updated: $SUCCESS_COUNT"
echo "  - Already organized: $((BRANCH_COUNT - SUCCESS_COUNT))"
echo ""
echo "⚠️  Note: Changes are committed locally. Run 'git push' on each branch to push to remote."