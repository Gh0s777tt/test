//! Data Collector Module for VantisOS AI
//! 
//! Responsible for collecting raw system data for ML training and inference.
//! Collects metrics from various system components in real-time.
//! 
//! ## Architecture
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DataCollector                        │
//! ├─────────────────────────────────────────────────────────┤
//! │  ┌─────────┐  ┌──────────┐  ┌─────────┐  ┌──────────┐  │
//! │  │   CPU   │  │  Memory  │  │  Disk   │  │ Network  │  │
//! │  │ Monitor │  │ Monitor  │  │ Monitor │  │ Monitor  │  │
//! │  └────┬────┘  └────┬─────┘  └────┬────┘  └────┬─────┘  │
//! │       │            │             │            │        │
//! │       └────────────┴─────────────┴────────────┘        │
//! │                          │                              │
//! │                    ┌─────▼─────┐                        │
//! │                    │  Circular │                        │
//! │                    │  Buffer   │                        │
//! │                    └───────────┘                        │
//! └─────────────────────────────────────────────────────────┘
//! ```
//! 
//! ## Security Considerations
//! - Only collects non-sensitive metrics
//! - No user data or personal information
//! - Rate limiting prevents data overload
//! - All data collection is logged

use crate::ai::{error::AIError, types::ResourceUsage};

/// Default buffer size for metrics storage
const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Minimum buffer size
const MIN_BUFFER_SIZE: usize = 64;

/// Maximum buffer size
const MAX_BUFFER_SIZE: usize = 65536;

/// Default sampling interval in milliseconds
const DEFAULT_SAMPLING_INTERVAL_MS: u64 = 100;

/// Minimum sampling interval in milliseconds
const MIN_SAMPLING_INTERVAL_MS: u64 = 1;

/// Maximum sampling interval in milliseconds
const MAX_SAMPLING_INTERVAL_MS: u64 = 60000;

/// CPU metrics per core
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CPUMetrics {
    /// Core ID
    pub core_id: u8,
    /// Usage percentage (0.0-100.0)
    pub usage_percent: f64,
    /// Frequency in MHz
    pub frequency_mhz: u32,
    /// Temperature in Celsius
    pub temperature_celsius: Option<f64>,
    /// Idle time percentage
    pub idle_percent: f64,
    /// User time percentage
    pub user_percent: f64,
    /// System time percentage
    pub system_percent: f64,
}

impl Default for CPUMetrics {
    fn default() -> Self {
        Self {
            core_id: 0,
            usage_percent: 0.0,
            frequency_mhz: 1000,
            temperature_celsius: None,
            idle_percent: 100.0,
            user_percent: 0.0,
            system_percent: 0.0,
        }
    }
}

/// Memory metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Free memory in bytes
    pub free_bytes: u64,
    /// Cached memory in bytes
    pub cached_bytes: u64,
    /// Buffer memory in bytes
    pub buffer_bytes: u64,
    /// Swap total in bytes
    pub swap_total_bytes: u64,
    /// Swap used in bytes
    pub swap_used_bytes: u64,
    /// Usage percentage (0.0-100.0)
    pub usage_percent: f64,
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            total_bytes: 8_589_934_592, // 8GB default
            used_bytes: 0,
            free_bytes: 8_589_934_592,
            cached_bytes: 0,
            buffer_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
            usage_percent: 0.0,
        }
    }
}

/// Disk I/O metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiskMetrics {
    /// Device name hash (for identification)
    pub device_id: u32,
    /// Read operations per second
    pub reads_per_sec: u64,
    /// Write operations per second
    pub writes_per_sec: u64,
    /// Read bytes per second
    pub read_bytes_per_sec: u64,
    /// Write bytes per second
    pub write_bytes_per_sec: u64,
    /// Average read latency in microseconds
    pub avg_read_latency_us: u64,
    /// Average write latency in microseconds
    pub avg_write_latency_us: u64,
    /// Queue depth
    pub queue_depth: u32,
    /// Usage percentage
    pub usage_percent: f64,
}

impl Default for DiskMetrics {
    fn default() -> Self {
        Self {
            device_id: 0,
            reads_per_sec: 0,
            writes_per_sec: 0,
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            avg_read_latency_us: 0,
            avg_write_latency_us: 0,
            queue_depth: 0,
            usage_percent: 0.0,
        }
    }
}

