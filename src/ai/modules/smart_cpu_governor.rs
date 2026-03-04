//! Smart CPU Governor with Predictive Scaling
//! 
//! This module implements an intelligent CPU frequency governor that uses
//! machine learning to predict workload patterns and adjust CPU frequencies
//! proactively for optimal performance and power efficiency.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Configuration for the smart CPU governor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorConfig {
    /// Minimum CPU frequency in MHz
    pub min_frequency_mhz: u32,
    
    /// Maximum CPU frequency in MHz
    pub max_frequency_mhz: u32,
    
    /// Sampling interval for performance metrics
    pub sampling_interval_ms: u64,
    
    /// Prediction window size (number of samples)
    pub prediction_window: usize,
    
    /// Aggressiveness of frequency scaling (0.0 - 1.0)
    pub scaling_aggressiveness: f64,
    
    /// Power efficiency mode
    pub power_mode: PowerMode,
    
    /// Enable predictive scaling
    pub enable_prediction: bool,
}

/// Power efficiency mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PowerMode {
    /// Maximum performance, ignore power consumption
    Performance,
    
    /// Balanced performance and power
    Balanced,
    
    /// Maximum power savings
    PowerSaver,
}

/// CPU core workload characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadCharacteristics {
    /// CPU utilization percentage (0-100)
    pub utilization: f64,
    
    /// Number of context switches
    pub context_switches: u64,
    
    /// Number of interrupts
    pub interrupts: u64,
    
    /// Cache miss rate
    pub cache_miss_rate: f64,
    
    /// Branch prediction accuracy
    pub branch_accuracy: f64,
    
    /// Memory bandwidth usage
    pub memory_bandwidth_mbps: f64,
}

/// CPU performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Current frequency in MHz
    pub current_frequency_mhz: u32,
    
    /// Temperature in Celsius
    pub temperature_celsius: f64,
    
    /// Power consumption in watts
    pub power_watts: f64,
    
    /// Workload characteristics
    pub workload: WorkloadCharacteristics,
    
    /// Timestamp of measurement
    pub timestamp: Instant,
}

/// Predicted workload for future time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadPrediction {
    /// Predicted CPU utilization (0-100)
    pub predicted_utilization: f64,
    
    /// Confidence in prediction (0-1)
    pub confidence: f64,
    
    /// Recommended frequency in MHz
    pub recommended_frequency_mhz: u32,
    
    /// Time until this prediction
    pub time_horizon: Duration,
}

/// ML prediction model for workload
#[derive(Debug)]
pub struct WorkloadPredictionModel {
    /// Historical workload samples
    history: VecDeque<CpuMetrics>,
    
    /// Pattern weights for different time scales
    short_term_pattern: f64,
    medium_term_pattern: f64,
    long_term_pattern: f64,
    
    /// Seasonality detection
    seasonal_patterns: HashMap<u64, f64>,
}

