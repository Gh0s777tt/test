#!/bin/bash

# VantisOS v0.5.0 - Simple VGA Test ISO Creation Script

set -e

echo "Creating VantisOS v0.5.0 Simple VGA Test ISO..."

# Clean previous ISO
echo "Cleaning previous ISO..."
rm -f vantisos-0.5.0-simple-vga-test.iso

# Create ISO directory structure
echo "Creating ISO directory structure..."
mkdir -p iso/boot/grub

# Copy kernel to ISO
echo "Copying kernel to ISO..."
cp build/kernel.bin iso/boot/vantisos-kernel.bin

# Create GRUB configuration
echo "Creating GRUB configuration..."
cat > iso/boot/grub/grub.cfg << 'EOF'
menuentry "VantisOS 0.5.0 - Simple VGA Test" {
    multiboot /boot/vantisos-kernel.bin
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
echo "Creating ISO using grub-mkrescue..."
grub-mkrescue -o vantisos-0.5.0-simple-vga-test.iso iso

# Display results
echo ""
echo "=========================================="
echo "ISO Creation Complete"
echo "=========================================="
echo "ISO file: $(ls -lh vantisos-0.5.0-simple-vga-test.iso | awk '{print $5}')"
echo ""
echo "To test in QEMU:"
echo "  qemu-system-x86_64 -cdrom vantisos-0.5.0-simple-vga-test.iso -m 512M"
echo ""