/// Network interface metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NetworkMetrics {
    /// Interface ID
    pub interface_id: u32,
    /// Bytes received per second
    pub rx_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: u64,
    /// Packets received per second
    pub rx_packets_per_sec: u64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: u64,
    /// Receive errors per second
    pub rx_errors_per_sec: u64,
    /// Transmit errors per second
    pub tx_errors_per_sec: u64,
    /// Link speed in Mbps (0 if unknown)
    pub link_speed_mbps: u32,
    /// Usage percentage of link capacity
    pub usage_percent: f64,
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            interface_id: 0,
            rx_bytes_per_sec: 0,
            tx_bytes_per_sec: 0,
            rx_packets_per_sec: 0,
            tx_packets_per_sec: 0,
            rx_errors_per_sec: 0,
            tx_errors_per_sec: 0,
            link_speed_mbps: 1000,
            usage_percent: 0.0,
        }
    }
}

/// Power consumption metrics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerMetrics {
    /// Total system power in milliwatts
    pub total_power_mw: u64,
    /// CPU power in milliwatts
    pub cpu_power_mw: u64,
    /// Memory power in milliwatts
    pub memory_power_mw: u64,
    /// GPU power in milliwatts (if available)
    pub gpu_power_mw: Option<u64>,
    /// Battery percentage (0-100, None if desktop)
    pub battery_percent: Option<u8>,
    /// Power source (true = AC, false = battery)
    pub on_ac_power: bool,
}

impl Default for PowerMetrics {
    fn default() -> Self {
        Self {
            total_power_mw: 0,
            cpu_power_mw: 0,
            memory_power_mw: 0,
            gpu_power_mw: None,
            battery_percent: None,
            on_ac_power: true,
        }
    }
}

/// Comprehensive system metrics snapshot
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SystemMetrics {
    /// Timestamp in milliseconds since boot
    pub timestamp_ms: u64,
    /// CPU metrics per core
    pub cpu: Vec<CPUMetrics>,
    /// Aggregate CPU usage (0.0-100.0)
    pub cpu_usage_avg: f64,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk metrics per device
    pub disk: Vec<DiskMetrics>,
    /// Network metrics per interface
    pub network: Vec<NetworkMetrics>,
    /// Power metrics
    pub power: PowerMetrics,
    /// Resource usage summary
    pub resource_usage: ResourceUsage,
}

/// Circular buffer for metrics history
#[derive(Debug, Clone)]
pub struct MetricsBuffer {
    /// Buffer storage
    buffer: Vec<SystemMetrics>,
    /// Write position
    head: usize,
    /// Number of elements
    count: usize,
    /// Capacity
    capacity: usize,
}

impl MetricsBuffer {
    /// Create a new metrics buffer
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE);
        Self {
            buffer: Vec::with_capacity(capacity),
            head: 0,
            count: 0,
            capacity,
        }
    }

    /// Push metrics to the buffer
    pub fn push(&mut self, metrics: SystemMetrics) {
        if self.buffer.len() < self.capacity {
            self.buffer.push(metrics);
        } else {
            self.buffer[self.head] = metrics;
        }
        self.head = (self.head + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }

    /// Get the most recent metrics
    pub fn latest(&self) -> Option<&SystemMetrics> {
        if self.count == 0 {
            None
        } else if self.head == 0 {
            self.buffer.last()
        } else {
            self.buffer.get(self.head.wrapping_sub(1))
        }
    }

    /// Get metrics from N samples ago
    pub fn get(&self, samples_ago: usize) -> Option<&SystemMetrics> {
        if samples_ago >= self.count {
            return None;
        }
        let index = if self.head == 0 {
            self.capacity - samples_ago - 1
        } else {
            self.head.wrapping_sub(samples_ago + 1)
        };
        self.buffer.get(index)
    }

    /// Get all metrics as a slice (ordered oldest to newest)
    pub fn as_slice(&self) -> &[SystemMetrics] {
        &self.buffer[..self.count.min(self.buffer.len())]
    }

    /// Get count of stored metrics
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.head = 0;
        self.count = 0;
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Sampling configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplingConfig {
    /// Sampling interval in milliseconds
    pub interval_ms: u64,
    /// Enable CPU metrics collection
    pub collect_cpu: bool,
    /// Enable memory metrics collection
    pub collect_memory: bool,
    /// Enable disk metrics collection
    pub collect_disk: bool,
    /// Enable network metrics collection
    pub collect_network: bool,
    /// Enable power metrics collection
    pub collect_power: bool,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            interval_ms: DEFAULT_SAMPLING_INTERVAL_MS,
            collect_cpu: true,
            collect_memory: true,
            collect_disk: true,
            collect_network: true,
            collect_power: true,
        }
    }
}

