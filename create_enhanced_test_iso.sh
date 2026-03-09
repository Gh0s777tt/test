#!/bin/bash
# VantisOS v0.5.0 - Enhanced Test ISO Creation Script
# Phase 2, Day 6

set -e

echo "Creating VantisOS v0.5.0 Enhanced Test ISO..."

# Create ISO directory structure
rm -rf iso_enhanced
mkdir -p iso_enhanced/boot/grub

# Copy kernel (use ELF file with multiboot header)
cp build/kernel_enhanced.elf iso_enhanced/boot/vantisos-kernel.elf

# Create GRUB configuration
cat > iso_enhanced/boot/grub/grub.cfg << 'EOF'
set timeout=0
set default=0

menuentry "VantisOS 0.5.0 - Enhanced Kernel Test" {
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

# Create ISO with GRUB 2
echo "Creating ISO with GRUB 2..."
grub-mkrescue -o vantisos-0.5.0-enhanced-test.iso iso_enhanced

echo ""
echo "ISO created successfully!"
echo "ISO file: vantisos-0.5.0-enhanced-test.iso"
ls -lh vantisos-0.5.0-enhanced-test.iso