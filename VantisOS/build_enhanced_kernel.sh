#!/bin/bash
# VantisOS v0.5.0 - Enhanced Kernel Build Script
# Phase 2, Day 6

set -e

echo "Building VantisOS v0.5.0 Enhanced Kernel..."

# Create build directory
mkdir -p build

# Step 1: Compile as object file
echo "Step 1: Compiling as object file..."
rustc --target x86_64-unknown-none \
     -O \
     -C opt-level=z \
     --emit=obj \
     --crate-type bin \
     kernel/enhanced_entry.rs \
     -o build/kernel_enhanced.o

# Step 2: Link to ELF
echo "Step 2: Linking to ELF..."
ld -m elf_x86_64 \
   -T kernel/linker.ld \
   build/kernel_enhanced.o \
   -o build/kernel_enhanced.elf

# Step 3: Convert to raw binary
echo "Step 3: Converting to raw binary..."
objcopy -O binary \
        build/kernel_enhanced.elf \
        build/kernel_enhanced.bin

# Step 4: Verify multiboot header
echo "Step 4: Verifying multiboot header..."
echo "First 12 bytes (multiboot header):"
xxd -l 12 build/kernel_enhanced.bin

# Step 5: Display file sizes
echo ""
echo "File sizes:"
ls -lh build/kernel_enhanced.o build/kernel_enhanced.elf build/kernel_enhanced.bin

echo ""
echo "Build complete!"
echo "Kernel binary: build/kernel_enhanced.bin"