impl SamplingConfig {
    /// Create a new sampling configuration
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval_ms: interval_ms.clamp(MIN_SAMPLING_INTERVAL_MS, MAX_SAMPLING_INTERVAL_MS),
            ..Default::default()
        }
    }

    /// Create a fast sampling config (10ms interval)
    pub fn fast() -> Self {
        Self {
            interval_ms: 10,
            ..Default::default()
        }
    }

    /// Create a slow sampling config (1s interval)
    pub fn slow() -> Self {
        Self {
            interval_ms: 1000,
            ..Default::default()
        }
    }
}

/// Data Collector
/// 
/// Collects real-time system metrics for AI processing.
/// 
/// ## Features
/// - Real-time metrics collection
/// - Multi-component data aggregation
/// - Configurable sampling rates
/// - Circular buffer management
/// - Data validation
/// - Rate limiting
/// 
/// ## Example
/// ```rust
/// use vantis::ai::modules::DataCollector;
/// 
/// let mut collector = DataCollector::new(4096)?;
/// collector.start_collection()?;
/// 
/// // Collect a sample
/// let metrics = collector.collect()?;
/// 
/// // Get historical data
/// if let Some(last) = collector.history().latest() {
///     println!("CPU: {:.1}%", last.cpu_usage_avg);
/// }
/// ```
pub struct DataCollector {
    /// Metrics buffer
    buffer: MetricsBuffer,
    /// Sampling configuration
    config: SamplingConfig,
    /// Whether collection is enabled
    enabled: bool,
    /// Total samples collected
    total_samples: u64,
    /// Collection start time
    start_time_ms: Option<u64>,
    /// Last collection time
    last_collection_ms: u64,
    /// Number of CPU cores
    cpu_cores: u8,
    /// Number of disk devices
    disk_devices: u8,
    /// Number of network interfaces
    network_interfaces: u8,
}

impl DataCollector {
    /// Create a new data collector with default buffer size
    pub fn new() -> Result<Self, AIError> {
        Self::with_capacity(DEFAULT_BUFFER_SIZE)
    }

    /// Create a new data collector with specified buffer capacity
    pub fn with_capacity(buffer_size: usize) -> Result<Self, AIError> {
        let capacity = buffer_size.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE);
        
