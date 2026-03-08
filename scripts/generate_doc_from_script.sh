#!/bin/bash
# Script: generate_doc_from_script.sh
# Purpose: Auto-generate markdown documentation from script headers
# Usage: ./scripts/generate_doc_from_script.sh <script_file> [--output-dir docs/]
# Requirements: bash, grep, sed, basename
# Author: VantisOS Team
# Date: 2025-03-06
# Version: 1.0.0
# License: MIT

set -euo pipefail

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

# Default output directory
OUTPUT_DIR="docs"

# Parse arguments
SCRIPT_FILE=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --output-dir|-o)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $(basename "$0") <script_file> [--output-dir docs/]"
            echo ""
            echo "Arguments:"
            echo "  script_file        Path to the script to document"
            echo "  --output-dir, -o   Output directory for generated docs (default: docs/)"
            echo "  --help, -h         Show this help message"
            exit 0
            ;;
        *)
            SCRIPT_FILE="$1"
            shift
            ;;
    esac
done

# Validate script file
if [[ -z "$SCRIPT_FILE" ]]; then
    log_error "No script file specified"
    echo "Usage: $(basename "$0") <script_file> [--output-dir docs/]"
    exit 1
fi

if [[ ! -f "$SCRIPT_FILE" ]]; then
    log_error "Script file not found: $SCRIPT_FILE"
    exit 1
fi

log_info "Generating documentation for: $SCRIPT_FILE"

# Extract script metadata
extract_header_value() {
    local script="$1"
    local field="$2"
    grep "^# ${field}:" "$script" 2>/dev/null | head -1 | sed "s/^# ${field}: //" || echo "N/A"
}

# Get script information
SCRIPT_NAME=$(basename "$SCRIPT_FILE")
PURPOSE=$(extract_header_value "$SCRIPT_FILE" "Purpose")
USAGE=$(extract_header_value "$SCRIPT_FILE" "Usage")
REQUIREMENTS=$(extract_header_value "$SCRIPT_FILE" "Requirements")
AUTHOR=$(extract_header_value "$SCRIPT_FILE" "Author")
DATE=$(extract_header_value "$SCRIPT_FILE" "Date")
VERSION=$(extract_header_value "$SCRIPT_FILE" "Version")
LICENSE=$(extract_header_value "$SCRIPT_FILE" "License")

# Generate markdown output
DOC_NAME="${SCRIPT_NAME%.sh}.md"
OUTPUT_PATH="${OUTPUT_DIR}/${DOC_NAME}"

log_info "Creating documentation: $OUTPUT_PATH"

cat > "$OUTPUT_PATH" << EOF
# ${SCRIPT_NAME}

## Overview

**Purpose:** ${PURPOSE}

| Attribute | Value |
|-----------|-------|
| Author | ${AUTHOR} |
| Date | ${DATE} |
| Version | ${VERSION} |
| License | ${LICENSE} |

## Usage

\`\`\`bash
${USAGE}
\`\`\`

## Requirements

${REQUIREMENTS}

## Examples

### Basic Usage

\`\`\`bash
# Run the script
./${SCRIPT_NAME}
\`\`\`

### With Options

\`\`\`bash
# Run with custom options
./${SCRIPT_NAME} --option value
\`\`\`

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Missing dependencies |

## See Also

- [Scripts Reference](SCRIPTS_REFERENCE.md)
- [Scripting Standards](SCRIPTING_STANDARDS.md)

---

*This documentation was auto-generated from ${SCRIPT_NAME}*
EOF

log_info "Documentation created successfully: $OUTPUT_PATH"
echo "$OUTPUT_PATH"