#!/bin/bash

# VantisOS v0.5.0 - VGA Console Test ISO Creation Script
# This script creates a bootable ISO with the VGA console kernel

set -e

echo "Creating VantisOS v0.5.0 VGA Console Test ISO..."

# Create ISO directory structure
ISO_DIR="vga_console_iso"
mkdir -p "$ISO_DIR/boot/grub"

# Copy kernel ELF file
echo "Copying kernel ELF file..."
cp build/kernel.elf "$ISO_DIR/boot/vantisos-kernel.elf"

# Create GRUB 2 configuration
echo "Creating GRUB 2 configuration..."
cat > "$ISO_DIR/boot/grub/grub.cfg" << 'EOF'
set timeout=2
set default=0

menuentry "VantisOS 0.5.0 - VGA Console Test" {
    multiboot /boot/vantisos-kernel.elf
    boot
}

menuentry "Reboot" {
    reboot
}

menuentry "Poweroff" {
    halt
}
EOF

# Create ISO using grub-mkrescue
echo "Creating ISO with GRUB 2..."
grub-mkrescue -o vantisos-0.5.0-vga-console.iso "$ISO_DIR" 2>&1 | grep -v "xorriso"

# Clean up
echo "Cleaning up..."
rm -rf "$ISO_DIR"

# Display ISO information
echo ""
echo "=========================================="
echo "VantisOS v0.5.0 VGA Console Test ISO Created"
echo "=========================================="
echo "ISO file: vantisos-0.5.0-vga-console.iso"
ls -lh vantisos-0.5.0-vga-console.iso
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -cdrom vantisos-0.5.0-vga-console.iso -m 512M"
echo ""