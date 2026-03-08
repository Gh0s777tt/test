//! System Profiler for Performance Optimization
//! 
//! Comprehensive profiling of system resources and AI module performance.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

/// System profiler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// Profiling interval in milliseconds
    pub interval_ms: u64,
    /// Enable CPU profiling
    pub profile_cpu: bool,
    /// Enable memory profiling
    pub profile_memory: bool,
    /// Enable GPU profiling
    pub profile_gpu: bool,
    /// Enable I/O profiling
    pub profile_io: bool,
    /// Enable network profiling
    pub profile_network: bool,
    /// History buffer size
    pub history_size: usize,
    /// Enable flame graph generation
    pub enable_flame_graphs: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            interval_ms: 100,
            profile_cpu: true,
            profile_memory: true,
            profile_gpu: true,
            profile_io: true,
            profile_network: true,
            history_size: 1000,
            enable_flame_graphs: false,
        }
    }
}

/// CPU profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfile {
    /// Total CPU usage percentage (0.0 - 100.0)
    pub total_usage: f64,
    /// Per-core usage percentages
    pub core_usage: Vec<f64>,
    /// User mode percentage
    pub user_percent: f64,
    /// System/kernel mode percentage
    pub system_percent: f64,
    /// Idle percentage
    pub idle_percent: f64,
    /// I/O wait percentage
    pub iowait_percent: f64,
    /// Current frequency per core in MHz
    pub core_frequencies: Vec<u32>,
    /// Temperature per core in Celsius
    pub core_temperatures: Vec<f32>,
    /// Context switches per second
    pub context_switches: u64,
    /// Interrupts per second
    pub interrupts: u64,
}

/// Memory profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    /// Total memory in bytes
    pub total: u64,
    /// Used memory in bytes
    pub used: u64,
    /// Available memory in bytes
    pub available: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Cached memory in bytes
    pub cached: u64,
    /// Buffer memory in bytes
    pub buffers: u64,
    /// Shared memory in bytes
    pub shared: u64,
    /// Swap total in bytes
    pub swap_total: u64,
    /// Swap used in bytes
    pub swap_used: u64,
    /// Page faults per second
    pub page_faults: u64,
    /// Major page faults per second
    pub major_faults: u64,
}

/// GPU profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProfile {
    /// GPU utilization percentage
    pub gpu_usage: f64,
    /// Memory utilization percentage
    pub memory_usage: f64,
    /// Total GPU memory in bytes
    pub memory_total: u64,
    /// Used GPU memory in bytes
    pub memory_used: u64,
    /// GPU temperature in Celsius
    pub temperature: f32,
    /// Power draw in watts
    pub power_draw: f32,
    /// Power limit in watts
    pub power_limit: f32,
    /// Clock frequency in MHz
    pub clock_frequency: u32,
    /// Memory clock frequency in MHz
    pub memory_frequency: u32,
    /// Encoder utilization percentage
    pub encoder_usage: f64,
    /// Decoder utilization percentage
    pub decoder_usage: f64,
}

/// I/O profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoProfile {
    /// Read operations per second
    pub read_ops_per_sec: u64,
    /// Write operations per second
    pub write_ops_per_sec: u64,
    /// Bytes read per second
    pub read_bytes_per_sec: u64,
    /// Bytes written per second
    pub write_bytes_per_sec: u64,
    /// Average read latency in microseconds
    pub read_latency_us: u64,
    /// Average write latency in microseconds
    pub write_latency_us: u64,
    /// I/O queue depth
    pub queue_depth: u64,
    /// Disk utilization percentage
    pub disk_utilization: f64,
}

/// Network profile data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
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
    /// TCP connections established
    pub tcp_connections: u64,
    /// UDP connections
    pub udp_connections: u64,
    /// Network interface utilization percentage
    pub interface_utilization: f64,
}

/// AI Module performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetrics {
    /// Module name
    pub name: String,
    /// CPU time consumed
    pub cpu_time_ms: u64,
    /// Memory allocated
    pub memory_bytes: u64,
    /// Inference count
    pub inference_count: u64,
    /// Average inference latency
    pub avg_latency_ms: f64,
    /// 95th percentile latency
    pub p95_latency_ms: f64,
    /// 99th percentile latency
    pub p99_latency_ms: f64,
    /// Throughput (operations per second)
    pub throughput: f64,
    /// Error count
    pub error_count: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Complete system profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemProfile {
    /// Profile timestamp
    pub timestamp: u64,
    /// CPU profile
    pub cpu: CpuProfile,
    /// Memory profile
    pub memory: MemoryProfile,
    /// GPU profile (if available)
    pub gpu: Option<GpuProfile>,
    /// I/O profile
    pub io: IoProfile,
    /// Network profile
    pub network: NetworkProfile,
    /// AI module metrics
    pub modules: HashMap<String, ModuleMetrics>,
    /// System uptime in seconds
    pub uptime_secs: u64,
    /// Load average (1, 5, 15 min)
    pub load_average: (f64, f64, f64),
}

