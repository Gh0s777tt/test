//! VantisOS System Monitor
//! 
//! A real-time system monitoring application with graphs and process management.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

/// System monitor application
pub struct SystemMonitor {
    /// Configuration
    config: MonitorConfig,
    /// CPU monitor
    cpu: CpuMonitor,
    /// Memory monitor
    memory: MemoryMonitor,
    /// Disk monitor
    disk: DiskMonitor,
    /// Network monitor
    network: NetworkMonitor,
    /// Process manager
    processes: ProcessManager,
    /// Update interval
    update_interval: Duration,
    /// Last update
    last_update: Option<Instant>,
}

/// Monitor configuration
#[derive(Clone, Debug)]
pub struct MonitorConfig {
    /// Update interval in milliseconds
    pub update_interval_ms: u64,
    /// Graph history size (data points)
    pub graph_history_size: usize,
    /// Show temperatures
    pub show_temperatures: bool,
    /// Show network addresses
    pub show_network_addresses: bool,
    /// Show disk mount points
    pub show_mount_points: bool,
    /// Temperature unit (Celsius/Fahrenheit)
    pub temperature_unit: TemperatureUnit,
    /// Network speed unit (bits/bytes)
    pub network_unit: NetworkUnit,
    /// Show process icons
    pub show_process_icons: bool,
    /// Confirm kill process
    pub confirm_kill: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            update_interval_ms: 1000,
            graph_history_size: 60,
            show_temperatures: true,
            show_network_addresses: true,
            show_mount_points: true,
            temperature_unit: TemperatureUnit::Celsius,
            network_unit: NetworkUnit::Bytes,
            show_process_icons: true,
            confirm_kill: true,
        }
    }
}

/// Temperature unit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// Network speed unit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkUnit {
    Bits,
    Bytes,
}

/// CPU Monitor
#[derive(Clone, Debug)]
pub struct CpuMonitor {
    /// CPU cores
    pub cores: Vec<CpuCore>,
    /// Total CPU usage history
    pub total_history: VecDeque<f32>,
    /// Per-core history
    pub core_history: Vec<VecDeque<f32>>,
    /// CPU frequency (MHz)
    pub frequency: CpuFrequency,
    /// CPU temperatures
    pub temperatures: Vec<Temperature>,
    /// CPU info
    pub info: CpuInfo,
}

/// CPU core
#[derive(Clone, Debug)]
pub struct CpuCore {
    /// Core ID
    pub id: usize,
    /// Usage percentage (0-100)
    pub usage: f32,
    /// User time
    pub user: u64,
    /// System time
    pub system: u64,
    /// Idle time
    pub idle: u64,
    /// Total time
    pub total: u64,
}

/// CPU frequency
#[derive(Clone, Debug)]
pub struct CpuFrequency {
    /// Current frequency in MHz
    pub current: u32,
    /// Minimum frequency
    pub min: u32,
    /// Maximum frequency
    pub max: u32,
    /// Governor
    pub governor: String,
}

/// Temperature reading
#[derive(Clone, Debug)]
pub struct Temperature {
    /// Sensor name
    pub name: String,
    /// Temperature in Celsius
    pub celsius: f32,
    /// Critical temperature
    pub critical: Option<f32>,
    /// Maximum temperature
    pub maximum: Option<f32>,
}

/// CPU information
#[derive(Clone, Debug)]
pub struct CpuInfo {
    /// Model name
    pub model: String,
    /// Vendor
    pub vendor: String,
    /// Architecture
    pub arch: String,
    /// Core count
    pub cores: usize,
    /// Thread count
    pub threads: usize,
    /// Cache size (KB)
    pub cache_size: u32,
    /// Clock speed (MHz)
    pub clock_speed: u32,
}

impl CpuMonitor {
    pub fn new(core_count: usize, history_size: usize) -> Self {
        let mut cores = Vec::with_capacity(core_count);
        for i in 0..core_count {
            cores.push(CpuCore {
                id: i,
                usage: 0.0,
                user: 0,
                system: 0,
                idle: 0,
                total: 0,
            });
        }
        
        let core_history = vec![VecDeque::with_capacity(history_size); core_count];
        
        Self {
            cores,
            total_history: VecDeque::with_capacity(history_size),
            core_history,
            frequency: CpuFrequency {
                current: 0,
                min: 0,
                max: 0,
                governor: "unknown".to_string(),
            },
            temperatures: Vec::new(),
            info: CpuInfo {
                model: "Unknown".to_string(),
                vendor: "Unknown".to_string(),
                arch: "Unknown".to_string(),
                cores: core_count,
                threads: core_count,
                cache_size: 0,
                clock_speed: 0,
            },
        }
    }
    
