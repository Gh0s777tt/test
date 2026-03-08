# VantisOS Troubleshooting Guide

## Common Issues

### Boot Issues

#### System won't boot from USB
- Verify ISO checksum matches
- Recreate bootable USB
- Try different USB port
- Disable Secure Boot in BIOS

#### Black screen on boot
- Add `nomodeset` to kernel parameters
- Try safe graphics mode
- Check hardware compatibility

### Hardware Issues

#### Wi-Fi not working
```bash
# Check device status
ip link show

# Load driver module
modprobe <driver-name>

# Check dmesg for errors
dmesg | grep -i wifi
```

#### Audio not working
```bash
# Check audio devices
aplay -l

# Restart audio service
systemctl --user restart pipewire
```

### Performance Issues

#### Slow performance
- Check memory usage: `free -h`
- Check CPU usage: `top` or `htop`
- Disable unnecessary services

#### High disk usage
```bash
# Clean package cache
vantis clean

# Check disk usage
df -h
```

### Network Issues

#### No internet connection
```bash
# Check connection
ping google.com

# Restart network
systemctl restart NetworkManager

# Check DNS
resolvectl status
```

## Getting Help

- **Discord**: https://discord.gg/vantisos
- **Forum**: https://forum.vantis.os
- **GitHub Issues**: https://github.com/vantisCorp/VantisOS/issues