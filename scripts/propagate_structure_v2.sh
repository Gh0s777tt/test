#!/bin/bash

# Script to propagate the organized structure to active feature branches
# This version uses a cleaner approach: copy files instead of merge

echo "🔄 VantisOS Structure Propagation Tool v2"
echo "=========================================="
echo ""

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"
echo ""

# Define target branches
TARGET_BRANCHES=(
    "feature/developer-guide-v2"
    "feature/developer-onboarding-guide"
    "feature/formal-verification-pipeline"
    "feature/formal-verification-v2"
    "master"
)

# Create propagation report
REPORT_FILE="STRUCTURE_PROPAGATION_REPORT_V2.md"
echo "# VantisOS Structure Propagation Report v2" > $REPORT_FILE
echo "" >> $REPORT_FILE
echo "**Generated**: $(date)" >> $REPORT_FILE
echo "**Source Branch**: $CURRENT_BRANCH" >> $REPORT_FILE
echo "**Method**: File copying (cleaner approach)" >> $REPORT_FILE
echo "" >> $REPORT_FILE

# Save current state
echo "💾 Saving current state..."
TEMP_DIR="/tmp/vantis_structure_$$"
mkdir -p $TEMP_DIR

# Copy the organized structure
cp -r docs $TEMP_DIR/ 2>/dev/null
cp -r history $TEMP_DIR/ 2>/dev/null
cp -r scripts $TEMP_DIR/ 2>/dev/null

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
    
    # Check what exists
    HAS_DOCS=false
    HAS_HISTORY=false
    HAS_SCRIPTS=false
    
    if [ -d "docs" ]; then
        HAS_DOCS=true
    fi
    
    if [ -d "history" ]; then
        HAS_HISTORY=true
    fi
    
    if [ -d "scripts" ]; then
        HAS_SCRIPTS=true
    fi
    
    # Track changes
    CHANGES_MADE=false
    CHANGES_LIST=""
    
    # Copy docs if needed
    if [ "$HAS_DOCS" = false ]; then
        echo "  Copying docs/ directory..."
        cp -r $TEMP_DIR/docs . 2>/dev/null
        if [ $? -eq 0 ]; then
            git add docs/
            CHANGES_MADE=true
            CHANGES_LIST="${CHANGES_LIST}- ✅ Added docs/ directory with organized documentation\n"
        fi
    fi
    
    # Copy history if needed
    if [ "$HAS_HISTORY" = false ]; then
        echo "  Copying history/ directory..."
        cp -r $TEMP_DIR/history . 2>/dev/null
        if [ $? -eq 0 ]; then
            git add history/
            CHANGES_MADE=true
            CHANGES_LIST="${CHANGES_LIST}- ✅ Added history/ directory for historical records\n"
        fi
    fi
    
    # Copy scripts if needed (merge with existing)
    if [ "$HAS_SCRIPTS" = false ]; then
        echo "  Copying scripts/ directory..."
        cp -r $TEMP_DIR/scripts . 2>/dev/null
        if [ $? -eq 0 ]; then
            git add scripts/
            CHANGES_MADE=true
            CHANGES_LIST="${CHANGES_LIST}- ✅ Added scripts/ directory for maintenance tools\n"
        fi
    else
        # Merge scripts
        echo "  Merging scripts/ directory..."
        cp $TEMP_DIR/scripts/*.sh scripts/ 2>/dev/null
        if [ $? -eq 0 ]; then
            git add scripts/
            CHANGES_MADE=true
            CHANGES_LIST="${CHANGES_LIST}- ✅ Updated scripts/ directory with new tools\n"
        fi
    fi
    
    # If no changes, skip
    if [ "$CHANGES_MADE" = false ]; then
        echo "✅ **Status**: Already has organized structure" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return
    fi
    
    # Commit changes
    git commit -m "chore: Propagate organized structure from 0.4.1

- Add docs/ directory with organized documentation
- Add history/ directory for historical records
- Add scripts/ directory for maintenance tools
- Maintain clean root directory structure

This brings the branch in line with the new organizational
structure implemented in 0.4.1 for better maintainability." 2>/dev/null 1>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ **Status**: Structure propagated successfully" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        echo "**Changes Applied**:" >> $REPORT_FILE
        echo -e "$CHANGES_LIST" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return 0
    else
        echo "❌ **Status**: Commit failed" >> $REPORT_FILE
        echo "" >> $REPORT_FILE
        return 1
    fi
}

# Propagate to each target branch
echo "🔄 Propagating structure to target branches..."
echo ""

BRANCH_COUNT=0
SUCCESS_COUNT=0
SKIP_COUNT=0
FAIL_COUNT=0

for branch in "${TARGET_BRANCHES[@]}"; do
    BRANCH_COUNT=$((BRANCH_COUNT + 1))
    echo "[$BRANCH_COUNT/${#TARGET_BRANCHES[@]}] Processing: $branch"
    
    propagate_to_branch $branch
    RESULT=$?
    
    if [ $RESULT -eq 0 ]; then
        if grep -q "✅ \*\*Status\*\*: Structure propagated successfully" $REPORT_FILE | tail -1 > /dev/null 2>&1; then
            SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
            SKIP_COUNT=$((SKIP_COUNT + 1))
        fi
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

# Return to original branch
echo ""
echo "🔙 Returning to original branch: $CURRENT_BRANCH"
git checkout $CURRENT_BRANCH 2>/dev/null 1>/dev/null

# Cleanup temp directory
rm -rf $TEMP_DIR

# Summary
echo "" >> $REPORT_FILE
echo "---" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "## Summary" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "**Total Branches Processed**: $BRANCH_COUNT" >> $REPORT_FILE
echo "**Successfully Updated**: $SUCCESS_COUNT" >> $REPORT_FILE
echo "**Already Organized**: $SKIP_COUNT" >> $REPORT_FILE
echo "**Failed**: $FAIL_COUNT" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "### Next Steps" >> $REPORT_FILE
echo "" >> $REPORT_FILE
echo "To push all changes to remote:" >> $REPORT_FILE
echo "\`\`\`bash" >> $REPORT_FILE
for branch in "${TARGET_BRANCHES[@]}"; do
    echo "git push origin $branch" >> $REPORT_FILE
done
echo "\`\`\`" >> $REPORT_FILE
echo "" >> $REPORT_FILE

echo ""
echo "✅ Propagation complete!"
echo "📄 Report saved to: $REPORT_FILE"
echo ""
echo "Summary:"
echo "  - Total branches: $BRANCH_COUNT"
echo "  - Successfully updated: $SUCCESS_COUNT"
echo "  - Already organized: $SKIP_COUNT"
echo "  - Failed: $FAIL_COUNT"
echo ""
if [ $SUCCESS_COUNT -gt 0 ]; then
    echo "⚠️  Note: Changes are committed locally. Use the commands in the report to push to remote."
fi