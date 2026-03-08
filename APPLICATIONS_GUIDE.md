# VantisOS Applications Guide

## Pre-installed Applications

### System
- **Terminal** - Advanced terminal emulator
- **File Manager** - Modern file browser
- **Settings** - System configuration
- **Software Center** - Package management

### Development
- **Code Editor** - Lightweight editor with syntax highlighting
- **Build Tools** - GCC, Rust, Cargo

### Network
- **Web Browser** - Secure browsing
- **Network Manager** - Connection configuration

### Multimedia
- **Media Player** - Audio and video playback
- **Image Viewer** - Photo viewing

## Installing Applications

### Using vantis package manager
```bash
vantis install <package-name>
vantis search <search-term>
vantis remove <package-name>
```

### Using Cargo (Rust packages)
```bash
cargo install <crate-name>
```

## Default Application Locations

| Type | Path |
|------|------|
| System apps | `/usr/bin/` |
| User apps | `~/.local/bin/` |
| App data | `~/.local/share/` |