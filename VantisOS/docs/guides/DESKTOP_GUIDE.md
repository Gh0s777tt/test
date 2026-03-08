# VantisOS Desktop Guide

Complete guide to the VantisOS desktop environment - A modern, secure, and user-friendly interface built with Rust.

---

## Table of Contents

1. [Desktop Overview](#desktop-overview)
2. [Getting Started](#getting-started)
3. [Desktop Components](#desktop-components)
4. [Customization](#customization)
5. [Window Management](#window-management)
6. [File Management](#file-management)
7. [System Settings](#system-settings)
8. [Keyboard Shortcuts](#keyboard-shortcuts)
9. [Accessibility](#accessibility)

---

## Desktop Overview

VantisOS Desktop is a custom-built desktop environment designed with security and performance in mind. Key features include:

- **Vantis Shell (vshell)**: Modern Wayland-based compositor
- **Vantis Panel**: System panel with notifications, app launcher, and system tray
- **Vantis Files**: Secure file manager with built-in encryption support
- **Vantis Terminal**: High-performance terminal emulator

### Desktop Environment Stack

```
┌─────────────────────────────────────────┐
│          User Applications              │
├─────────────────────────────────────────┤
│     Vantis Panel  │  Vantis Files       │
│     Vantis Terminal  │  vsettings       │
├─────────────────────────────────────────┤
│        Vantis Shell (vshell)            │
│      Wayland Compositor │ UI Kit        │
├─────────────────────────────────────────┤
│           VantisOS Kernel               │
└─────────────────────────────────────────┘
```

---

## Getting Started

### First Login

After installation, you'll be greeted by the VantisOS login manager:

1. Select your user account
2. Enter your password
3. Optional: Select desktop session (Vantis Desktop, Vantis Minimal)
4. Press Enter or click the arrow to log in

### Initial Setup Wizard

On first login, the setup wizard guides you through:

- Language and region settings
- Keyboard layout configuration
- Network setup
- Privacy preferences
- Theme selection

---

## Desktop Components

### Vantis Panel

The system panel located at the bottom of the screen provides:

| Component | Description |
|-----------|-------------|
| App Launcher | Click to open application menu |
| Taskbar | Shows running applications |
| System Tray | Network, sound, battery indicators |
| Clock | Time, date, and calendar |
| User Menu | Power options, settings, logout |

**Panel Customization**:
```bash
# Panel configuration
vpanel-config --position top|bottom
vpanel-config --size 32|40|48
vpanel-config --autohide true|false
```

### Vantis Files

The secure file manager:

```bash
# Open file manager
vfiles

# Open specific directory
vfiles /home/user/Documents

# With encryption enabled
vfiles --encrypted /secure/
```

**Features**:
- Tabbed interface
- Built-in search with indexing
- Encrypted directory support
- Cloud storage integration
- Archive handling (zip, tar, 7z)

### Vantis Terminal

High-performance terminal emulator:

```bash
# Launch terminal
vterm

# With specific profile
vterm --profile development

# Split terminal
vterm --split vertical|horizontal
```

**Features**:
- GPU-accelerated rendering
- Multiple profiles
- Split panes
- Custom themes
- SSH integration

### Application Launcher

Press `Super` key or click the Vantis logo to open:

- Search applications by name
- Quick actions (Shutdown, Restart, Sleep)
- Recent documents
- Pinned applications

---

## Customization

### Themes

VantisOS supports theming through CSS-like stylesheets:

```bash
# List available themes
vtheme list

# Apply theme
vtheme apply dark-crimson

# Create custom theme
vtheme create my-theme --base dark

# Theme location
~/.config/vantis/themes/
```

### Default Themes

| Theme | Description |
|-------|-------------|
| Vantis Dark | Deep black (#0A0A0A) with crimson accents (#DC143C) |
| Vantis Light | Clean white with subtle accents |
| Midnight | Dark blue theme for night coding |
| High Contrast | Accessibility-optimized theme |

### Icons

```bash
# Change icon theme
vsettings set icon-theme "Papirus-Dark"

# Available icon themes
vsettings list icon-themes
```

### Fonts

```bash
# Set system fonts
vsettings set font-ui "Inter 11"
vsettings set font-monospace "JetBrains Mono 10"

# Font configuration file
~/.config/vantis/fonts.conf
```

### Wallpaper

```bash
# Set wallpaper
vwallpaper set /path/to/image.jpg

# Set wallpaper per workspace
vwallpaper set --workspace 1 /path/to/image1.jpg
vwallpaper set --workspace 2 /path/to/image2.jpg

# Slideshow mode
vwallpaper slideshow --interval 300 /path/to/wallpapers/
```

---

## Window Management

### Window Controls

- **Move**: `Super` + drag window
- **Resize**: `Super` + right-click drag
- **Maximize**: Double-click title bar or `Super` + `Up`
- **Minimize**: Click minimize button or `Super` + `Down`
- **Close**: `Alt` + `F4` or click close button

### Workspaces

VantisOS supports multiple workspaces for organization:

```bash
# Navigate workspaces
Super + [1-9]        # Switch to workspace N
Super + Shift + [1-9] # Move window to workspace N

# Workspace overview
Super + W            # Show workspace grid
```

### Tiling Mode

Enable automatic window tiling:

```bash
# Toggle tiling mode
Super + T

# Tiling layouts
vshell-tiling layout horizontal|vertical|grid|auto
```

### Window Rules

Configure automatic window behavior:

```bash
# Example: Always open terminal on workspace 2
vwindow-rule add --class "vterm" --workspace 2

# Example: Float specific applications
vwindow-rule add --class "calculator" --float true
```

---

## File Management

### Directory Structure

```
/home/user/
├── Desktop/          # Desktop files
├── Documents/        # Documents
├── Downloads/        # Downloaded files
├── Music/            # Music files
├── Pictures/         # Images
├── Videos/           # Video files
├── .config/          # Application configs
├── .local/           # Local data
└── . encrypted/      # Encrypted storage
```

### Encrypted Directories

VantisOS provides built-in encryption:

```bash
# Create encrypted directory
vencrypt create ~/.encrypted

# Mount encrypted directory
vencrypt mount ~/.encrypted

# Unmount
vencrypt unmount ~/.encrypted

# Auto-mount on login
vencrypt automount enable ~/.encrypted
```

### Cloud Integration

Connect cloud storage:

```bash
# Add cloud provider
vcloud add --provider nextcloud --url https://cloud.example.com

# Mount cloud storage
vcloud mount --name mycloud ~/Cloud/
```

---

## System Settings

### Access Settings

```bash
# Open settings panel
vsettings

# Or from terminal
vsettings-gui
```

### Settings Categories

| Category | Description |
|----------|-------------|
| Appearance | Themes, icons, fonts, wallpaper |
| Displays | Resolution, refresh rate, scaling |
| Sound | Volume, devices, alerts |
| Network | WiFi, Ethernet, VPN |
| Bluetooth | Devices, file transfer |
| Power | Battery, sleep, performance |
| Users | Accounts, login options |
| Privacy | Permissions, telemetry |
| Security | Firewall, encryption |
| About | System information |

### Configuration Files

```bash
# Main configuration
~/.config/vantis/settings.conf

# Per-application settings
~/.config/vantis/apps/

# System-wide defaults
/etc/vantis/defaults/
```

---

## Keyboard Shortcuts

### System Shortcuts

| Shortcut | Action |
|----------|--------|
| `Super` | Open application launcher |
| `Super` + `D` | Show desktop |
| `Super` + `E` | Open file manager |
| `Super` + `T` | Open terminal |
| `Super` + `L` | Lock screen |
| `Super` + `P` | Display settings |
| `Super` + `A` | Open settings |
| `Alt` + `F4` | Close window |
| `PrtSc` | Screenshot |
| `Shift` + `PrtSc` | Screenshot area |

### Window Management

| Shortcut | Action |
|----------|--------|
| `Super` + `Up` | Maximize window |
| `Super` + `Down` | Minimize/restore |
| `Super` + `Left/Right` | Snap window |
| `Super` + `Shift` + `Left/Right` | Move to workspace |
| `Alt` + `Tab` | Switch windows |
| `Super` + `Tab` | Workspace overview |

### Custom Shortcuts

```bash
# Add custom shortcut
vshortcut add --key "Super+Shift+S" --command "vfiles ~/Screenshots"

# List shortcuts
vshortcut list

# Remove shortcut
vshortcut remove "Super+Shift+S"
```

---

## Accessibility

### Accessibility Features

VantisOS is designed with accessibility in mind:

- **Screen Reader**: Built-in Orca integration
- **High Contrast Themes**: Optimized for low vision users
- **Screen Magnifier**: Zoom functionality
- **On-Screen Keyboard**: Virtual keyboard for touch/tablet devices
- **Sticky Keys**: Press modifier keys sequentially
- **Slow Keys**: Ignore brief key presses
- **Bounce Keys**: Ignore rapid duplicate key presses

### Enabling Accessibility

```bash
# Open accessibility settings
vsettings accessibility

# Enable screen reader
vaccess screen-reader enable

# Enable high contrast
vtheme apply high-contrast

# Enable screen magnifier
vaccess magnifier enable --level 2.0
```

### Screen Reader

```bash
# Start screen reader
orca --enable

# Configure
orca-settings
```

---

## Troubleshooting

### Desktop Not Starting

```bash
# Check logs
journalctl -u vshell

# Restart desktop
systemctl --user restart vshell

# Fallback to TTY
Ctrl + Alt + F2
```

### Display Issues

```bash
# Re-detect displays
vdisplay detect

# Set resolution manually
vdisplay set --output eDP-1 --mode 1920x1080
```

### Performance Issues

```bash
# Check resource usage
vtask-monitor

# Disable animations
vsettings set animations false

# Reduce effects
vsettings set compositor-effects none
```

---

*Last updated: March 2025 | VantisOS v1.4.0*