    /// Get average CPU usage
    pub fn average_usage(&self) -> f32 {
        if self.cores.is_empty() {
            return 0.0;
        }
        self.cores.iter().map(|c| c.usage).sum::<f32>() / self.cores.len() as f32
    }
}

/// Memory Monitor
#[derive(Clone, Debug)]
pub struct MemoryMonitor {
    /// Total memory in bytes
    pub total: u64,
    /// Used memory in bytes
    pub used: u64,
    /// Free memory in bytes
    pub free: u64,
    /// Available memory in bytes
    pub available: u64,
    /// Buffers in bytes
    pub buffers: u64,
    /// Cached in bytes
    pub cached: u64,
    /// Swap total in bytes
    pub swap_total: u64,
    /// Swap used in bytes
    pub swap_used: u64,
    /// Usage history
    pub history: VecDeque<f32>,
    /// Swap history
    pub swap_history: VecDeque<f32>,
}

impl MemoryMonitor {
    pub fn new(history_size: usize) -> Self {
        Self {
            total: 0,
            used: 0,
            free: 0,
            available: 0,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_used: 0,
            history: VecDeque::with_capacity(history_size),
            swap_history: VecDeque::with_capacity(history_size),
        }
    }
    
    /// Get memory usage percentage
    pub fn usage_percent(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64 * 100.0) as f32
    }
    
    /// Get swap usage percentage
    pub fn swap_percent(&self) -> f32 {
        if self.swap_total == 0 {
            return 0.0;
        }
        (self.swap_used as f64 / self.swap_total as f64 * 100.0) as f32
    }
    
    /// Format bytes to human readable
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;
        
        if bytes >= TB {
            format!("{:.2} TiB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GiB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MiB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KiB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}

/// Disk Monitor
#[derive(Clone, Debug)]
pub struct DiskMonitor {
    /// Disk devices
    pub disks: Vec<Disk>,
    /// I/O history
    pub io_history: VecDeque<IoStats>,
}

/// Disk information
#[derive(Clone, Debug)]
pub struct Disk {
    /// Device name
    pub name: String,
    /// Mount point
    pub mount_point: String,
    /// File system type
    pub fs_type: String,
    /// Total space in bytes
    pub total: u64,
    /// Used space in bytes
    pub used: u64,
    /// Free space in bytes
    pub free: u64,
    /// Read speed (bytes/sec)
    pub read_speed: u64,
    /// Write speed (bytes/sec)
    pub write_speed: u64,
    /// Total bytes read
    pub total_read: u64,
    /// Total bytes written
    pub total_written: u64,
    /// Is removable
    pub removable: bool,
    /// Is system disk
    pub system: bool,
}

impl Disk {
    /// Get usage percentage
    pub fn usage_percent(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64 * 100.0) as f32
    }
}

/// I/O statistics
#[derive(Clone, Debug, Default)]
pub struct IoStats {
    /// Read speed (bytes/sec)
    pub read_speed: u64,
    /// Write speed (bytes/sec)
    pub write_speed: u64,
    /// IOPS
    pub iops: u64,
}

impl DiskMonitor {
    pub fn new(history_size: usize) -> Self {
        Self {
            disks: Vec::new(),
            io_history: VecDeque::with_capacity(history_size),
        }
    }
}

/// Network Monitor
#[derive(Clone, Debug)]
pub struct NetworkMonitor {
    /// Network interfaces
    pub interfaces: Vec<NetworkInterface>,
    /// Total download history
    pub download_history: VecDeque<u64>,
    /// Total upload history
    pub upload_history: VecDeque<u64>,
    /// Total bytes received
    pub total_received: u64,
    /// Total bytes sent
    pub total_sent: u64,
}

/// Network interface
#[derive(Clone, Debug)]
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// MAC address
    pub mac: String,
    /// IPv4 addresses
    pub ipv4: Vec<String>,
    /// IPv6 addresses
    pub ipv6: Vec<String>,
    /// Is up
    pub is_up: bool,
    /// Is loopback
    pub is_loopback: bool,
    /// Receive speed (bytes/sec)
    pub rx_speed: u64,
    /// Transmit speed (bytes/sec)
    pub tx_speed: u64,
    /// Total received
    pub total_rx: u64,
    /// Total transmitted
    pub total_tx: u64,
    /// Signal strength (for wireless, 0-100)
    pub signal_strength: Option<u8>,
    /// SSID (for wireless)
    pub ssid: Option<String>,
}

impl NetworkMonitor {
    pub fn new(history_size: usize) -> Self {
        Self {
            interfaces: Vec::new(),
            download_history: VecDeque::with_capacity(history_size),
            upload_history: VecDeque::with_capacity(history_size),
            total_received: 0,
            total_sent: 0,
        }
    }
    
