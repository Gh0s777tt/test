#!/bin/bash
# VantisOS ISO Build Script v1.5.0
# Builds bootable ISO with Rust kernel

set -e

VERSION="1.5.0"
CODENAME="Quantum Ready"
ISO_NAME="VantisOS-${VERSION}.iso"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     VantisOS Build System v${VERSION}                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check for required tools
echo "[CHECK] Verifying build dependencies..."
MISSING=0

for tool in nasm ld grub-mkrescue xorriso; do
    if ! command -v $tool &> /dev/null; then
        echo "  [MISSING] $tool"
        MISSING=1
    else
        echo "  [OK] $tool"
    fi
done

# Check for Rust
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "  [OK] $RUST_VERSION"
else
    echo "  [MISSING] rustc (Rust compiler)"
    MISSING=1
fi

if [ $MISSING -eq 1 ]; then
    echo ""
    echo "[ERROR] Missing required tools. Install with:"
    echo "  apt-get install nasm binutils grub-pc-bin xorriso mtools"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo ""

# Create directories
echo "[BUILD] Creating directory structure..."
mkdir -p iso/boot/grub
mkdir -p build

# Build assembly kernel (fallback for testing)
echo "[BUILD] Building assembly kernel..."
nasm -f elf32 kernel.asm -o build/kernel_asm.o 2>/dev/null || {
    echo "  [WARN] Assembly kernel build failed, using fallback"
}

if [ -f build/kernel_asm.o ]; then
    ld -m elf_i386 -T linker.ld -o build/vantis-kernel-asm.bin build/kernel_asm.o
    echo "  [OK] Assembly kernel: build/vantis-kernel-asm.bin"
fi

# Try to build Rust kernel
echo ""
echo "[BUILD] Attempting Rust kernel build..."

# Check for nightly Rust
RUST_NIGHTLY=$(rustup show 2>/dev/null | grep nightly || true)
if [ -n "$RUST_NIGHTLY" ]; then
    echo "  Using nightly toolchain"
    
    # Build kernel
    cd kernel
    cargo build --release --target x86_64-unknown-none 2>/dev/null || {
        echo "  [WARN] Rust kernel build failed (target not available)"
        echo "  [INFO] Using assembly kernel fallback"
    }
    cd ..
    
    if [ -f kernel/target/x86_64-unknown-none/release/libvantis_kernel.a ]; then
        cp kernel/target/x86_64-unknown-none/release/libvantis_kernel.a build/vantis-kernel.a
        echo "  [OK] Rust kernel built successfully"
    fi
else
    echo "  [WARN] Nightly Rust not available"
    echo "  [INFO] Using assembly kernel fallback"
fi

# Choose kernel to use
if [ -f build/vantis-kernel.a ]; then
    KERNEL_FILE="build/vantis-kernel.a"
    echo ""
    echo "[INFO] Using Rust kernel"
elif [ -f build/vantis-kernel-asm.bin ]; then
    KERNEL_FILE="build/vantis-kernel-asm.bin"
    echo ""
    echo "[INFO] Using assembly kernel"
else
    echo ""
    echo "[ERROR] No kernel available!"
    exit 1
fi

# Copy kernel to ISO
cp $KERNEL_FILE iso/boot/vantis-kernel.bin
echo "[COPY] Kernel → iso/boot/vantis-kernel.bin"

# Create initramfs
echo ""
echo "[BUILD] Creating initramfs..."
if [ -d initramfs ]; then
    cd initramfs
    find . | cpio -H newc -o 2>/dev/null | gzip > ../iso/boot/initramfs.gz
    cd ..
    echo "  [OK] initramfs created"
else
    echo "  [WARN] initramfs directory not found, creating minimal"
    mkdir -p initramfs/{bin,dev,etc,proc,sys,usr/bin}
    echo "VantisOS" > initramfs/etc/hostname
    cd initramfs
    find . | cpio -H newc -o 2>/dev/null | gzip > ../iso/boot/initramfs.gz
    cd ..
fi

# Create GRUB config
echo ""
echo "[BUILD] Creating GRUB configuration..."
cat > iso/boot/grub/grub.cfg << 'GRUBCFG'
set timeout=3
set default=0

menuentry "VantisOS 1.5.0 'Quantum Ready'" {
    multiboot /boot/vantis-kernel.bin
    module /boot/initramfs.gz initramfs
    boot
}

menuentry "VantisOS 1.5.0 (Safe Mode)" {
    multiboot /boot/vantis-kernel.bin safe
    module /boot/initramfs.gz initramfs
    boot
}

menuentry "VantisOS 1.5.0 (Debug Mode)" {
    multiboot /boot/vantis-kernel.bin debug
    module /boot/initramfs.gz initramfs
    boot
}
GRUBCFG

echo "  [OK] GRUB configuration created"

# Build ISO
echo ""
echo "[BUILD] Creating bootable ISO..."
grub-mkrescue -o $ISO_NAME iso 2>/dev/null || {
    echo "[ERROR] Failed to create ISO!"
    exit 1
}

# Get ISO size
ISO_SIZE=$(du -h $ISO_NAME | cut -f1)

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                    BUILD SUCCESSFUL                           ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "  Output: $ISO_NAME"
echo "  Size:   $ISO_SIZE"
echo ""
echo "  Test with: qemu-system-x86_64 -cdrom $ISO_NAME -m 512M -nographic"
echo "  USB write: sudo dd if=$ISO_NAME of=/dev/sdX bs=4M status=progress"
echo ""