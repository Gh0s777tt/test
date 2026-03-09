#!/bin/bash
# VantisOS UEFI Bootloader Build Script
# 
# This script builds the UEFI bootloader for VantisOS.
# Requires nightly Rust with the x86_64-unknown-uefi target.

set -e

echo "========================================"
echo "  VantisOS UEFI Bootloader Build"
echo "========================================"
echo ""

# Check for nightly toolchain
if ! rustup show | grep -q "nightly"; then
    echo "Error: Nightly Rust toolchain required"
    echo "Install with: rustup install nightly"
    exit 1
fi

# Build the bootloader
echo "Building bootloader..."
cargo +nightly build --release \
    --target x86_64-unknown-uefi \
    -Z build-std=core,compiler_builtins,alloc \
    -Z build-std-features=compiler-builtins-mem

# Check if build succeeded
if [ -f "target/x86_64-unknown-uefi/release/vantis_bootloader.efi" ]; then
    echo ""
    echo "Build successful!"
    echo "Output: target/x86_64-unknown-uefi/release/vantis_bootloader.efi"
    ls -la target/x86_64-unknown-uefi/release/vantis_bootloader.efi
else
    echo "Error: Build failed - EFI file not found"
    exit 1
fi

echo ""
echo "To create a bootable USB drive:"
echo "  1. Format a USB drive as FAT32"
echo "  2. Copy vantis_bootloader.efi to /EFI/BOOT/BOOTX64.EFI"
echo "  3. Copy your kernel.elf to /EFI/VANTIS/kernel.elf"
echo "  4. Boot from the USB drive in UEFI mode"