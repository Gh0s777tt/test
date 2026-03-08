# VantisOS Performance Guide

## System Optimization

### Boot Optimization

```bash
# Analyze boot time
systemd-analyze

# View boot services
systemd-analyze blame

# Disable slow services
systemctl disable <service>
```

### Memory Optimization

```bash
# Check memory usage
free -h

# Clear caches
sync && echo 3 | sudo tee /proc/sys/vm/drop_caches

# View memory processes
top -o %MEM
```

### CPU Optimization

```bash
# View CPU usage
top -o %CPU

# Set CPU governor
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### Disk Optimization

```bash
# Check disk I/O
iotop

# SSD trim
sudo fstrim -v /

# Check disk health
sudo smartctl -a /dev/sda
```

## Performance Tuning

### Swappiness
```bash
# Reduce swap usage (default 60)
echo 10 | sudo tee /proc/sys/vm/swappiness
```

### Filesystem
- Use ext4 or btrfs for best performance
- Enable SSD optimizations in `/etc/fstab`

### Network
```bash
# Check network speed
ethtool eth0

# DNS optimization
# Edit /etc/resolv.conf
```

## Benchmarking

```bash
# CPU benchmark
sysbench cpu run

# Memory benchmark
sysbench memory run

# Disk benchmark
dd if=/dev/zero of=test bs=1M count=1024 conv=fdatasync
```

## Monitoring Tools

- `htop` - Process viewer
- `btop` - Resource monitor
- `iotop` - Disk I/O
- `nethogs` - Network per process