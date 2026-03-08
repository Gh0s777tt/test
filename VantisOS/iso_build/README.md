# VantisOS Bootable ISO

This directory contains the complete build system for creating a bootable VantisOS ISO image.

## Version
- **Version**: 1.5.0
- **Codename**: Quantum Ready

## Quick Start

### Build the ISO

```bash
chmod +x build.sh
./build.sh
```

### Run in QEMU

```bash
qemu-system-x86_64 -cdrom VantisOS-1.5.0.iso -m 512M
```

### Write to USB

```bash
sudo dd if=VantisOS-1.5.0.iso of=/dev/sdX bs=4M status=progress && sync
```

## Directory Structure

```
VantisOS_build/
├── build.sh           # Main build script
├── Makefile           # Alternative build system
├── README.md          # This file
├── kernel/            # Kernel source code
│   ├── Cargo.toml     # Rust package configuration
│   ├── linker.ld      # Linker script
│   └── src/           # Kernel source
│       ├── lib.rs     # Main kernel entry
│       ├── arch/      # Architecture-specific (x86_64)
│       ├── drivers/   # Hardware drivers (VGA, etc.)
│       ├── memory/    # Memory management
│       ├── process/   # Process management
│       ├── interrupts/# Interrupt handling
│       ├── syscall/   # System calls
│       ├── fs/        # Virtual filesystem
│       ├── ipc/       # Inter-process communication
│       ├── security/  # Security subsystem
│       └── quantum/   # Quantum computing module
├── initramfs/         # Initial RAM filesystem
│   ├── init           # Init script
│   ├── bin/           # User binaries
│   ├── sbin/          # System binaries
│   ├── etc/           # Configuration files
│   └── usr/           # User programs
│       └── bin/
│           └── vantis-shell  # Desktop shell
└── iso/               # ISO output directory
    └── boot/
        └── grub/
            └── grub.cfg
```

## Features

### Kernel
- x86_64 architecture support
- Preemptive multitasking
- Virtual memory with paging
- VGA text mode driver
- Serial port debugging
- Quantum computing simulation

### Desktop
- Modern TUI desktop shell
- System monitor
- File manager
- Settings panel
- Quantum simulator interface

### Installer
- User-friendly installation wizard
- Automatic partitioning
- User configuration
- Desktop environment options

## Requirements

### Build Requirements
- Rust nightly (for kernel compilation)
- NASM (assembly compilation)
- grub-mkrescue or xorriso (ISO creation)
- cpio, gzip (initramfs creation)

### Runtime Requirements
- x86_64 processor
- 512MB RAM minimum (2GB recommended)
- 20GB disk space for installation

## Boot Options

When booting the ISO, you can use these kernel parameters:

- `live` - Boot into live session
- `install` - Start installation wizard
- `safe` - Safe mode (minimal drivers)
- `debug` - Enable kernel debugging

## License

MIT License - See LICENSE file for details.

## Links

- Website: https://vantisos.org
- Documentation: https://docs.vantisos.org
- Source: https://github.com/vantisos/vantis