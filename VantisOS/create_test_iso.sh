#!/bin/bash
# VantisOS v0.5.0 Test ISO Creation Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== VantisOS v0.5.0 Test ISO Creation ===${NC}"
echo ""

# Configuration
KERNEL_BIN="build/kernel.bin"
ISO_DIR="iso"
ISO_FILE="vantisos-0.5.0-test.iso"
GRUB_CFG="$ISO_DIR/boot/grub/grub.cfg"

# Create ISO directory structure
echo -e "${YELLOW}[1/5] Creating ISO directory structure...${NC}"
mkdir -p "$ISO_DIR/boot/grub"
echo -e "${GREEN}✓ ISO directory structure created${NC}"

# Copy kernel binary
echo -e "${YELLOW}[2/5] Copying kernel binary...${NC}"
cp "$KERNEL_BIN" "$ISO_DIR/boot/vantisos-kernel.bin"
echo -e "${GREEN}✓ Kernel binary copied${NC}"

# Create GRUB configuration
echo -e "${YELLOW}[3/5] Creating GRUB configuration...${NC}"
cat > "$GRUB_CFG" << 'EOF'
menuentry "VantisOS 0.5.0 - Real Kernel Test" {
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
echo -e "${GREEN}✓ GRUB configuration created${NC}"

# Create ISO using GRUB 2
echo -e "${YELLOW}[4/5] Creating ISO using GRUB 2...${NC}"
grub-mkrescue -o "$ISO_FILE" "$ISO_DIR"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ ISO created successfully${NC}"
else
    echo -e "${RED}✗ ISO creation failed${NC}"
    exit 1
fi

# Display ISO information
echo -e "${YELLOW}[5/5] ISO information:${NC}"
echo "  ISO file: $ISO_FILE ($(ls -lh "$ISO_FILE" | awk '{print $5}'))"
echo "  Kernel: $ISO_DIR/boot/vantisos-kernel.bin ($(ls -lh "$ISO_DIR/boot/vantisos-kernel.bin" | awk '{print $5}'))"

echo ""
echo -e "${GREEN}=== Test ISO Creation Complete ===${NC}"
echo "Test ISO: $ISO_FILE"
echo "Ready for boot testing with GRUB 2"