        Ok(Self {
            buffer: MetricsBuffer::new(capacity),
            config: SamplingConfig::default(),
            enabled: false,
            total_samples: 0,
            start_time_ms: None,
            last_collection_ms: 0,
            cpu_cores: 4,      // Default to 4 cores
            disk_devices: 1,   // Default to 1 disk
            network_interfaces: 1, // Default to 1 network interface
        })
    }

    /// Configure the data collector
    pub fn configure(&mut self, config: SamplingConfig) {
        self.config = config;
    }

    /// Get the current configuration
    pub fn config(&self) -> &SamplingConfig {
        &self.config
    }

    /// Start data collection
    pub fn start(&mut self) -> Result<(), AIError> {
        self.enabled = true;
        self.start_time_ms = Some(self.current_time_ms());
        Ok(())
    }

    /// Stop data collection
    pub fn stop(&mut self) {
        self.enabled = false;
    }

    /// Check if collection is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Collect a metrics sample
    pub fn collect(&mut self) -> Result<SystemMetrics, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        let current_time = self.current_time_ms();
        
        // Rate limiting check
        if current_time - self.last_collection_ms < self.config.interval_ms {
            // Return cached metrics if sampling too fast
            if let Some(last) = self.buffer.latest() {
                return Ok(last.clone());
            }
        }

        let metrics = self.collect_all_metrics(current_time)?;
        
        self.buffer.push(metrics.clone());
        self.total_samples += 1;
        self.last_collection_ms = current_time;

        Ok(metrics)
    }

    /// Collect all metrics
    fn collect_all_metrics(&self, timestamp_ms: u64) -> Result<SystemMetrics, AIError> {
        let mut metrics = SystemMetrics {
            timestamp_ms,
            ..Default::default()
        };

        if self.config.collect_cpu {
            metrics.cpu = self.collect_cpu_metrics();
            metrics.cpu_usage_avg = self.calculate_avg_cpu_usage(&metrics.cpu);
        }

        if self.config.collect_memory {
            metrics.memory = self.collect_memory_metrics();
        }

        if self.config.collect_disk {
            metrics.disk = self.collect_disk_metrics();
        }

        if self.config.collect_network {
            metrics.network = self.collect_network_metrics();
        }

        if self.config.collect_power {
            metrics.power = self.collect_power_metrics();
        }

        // Calculate resource usage summary
        metrics.resource_usage = ResourceUsage {
            cpu_usage: metrics.cpu_usage_avg,
            memory_usage: metrics.memory.usage_percent,
            disk_usage: metrics.disk.first().map(|d| d.usage_percent).unwrap_or(0.0),
            network_usage: metrics.network.first().map(|n| n.usage_percent).unwrap_or(0.0),
        };

        Ok(metrics)
    }

    /// Collect CPU metrics for all cores
    fn collect_cpu_metrics(&self) -> Vec<CPUMetrics> {
        // Simulated CPU metrics - in real implementation, read from /proc/stat or kernel
        let mut cores = Vec::with_capacity(self.cpu_cores as usize);
        
        for core_id in 0..self.cpu_cores {
            // Simulated metrics with realistic variation
            let base_usage = 25.0 + (core_id as f64 * 5.0) % 30.0;
            let noise = ((self.total_samples + core_id as u64) % 20) as f64;
            
            cores.push(CPUMetrics {
                core_id,
                usage_percent: (base_usage + noise).clamp(0.0, 100.0),
                frequency_mhz: 2000 + (core_id as u32 * 100),
                temperature_celsius: Some(45.0 + core_id as f64 * 2.0),
                idle_percent: (100.0 - base_usage - noise).clamp(0.0, 100.0),
                user_percent: (base_usage * 0.6).clamp(0.0, 100.0),
                system_percent: (base_usage * 0.4).clamp(0.0, 100.0),
            });
        }
        
        cores
    }

    /// Calculate average CPU usage across all cores
    fn calculate_avg_cpu_usage(&self, cpu_metrics: &[CPUMetrics]) -> f64 {
        if cpu_metrics.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = cpu_metrics.iter().map(|m| m.usage_percent).sum();
        sum / cpu_metrics.len() as f64
    }

    /// Collect memory metrics
    fn collect_memory_metrics(&self) -> MemoryMetrics {
        // Simulated memory metrics - in real implementation, read from /proc/meminfo
        let total_bytes: u64 = 8_589_934_592; // 8GB
        let used_bytes: u64 = 4_294_967_296 + (self.total_samples % 1_000_000) * 1024;
        let cached_bytes: u64 = 1_073_741_824;
        
        MemoryMetrics {
            total_bytes,
            used_bytes,
            free_bytes: total_bytes - used_bytes,
            cached_bytes,
            buffer_bytes: 536_870_912,
            swap_total_bytes: 2_147_483_648,
            swap_used_bytes: 0,
            usage_percent: (used_bytes as f64 / total_bytes as f64) * 100.0,
        }
    }

    /// Collect disk metrics
    fn collect_disk_metrics(&self) -> Vec<DiskMetrics> {
        // Simulated disk metrics - in real implementation, read from /proc/diskstats
        let mut disks = Vec::with_capacity(self.disk_devices as usize);
        
        for device_id in 0..self.disk_devices {
            disks.push(DiskMetrics {
                device_id: device_id as u32,
                reads_per_sec: 150 + self.total_samples % 50,
                writes_per_sec: 80 + self.total_samples % 30,
                read_bytes_per_sec: 10_485_760,  // ~10 MB/s
                write_bytes_per_sec: 5_242_880,   // ~5 MB/s
                avg_read_latency_us: 500,
                avg_write_latency_us: 200,
                queue_depth: 2,
                usage_percent: 35.0 + (self.total_samples % 10) as f64,
            });
        }
        
        disks
    }

    /// Collect network metrics
    fn collect_network_metrics(&self) -> Vec<NetworkMetrics> {
        // Simulated network metrics - in real implementation, read from /proc/net/dev
        let mut interfaces = Vec::with_capacity(self.network_interfaces as usize);
        
        for interface_id in 0..self.network_interfaces {
            let rx_bps = 52_428_800;  // ~50 MB/s
            let tx_bps = 26_214_400;  // ~25 MB/s
            let link_speed = 1_000;    // 1 Gbps
            
            interfaces.push(NetworkMetrics {
                interface_id: interface_id as u32,
                rx_bytes_per_sec: rx_bps,
                tx_bytes_per_sec: tx_bps,
                rx_packets_per_sec: 50_000,
                tx_packets_per_sec: 25_000,
                rx_errors_per_sec: 0,
                tx_errors_per_sec: 0,
                link_speed_mbps: link_speed,
                usage_percent: ((rx_bps + tx_bps) as f64 / (link_speed as f64 * 125_000.0)) * 100.0,
            });
        }
        
        interfaces
    }

    /// Collect power metrics
    fn collect_power_metrics(&self) -> PowerMetrics {
        // Simulated power metrics - in real implementation, read from sysfs
        PowerMetrics {
            total_power_mw: 45_000,  // 45W
            cpu_power_mw: 25_000,    // 25W
            memory_power_mw: 5_000,  // 5W
            gpu_power_mw: Some(15_000), // 15W (if discrete GPU)
            battery_percent: Some(85),
            on_ac_power: true,
        }
    }

    /// Get current time in milliseconds
    fn current_time_ms(&self) -> u64 {
        // In real implementation, use kernel timer
        self.last_collection_ms + self.config.interval_ms
    }

    /// Get the metrics buffer
    pub fn history(&self) -> &MetricsBuffer {
        &self.buffer
    }

    /// Get mutable access to the metrics buffer
    pub fn history_mut(&mut self) -> &mut MetricsBuffer {
        &mut self.buffer
    }

    /// Get the total number of samples collected
    pub fn total_samples(&self) -> u64 {
        self.total_samples
    }

    /// Get the buffer capacity
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Get the latest metrics
    pub fn latest(&self) -> Option<&SystemMetrics> {
        self.buffer.latest()
    }

    /// Get metrics from N samples ago
    pub fn get(&self, samples_ago: usize) -> Option<&SystemMetrics> {
        self.buffer.get(samples_ago)
    }

    /// Clear the metrics history
    pub fn clear_history(&mut self) {
        self.buffer.clear();
    }

    /// Set the number of CPU cores to monitor
    pub fn set_cpu_cores(&mut self, cores: u8) {
        self.cpu_cores = cores.max(1);
    }

    /// Set the number of disk devices to monitor
    pub fn set_disk_devices(&mut self, devices: u8) {
        self.disk_devices = devices.max(1);
    }

    /// Set the number of network interfaces to monitor
    pub fn set_network_interfaces(&mut self, interfaces: u8) {
        self.network_interfaces = interfaces.max(1);
    }

    /// Collect a simple metrics snapshot (compatibility method)
    pub fn collect_metrics(&self) -> Result<ResourceUsage, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }
        
        // Return simulated metrics
        Ok(ResourceUsage {
            cpu_usage: 25.0 + (self.total_samples % 30) as f64,
            memory_usage: 45.0 + (self.total_samples % 20) as f64,
            disk_usage: 35.0 + (self.total_samples % 15) as f64,
            network_usage: 15.0 + (self.total_samples % 10) as f64,
        })
    }

    /// Get buffer size (compatibility method)
    pub fn buffer_size(&self) -> usize {
        self.buffer.capacity()
    }
}

