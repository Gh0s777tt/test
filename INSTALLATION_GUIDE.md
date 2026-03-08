# VantisOS Installation Guide

## System Requirements

### Minimum Requirements
- CPU: 64-bit processor (x86_64)
- RAM: 4 GB
- Storage: 20 GB
- Boot: UEFI or Legacy BIOS

### Recommended Requirements
- CPU: Modern 64-bit processor with virtualization support
- RAM: 8 GB or more
- Storage: 50 GB SSD
- Boot: UEFI

## Installation Methods

### Method 1: ISO Installation

1. Download the latest ISO from [Releases](https://github.com/vantisCorp/VantisOS/releases/latest)
2. Create bootable USB:
   ```bash
   # Linux
   dd if=VantisOS-1.5.0.iso of=/dev/sdX bs=4M status=progress && sync
   
   # Windows (use Rufus or balenaEtcher)
   # macOS (use balenaEtcher)
   ```
3. Boot from USB
4. Follow the graphical installer

### Method 2: Virtual Machine

1. Download ISO
2. Create new VM in VirtualBox/VMware/QEMU
3. Mount ISO and boot
4. Follow installation steps

## Post-Installation

1. Update the system:
   ```bash
   vantis update
   ```
2. Install additional packages:
   ```bash
   vantis install <package>
   ```

## Troubleshooting

See [TROUBLESHOOTING_GUIDE.md](TROUBLESHOOTING_GUIDE.md) for common issues.