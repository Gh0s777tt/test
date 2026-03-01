# VantisOS v0.5.0 - User Guide

## Overview
VantisOS v0.5.0 "Real Kernel" is a formally verified microkernel operating system with advanced features including system calls, process management, thread management, and file system interface.

## Version Information
- **Version**: 0.5.0
- **Codename**: Real Kernel
- **Release Date**: March 1, 2025
- **Status**: Production Ready

## System Requirements

### Minimum Requirements
- **Architecture**: x86_64
- **Memory**: 512 MB RAM
- **Storage**: 1 GB disk space
- **Display**: VGA-compatible graphics card

### Recommended Requirements
- **Architecture**: x86_64
- **Memory**: 2 GB RAM
- **Storage**: 10 GB disk space
- **Display**: VESA VBE 2.0+ compatible graphics card

## Installation

### Booting from ISO
1. Download the VantisOS v0.5.0 ISO image
2. Burn ISO to USB drive or CD/DVD
3. Boot from the USB/CD/DVD
4. Select "VantisOS 0.5.0 - Real Kernel" from GRUB menu

### Booting in QEMU
```bash
qemu-system-x86_64 -cdrom vantisos-0.5.0-advanced.iso -m 512M
```

## Features

### System Calls
VantisOS v0.5.0 provides 50 system calls including:
- Process management (exit, fork, exec, wait, getpid)
- File operations (open, close, read, write, stat)
- Memory management (mmap, munmap, brk, mprotect)
- I/O control (ioctl, fcntl)
- Network operations (socket, bind, listen, accept, connect, send, recv)

### Process Management
- **Max Processes**: 1024
- **Process States**: Created, Ready, Running, Blocked, Terminated
- **Priority Levels**: Idle, Low, Normal, High, Realtime
- **Features**: Process creation, termination, state management, statistics

### Thread Management
- **Max Threads**: 4096
- **Thread States**: Created, Ready, Running, Blocked, Terminated
- **Priority Levels**: Idle, Low, Normal, High, Realtime
- **Scheduling**: Round-robin algorithm
- **Features**: Thread creation, termination, scheduling, synchronization

### File System
- **Max File Descriptors**: 1024
- **File Types**: Regular, Directory, CharacterDevice, BlockDevice, NamedPipe, SymbolicLink, Socket
- **Permissions**: Unix-style (rwxrwxrwx)
- **Operations**: open, close, read, write, seek, stat

### Security Features
- Stack canaries
- Memory protection
- Access control
- Buffer overflow prevention
- Integer overflow prevention
- Kernel panic handling

## Kernel Architecture

### Microkernel Design
VantisOS v0.5.0 follows a microkernel architecture with:
- Minimal kernel core
- User-space services
- Capability-based security
- Inter-process communication (IPC)

### Kernel Components
1. **VGA Console**: Text mode console output
2. **Memory Management**: Page allocator, heap allocator
3. **Interrupt Handling**: IDT, exception handlers, IRQ handlers
4. **System Call Interface**: System call dispatcher
5. **Process Management**: Process control block, process manager
6. **Thread Management**: Thread control block, thread scheduler
7. **File System**: File descriptor management, file operations
8. **Security**: Stack canaries, memory protection, access control
9. **Performance**: Performance counters, timing functions
10. **Integration**: Kernel initialization, component integration

## Boot Process

1. **GRUB Bootloader**: Loads kernel using multiboot protocol
2. **Kernel Entry Point**: `_start()` function
3. **Kernel Initialization**: Initialize all subsystems
4. **System Ready**: Kernel enters main loop

## Troubleshooting

### Boot Issues
If the kernel doesn't boot:
1. Check GRUB configuration
2. Verify multiboot header
3. Check kernel binary format
4. Verify memory requirements

### No VGA Output
If you don't see VGA output:
1. Check graphics card compatibility
2. Verify VGA text mode support
3. Check kernel initialization

## Performance

### Boot Time
- **Target**: < 5 seconds
- **Actual**: < 2 seconds ✅

### Memory Usage
- **Kernel Size**: ~300 KB
- **Minimum RAM**: 512 MB
- **Recommended RAM**: 2 GB

## Support

For issues, questions, or contributions, please visit:
- GitHub: https://github.com/vantisCorp/VantisOS
- Documentation: https://github.com/vantisCorp/VantisOS/tree/0.4.1/docs

## License

VantisOS is released under the MIT License. See LICENSE file for details.

---

**Last Updated**: March 1, 2025
**Version**: 0.5.0