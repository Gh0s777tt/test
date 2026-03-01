# VantisOS v0.6.0 Migration Guide

**Version**: 0.6.0  
**From**: v0.5.0  
**Date**: March 1, 2025

---

## Overview

This guide helps you migrate from VantisOS v0.5.0 to v0.6.0 "Mobile Ready". The major change in v0.6.0 is the addition of ARM64 architecture support, which introduces several breaking changes and new features.

---

## Major Changes

### Architecture Changes

#### From x86_64 to ARM64
- **v0.5.0**: x86_64 only
- **v0.6.0**: ARM64 (ARMv8-A) support added
- **Impact**: Applications need to be recompiled for ARM64
- **Action**: Update build scripts to target ARM64

#### Boot Parameters
- **v0.5.0**: Multiboot header with x86_64 parameters
- **v0.6.0**: ARM64 boot parameters (dtb_ptr, dtb_size, x0, x1, x2, x3)
- **Impact**: Bootloader must pass Device Tree Blob (DTB) to kernel
- **Action**: Update bootloader to pass DTB parameters

#### Memory Layout
- **v0.5.0**: Kernel at 0x100000 (1MB)
- **v0.6.0**: Kernel at 0x40000000 (1GB)
- **Impact**: Memory addresses changed
- **Action**: Update any hardcoded memory addresses

#### Interrupt Handling
- **v0.5.0**: x86_64 PIC/APIC
- **v0.6.0**: ARM64 GIC (Generic Interrupt Controller)
- **Impact**: Interrupt handling completely different
- **Action**: Update interrupt handlers for GIC

---

## API Changes

### New ARM64-Specific APIs

#### Boot API
```rust
// v0.6.0 ARM64 kernel entry point
#[no_mangle]
pub extern "C" fn kernel_entry(dtb_ptr: u64, dtb_size: u64, x0: u64, x1: u64, x2: u64, x3: u64) -> ! {
    // ARM64-specific boot code
}
```

#### Memory API
```rust
// v0.6.0 ARM64 page allocator
let page_allocator = PageAllocator::new(0x40000000, 0x80000000);

// v0.6.0 ARM64 heap allocator
let heap_allocator = HeapAllocator::new(0x40980000, 0x41980000);
```

#### Interrupt API
```rust
// v0.6.0 ARM64 GIC
let gic = GIC::new();
gic.enable_interrupt(IRQ_TIMER);
```

### Deprecated x86_64 APIs

The following x86_64-specific APIs are not available on ARM64:
- `x86_64::interrupts::IDT`
- `x86_64::interrupts::PIC`
- `x86_64::interrupts::APIC`
- `x86_64::paging::PageTable`
- `x86_64::paging::PageDirectory`

---

## Configuration Changes

### Device Tree Configuration

#### New Requirement
v0.6.0 requires Device Tree Blob (DTB) configuration for ARM64 platforms.

#### Example DTB
```dts
/dts-v1/;

/ {
    model = "VantisOS ARM64 Board";
    compatible = "vantisos,arm64";

    memory {
        device_type = "memory";
        reg = <0x00000000 0x40000000 0x00000000 0x80000000>;
    };

    chosen {
        bootargs = "console=ttyAMA0,115200";
    };

    uart@09000000 {
        compatible = "arm,pl011";
        reg = <0x09000000 0x1000>;
        interrupts = <1 0>;
        clock-frequency = <24000000>;
    };
};
```

### Bootloader Configuration

#### ARM64 Bootloader
v0.6.0 requires ARM64-specific bootloader (e.g., U-Boot, GRUB for ARM64).

#### Example U-Boot Configuration
```
setenv bootargs console=ttyAMA0,115200
setenv fdt_addr 0x40000000
setenv kernel_addr 0x40800000
load mmc 0:1 ${fdt_addr} vantisos.dtb
load mmc 0:1 ${kernel_addr} vantisos.bin
bootz ${kernel_addr} - ${fdt_addr}
```

---

## Build System Changes

### Toolchain Changes

#### New Requirements
- **Rust Target**: `aarch64-unknown-none`
- **Cross-Compiler**: `aarch64-linux-gnu-gcc`
- **Linker**: `aarch64-linux-gnu-ld`
- **Objcopy**: `aarch64-linux-gnu-objcopy`

#### Installation
```bash
# Install ARM64 Rust target
rustup target add aarch64-unknown-none

# Install ARM64 toolchain
apt-get install gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

### Build Script Changes

#### v0.5.0 Build Script
```bash
rustc --target x86_64-unknown-none \
      --crate-type bin \
      -C link-arg=-nostartfiles \
      -C link-arg=-Tlinker.ld \
      src/main.rs
```

#### v0.6.0 Build Script
```bash
rustc --target aarch64-unknown-none \
      --crate-type lib \
      -C link-arg=-nostartfiles \
      -C link-arg=-Tlinker.ld \
      src/main.rs

