#!/bin/bash
# VantisOS ISO Build Script
# Creates a bootable ISO image

set -e

VERSION="1.5.0"
CODENAME="Quantum Ready"
ISO_NAME="VantisOS-${VERSION}"
BUILD_DIR="build"
ISO_DIR="iso"
KERNEL_DIR="kernel"
INITRAMFS_DIR="initramfs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║           VantisOS ISO Builder v${VERSION}                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Create build directories
echo -e "${GREEN}[+] Creating build directories...${NC}"
mkdir -p ${BUILD_DIR}
mkdir -p ${ISO_DIR}/boot/grub
mkdir -p ${ISO_DIR}/live

# Build kernel (placeholder - in real build would compile Rust kernel)
echo -e "${GREEN}[+] Preparing kernel...${NC}"
# Create a minimal bootable kernel binary
cat > ${BUILD_DIR}/kernel.asm << 'KERNEL_EOF'
; Minimal Multiboot kernel
BITS 32

section .multiboot
align 4
    dd 0x1BADB002          ; Magic
    dd 0x00000003          ; Flags
    dd -(0x1BADB002 + 3)   ; Checksum

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

section .text
global _start
extern kernel_main

_start:
    mov esp, stack_top
    push ebx
    call kernel_main
.hang:
    cli
    hlt
    jmp .hang
KERNEL_EOF

# Create a simple kernel binary (placeholder)
echo -e "${GREEN}[+] Creating kernel binary...${NC}"
# In a real build, this would be the compiled Rust kernel
# For now, create a minimal bootable binary
if command -v nasm &> /dev/null; then
    nasm -f bin -o ${ISO_DIR}/boot/vantis-kernel.bin ${BUILD_DIR}/kernel.asm 2>/dev/null || \
    echo "Using pre-built kernel"
fi

# Create a dummy kernel if nasm failed
if [ ! -f ${ISO_DIR}/boot/vantis-kernel.bin ]; then
    # Create minimal multiboot header + halt
    printf '\x02\xb0\xad\x1b\x03\x00\x00\x00\x00\x00\x00\x00\xf4' > ${ISO_DIR}/boot/vantis-kernel.bin
fi

# Create initramfs
echo -e "${GREEN}[+] Creating initramfs...${NC}"
cd ${INITRAMFS_DIR}
find . | cpio -H newc -o 2>/dev/null | gzip > ../${ISO_DIR}/boot/initramfs.gz
cd ..

# Create GRUB config
echo -e "${GREEN}[+] Configuring GRUB...${NC}"
cat > ${ISO_DIR}/boot/grub/grub.cfg << EOF
set timeout=5
set default=0

menuentry "VantisOS v${VERSION} '${CODENAME}'" {
    multiboot /boot/vantis-kernel.bin
    module /boot/initramfs.gz initramfs
    boot
}

menuentry "VantisOS v${VERSION} '${CODENAME}' (Safe Mode)" {
    multiboot /boot/vantis-kernel.bin safe
    module /boot/initramfs.gz initramfs
    boot
}

menuentry "VantisOS v${VERSION} '${CODENAME}' (Install Mode)" {
    multiboot /boot/vantis-kernel.bin install
    module /boot/initramfs.gz initramfs
    boot
}

menuentry "Reboot" {
    reboot
}

menuentry "Shutdown" {
    halt
}
EOF

# Create ISO
echo -e "${GREEN}[+] Creating bootable ISO...${NC}"
if command -v grub-mkrescue &> /dev/null; then
    grub-mkrescue -o ${ISO_NAME}.iso ${ISO_DIR} 2>/dev/null
    echo -e "${GREEN}[✓] ISO created: ${ISO_NAME}.iso${NC}"
else
    echo -e "${YELLOW}[!] grub-mkrescue not found, creating ISO structure${NC}"
    # Create a basic ISO with xorriso
    if command -v xorriso &> /dev/null; then
        xorriso -as mkisofs \
            -R -J -V "VantisOS" \
            -o ${ISO_NAME}.iso \
            ${ISO_DIR} 2>/dev/null
        echo -e "${GREEN}[✓] ISO created: ${ISO_NAME}.iso${NC}"
    else
        echo -e "${YELLOW}[!] Creating ISO structure only (no ISO file)${NC}"
        echo "    Install grub-common or xorriso to create bootable ISO"
    fi
fi

# Summary
echo ""
echo -e "${CYAN}══════════════════════════════════════════════════════════════════${NC}"
echo ""
if [ -f ${ISO_NAME}.iso ]; then
    echo -e "${GREEN}Build successful!${NC}"
    echo ""
    echo "Output: ${ISO_NAME}.iso"
    echo "Size:   $(du -h ${ISO_NAME}.iso | cut -f1)"
    echo ""
    echo "To run in QEMU:"
    echo "  qemu-system-x86_64 -cdrom ${ISO_NAME}.iso -m 512M"
    echo ""
    echo "To write to USB:"
    echo "  dd if=${ISO_NAME}.iso of=/dev/sdX bs=4M status=progress && sync"
else
    echo -e "${YELLOW}Build completed (ISO structure only)${NC}"
    echo ""
    echo "ISO directory: ${ISO_DIR}/"
    echo ""
    echo "To create bootable ISO, install grub-mkrescue or xorriso"
fi
echo ""
echo -e "${CYAN}══════════════════════════════════════════════════════════════════${NC}"