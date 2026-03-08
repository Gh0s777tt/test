# VantisOS Scripting Standards

Comprehensive guide for writing high-quality, maintainable scripts for VantisOS.

## Table of Contents

- [Overview](#overview)
- [Script Structure](#script-structure)
- [Coding Conventions](#coding-conventions)
- [Best Practices](#best-practices)
- [Error Handling](#error-handling)
- [Logging Standards](#logging-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Requirements](#documentation-requirements)
- [Security Considerations](#security-considerations)
- [Performance Guidelines](#performance-guidelines)

---

## Overview

This document defines the standards and best practices for writing scripts in the VantisOS project. Following these standards ensures:

- **Consistency** across all scripts
- **Maintainability** over time
- **Reliability** in production
- **Readability** for contributors
- **Security** by default

### Why Standards Matter

Well-written scripts are easier to:
- Debug when issues arise
- Extend with new features
- Review and understand
- Test and validate
- Document and maintain

---

## Script Structure

### Required Header

All scripts MUST include a standardized header:

```bash
#!/bin/bash
# Script: script_name.sh
# Purpose: Brief description of what the script does in one sentence
# Usage: ./script_name.sh [options] [arguments]
# Requirements: List of required tools/dependencies (e.g., git, docker, root)
# Author: Author name or "VantisOS Team"
# Date: Creation date (YYYY-MM-DD)
# Version: 1.0.0
# License: SPDX-License-Identifier: MIT OR GPL-2.0
```

### Source Common Library

All scripts MUST source the common library after the header:

```bash
# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common library
source "${SCRIPT_DIR}/lib/common.sh" || {
    echo "Error: Failed to load common library" >&2
    exit 1
}
```

### Enable Error Handling

All scripts MUST enable strict error handling:

```bash
# Exit on error, undefined variables, and pipe failures
set -euo pipefail

# Enable debug mode if needed
# set -x  # Uncomment for debugging
```

### Set Up Traps

Set up error and exit handlers:

```bash
# Setup error handling and cleanup
setup_traps

# Or define custom handlers
trap cleanup EXIT
trap 'error_handler $LINENO' ERR
```

### Main Function Pattern

Use a main function pattern for better organization:

```bash
# Main script logic
main() {
    # Parse arguments
    parse_arguments "$@"
    
    # Validate prerequisites
    validate_prerequisites
    
    # Execute main logic
    execute_operation
    
    # Success
    log_info "Operation completed successfully"
}

# Run main function with all arguments
main "$@"
```

---

## Coding Conventions

### Naming Conventions

**Variables:**
- Use lowercase with underscores: `my_variable_name`
- Constants use uppercase: `MAX_RETRIES`, `LOG_DIR`
- Private variables (internal use) prefix with underscore: `_internal_var`

```bash
# Good
user_name="John"
MAX_RETRIES=5
_temp_file="/tmp/temp.txt"

# Bad
username="John"
maxRetries=5
TempFile="/tmp/temp.txt"
```

**Functions:**
- Use lowercase with underscores: `function_name()`
- Use descriptive names: `validate_input()`, `download_file()`

```bash
# Good
validate_dependencies() {
    # ...
}

# Bad
validateDeps() {
    # ...
}
```

**Files:**
- Use lowercase with underscores: `script_name.sh`
- Use descriptive names: `build_iso.sh`, `test_install.sh`

### Indentation and Formatting

- Use 4 spaces for indentation (no tabs)
- Maximum line length: 120 characters
- Space after `if`, `while`, `for`: `if [ ... ]`
- Space around operators: `a=1` vs `a = 1`

```bash
# Good
if [ "$var1" -eq "$var2" ]; then
    echo "Equal"
fi

for i in {1..10}; do
    echo "Processing $i"
done

# Bad
if [ "$var1" -eq "$var2" ];then
echo "Equal"
fi
```

### Comments

- Use `#` for single-line comments
- Comment complex logic
- Document function purpose
- Include usage examples

```bash
# Validate that required commands are available
validate_commands() {
    # Check for git command
    if ! command -v git &> /dev/null; then
        log_error "git is required but not installed"
        exit 1
    fi
    
    # Check for docker if needed
    if [ "$USE_DOCKER" = "true" ]; then
        check_command "docker"
    fi
}
```

---

## Best Practices

### Use Functions

Break down complex scripts into functions:

```bash
# Good
main() {
    validate_prerequisites
    download_sources
    build_components
    run_tests
    create_package
}

# Bad
# All logic in main script without functions
```

### Validate Inputs

Always validate user input:

```bash
# Validate number of arguments
validate_args 2 "$@"

# Validate arguments
version="$1"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format: $version"
    exit 1
fi

# Validate paths
if [[ ! -d "$output_dir" ]]; then
    log_error "Output directory does not exist: $output_dir"
    exit 1
fi
```

### Check Dependencies

Verify required tools are available:

```bash
# Check single command
check_command "docker"

# Check multiple commands
check_commands "git" "docker" "make"
```

### Use Set for Options

```bash
#!/bin/bash
# Enable strict mode
set -euo pipefail

# Disable globbing
set -f

# Enable debug mode (uncomment for debugging)
# set -x
```

### Quote Variables

Always quote variables to prevent word splitting:

```bash
# Good
echo "User: $user_name"
rm -rf "${temp_dir}/*"

# Bad - can cause issues with spaces
echo "User: $user_name"
rm -rf $temp_dir/*
```

### Use Arrays for Lists

```bash
# Good - use arrays
files=("file1.txt" "file2.txt" "file3.txt")
for file in "${files[@]}"; do
    echo "Processing: $file"
done

# Bad - space-separated strings
files="file1.txt file2.txt file3.txt"
for file in $files; do
    echo "Processing: $file"
done
```

---

## Error Handling

### Set Exit Trap

```bash
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        log_error "Script failed with exit code: $exit_code"
    fi
    
    # Cleanup temporary files
    if [ -n "${TEMP_DIR:-}" ]; then
        rm -rf "$TEMP_DIR"
    fi
}

trap cleanup EXIT
```

### Handle Errors Gracefully

```bash
# Good - handle errors
if ! run_cmd "Build ISO" build_iso; then
    log_error "Build failed, rolling back..."
    rollback
    exit 1
fi

# Bad - let errors propagate
build_iso
```

### Check Command Success

```bash
# Check command success explicitly
if command -v docker &> /dev/null; then
    log_info "Docker is installed"
else
    log_error "Docker is not installed"
    exit 1
fi

# Or use || (OR operator)
docker --version || {
    log_error "Docker not found"
    exit 1
}
```

### Provide Meaningful Error Messages

```bash
# Good - specific error message
if [ ! -f "$config_file" ]; then
    log_error "Configuration file not found: $config_file"
    log_info "Please create the configuration file or run: setup_config.sh"
    exit 1
fi

# Bad - generic error message
if [ ! -f "$config_file" ]; then
    echo "Error" >&2
    exit 1
fi
```

---

## Logging Standards

### Use Common Library Logging

Always use the logging functions from `scripts/lib/common.sh`:

```bash
# Log levels
log_debug "Detailed debugging information"
log_info "General informational message"
log_warn "Warning message"
log_error "Error message"
log_fatal "Fatal error, will exit"

# Convenience functions
print_header "Build Process"
print_subheader "Compiling Kernel"
print_success "Build completed"
print_error "Build failed"
```

### Log Important Events

```bash
log_info "Starting build process"
log_debug "Build configuration: $config"
log_info "Compiling kernel..."
print_success "Kernel compiled"
log_info "Build completed in $((end_time - start_time)) seconds"
```

### Configure Log Level

```bash
# Set log level via environment variable
export LOG_LEVEL=DEBUG  # DEBUG, INFO, WARN, ERROR, FATAL

# Or in script
LOG_LEVEL="${LOG_LEVEL:-INFO}"
```

### Use Structured Logging

```bash
# Structured logging for better parsing
log_info "Starting operation" \
    "operation=build" \
    "version=$version" \
    "target=$target"
```

---

## Testing Guidelines

### Write Testable Code

```bash
# Good - use functions that can be tested
build_component() {
    local component="$1"
    # Build logic
    return $?
}

# Bad - hard to test inline logic
if [ "$component" = "kernel" ]; then
    # Inline build logic
fi
```

### Add Validation

```bash
# Validate script can run
validate_prerequisites() {
    log_info "Validating prerequisites..."
    
    # Check required commands
    check_commands "git" "docker" || return 1
    
    # Check disk space
    check_disk_space "/tmp" 10 || return 1
    
    # Check permissions
    if [ ! -w "$BUILD_DIR" ]; then
        log_error "No write permission for build directory"
        return 1
    fi
    
    log_info "Prerequisites validated"
    return 0
}
```

### Provide Dry Run Mode

```bash
DRY_RUN="${DRY_RUN:-false}"

run_command() {
    local cmd="$1"
    
    if [ "$DRY_RUN" = "true" ]; then
        log_info "DRY RUN: Would execute: $cmd"
        return 0
    fi
    
    eval "$cmd"
}
```

---

## Documentation Requirements

### Inline Documentation

Document complex logic:

```bash
# Calculate optimal block size based on available memory
# Formula: min(available_memory / 2, 1GB)
calculate_block_size() {
    local available_memory
    available_memory=$(free -g | awk '/^Mem:/{print $7}')
    
    # Use half of available memory, max 1GB
    local block_size=$((available_memory / 2))
    if [ $block_size -gt 1 ]; then
        block_size=1
    fi
    
    echo "${block_size}G"
}
```

### Function Documentation

Document function purpose and parameters:

```bash
# Downloads a file from URL with progress reporting
# Arguments:
#   $1 - URL to download
#   $2 - Output file path
# Returns:
#   0 on success, 1 on failure
download_file() {
    local url="$1"
    local output="$2"
    
    # Implementation
}
```

### Usage Examples

Provide usage examples:

```bash
# Usage:
#   ./build_iso.sh [options]
#
# Options:
#   --clean       Clean build directory before building
#   --debug       Enable debug output
#   --version V   Set version number
#   --help        Show this help message
#
# Examples:
#   ./build_iso.sh
#   ./build_iso.sh --clean --debug
#   ./build_iso.sh --version 1.5.0
```

---

## Security Considerations

### Validate All Inputs

```bash
# Sanitize file paths
file_path="${1:-}"
if [[ "$file_path" =~ \.\. ]]; then
    log_error "Invalid file path: contains '..'"
    exit 1
fi

# Sanitize version numbers
version="${1:-}"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format"
    exit 1
fi
```

### Avoid Dangerous Operations

```bash
# Bad - dangerous
rm -rf /tmp/*
chmod 777 /var/www

# Good - safe
rm -rf "${TEMP_DIR:-/tmp/vantisos}/*"
chmod 755 "${INSTALL_DIR:-/var/www/vantisos}"
```

### Use Secure Defaults

```bash
# Use secure defaults
UMASK="${UMASK:-022}"
export PATH="/usr/local/bin:/usr/bin:/bin"

# Don't expose secrets
if [ -f "$SECRET_FILE" ]; then
    source "$SECRET_FILE"
    # Don't echo secrets
fi
```

### Check Permissions

```bash
# Check file permissions
if [ -f "$config_file" ]; then
    if [ "$(stat -c %a "$config_file")" != "600" ]; then
        log_error "Configuration file must have 600 permissions"
        exit 1
    fi
fi
```

---

## Performance Guidelines

### Use Efficient Commands

```bash
# Good - efficient
find . -type f -name "*.sh" | head -10

# Bad - inefficient
find . -type f -name "*.sh" | grep -v node_modules | head -10
```

### Minimize Subshells

```bash
# Good - avoid unnecessary subshells
while IFS= read -r line; do
    process "$line"
done < file.txt

# Bad - creates subshell for each line
cat file.txt | while read line; do
    process "$line"
done
```

### Use Built-in Features

```bash
# Good - use bash builtins
for i in {1..1000}; do
    # Something
done

# Bad - external command
seq 1 1000 | while read i; do
    # Something
done
```

### Cache Results

```bash
# Cache expensive operations
if [ -z "${CACHED_VALUE:-}" ]; then
    CACHED_VALUE=$(expensive_operation)
fi

# Use cached value
process "$CACHED_VALUE"
```

---

## Code Review Checklist

Before submitting a script, ensure:

- [ ] Standard header is present
- [ ] Common library is sourced
- [ ] Error handling is enabled (`set -euo pipefail`)
- [ ] All inputs are validated
- [ ] Dependencies are checked
- [ ] Logging is used throughout
- [ ] Error messages are clear
- [ ] Functions are well-documented
- [ ] Security best practices are followed
- [ ] Script is executable
- [ ] Shellcheck passes without warnings
- [ ] Usage examples are provided

---

## Example Script Template

```bash
#!/bin/bash
# Script: example_script.sh
# Purpose: Example script following VantisOS standards
# Usage: ./example_script.sh [options] [arguments]
# Requirements: git, bash 4.0+
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

# Script configuration
VERSION="${VERSION:-1.0.0}"
OUTPUT_DIR="${OUTPUT_DIR:-./output}"
VERBOSE="${VERBOSE:-false}"

# Parse command line arguments
parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                VERSION="$2"
                shift 2
                ;;
            --output)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
}

# Validate prerequisites
validate_prerequisites() {
    print_header "Validating Prerequisites"
    
    check_commands "git" "make" || return 1
    check_disk_space "$OUTPUT_DIR" 5 || return 1
    
    ensure_dir "$OUTPUT_DIR"
    
    print_success "Prerequisites validated"
    return 0
}

# Main operation
execute_operation() {
    print_header "Executing Operation"
    
    log_info "Starting operation with version $VERSION"
    log_debug "Output directory: $OUTPUT_DIR"
    
    # Perform operation
    # ...
    
    print_success "Operation completed"
    return 0
}

# Main function
main() {
    print_header "Example Script v$VERSION"
    
    # Parse arguments
    parse_arguments "$@"
    
    # Validate prerequisites
    validate_prerequisites || exit 1
    
    # Execute operation
    execute_operation || exit 1
    
    # Success
    log_info "All operations completed successfully"
}

# Run main function
main "$@"
```

---

## Resources

- [Bash Reference Manual](https://www.gnu.org/software/bash/manual/)
- [ShellCheck](https://www.shellcheck.net/) - Static analysis for shell scripts
- [Bash Style Guide](https://google.github.io/styleguide/shellguide.html)
- VantisOS Scripts Reference: `docs/SCRIPTS_REFERENCE.md`
- Common Library: `scripts/lib/common.sh`

---

**Document Version:** 1.0  
**Last Updated:** March 5, 2025  
**Maintained by:** VantisOS Team