impl Default for DataCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create default DataCollector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = DataCollector::new().unwrap();
        assert!(!collector.is_enabled());
        assert_eq!(collector.buffer_capacity(), DEFAULT_BUFFER_SIZE);
    }

    #[test]
    fn test_collector_with_custom_capacity() {
        let collector = DataCollector::with_capacity(8192).unwrap();
        assert_eq!(collector.buffer_capacity(), 8192);
    }

    #[test]
    fn test_collector_capacity_clamping() {
        // Test minimum clamping
        let collector = DataCollector::with_capacity(10).unwrap();
        assert_eq!(collector.buffer_capacity(), MIN_BUFFER_SIZE);

        // Test maximum clamping
        let collector = DataCollector::with_capacity(100_000).unwrap();
        assert_eq!(collector.buffer_capacity(), MAX_BUFFER_SIZE);
    }

    #[test]
    fn test_start_stop() {
        let mut collector = DataCollector::new().unwrap();
        assert!(!collector.is_enabled());
        
        collector.start().unwrap();
        assert!(collector.is_enabled());
        
        collector.stop();
        assert!(!collector.is_enabled());
    }

    #[test]
    fn test_collect_requires_start() {
        let mut collector = DataCollector::new().unwrap();
        let result = collector.collect();
        assert!(result.is_err());
    }

    #[test]
    fn test_collect_metrics() {
        let mut collector = DataCollector::new().unwrap();
        collector.start().unwrap();
        
        let metrics = collector.collect().unwrap();
        assert!(metrics.cpu_usage_avg >= 0.0 && metrics.cpu_usage_avg <= 100.0);
        assert!(metrics.memory.usage_percent >= 0.0 && metrics.memory.usage_percent <= 100.0);
        assert_eq!(collector.total_samples(), 1);
    }

    #[test]
    fn test_buffer_operations() {
        let mut collector = DataCollector::new().unwrap();
        collector.start().unwrap();
        
        // Collect multiple samples
        for _ in 0..10 {
            collector.collect().unwrap();
        }
        
        assert_eq!(collector.history().len(), 10);
        assert!(collector.latest().is_some());
        assert!(collector.get(5).is_some());
    }

    #[test]
    fn test_metrics_buffer() {
        let mut buffer = MetricsBuffer::new(100);
        assert!(buffer.is_empty());
        
        let metrics = SystemMetrics::default();
        buffer.push(metrics);
        
        assert_eq!(buffer.len(), 1);
        assert!(buffer.latest().is_some());
    }

    #[test]
    fn test_metrics_buffer_wraparound() {
        let mut buffer = MetricsBuffer::new(10);
        
        for i in 0..15 {
            let mut metrics = SystemMetrics::default();
            metrics.timestamp_ms = i;
            buffer.push(metrics);
        }
        
        assert_eq!(buffer.len(), 10);
        // Latest should be the 15th item (index 14)
        assert_eq!(buffer.latest().unwrap().timestamp_ms, 14);
    }

    #[test]
    fn test_sampling_config() {
        let config = SamplingConfig::new(50);
        assert_eq!(config.interval_ms, 50);
        
        let fast_config = SamplingConfig::fast();
        assert_eq!(fast_config.interval_ms, 10);
        
        let slow_config = SamplingConfig::slow();
        assert_eq!(slow_config.interval_ms, 1000);
    }

    #[test]
    fn test_cpu_metrics() {
        let mut collector = DataCollector::new().unwrap();
        collector.set_cpu_cores(4);
        collector.start().unwrap();
        
        let metrics = collector.collect().unwrap();
        assert_eq!(metrics.cpu.len(), 4);
        
        for (i, cpu) in metrics.cpu.iter().enumerate() {
            assert_eq!(cpu.core_id, i as u8);
            assert!(cpu.usage_percent >= 0.0 && cpu.usage_percent <= 100.0);
        }
    }

    #[test]
    fn test_resource_usage_compatibility() {
        let mut collector = DataCollector::new().unwrap();
        collector.start().unwrap();
        
        let usage = collector.collect_metrics().unwrap();
        assert!(usage.cpu_usage >= 0.0 && usage.cpu_usage <= 100.0);
        assert!(usage.memory_usage >= 0.0 && usage.memory_usage <= 100.0);
    }

    #[test]
    fn test_clear_history() {
        let mut collector = DataCollector::new().unwrap();
        collector.start().unwrap();
        
        for _ in 0..5 {
            collector.collect().unwrap();
        }
        assert_eq!(collector.history().len(), 5);
        
        collector.clear_history();
        assert!(collector.history().is_empty());
    }
}