#!/bin/bash

# VantisOS v0.5.0 - VGA Console Build Script
# This script compiles the VGA console kernel

set -e

echo "Building VantisOS v0.5.0 VGA Console Kernel..."

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf build
mkdir -p build

# Step 1: Compile as object file
echo "Step 1: Compiling as object file..."
rustc --target x86_64-unknown-none --crate-type bin --edition 2021 \
    -C opt-level=z -C codegen-units=1 -C panic=abort \
    --emit=obj \
    --cfg 'feature="platform-generic"' \
    src/verified/v0.5.0_kernel/main.rs -o build/kernel.o 2>&1 | grep -v "warning:"

# Step 2: Link to ELF
echo "Step 2: Linking to ELF..."
ld -m elf_x86_64 -T src/verified/v0.5.0_kernel/linker.ld -o build/kernel.elf build/kernel.o

# Step 3: Convert to raw binary
echo "Step 3: Converting to raw binary..."
objcopy -O binary build/kernel.elf build/kernel.bin

# Step 4: Verify multiboot header
echo "Step 4: Verifying multiboot header..."
echo "First 16 bytes of kernel.bin:"
od -A x -t x1z -N 16 build/kernel.bin

# Display build results
echo ""
echo "=========================================="
echo "Build Complete"
echo "=========================================="
echo "Object file: $(ls -lh build/kernel.o | awk '{print $5}')"
echo "ELF file: $(ls -lh build/kernel.elf | awk '{print $5}')"
echo "Binary file: $(ls -lh build/kernel.bin | awk '{print $5}')"
echo ""
echo "To create ISO:"
echo "  bash create_vga_console_iso.sh"
echo ""