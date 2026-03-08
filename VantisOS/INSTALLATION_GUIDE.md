# VantisOS Installation Guide

Complete installation guide for VantisOS - A formally verified, mathematically proven operating system built with Rust and Verus.

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installation Methods](#installation-methods)
3. [Building from Source](#building-from-source)
4. [Creating Bootable Media](#creating-bootable-media)
5. [Installing VantisOS](#installing-vantisos)
6. [Post-Installation](#post-installation)
7. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| CPU | 64-bit x86 processor (x86_64) |
| RAM | 2 GB minimum, 4 GB recommended |
| Storage | 10 GB minimum, 20 GB recommended |
| Firmware | UEFI boot support |
| Display | VESA-compatible graphics |

### Recommended Requirements

| Component | Requirement |
|-----------|-------------|
| CPU | Modern 64-bit x86 processor with virtualization support |
| RAM | 8 GB or more |
| Storage | 50 GB SSD |
| Firmware | UEFI 2.3.1+ with Secure Boot support |
| Network | Ethernet or Wi-Fi adapter |

### Supported Hardware

- **Processors**: Intel Core series (6th gen+), AMD Ryzen series
- **Chipsets**: Intel 100-series+, AMD B350/X370+
- **Graphics**: Integrated graphics, basic VESA support
- **Storage**: SATA, NVMe, USB storage devices
- **Network**: Intel, Realtek, Broadcom adapters

---

## Installation Methods

### Method 1: Download Pre-built ISO

Download the latest release from GitHub:

```bash
# Download latest ISO
wget https://github.com/vantisCorp/VantisOS/releases/latest/download/VantisOS-x86_64.iso

# Verify checksum
sha256sum VantisOS-x86_64.iso
```

### Method 2: Build from Source

For developers and advanced users:

```bash
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS
./scripts/build_iso.sh
```

### Method 3: Cloud Images

Deploy VantisOS in cloud environments:

```bash
# AWS
aws ec2 run-instances --image-id ami-vantisos-latest

# Azure
az vm create --image vantisos:latest

# GCP
gcloud compute instances create --image=vantisos-latest
```

---

## Building from Source

### Prerequisites

Install required build dependencies:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential git curl wget

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
rustup component add rust-src rustfmt clippy

# Additional tools
cargo install cargo-xbuild bootimage
```

### Build Steps

```bash
# Clone repository
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS

# Build kernel
./scripts/build_kernel.sh

# Build ISO
./scripts/build_iso.sh

# Output: target/x86_64-vantisos/release/bootimage-VantisOS.iso
```

### Build Options

```bash
# Debug build (with symbols)
./scripts/build_kernel.sh --debug

# Release build (optimized)
./scripts/build_kernel.sh --release

# Build all components
./scripts/build_all.sh

# Run tests
./scripts/test_all.sh
```

---

## Creating Bootable Media

### USB Flash Drive

Create a bootable USB drive:

```bash
# Using dd (Linux/macOS)
sudo dd if=VantisOS-x86_64.iso of=/dev/sdX bs=4M status=progress && sync

# Using Etcher (cross-platform)
# Download from: https://www.balena.io/etcher/

# Using our automated script
./scripts/create_live_usb.sh /dev/sdX
```

### Virtual Machine

Run VantisOS in a VM:

```bash
# QEMU
./scripts/run_qemu.sh

# VirtualBox
VBoxManage createvm --name VantisOS --register
VBoxManage storagectl VantisOS --name "IDE" --add ide
VBoxManage storageattach VantisOS --storagectl "IDE" --port 0 --device 0 --type dvddrive --medium VantisOS-x86_64.iso
VBoxManage startvm VantisOS

# VMware
# Create new VM and attach ISO as CD/DVD
```

---

## Installing VantisOS

### Live Environment

1. Boot from installation media
2. Select "Boot VantisOS Live" from boot menu
3. The live environment allows testing before installation

### Installation Process

1. **Boot the Installer**
   - Insert bootable media
   - Boot from USB/ISO (F12 or BIOS menu)
   - Select "Install VantisOS"

2. **Language Selection**
   - Select your preferred language
   - Supported: English, Polish, German, Japanese, Chinese, Spanish, French

3. **Disk Configuration**
   - Automatic (use entire disk)
   - Manual (custom partitioning)
   - Recommended: 20+ GB for root partition

4. **User Setup**
   - Create admin user
   - Set hostname
   - Configure network (optional)

5. **Installation**
   - Confirm settings
   - Installation progress
   - Typically 5-15 minutes

### Automated Installation

For unattended installation:

```bash
# Create kickstart file
cat > vantisos-ks.cfg << EOF
# VantisOS Kickstart Configuration
lang en_US.UTF-8
keyboard us
timezone UTC
rootpw --plaintext vantisoroot
user --name=vantis --password=vantis --plaintext
bootloader --location=mbr
clearpart --all --initlabel
part / --fstype=ext4 --size=20000 --grow
EOF

# Boot with kickstart
# Append to kernel: ks=http://server/vantisos-ks.cfg
```

---

## Post-Installation

### First Boot

After installation completes:

1. Remove installation media
2. Reboot system
3. Log in with created credentials
4. Complete initial setup wizard

### System Updates

Keep VantisOS up to date:

```bash
# Check for updates
vantis-update check

# Apply updates
vantis-update apply

# Full system upgrade
vantis-upgrade
```

### Essential Commands

```bash
# System information
vantis-info

# Hardware detection
vantis-hardware detect

# Install packages
vantis-pkg install <package>

# Service management
vantis-service start|stop|status <service>
```

### Network Configuration

```bash
# List interfaces
ip link show

# Configure WiFi
vantis-wifi connect <SSID>

# Static IP
vantis-network configure --static --ip 192.168.1.100 --gateway 192.168.1.1
```

---

## Troubleshooting

### Boot Issues

**Problem**: System doesn't boot from USB

**Solutions**:
- Check BIOS boot order
- Disable Secure Boot temporarily
- Verify ISO integrity: `sha256sum VantisOS.iso`
- Try different USB port

### Installation Errors

**Problem**: Disk partitioning fails

**Solutions**:
- Use manual partitioning
- Check disk health: `smartctl -a /dev/sda`
- Try formatting first: `wipefs -a /dev/sda`

**Problem**: Installation hangs

**Solutions**:
- Check system memory
- Verify installation media
- Try text-mode installer: boot parameter `text`

### Post-Install Issues

**Problem**: No network after installation

**Solutions**:
```bash
# Check driver status
lspci -v | grep -i network
dmesg | grep -i eth

# Manual configuration
ip link set eth0 up
dhclient eth0
```

**Problem**: Graphics not working properly

**Solutions**:
- Update graphics firmware
- Use VESA fallback: `vantis-gpu --vesa`
- Check kernel logs: `dmesg | grep -i gpu`

### Getting Help

- **Documentation**: https://docs.vantis.os
- **GitHub Issues**: https://github.com/vantisCorp/VantisOS/issues
- **Discord**: https://discord.gg/vantisos
- **Email**: support@vantis.os

---

## Next Steps

- Read the [User Guide](./USER_GUIDE.md)
- Explore [Applications](./APPLICATIONS.md)
- Configure [Desktop Environment](./DESKTOP_GUIDE.md)
- Join the [Community](https://discord.gg/vantisos)

---

*Last updated: March 2025 | VantisOS v1.4.0*