    /// Get total download speed
    pub fn total_download_speed(&self) -> u64 {
        self.interfaces.iter().map(|i| i.rx_speed).sum()
    }
    
    /// Get total upload speed
    pub fn total_upload_speed(&self) -> u64 {
        self.interfaces.iter().map(|i| i.tx_speed).sum()
    }
}

/// Process Manager
#[derive(Clone, Debug)]
pub struct ProcessManager {
    /// Process list
    pub processes: Vec<Process>,
    /// Sort column
    pub sort_by: ProcessSort,
    /// Sort descending
    pub sort_descending: bool,
    /// Filter query
    pub filter: Option<String>,
    /// Selected process PID
    pub selected: Option<u32>,
    /// Total process count
    pub total_count: usize,
    /// Running count
    pub running_count: usize,
    /// Sleeping count
    pub sleeping_count: usize,
    /// Zombie count
    pub zombie_count: usize,
}

/// Process information
#[derive(Clone, Debug)]
pub struct Process {
    /// Process ID
    pub pid: u32,
    /// Parent PID
    pub ppid: u32,
    /// Process name
    pub name: String,
    /// Command line
    pub cmd: String,
    /// User name
    pub user: String,
    /// Process state
    pub state: ProcessState,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory: u64,
    /// Memory percentage
    pub memory_percent: f32,
    /// Virtual memory
    pub virtual_memory: u64,
    /// Shared memory
    pub shared_memory: u64,
    /// Priority
    pub priority: i32,
    /// Nice value
    pub nice: i32,
    /// Number of threads
    pub threads: u32,
    /// Start time
    pub start_time: SystemTime,
    /// CPU time (user + system)
    pub cpu_time: Duration,
}

/// Process state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    Zombie,
    Dead,
    Unknown,
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "Running"),
            Self::Sleeping => write!(f, "Sleeping"),
            Self::DiskSleep => write!(f, "Disk Sleep"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Zombie => write!(f, "Zombie"),
            Self::Dead => write!(f, "Dead"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Process sort column
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessSort {
    Pid,
    Name,
    User,
    Cpu,
    Memory,
    Priority,
    State,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            sort_by: ProcessSort::Cpu,
            sort_descending: true,
            filter: None,
            selected: None,
            total_count: 0,
            running_count: 0,
            sleeping_count: 0,
            zombie_count: 0,
        }
    }
    
    /// Sort processes
    pub fn sort(&mut self) {
        self.processes.sort_by(|a, b| {
            let cmp = match self.sort_by {
                ProcessSort::Pid => a.pid.cmp(&b.pid),
                ProcessSort::Name => a.name.cmp(&b.name),
                ProcessSort::User => a.user.cmp(&b.user),
                ProcessSort::Cpu => a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
                ProcessSort::Memory => a.memory.cmp(&b.memory),
                ProcessSort::Priority => a.priority.cmp(&b.priority),
                ProcessSort::State => a.state.cmp(&b.state),
            };
            
            if self.sort_descending {
                cmp.reverse()
            } else {
                cmp
            }
        });
    }
    
    /// Filter processes
    pub fn apply_filter(&mut self) {
        if let Some(filter) = &self.filter {
            let filter = filter.to_lowercase();
            self.processes.retain(|p| {
                p.name.to_lowercase().contains(&filter) ||
                p.user.to_lowercase().contains(&filter) ||
                p.cmd.to_lowercase().contains(&filter)
            });
        }
    }
    
    /// Kill a process
    pub fn kill_process(&mut self, pid: u32, signal: ProcessSignal) -> Result<(), MonitorError> {
        // Find the process
        if !self.processes.iter().any(|p| p.pid == pid) {
            return Err(MonitorError::ProcessNotFound(pid));
        }
        
        // In a real implementation, this would send the signal to the process
        // For now, we just remove it from our list
        self.processes.retain(|p| p.pid != pid);
        
        Ok(())
    }
    
    /// Change process priority
    pub fn set_priority(&mut self, pid: u32, priority: i32) -> Result<(), MonitorError> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.priority = priority;
            Ok(())
        } else {
            Err(MonitorError::ProcessNotFound(pid))
        }
    }
}

/// Process signal
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessSignal {
    /// Terminate (SIGTERM)
    Terminate,
    /// Kill (SIGKILL)
    Kill,
    /// Interrupt (SIGINT)
    Interrupt,
    /// Stop (SIGSTOP)
    Stop,
    /// Continue (SIGCONT)
    Continue,
    /// Hangup (SIGHUP)
    Hangup,
}

