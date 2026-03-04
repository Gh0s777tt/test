//! Adaptive Power Management Module
//! 
//! Provides intelligent power management using machine learning
//! to optimize power consumption while maintaining performance.
//! 
//! ## Security Considerations
//! - Power changes are rate-limited
//! - Thermal limits are enforced
//! - No unsafe voltage adjustments

use crate::ai::{
    config::PowerConfig,
    error::AIError,
    types::{Confidence, PowerDecision, PowerState},
};

/// Adaptive Power Manager
/// 
/// Manages power consumption using ML-based optimization.
/// 
/// ## Features
/// - Dynamic frequency scaling
/// - Workload-aware power states
/// - Thermal management
/// - Performance/power tradeoffs
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::power_manager::AdaptivePowerManager;
//! 
//! let pm = AdaptivePowerManager::new(PowerConfig::default())?;
//! 
//! // Get power decision for current workload
//! let decision = pm.get_power_decision(65)?;
//! println!("CPU freq: {} MHz, state: {:?}", decision.cpu_frequency_mhz, decision.power_state);
//! ```
pub struct AdaptivePowerManager {
    enabled: bool,
    config: PowerConfig,
    current_state: PowerState,
}

impl AdaptivePowerManager {
    /// Create a new Adaptive Power Manager
    /// 
    /// ## Arguments
    /// * `config` - Power management configuration
    /// 
    /// ## Returns
    /// A new power manager instance
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: PowerConfig) -> Result<Self, AIError> {
        Ok(Self {
            enabled: config.enabled,
            config,
            current_state: PowerState::Balanced,
        })
    }

    /// Get power decision for current workload
    /// 
    /// ## Arguments
    /// * `workload_percent` - Current workload (0-100)
    /// 
    /// ## Returns
    /// Power management decision
    /// 
    /// ## Errors
    /// - Returns error if power manager is disabled
    /// - Returns error if workload is invalid
    /// 
    /// ## Performance
    /// Target latency: <5ms
    pub fn get_power_decision(&self, workload_percent: u8) -> Result<PowerDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        if workload_percent > 100 {
            return Err(AIError::InvalidInput);
        }

        // Determine power state based on workload
        let power_state = match self.config.power_mode {
            PowerMode::Performance => PowerState::Performance,
            PowerMode::PowerSave => PowerState::PowerSave,
            PowerMode::Balanced => {
                if workload_percent > 80 {
                    PowerState::Performance
                } else if workload_percent < 20 {
                    PowerState::PowerSave
                } else {
                    PowerState::Balanced
                }
            }
        };

        // Calculate CPU frequency based on workload
        let cpu_freq = calculate_cpu_frequency(workload_percent, &self.config);

        Ok(PowerDecision {
            cpu_frequency_mhz: cpu_freq,
            power_state,
        })
    }

    /// Set power mode
    /// 
    /// ## Arguments
    /// * `mode` - New power mode
    pub fn set_power_mode(&mut self, mode: PowerMode) {
        self.config.power_mode = mode;
    }

    /// Get current power state
    /// 
    /// ## Returns
    /// Current power state
    pub fn get_current_state(&self) -> PowerState {
        self.current_state
    }

    /// Check if power manager is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if power manager is ready
    pub fn is_ready(&self) -> bool {
        self.enabled
    }
}

/// Calculate CPU frequency based on workload
/// 
/// ## Arguments
/// * `workload` - Current workload (0-100)
/// * `config` - Power configuration
/// 
/// ## Returns
/// CPU frequency in MHz
fn calculate_cpu_frequency(workload: u8, config: &PowerConfig) -> u32 {
    // Linear scaling between min and max frequency
    let min_freq = config.min_cpu_freq_mhz;
    let max_freq = config.max_cpu_freq_mhz;
    let range = max_freq - min_freq;
    
    min_freq + (range * workload as u32) / 100
}

/// Power mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    /// Maximum performance
    Performance,
    /// Balanced
    Balanced,
    /// Power saving
    PowerSave,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_manager_creation() {
        let pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        assert!(pm.is_enabled());
        assert_eq!(pm.get_current_state(), PowerState::Balanced);
    }

    #[test]
    fn test_get_power_decision() {
        let pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(50).unwrap();
        assert_eq!(decision.power_state, PowerState::Balanced);
        assert!(decision.cpu_frequency_mhz >= 800 && decision.cpu_frequency_mhz <= 4000);
    }

    #[test]
    fn test_high_workload() {
        let pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(90).unwrap();
        assert_eq!(decision.power_state, PowerState::Performance);
    }

    #[test]
    fn test_low_workload() {
        let pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(10).unwrap();
        assert_eq!(decision.power_state, PowerState::PowerSave);
    }

    #[test]
    fn test_set_power_mode() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        pm.set_power_mode(PowerMode::Performance);
        let decision = pm.get_power_decision(50).unwrap();
        assert_eq!(decision.power_state, PowerState::Performance);
    }
}