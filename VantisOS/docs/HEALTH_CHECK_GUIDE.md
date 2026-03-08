# VantisOS Repository Health Check Guide

## Overview

The `scripts/health_check.sh` script provides comprehensive health monitoring for the VantisOS repository, checking various aspects of repository quality, documentation, code standards, and security.

## Features

### Automated Checks

The health check script performs the following automated checks:

1. **Git Repository Status**
   - Verifies git repository integrity
   - Checks for uncommitted changes
   - Identifies untracked files

2. **Required Files**
   - Validates presence of core files (README.md, LICENSE, Cargo.toml, etc.)
   - Ensures proper repository structure

3. **Documentation**
   - Checks documentation directory structure
   - Verifies key documentation files exist
   - Ensures documentation completeness

4. **Scripts**
   - Validates script permissions
   - Checks script executability
   - Can automatically fix permission issues

5. **GitHub Workflows**
   - Validates workflow configuration
   - Checks for common workflow files
   - Monitors CI/CD setup

6. **Pre-commit Hooks**
   - Verifies pre-commit configuration
   - Checks hook installation status
   - Can automatically install missing hooks

7. **Code Quality Tools**
   - Validates Rust tooling (cargo, rustfmt, clippy)
   - Checks shell script linting (shellcheck)
   - Ensures development tools are available

8. **Security**
   - Verifies security policy exists
   - Checks for CodeQL configuration
   - Validates dependency scanning setup

9. **Dependencies**
   - Checks dependency locking (Cargo.lock)
   - Validates security audit tools
   - Monitors dependency update tools

## Usage

### Basic Usage

```bash
# Run health check with default settings
./scripts/health_check.sh

# Run with detailed output
./scripts/health_check.sh --verbose

# Run with JSON output
./scripts/health_check.sh --json

# Attempt to fix issues automatically
./scripts/health_check.sh --fix

# Show help
./scripts/health_check.sh --help
```

### Examples

**Check repository health:**
```bash
cd /path/to/VantisOS
./scripts/health_check.sh
```

**Run with detailed information:**
```bash
./scripts/health_check.sh --verbose
```

**Automatically fix issues:**
```bash
./scripts/health_check.sh --fix
```

**Get JSON output for CI/CD:**
```bash
./scripts/health_check.sh --json > health_report.json
```

## Output Format

### Standard Output

```
VantisOS Repository Health Check
=================================
✓ Git repository detected
✓ Working directory clean
✓ Required file exists: README.md
✓ Documentation directory exists
⚠ Missing documentation: CONTRIBUTING.md

=========================================
         REPOSITORY HEALTH SUMMARY
=========================================
Passed:   15
Failed:   2
Warnings: 3
=========================================
```

### JSON Output

```json
{
  "summary": {
    "passed": 15,
    "failed": 2,
    "warnings": 3,
    "total": 20
  },
  "checks": {
    "passed": [
      "Git repository detected",
      "Working directory clean",
      "Required file exists: README.md"
    ],
    "failed": [
      "Missing required file: LICENSE",
      "Scripts directory missing"
    ],
    "warnings": [
      "Missing documentation: CONTRIBUTING.md",
      "Script not executable: setup.sh"
    ]
  }
}
```

## Integration with CI/CD

### GitHub Actions Example

Add to `.github/workflows/health-check.yml`:

```yaml
name: Repository Health Check

on:
  schedule:
    - cron: '0 9 * * *'  # Daily at 9 AM
  pull_request:
  workflow_dispatch:

jobs:
  health-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Run health check
        run: ./scripts/health_check.sh --verbose
        continue-on-error: false

      - name: Upload health report
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: health-report
          path: health_report.json
```

### Makefile Integration

Add to your Makefile:

```makefile
.PHONY: health-check

health-check: ## Run repository health check
	@echo "Running repository health check..."
	@./scripts/health_check.sh --verbose
```

Usage:
```bash
make health-check
```

## Customization

### Adding Custom Checks

You can extend the health check script by adding custom check functions:

```bash
check_custom_metric() {
    # Your custom check logic here
    if [[ condition ]]; then
        log_pass "Custom check passed"
    else
        log_fail "Custom check failed"
    fi
}

# Add to main() function
main() {
    # ... existing checks ...
    check_custom_metric
    print_summary
}
```

### Modifying Required Files

Update the `check_required_files()` function:

```bash
local required_files=(
    "README.md"
    "LICENSE"
    "Cargo.toml"
    "YOUR_CUSTOM_FILE.md"  # Add your file
)
```

## Troubleshooting

### Common Issues

**Issue: "Not a git repository"**
- Solution: Run the script from within the VantisOS git repository
- Example: `cd VantisOS && ./scripts/health_check.sh`

**Issue: "Permission denied" on health_check.sh**
- Solution: Make the script executable
- Example: `chmod +x scripts/health_check.sh`

**Issue: JSON output empty**
- Solution: Install jq for JSON processing
- Example: `sudo apt-get install jq`

### Automatic Fixes

The `--fix` flag can automatically resolve certain issues:

- Make scripts executable
- Install pre-commit hooks
- Fix file permissions

Example:
```bash
./scripts/health_check.sh --fix
```

## Best Practices

### Regular Health Checks

1. **Before commits**: Run health check to ensure repository quality
2. **CI/CD integration**: Add to CI pipeline for continuous monitoring
3. **Scheduled checks**: Run daily/weekly to catch issues early
4. **Before releases**: Comprehensive check before releasing

### Monitoring Health Trends

Track health check results over time:
```bash
# Log results to file
./scripts/health_check.sh --json >> health_log.json

# Analyze trends
jq '.summary' health_log.json
```

### Team Integration

- Add health check to contribution guidelines
- Require passing health checks for pull requests
- Use health check results in code reviews

## Exit Codes

- `0`: All checks passed (no failures)
- `1`: Some checks failed or warnings present

## Requirements

- **Required**: bash, git
- **Optional**: jq (for JSON output)
- **Optional**: cargo, rustfmt, clippy (for Rust checks)
- **Optional**: shellcheck (for script linting)
- **Optional**: pre-commit (for hook checks)

## Advanced Usage

### Conditional Execution

```bash
# Only fail on critical issues
if ! ./scripts/health_check.sh | grep -q "Failed.*[1-9]"; then
    echo "No critical issues found"
else
    echo "Critical issues detected!"
    exit 1
fi
```

### Integration with Monitoring

```bash
# Send health report to monitoring system
./scripts/health_check.sh --json | \
  curl -X POST -H "Content-Type: application/json" \
  -d @- https://monitoring.example.com/api/health
```

### Multiple Repository Checks

```bash
# Check multiple repositories
for repo in /path/to/repos/*/; do
    echo "Checking: $repo"
    cd "$repo"
    ./scripts/health_check.sh --json > "${repo##*/}_health.json"
done
```

## Contributing

To improve the health check script:

1. Add new check functions following the naming pattern `check_*()`
2. Use `log_pass()`, `log_fail()`, `log_warn()`, `log_info()` for output
3. Update documentation for new checks
4. Test thoroughly before submitting

## Support

For issues or questions:
1. Check this documentation
2. Review the script comments
3. Run with `--verbose` for detailed output
4. Open an issue on GitHub with health check output

---

**Version**: 1.0.0  
**Last Updated**: March 2025  
**Maintained by**: VantisOS Team