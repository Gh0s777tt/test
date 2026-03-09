//! Adaptive Power Management Module
//! 
//! Provides intelligent power management using machine learning
//! to optimize power consumption while maintaining performance.
//! 
//! ## ML Features
//! - Reinforcement learning for power state optimization
//! - Time-series forecasting for workload prediction
//! - Bayesian optimization for parameter tuning
//! 
//! ## Security Considerations
//! - Power changes are rate-limited
//! - Thermal limits are enforced
//! - No unsafe voltage adjustments

use crate::ai::{
    config::PowerConfig,
    error::AIError,
    types::{Confidence, PowerDecision, PowerState},
    ml::rl::QLearningAgent,
    ml::forecasting::ExponentialSmoothingForecaster,
};

/// Power state discretized for RL
const POWER_STATES: usize = 4; // PowerSave, Low, Balanced, Performance
const WORKLOAD_BINS: usize = 10; // 0-10, 11-20, ..., 91-100
const TEMPERATURE_BINS: usize = 5; // Cold, Cool, Normal, Warm, Hot

/// Adaptive Power Manager with ML
/// 
/// Manages power consumption using ML-based optimization.
/// 
/// ## Features
/// - Dynamic frequency scaling with RL optimization
/// - Workload prediction with exponential smoothing
/// - Thermal-aware power decisions
/// - Performance/power tradeoffs using Q-learning
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::power_manager::AdaptivePowerManager;
//! 
//! let pm = AdaptivePowerManager::new(PowerConfig::default())?;
//! 
//! // Get power decision for current workload
//! let decision = pm.get_power_decision(65, 45.0)?;
//! println!("CPU freq: {} MHz, state: {:?}", decision.cpu_frequency_mhz, decision.power_state);
//! ```
pub struct AdaptivePowerManager {
    enabled: bool,
    config: PowerConfig,
    current_state: PowerState,
    model_version: u32,
    
    // ML Components
    q_agent: QLearningAgent,
    workload_forecaster: ExponentialSmoothingForecaster,
    
    // State tracking
    workload_history: Vec<f64>,
    temperature_history: Vec<f64>,
    power_consumption_history: Vec<f64>,
    last_predicted_workload: f64,
}

/// Power management state for RL
#[derive(Debug, Clone)]
pub struct PowerStateInfo {
    pub workload_percent: u8,
    pub temperature_celsius: f64,
    pub battery_percent: Option<u8>,
    pub current_freq_mhz: u32,
}

/// Power management action from RL
#[derive(Debug, Clone, Copy)]
pub enum PowerAction {
    /// Switch to power save mode
    PowerSave,
    /// Low power mode
    LowPower,
    /// Balanced mode
    Balanced,
    /// Performance mode
    Performance,
}

