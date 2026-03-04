//! AI Security Monitor Module
//! 
//! Provides continuous monitoring of AI system health and security.
//! 
//! ## Security Considerations
//! - All AI operations are logged
//! - Anomaly detection for AI behavior
//! - Model drift detection
//! - Resource usage monitoring

use crate::ai::{
    config::MonitoringConfig,
    error::AIError,
    types::{Confidence, ResourceUsage},
};

/// AI Security Monitor
/// 
/// Monitors AI system health and security.
/// 
/// ## Features
/// - Real-time metrics collection
/// - Drift detection
/// - Anomaly alerting
/// - Performance tracking
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::monitoring::AISecurityMonitor;
//! 
//! let monitor = AISecurityMonitor::new(MonitoringConfig::default())?;
//! 
//! // Collect metrics
//! let metrics = monitor.collect_metrics()?;
//! println!("CPU: {}%, Memory: {}%", metrics.cpu_usage, metrics.memory_usage);
//! ```
pub struct AISecurityMonitor {
    enabled: bool,
    config: MonitoringConfig,
    metrics_count: u64,
}

impl AISecurityMonitor {
    /// Create a new AI Security Monitor
    /// 
    /// ## Arguments
    /// * `config` - Monitoring configuration
    /// 
    /// ## Returns
    /// A new security monitor
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: MonitoringConfig) -> Result<Self, AIError> {
        Ok(Self {
            enabled: config.enabled,
            config,
            metrics_count: 0,
        })
    }

    /// Collect system metrics
    /// 
    /// ## Returns
    /// Current resource usage metrics
    /// 
    /// ## Errors
    /// - Returns error if monitor is disabled
    /// - Returns error if collection fails
    /// 
    /// ## Performance
    /// Target latency: <5ms
    pub fn collect_metrics(&mut self) -> Result<ResourceUsage, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        self.metrics_count += 1;

        // STUB: Will implement full metrics collection in v1.3.0
        Ok(ResourceUsage {
            cpu_usage: 25.0,
            memory_usage: 40.0,
            disk_usage: 55.0,
            network_usage: 10.0,
        })
    }

    /// Check for model drift
    /// 
    /// ## Returns
    /// True if drift is detected
    /// 
    /// ## Errors
    /// - Returns error if monitor is disabled
    /// - Returns error if detection fails
    pub fn detect_drift(&self) -> Result<bool, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        // STUB: Will implement drift detection in v1.3.0
        Ok(false)
    }

    /// Get metrics count
    /// 
    /// ## Returns
    /// Number of metrics collections performed
    pub fn metrics_count(&self) -> u64 {
        self.metrics_count
    }

    /// Check if monitor is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if monitor is ready
    pub fn is_ready(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let monitor = AISecurityMonitor::new(MonitoringConfig::default()).unwrap();
        assert!(monitor.is_enabled());
        assert_eq!(monitor.metrics_count(), 0);
    }

    #[test]
    fn test_collect_metrics() {
        let mut monitor = AISecurityMonitor::new(MonitoringConfig::default()).unwrap();
        let metrics = monitor.collect_metrics().unwrap();
        
        assert_eq!(monitor.metrics_count(), 1);
        assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0);
        assert!(metrics.memory_usage >= 0.0 && metrics.memory_usage <= 100.0);
    }

    #[test]
    fn test_drift_detection() {
        let monitor = AISecurityMonitor::new(MonitoringConfig::default()).unwrap();
        let drift = monitor.detect_drift().unwrap();
        assert!(!drift); // No drift expected initially
    }
}