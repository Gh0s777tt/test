# Compatibility Tests

This directory contains tests for VantisOS compatibility systems.

## Test Structure

```
tests/compatibility/
├── mod.rs                      # Test module
├── vnt_apps_test.rs           # VNT Apps tests
├── android_subsystem_test.rs  # Android Subsystem tests
└── legacy_airlock_test.rs     # Legacy Airlock tests
```

## Running Tests

### Run All Compatibility Tests
```bash
cargo test --test compatibility
```

### Run Specific Test Suite
```bash
# VNT Apps tests
cargo test --test vnt_apps_test

# Android Subsystem tests
cargo test --test android_subsystem_test

# Legacy Airlock tests
cargo test --test legacy_airlock_test
```

### Run Specific Test
```bash
cargo test test_vnt_manifest_creation
cargo test test_android_manifest_creation
cargo test test_wine_integration
```

## Test Coverage

### VNT Apps Tests
- ✅ VNT manifest creation
- ✅ VNT permissions
- ✅ VNT resources
- ✅ VNT package manager
- ✅ WASM runtime
- ✅ Capability system
- ✅ Sandbox isolation

### Android Subsystem Tests
- ✅ Android manifest creation
- ✅ Activity creation
- ✅ Service creation
- ✅ Android runtime
- ✅ Binder IPC
- ✅ HAL manager
- ✅ Permission system
- ✅ Android sandbox
- ✅ Play Services

### Legacy Airlock Tests
- ✅ Wine integration
- ✅ EXE loader
- ✅ Windows API translation
- ✅ DLL loading
- ✅ Registry emulation
- ✅ File system redirection
- ✅ Wine sandbox
- ✅ Malware scanning
- ✅ Compatibility mode

## Test Results

All tests should pass with 100% success rate.

## Adding New Tests

1. Create test function in appropriate test file
2. Use `#[test]` attribute
3. Follow naming convention: `test_<feature>_<functionality>`
4. Add assertions to verify expected behavior
5. Run tests to verify they pass

## Continuous Integration

Tests are automatically run on:
- Every pull request
- Every merge to main branch
- Nightly builds

## Troubleshooting

### Test Failures

If tests fail:
1. Check test output for error messages
2. Verify test dependencies are installed
3. Ensure compatibility systems are properly initialized
4. Check for resource conflicts
5. Review test assertions

### Common Issues

**Issue**: Tests timeout
- **Solution**: Increase timeout in test configuration

**Issue**: Tests fail with permission errors
- **Solution**: Run tests with appropriate permissions

**Issue**: Tests fail with missing dependencies
- **Solution**: Install required dependencies

## Contributing

When adding new compatibility features:
1. Add corresponding tests
2. Ensure 100% test coverage
3. Update this README
4. Run all tests before submitting PR

## Resources

- VantisOS Documentation: https://docs.vantisos.ai
- Compatibility Guide: ../docs/compatibility/COMPATIBILITY_GUIDE.md
- Test Documentation: https://docs.vantisos.ai/testing