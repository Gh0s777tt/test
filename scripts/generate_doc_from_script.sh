#!/bin/bash
# Script: generate_doc_from_script.sh
# Purpose: Generate documentation from script headers automatically
# Usage: ./scripts/generate_doc_from_script.sh [script_file]
# Requirements: bash, grep, sed
# Author: VantisOS Team
# Date: 2025-03-05
# Version: 1.0.0
# License: SPDX-License-Identifier: MIT

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common library
source "${SCRIPT_DIR}/lib/common.sh" || {
    echo "Error: Failed to load common library" >&2
    exit 1
}

# Enable strict error handling
set -euo pipefail

# Configuration
OUTPUT_DIR="${OUTPUT_DIR:-docs/generated}"
SCRIPTS_DIR="${SCRIPTS_DIR:-scripts}"

# Parse script header and generate documentation
generate_doc_for_script() {
    local script_file="$1"
    local script_name
    script_name=$(basename "$script_file" .sh)
    
    print_header "Processing: $script_name"
    
    # Extract header information
    local purpose usage requirements author date version license
    
    purpose=$(grep '^# Purpose:' "$script_file" | cut -d' ' -f3-)
    usage=$(grep '^# Usage:' "$script_file" | cut -d' ' -f3-)
    requirements=$(grep '^# Requirements:' "$script_file" | cut -d' ' -f3-)
    author=$(grep '^# Author:' "$script_file" | cut -d' ' -f3-)
    date=$(grep '^# Date:' "$script_file" | cut -d' ' -f3-)
    version=$(grep '^# Version:' "$script_file" | cut -d' ' -f3-)
    license=$(grep '^# License:' "$script_file" | cut -d' ' -f3-)
    
    # Extract examples from comments
    local examples
    examples=$(grep -A 10 'Examples:' "$script_file" | grep '^#' | sed 's/^# //' | sed '/^$/d' || echo "No examples found")
    
    # Extract options
    local options
    options=$(grep -A 20 'Options:' "$script_file" | grep '^# \-' | sed 's/^# //' | sed '/^$/d' || echo "No options")
    
    # Generate markdown
    local output_file="${OUTPUT_DIR}/${script_name}.md"
    
    cat > "$output_file" << EOF
# ${script_name}.sh

## Purpose

${purpose}

## Usage

\`\`\`bash
${usage}
\`\`\`

## Requirements

${requirements}

## Options

${options}

## Examples

\`\`\`bash
${examples}
\`\`\`

## Metadata

- **Author:** ${author:-Unknown}
- **Date:** ${date:-Unknown}
- **Version:** ${version:-Unknown}
- **License:** ${license:-Unknown}
- **Location:** \`scripts/${script_name}.sh\`

## Implementation Details

This script follows the VantisOS scripting standards. For more information, see [SCRIPTING_STANDARDS.md](../SCRIPTING_STANDARDS.md).

EOF

    print_success "Generated: $output_file"
}

# Main function
main() {
    local script_file="$1"
    
    print_header "Automated Documentation Generator"
    
    # Ensure output directory exists
    ensure_dir "$OUTPUT_DIR"
    
    if [ -z "$script_file" ]; then
        log_info "No script specified, processing all scripts in $SCRIPTS_DIR"
        
        # Process all scripts
        find "$SCRIPTS_DIR" -type f -name "*.sh" -not -path "*/lib/*" | while read -r script; do
            generate_doc_for_script "$script"
        done
    else
        # Process single script
        if [ ! -f "$script_file" ]; then
            log_error "Script not found: $script_file"
            exit 1
        fi
        generate_doc_for_script "$script_file"
    fi
    
    print_success "Documentation generation completed"
    log_info "Output directory: $OUTPUT_DIR"
}

# Run main function
main "$@"