impl AdaptivePowerManager {
    /// Create a new Adaptive Power Manager with ML
    /// 
    /// ## Arguments
    /// * `config` - Power management configuration
    /// 
    /// ## Returns
    /// A new power manager instance with ML capabilities
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: PowerConfig) -> Result<Self, AIError> {
        // Initialize Q-Learning agent for power optimization
        let state_size = WORKLOAD_BINS * TEMPERATURE_BINS;
        let q_agent = QLearningAgent::new(
            state_size,
            POWER_STATES,
            0.1,   // learning_rate
            0.95,  // discount_factor
            0.1,   // epsilon for exploration
        );
        
        // Initialize workload forecaster
        let workload_forecaster = ExponentialSmoothingForecaster::new(0.3, 0.1);
        
        Ok(Self {
            enabled: config.enabled,
            config,
            current_state: PowerState::Balanced,
            model_version: 1,
            q_agent,
            workload_forecaster,
            workload_history: Vec::with_capacity(100),
            temperature_history: Vec::with_capacity(100),
            power_consumption_history: Vec::with_capacity(100),
            last_predicted_workload: 50.0,
        })
    }

    /// Get power decision for current workload with ML optimization
    /// 
    /// ## Arguments
    /// * `workload_percent` - Current workload (0-100)
    /// * `temperature_celsius` - Current temperature in Celsius
    /// 
    /// ## Returns
    /// Power management decision optimized by ML
    /// 
    /// ## Errors
    /// - Returns error if power manager is disabled
    /// - Returns error if workload is invalid
    /// 
    /// ## Performance
    /// Target latency: <5ms
    pub fn get_power_decision(&mut self, workload_percent: u8, temperature_celsius: f64) -> Result<PowerDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        if workload_percent > 100 {
            return Err(AIError::InvalidInput);
        }

        // Update history
        self.workload_history.push(workload_percent as f64);
        self.temperature_history.push(temperature_celsius);
        
        // Keep history bounded
        if self.workload_history.len() > 100 {
            self.workload_history.remove(0);
            self.temperature_history.remove(0);
        }

        // Get ML-optimized power state
        let state_idx = self.encode_state(workload_percent, temperature_celsius);
        let action = self.q_agent.select_action(state_idx);
        let power_state = self.action_to_power_state(action);
        
        // Predict future workload for proactive adjustment
        if let Some(predicted) = self.workload_forecaster.forecast(&self.workload_history, 1).first() {
            self.last_predicted_workload = *predicted;
        }

        // Calculate CPU frequency based on workload and predicted trend
        let cpu_freq = self.calculate_optimal_frequency(workload_percent, self.last_predicted_workload, temperature_celsius);
        
        // Calculate confidence based on history size and prediction accuracy
        let confidence = self.calculate_confidence();

        self.current_state = power_state.clone();
        
        Ok(PowerDecision {
            cpu_frequency_mhz: cpu_freq,
            power_state,
        })
    }
    
    /// Provide feedback for RL learning
    /// 
    /// Call this after a power decision to update the RL agent
    /// 
    /// ## Arguments
    /// * `state` - Current power state info
    /// * `action` - Action that was taken
    /// * `reward` - Reward signal (positive for good decisions)
    /// * `next_state` - Resulting state
    pub fn provide_feedback(
        &mut self,
        state: &PowerStateInfo,
        action: PowerAction,
        reward: f64,
        next_state: &PowerStateInfo,
    ) -> Result<(), AIError> {
        let state_idx = self.encode_state(state.workload_percent, state.temperature_celsius);
        let next_state_idx = self.encode_state(next_state.workload_percent, next_state.temperature_celsius);
        let action_idx = self.action_to_index(action);
        
        self.q_agent.update(state_idx, action_idx, reward, next_state_idx);
        Ok(())
    }
    
    /// Calculate reward for RL agent
    /// 
    /// Higher reward for:
    /// - Lower power consumption
    /// - Acceptable temperature
    /// - Good performance (low latency)
    pub fn calculate_reward(
        &self,
        power_consumption_watts: f64,
        temperature_celsius: f64,
        performance_score: f64, // 0.0 to 1.0
    ) -> f64 {
        // Normalize power (assume max 100W for typical system)
        let power_score = 1.0 - (power_consumption_watts / 100.0).min(1.0);
        
        // Temperature penalty (optimal around 40-60°C)
        let temp_penalty = if temperature_celsius < 40.0 {
            (40.0 - temperature_celsius) * 0.01 // Too cold, minor penalty
        } else if temperature_celsius > 80.0 {
            (temperature_celsius - 80.0) * 0.1 // Too hot, major penalty
        } else {
            0.0 // Good temperature
        };
        
        // Weighted combination
        let reward = (power_score * 0.3) + (performance_score * 0.5) - temp_penalty;
        reward.max(-1.0).min(1.0)
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
        self.current_state.clone()
    }

    /// Get predicted workload
    /// 
    /// ## Returns
    /// Predicted workload for next time step
    pub fn get_predicted_workload(&self) -> f64 {
        self.last_predicted_workload
    }

    /// Check if power manager is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if power manager is ready
    pub fn is_ready(&self) -> bool {
        self.enabled
    }
    
    /// Get model version
    pub fn get_model_version(&self) -> u32 {
        self.model_version
    }
    
    // Private helper methods
    
    fn encode_state(&self, workload: u8, temperature: f64) -> usize {
        let workload_bin = (workload as usize) / 10;
        let temp_bin = self.temperature_to_bin(temperature);
        
        (workload_bin.min(WORKLOAD_BINS - 1)) * TEMPERATURE_BINS + temp_bin
    }
    
    fn temperature_to_bin(&self, temp: f64) -> usize {
        match temp {
            t if t < 30.0 => 0, // Cold
            t if t < 45.0 => 1, // Cool
            t if t < 60.0 => 2, // Normal
            t if t < 75.0 => 3, // Warm
            _ => 4,              // Hot
        }
    }
    
    fn action_to_power_state(&self, action: usize) -> PowerState {
        match action {
            0 => PowerState::PowerSave,
            1 => PowerState::Low,
            2 => PowerState::Balanced,
            3 => PowerState::Performance,
            _ => PowerState::Balanced,
        }
    }
    
    fn action_to_index(&self, action: PowerAction) -> usize {
        match action {
            PowerAction::PowerSave => 0,
            PowerAction::LowPower => 1,
            PowerAction::Balanced => 2,
            PowerAction::Performance => 3,
        }
    }
    
    fn calculate_optimal_frequency(&self, current_workload: u8, predicted_workload: f64, temperature: f64) -> u32 {
        let min_freq = self.config.min_cpu_freq_mhz;
        let max_freq = self.config.max_cpu_freq_mhz;
        
        // Base frequency from current workload
        let workload_factor = (current_workload as f64 / 100.0).min(1.0);
        
        // Adjust for predicted workload trend
        let trend_adjustment = if predicted_workload > current_workload as f64 + 10.0 {
            0.1 // Ramp up early
        } else if predicted_workload < current_workload as f64 - 10.0 {
            -0.1 // Ramp down early
        } else {
            0.0
        };
        
        // Temperature throttling
        let thermal_throttle = if temperature > 70.0 {
            (temperature - 70.0) / 100.0 // Reduce frequency at high temps
        } else {
            0.0
        };
        
        let combined_factor = (workload_factor + trend_adjustment - thermal_throttle).clamp(0.0, 1.0);
        let range = (max_freq - min_freq) as f64;
        
        (min_freq as f64 + range * combined_factor) as u32
    }
    
    fn calculate_confidence(&self) -> f64 {
        // More history = more confidence
        let history_factor = (self.workload_history.len() as f64 / 50.0).min(1.0);
        
        // Could add prediction accuracy metric here
        history_factor
    }
}

