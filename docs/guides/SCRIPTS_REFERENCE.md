# VantisOS Scripts Reference Guide

Complete reference for all automation and build scripts in VantisOS.

## Table of Contents

- [Core Build Scripts](#core-build-scripts)
- [Development Scripts](#development-scripts)
- [Testing Scripts](#testing-scripts)
- [Deployment Scripts](#deployment-scripts)
- [Utility Scripts](#utility-scripts)
- [Script Development Standards](#script-development-standards)

---

## Core Build Scripts

### `scripts/build_iso.sh`
Builds VantisOS ISO image.

**Usage:**
```bash
./scripts/build_iso.sh [options]
```

**Options:**
- `--clean` - Clean build directory before building
- `--debug` - Enable debug output
- `--help` - Show help message

**Requirements:**
- Root privileges
- At least 10GB free disk space
- Build dependencies installed

**Examples:**
```bash
# Standard build
./scripts/build_iso.sh

# Clean build
./scripts/build_iso.sh --clean

# Debug build
./scripts/build_iso.sh --debug
```

---

### `scripts/build_all.sh`
Builds all components of VantisOS.

**Usage:**
```bash
./scripts/build_all.sh [options]
```

**Options:**
- `--skip-tests` - Skip running tests
- `--parallel` - Enable parallel builds
- `--help` - Show help message

**Requirements:**
- Root privileges
- 20GB free disk space
- All build dependencies

**Examples:**
```bash
# Full build
./scripts/build_all.sh

# Build without tests
./scripts/build_all.sh --skip-tests
```

---

### `scripts/build_installable_iso.sh`
Creates installable ISO with installer.

**Usage:**
```bash
./scripts/build_installable_iso.sh [options]
```

**Options:**
- `--version VERSION` - Set version number
- `--output PATH` - Output directory
- `--help` - Show help message

**Requirements:**
- Root privileges
- 15GB free disk space
- Complete build system

**Examples:**
```bash
# Standard installable ISO
./scripts/build_installable_iso.sh

# Custom version
./scripts/build_installable_iso.sh --version 1.5.0
```

---

### `scripts/minimal_build.sh`
Builds minimal VantisOS for testing.

**Usage:**
```bash
./scripts/minimal_build.sh [options]
```

**Options:**
- `--kernel-only` - Build only kernel
- `--help` - Show help message

**Requirements:**
- Root privileges
- 5GB free disk space

**Examples:**
```bash
# Minimal build
./scripts/minimal_build.sh

# Kernel only
./scripts/minimal_build.sh --kernel-only
```

---

### `scripts/start_full_build.sh`
Initiates complete build process with monitoring.

**Usage:**
```bash
./scripts/start_full_build.sh [options]
```

**Options:**
- `--monitor` - Enable build monitoring
- `--email EMAIL` - Email notification on completion
- `--help` - Show help message

**Requirements:**
- Root privileges
- 20GB free disk space

**Examples:**
```bash
# Full build
./scripts/start_full_build.sh

# With monitoring
./scripts/start_full_build.sh --monitor
```

---

## Development Scripts

### `scripts/add_allow_dead_code.sh`
Adds `#[allow(dead_code)]` attribute to Rust code.

**Usage:**
```bash
./scripts/add_allow_dead_code.sh [file|directory]
```

**Arguments:**
- `file|directory` - Rust file or directory to process

**Requirements:**
- Rust toolchain
- `grep`, `sed`, `awk`

**Examples:**
```bash
# Process single file
./scripts/add_allow_dead_code.sh core/kernel/main.rs

# Process directory
./scripts/add_allow_dead_code.sh core/
```

---

### `scripts/add_license.sh`
Adds license headers to source files.

**Usage:**
```bash
./scripts/add_license.sh [options] [file|directory]
```

**Options:**
- `--license TYPE` - License type (MIT, GPL, Apache)
- `--year YEAR` - Copyright year
- `--recursive` - Process directories recursively
- `--help` - Show help message

**Requirements:**
- `sed`, `find`

**Examples:**
```bash
# Add MIT license
./scripts/add_license.sh --license MIT core/

# Add GPL license with custom year
./scripts/add_license.sh --license GPL --year 2025 scripts/
```

---

### `scripts/analyze_dependencies.sh`
Analyzes project dependencies and creates report.

**Usage:**
```bash
./scripts/analyze_dependencies.sh [options]
```

**Options:**
- `--output FILE` - Output file for report
- `--format FORMAT` - Output format (text, json, html)
- `--vulnerability` - Check for vulnerabilities
- `--help` - Show help message

**Requirements:**
- `cargo` (for Rust projects)
- `npm` (for Node.js projects)

**Examples:**
```bash
# Generate text report
./scripts/analyze_dependencies.sh --output deps.txt

# Generate JSON report
./scripts/analyze_dependencies.sh --output deps.json --format json

# Check vulnerabilities
./scripts/analyze_dependencies.sh --vulnerability
```

---

### `scripts/bootstrap_legacy_tree.sh`
Bootstraps legacy code tree structure.

**Usage:**
```bash
./scripts/bootstrap_legacy_tree.sh [options]
```

**Options:**
- `--source PATH` - Source directory
- `--target PATH` - Target directory
- `--help` - Show help message

**Requirements:**
- `git`

**Examples:**
```bash
# Bootstrap to default location
./scripts/bootstrap_legacy_tree.sh

# Custom target
./scripts/bootstrap_legacy_tree.sh --target /tmp/legacy
```

---

### `scripts/init_citadel.sh`
Initializes Citadel security module.

**Usage:**
```bash
./scripts/init_citadel.sh [options]
```

**Options:**
- `--config FILE` - Configuration file
- `--keys PATH` - Keys directory
- `--help` - Show help message

**Requirements:**
- Root privileges
- Citadel dependencies

**Examples:**
```bash
# Initialize with default config
./scripts/init_citadel.sh

# Custom config
./scripts/init_citadel.sh --config /etc/citadel/config.toml
```

---

### `scripts/install_deps.sh`
Installs all build dependencies.

**Usage:**
```bash
./scripts/install_deps.sh [options]
```

**Options:**
- `--minimal` - Install minimal dependencies
- `--dev` - Install development dependencies
- `--docker` - Use Docker for dependencies
- `--help` - Show help message

**Requirements:**
- Root privileges or sudo access
- Internet connection

**Examples:**
```bash
# Install all dependencies
./scripts/install_deps.sh

# Install minimal
./scripts/install_deps.sh --minimal

# Install development tools
./scripts/install_deps.sh --dev
```

---

## Testing Scripts

### `scripts/test_all.sh`
Runs all test suites.

**Usage:**
```bash
./scripts/test_all.sh [options]
```

**Options:**
- `--verbose` - Verbose output
- `--fail-fast` - Stop on first failure
- `--coverage` - Generate coverage report
- `--help` - Show help message

**Requirements:**
- All test dependencies installed
- Built project

**Examples:**
```bash
# Run all tests
./scripts/test_all.sh

# Verbose output
./scripts/test_all.sh --verbose

# With coverage
./scripts/test_all.sh --coverage
```

---

### `scripts/test_coverage.sh`
Generates code coverage report.

**Usage:**
```bash
./scripts/test_coverage.sh [options]
```

**Options:**
- `--output DIR` - Output directory
- `--format FORMAT` - Report format (html, xml, json)
- `--help` - Show help message

**Requirements:**
- Coverage tools installed
- Built project with coverage instrumentation

**Examples:**
```bash
# Generate HTML report
./scripts/test_coverage.sh --output coverage/

# Generate XML report
./scripts/test_coverage.sh --format xml
```

---

### `scripts/test_installer.sh`
Tests installer in QEMU.

**Usage:**
```bash
./scripts/test_installer.sh [options]
```

**Options:**
- `--iso PATH` - ISO file to test
- `--memory SIZE` - Memory size (default: 2G)
- `--help` - Show help message

**Requirements:**
- QEMU installed
- Root privileges

**Examples:**
```bash
# Test default ISO
./scripts/test_installer.sh

# Test specific ISO
./scripts/test_installer.sh --iso build/vantisos-1.5.0.iso

# Custom memory
./scripts/test_installer.sh --memory 4G
```

---

### `scripts/test_install_e2e.sh`
End-to-end installer testing.

**Usage:**
```bash
./scripts/test_install_e2e.sh [options]
```

**Options:**
- `--scenario FILE` - Test scenario file
- `--timeout SECONDS` - Test timeout
- `--help` - Show help message

**Requirements:**
- QEMU installed
- Root privileges
- Test scenarios defined

**Examples:**
```bash
# Run all scenarios
./scripts/test_install_e2e.sh

# Run specific scenario
./scripts/test_install_e2e.sh --scenario tests/scenario1.yml
```

---

### `scripts/run_benchmarks.sh`
Runs performance benchmarks.

**Usage:**
```bash
./scripts/run_benchmarks.sh [options]
```

**Options:**
- `--output FILE` - Output file for results
- `--iterations N` - Number of iterations
- `--help` - Show help message

**Requirements:**
- Benchmark tools installed
- Built project

**Examples:**
```bash
# Run benchmarks
./scripts/run_benchmarks.sh

# Custom iterations
./scripts/run_benchmarks.sh --iterations 10
```

---

## Deployment Scripts

### `scripts/deploy.sh`
Deploys VantisOS to target systems.

**Usage:**
```bash
./scripts/deploy.sh [options]
```

**Options:**
- `--target HOST` - Target host
- `--user USER` - SSH user
- `--key PATH` - SSH key path
- `--help` - Show help message

**Requirements:**
- SSH access to target
- Built ISO

**Examples:**
```bash
# Deploy to host
./scripts/deploy.sh --target example.com --user admin

# With custom key
./scripts/deploy.sh --target example.com --key ~/.ssh/deploy_key
```

---

### `scripts/release.sh`
Creates release package.

**Usage:**
```bash
./scripts/release.sh [options]
```

**Options:**
- `--version VERSION` - Version number
- `--dry-run` - Dry run without creating release
- `--help` - Show help message

**Requirements:**
- Built ISO
- Git configured
- GitHub CLI installed

**Examples:**
```bash
# Create release
./scripts/release.sh --version 1.5.0

# Dry run
./scripts/release.sh --version 1.5.0 --dry-run
```

---

### `scripts/rollback.sh`
Rolls back to previous version.

**Usage:**
```bash
./scripts/rollback.sh [options]
```

**Options:**
- `--version VERSION` - Target version
- `--force` - Force rollback
- `--help` - Show help message

**Requirements:**
- Previous versions available
- Backup configuration

**Examples:**
```bash
# Rollback to previous
./scripts/rollback.sh

# Rollback to specific version
./scripts/rollback.sh --version 1.4.0

# Force rollback
./scripts/rollback.sh --force
```

---

### `scripts/create_live_usb.sh`
Creates bootable live USB.

**Usage:**
```bash
./scripts/create_live_usb.sh [options] [device]
```

**Options:**
- `--iso PATH` - ISO file path
- `--format` - Format device before writing
- `--verify` - Verify after writing
- `--help` - Show help message

**Arguments:**
- `device` - USB device (e.g., /dev/sdb)

**Requirements:**
- Root privileges
- ISO file
- USB device

**Examples:**
```bash
# Create live USB
./scripts/create_live_usb.sh /dev/sdb

# With verification
./scripts/create_live_usb.sh /dev/sdb --verify

# Custom ISO
./scripts/create_live_usb.sh /dev/sdb --iso build/vantisos.iso
```

---

## Utility Scripts

### `scripts/check_installability.sh`
Checks ISO installability.

**Usage:**
```bash
./scripts/check_installability.sh [options] [iso]
```

**Options:**
- `--verbose` - Verbose output
- `--help` - Show help message

**Arguments:**
- `iso` - ISO file path

**Requirements:**
- Root privileges
- ISO file

**Examples:**
```bash
# Check ISO
./scripts/check_installability.sh build/vantisos.iso

# Verbose output
./scripts/check_installability.sh build/vantisos.iso --verbose
```

---

### `scripts/checksum.sh`
Calculates file checksums.

**Usage:**
```bash
./scripts/checksum.sh [options] [file]
```

**Options:**
- `--algorithm ALGO` - Checksum algorithm (md5, sha1, sha256, sha512)
- `--output FILE` - Output file
- `--verify FILE` - Verify against checksum file
- `--help` - Show help message

**Arguments:**
- `file` - File to calculate checksum for

**Requirements:**
- `md5sum`, `sha1sum`, `sha256sum`, or `sha512sum`

**Examples:**
```bash
# Calculate SHA256
./scripts/checksum.sh build/vantisos.iso

# Calculate MD5
./scripts/checksum.sh --algorithm md5 build/vantisos.iso

# Verify checksum
./scripts/checksum.sh --verify checksums.txt build/vantisos.iso
```

---

### `scripts/cleanup.sh`
Cleans build artifacts.

**Usage:**
```bash
./scripts/cleanup.sh [options]
```

**Options:**
- `--all` - Clean all artifacts including caches
- `--keep-iso` - Keep ISO files
- `--dry-run` - Show what would be deleted
- `--help` - Show help message

**Requirements:**
- None

**Examples:**
```bash
# Clean build artifacts
./scripts/cleanup.sh

# Clean all
./scripts/cleanup.sh --all

# Keep ISOs
./scripts/cleanup.sh --keep-iso

# Dry run
./scripts/cleanup.sh --dry-run
```

---

### `scripts/docs_update_checker.sh`
Checks for documentation updates.

**Usage:**
```bash
./scripts/docs_update_checker.sh [options]
```

**Options:**
- `--fix` - Auto-fix issues
- `--help` - Show help message

**Requirements:**
- None

**Examples:**
```bash
# Check documentation
./scripts/docs_update_checker.sh

# Auto-fix issues
./scripts/docs_update_checker.sh --fix
```

---

### `scripts/generate_docs.sh`
Generates documentation from code.

**Usage:**
```bash
./scripts/generate_docs.sh [options]
```

**Options:**
- `--output DIR` - Output directory
- `--format FORMAT` - Output format (markdown, html, pdf)
- `--help` - Show help message

**Requirements:**
- Documentation tools installed

**Examples:**
```bash
# Generate documentation
./scripts/generate_docs.sh

# Custom output
./scripts/generate_docs.sh --output docs/generated/
```

---

### `scripts/package_iso_assets.sh`
Packages ISO assets.

**Usage:**
```bash
./scripts/package_iso_assets.sh [options]
```

**Options:**
- `--source DIR` - Source directory
- `--output FILE` - Output file
- `--compress` - Compress output
- `--help` - Show help message

**Requirements:**
- Source assets directory

**Examples:**
```bash
# Package assets
./scripts/package_iso_assets.sh

# Compress output
./scripts/package_iso_assets.sh --compress
```

---

### `scripts/sign.sh`
Signs files with GPG.

**Usage:**
```bash
./scripts/sign.sh [options] [file]
```

**Options:**
- `--key KEY_ID` - GPG key ID
- `--output FILE` - Output signature file
- `--verify FILE` - Verify signature
- `--help` - Show help message

**Arguments:**
- `file` - File to sign

**Requirements:**
- GPG installed
- GPG key configured

**Examples:**
```bash
# Sign file
./scripts/sign.sh build/vantisos.iso

# Sign with specific key
./scripts/sign.sh --key ABC123 build/vantisos.iso

# Verify signature
./scripts/sign.sh --verify build/vantisos.iso.sig build/vantisos.iso
```

---

### `scripts/verify_repo.sh`
Verifies repository integrity.

**Usage:**
```bash
./scripts/verify_repo.sh [options]
```

**Options:**
- `--fix` - Auto-fix issues
- `--verbose` - Verbose output
- `--help` - Show help message

**Requirements:**
- Git repository

**Examples:**
```bash
# Verify repository
./scripts/verify_repo.sh

# Auto-fix
./scripts/verify_repo.sh --fix

# Verbose output
./scripts/verify_repo.sh --verbose
```

---

### `scripts/security/`
Security-related scripts.

**Available Scripts:**
- Security scanning
- Vulnerability checks
- Compliance verification
- Security testing

---

### `scripts/dev/`
Development utilities.

**Available Scripts:**
- Development environment setup
- Hot reload tools
- Debug helpers

---

### `scripts/lib/`
Shared libraries for scripts.

**Available Libraries:**
- `common.sh` - Common functions and utilities (NEW!)

---

## Script Development Standards

### Header Template

All scripts should include this header:

```bash
#!/bin/bash
# Script: script_name.sh
# Purpose: Brief description of what the script does
# Usage: ./script_name.sh [options] [arguments]
# Requirements: List of required tools/dependencies
# Author: Author name
# Date: Creation date
# Version: 1.0.0

# Source common library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh" || exit 1

# Enable error handling
set -euo pipefail
```

### Best Practices

1. **Always source the common library:**
   ```bash
   source scripts/lib/common.sh
   ```

2. **Use standardized logging:**
   ```bash
   log_info "Starting operation"
   log_warn "Warning message"
   log_error "Error occurred"
   ```

3. **Validate inputs:**
   ```bash
   validate_args 2 "$@"
   check_command "git"
   ```

4. **Handle errors gracefully:**
   ```bash
   setup_traps
   ```

5. **Provide helpful output:**
   ```bash
   print_header "Build Process"
   print_success "Build completed"
   ```

6. **Include usage information:**
   ```bash
   show_help
   ```

### Common Functions Reference

The `scripts/lib/common.sh` library provides:

- **Logging:** `log`, `log_info`, `log_warn`, `log_error`, `log_fatal`
- **Output:** `print_header`, `print_subheader`, `print_success`, `print_error`
- **Validation:** `check_command`, `check_file`, `check_dir`, `validate_args`
- **Execution:** `run_cmd`, `run_silent`
- **Helpers:** `confirm`, `get_os`, `get_arch`, `is_root`, `ensure_root`
- **Utilities:** `download_file`, `calc_checksum`, `backup_file`

For detailed documentation, see `scripts/lib/common.sh` source code.

---

## Troubleshooting

### Common Issues

**Script fails with "command not found":**
- Install required dependencies using `scripts/install_deps.sh`
- Check PATH variable includes required tools

**Permission denied errors:**
- Run script with sudo or as root
- Check script has execute permissions: `chmod +x script.sh`

**Out of disk space:**
- Clean build artifacts: `scripts/cleanup.sh --all`
- Check disk space: `df -h`

**Slow build times:**
- Use `--parallel` flag where available
- Ensure sufficient RAM (8GB minimum recommended)
- Use SSD for faster I/O

### Getting Help

For more help:
- Check inline documentation: `./script.sh --help`
- Read this guide
- Check `docs/guides/TROUBLESHOOTING.md`
- Open an issue on GitHub

---

**Document Version:** 1.0  
**Last Updated:** March 5, 2025  
**Maintained by:** VantisOS Team