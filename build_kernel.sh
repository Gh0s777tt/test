#!/bin/bash
# VantisOS Kernel Build Script
# This script compiles the VantisOS kernel and converts it to a raw binary

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
KERNEL_ENTRY="src/verified/minimal_kernel/simple_entry.rs"
LINKER_SCRIPT="kernel/linker.ld"
TARGET="x86_64-unknown-linux-gnu"
KERNEL_ELF="kernel.elf"
KERNEL_BIN="kernel.bin"
OUTPUT_DIR="build"

echo -e "${GREEN}=== VantisOS Kernel Build Script ===${NC}"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Step 1: Compile kernel as object file
echo -e "${YELLOW}[1/4] Compiling kernel as object file...${NC}"
rustc --target "$TARGET" \
    --crate-type lib \
    --emit=obj \
    -C opt-level=z \
    -C panic=abort \
    -C codegen-units=1 \
    -C lto=fat \
    "$KERNEL_ENTRY" \
    -o "$OUTPUT_DIR/kernel.o"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Object compilation successful${NC}"
else
    echo -e "${RED}✗ Object compilation failed${NC}"
    exit 1
fi

# Step 2: Link object file to ELF
echo -e "${YELLOW}[2/4] Linking object file to ELF...${NC}"
ld "$OUTPUT_DIR/kernel.o" \
    -T "$LINKER_SCRIPT" \
    -o "$OUTPUT_DIR/$KERNEL_ELF" \
    -nostdlib \
    -static \
    -no-pie

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ ELF linking successful${NC}"
else
    echo -e "${RED}✗ ELF linking failed${NC}"
    exit 1
fi

# Step 3: Convert ELF to raw binary
echo -e "${YELLOW}[3/4] Converting ELF to raw binary...${NC}"
objcopy -O binary "$OUTPUT_DIR/$KERNEL_ELF" "$OUTPUT_DIR/$KERNEL_BIN"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Binary conversion successful${NC}"
else
    echo -e "${RED}✗ Binary conversion failed${NC}"
    exit 1
fi

# Step 4: Verify multiboot header
echo -e "${YELLOW}[4/4] Verifying multiboot header...${NC}"
HEADER=$(xxd -p "$OUTPUT_DIR/$KERNEL_BIN" | head -1 | cut -c1-8)
MAGIC="02b0ad1b"  # Little-endian representation of 0x1BADB002

if [ "$HEADER" = "$MAGIC" ]; then
    echo -e "${GREEN}✓ Multiboot header found at offset 0${NC}"
    echo "  Magic (little-endian): 0x$HEADER"
    echo "  Magic (big-endian): 0x1BADB002"
else
    echo -e "${RED}✗ Multiboot header not found${NC}"
    echo "  Expected (little-endian): 0x$MAGIC"
    echo "  Expected (big-endian): 0x1BADB002"
    echo "  Found: 0x$HEADER"
    exit 1
fi

# Display file information
echo ""
echo -e "${YELLOW}File information:${NC}"
echo "  Object file: $OUTPUT_DIR/kernel.o ($(ls -l "$OUTPUT_DIR/kernel.o" | awk '{print $5}') bytes)"
echo "  ELF file: $OUTPUT_DIR/$KERNEL_ELF ($(ls -l "$OUTPUT_DIR/$KERNEL_ELF" | awk '{print $5}') bytes)"
echo "  Binary file: $OUTPUT_DIR/$KERNEL_BIN ($(ls -l "$OUTPUT_DIR/$KERNEL_BIN" | awk '{print $5}') bytes)"

echo ""
echo -e "${GREEN}=== Build Complete ===${NC}"
echo "Kernel binary: $OUTPUT_DIR/$KERNEL_BIN"
echo "Ready for booting with GRUB 2"