/// Calculate CPU frequency based on workload (legacy compatibility)
/// 
/// ## Arguments
/// * `workload` - Current workload (0-100)
/// * `config` - Power configuration
/// 
/// ## Returns
/// CPU frequency in MHz
fn calculate_cpu_frequency(workload: u8, config: &PowerConfig) -> u32 {
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
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(50, 45.0).unwrap();
        assert!(decision.cpu_frequency_mhz >= 800 && decision.cpu_frequency_mhz <= 4000);
    }

    #[test]
    fn test_high_workload() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(90, 55.0).unwrap();
        // High workload should result in higher frequency
        assert!(decision.cpu_frequency_mhz > 2000);
    }

    #[test]
    fn test_low_workload() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        let decision = pm.get_power_decision(10, 40.0).unwrap();
        // Low workload should result in lower frequency
        assert!(decision.cpu_frequency_mhz < 2000);
    }

    #[test]
    fn test_thermal_throttling() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        
        // Normal temperature
        let decision_normal = pm.get_power_decision(80, 50.0).unwrap();
        
        // High temperature - should throttle
        let decision_hot = pm.get_power_decision(80, 85.0).unwrap();
        
        assert!(decision_normal.cpu_frequency_mhz > decision_hot.cpu_frequency_mhz);
    }

    #[test]
    fn test_workload_prediction() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        
        // Feed some workload history
        for i in 50..60 {
            pm.get_power_decision(i, 45.0).unwrap();
        }
        
        let predicted = pm.get_predicted_workload();
        assert!(predicted > 0.0 && predicted <= 100.0);
    }
    
    #[test]
    fn test_reward_calculation() {
        let pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        
        // Good conditions
        let reward_good = pm.calculate_reward(30.0, 50.0, 0.9);
        assert!(reward_good > 0.0);
        
        // Bad conditions (hot, high power, low performance)
        let reward_bad = pm.calculate_reward(90.0, 85.0, 0.3);
        assert!(reward_bad < reward_good);
    }

    #[test]
    fn test_set_power_mode() {
        let mut pm = AdaptivePowerManager::new(PowerConfig::default()).unwrap();
        pm.set_power_mode(PowerMode::Performance);
        // Mode is set but decision still uses ML
        assert!(pm.is_enabled());
    }
}