//! AI Configuration Module

use crate::ai::error::AIError;

/// Main AI configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AIConfig {
    /// Enable AI module
    pub enabled: bool,
    /// Scheduler configuration
    pub scheduler: SchedulerConfig,
    /// Power management configuration
    pub power: PowerConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scheduler: SchedulerConfig::default(),
            power: PowerConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Clone for AIConfig {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            scheduler: self.scheduler.clone(),
            power: self.power.clone(),
            security: self.security.clone(),
            monitoring: self.monitoring.clone(),
        }
    }
}

/// Scheduler configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerConfig {
    /// Enable ML scheduling
    pub enabled: bool,
    /// Maximum latency in ms
    pub max_latency_ms: u64,
    /// Minimum time slice in ms
    pub min_time_slice_ms: u64,
    /// Maximum time slice in ms
    pub max_time_slice_ms: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_latency_ms: 10,
            min_time_slice_ms: 5,
            max_time_slice_ms: 100,
        }
    }
}

impl Clone for SchedulerConfig {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            max_latency_ms: self.max_latency_ms,
            min_time_slice_ms: self.min_time_slice_ms,
            max_time_slice_ms: self.max_time_slice_ms,
        }
    }
}

/// Power management configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerConfig {
    /// Enable adaptive power management
    pub enabled: bool,
    /// Power mode
    pub power_mode: PowerMode,
    /// Minimum CPU frequency in MHz
    pub min_cpu_freq_mhz: u32,
    /// Maximum CPU frequency in MHz
    pub max_cpu_freq_mhz: u32,
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            power_mode: PowerMode::Balanced,
            min_cpu_freq_mhz: 800,
            max_cpu_freq_mhz: 4000,
        }
    }
}

/// Power mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    /// Maximum performance
    Performance,
    /// Balanced performance/power
    Balanced,
    /// Power saving
    PowerSave,
}

/// Security configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityConfig {
    /// Enable threat detection
    pub enabled: bool,
    /// Response mode
    pub response_mode: ResponseMode,
    /// Alert threshold (0-100)
    pub alert_threshold: u8,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            response_mode: ResponseMode::LogAndAlert,
            alert_threshold: 70,
        }
    }
}

impl Clone for SecurityConfig {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            response_mode: self.response_mode,
            alert_threshold: self.alert_threshold,
        }
    }
}

/// Response mode for threats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseMode {
    /// Monitor only
    Monitor,
    /// Log and alert
    LogAndAlert,
    /// Block and alert
    BlockAndAlert,
}

/// Monitoring configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Monitoring interval in ms
    pub interval_ms: u64,
    /// Enable drift detection
    pub drift_detection: bool,
    /// Drift threshold (0-100)
    pub drift_threshold: u8,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_ms: 1000,
            drift_detection: true,
            drift_threshold: 10,
        }
    }
}

impl Clone for MonitoringConfig {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            interval_ms: self.interval_ms,
            drift_detection: self.drift_detection,
            drift_threshold: self.drift_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AIConfig::default();
        assert!(config.enabled);
        assert!(config.scheduler.enabled);
        assert!(config.power.enabled);
        assert!(config.security.enabled);
        assert!(config.monitoring.enabled);
    }

    #[test]
    fn test_scheduler_config() {
        let config = SchedulerConfig::default();
        assert_eq!(config.max_latency_ms, 10);
        assert_eq!(config.min_time_slice_ms, 5);
        assert_eq!(config.max_time_slice_ms, 100);
    }

    #[test]
    fn test_power_config() {
        let config = PowerConfig::default();
        assert_eq!(config.power_mode, PowerMode::Balanced);
        assert_eq!(config.min_cpu_freq_mhz, 800);
        assert_eq!(config.max_cpu_freq_mhz, 4000);
    }
}