impl WorkloadPredictionModel {
    pub fn new(prediction_window: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(prediction_window * 2),
            short_term_pattern: 0.5,
            medium_term_pattern: 0.3,
            long_term_pattern: 0.2,
            seasonal_patterns: HashMap::new(),
        }
    }

    /// Add new metrics to history
    pub fn add_metrics(&mut self, metrics: CpuMetrics) {
        if self.history.len() >= self.history.capacity() {
            self.history.pop_front();
        }
        self.history.push_back(metrics);
        
        // Update seasonal patterns based on time of day
        let time_key = chrono::Local::now().timestamp() / 3600; // Hour granularity
        let utilization = metrics.workload.utilization;
        
        let current_pattern = self.seasonal_patterns.entry(time_key).or_insert(0.0);
        *current_pattern = current_pattern * 0.95 + utilization * 0.05;
    }

    /// Predict future workload
    pub fn predict(&self, horizon_ms: u64) -> Option<WorkloadPrediction> {
        if self.history.len() < 3 {
            return None;
        }

        let recent_metrics: Vec<_> = self.history.iter().collect();
        let latest = recent_metrics.last()?;
        let current_util = latest.workload.utilization;

        // Calculate trend
        let trend = if recent_metrics.len() >= 5 {
            let recent: f64 = recent_metrics.iter()
                .rev()
                .take(5)
                .map(|m| m.workload.utilization)
                .sum::<f64>() / 5.0;
            
            let older: f64 = recent_metrics.iter()
                .rev()
                .skip(5)
                .take(5)
                .map(|m| m.workload.utilization)
                .sum::<f64>() / 5.0;
            
            recent - older
        } else {
            0.0
        };

        // Calculate volatility
        let volatility = if recent_metrics.len() >= 3 {
            let mean: f64 = recent_metrics.iter()
                .take(10)
                .map(|m| m.workload.utilization)
                .sum::<f64>() / recent_metrics.len().min(10) as f64;
            
            let variance: f64 = recent_metrics.iter()
                .take(10)
                .map(|m| (m.workload.utilization - mean).powi(2))
                .sum::<f64>() / recent_metrics.len().min(10) as f64;
            
            variance.sqrt()
        } else {
            10.0
        };

        // Get seasonal adjustment
        let future_hour = (chrono::Local::now().timestamp() as u64 + horizon_ms / 1000) / 3600;
        let seasonal_adjustment = self.seasonal_patterns.get(&future_hour)
            .copied()
            .unwrap_or(current_util);

        // Combine predictions
        let base_prediction = current_util + trend * (horizon_ms as f64 / 1000.0 / 60.0);
        let seasonal_prediction = seasonal_adjustment;
        
        // Weight the predictions
        let predicted_utilization = base_prediction * 0.6 + seasonal_prediction * 0.4;

        // Clamp to valid range
        let predicted_utilization = predicted_utilization.clamp(0.0, 100.0);

        // Calculate confidence based on volatility and data quality
        let confidence = 1.0 - (volatility / 50.0).min(1.0);
        let confidence = confidence * (self.history.len() as f64 / 20.0).min(1.0);

        Some(WorkloadPrediction {
            predicted_utilization,
            confidence,
            recommended_frequency_mhz: 0, // Will be calculated by governor
            time_horizon: Duration::from_millis(horizon_ms),
        })
    }

    /// Update model parameters
    pub fn update_parameters(&mut self, prediction_error: f64) {
        // Adjust pattern weights based on prediction accuracy
        if prediction_error < 10.0 {
            // Good prediction, reinforce current weights
            self.short_term_pattern *= 1.01;
            self.medium_term_pattern *= 1.005;
        } else {
            // Poor prediction, adjust weights
            self.long_term_pattern *= 1.02;
            self.short_term_pattern *= 0.98;
        }

        // Normalize weights
        let total = self.short_term_pattern + self.medium_term_pattern + self.long_term_pattern;
        self.short_term_pattern /= total;
        self.medium_term_pattern /= total;
        self.long_term_pattern /= total;
    }
}

/// Smart CPU Governor
pub struct SmartCpuGovernor {
    config: GovernorConfig,
    
    /// Current metrics for each CPU core
    metrics: Arc<RwLock<HashMap<u32, CpuMetrics>>>,
    
    /// Prediction model
    model: Arc<RwLock<WorkloadPredictionModel>>,
    
    /// Current frequency settings
    current_frequencies: Arc<RwLock<HashMap<u32, u32>>>,
    
    /// Performance history
    performance_history: Arc<RwLock<VecDeque<(f64, f64)>>>,
}

impl SmartCpuGovernor {
    pub fn new(config: GovernorConfig) -> Self {
        let model = WorkloadPredictionModel::new(config.prediction_window);
        
        Self {
            config,
            metrics: Arc::new(RwLock::new(HashMap::new())),
            model: Arc::new(RwLock::new(model)),
            current_frequencies: Arc::new(RwLock::new(HashMap::new())),
            performance_history: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
        }
    }

    /// Update CPU metrics for a specific core
    pub async fn update_metrics(&self, core_id: u32, metrics: CpuMetrics) {
        {
            let mut metrics_map = self.metrics.write().await;
            metrics_map.insert(core_id, metrics.clone());
        }

        // Update prediction model
        {
            let mut model = self.model.write().await;
            model.add_metrics(metrics);
        }
    }

