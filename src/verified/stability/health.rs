//! Health Monitoring Module
//! 
//! This module provides comprehensive health monitoring capabilities
//! for tracking system health metrics and detecting degradation.

use alloc::sync::Arc;
use spin::Mutex;

/// Health metric type
#[derive(Debug, Clone, Copy)]
pub enum HealthMetricType {
    CpuUsage,
    MemoryUsage,
    DiskUsage,
    NetworkLatency,
    ProcessCount,
    ThreadCount,
    ErrorRate,
    ResponseTime,
    Custom(usize),
}

/// Health metric value
#[derive(Debug, Clone, Copy)]
pub enum HealthMetricValue {
    Float(f64),
    Integer(u64),
    Boolean(bool),
}

/// Health metric
#[derive(Debug, Clone)]
pub struct HealthMetric {
    pub metric_type: HealthMetricType,
    pub value: HealthMetricValue,
    pub timestamp: u64,
    pub threshold_warning: Option<f64>,
    pub threshold_critical: Option<f64>,
}

impl HealthMetric {
    /// Get health status based on threshold
    pub fn status(&self) -> HealthStatus {
        match self.value {
            HealthMetricValue::Float(val) => {
                if let Some(critical) = self.threshold_critical {
                    if val >= critical {
                        return HealthStatus::Critical;
                    }
                }
                if let Some(warning) = self.threshold_warning {
                    if val >= warning {
                        return HealthStatus::Warning;
                    }
                }
                HealthStatus::Healthy
            }
            HealthMetricValue::Integer(val) => {
                if let Some(critical) = self.threshold_critical {
                    if (val as f64) >= critical {
                        return HealthStatus::Critical;
                    }
                }
                if let Some(warning) = self.threshold_warning {
                    if (val as f64) >= warning {
                        return HealthStatus::Warning;
                    }
                }
                HealthStatus::Healthy
            }
            HealthMetricValue::Boolean(_) => HealthStatus::Healthy,
        }
    }
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Health monitor
pub struct HealthMonitor {
    metrics: Arc<Mutex<Vec<HealthMetric>>>,
    status: Arc<Mutex<HealthStatus>>,
    monitoring_enabled: Arc<Mutex<bool>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new(HealthStatus::Unknown)),
            monitoring_enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Enable or disable health monitoring
    pub fn set_monitoring_enabled(&self, enabled: bool) {
        *self.monitoring_enabled.lock() = enabled;
    }

    /// Record a health metric
    pub fn record_metric(&self, metric: HealthMetric) {
        if !*self.monitoring_enabled.lock() {
            return;
        }

        let mut metrics = self.metrics.lock();
        metrics.push(metric);
        
        // Update overall health status
        let mut status = self.status.lock();
        *status = HealthStatus::Healthy;
        
        for metric in metrics.iter() {
            match metric.status() {
                HealthStatus::Critical => {
                    *status = HealthStatus::Critical;
                    break;
                }
                HealthStatus::Warning => {
                    *status = HealthStatus::Warning;
                }
                _ => {}
            }
        }
    }

    /// Get overall health status
    pub fn status(&self) -> HealthStatus {
        *self.status.lock()
    }

    /// Get all metrics
    pub fn metrics(&self) -> Vec<HealthMetric> {
        self.metrics.lock().clone()
    }

    /// Get metrics of a specific type
    pub fn metrics_by_type(&self, metric_type: HealthMetricType) -> Vec<HealthMetric> {
        self.metrics
            .lock()
            .iter()
            .filter(|m| m.metric_type == metric_type)
            .cloned()
            .collect()
    }

    /// Clear all metrics
    pub fn clear(&self) {
        self.metrics.lock().clear();
        *self.status.lock() = HealthStatus::Healthy;
    }

    /// Get health summary
    pub fn summary(&self) -> HealthSummary {
        let metrics = self.metrics.lock();
        
        let mut healthy = 0;
        let mut warning = 0;
        let mut critical = 0;
        
        for metric in metrics.iter() {
            match metric.status() {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Warning => warning += 1,
                HealthStatus::Critical => critical += 1,
                _ => {}
            }
        }
        
        HealthSummary {
            overall_status: *self.status.lock(),
            total_metrics: metrics.len(),
            healthy_metrics: healthy,
            warning_metrics: warning,
            critical_metrics: critical,
        }
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Health summary
#[derive(Debug, Clone, Copy)]
pub struct HealthSummary {
    pub overall_status: HealthStatus,
    pub total_metrics: usize,
    pub healthy_metrics: usize,
    pub warning_metrics: usize,
    pub critical_metrics: usize,
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    pub check_interval_seconds: u64,
    pub metrics_to_monitor: Vec<HealthMetricType>,
    pub warning_thresholds: Vec<(HealthMetricType, f64)>,
    pub critical_thresholds: Vec<(HealthMetricType, f64)>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 10,
            metrics_to_monitor: vec![
                HealthMetricType::CpuUsage,
                HealthMetricType::MemoryUsage,
                HealthMetricType::DiskUsage,
            ],
            warning_thresholds: vec![
                (HealthMetricType::CpuUsage, 80.0),
                (HealthMetricType::MemoryUsage, 80.0),
                (HealthMetricType::DiskUsage, 80.0),
            ],
            critical_thresholds: vec![
                (HealthMetricType::CpuUsage, 95.0),
                (HealthMetricType::MemoryUsage, 95.0),
                (HealthMetricType::DiskUsage, 95.0),
            ],
        }
    }
}

/// Global health monitor
static HEALTH_MONITOR: spin::Once<HealthMonitor> = spin::Once::new();

/// Get the global health monitor
pub fn health_monitor() -> &'static HealthMonitor {
    HEALTH_MONITOR.call_once(|| HealthMonitor::new())
}

/// Record a health metric
pub fn record_metric(metric: HealthMetric) {
    health_monitor().record_metric(metric);
}

/// Get overall health status
pub fn health_status() -> HealthStatus {
    health_monitor().status()
}

/// Get health summary
pub fn health_summary() -> HealthSummary {
    health_monitor().summary()
}