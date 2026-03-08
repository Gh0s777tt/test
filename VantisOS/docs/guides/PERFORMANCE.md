# VantisOS Performance Guide

Comprehensive guide to optimizing and monitoring VantisOS performance.

---

## Table of Contents

1. [Performance Overview](#performance-overview)
2. [System Monitoring](#system-monitoring)
3. [CPU Optimization](#cpu-optimization)
4. [Memory Management](#memory-management)
5. [Storage Performance](#storage-performance)
6. [Network Optimization](#network-optimization)
7. [Graphics Performance](#graphics-performance)
8. [Application Optimization](#application-optimization)
9. [Power Management](#power-management)
10. [Benchmarking](#benchmarking)

---

## Performance Overview

### VantisOS Performance Philosophy

VantisOS is built with performance as a core principle:

- **Rust-based kernel**: Memory-safe, zero-cost abstractions
- **Formally verified**: No undefined behavior, predictable performance
- **Minimal overhead**: Lean system services
- **Optimized scheduler**: Real-time capable process scheduling

### Performance Targets

| Metric | Target | Actual |
|--------|--------|--------|
| Boot Time | < 10s | ~8s |
| Memory Usage (Idle) | < 500MB | ~350MB |
| Context Switch | < 1μs | ~0.8μs |
| System Call | < 100ns | ~80ns |
| Interrupt Latency | < 10μs | ~8μs |

---

## System Monitoring

### Real-Time Monitoring

```bash
# VantisOS System Monitor (GUI)
vmonitor

# Terminal-based monitoring
htop

# VantisOS monitoring tool
vperf monitor
```

### Key Metrics

```bash
# CPU usage
cat /proc/cpuinfo
top -n 1 | head -n 5

# Memory usage
free -h
cat /proc/meminfo

# Disk I/O
iostat -x 1

# Network
netstat -i
ss -tuln

# All metrics at once
vperf status
```

### Performance Dashboard

```bash
# Launch performance dashboard
vperf-dashboard

# Export metrics
vperf export --format json > metrics.json

# Continuous logging
vperf log --interval 1 --output performance.log
```

### System Statistics

```bash
# Load average
cat /proc/loadavg

# Running processes
ps aux --sort=-%cpu | head -20

# Open files
lsof | wc -l

# Network connections
ss -s
```

---

## CPU Optimization

### CPU Frequency Scaling

```bash
# Check current frequency
cat /proc/cpuinfo | grep "cpu MHz"

# Available governors
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors

# Set performance mode
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Use VantisOS power tool
vpower profile performance
```

### CPU Governor Profiles

| Profile | Description | Use Case |
|---------|-------------|----------|
| performance | Maximum frequency | Gaming, compiling |
| balanced | Dynamic scaling | Daily use |
| powersave | Minimum frequency | Battery saving |
| ondemand | Scale on demand | General use |

### CPU Scheduling

```bash
# Check scheduler
cat /sys/block/sda/queue/scheduler

# Process priority
nice -n <priority> <command>
renice -n <priority> -p <pid>

# CPU affinity
taskset -c <cpu-list> <command>

# Real-time scheduling
chrt -f <priority> <command>
```

### CPU Optimization Tips

```bash
# Disable unnecessary services
sudo systemctl disable <service>

# Reduce swappiness
echo 10 | sudo tee /proc/sys/vm/swappiness

# Disable transparent hugepages (for databases)
echo never | sudo tee /sys/kernel/mm/transparent_hugepage/enabled

# Use VantisOS optimizer
voptimize cpu --profile auto
```

---

## Memory Management

### Memory Information

```bash
# Detailed memory info
cat /proc/meminfo

# Memory map
pmap <pid>

# Shared memory
ipcs -m

# Slab allocator
cat /proc/slabinfo
```

### Memory Optimization

```bash
# Clear caches
sudo sync; echo 3 | sudo tee /proc/sys/vm/drop_caches

# Adjust swappiness
echo 10 | sudo tee /proc/sys/vm/swappiness

# Adjust vfs_cache_pressure
echo 50 | sudo tee /proc/sys/vm/vfs_cache_pressure

# Compact memory
echo 1 | sudo tee /proc/sys/vm/compact_memory
```

### Memory Limits

```bash
# Set process memory limits
ulimit -v <kilobytes>

# Check limits
ulimit -a

# System-wide limits
cat /etc/security/limits.conf
```

### Memory Profiling

```bash
# Profile application memory
valgrind --tool=massif <application>

# Check memory leaks
valgrind --leak-check=full <application>

# Real-time memory tracking
heaptrack <application>
```

### Zram Configuration

```bash
# Enable zram (compressed RAM disk)
sudo modprobe zram
echo lz4 | sudo tee /sys/block/zram0/comp_algorithm
echo 2G | sudo tee /sys/block/zram0/disksize
sudo mkswap /dev/zram0
sudo swapon /dev/zram0

# VantisOS zram tool
vram enable --size 2G --algorithm lz4
```

---

## Storage Performance

### Disk Benchmarking

```bash
# Sequential read speed
sudo hdparm -tT /dev/sda

# Random I/O benchmark
sudo fio --name=randread --ioengine=libaio --iodepth=16 --rw=randread --bs=4k --direct=1 --size=512M --numjobs=4 --time_based --runtime=60

# VantisOS disk benchmark
vdisk-bench /dev/sda
```

### SSD Optimization

```bash
# Enable TRIM
sudo systemctl enable fstrim.timer
sudo systemctl start fstrim.timer

# Check TRIM support
lsblk -D

# Manual TRIM
sudo fstrim -v /

# Check I/O scheduler (should be mq-deadline or none for SSD)
cat /sys/block/nvme0n1/queue/scheduler
```

### Filesystem Optimization

```bash
# Mount options for SSD
# Add to /etc/fstab:
# /dev/nvme0n1p1 / ext4 defaults,noatime,discard 0 1

# Disable access time updates
sudo mount -o remount,noatime /

# Increase commit interval
sudo mount -o remount,commit=60 /

# Filesystem check
sudo fsck -f /dev/sda1
```

### I/O Scheduling

```bash
# Available schedulers
cat /sys/block/sda/queue/scheduler

# Set scheduler
echo mq-deadline | sudo tee /sys/block/sda/queue/scheduler

# For NVMe SSDs
echo none | sudo tee /sys/block/nvme0n1/queue/scheduler

# Permanent configuration
echo 'ACTION=="add|change", KERNEL=="sd[a-z]", ATTR{queue/scheduler}="mq-deadline"' | sudo tee /etc/udev/rules.d/60-scheduler.rules
```

### Disk Cache Optimization

```bash
# Read-ahead
sudo blockdev --setra 256 /dev/sda

# Dirty page ratio
echo 5 | sudo tee /proc/sys/vm/dirty_ratio
echo 3 | sudo tee /proc/sys/vm/dirty_background_ratio

# VantisOS disk optimizer
vdisk-optimize /dev/sda
```

---

## Network Optimization

### Network Benchmarking

```bash
# Bandwidth test
iperf3 -c <server>

# Latency test
ping -c 100 google.com | tail -1

# DNS resolution time
dig google.com | grep "Query time"

# Network statistics
netstat -s
```

### TCP Optimization

```bash
# Check current TCP settings
sysctl net.ipv4.tcp_congestion_control

# Set TCP congestion control
sudo sysctl -w net.ipv4.tcp_congestion_control=bbr

# Increase buffer sizes
sudo sysctl -w net.core.rmem_max=16777216
sudo sysctl -w net.core.wmem_max=16777216
sudo sysctl -w net.ipv4.tcp_rmem="4096 87380 16777216"
sudo sysctl -w net.ipv4.tcp_wmem="4096 65536 16777216"

# Enable TCP Fast Open
sudo sysctl -w net.ipv4.tcp_fastopen=3
```

### Network Interface Optimization

```bash
# Check offload settings
ethtool -k eth0

# Enable hardware offloading
sudo ethtool -K eth0 tso on gso on gro on

# Set ring buffers
sudo ethtool -G eth0 rx 4096 tx 4096

# Interrupt coalescing
sudo ethtool -C eth0 rx-usecs 50 tx-usecs 50
```

### DNS Optimization

```bash
# Use fast DNS servers
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf
echo "nameserver 1.1.1.1" | sudo tee -a /etc/resolv.conf

# Enable DNS caching
sudo systemctl enable systemd-resolved
sudo systemctl start systemd-resolved

# VantisOS DNS optimization
vdns-optimize --servers google,cloudflare
```

### WiFi Optimization

```bash
# Check WiFi settings
iw dev wlan0 link

# Set regulatory domain
sudo iw reg set US

# Disable power saving
sudo iw dev wlan0 set power_save off

# Change WiFi channel
vantis-wifi --channel 6
```

---

## Graphics Performance

### GPU Information

```bash
# GPU info
lspci -k | grep -A 2 -i "VGA"

# OpenGL info
glxinfo | head -20

# Vulkan info
vulkaninfo | head -30

# Current GPU usage
vmonitor --gpu
```

### Graphics Optimization

```bash
# Enable hardware acceleration
vsettings set hardware-acceleration true

# Set graphics profile
vgraphics profile performance

# Disable compositing (for gaming)
vsettings set compositor-effects none
```

### NVIDIA Optimization

```bash
# Set performance mode
nvidia-settings -a "[gpu:0]/GPUPowerMizerMode=1"

# Overclocking (use with caution)
nvidia-settings -a "[gpu:0]/GPUGraphicsClockOffset[3]=100"

# Force full composition pipeline
nvidia-settings --assign CurrentMetaMode="nvidia-auto-select +0+0 { ForceFullCompositionPipeline = On }"
```

### AMD Optimization

```bash
# Performance level
echo high | sudo tee /sys/class/drm/card0/device/power_dpm_force_performance_level

# Fan curve
vamd-fan --curve "30c:30%,60c:50%,80c:80%"
```

### Intel Optimization

```bash
# Check GPU frequency
cat /sys/class/drm/card0/gt_cur_freq_mhz

# Set maximum frequency
echo 1000 | sudo tee /sys/class/drm/card0/gt_max_freq_mhz
```

---

## Application Optimization

### Application Profiling

```bash
# Profile with perf
perf record -g <application>
perf report

# CPU profiling
perf top

# Flame graph
perf record -g <application>
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg

# VantisOS profiler
vprofile <application>
```

### Application Tuning

```bash
# Set process priority
nice -n -10 <application>

# Set CPU affinity
taskset -c 0,1 <application>

# Set I/O priority
ionice -c 1 -n 0 -p <pid>

# Preload libraries
vantis-pkg install preload
sudo systemctl enable preload
```

### Memory Optimization

```bash
# Limit application memory
systemd-run --scope -p MemoryMax=2G <application>

# Use jemalloc for better memory performance
LD_PRELOAD=/usr/lib/libjemalloc.so <application>

# Disable swap for application
mlockall --all <application>
```

### Compiler Optimization

```bash
# Rust optimization flags
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release

# GCC optimization
gcc -O3 -march=native -flto program.c -o program

# Clang optimization
clang -O3 -march=native -flto program.c -o program
```

---

## Power Management

### Power Profiles

```bash
# Available profiles
vpower profiles

# Set profile
vpower profile performance    # Maximum performance
vpower profile balanced       # Balanced
vpower profile power-saver    # Battery saving

# Current profile
vpower status
```

### CPU Power Management

```bash
# Intel P-state
cat /sys/devices/system/cpu/intel_pstate/status

# Set EPP (Energy Performance Preference)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference

# Disable turbo boost (saves power)
echo 0 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo
```

### Device Power Management

```bash
# USB autosuspend
echo auto | sudo tee /sys/bus/usb/devices/*/power/level

# SATA power management
echo min_power | sudo tee /sys/class/scsi_host/host*/link_power_management_policy

# Audio power saving
echo 1 | sudo tee /sys/module/snd_hda_intel/parameters/power_save
```

---

## Benchmarking

### System Benchmarks

```bash
# Phoronix Test Suite
vantis-pkg install phoronix-test-suite
phoronix-test-suite benchmark

# VantisOS benchmark suite
vbench run --all

# Quick benchmark
vbench quick
```

### CPU Benchmarks

```bash
# Sysbench CPU
sysbench cpu --cpu-max-prime=20000 run

# Stress test
stress-ng --cpu 4 --timeout 60s

# Geekbench
geekbench5
```

### Memory Benchmarks

```bash
# Stream benchmark
stream-bench

# Sysbench memory
sysbench memory run

# VantisOS memory benchmark
vbench memory
```

### Disk Benchmarks

```bash
# dd test
dd if=/dev/zero of=test bs=1M count=1024 conv=fdatasync

# fio comprehensive test
fio --name=test --ioengine=libaio --iodepth=32 --rw=randrw --bs=4k --direct=1 --size=1G --numjobs=8

# VantisOS disk benchmark
vbench disk /dev/sda
```

### Network Benchmarks

```bash
# iperf3
iperf3 -s  # Server
iperf3 -c <server>  # Client

# netperf
netperf -H <server>

# VantisOS network benchmark
vbench network
```

### Results Storage

```bash
# Save benchmark results
vbench run --all --output benchmarks.json

# Compare results
vbench compare benchmarks-old.json benchmarks-new.json

# Upload to VantisOS database
vbench upload --anonymous
```

---

## Performance Troubleshooting

### Slow Boot

```bash
# Analyze boot time
systemd-analyze time

# Boot chart
systemd-analyze plot > boot-chart.svg

# Critical services
systemd-analyze critical-chain

# Disable slow services
sudo systemctl disable <service>
```

### High CPU Usage

```bash
# Find CPU hogs
top -n 1 | head -20

# Detailed analysis
perf top

# Check for runaway processes
ps aux --sort=-%cpu | head
```

### High Memory Usage

```bash
# Memory analysis
free -h
ps aux --sort=-%mem | head

# Find memory leaks
valgrind --leak-check=full <application>

# Clear caches
sudo sync; echo 3 | sudo tee /proc/sys/vm/drop_caches
```

### Slow Disk I/O

```bash
# I/O statistics
iostat -x 1

# Find I/O heavy processes
iotop

# Check disk health
sudo smartctl -a /dev/sda
```

---

*Last updated: March 2025 | VantisOS v1.4.0*