/// Monitor errors
#[derive(Debug, Clone)]
pub enum MonitorError {
    /// Process not found
    ProcessNotFound(u32),
    /// Permission denied
    PermissionDenied(String),
    /// Read failed
    ReadFailed(String),
    /// Write failed
    WriteFailed(String),
    /// Invalid signal
    InvalidSignal(String),
}

impl std::fmt::Display for MonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MonitorError::ProcessNotFound(pid) => write!(f, "Process not found: {}", pid),
            MonitorError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            MonitorError::ReadFailed(msg) => write!(f, "Read failed: {}", msg),
            MonitorError::WriteFailed(msg) => write!(f, "Write failed: {}", msg),
            MonitorError::InvalidSignal(msg) => write!(f, "Invalid signal: {}", msg),
        }
    }
}

impl std::error::Error for MonitorError {}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        Self::with_config(MonitorConfig::default())
    }
    
    /// Create with configuration
    pub fn with_config(config: MonitorConfig) -> Self {
        let history_size = config.graph_history_size;
        
        Self {
            cpu: CpuMonitor::new(4, history_size), // Default 4 cores, will be updated
            memory: MemoryMonitor::new(history_size),
            disk: DiskMonitor::new(history_size),
            network: NetworkMonitor::new(history_size),
            processes: ProcessManager::new(),
            update_interval: Duration::from_millis(config.update_interval_ms),
            last_update: None,
            config,
        }
    }
    
    /// Update all monitors
    pub fn update(&mut self) {
        let now = Instant::now();
        
        // Check if we need to update
        if let Some(last) = self.last_update {
            if now.duration_since(last) < self.update_interval {
                return;
            }
        }
        
        self.last_update = Some(now);
        
        // Update CPU history
        let avg = self.cpu.average_usage();
        self.cpu.total_history.push_back(avg);
        if self.cpu.total_history.len() > self.config.graph_history_size {
            self.cpu.total_history.pop_front();
        }
        
        // Update memory history
        self.memory.history.push_back(self.memory.usage_percent());
        if self.memory.history.len() > self.config.graph_history_size {
            self.memory.history.pop_front();
        }
        
        self.memory.swap_history.push_back(self.memory.swap_percent());
        if self.memory.swap_history.len() > self.config.graph_history_size {
            self.memory.swap_history.pop_front();
        }
        
        // Update network history
        let total_rx = self.network.total_download_speed();
        let total_tx = self.network.total_upload_speed();
        
        self.network.download_history.push_back(total_rx);
        if self.network.download_history.len() > self.config.graph_history_size {
            self.network.download_history.pop_front();
        }
        
        self.network.upload_history.push_back(total_tx);
        if self.network.upload_history.len() > self.config.graph_history_size {
            self.network.upload_history.pop_front();
        }
        
        // Update disk I/O history
        let total_read: u64 = self.disk.disks.iter().map(|d| d.read_speed).sum();
        let total_write: u64 = self.disk.disks.iter().map(|d| d.write_speed).sum();
        
        self.disk.io_history.push_back(IoStats {
            read_speed: total_read,
            write_speed: total_write,
            iops: 0, // Would calculate from actual I/O stats
        });
        
        if self.disk.io_history.len() > self.config.graph_history_size {
            self.disk.io_history.pop_front();
        }
    }
    
    /// Get CPU monitor
    pub fn cpu(&self) -> &CpuMonitor {
        &self.cpu
    }
    
    /// Get CPU monitor mutable
    pub fn cpu_mut(&mut self) -> &mut CpuMonitor {
        &mut self.cpu
    }
    
    /// Get memory monitor
    pub fn memory(&self) -> &MemoryMonitor {
        &self.memory
    }
    
    /// Get memory monitor mutable
    pub fn memory_mut(&mut self) -> &mut MemoryMonitor {
        &mut self.memory
    }
    
    /// Get disk monitor
    pub fn disk(&self) -> &DiskMonitor {
        &self.disk
    }
    
    /// Get disk monitor mutable
    pub fn disk_mut(&mut self) -> &mut DiskMonitor {
        &mut self.disk
    }
    
    /// Get network monitor
    pub fn network(&self) -> &NetworkMonitor {
        &self.network
    }
    
    /// Get network monitor mutable
    pub fn network_mut(&mut self) -> &mut NetworkMonitor {
        &mut self.network
    }
    
    /// Get process manager
    pub fn processes(&self) -> &ProcessManager {
        &self.processes
    }
    
    /// Get process manager mutable
    pub fn processes_mut(&mut self) -> &mut ProcessManager {
        &mut self.processes
    }
    
    /// Get configuration
    pub fn config(&self) -> &MonitorConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: MonitorConfig) {
        self.update_interval = Duration::from_millis(config.update_interval_ms);
        self.config = config;
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}