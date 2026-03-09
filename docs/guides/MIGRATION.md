# VantisOS Migration Guide

Complete guide to migrating to VantisOS from other operating systems.

---

## Table of Contents

1. [Before You Migrate](#before-you-migrate)
2. [Migration Paths](#migration-paths)
3. [Migrating from Windows](#migrating-from-windows)
4. [Migrating from macOS](#migrating-from-macos)
5. [Migrating from Other Linux Distros](#migrating-from-other-linux-distros)
6. [Data Migration](#data-migration)
7. [Application Migration](#application-migration)
8. [Configuration Migration](#configuration-migration)
9. [Common Migration Challenges](#common-migration-challenges)

---

## Before You Migrate

### System Requirements

Ensure your system meets the minimum requirements:

| Component | Requirement |
|-----------|-------------|
| CPU | 64-bit x86 processor |
| RAM | 2 GB minimum, 4 GB recommended |
| Storage | 10 GB minimum, 20 GB recommended |
| Boot | UEFI support |
| Graphics | VESA-compatible |

### Hardware Compatibility

Check your hardware before migrating:

```bash
# On Linux before migration
lspci -nnk | grep -i vga
lsusb
lspci -nnk | grep -i net

# Use VantisOS live system to test
# Boot from USB without installing
```

### Backup Your Data

**CRITICAL**: Always backup before migrating:

```bash
# On Windows
# Copy important files to external drive

# On macOS
# Use Time Machine or external backup

# On Linux
rsync -av /home/user /backup/location/
```

### Migration Checklist

- [ ] Backup all important data
- [ ] Export browser bookmarks/passwords
- [ ] List installed applications
- [ ] Note custom configurations
- [ ] Check hardware compatibility
- [ ] Create bootable VantisOS media
- [ ] Test VantisOS in live mode
- [ ] Plan partition scheme

---

## Migration Paths

### Clean Installation

Recommended for most users:

1. Backup data to external drive
2. Install VantisOS on fresh partition
3. Restore data after installation
4. Reinstall applications

**Pros**: Clean system, no conflicts, optimal performance
**Cons**: Need to reinstall applications, reconfigure system

### Dual Boot

Run VantisOS alongside existing OS:

1. Create free space on disk
2. Install VantisOS to new partition
3. Configure bootloader (GRUB) to boot both OS

**Pros**: Keep existing OS, safe migration path
**Cons**: Reduced disk space, potential bootloader issues

### In-Place Migration

Replace existing OS with VantisOS:

1. Install VantisOS over existing partition
2. Preserve home directory (optional)
3. Migrate configurations

**Pros**: Keep data, save disk space
**Cons**: Potential conflicts, complex process

### Virtual Machine

Test VantisOS in VM before full migration:

```bash
# Use QEMU, VirtualBox, or VMware
# Clone existing VM to VantisOS
# Test applications and workflows
```

---

## Migrating from Windows

### Pre-Migration Checklist

```powershell
# List installed programs
Get-WmiObject -Class Win32_Product | Select-Object Name,Version > installed-programs.txt

# Export network settings
netsh interface show interface > network-config.txt

# Note Windows version
systeminfo | findstr /B /C:"OS Name" /C:"OS Version"
```

### Dual Boot with Windows

1. **Create free space in Windows**
   - Open Disk Management
   - Shrink Windows partition
   - Leave 20-40 GB for VantisOS

2. **Disable Fast Startup**
   ```powershell
   powercfg -h off
   ```

3. **Install VantisOS**
   - Boot from VantisOS USB
   - Install to free space
   - Install GRUB to UEFI partition

4. **Configure boot order**
   - GRUB will automatically detect Windows
   - Set VantisOS as default (optional)

### Migrating Data from Windows

```bash
# Mount Windows partition in VantisOS
sudo mkdir /mnt/windows
sudo mount /dev/sdX1 /mnt/windows

# Copy user data
rsync -av /mnt/windows/Users/<username>/Documents/ ~/Documents/
rsync -av /mnt/windows/Users/<username>/Desktop/ ~/Desktop/
rsync -av /mnt/windows/Users/<username>/Pictures/ ~/Pictures/

# Copy application data
rsync -av /mnt/windows/Users/<username>/AppData/Local/ ~/windows-appdata/
```

### Windows Application Equivalents

| Windows Application | VantisOS Equivalent | Installation |
|---------------------|---------------------|--------------|
| Microsoft Office | LibreOffice | `vantis-pkg install libreoffice` |
| Visual Studio | VS Code, RustRover | `vantis-pkg install vscode` |
| Photoshop | GIMP, Krita | `vantis-pkg install gimp` |
| Chrome | Firefox, Chromium | `vantis-pkg install firefox` |
| Notepad | Vantis Editor, Neovim | `vantis-pkg install neovim` |
| File Explorer | Vantis Files | Pre-installed |
| Windows Terminal | Vantis Terminal | Pre-installed |
| PowerShell | Bash, Zsh | Pre-installed |
| Task Manager | Vantis Monitor | `vmonitor` |

### Windows Configuration Migration

```bash
# Import Windows fonts
sudo mkdir -p /usr/share/fonts/windows
sudo cp /mnt/windows/Windows/Fonts/* /usr/share/fonts/windows/
sudo fc-cache -fv

# Import browser data
# Firefox: Copy profile from Windows to VantisOS Firefox profile
# Chrome: Use Google account sync

# Import SSH keys
cp /mnt/windows/Users/<username>/.ssh/* ~/.ssh/
chmod 600 ~/.ssh/id_rsa
```

### Running Windows Applications

For applications without VantisOS equivalents:

```bash
# Install Wine
vantis-pkg install wine

# Install Windows application
wine installer.exe

# Or use Bottles for better compatibility
vantis-pkg install bottles
```

---

## Migrating from macOS

### Pre-Migration Checklist

```bash
# List installed applications
ls /Applications > installed-apps.txt

# Export network settings
networksetup -listallhardwareports > network-config.txt

# Note macOS version
sw_vers
```

### Dual Boot with macOS

1. **Create free space**
   ```bash
   # In macOS Disk Utility
   # Resize macOS partition
   # Leave 20-40 GB for VantisOS
   ```

2. **Install VantisOS**
   - Boot from VantisOS USB (hold Option during boot)
   - Install to free space
   - Install GRUB to VantisOS partition

3. **Configure bootloader**
   ```bash
   # Use rEFInd for dual boot
   # Or use VantisOS bootloader
   ```

### Migrating Data from macOS

```bash
# Mount macOS partition
sudo mkdir /mnt/macos
sudo mount -t hfsplus /dev/sdX2 /mnt/macos

# Copy user data
rsync -av /mnt/macos/Users/<username>/Documents/ ~/Documents/
rsync -av /mnt/macos/Users/<username>/Desktop/ ~/Desktop/
rsync -av /mnt/macos/Users/<username>/Pictures/ ~/Pictures/

# Copy application data
rsync -av /mnt/macos/Users/<username>/Library/ ~/macos-library/
```

### macOS Application Equivalents

| macOS Application | VantisOS Equivalent | Installation |
|-------------------|---------------------|--------------|
| Pages/Numbers/Keynote | LibreOffice | `vantis-pkg install libreoffice` |
| Xcode | VS Code, RustRover | `vantis-pkg install vscode` |
| Safari | Vantis Browser, Firefox | `vantis-pkg install firefox` |
| Preview | Vantis Image Viewer | Pre-installed |
| Finder | Vantis Files | Pre-installed |
| Terminal | Vantis Terminal | Pre-installed |
| iTerm2 | Vantis Terminal | Pre-installed |
| Homebrew | Vantis Package Manager | Built-in |

### macOS Configuration Migration

```bash
# Import macOS fonts
sudo cp /mnt/macos/Library/Fonts/* /usr/share/fonts/macos/
sudo fc-cache -fv

# Import browser bookmarks
# Safari: Export HTML from Safari, import to Vantis Browser
# Chrome: Use Google account sync

# Import SSH keys
cp /mnt/macos/Users/<username>/.ssh/* ~/.ssh/
chmod 600 ~/.ssh/id_rsa

# Import GPG keys
cp /mnt/macos/Users/<username>/.gnupg/* ~/.gnupg/
```

---

## Migrating from Other Linux Distros

### Pre-Migration Checklist

```bash
# List installed packages
dpkg -l > installed-packages.txt  # Debian/Ubuntu
rpm -qa > installed-packages.txt  # RHEL/Fedora
pacman -Q > installed-packages.txt # Arch

# Export system configurations
tar czf configs-backup.tar.gz ~/.config/

# Note distribution version
cat /etc/os-release
```

### Preserving Home Directory

Install VantisOS while preserving existing home directory:

1. **Backup home directory**
   ```bash
   tar czf home-backup.tar.gz /home/user/
   ```

2. **Install VantisOS**
   - Use manual partitioning
   - Keep existing home partition
   - Install root to new partition

3. **Restore configuration**
   ```bash
   # Restore specific configs
   tar xzf home-backup.tar.gz /home/user/.config/
   ```

### Package Mapping

```bash
# Automatic package mapping script
vantis-migrate --distro ubuntu --packages installed-packages.txt

# This will:
# - Map Ubuntu packages to VantisOS equivalents
# - Create install script
# - Handle system-specific packages
```

### Application Migration

Most Linux applications will work natively on VantisOS:

```bash
# Install equivalent packages
vantis-pkg install <application>

# Or compile from source if not available
git clone <repo>
cargo build --release
sudo install target/release/<app> /usr/local/bin/
```

### Configuration Migration

```bash
# Copy application configurations
cp ~/.config/ ~/.config-backup/

# Manually migrate configs:
~/.config/vimrc          -> ~/.config/nvim/init.vim
~/.bashrc               -> ~/.bashrc
~/.ssh/                 -> ~/.ssh/
~/.gnupg/               -> ~/.gnupg/
```

### Service Migration

```bash
# List systemd services
systemctl list-unit-files --state=enabled

# Migrate custom services
sudo cp /etc/systemd/system/*.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable <service>
```

---

## Data Migration

### Automated Migration Tool

VantisOS provides automated migration:

```bash
# Launch migration assistant
vantis-migrate-assistant

# This will:
# - Detect installed OS
# - Scan for data and applications
# - Create migration plan
# - Execute migration
```

### Manual Data Migration

```bash
# Mount source partition
sudo mkdir /mnt/source
sudo mount /dev/sdX1 /mnt/source

# Create migration directories
mkdir -p ~/Migrated/{Documents,Pictures,Music,Videos,Downloads}

# Copy data
rsync -av --progress /mnt/source/home/user/Documents/ ~/Migrated/Documents/
rsync -av --progress /mnt/source/home/user/Pictures/ ~/Migrated/Pictures/
rsync -av --progress /mnt/source/home/user/Music/ ~/Migrated/Music/
rsync -av --progress /mnt/source/home/user/Videos/ ~/Migrated/Videos/
```

### Selective Migration

```bash
# Migrate only specific directories
vantis-migrate --dirs Documents,Projects,Code

# Exclude directories
vantis-migrate --exclude node_modules,.cache

# Migrate by file type
vantis-migrate --extensions .jpg,.png,.mp3,.mp4
```

---

## Application Migration

### Browser Data

```bash
# Firefox
# Use Firefox Sync to sync bookmarks, passwords, history

# Chrome/Chromium
# Use Google account sync

# Edge
# Export bookmarks to HTML, import to Vantis Browser
```

### Email

```bash
# Thunderbird
# Copy profile from source OS
cp -r /mnt/source/.thunderbird/ ~/.thunderbird/

# Or use IMAP/POP3
# Setup new account in VantisOS mail client
```

### Development Tools

```bash
# Git configuration
cp ~/.gitconfig ~/.gitconfig.backup
cp /mnt/source/home/user/.gitconfig ~/.gitconfig

# SSH keys
cp -r /mnt/source/home/user/.ssh ~/.ssh/
chmod 600 ~/.ssh/id_rsa

# GPG keys
gpg --import /mnt/source/home/user/.gnupg/*.asc

# Docker containers (if using)
docker save <image> > image.tar
docker load < image.tar
```

---

## Configuration Migration

### Shell Configuration

```bash
# Backup existing shell configs
cp ~/.bashrc ~/.bashrc.old

# Migrate aliases and functions
# Copy custom entries from old .bashrc to new .bashrc

# Migrate .profile
cp /mnt/source/home/user/.profile ~/.profile
```

### Desktop Environment Settings

```bash
# Migrate Vantis Shell settings (if upgrading)
cp ~/.config/vshell/ ~/.config/vshell/

# Migrate theme settings
vtheme apply <theme-name>
```

### Network Configuration

```bash
# Import Wi-Fi passwords
vantis-wifi import --source /mnt/source/

# Import network profiles
vantis-network import /mnt/source/etc/NetworkManager/system-connections/
```

---

## Common Migration Challenges

### Hardware Issues

**Problem**: WiFi adapter not working

**Solution**:
```bash
# Check if driver exists
lspci -k | grep -i wifi

# Install appropriate driver
vantis-pkg install firmware-iwlwifi  # Intel
vantis-pkg install firmware-atheros  # Atheros
```

**Problem**: Graphics not working

**Solution**:
```bash
# Use VESA fallback
vdisplay --vesa

# Install proprietary drivers
vantis-pkg install nvidia-driver      # NVIDIA
vantis-pkg install amdgpu-driver      # AMD
```

### Software Compatibility

**Problem**: Windows-specific application needed

**Solution**:
```bash
# Use Wine for Windows applications
vantis-pkg install wine

# Or use Bottles for better compatibility
vantis-pkg install bottles

# Or run Windows in VM for critical apps
vantis-pkg install virt-manager
```

### Data Permissions

**Problem**: Cannot access migrated data

**Solution**:
```bash
# Fix ownership
sudo chown -R $USER:$USER ~/Migrated/

# Fix permissions
find ~/Migrated/ -type d -exec chmod 755 {} \;
find ~/Migrated/ -type f -exec chmod 644 {} \;
```

### Dual Boot Bootloader

**Problem**: Cannot boot Windows after VantisOS installation

**Solution**:
```bash
# Update GRUB
sudo vupdate-grub

# Or add Windows entry manually
sudo nano /etc/vantis-grub.d/40_custom

# Add:
menuentry "Windows 10" {
    insmod part_gpt
    insmod fat
    set root='(hd0,gpt1)'
    chainloader /EFI/Microsoft/Boot/bootmgfw.efi
}
```

---

## Post-Migration

### Verification

```bash
# Verify data integrity
vantis-verify-integrity ~/Migrated/

# Check application installations
vantis-pkg list --installed

# Test hardware
vhardware-check
```

### Optimization

```bash
# Update system
vantis-pkg update && vantis-pkg upgrade

# Remove old packages
vantis-pkg autoremove

# Clean cache
vantis-pkg clean

# Optimize disk
sudo vdisk-optimize /
```

### Customization

```bash
# Set theme
vtheme apply vantis-dark

# Configure desktop
vsettings --wizard

# Install favorite applications
vantis-pkg install firefox libreoffice code gimp
```

---

## Getting Help

### Migration Support

- **Documentation**: https://docs.vantis.os/migration
- **Migration Forum**: https://community.vantis.os/c/migration
- **Discord**: https://discord.gg/vantisos
- **Email**: migration@vantis.os

### Migration Wizard

Run the interactive migration wizard:

```bash
vantis-migrate-wizard
```

---

*Last updated: March 2025 | VantisOS v1.4.0*