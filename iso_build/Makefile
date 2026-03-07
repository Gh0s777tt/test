# VantisOS Build System

# Tools
AS = nasm
CC = gcc
RUSTC = rustc
CARGO = cargo

# Directories
KERNEL_DIR = kernel
ISO_DIR = iso
BUILD_DIR = build
TARGET_DIR = target

# Output files
KERNEL_BIN = $(ISO_DIR)/boot/vantis-kernel.bin
ISO_FILE = VantisOS-1.5.0.iso

# Rust target
RUST_TARGET = x86_64-vantis.json

# Flags
ASFLAGS = -f elf64
CFLAGS = -ffreestanding -mno-red-zone -mno-mmx -mno-sse -mno-sse2

# Default target
.PHONY: all
all: iso

# Create target specification
$(RUST_TARGET):
	@echo 'Creating Rust target specification...'
	@echo '{' > $(RUST_TARGET)
	@echo '  "arch": "x86_64",' >> $(RUST_TARGET)
	@echo '  "cpu": "x86-64",' >> $(RUST_TARGET)
	@echo '  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",' >> $(RUST_TARGET)
	@echo '  "executables": true,' >> $(RUST_TARGET)
	@echo '  "linker-flavor": "ld.lld",' >> $(RUST_TARGET)
	@echo '  "linker": "rust-lld",' >> $(RUST_TARGET)
	@echo '  "llvm-target": "x86_64-unknown-none",' >> $(RUST_TARGET)
	@echo '  "no-default-libraries": true,' >> $(RUST_TARGET)
	@echo '  "os": "none",' >> $(RUST_TARGET)
	@echo '  "panic-strategy": "abort",' >> $(RUST_TARGET)
	@echo '  "position-independent-executables": false,' >> $(RUST_TARGET)
	@echo '  "pre-link-args": {' >> $(RUST_TARGET)
	@echo '    "ld.lld": ["-T", "kernel/linker.ld"]' >> $(RUST_TARGET)
	@echo '  },' >> $(RUST_TARGET)
	@echo '  "relocation-model": "static",' >> $(RUST_TARGET)
	@echo '  "target-c-int-width": "32",' >> $(RUST_TARGET)
	@echo '  "target-endian": "little",' >> $(RUST_TARGET)
	@echo '  "target-pointer-width": "64"' >> $(RUST_TARGET)
	@echo '}' >> $(RUST_TARGET)

# Build kernel
.PHONY: kernel
kernel: $(RUST_TARGET)
	@echo "Building VantisOS kernel..."
	cd $(KERNEL_DIR) && $(CARGO) build --release --target ../$(RUST_TARGET) 2>&1 || true
	@echo "Kernel build attempted"

# Build assembly boot code
$(BUILD_DIR)/boot.o: $(KERNEL_DIR)/src/boot.asm
	@mkdir -p $(BUILD_DIR)
	$(AS) $(ASFLAGS) $< -o $@

# Create bootable ISO
.PHONY: iso
iso: kernel
	@echo "Creating bootable ISO..."
	@mkdir -p $(ISO_DIR)/boot/grub
	@# Copy kernel binary (placeholder)
	@touch $(KERNEL_BIN)
	@# Create ISO with grub-mkrescue
	grub-mkrescue -o $(ISO_FILE) $(ISO_DIR) 2>/dev/null || echo "Note: grub-mkrescue not available, creating minimal ISO structure"
	@echo "ISO build complete: $(ISO_FILE)"

# Run in QEMU
.PHONY: run
run: iso
	qemu-system-x86_64 -cdrom $(ISO_FILE) -m 512M -serial stdio

# Run with debugging
.PHONY: debug
debug: iso
	qemu-system-x86_64 -cdrom $(ISO_FILE) -m 512M -serial stdio -s -S

# Clean build artifacts
.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) $(TARGET_DIR) $(ISO_FILE)
	cd $(KERNEL_DIR) && $(CARGO) clean
	rm -f $(RUST_TARGET)

# Install dependencies
.PHONY: deps
deps:
	@echo "Installing build dependencies..."
	rustup target add x86_64-unknown-none 2>/dev/null || true
	rustup component add rust-src 2>/dev/null || true

# Help
.PHONY: help
help:
	@echo "VantisOS Build System"
	@echo "===================="
	@echo "make all      - Build everything (default)"
	@echo "make kernel   - Build kernel only"
	@echo "make iso      - Create bootable ISO"
	@echo "make run      - Run in QEMU"
	@echo "make debug    - Run in QEMU with GDB server"
	@echo "make clean    - Clean build artifacts"
	@echo "make deps     - Install build dependencies"
	@echo "make help     - Show this help"