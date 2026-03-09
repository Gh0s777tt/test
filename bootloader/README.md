# VantisOS UEFI Bootloader

A 6-phase UEFI bootloader for VantisOS written in Rust using the `uefi-rs` crate.

## Features

- **Phase 1**: Bare metal UEFI environment setup (no_std, no_main)
- **Phase 2**: UEFI entry point with console output
- **Phase 3**: File system access via Simple File System Protocol
- **Phase 4**: ELF binary parsing and loading
- **Phase 5**: Hardware information collection (memory map, framebuffer)
- **Phase 6**: Exit boot services and kernel handoff

## Requirements

- Rust nightly toolchain
- x86_64-unknown-uefi target

## Building

```bash
# Install nightly and the UEFI target
rustup install nightly
rustup target add x86_64-unknown-uefi --toolchain nightly

# Build the bootloader
./build.sh

# Or build manually
cargo +nightly build --release \
    --target x86_64-unknown-uefi \
    -Z build-std=core,compiler_builtins,alloc \
    -Z build-std-features=compiler-builtins-mem
```

## Output

The build produces `vantis_bootloader.efi` - a UEFI executable that can be loaded by UEFI firmware.

## Creating a Bootable Image

1. Format a USB drive as FAT32
2. Copy `vantis_bootloader.efi` to `/EFI/BOOT/BOOTX64.EFI`
3. Copy your kernel to `/EFI/VANTIS/kernel.elf`
4. Boot from the USB drive in UEFI mode

## Directory Structure

```
bootloader/
├── Cargo.toml          # Package configuration
├── build.sh            # Build script
├── README.md           # This file
├── .cargo/
│   └── config.toml     # Cargo configuration for UEFI target
└── src/
    ├── main.rs         # Main bootloader logic
    ├── boot_info.rs    # Boot information structures
    ├── elf_loader.rs   # ELF parsing and loading
    └── file_system.rs  # UEFI file system access
```

## Boot Information

The bootloader passes a `BootInfo` structure to the kernel containing:

- Memory map size
- Framebuffer information (if available)
- Kernel entry point
- Kernel size

## License

MIT License - See LICENSE file in the VantisOS repository.