/// Profiling session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingSession {
    /// Session ID
    pub session_id: String,
    /// Start time
    pub start_time: u64,
    /// Duration in seconds
    pub duration_secs: u64,
    /// Number of samples collected
    pub samples_collected: u64,
    /// Profiles collected
    pub profiles: Vec<SystemProfile>,
    /// Summary statistics
    pub summary: ProfileSummary,
}

/// Summary statistics for a profiling session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummary {
    /// Average CPU usage
    pub avg_cpu_usage: f64,
    /// Peak CPU usage
    pub peak_cpu_usage: f64,
    /// Average memory usage
    pub avg_memory_usage: f64,
    /// Peak memory usage
    pub peak_memory_usage: f64,
    /// Average GPU usage
    pub avg_gpu_usage: f64,
    /// Peak GPU usage
    pub peak_gpu_usage: f64,
    /// Total I/O operations
    pub total_io_ops: u64,
    /// Total network bytes
    pub total_network_bytes: u64,
    /// Bottlenecks identified
    pub bottlenecks: Vec<Bottleneck>,
}

/// Identified bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Bottleneck type
    pub bottleneck_type: BottleneckType,
    /// Severity (1-10)
    pub severity: u8,
    /// Description
    pub description: String,
    /// Affected component
    pub component: String,
    /// Suggested resolution
    pub suggestion: String,
}

/// Types of bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    CpuBound,
    MemoryBound,
    GpuBound,
    IoBound,
    NetworkBound,
    LockContention,
    ThreadStarvation,
    MemoryLeak,
    CacheMiss,
    ThermalThrottling,
}

/// System profiler
pub struct SystemProfiler {
    config: ProfilerConfig,
    profile_history: Arc<RwLock<Vec<SystemProfile>>>,
    active_session: Arc<RwLock<Option<ProfilingSession>>>,
    running: Arc<RwLock<bool>>,
}

impl SystemProfiler {
    /// Create a new system profiler
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            config,
            profile_history: Arc::new(RwLock::new(Vec::new())),
            active_session: Arc::new(RwLock::new(None)),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start profiling
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut running = self.running.write().await;
        if *running {
            return Err("Profiler already running".into());
        }
        *running = true;
        