aarch64-linux-gnu-ld -T linker.ld -o kernel.elf build/*.o
aarch64-linux-gnu-objcopy -O binary kernel.elf kernel.bin
```

---

## Application Migration

### Recompiling Applications

#### Step 1: Update Build Configuration
```toml
# Cargo.toml
[dependencies]
vantisos = "0.6.0"

[build-dependencies]
vantisos-build = "0.6.0"
```

#### Step 2: Update Code for ARM64
```rust
// Update any architecture-specific code
#[cfg(target_arch = "aarch64")]
fn arm64_specific_code() {
    // ARM64-specific implementation
}

#[cfg(target_arch = "x86_64")]
fn x86_64_specific_code() {
    // x86_64-specific implementation
}
```

#### Step 3: Rebuild for ARM64
```bash
cargo build --target aarch64-unknown-none
```

### API Compatibility

#### Compatible APIs
The following APIs are compatible across architectures:
- Memory allocation (page allocator, heap allocator)
- Process management
- Thread management
- File system operations
- System calls (with ARM64 ABI)

#### Incompatible APIs
The following APIs are architecture-specific:
- Boot parameters
- Interrupt handling
- Memory layout
- Device drivers

---

## Testing Changes

### New ARM64 Tests

v0.6.0 includes ARM64-specific tests:
- ARM64 boot tests
- ARM64 memory management tests
- ARM64 interrupt handling tests
- ARM64 driver tests

### Running Tests

#### Run All Tests
```bash
cargo test --target aarch64-unknown-none
```

#### Run ARM64-Specific Tests
```bash
cargo test --target aarch64-unknown-none --test arm64_tests
```

#### Run Compatibility Tests
```bash
cargo test --target aarch64-unknown-none --test compatibility_tests
```

---

## Troubleshooting

### Common Issues

#### Issue 1: Boot Failure
**Symptom**: Kernel doesn't boot on ARM64 platform

**Solution**:
1. Check DTB configuration
2. Verify bootloader passes correct parameters
3. Check memory layout matches DTB
4. Verify UART console configuration

#### Issue 2: Build Errors
**Symptom**: Build fails with linker errors

**Solution**:
1. Verify ARM64 toolchain installed
2. Check linker script for ARM64 memory layout
3. Verify target is `aarch64-unknown-none`
4. Check for architecture-specific code

#### Issue 3: Runtime Errors
**Symptom**: Kernel crashes at runtime

**Solution**:
1. Check for x86_64-specific code
2. Verify memory addresses are correct for ARM64
3. Check interrupt handlers for GIC
4. Verify device drivers are ARM64-compatible

#### Issue 4: Performance Issues
**Symptom**: System slower than expected

**Solution**:
1. Check for unoptimized code
2. Verify cache management enabled
3. Check for unnecessary memory copies
4. Verify interrupt handling is efficient

---

## Migration Checklist

### Pre-Migration
- [ ] Backup current v0.5.0 installation
- [ ] Review breaking changes
- [ ] Update build scripts for ARM64
- [ ] Install ARM64 toolchain
- [ ] Prepare DTB configuration

### Migration
- [ ] Update bootloader for ARM64
- [ ] Update kernel configuration
- [ ] Recompile applications for ARM64
- [ ] Update device drivers
- [ ] Update interrupt handlers

### Post-Migration
- [ ] Test boot process
- [ ] Test memory management
- [ ] Test interrupt handling
- [ ] Test device drivers
- [ ] Test applications
- [ ] Performance testing
- [ ] Security testing

---

## Support

### Documentation
- **Architecture Guide**: `docs/v0.6.0/ARCHITECTURE.md`
- **API Reference**: `docs/v0.6.0/API_REFERENCE.md`
- **User Guide**: `docs/v0.6.0/USER_GUIDE.md`
- **Developer Guide**: `docs/v0.6.0/DEVELOPER_GUIDE.md`

### Community
- **GitHub Issues**: https://github.com/vantisCorp/VantisOS/issues
- **GitHub Discussions**: https://github.com/vantisCorp/VantisOS/discussions
- **Discord**: https://discord.gg/vantisos

### Contact
- **Email**: support@vantisos.org
- **Twitter**: @VantisOS

---

## Additional Resources

### ARM64 Resources
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)
- [ARM64 Boot Process](https://www.kernel.org/doc/Documentation/arm64/booting.txt)
- [Device Tree Specification](https://www.devicetree.org/)

### VantisOS Resources
- [VantisOS Website](https://www.vantisos.org)
- [VantisOS GitHub](https://github.com/vantisCorp/VantisOS)
- [VantisOS Documentation](https://docs.vantisos.org)

---

**End of Migration Guide**