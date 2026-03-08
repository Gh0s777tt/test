#!/bin/bash
# Create ISO for VantisOS v0.5.0 - Advanced Kernel

set -e

echo "Creating ISO for VantisOS v0.5.0 - Advanced Kernel..."
echo "======================================================"

# Create ISO directory structure
rm -rf iso
mkdir -p iso/boot/grub

# Copy kernel to ISO (use stripped ELF)
cp build/kernel_stripped.elf iso/boot/vantisos-kernel.elf

# Create GRUB configuration
cat > iso/boot/grub/grub.cfg << 'EOF'
set timeout=0
set default=0

menuentry "VantisOS 0.5.0 - Advanced Kernel" {
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
grub-mkrescue -o vantisos-0.5.0-advanced.iso iso

echo ""
echo "ISO created: vantisos-0.5.0-advanced.iso"
echo "======================================================"

# Display ISO size
ls -lh vantisos-0.5.0-advanced.iso