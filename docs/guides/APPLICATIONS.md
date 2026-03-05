# VantisOS Applications Guide

Complete guide to applications available in VantisOS - A curated selection of secure, performant, and privacy-respecting software.

---

## Table of Contents

1. [Application Management](#application-management)
2. [Pre-installed Applications](#pre-installed-applications)
3. [Installing Additional Software](#installing-additional-software)
4. [Application Categories](#application-categories)
5. [Development Tools](#development-tools)
6. [Multimedia Applications](#multimedia-applications)
7. [Productivity Suite](#productivity-suite)
8. [Security Tools](#security-tools)
9. [Vantis Native Apps](#vantis-native-apps)

---

## Application Management

### Vantis Package Manager

VantisOS uses a modern package management system:

```bash
# Search for packages
vantis-pkg search <query>

# Install package
vantis-pkg install <package>

# Remove package
vantis-pkg remove <package>

# Update all packages
vantis-pkg update && vantis-pkg upgrade

# List installed packages
vantis-pkg list --installed

# Package info
vantis-pkg info <package>
```

### Application Store

Launch the graphical application store:

```bash
# Open Vantis Store
vstore
```

**Features**:
- Curated application catalog
- One-click install
- Automatic updates
- User ratings and reviews
- Screenshots and descriptions

### Flatpak Support

VantisOS supports Flatpak for sandboxed applications:

```bash
# Add Flathub repository
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Search Flatpak apps
flatpak search <query>

# Install Flatpak
flatpak install flathub <app-id>

# Run Flatpak
flatpak run <app-id>
```

---

## Pre-installed Applications

### Core Applications

| Application | Description | Command |
|-------------|-------------|---------|
| Vantis Files | File manager | `vfiles` |
| Vantis Terminal | Terminal emulator | `vterm` |
| Vantis Browser | Web browser | `vbrowser` |
| Vantis Editor | Text editor | `vedit` |
| Vantis Settings | System settings | `vsettings` |
| Vantis Store | App store | `vstore` |
| Vantis Calculator | Calculator | `vcalc` |
| Vantis Calendar | Calendar | `vcalendar` |

### System Utilities

| Utility | Description | Command |
|---------|-------------|---------|
| System Monitor | Resource monitor | `vmonitor` |
| Disk Utility | Disk management | `vdisk` |
| Archive Manager | Archive tool | `varchive` |
| Screenshot Tool | Screen capture | `vscreenshot` |
| Power Manager | Power settings | `vpower` |

---

## Installing Additional Software

### Official Repositories

```bash
# Install from official repo
vantis-pkg install firefox
vantis-pkg install vscode
vantis-pkg install gimp
```

### Popular Applications

```bash
# Web Browsers
vantis-pkg install firefox chromium

# Office Suite
vantis-pkg install libreoffice

# Media Players
vantis-pkg install vlc mpv

# Graphics
vantis-pkg install gimp inkscape blender

# Development
vantis-pkg install vscode rust-analyzer
```

### Building from Source

For applications not in repositories:

```bash
# Clone and build
git clone <repository>
cd <project>

# Using Cargo (Rust)
cargo build --release

# Using Make
./configure
make
sudo make install
```

---

## Application Categories

### Internet & Networking

```bash
# Web Browsers
vantis-pkg install firefox       # Mozilla Firefox
vantis-pkg install chromium      # Chromium Browser

# Email Clients
vantis-pkg install thunderbird   # Mozilla Thunderbird
vantis-pkg install geary         # Simple email client

# File Transfer
vantis-pkg install filezilla     # FTP client
vantis-pkg install transmission  # BitTorrent client

# Communication
vantis-pkg install discord       # Discord
vantis-pkg install telegram      # Telegram Desktop
```

### Office & Productivity

```bash
# Office Suite
vantis-pkg install libreoffice   # Complete office suite

# Documents
vantis-pkg install calibre       # E-book manager
vantis-pkg install pdfarranger   # PDF manipulation

# Notes & Organization
vantis-pkg install obsidian      # Knowledge base
vantis-pkg install joplin        # Note taking
```

### Graphics & Design

```bash
# Image Editing
vantis-pkg install gimp          # GNU Image Manipulation
vantis-pkg install krita         # Digital painting

# Vector Graphics
vantis-pkg install inkscape      # SVG editor

# 3D Graphics
vantis-pkg install blender       # 3D creation suite
```

### Multimedia

```bash
# Video Players
vantis-pkg install vlc           # VLC Media Player
vantis-pkg install mpv           # Minimal player

# Audio
vantis-pkg install audacity      # Audio editor
vantis-pkg install rhythmbox     # Music player

# Video Editing
vantis-pkg install kdenlive      # Video editor
vantis-pkg install obs-studio    # Streaming/recording
```

---

## Development Tools

### Editors & IDEs

```bash
# Text Editors
vantis-pkg install vscode        # Visual Studio Code
vantis-pkg install neovim        # Modern Vim
vantis-pkg install helix         # Helix editor

# IDEs
vantis-pkg install jetbrains-rustRover  # Rust IDE
vantis-pkg install clion         # C/C++ IDE
```

### Compilers & Runtimes

```bash
# Rust
rustup toolchain install stable
rustup component add rustfmt clippy

# Python
vantis-pkg install python3 python3-pip

# Node.js
vantis-pkg install nodejs npm

# C/C++
vantis-pkg install gcc g++ cmake

# Go
vantis-pkg install golang
```

### Development Utilities

```bash
# Version Control
vantis-pkg install git git-gui

# Containers
vantis-pkg install podman docker

# Database
vantis-pkg install postgresql sqlite
```

---

## Multimedia Applications

### Audio Production

```bash
# Digital Audio Workstation
vantis-pkg install ardour         # Professional DAW
vantis-pkg install lmms           # Music production

# Audio Tools
vantis-pkg install audacity       # Audio editor
vantis-pkg install pulseaudio     # Sound server
```

### Video Production

```bash
# Video Editing
vantis-pkg install kdenlive       # Non-linear editor
vantis-pkg install shotcut        # Video editor

# Streaming
vantis-pkg install obs-studio     # Open Broadcaster

# Codecs
vantis-pkg install ffmpeg gstreamer
```

### Graphics

```bash
# Image Editing
vantis-pkg install gimp           # Photo editing
vantis-pkg install krita          # Digital art
vantis-pkg install darktable      # Photo workflow

# Vector & Design
vantis-pkg install inkscape       # Vector graphics
vantis-pkg install figma-linux    # Figma client
```

---

## Productivity Suite

### Office Applications

```bash
# LibreOffice Suite
vantis-pkg install libreoffice-writer  # Word processor
vantis-pkg install libreoffice-calc    # Spreadsheet
vantis-pkg install libreoffice-impress # Presentations

# Or complete suite
vantis-pkg install libreoffice
```

### Note Taking

```bash
# Modern Note Apps
vantis-pkg install obsidian       # Knowledge management
vantis-pkg install joplin         # Markdown notes
vantis-pkg install notion-app     # Notion client

# Simple Notes
vantis-pkg install gnote          # GNOME Notes
```

### Task Management

```bash
vantis-pkg install taskwarrior    # CLI task manager
vantis-pkg install gtk-todo       # GUI task app
```

---

## Security Tools

### System Security

```bash
# Firewall
vantis-pkg install ufw            # Uncomplicated Firewall
sudo ufw enable

# Antivirus
vantis-pkg install clamav         # ClamAV scanner

# Encryption
vantis-pkg install veracrypt      # Disk encryption
vantis-pkg install gnupg          # GPG encryption
```

### Network Security

```bash
# Network Analysis
vantis-pkg install wireshark      # Protocol analyzer
vantis-pkg install nmap           # Network scanner
vantis-pkg install tcpdump        # Packet analyzer

# Password Management
vantis-pkg install keepassxc      # Password manager
vantis-pkg install bitwarden      # Bitwarden client
```

### Privacy Tools

```bash
# VPN
vantis-pkg install openvpn        # OpenVPN client
vantis-pkg install wireguard      # WireGuard VPN

# Privacy
vantis-pkg install tor-browser    # Tor Browser
vantis-pkg install metadata-cleaner # Clean metadata
```

---

## Vantis Native Apps

### Vantis Exclusive Applications

VantisOS includes native applications built specifically for the platform:

| Application | Description | Command |
|-------------|-------------|---------|
| Vantis Guard | Security center | `vguard` |
| Vantis Sync | Cloud sync tool | `vsync` |
| Vantis Backup | Backup utility | `vbackup` |
| Vantis Update | System updater | `vupdate` |
| Vantis Diagnostics | System diagnostics | `vdiag` |

### Vantis Guard

Security management application:

```bash
# Launch Vantis Guard
vguard

# Scan system
vguard scan

# View security status
vguard status
```

**Features**:
- Real-time threat monitoring
- Firewall management
- Encryption status
- Vulnerability scanning
- Security recommendations

### Vantis Backup

System backup utility:

```bash
# Create backup
vbackup create --full /backup/location

# Incremental backup
vbackup create --incremental /backup/location

# Restore backup
vbackup restore /backup/location

# Schedule backups
vbackup schedule --daily 02:00
```

### Vantis Diagnostics

System diagnostics tool:

```bash
# Run full diagnostics
vdiag --full

# Check specific component
vdiag --component memory
vdiag --component disk
vdiag --component network

# Generate report
vdiag --report > diagnostics-report.txt
```

---

## Application Configuration

### Configuration Locations

```bash
# User configurations
~/.config/<application>/

# Application data
~/.local/share/<application>/

# Cache
~/.cache/<application>/
```

### Environment Variables

```bash
# Set application environment
export VANTIS_THEME=dark
export VANTIS_LANG=en_US

# Add to shell profile
echo 'export VANTIS_THEME=dark' >> ~/.bashrc
```

---

## Troubleshooting

### Application Won't Start

```bash
# Check dependencies
ldd /usr/bin/<application>

# Run from terminal for errors
<application> --verbose

# Clear cache
rm -rf ~/.cache/<application>
```

### Package Installation Issues

```bash
# Update package database
vantis-pkg update

# Fix broken packages
vantis-pkg fix

# Clean cache
vantis-pkg clean
```

### Flatpak Issues

```bash
# Update Flatpak
flatpak update

# Repair installation
flatpak repair

# Clear cache
rm -rf ~/.var/app/
```

---

*Last updated: March 2025 | VantisOS v1.4.0*