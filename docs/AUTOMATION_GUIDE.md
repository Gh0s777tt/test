# VantisOS Automation Guide

## Overview

This comprehensive guide covers all automation tools, workflows, and development utilities available in the VantisOS project. The automation infrastructure is designed to streamline development, ensure code quality, and maintain consistency across the codebase.

---

## Table of Contents

1. [GitHub Actions Workflows](#github-actions-workflows)
2. [Pre-commit Hooks](#pre-commit-hooks)
3. [Development Scripts](#development-scripts)
4. [Makefile Commands](#makefile-commands)
5. [Continuous Integration](#continuous-integration)
6. [Documentation Automation](#documentation-automation)
7. [Quality Assurance](#quality-assurance)
8. [Troubleshooting](#troubleshooting)

---

## GitHub Actions Workflows

VantisOS uses GitHub Actions for continuous integration and deployment. All workflow files are located in `.github/workflows/`.

### Available Workflows

| Workflow | Purpose | Trigger |
|----------|---------|---------|
| `script-validation.yml` | Validate shell scripts with shellcheck | Push, PR |
| `dependency-validation.yml` | Check for outdated/vulnerable dependencies | Weekly, Push |
| `build.yml` | Build the project | Push |
| `ci.yml` | Run CI pipeline | Push, PR |
| `docs.yml` | Build and deploy documentation | Push to main |
| `release.yml` | Create releases | Tags |

### Script Validation Workflow

The script validation workflow ensures all shell scripts follow best practices:

```yaml
# Located at .github/workflows/script-validation.yml
# Runs shellcheck on all .sh files
# Validates script syntax
# Checks for proper permissions
```

To run locally:
```bash
# Install shellcheck
sudo apt-get install shellcheck  # Debian/Ubuntu
brew install shellcheck           # macOS

# Run on all scripts
find . -name '*.sh' -exec shellcheck {} +
```

### Dependency Validation Workflow

Automatically checks for:
- Outdated packages (Rust, Node.js, Python)
- Security vulnerabilities
- License compliance

Runs weekly on Monday at 6:00 AM UTC and on dependency file changes.

---

## Pre-commit Hooks

Pre-commit hooks run automatic checks before each commit to catch issues early.

### Configuration

The configuration is in `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/koalaman/shellcheck-precommit
    rev: v0.9.0
    hooks:
      - id: shellcheck
  - repo: https://github.com/igorshubovych/markdownlint-cli
    rev: v0.37.0
    hooks:
      - id: markdownlint
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-json
      - id: check-added-large-files
```

### Installation

```bash
# Install pre-commit (requires Python)
pip install pre-commit

# Install hooks in the repository
pre-commit install

# Run on all files manually
pre-commit run --all-files

# Run on specific files
pre-commit run --files path/to/file.sh
```

### Included Checks

| Check | Description |
|-------|-------------|
| `shellcheck` | Shell script static analysis |
| `markdownlint` | Markdown formatting |
| `trailing-whitespace` | Remove trailing spaces |
| `end-of-file-fixer` | Ensure newline at EOF |
| `check-yaml` | Validate YAML syntax |
| `check-json` | Validate JSON syntax |
| `check-added-large-files` | Prevent large files |

---

## Development Scripts

### Script Library (`scripts/lib/common.sh`)

The common library provides standardized functions for all scripts:

```bash
#!/bin/bash
source scripts/lib/common.sh

# Available functions:
log_info "Information message"
log_warn "Warning message"
log_error "Error message"
log_debug "Debug message"
log_fatal "Fatal error and exit"

print_header "Section Title"

check_command "git"
check_file "Cargo.toml"
validate_args "$#" 2

run_cmd "git status"
ensure_dir "build"
download_file "https://example.com/file.tar.gz" "downloads/"
```

### Setup Script (`scripts/dev/setup_environment.sh`)

Complete development environment setup for various Linux distributions:

```bash
# Basic setup
./scripts/dev/setup_environment.sh

# Full setup with QEMU, Docker, Vagrant
./scripts/dev/setup_environment.sh --full

# Skip package installation (configure only)
./scripts/dev/setup_environment.sh --skip-packages

# Show help
./scripts/dev/setup_environment.sh --help
```

Supported distributions:
- Debian/Ubuntu/Pop!_OS/Linux Mint
- Fedora/RHEL/CentOS
- Arch Linux/Manjaro/EndeavourOS

### Documentation Generator (`scripts/generate_doc_from_script.sh`)

Auto-generates markdown documentation from script headers:

```bash
# Generate doc for a single script
./scripts/generate_doc_from_script.sh scripts/my_script.sh

# Specify output directory
./scripts/generate_doc_from_script.sh scripts/my_script.sh --output-dir docs/scripts/
```

### Script Header Template

For the documentation generator to work, scripts must include this header:

```bash
#!/bin/bash
# Script: script_name.sh
# Purpose: Brief description of what the script does
# Usage: ./scripts/script_name.sh [options]
# Requirements: list of required tools/dependencies
# Author: Your Name
# Date: YYYY-MM-DD
# Version: 1.0.0
# License: MIT
```

---

## Makefile Commands

The Makefile provides a unified interface for common operations:

```bash
# Show all available commands
make help

# Quick Start
make setup          # Initial setup (install cargo tools)
make build          # Build the project
make test           # Run tests

# Development
make dev            # Start development with cargo watch
make fmt            # Format code
make lint           # Run linters
make check          # Run all checks

# Documentation
make docs           # Build documentation
make docs-serve     # Serve docs locally

# Release
make release        # Create release
make changelog      # Generate changelog

# Utilities
make install        # Install dependencies
make update         # Update dependencies
make version        # Show version info
make clean          # Clean build artifacts

# Complete check
make all            # Run fmt + lint + check + test
```

---

## Continuous Integration

### CI Pipeline Stages

1. **Code Quality**
   - Format checking
   - Linting
   - Static analysis

2. **Testing**
   - Unit tests
   - Integration tests
   - Code coverage

3. **Security**
   - Dependency scanning
   - Vulnerability detection
   - License compliance

4. **Build**
   - Compile project
   - Generate artifacts
   - Create ISO images

### Running CI Locally

```bash
# Run all pre-commit checks
pre-commit run --all-files

# Run Rust tests
cargo test --workspace

# Run clippy
cargo clippy --workspace -- -D warnings

# Check formatting
cargo fmt --check

# Full local CI
make all
```

---

## Documentation Automation

### Auto-generated Documentation

Documentation is automatically generated from:

1. **Script Headers** → `docs/scripts/*.md`
2. **Rust Doc Comments** → `target/doc/`
3. **README Sections** → Wiki pages

### Generating Documentation

```bash
# Generate Rust documentation
cargo doc --workspace --no-deps --open

# Generate script documentation
for script in scripts/*.sh; do
    ./scripts/generate_doc_from_script.sh "$script"
done

# Build MkDocs (if configured)
mkdocs build
mkdocs serve  # Local preview
```

### Documentation Standards

- Use Markdown for all documentation
- Include code examples with syntax highlighting
- Keep lines under 100 characters
- Use consistent heading hierarchy
- Include table of contents for long documents

---

## Quality Assurance

### Code Quality Tools

| Tool | Language | Purpose |
|------|----------|---------|
| `rustfmt` | Rust | Code formatting |
| `clippy` | Rust | Linting |
| `shellcheck` | Shell | Script analysis |
| `markdownlint` | Markdown | Documentation linting |
| `prettier` | JS/JSON/YAML | Format checking |
| `eslint` | JavaScript | JS linting |

### Running Quality Checks

```bash
# All quality checks
make lint
make check

# Individual checks
cargo fmt --check
cargo clippy --workspace
shellcheck scripts/**/*.sh
markdownlint docs/**/*.md
```

### Quality Gates

All code must pass these gates before merge:

1. ✅ All tests pass (`make test`)
2. ✅ No clippy warnings (`cargo clippy`)
3. ✅ Proper formatting (`cargo fmt --check`)
4. ✅ No shellcheck errors (`pre-commit run --all-files`)
5. ✅ Documentation builds (`make docs`)

---

## Troubleshooting

### Common Issues

#### Pre-commit Not Running

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Check configuration
cat .pre-commit-config.yaml

# Run manually to debug
pre-commit run --all-files --verbose
```

#### Shellcheck Errors

```bash
# View specific error
shellcheck scripts/my_script.sh

# Disable specific check inline
# shellcheck disable=SC2086
echo "Files: $FILES"
```

#### CI Fails Locally

```bash
# Clear caches
cargo clean
rm -rf target/
pre-commit clean

# Rebuild
make clean
make build
```

#### Permission Denied on Scripts

```bash
# Make all scripts executable
find scripts -name '*.sh' -exec chmod +x {} +

# Check permissions
ls -la scripts/*.sh
```

### Debug Mode

Enable verbose logging for scripts:

```bash
# Set log level
export LOG_LEVEL=DEBUG

# Run script with verbose output
bash -x scripts/my_script.sh

# Or use the -v flag if supported
./scripts/my_script.sh --verbose
```

### Getting Help

1. Check this guide for common solutions
2. Run `make help` for available commands
3. Check workflow files in `.github/workflows/`
4. Review `docs/SCRIPTING_STANDARDS.md` for best practices
5. Open an issue on GitHub with the `automation` label

---

## Quick Reference

```bash
# Setup
./scripts/dev/setup_environment.sh --full

# Development workflow
make setup && make build && make test

# Before commit
pre-commit run --all-files
make lint

# Documentation
make docs

# Full CI (local)
make all
```

---

*This guide is maintained by the VantisOS team. Last updated: March 2025.*