    /// Get recommended frequency for a CPU core
    pub async fn get_recommended_frequency(&self, core_id: u32) -> Option<u32> {
        let metrics_map = self.metrics.read().await;
        let metrics = metrics_map.get(&core_id)?;
        let utilization = metrics.workload.utilization;

        // Get prediction if enabled
        let predicted_utilization = if self.config.enable_prediction {
            let model = self.model.read().await;
            let prediction = model.predict(500); // 500ms horizon
            
            prediction.map(|p| {
                // Record prediction accuracy
                self.record_prediction(utilization, p.predicted_utilization).await;
                
                p.predicted_utilization
            })
        } else {
            None
        };

        // Calculate target utilization
        let target_utilization = match self.config.power_mode {
            PowerMode::Performance => 80.0,
            PowerMode::Balanced => 60.0,
            PowerMode::PowerSaver => 40.0,
        };

        // Combine current and predicted utilization
        let combined_utilization = if let Some(pred) = predicted_utilization {
            let prediction_weight = match self.config.power_mode {
                PowerMode::Performance => 0.3,
                PowerMode::Balanced => 0.5,
                PowerMode::PowerSaver => 0.7,
            };
            utilization * (1.0 - prediction_weight) + pred * prediction_weight
        } else {
            utilization
        };

        // Calculate recommended frequency
        let frequency_range = self.config.max_frequency_mhz - self.config.min_frequency_mhz;
        let utilization_ratio = combined_utilization / 100.0;
        
        let mut recommended = self.config.min_frequency_mhz 
            + (frequency_range as f64 * utilization_ratio.powf(1.0 / self.config.scaling_aggressiveness)) as u32;

        // Apply power mode adjustments
        recommended = match self.config.power_mode {
            PowerMode::Performance => recommended,
            PowerMode::Balanced => (recommended as f64 * 0.95) as u32,
            PowerMode::PowerSaver => (recommended as f64 * 0.85) as u32,
        };

        // Clamp to allowed range
        recommended = recommended.clamp(
            self.config.min_frequency_mhz,
            self.config.max_frequency_mhz
        );

        // Apply thermal throttling if needed
        if metrics.temperature_celsius > 80.0 {
            let thermal_factor = (85.0 - metrics.temperature_celsius) / 5.0;
            recommended = (recommended as f64 * thermal_factor.max(0.5)) as u32;
        }

        Some(recommended)
    }

    /// Apply frequency changes to CPU core
    pub async fn apply_frequency(&self, core_id: u32, frequency_mhz: u32) -> Result<(), String> {
        // Clamp frequency to allowed range
        let frequency_mhz = frequency_mhz.clamp(
            self.config.min_frequency_mhz,
            self.config.max_frequency_mhz
        );

        // Update current frequency
        {
            let mut freqs = self.current_frequencies.write().await;
            freqs.insert(core_id, frequency_mhz);
        }

        // In a real implementation, this would interface with the hardware
        // For now, we just record the change
        log::info!(
            "CPU Core {}: Setting frequency to {} MHz",
            core_id, frequency_mhz
        );

        Ok(())
    }

    /// Run the governor control loop
    pub async fn run(&self, num_cores: u32) {
        let mut interval = tokio::time::interval(Duration::from_millis(self.config.sampling_interval_ms));
        
        loop {
            interval.tick().await;

            for core_id in 0..num_cores {
                if let Some(recommended) = self.get_recommended_frequency(core_id).await {
                    // Check if change is significant enough
                    let current = {
                        let freqs = self.current_frequencies.read().await;
                        *freqs.get(&core_id).unwrap_or(&self.config.min_frequency_mhz)
                    };

                    let change_threshold = (self.config.max_frequency_mhz - self.config.min_frequency_mhz) as f64 * 0.05;
                    
                    if (recommended as i32 - current as i32).abs() as f64 > change_threshold {
                        if let Err(e) = self.apply_frequency(core_id, recommended).await {
                            log::error!("Failed to apply frequency: {}", e);
                        }
                    }
                }
            }
        }
    }

    /// Get current governor statistics
    pub async fn get_statistics(&self) -> GovernorStats {
        let metrics_map = self.metrics.read().await;
        let freqs = self.current_frequencies.read().await;
        
        let avg_utilization: f64 = metrics_map.values()
            .map(|m| m.workload.utilization)
            .sum::<f64>() / metrics_map.len().max(1) as f64;
        
        let avg_frequency: u32 = freqs.values()
            .sum::<u32>() / freqs.len().max(1);
        
        let avg_power: f64 = metrics_map.values()
            .map(|m| m.power_watts)
            .sum::<f64>() / metrics_map.len().max(1) as f64;
        
        let avg_temp: f64 = metrics_map.values()
            .map(|m| m.temperature_celsius)
            .sum::<f64>() / metrics_map.len().max(1) as f64;

        GovernorStats {
            avg_utilization,
            avg_frequency_mhz: avg_frequency,
            avg_power_watts: avg_power,
            avg_temperature_celsius: avg_temp,
            power_mode: self.config.power_mode,
        }
    }

