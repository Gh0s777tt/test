#!/bin/bash
# Build script for VantisOS v0.5.0 - Advanced Kernel

set -e

echo "Building VantisOS v0.5.0 - Advanced Kernel..."
echo "=========================================="

# Step 1: Compile as object file
echo "Step 1: Compiling as object file..."
rustc --target x86_64-unknown-none \
     --edition 2021 \
     --crate-type lib \
     --emit=obj \
     -C opt-level=z \
     -C codegen-units=1 \
     -C lto=fat \
     -C panic=abort \
     -o build/kernel.o \
     src/verified/v0.5.0_kernel/main.rs

echo "Object file created: build/kernel.o"

# Step 2: Link to ELF
echo "Step 2: Linking to ELF..."
ld -m elf_x86_64 \
   -T src/verified/v0.5.0_kernel/linker.ld \
   -o build/kernel.elf \
   build/kernel.o

echo "ELF file created: build/kernel.elf"

# Step 3: Strip ELF
echo "Step 3: Stripping ELF..."
strip build/kernel.elf -o build/kernel_stripped.elf

echo "Stripped ELF created: build/kernel_stripped.elf"

# Step 4: Convert to raw binary
echo "Step 4: Converting to raw binary..."
objcopy -O binary build/kernel_stripped.elf build/kernel.bin

echo "Binary file created: build/kernel.bin"

# Step 5: Verify multiboot header
echo "Step 5: Verifying multiboot header..."
MAGIC=$(xxd -l 4 -p build/kernel.bin)
echo "Multiboot magic: 0x$MAGIC"

if [ "$MAGIC" = "02b0ad1b" ]; then
    echo "✓ Multiboot header verified!"
else
    echo "✗ Multiboot header not found!"
    exit 1
fi

# Step 6: Display file sizes
echo ""
echo "File sizes:"
ls -lh build/kernel.o build/kernel.elf build/kernel_stripped.elf build/kernel.bin

echo ""
echo "Build complete!"
echo "=========================================="