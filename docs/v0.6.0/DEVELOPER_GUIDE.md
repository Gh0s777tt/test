# VantisOS v0.6.0 "Mobile Ready" - Developer Guide

**Version**: 0.6.0  
**Date**: March 1, 2025  
**Status**: Production Ready

---

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Build Process](#build-process)
4. [Project Structure](#project-structure)
5. [Coding Standards](#coding-standards)
6. [Testing](#testing)
7. [Debugging](#debugging)
8. [Contribution Guidelines](#contribution-guidelines)

---

## Introduction

Welcome to the VantisOS v0.6.0 "Mobile Ready" Developer Guide. This guide provides comprehensive information for developers working on VantisOS, including build process, project structure, coding standards, testing, debugging, and contribution guidelines.

### Developer Goals

- Build VantisOS from source
- Understand VantisOS architecture
- Follow coding standards
- Write clean, maintainable code
- Test your code thoroughly
- Debug issues effectively
- Contribute to VantisOS development

---

## Getting Started

### Prerequisites

**Required Tools**:
- Rust 1.93.1 or later
- ARM64 toolchain: `aarch64-linux-gnu-gcc`, `aarch64-linux-gnu-ld`, `aarch64-linux-gnu-objcopy`
- Git for version control
- QEMU ARM64 for testing (optional but recommended)

**Install Rust**:
```bash
curl --proto '=https' --tlsv1.93.1 https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Install ARM64 Toolchain**:
```bash
sudo apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

**Clone Repository**:
```bash
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
git checkout 0.4.1
```

---

## Build Process

### Building VantisOS Kernel

VantisOS uses a custom build process for the ARM64 kernel.

#### Build Script

The main build script is located at `src/verified/v0.6.0_kernel/build_arm64.sh`:

```bash
#!/bin/bash

echo "Building VantisOS v0.6.0 ARM64 kernel..."

# Step 1: Compile as object file
echo "Step 1: Compiling as object file..."
rustc --target aarch64-unknown-none --edition 2021 \
     -C opt-level=z -C panic=abort -C lto=fat \
     -o build/arm64_kernel.o src/lib.rs

# Step 2: Link to ELF
echo "Step 2: Linking to ELF..."
aarch64-linux-gnu-ld -T arm64/linker.ld \
    -o build/arm64_kernel.elf \
    build/arm64_kernel.o

# Step 3: Convert to raw binary
echo "Step 3: Converting to raw binary..."
aarch64-linux-gnu-objcopy -O binary build/arm64_kernel.elf build/arm64_kernel.bin

# Step 4: Strip ELF
echo "Step 4: Stripping ELF..."
aarch64-linux-gnu-strip --strip-all build/arm64_kernel.elf

echo "Build complete!"
echo ""
echo "Build results:"
echo "  Object file: $(stat -c%s build/arm64_kernel.o) bytes"
echo "  ELF file: $(stat -c%s build/arm64_kernel.elf) bytes"
echo "  Binary file: $(stat -c%s build/arm64_kernel.bin) bytes"
echo ""
echo "Binary header:"
xxd build/arm64_kernel.bin | head -n 1
echo ""
echo "ARM64 kernel ready for testing!"
echo ""
echo "To test in QEMU ARM64:"
echo "  qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M -kernel build/arm64_kernel.bin -serial stdio"
```

#### Building the Kernel

To build the VantisOS kernel:

```bash
cd src/verified/v0.6.0_kernel
bash build_arm64.sh
```

**Build Output**:
```
Building VantisOS v0.6.0 ARM64 kernel...
Step 1: Compiling as object file...
warning: 3 warnings emitted

Step 2: Linking to ELF...
aarch64-linux-gnu-ld: warning: build/arm64_kernel.elf has a LOAD segment with RWX permissions

Step 3: Converting to raw binary...
Step 4: Stripping ELF...

Build complete!

Build results:
  Object file: 56K
  ELF file:    76K
  Binary file: 12K

Binary header:
00000000: fe0f 1ff8 0000 00b0 00d0 0491            ............

ARM64 kernel ready for testing!

To test in QEMU ARM64:
  qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M -kernel build/arm64_kernel.bin -serial stdio
```

#### Build Artifacts

The build process produces the following artifacts:
- `build/arm64_kernel.o` - Object file (56KB)
- `build/arm64_kernel.elf` - ELF file (76KB)
- `build/arm64_kernel.bin` - Raw binary (12KB)
- `build/arm64_kernel_stripped.elf` - Stripped ELF (smaller size)

---

### Creating Bootable ISO

To create a bootable ISO for testing:

```bash
cd src/verified/v0.6.0_kernel
bash create_test_iso.sh
```

This creates `vantisos-0.6.0-test.iso` which can be booted in QEMU ARM64.

---

### Testing in QEMU

To test the kernel in QEMU ARM64:

```bash
qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M \
  -kernel build/arm64_kernel.bin \
  -serial stdio \
  -nographic
```

**Expected Output**:
```
VantisOS v0.6.0 ARM64 kernel booting...
Boot parameters:
  DTB pointer: 0x80000000
  DTB size: 0x00010000
  x0: 0x00000000
  x1: 0x00000000
  x2: 0x00000000
  3: 0x00000000

ARM64 kernel initialized successfully!
```

---

## Project Structure

### Directory Structure

```
VantisOS/
├── src/
│   └── verified/
│       ├── v0.6.0_kernel/
│       │   ├── arm64/
│       │   │   ├── boot.rs
│       │   │   ├── memory.rs
│       │   │   ├── interrupt.rs
│       │   │   ├── optimization.rs
│       │   │   ├── benchmark.rs
│       │   │   ├── memset_memcpy.rs
│       │   ├── display/
│       │   │   ├── mipi_dsi.rs
│       │   │   ├── touchscreen.rs
│       │   │   └── gpu.rs
│       │   ├── input/
│       │   │   ├── accelerometer.rs
│       │   │   ├── gyroscope.rs
│       │   │   └── magnetometer.rs
│       │   ├── network/
│       │   │   ├── wifi.rs
│             │   │   ├── bluetooth.rs
│       │   │   ├── cellular.rs
│       │   │   └── gps.rs
│       │   ├── storage/
│       │   │   ├── emmc.rs
│       │   │   ├── sdcard.rs
│       │   │   └── ufs.rs
│       │   ├── ui/
│       │   │   ├── framework.rs
│       │   │   ├── widgets.rs
│       │   │   ├── event_routing.rs
│       │   │   ├── touch_event.rs
│       │   │   ├── gestures.rs
│       │   │   ├── animations.rs
│       │   │   ├── system_ui.rs
│       │   │   └── app_framework.rs
│       │   ├── lib.rs
│       │   └── linker.ld
│       ├── compliance/
│       │   ├── compliance_pci_dss.rs
│       │   ├── compliance_medical.rs
│       │   └── compliance_iso27001.rs
│       └── ...
├── docs/
│   ├── v0.6.0/
│   │   ├── ARCHITECTURE.md
│   │   ├── API_REFERENCE.md
│   │   ├── USER_GUIDE.md
│   │   └── DEVELOPER_GUIDE.md
│   ├── compliance/
│   ├── reports/
│   └── ...
├── tests/
│   ├── integration/
│   ├── performance/
│   ├── security/
│   ├── compatibility/
│   ├── stress/
│   └── ui/
└── build_arm64.sh
```

### Module Organization

VantisOS is organized into logical modules:

**Kernel Modules** (`src/verified/v0.6.0_kernel/arm64/`):
- `boot.rs` - Boot process and initialization
- `memory.rs` - Memory management (page allocator, heap allocator, virtual memory)
- `interrupt.rs` - Interrupt handling (GIC, exceptions, IRQs)
- `optimization.rs` - Performance optimization and benchmarking
- `benchmark.rs` - Performance benchmarks

**Driver Modules**:
- `display/` - Display drivers (MIPI DSI, touchscreen, GPU)
- `input/` - Input drivers (accelerometer, gyroscope, magnetometer)
- `network/` - Network drivers (WiFi, Bluetooth, cellular, GPS)
- `storage/` - Storage drivers (eMMC, SD Card, UFS)

**UI Modules** (`src/verified/v0.6.0_kernel/arm64/ui/`):
- `framework.rs` - UI framework core
- `widgets.rs` - Widget system (Button, Label, TextField)
- `event_routing.rs` - Event routing system
- `touch_event.rs` - Touch event system
- `gestures.rs` - Gesture recognition
- `animations.rs` - Animation system
- `system_ui.rs` - System UI components
- `app_framework.rs` - Application framework

**Compliance Modules** (`src/verified/compliance/`):
- `compliance_pci_dss.rs` - PCI DSS compliance
- `compliance_medical.rs` - Medical compliance (HIPAA, IEC 62304)
- `compliance_iso27001.rs` - ISO/IEC 27001:2022 compliance

---

## Coding Standards

### Rust Coding Style

#### Naming Conventions

**Functions**: `snake_case`
```rust
fn create_process(name: &str, priority: ProcessPriority) -> Option<u32> {
    // Implementation
}
```

**Structs**: `PascalCase`
```rust
pub struct ProcessManager {
    process_table: Vec<Option<Process>>,
    next_pid: u32,
}
```

**Enums**: `PascalCase`
```rust
pub enum ProcessState {
    Created,
    Loading,
    Ready,
    Running,
    Blocked,
    Terminated,
}
```

**Constants**: `SCREAMING_SNAKE_CASE`
```rust
pub const MAX_PROCESSES: usize = 1024;
pub const DEFAULT_TIME_QUANTUM_MS: u64 = 5;
```

#### File Organization

**Module Files**: `snake_case.rs`
```
memory.rs
process.rs
interrupt.rs
```

**Test Files**: `snake_case_test.rs`
```
memory_test.rs
process_test.rs
interrupt_test.rs
```

#### Code Formatting

**Indentation**: 4 spaces (no tabs)

**Line Length**: Maximum 100 characters

**Brace Style**: K&R style
```rust
fn example_function(param: &str) -> Result<String, Error> {
    if param.is_empty() {
        return Err(Error::InvalidParameter);
    }
    
    Ok(format!("Processed: {}", param))
}
```

#### Comments

**Documentation Comments**:
```rust
/// Creates a new process with the specified name and priority.
///
/// # Arguments
/// * `name` - The name of the process
/// * `priority` - The priority of the process
///
/// # Returns
/// * `Some(pid)` - The process ID of the created process
/// * `None` - If process table is full
///
/// # Examples
/// ```
/// let mut process_manager = ProcessManager::new();
/// let pid = process_manager.create_process("my_app", ProcessPriority::Normal);
/// ```
///
/// # Errors
/// Returns `None` if process table is full
pub fn create_process(
    &mut self,
    name: &str,
    priority: ProcessPriority
) -> Option<u32> {
    // Implementation
}
```

**Inline Comments**:
```rust
// Allocate a page from the page allocator
let page_addr = match page_allocator.allocate() {
    Some(addr) => addr,
    None => return None,
};
```

#### Error Handling

**Use Result Type**:
```rust
pub fn create_process(
    &mut self,
    name: &str,
    priority: ProcessPriority
) -> Result<u32, ProcessError> {
    // Validate name
    if name.is_empty() {
        return Err(ProcessError::InvalidName);
    }
    
    // Check if process table is full
    if self.process_table.len() >= MAX_PROCESSES {
        return Err(ProcessError::ProcessTableFull);
    }
    
    // Allocate PID
    let pid = self.next_pid;
    self.next_pid += 1;
    
    // Create process
    let process = Process::new(pid, name, priority);
    
    // Add to process table
    self.process_table.push(Some(process));
    
    Ok(pid)
}
```

**Define Custom Error Types**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError {
    ProcessNotFound,
    ProcessAlreadyExists,
    ProcessNotReady,
    ProcessNotRunning,
    ProcessNotStopped,
    InvalidProcessState,
    ProcessTableFull,
    InvalidName,
}
```

#### Testing

**Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_process() {
        let mut process_manager = ProcessManager::new();
        let pid = process_manager.create_process("test", ProcessPriority::Normal);
        assert!(pid.is_some());
        assert_eq!(pid.unwrap(), 2); // First PID is 2
    }
    
    #[test]
    fn test_terminate_process() {
        let mut process_manager = ProcessManager::new();
        let pid = process_manager.create_process("test", ProcessPriority::Normal).unwrap();
        let result = process_manager.terminate_process(pid);
        assert!(result.is_ok());
    }
}
```

**Integration Tests**:
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_process_lifecycle() {
        let mut process_manager = ProcessManager::new();
        
        // Create process
        let pid = process_manager.create_process("test", ProcessPriority::Normal).unwrap();
        
        // Start process
        let result = process_manager.start_process(pid);
        assert!(result.is_ok());
        
        // Terminate process
        let result = process_manager.terminate_process(pid);
        assert!(result.is_ok());
    }
}
```

---

## Testing

### Test Framework

VantisOS uses a custom test framework with `TestResult` and `TestSuite` structures.

#### Test Result Structure
```rust
#[derive(Debug, Clone, Copy)]
pub enum TestResult {
    Pass,
    Fail,
}
```

#### Test Suite Structure
```rust
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<(String, TestResult)>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        TestSuite {
            name: name.to_string(),
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, name: &str, result: TestResult) {
        self.tests.push((name.to_string(), result));
    }

    pub fn print_summary(&self) {
        println!("\n=== {} ===", self.name);
        let passed = self.tests.iter().filter(|(_, r)| *r == TestResult::Pass).count();
        let total = self.tests.len();
        println!("Passed: {}/{}", passed, total);
        
        for (name, result) in &self.tests {
            let status = match result {
                TestResult::Pass => "✓ PASS",
                TestResult::Fail => "✗ FAIL",
            };
            println!("  {}: {}", status, name);
        }
    }
}
```

#### Running Tests

**Run All Tests**:
```bash
cd tests
bash run_all_tests.sh
```

**Run Specific Test Suite**:
```bash
cd tests/integration
cargo test --test integration_tests
```

**Generate Coverage Report**:
```bash
cd tests
bash generate_coverage.sh
```

### Test Coverage

**Current Coverage**: 60% overall

**Coverage by Module**:
- Kernel: ~80%
- Memory: ~70%
- Process: ~75%
- Interrupt: ~70%
- UI: ~60%
- Gestures: ~60%
- Animations: ~60%
- Applications: ~50%

---

## Debugging

### Debugging Techniques

#### Print Debugging

**Print Statements**:
```rust
println!("Debug: Process created with PID: {}", pid);
println!("Debug: Process state: {:?}", process.get_state());
```

**Conditional Debugging**:
```rust
#[cfg(debug_assertions)]
println!("Debug: Entering function: {}", function_name!());
```

#### Logging

**Logging Macros**:
```rust
// Debug logging
debug!("Process created: PID={}, Name={}", pid, name);

// Info logging
info!("Process state changed: PID={}, Old State={:?}, New State={:?}", 
     pid, old_state, new_state);

// Warn logging
warn!("Process table nearly full: {}/{} processes", 
      used, MAX_PROCESSES);

// Error logging
error!("Failed to create process: {:?}", error);
```

#### QEMU Debugging

**Enable Serial Output**:
```bash
qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M \
  -kernel build/arm64_kernel.bin \
  -serial stdio \
  -nographic
```

**Enable GDB Debugging**:
```bash
qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M \
  -kernel build/arm64_kernel.elf \
  -s -S -gdb tcp::1234:1234
```

#### Common Debugging Scenarios

**Kernel Not Booting**:
1. Check multiboot header: `xxd build/arm64_kernel.bin | head -n 1`
2. Verify multiboot magic: Should be `0x1BADB002` (little-endian)
3. Check kernel entry point in linker script
4. Verify boot parameters are correct

**Memory Allocation Failures**:
1. Check page allocator statistics
2. Verify memory map is correct
3. Check for memory leaks
4. Verify page allocator is not corrupted

**Process Creation Failures**:
1. Check process table capacity
2. Verify PID allocation
3. Check process state transitions
4. Verify process manager is initialized

**Interrupt Handling Failures**:
1. Check GIC initialization
2. Check interrupt priorities
3. Check interrupt handler registration
4. Verify interrupt enable/disable

**UI Rendering Issues**:
1. Check UI context initialization
2. Check UI element rendering
3. Check layout updates
4. Check event routing

---

## Contribution Guidelines

### Workflow

1. **Fork Repository**: Fork the VantisOS repository on GitHub
2. **Create Branch**: Create a feature branch from `0.4.1`
3. **Make Changes**: Make your changes following coding standards
4. **Test Thoroughly**: Write unit tests and integration tests
5. **Update Documentation**: Update relevant documentation
6. **Commit Changes**: Write clear, descriptive commit messages
7. Push Branch**: Push your branch to GitHub
8. **Create Pull Request**: Create pull request to `0.4.1`
9. **Code Review**: Respond to code review feedback
10. **Merge**: After approval, your PR will be merged

### Commit Message Format

**Format**:
```
<type>: <subject>

<optional body>

<footer>
<optional footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `style`: Code style changes
- `chore`: Maintenance tasks

**Examples**:
```
feat: Add gesture recognition for pinch and zoom gestures

fix: Fix memory leak in page allocator

docs: Update API reference with new APIs

test: Add stress tests for memory management

refactor: Refactor process manager for better performance

perf: Optimize interrupt handling for lower latency

style: Fix code style issues in memory module

chore: Update deprecated API usage
```

### Code Review Checklist

- [ ] Code follows coding standards
- [ ] Code is well-documented
- [ ] Tests are comprehensive
- - Unit tests
- - Integration tests
- - Performance tests
- [ ] No compiler warnings (except allowed warnings)
- [ ] No security vulnerabilities
- [ ] Performance meets requirements
- [ ] Documentation is updated
- [ ] Commit message is clear and descriptive

### Pull Request Guidelines

**Title Format**:
```
<type>: <brief description>

<detailed description>
```

**Description Template**:
```
## Changes
- List major changes made

## Testing
- Describe testing performed
- Test results

## Related Issues
- Link to related issues

## Checklist
- [ ] Code follows coding standards
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] No security vulnerabilities
- [ ] Performance meets requirements
```

### Issue Reporting

**Issue Template**:
```
**Title**: Brief description of the issue

**Description**:
- Detailed description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment information

**Environment**:
- VantisOS version: 0.6.0
- Device/Platform: ARM64 device
- Build: Debug/Release
- Other relevant environment info

**Logs**:
```
Paste relevant logs here
```

**Attachments**:
- Screenshots
- Log files
- Test results
```

---

## Conclusion

This developer guide provides comprehensive information for developing VantisOS v0.6.0. For more information, see the API Reference and User Guide.

**Status**: ✅ COMPLETE

**Next**: See User Guide for user-facing documentation