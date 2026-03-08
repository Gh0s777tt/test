#!/bin/bash

# Build script for VantisOS v0.6.0 ARM64 kernel

set -e

echo "Building VantisOS v0.6.0 ARM64 kernel..."

# Check if ARM64 target is installed
if ! rustup target list | grep -q "aarch64-unknown-none"; then
    echo "Installing ARM64 target..."
    rustup target add aarch64-unknown-none
fi

# Create build directory
mkdir -p build

# Step 1: Compile as object file
echo "Step 1: Compiling as object file..."
rustc --target aarch64-unknown-none \
     --crate-type lib \
     --emit=obj \
     -C opt-level=z \
     -C lto=fat \
     -C panic=abort \
     -C codegen-units=1 \
     lib.rs \
     -o build/arm64_kernel.o

# Step 2: Link to ELF
echo "Step 2: Linking to ELF..."
aarch64-linux-gnu-ld -T arm64/linker.ld \
   -o build/arm64_kernel.elf \
   build/arm64_kernel.o

# Step 3: Convert to raw binary
echo "Step 3: Converting to raw binary..."
aarch64-linux-gnu-objcopy -O binary \
        build/arm64_kernel.elf \
        build/arm64_kernel.bin

# Step 4: Strip ELF
echo "Step 4: Stripping ELF..."
aarch64-linux-gnu-strip build/arm64_kernel.elf

# Display build results
echo ""
echo "Build complete!"
echo ""
echo "Build results:"
echo "  Object file: $(du -h build/arm64_kernel.o | cut -f1)"
echo "  ELF file:    $(du -h build/arm64_kernel.elf | cut -f1)"
echo "  Binary file: $(du -h build/arm64_kernel.bin | cut -f1)"
echo ""

# Verify multiboot header (if applicable)
if [ -f "build/arm64_kernel.bin" ]; then
    echo "Binary header:"
    xxd -l 12 build/arm64_kernel.bin
    echo ""
fi

echo "ARM64 kernel ready for testing!"
echo ""
echo "To test in QEMU ARM64:"
echo "  qemu-system-aarch64 -M virt -cpu cortex-a57 -m 512M -kernel build/arm64_kernel.bin -serial stdio"