    /// Record prediction accuracy
    async fn record_prediction(&self, actual: f64, predicted: f64) {
        let error = (actual - predicted).abs();
        
        let mut history = self.performance_history.write().await;
        history.push_back((actual, predicted));
        
        if history.len() > 100 {
            history.pop_front();
        }

        // Update model parameters based on prediction error
        if history.len() >= 10 {
            let avg_error: f64 = history.iter()
                .map(|(a, p)| (a - p).abs())
                .sum::<f64>() / history.len() as f64;
            
            let mut model = self.model.write().await;
            model.update_parameters(avg_error);
        }
    }
}

/// Governor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernorStats {
    pub avg_utilization: f64,
    pub avg_frequency_mhz: u32,
    pub avg_power_watts: f64,
    pub avg_temperature_celsius: f64,
    pub power_mode: PowerMode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governor_config() {
        let config = GovernorConfig {
            min_frequency_mhz: 800,
            max_frequency_mhz: 4000,
            sampling_interval_ms: 100,
            prediction_window: 100,
            scaling_aggressiveness: 0.8,
            power_mode: PowerMode::Balanced,
            enable_prediction: true,
        };

        assert_eq!(config.min_frequency_mhz, 800);
        assert_eq!(config.max_frequency_mhz, 4000);
    }

    #[test]
    fn test_workload_characteristics() {
        let workload = WorkloadCharacteristics {
            utilization: 50.0,
            context_switches: 1000,
            interrupts: 500,
            cache_miss_rate: 0.05,
            branch_accuracy: 0.95,
            memory_bandwidth_mbps: 5000.0,
        };

        assert_eq!(workload.utilization, 50.0);
        assert_eq!(workload.context_switches, 1000);
    }

    #[test]
    fn test_prediction_model() {
        let mut model = WorkloadPredictionModel::new(10);
        
        let metrics = CpuMetrics {
            current_frequency_mhz: 2000,
            temperature_celsius: 45.0,
            power_watts: 10.0,
            workload: WorkloadCharacteristics {
                utilization: 50.0,
                context_switches: 1000,
                interrupts: 500,
                cache_miss_rate: 0.05,
                branch_accuracy: 0.95,
                memory_bandwidth_mbps: 5000.0,
            },
            timestamp: Instant::now(),
        };

        model.add_metrics(metrics.clone());
        model.add_metrics(metrics);
        
        // With only 2 samples, prediction should be None
        assert!(model.predict(500).is_none());
    }

    #[test]
    fn test_governor_creation() {
        let config = GovernorConfig {
            min_frequency_mhz: 800,
            max_frequency_mhz: 4000,
            sampling_interval_ms: 100,
            prediction_window: 100,
            scaling_aggressiveness: 0.8,
            power_mode: PowerMode::Balanced,
            enable_prediction: true,
        };

        let governor = SmartCpuGovernor::new(config);
        
        // Should create successfully
        assert_eq!(governor.config.min_frequency_mhz, 800);
    }

    #[tokio::test]
    async fn test_metrics_update() {
        let config = GovernorConfig {
            min_frequency_mhz: 800,
            max_frequency_mhz: 4000,
            sampling_interval_ms: 100,
            prediction_window: 10,
            scaling_aggressiveness: 0.8,
            power_mode: PowerMode::Balanced,
            enable_prediction: true,
        };

        let governor = SmartCpuGovernor::new(config);
        
        let metrics = CpuMetrics {
            current_frequency_mhz: 2000,
            temperature_celsius: 45.0,
            power_watts: 10.0,
            workload: WorkloadCharacteristics {
                utilization: 50.0,
                context_switches: 1000,
                interrupts: 500,
                cache_miss_rate: 0.05,
                branch_accuracy: 0.95,
                memory_bandwidth_mbps: 5000.0,
            },
            timestamp: Instant::now(),
        };

        governor.update_metrics(0, metrics).await;
        
        let metrics_map = governor.metrics.read().await;
        assert!(metrics_map.contains_key(&0));
    }
}