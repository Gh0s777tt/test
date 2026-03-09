# VantisOS v1.4.0 - Developer Guide

## Overview
This guide provides developers with information on building, modifying, and extending VantisOS v1.4.0.

## Development Environment

### Required Tools
- **Rust**: 1.93.1 or later
- **QEMU**: 7.2.22 or later (for testing)
- **GRUB**: 2.06-13+ (for booting)
- **Git**: For version control
- **Make**: For building

### Optional Tools
- **GDB**: For debugging
- **Objdump**: For analyzing binaries
- **Readelf**: for examining ELF files

## Building VantisOS

### Prerequisites
1. Install Rust toolchain
2. Install QEMU for testing
3. Clone the repository

### Build Process
```bash
# Build kernel
bash build_advanced_kernel.sh

# Create ISO
bash create_advanced_iso.sh

# Test in QEMU
qemu-system-x86_64 -cdrom vantisos-0.5.0-advanced.iso -m 512M
```

### Build Steps
1. **Compile**: `rustc --target x86_64-unknown-none`
2. **Link**: `ld -T linker.ld`
3. **Strip**: `strip kernel.elf`
4. **Convert**: `objcopy -O binary`
5. **Verify**: Check multiboot header

## Project Structure

```
src/verified/v0.5.0_kernel/
├── main.rs              # Kernel entry point
├── vga_console.rs       # VGA console
├── memory.rs            # Memory management
├── interrupt.rs         # Interrupt handling
├── syscall.rs           # System call interface
├── process.rs           # Process management
├── thread.rs            # Thread management
├── filesystem.rs        # File system interface
├── integration.rs       # Integration
├── performance.rs       # Performance counters
├── security.rs          # Security features
└── linker.ld            # Linker script

tests/
├── test_framework.rs     # Test framework
├── unit_tests.rs        # Unit tests
├── integration_tests.rs # Integration tests
├── benchmarks.rs        # Performance benchmarks
└── security_tests.rs    # Security tests
```

## Adding New Features

### Adding a New System Call
1. Define system call number in `syscall.rs`
2. Implement system call handler
3. Add to system call dispatcher
4. Write tests

### Adding a New Kernel Module
1. Create module file in `src/verified/v0.5.0_kernel/`
2. Add module to `main.rs`
3. Implement initialization
4. Write tests

## Coding Standards

### Rust Style
- Use `#![no_std]` for kernel code
- Use `unsafe` only when necessary
- Document unsafe blocks
- Use Result types for error handling
- Avoid panics in kernel code

### Naming Conventions
- Functions: `snake_case`
- Types: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values
- Handle errors gracefully
- Log errors for debugging

## Testing

### Running Tests
```bash
# Run all tests
bash tests/run_tests.sh

# Run unit tests
bash tests/run_unit_tests.sh

# Run integration tests
bash tests/run_integration_tests.sh
```

### Writing Tests
1. Use test framework in `tests/test_framework.rs`
2. Write unit tests in `tests/unit_tests.rs`
3. Write integration tests in `tests/integration_tests.rs`
4. Run tests and verify results

## Debugging

### Using GDB
```bash
# Start QEMU with GDB
qemu-system-x86_64 -cdrom vantisos-0.5.0-advanced.iso -s -S

# Connect GDB
gdb build/kernel.elf
(gdb) target remote :1234
(gdb) break _start
(gdb) continue
```

### Common Issues
- **Multiboot header not found**: Check linker script
- **Kernel doesn't boot**: Verify entry point
- **No VGA output**: Check console initialization
- **Kernel panic**: Check error message

## Performance Optimization

### Optimization Techniques
1. Use `#[inline]` for small functions
2. Use `const` for compile-time constants
3. Avoid dynamic allocation in hot paths
4. Use efficient algorithms
5. Profile and optimize bottlenecks

### Performance Targets
- **Boot Time**: < 5 seconds
- **System Call**: < 5 μs
- **Context Switch**: < 1 μs
- **Memory Allocation**: < 1 μs

## Security Considerations

### Memory Safety
- Use Rust's type system
- Avoid unsafe code when possible
- Validate pointers before use
- Use bounds checking

### Access Control
- Implement privilege checks
- Use capability-based security
- Validate user input
- Prevent privilege escalation

## Contributing

### Contribution Guidelines
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests
5. Submit a pull request

### Code Review
- Follow coding standards
- Write clear commit messages
- Include tests for new features
- Update documentation

## Resources

### Documentation
- User Guide: `docs/v0.5.0/USER_GUIDE.md`
- API Reference: `docs/v0.5.0/API_REFERENCE.md`
- Architecture: `docs/v0.5.0/ARCHITECTURE.md`

### External Resources
- Rust Book: https://doc.rust-lang.org/book/
- OSDev Wiki: https://wiki.osdev.org/
- Multiboot Specification: https://www.gnu.org/software/grub/manual/multiboot.html

---

**Last Updated**: March 1, 2025
**Version**: 0.5.0