        // Initialize profiling session
        let session = ProfilingSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            duration_secs: 0,
            samples_collected: 0,
            profiles: Vec::new(),
            summary: ProfileSummary {
                avg_cpu_usage: 0.0,
                peak_cpu_usage: 0.0,
                avg_memory_usage: 0.0,
                peak_memory_usage: 0.0,
                avg_gpu_usage: 0.0,
                peak_gpu_usage: 0.0,
                total_io_ops: 0,
                total_network_bytes: 0,
                bottlenecks: Vec::new(),
            },
        };
        
        let mut active_session = self.active_session.write().await;
        *active_session = Some(session);
        
        Ok(())
    }

    /// Stop profiling
    pub async fn stop(&self) -> Result<ProfilingSession, Box<dyn std::error::Error>> {
        let mut running = self.running.write().await;
        if !*running {
            return Err("Profiler not running".into());
        }
        *running = false;
        
        let mut active_session = self.active_session.write().await;
        let session = active_session.take().ok_or("No active session")?;
        
        Ok(session)
    }

    /// Collect a single profile sample
    pub async fn collect_sample(&self) -> Result<SystemProfile, Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let cpu = self.profile_cpu().await?;
        let memory = self.profile_memory().await?;
        let gpu = if self.config.profile_gpu {
            Some(self.profile_gpu().await?)
        } else {
            None
        };
        let io = self.profile_io().await?;
        let network = self.profile_network().await?;

        let profile = SystemProfile {
            timestamp,
            cpu,
            memory,
            gpu,
            io,
            network,
            modules: self.collect_module_metrics().await?,
            uptime_secs: self.get_uptime().await?,
            load_average: self.get_load_average().await?,
        };

        // Store in history
        let mut history = self.profile_history.write().await;
        if history.len() >= self.config.history_size {
            history.remove(0);
        }
        history.push(profile.clone());

        Ok(profile)
    }

    /// Profile CPU
    async fn profile_cpu(&self) -> Result<CpuProfile, Box<dyn std::error::Error>> {
        // Simulated CPU profiling - in production would read from /proc/stat
        Ok(CpuProfile {
            total_usage: 45.0,
            core_usage: vec![40.0, 50.0, 45.0, 42.0, 48.0, 44.0, 46.0, 43.0],
            user_percent: 35.0,
            system_percent: 10.0,
            idle_percent: 55.0,
            iowait_percent: 2.0,
            core_frequencies: vec![3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000],
            core_temperatures: vec![45.0, 46.0, 44.0, 45.0, 47.0, 46.0, 45.0, 44.0],
            context_switches: 15000,
            interrupts: 5000,
        })
    }

    /// Profile memory
    async fn profile_memory(&self) -> Result<MemoryProfile, Box<dyn std::error::Error>> {
        // Simulated memory profiling - in production would read from /proc/meminfo
        Ok(MemoryProfile {
            total: 16 * 1024 * 1024 * 1024, // 16 GB
            used: 8 * 1024 * 1024 * 1024,   // 8 GB
            available: 8 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            cached: 2 * 1024 * 1024 * 1024,
            buffers: 512 * 1024 * 1024,
            shared: 256 * 1024 * 1024,
            swap_total: 8 * 1024 * 1024 * 1024,
            swap_used: 0,
            page_faults: 1000,
            major_faults: 10,
        })
    }

    /// Profile GPU
    async fn profile_gpu(&self) -> Result<GpuProfile, Box<dyn std::error::Error>> {
        // Simulated GPU profiling - in production would use NVML or similar
        Ok(GpuProfile {
            gpu_usage: 35.0,
            memory_usage: 40.0,
            memory_total: 8 * 1024 * 1024 * 1024, // 8 GB
            memory_used: 3 * 1024 * 1024 * 1024,  // 3 GB
            temperature: 55.0,
            power_draw: 120.0,
            power_limit: 250.0,
            clock_frequency: 1500,
            memory_frequency: 7000,
            encoder_usage: 0.0,
            decoder_usage: 5.0,
        })
    }

    /// Profile I/O
    async fn profile_io(&self) -> Result<IoProfile, Box<dyn std::error::Error>> {
        // Simulated I/O profiling - in production would read from /proc/diskstats
        Ok(IoProfile {
            read_ops_per_sec: 150,
            write_ops_per_sec: 75,
            read_bytes_per_sec: 50 * 1024 * 1024,  // 50 MB/s
            write_bytes_per_sec: 25 * 1024 * 1024, // 25 MB/s
            read_latency_us: 500,
            write_latency_us: 750,
            queue_depth: 4,
            disk_utilization: 30.0,
        })
    }

    /// Profile network
    async fn profile_network(&self) -> Result<NetworkProfile, Box<dyn std::error::Error>> {
        // Simulated network profiling - in production would read from /proc/net/dev
        Ok(NetworkProfile {
            rx_bytes_per_sec: 10 * 1024 * 1024,  // 10 MB/s
            tx_bytes_per_sec: 5 * 1024 * 1024,   // 5 MB/s
            rx_packets_per_sec: 15000,
            tx_packets_per_sec: 8000,
            rx_errors_per_sec: 0,
            tx_errors_per_sec: 0,
            tcp_connections: 150,
            udp_connections: 25,
            interface_utilization: 5.0,
        })
    }

    /// Collect AI module metrics
    async fn collect_module_metrics(&self) -> Result<HashMap<String, ModuleMetrics>, Box<dyn std::error::Error>> {
        let mut metrics = HashMap::new();
        
        // Simulated module metrics
        metrics.insert("predictive_caching".to_string(), ModuleMetrics {
            name: "predictive_caching".to_string(),
            cpu_time_ms: 1500,
            memory_bytes: 128 * 1024 * 1024,
            inference_count: 10000,
            avg_latency_ms: 2.5,
            p95_latency_ms: 5.0,
            p99_latency_ms: 8.0,
            throughput: 4000.0,
            error_count: 0,
            cache_hit_rate: 0.85,
        });

        metrics.insert("intelligent_scheduling".to_string(), ModuleMetrics {
            name: "intelligent_scheduling".to_string(),
            cpu_time_ms: 2500,
            memory_bytes: 256 * 1024 * 1024,
            inference_count: 5000,
            avg_latency_ms: 5.0,
            p95_latency_ms: 10.0,
            p99_latency_ms: 15.0,
            throughput: 1000.0,
            error_count: 0,
            cache_hit_rate: 0.92,
        });

        metrics.insert("voice_assistant".to_string(), ModuleMetrics {
            name: "voice_assistant".to_string(),
            cpu_time_ms: 3500,
            memory_bytes: 512 * 1024 * 1024,
            inference_count: 2000,
            avg_latency_ms: 50.0,
            p95_latency_ms: 100.0,
            p99_latency_ms: 150.0,
            throughput: 40.0,
            error_count: 2,
            cache_hit_rate: 0.78,
        });

        Ok(metrics)
    }

    /// Get system uptime
    async fn get_uptime(&self) -> Result<u64, Box<dyn std::error::Error>> {
        // In production would read from /proc/uptime
        Ok(3600) // 1 hour
    }

    /// Get load average
    async fn get_load_average(&self) -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
        // In production would read from /proc/loadavg
        Ok((2.5, 2.3, 2.1))
    }

    /// Analyze profile for bottlenecks
    pub async fn analyze_bottlenecks(&self, profile: &SystemProfile) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // CPU bottleneck detection
        if profile.cpu.total_usage > 80.0 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::CpuBound,
                severity: 8,
                description: format!("High CPU usage: {:.1}%", profile.cpu.total_usage),
                component: "CPU".to_string(),
                suggestion: "Consider scaling CPU resources or optimizing CPU-intensive tasks".to_string(),
            });
        }

        // Memory bottleneck detection
        if profile.memory.usage_percent > 85.0 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::MemoryBound,
                severity: 7,
                description: format!("High memory usage: {:.1}%", profile.memory.usage_percent),
                component: "Memory".to_string(),
                suggestion: "Increase available memory or optimize memory-intensive applications".to_string(),
            });
        }

        // GPU bottleneck detection
        if let Some(ref gpu) = profile.gpu {
            if gpu.gpu_usage > 90.0 {
                bottlenecks.push(Bottleneck {
                    bottleneck_type: BottleneckType::GpuBound,
                    severity: 8,
                    description: format!("High GPU usage: {:.1}%", gpu.gpu_usage),
                    component: "GPU".to_string(),
                    suggestion: "Consider GPU scaling or workload distribution".to_string(),
                });
            }
            if gpu.temperature > 80.0 {
                bottlenecks.push(Bottleneck {
                    bottleneck_type: BottleneckType::ThermalThrottling,
                    severity: 6,
                    description: format!("High GPU temperature: {:.1}°C", gpu.temperature),
                    component: "GPU".to_string(),
                    suggestion: "Improve cooling or reduce GPU workload".to_string(),
                });
            }
        }

        // I/O bottleneck detection
        if profile.io.disk_utilization > 80.0 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::IoBound,
                severity: 6,
                description: format!("High disk utilization: {:.1}%", profile.io.disk_utilization),
                component: "Storage".to_string(),
                suggestion: "Consider using faster storage or optimizing I/O patterns".to_string(),
            });
        }

        bottlenecks
    }

    /// Generate performance report
    pub async fn generate_report(&self) -> Result<PerformanceReport, Box<dyn std::error::Error>> {
        let history = self.profile_history.read().await;
        
        if history.is_empty() {
            return Err("No profile data available".into());
        }

        let mut cpu_usages = Vec::new();
        let mut memory_usages = Vec::new();
        let mut gpu_usages = Vec::new();

        for profile in history.iter() {
            cpu_usages.push(profile.cpu.total_usage);
            memory_usages.push(profile.memory.usage_percent);
            if let Some(ref gpu) = profile.gpu {
                gpu_usages.push(gpu.gpu_usage);
            }
        }

        Ok(PerformanceReport {
            samples_analyzed: history.len(),
            avg_cpu_usage: cpu_usages.iter().sum::<f64>() / cpu_usages.len() as f64,
            peak_cpu_usage: cpu_usages.iter().cloned().fold(0.0, f64::max),
            avg_memory_usage: memory_usages.iter().sum::<f64>() / memory_usages.len() as f64,
            peak_memory_usage: memory_usages.iter().cloned().fold(0.0, f64::max),
            avg_gpu_usage: if gpu_usages.is_empty() { 0.0 } else { gpu_usages.iter().sum::<f64>() / gpu_usages.len() as f64 },
            peak_gpu_usage: gpu_usages.iter().cloned().fold(0.0, f64::max),
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
    }
}

/// Performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Number of samples analyzed
    pub samples_analyzed: usize,
    /// Average CPU usage
    pub avg_cpu_usage: f64,
    /// Peak CPU usage
    pub peak_cpu_usage: f64,
    /// Average memory usage
    pub avg_memory_usage: f64,
    /// Peak memory usage
    pub peak_memory_usage: f64,
    /// Average GPU usage
    pub avg_gpu_usage: f64,
    /// Peak GPU usage
    pub peak_gpu_usage: f64,
    /// Report generation timestamp
    pub generated_at: u64,
}