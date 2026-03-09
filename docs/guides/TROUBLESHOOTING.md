# VantisOS Troubleshooting Guide

Comprehensive troubleshooting guide for common issues and solutions in VantisOS.

---

## Table of Contents

1. [Boot Issues](#boot-issues)
2. [Hardware Problems](#hardware-problems)
3. [Network Issues](#network-issues)
4. [Performance Problems](#performance-problems)
5. [Application Issues](#application-issues)
6. [Security Problems](#security-problems)
7. [File System Issues](#file-system-issues)
8. [Graphics & Display](#graphics--display)
9. [Sound Problems](#sound-problems)
10. [Recovery & Rescue](#recovery--rescue)

---

## Boot Issues

### System Won't Boot

**Symptoms**: Black screen, no boot logo, system hangs

**Solutions**:

```bash
# 1. Check boot order in BIOS/UEFI
# Enter BIOS (F2, F12, Del, or Esc)
# Ensure VantisOS boot entry is first

# 2. Verify boot files
ls /boot/efi/EFI/VantisOS/

# 3. Reinstall bootloader
sudo vinstall-bootloader --target x86_64-efi --efi-directory /boot/efi

# 4. Boot from recovery partition
# Select "Advanced Options" at boot menu
# Choose "Recovery Mode"
```

### Kernel Panic

**Symptoms**: System displays error messages and freezes during boot

**Solutions**:

```bash
# 1. Boot with previous kernel
# Select "Advanced Options" at boot menu
# Choose previous kernel version

# 2. Boot in safe mode
# Add kernel parameter: vantis.safe=1

# 3. Check hardware compatibility
vhardware-check

# 4. Disable problematic drivers
# Edit kernel parameters in /etc/default/vantis-grub
# Add: modprobe.blacklist=<driver_name>
```

### UEFI Secure Boot Issues

**Symptoms**: System refuses to boot with Secure Boot enabled

**Solutions**:

```bash
# 1. Disable Secure Boot temporarily
# Enter BIOS and disable Secure Boot

# 2. Enroll MOK key
sudo mokutil --import /etc/vantis/mok.der
# Follow prompts on reboot

# 3. Sign kernel
sudo sbsign --key /etc/vantis/db.key --cert /etc/vantis/db.crt /boot/vmlinuz
```

---

## Hardware Problems

### Hardware Not Detected

**Symptoms**: Device doesn't appear in system

**Solutions**:

```bash
# 1. List all hardware
lspci -v
lsusb -v

# 2. Check kernel messages
dmesg | grep -i <device>

# 3. Load driver manually
sudo modprobe <driver_name>

# 4. Scan for new hardware
sudo echo 1 > /sys/bus/pci/rescan
```

### USB Devices Not Working

**Symptoms**: USB devices not recognized

**Solutions**:

```bash
# 1. Check USB controller
lsusb
dmesg | grep -i usb

# 2. Reload USB driver
sudo modprobe -r xhci_hcd
sudo modprobe xhci_hcd

# 3. Check USB power
cat /sys/bus/usb/devices/*/power/level

# 4. Reset USB subsystem
sudo usbreset
```

### Memory Issues

**Symptoms**: Random crashes, applications close unexpectedly

**Solutions**:

```bash
# 1. Test memory
sudo memtest86+

# Or use built-in test
vdiag --component memory

# 2. Check memory usage
free -h
cat /proc/meminfo

# 3. Clear cache
sudo sync; sudo sysctl -w vm.drop_caches=3

# 4. Check for memory errors
dmesg | grep -i "Out of memory"
```

### Storage Issues

**Symptoms**: Slow disk performance, read/write errors

**Solutions**:

```bash
# 1. Check disk health
sudo smartctl -a /dev/sda

# 2. Check filesystem
sudo fsck -f /dev/sda1

# 3. Monitor disk I/O
iostat -x 1

# 4. Test disk speed
sudo hdparm -tT /dev/sda
```

---

## Network Issues

### No Network Connection

**Symptoms**: Cannot connect to network, no IP address

**Solutions**:

```bash
# 1. Check interface status
ip link show
ip addr show

# 2. Bring interface up
sudo ip link set eth0 up

# 3. Request DHCP lease
sudo dhclient eth0

# 4. Check network service
sudo systemctl status NetworkManager
sudo systemctl restart NetworkManager
```

### WiFi Not Connecting

**Symptoms**: WiFi network not detected or won't connect

**Solutions**:

```bash
# 1. Check WiFi adapter
ip link show
lspci | grep -i wireless

# 2. Scan networks
sudo iw dev wlan0 scan | grep SSID

# 3. Connect manually
sudo iw dev wlan0 connect <SSID>

# 4. Check for driver issues
dmesg | grep -i wifi
lspci -k | grep -A 3 -i "network"

# 5. Regenerate WiFi configuration
vantis-wifi --reset
```

### DNS Resolution Fails

**Symptoms**: Can ping IPs but not hostnames

**Solutions**:

```bash
# 1. Check DNS settings
cat /etc/resolv.conf

# 2. Set DNS manually
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf

# 3. Flush DNS cache
sudo systemd-resolve --flush-caches

# 4. Test DNS
nslookup google.com
dig google.com
```

### Slow Network Performance

**Symptoms**: Slow downloads, high latency

**Solutions**:

```bash
# 1. Check link speed
ethtool eth0 | grep Speed

# 2. Check for packet loss
ping -c 100 google.com

# 3. Disable power saving
sudo iw dev wlan0 set power_save off

# 4. Check for interference
sudo iw dev wlan0 link
```

---

## Performance Problems

### System Running Slow

**Symptoms**: Slow response, high CPU/disk usage

**Solutions**:

```bash
# 1. Check resource usage
top
htop
vmonitor

# 2. Find CPU hogs
ps aux --sort=-%cpu | head

# 3. Check I/O wait
iostat -x 1

# 4. Identify heavy processes
iotop

# 5. Kill problematic process
kill -9 <PID>
```

### High Memory Usage

**Symptoms**: System uses swap frequently, slow

**Solutions**:

```bash
# 1. Check memory usage
free -h
cat /proc/meminfo

# 2. Find memory hogs
ps aux --sort=-%mem | head

# 3. Clear caches
sudo sync; echo 3 | sudo tee /proc/sys/vm/drop_caches

# 4. Adjust swappiness
echo 10 | sudo tee /proc/sys/vm/swappiness

# 5. Restart memory-intensive services
sudo systemctl restart <service>
```

### Overheating

**Symptoms**: System overheats, thermal throttling

**Solutions**:

```bash
# 1. Check temperatures
sensors
vcamera --thermal

# 2. Monitor fan speeds
fancontrol --test

# 3. Check CPU frequency
cat /proc/cpuinfo | grep MHz

# 4. Set performance profile
vpower profile balanced

# 5. Clean fan/thermal paste (hardware)
```

---

## Application Issues

### Application Won't Start

**Symptoms**: Application crashes on launch or doesn't open

**Solutions**:

```bash
# 1. Run from terminal
<application> --verbose

# 2. Check dependencies
ldd $(which <application>)

# 3. Clear cache
rm -rf ~/.cache/<application>
rm -rf ~/.config/<application>

# 4. Reinstall
vantis-pkg remove <application>
vantis-pkg install <application>

# 5. Check logs
journalctl -xe | grep <application>
```

### Application Crashes

**Symptoms**: Application crashes during use

**Solutions**:

```bash
# 1. Check core dumps
coredumpctl list
coredumpctl info <PID>

# 2. Check for missing libraries
ldd /usr/bin/<application>

# 3. Run with debug
G_DEBUG=1 <application>

# 4. Check permissions
ls -la ~/.config/<application>
```

### Missing Dependencies

**Symptoms**: "library not found" errors

**Solutions**:

```bash
# 1. Find which package provides library
vantis-pkg whatprovides <library.so>

# 2. Install missing dependency
vantis-pkg install <dependency-package>

# 3. Update library cache
sudo ldconfig

# 4. Rebuild application
vantis-pkg reinstall <application>
```

---

## Security Problems

### Firewall Blocking Connections

**Symptoms**: Cannot connect to services

**Solutions**:

```bash
# 1. Check firewall status
sudo ufw status

# 2. List rules
sudo ufw status numbered

# 3. Allow port
sudo ufw allow <port>

# 4. Reset firewall
sudo ufw reset
sudo ufw enable

# 5. Check Vantis Guard
vguard status
vguard firewall list
```

### Permission Denied

**Symptoms**: Cannot access files or run commands

**Solutions**:

```bash
# 1. Check file permissions
ls -la <file>

# 2. Fix permissions
sudo chmod 755 <file>
sudo chown $USER:$USER <file>

# 3. Check user groups
groups $USER

# 4. Add to required group
sudo usermod -aG <group> $USER

# 5. Use sudo for system files
sudo <command>
```

### SELinux/AppArmor Issues

**Symptoms**: Applications blocked by security policies

**Solutions**:

```bash
# 1. Check audit logs
sudo ausearch -m avc -ts recent

# 2. Generate policy
sudo audit2allow -a -M <policy_name>
sudo semodule -i <policy_name>.pp

# 3. Set permissive mode (temporary)
sudo setenforce 0

# 4. Check AppArmor status
sudo aa-status
```

---

## File System Issues

### Read-Only Filesystem

**Symptoms**: Cannot write to disk, "read-only filesystem" error

**Solutions**:

```bash
# 1. Check mount status
mount | grep " / "

# 2. Remount as read-write
sudo mount -o remount,rw /

# 3. Check for errors
sudo dmesg | grep -i "read-only"

# 4. Run filesystem check
sudo fsck -f /dev/sda1
```

### Disk Full

**Symptoms**: Cannot write files, low disk space warnings

**Solutions**:

```bash
# 1. Check disk usage
df -h

# 2. Find large files
sudo du -h --max-depth=1 / | sort -hr | head

# 3. Clean package cache
sudo vantis-pkg clean

# 4. Remove old kernels
sudo vcleanup --kernels

# 5. Find and remove large log files
sudo journalctl --vacuum-size=100M
```

### Corrupted Files

**Symptoms**: I/O errors, cannot read files

**Solutions**:

```bash
# 1. Check filesystem
sudo fsck -f -y /dev/sda1

# 2. Check disk health
sudo smartctl -a /dev/sda

# 3. Recover files (if possible)
sudo photorec /dev/sda

# 4. Restore from backup
vbackup restore /backup/location
```

---

## Graphics & Display

### No Display / Black Screen

**Symptoms**: System boots but screen is black

**Solutions**:

```bash
# 1. Check display connection
xrandr
vdisplay detect

# 2. Try different TTY
Ctrl + Alt + F2

# 3. Restart display manager
sudo systemctl restart gdm
# Or for Vantis Shell
systemctl --user restart vshell

# 4. Check graphics driver
lspci -k | grep -A 2 -i "VGA"

# 5. Use VESA fallback
vdisplay --vesa
```

### Screen Tearing

**Symptoms**: Horizontal lines during video/games

**Solutions**:

```bash
# 1. Enable VSync
vdisplay --vsync on

# 2. Configure compositor
vsettings set compositor-tearing false

# 3. Force full composition pipeline
nvidia-settings --assign CurrentMetaMode="nvidia-auto-select +0+0 { ForceFullCompositionPipeline = On }"
```

### Wrong Resolution

**Symptoms**: Display resolution incorrect

**Solutions**:

```bash
# 1. List available resolutions
xrandr

# 2. Set resolution
xrandr --output <display> --mode 1920x1080

# 3. Add custom resolution
cvt 1920 1080 60
xrandr --newmode "1920x1080_60.00" <cvt-output>
xrandr --addmode <display> "1920x1080_60.00"

# 4. Save configuration
vdisplay save
```

---

## Sound Problems

### No Sound

**Symptoms**: No audio output

**Solutions**:

```bash
# 1. Check volume levels
pactl get-sink-volume @DEFAULT_SINK@

# 2. Unmute
pactl set-sink-mute @DEFAULT_SINK@ 0
pactl set-sink-volume @DEFAULT_SINK@ 100%

# 3. Check audio devices
pactl list short sinks
aplay -l

# 4. Restart audio
pulseaudio -k
pulseaudio --start

# 5. Test audio
speaker-test -t sine -f 440 -c 2
```

### Microphone Not Working

**Symptoms**: No audio input

**Solutions**:

```bash
# 1. Check input devices
pactl list short sources

# 2. Unmute microphone
pactl set-source-mute @DEFAULT_SOURCE@ 0
pactl set-source-volume @DEFAULT_SOURCE@ 100%

# 3. Test microphone
arecord -d 5 test.wav && aplay test.wav

# 4. Check alsamixer
alsamixer
```

---

## Recovery & Rescue

### Boot into Recovery Mode

```bash
# 1. At boot menu, select "Advanced Options"
# 2. Choose "Recovery Mode"
# 3. Select from recovery menu:
#    - Clean (clean packages)
#    - Dpkg (repair packages)
#    - Fsck (filesystem check)
#    - Root (root shell)
#    - Network (enable networking)
```

### Create Recovery USB

```bash
# Create bootable recovery USB
sudo dd if=/usr/share/vantis/recovery.iso of=/dev/sdX bs=4M status=progress && sync
```

### Emergency Shell

```bash
# If system fails to boot, add to kernel parameters:
init=/bin/bash

# Mount root filesystem
mount -o remount,rw /

# Fix issues and reboot
exec /sbin/init
```

### System Restore

```bash
# List available restore points
vbackup list

# Restore to previous state
vbackup restore --point <restore-point>

# Factory reset (use with caution)
vantis-reset --factory
```

---

## Getting Help

### Log Files

```bash
# System logs
/var/log/syslog
/var/log/vantis/

# Journal
journalctl -b            # Current boot
journalctl -p err        # Errors only
journalctl -u <service>  # Service logs

# Kernel messages
dmesg
/var/log/kern.log
```

### Diagnostic Report

```bash
# Generate full diagnostic
vdiag --full > vantis-diagnostics.txt

# Upload for support
vdiag --upload
```

### Support Channels

- **Documentation**: https://docs.vantis.os
- **GitHub Issues**: https://github.com/vantisCorp/VantisOS/issues
- **Discord**: https://discord.gg/vantisos
- **Forum**: https://community.vantis.os
- **Email**: support@vantis.os

---

*Last updated: March 2025 | VantisOS v1.4.0*