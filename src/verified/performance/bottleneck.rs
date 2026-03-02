//! Bottleneck Analysis Module
//! 
//! This module provides bottleneck detection and analysis capabilities
//! for identifying performance bottlenecks in the system.

use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// Bottleneck type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckType {
    CpuBound,
    MemoryBound,
    IoBound,
    NetworkBound,
    LockContention,
    CacheThrashing,
    ContextSwitching,
    Unknown,
}

/// Bottleneck severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Bottleneck information
#[derive(Debug, Clone)]
pub struct BottleneckInfo {
    pub bottleneck_type: BottleneckType,
    pub severity: BottleneckSeverity,
    pub description: alloc::string::String,
    pub location: Option<alloc::string::String>,
    pub impact_score: f64, // 0.0 to 1.0
    pub suggested_fixes: Vec<alloc::string::String>,
}

/// Bottleneck detector
pub struct BottleneckDetector {
    detected_bottlenecks: Arc<Mutex<Vec<BottleneckInfo>>>,
    enabled: Arc<Mutex<bool>>,
}

impl BottleneckDetector {
    /// Create a new bottleneck detector
    pub fn new() -> Self {
        Self {
            detected_bottlenecks: Arc::new(Mutex::new(Vec::new())),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Enable or disable bottleneck detection
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock() = enabled;
    }

    /// Analyze performance data for bottlenecks
    pub fn analyze(&self, data: &PerformanceData) -> Vec<BottleneckInfo> {
        let mut bottlenecks = Vec::new();

        // Check CPU bottlenecks
        if data.cpu_utilization > 95.0 {
            bottlenecks.push(BottleneckInfo {
                bottleneck_type: BottleneckType::CpuBound,
                severity: BottleneckSeverity::High,
                description: alloc::string::String::from("CPU utilization is very high"),
                location: None,
                impact_score: 0.9,
                suggested_fixes: vec![
                    alloc::string::String::from("Optimize algorithms"),
                    alloc::string::String::from("Use better data structures"),
                    alloc::string::String::from("Reduce computational complexity"),
                ],
            });
        }

        // Check memory bottlenecks
        if data.memory_utilization > 90.0 {
            bottlenecks.push(BottleneckInfo {
                bottleneck_type: BottleneckType::MemoryBound,
                severity: BottleneckSeverity::High,
                description: alloc::string::String::from("Memory utilization is very high"),
                location: None,
                impact_score: 0.85,
                suggested_fixes: vec![
                    alloc::string::String::from("Reduce memory allocations"),
                    alloc::string::String::from("Use memory pools"),
                    alloc::string::String::from("Implement data streaming"),
                ],
            });
        }

        // Check I/O bottlenecks
        if data.io_wait_time > data.cpu_time * 2.0 {
            bottlenecks.push(BottleneckInfo {
                bottleneck_type: BottleneckType::IoBound,
                severity: BottleneckSeverity::High,
                description: alloc::string::String::from("System is I/O bound"),
                location: None,
                impact_score: 0.8,
                suggested_fixes: vec![
                    alloc::string::String::from("Use asynchronous I/O"),
                    alloc::string::String::from("Implement caching"),
                    alloc::string::String::from("Batch I/O operations"),
                ],
            });
        }

        // Check network bottlenecks
        if data.network_latency > 100.0 {
            bottlenecks.push(BottleneckInfo {
                bottleneck_type: BottleneckType::NetworkBound,
                severity: BottleneckSeverity::Medium,
                description: alloc::string::String::from("Network latency is high"),
                location: None,
                impact_score: 0.7,
                suggested_fixes: vec![
                    alloc::string::String::from("Use connection pooling"),
                    alloc::string::String::from("Implement request batching"),
                    alloc::string::String::from("Use CDN or edge computing"),
                ],
            });
        }

        bottlenecks
    }

    /// Detect bottlenecks based on profiling data
    pub fn detect_from_profiles(&self, profiles: &[super::profiling::FunctionProfile]) -> Vec<BottleneckInfo> {
        let mut bottlenecks = Vec::new();

        // Find functions with high self time
        for profile in profiles.iter().take(10) {
            if profile.self_time_ns > profile.total_time_ns / 2 {
                bottlenecks.push(BottleneckInfo {
                    bottleneck_type: BottleneckType::CpuBound,
                    severity: BottleneckSeverity::Medium,
                    description: alloc::format!("Function {} has high self time", profile.function_name),
                    location: Some(profile.function_name.clone()),
                    impact_score: 0.6,
                    suggested_fixes: vec![
                        alloc::format!("Optimize {}", profile.function_name),
                        alloc::format!("Reduce complexity in {}", profile.function_name),
                    ],
                });
            }
        }

        bottlenecks
    }

    /// Get detected bottlenecks
    pub fn detected_bottlenecks(&self) -> Vec<BottleneckInfo> {
        self.detected_bottlenecks.lock().clone()
    }

    /// Clear detected bottlenecks
    pub fn clear(&self) {
        self.detected_bottlenecks.lock().clear();
    }

    /// Get bottleneck statistics
    pub fn statistics(&self) -> BottleneckStatistics {
        let bottlenecks = self.detected_bottlenecks.lock();
        
        let mut cpu = 0;
        let mut memory = 0;
        let mut io = 0;
        let mut network = 0;
        
        for b in bottlenecks.iter() {
            match b.bottleneck_type {
                BottleneckType::CpuBound => cpu += 1,
                BottleneckType::MemoryBound => memory += 1,
                BottleneckType::IoBound => io += 1,
                BottleneckType::NetworkBound => network += 1,
                _ => {}
            }
        }
        
        BottleneckStatistics {
            total_bottlenecks: bottlenecks.len(),
            cpu_bound: cpu,
            memory_bound: memory,
            io_bound: io,
            network_bound: network,
        }
    }
}

impl Default for BottleneckDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance data for bottleneck analysis
#[derive(Debug, Clone, Copy)]
pub struct PerformanceData {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub io_wait_time: f64,
    pub cpu_time: f64,
    pub network_latency: f64,
    pub context_switches: u64,
    pub cache_misses: u64,
    pub page_faults: u64,
}

/// Bottleneck statistics
#[derive(Debug, Clone, Copy)]
pub struct BottleneckStatistics {
    pub total_bottlenecks: usize,
    pub cpu_bound: usize,
    pub memory_bound: usize,
    pub io_bound: usize,
    pub network_bound: usize,
}

/// Global bottleneck detector
static BOTTLENECK_DETECTOR: spin::Once<BottleneckDetector> = spin::Once::new();

/// Get the global bottleneck detector
pub fn bottleneck_detector() -> &'static BottleneckDetector {
    BOTTLENECK_DETECTOR.call_once(|| BottleneckDetector::new())
}

/// Analyze performance data
pub fn analyze_bottlenecks(data: &PerformanceData) -> Vec<BottleneckInfo> {
    bottleneck_detector().analyze(data)
}

/// Get detected bottlenecks
pub fn detected_bottlenecks() -> Vec<BottleneckInfo> {
    bottleneck_detector().detected_bottlenecks()
}

/// Get bottleneck statistics
pub fn bottleneck_statistics() -> BottleneckStatistics {
    bottleneck_detector().statistics()
}