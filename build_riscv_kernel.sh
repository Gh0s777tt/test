#!/bin/bash
# Build RISC-V Kernel for VantisOS v0.7.0

set -e

echo "Building VantisOS v0.7.0 - RISC-V Kernel..."

# Set RISC-V target
export RUST_TARGET=riscv64gc-unknown-none-elf

# Set RISC-V toolchain
export RUSTFLAGS="-C target-cpu=generic-rv64 -C target-feature=+m,+a,+f,+d,+c"

# Build kernel
echo "Building kernel..."
cargo build --target $RUST_TARGET --release -p vantisos-kernel

# Create ELF file
echo "Creating ELF file..."
objcopy -O binary target/$RUST_TARGET/release/vantisos-kernel kernel.bin

# Create raw binary
echo "Creating raw binary..."
objcopy -O elf64-littleriscv target/$RUST_TARGET/release/vantisos-kernel kernel.elf

# Print size information
echo "Kernel size information:"
ls -lh kernel.bin kernel.elf

echo "Build complete!"
echo "Kernel binary: kernel.bin"
echo "Kernel ELF: kernel.elf"