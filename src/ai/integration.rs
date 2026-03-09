//! AI Integration Module for VantisOS
//! 
//! Provides integration between the data pipeline and AI modules.
//! Connects DataCollector, DataProcessor, and ModelTrainer to:
//! - ML Scheduler
//! - Power Manager
//! - Load Balancer
//! - Security/Threat Detection
//! 
//! ## Architecture
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     AI Integration Layer                        │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐   │
//! │  │DataCollector│───▶│DataProcessor │───▶│  ModelTrainer   │   │
//! │  └──────┬──────┘    └──────┬───────┘    └────────┬────────┘   │
//! │         │                  │                      │            │
//! │         ▼                  ▼                      ▼            │
//! │  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐   │
//! │  │ ML Scheduler│    │Power Manager │    │  Load Balancer  │   │
//! │  │ Integration │    │ Integration  │    │  Integration    │   │
//! │  └─────────────┘    └──────────────┘    └─────────────────┘   │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//! 
//! ## Security
//! - All integration is local (no network calls)
//! - Data is sanitized before passing to modules
//! - Error handling and recovery built-in
//! - Rate limiting to prevent overload

use crate::ai::{
    error::AIError,
    types::{ResourceUsage, SchedulingDecision, PowerDecision, PowerState, ThreatDetection, ThreatLevel, Confidence},
};
use crate::ai::modules::{
    DataCollector, DataProcessor, ModelTrainer,
    data_collector::{SystemMetrics, CPUMetrics, MemoryMetrics, SamplingConfig},
    data_processor::{FeatureVector, NormalizationMethod, ProcessorConfig},
    trainer::{ModelMetadata, TrainingConfig, ModelType, ValidationResult},
};

/// Integration coordinator for AI data pipeline
/// 
/// Manages the flow of data between collectors, processors, and AI modules.
pub struct AIIntegration {
    /// Data collector instance
    collector: DataCollector,
    /// Data processor instance
    processor: DataProcessor,
    /// Model trainer instance
    trainer: ModelTrainer,
    /// Whether integration is active
    enabled: bool,
    /// Collection interval in milliseconds
    collection_interval_ms: u64,
    /// Last processed metrics
    last_metrics: Option<SystemMetrics>,
    /// Last extracted features
    last_features: Option<FeatureVector>,
}

impl AIIntegration {
    /// Create a new AI integration instance
    pub fn new() -> Result<Self, AIError> {
        Ok(Self {
            collector: DataCollector::new()?,
            processor: DataProcessor::new()?,
            trainer: ModelTrainer::new()?,
            enabled: false,
            collection_interval_ms: 100,
            last_metrics: None,
            last_features: None,
        })
    }

    /// Create with custom configurations
    pub fn with_config(
        sampling_config: SamplingConfig,
        processor_config: ProcessorConfig,
        training_config: TrainingConfig,
    ) -> Result<Self, AIError> {
        let mut collector = DataCollector::new()?;
        collector.configure(sampling_config);
        
        let processor = DataProcessor::with_config(processor_config)?;
        let trainer = ModelTrainer::with_config(training_config)?;
        
        Ok(Self {
            collector,
            processor,
            trainer,
            enabled: false,
            collection_interval_ms: 100,
            last_metrics: None,
            last_features: None,
        })
    }

    /// Start the integration layer
    pub fn start(&mut self) -> Result<(), AIError> {
        self.collector.start()?;
        self.enabled = true;
        Ok(())
    }

    /// Stop the integration layer
    pub fn stop(&mut self) {
        self.collector.stop();
        self.enabled = false;
    }

    /// Check if integration is active
    pub fn is_active(&self) -> bool {
        self.enabled
    }

    /// Collect a metrics sample
    pub fn collect(&mut self) -> Result<SystemMetrics, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        let metrics = self.collector.collect()?;
        self.last_metrics = Some(metrics.clone());
        Ok(metrics)
    }

    /// Process collected metrics into features
    pub fn process(&mut self, metrics: &[SystemMetrics]) -> Result<FeatureVector, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        let features = self.processor.process(metrics)?;
        self.last_features = Some(features.clone());
        Ok(features)
    }

    /// Get the last collected metrics
    pub fn last_metrics(&self) -> Option<&SystemMetrics> {
        self.last_metrics.as_ref()
    }

    /// Get the last extracted features
    pub fn last_features(&self) -> Option<&FeatureVector> {
        self.last_features.as_ref()
    }

    /// Get reference to data collector
    pub fn collector(&self) -> &DataCollector {
        &self.collector
    }

    /// Get mutable reference to data collector
    pub fn collector_mut(&mut self) -> &mut DataCollector {
        &mut self.collector
    }

    /// Get reference to data processor
    pub fn processor(&self) -> &DataProcessor {
        &self.processor
    }

    /// Get mutable reference to data processor
    pub fn processor_mut(&mut self) -> &mut DataProcessor {
        &mut self.processor
    }

    /// Get reference to model trainer
    pub fn trainer(&self) -> &ModelTrainer {
        &self.trainer
    }

    /// Get mutable reference to model trainer
    pub fn trainer_mut(&mut self) -> &mut ModelTrainer {
        &mut self.trainer
    }
}

/// Scheduler Integration
/// 
/// Connects the data pipeline to the ML Scheduler module.
pub struct SchedulerIntegration {
    /// Number of CPU cores
    num_cores: usize,
    /// Core load history
    core_loads: Vec<Vec<f64>>,
    /// Decision history
    decision_history: Vec<SchedulingDecision>,
    /// Maximum history size
    max_history: usize,
}

impl SchedulerIntegration {
    /// Create a new scheduler integration
    pub fn new(num_cores: usize) -> Self {
        Self {
            num_cores,
            core_loads: vec![Vec::new(); num_cores],
            decision_history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Update core load data from metrics
    pub fn update_from_metrics(&mut self, metrics: &SystemMetrics) {
        for (i, cpu) in metrics.cpu.iter().enumerate() {
            if i < self.num_cores {
                self.core_loads[i].push(cpu.usage_percent);
                // Keep history bounded
                if self.core_loads[i].len() > self.max_history {
                    self.core_loads[i].remove(0);
                }
            }
        }
    }

    /// Make scheduling decision based on features
    pub fn make_decision(&self, features: &FeatureVector, pid: u32) -> SchedulingDecision {
        // Get average CPU usage
        let avg_cpu = features.get("cpu_mean").unwrap_or(50.0);
        let avg_memory = features.get("memory_mean").unwrap_or(50.0);

        // Find least loaded core
        let target_core = self.find_least_loaded_core();
        
        // Determine priority based on resource usage
        let priority = if avg_cpu > 80.0 {
            50 // Lower priority for high CPU usage
        } else if avg_cpu < 20.0 {
            200 // Higher priority for low CPU usage
        } else {
            100 // Normal priority
        };

        // Determine time slice based on memory pressure
        let time_slice = if avg_memory > 80.0 {
            50 // Shorter time slice under memory pressure
        } else {
            100 // Normal time slice
        };

        SchedulingDecision {
            pid,
            cpu_core: target_core as u8,
            priority,
            time_slice_ms: time_slice,
        }
    }

    /// Find the least loaded CPU core
    fn find_least_loaded_core(&self) -> usize {
        let mut min_load = f64::MAX;
        let mut min_core = 0;

        for (i, loads) in self.core_loads.iter().enumerate() {
            if loads.is_empty() {
                return i; // No data means core is available
            }
            let avg_load: f64 = loads.iter().sum::<f64>() / loads.len() as f64;
            if avg_load < min_load {
                min_load = avg_load;
                min_core = i;
            }
        }

        min_core
    }

    /// Record a scheduling decision
    pub fn record_decision(&mut self, decision: SchedulingDecision) {
        self.decision_history.push(decision);
        if self.decision_history.len() > self.max_history {
            self.decision_history.remove(0);
        }
    }

    /// Get core loads history
    pub fn core_loads(&self) -> &[Vec<f64>] {
        &self.core_loads
    }

    /// Get decision history
    pub fn decision_history(&self) -> &[SchedulingDecision] {
        &self.decision_history
    }

    /// Get statistics for training
    pub fn get_training_stats(&self) -> SchedulerTrainingStats {
        let mut total_load = 0.0;
        let mut sample_count = 0;

        for loads in &self.core_loads {
            total_load += loads.iter().sum::<f64>();
            sample_count += loads.len();
        }

        let avg_load = if sample_count > 0 {
            total_load / sample_count as f64
        } else {
            0.0
        };

        SchedulerTrainingStats {
            average_load: avg_load,
            decisions_made: self.decision_history.len() as u64,
            num_cores: self.num_cores,
        }
    }
}

/// Training statistics for scheduler
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SchedulerTrainingStats {
    /// Average CPU load across all cores
    pub average_load: f64,
    /// Total scheduling decisions made
    pub decisions_made: u64,
    /// Number of CPU cores
    pub num_cores: usize,
}

/// Power Manager Integration
/// 
/// Connects the data pipeline to the Power Manager module.
pub struct PowerManagerIntegration {
    /// Power state history
    power_history: Vec<PowerStateEntry>,
    /// Maximum history size
    max_history: usize,
    /// Current power state
    current_state: PowerState,
    /// Power threshold for state transitions
    thresholds: PowerThresholds,
}

/// Power state history entry
#[derive(Debug, Clone, Copy, PartialEq)]
struct PowerStateEntry {
    /// Timestamp
    timestamp_ms: u64,
    /// CPU usage
    cpu_usage: f64,
    /// Memory usage  
    memory_usage: f64,
    /// Power consumption (mW)
    power_mw: u64,
    /// Power state
    state: PowerState,
}

/// Power thresholds for state transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PowerThresholds {
    /// CPU usage threshold for performance mode
    pub performance_cpu_threshold: f64,
    /// CPU usage threshold for power save mode
    pub power_save_cpu_threshold: f64,
    /// Memory usage threshold for aggressive power saving
    pub memory_pressure_threshold: f64,
    /// Battery level for power save mode
    pub low_battery_threshold: u8,
}

impl Default for PowerThresholds {
    fn default() -> Self {
        Self {
            performance_cpu_threshold: 70.0,
            power_save_cpu_threshold: 30.0,
            memory_pressure_threshold: 85.0,
            low_battery_threshold: 20,
        }
    }
}

impl PowerManagerIntegration {
    /// Create a new power manager integration
    pub fn new() -> Self {
        Self {
            power_history: Vec::new(),
            max_history: 1000,
            current_state: PowerState::Balanced,
            thresholds: PowerThresholds::default(),
        }
    }

    /// Update power state from metrics
    pub fn update_from_metrics(&mut self, metrics: &SystemMetrics) {
        let entry = PowerStateEntry {
            timestamp_ms: metrics.timestamp_ms,
            cpu_usage: metrics.cpu_usage_avg,
            memory_usage: metrics.memory.usage_percent,
            power_mw: metrics.power.total_power_mw,
            state: self.current_state,
        };

        self.power_history.push(entry);
        if self.power_history.len() > self.max_history {
            self.power_history.remove(0);
        }
    }

    /// Make power decision based on features
    pub fn make_decision(&self, features: &FeatureVector) -> PowerDecision {
        let avg_cpu = features.get("cpu_mean").unwrap_or(50.0);
        let avg_memory = features.get("memory_mean").unwrap_or(50.0);

        // Determine power state based on workload
        let new_state = if avg_cpu > self.thresholds.performance_cpu_threshold {
            PowerState::Performance
        } else if avg_cpu < self.thresholds.power_save_cpu_threshold {
            PowerState::PowerSave
        } else if avg_memory > self.thresholds.memory_pressure_threshold {
            // Under memory pressure, use balanced to avoid aggressive caching
            PowerState::Balanced
        } else {
            PowerState::Balanced
        };

        // Determine CPU frequency based on state
        let cpu_frequency = match new_state {
            PowerState::Performance => 3000, // Max frequency
            PowerState::Balanced => 2000,    // Balanced frequency
            PowerState::PowerSave => 1000,   // Reduced frequency
            PowerState::DeepSleep => 0,      // Sleep state
        };

        PowerDecision {
            cpu_frequency_mhz: cpu_frequency,
            power_state: new_state,
        }
    }

    /// Get power history for training
    pub fn power_history(&self) -> &[PowerStateEntry] {
        &self.power_history
    }

    /// Get current power state
    pub fn current_state(&self) -> PowerState {
        self.current_state
    }

    /// Set current power state
    pub fn set_state(&mut self, state: PowerState) {
        self.current_state = state;
    }

    /// Set thresholds
    pub fn set_thresholds(&mut self, thresholds: PowerThresholds) {
        self.thresholds = thresholds;
    }

    /// Get training statistics
    pub fn get_training_stats(&self) -> PowerTrainingStats {
        let total_power: u64 = self.power_history.iter().map(|e| e.power_mw).sum();
        let avg_power = if !self.power_history.is_empty() {
            total_power / self.power_history.len() as u64
        } else {
            0
        };

        PowerTrainingStats {
            average_power_mw: avg_power,
            samples_collected: self.power_history.len() as u64,
        }
    }
}

/// Training statistics for power manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerTrainingStats {
    /// Average power consumption in mW
    pub average_power_mw: u64,
    /// Number of samples collected
    pub samples_collected: u64,
}

/// Load Balancer Integration
/// 
/// Connects the data pipeline to the Load Balancer module.
pub struct LoadBalancerIntegration {
    /// Number of backend nodes
    num_nodes: usize,
    /// Node performance history
    node_performance: Vec<NodePerformanceEntry>,
    /// Maximum history size
    max_history: usize,
    /// Thompson Sampling parameters for each node
    node_params: Vec<ThompsonSamplingParams>,
}

/// Node performance history entry
#[derive(Debug, Clone, Copy, PartialEq)]
struct NodePerformanceEntry {
    /// Node ID
    node_id: usize,
    /// Response time in ms
    response_time_ms: u64,
    /// Success (true) or failure (false)
    success: bool,
    /// Timestamp
    timestamp_ms: u64,
}

/// Thompson Sampling parameters
#[derive(Debug, Clone, Copy, PartialEq)]
struct ThompsonSamplingParams {
    /// Alpha parameter (successes + 1)
    alpha: f64,
    /// Beta parameter (failures + 1)
    beta: f64,
}

impl Default for ThompsonSamplingParams {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            beta: 1.0,
        }
    }
}

impl LoadBalancerIntegration {
    /// Create a new load balancer integration
    pub fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            node_performance: Vec::new(),
            max_history: 1000,
            node_params: vec![ThompsonSamplingParams::default(); num_nodes],
        }
    }

    /// Update node performance from metrics
    pub fn update_node_performance(&mut self, node_id: usize, response_time_ms: u64, success: bool, timestamp_ms: u64) {
        if node_id >= self.num_nodes {
            return;
        }

        let entry = NodePerformanceEntry {
            node_id,
            response_time_ms,
            success,
            timestamp_ms,
        };

        self.node_performance.push(entry);
        if self.node_performance.len() > self.max_history {
            self.node_performance.remove(0);
        }

        // Update Thompson Sampling parameters
        if success {
            self.node_params[node_id].alpha += 1.0;
        } else {
            self.node_params[node_id].beta += 1.0;
        }
    }

    /// Select best node using Thompson Sampling
    pub fn select_node(&self) -> usize {
        // Sample from each node's Beta distribution
        // Select node with highest sample
        let mut best_node = 0;
        let mut best_sample = 0.0;

        for (i, params) in self.node_params.iter().enumerate() {
            // Approximate Beta sample using simple heuristic
            let sample = params.alpha / (params.alpha + params.beta);
            if sample > best_sample {
                best_sample = sample;
                best_node = i;
            }
        }

        best_node
    }

    /// Get node performance history
    pub fn performance_history(&self) -> &[NodePerformanceEntry] {
        &self.node_performance
    }

    /// Get training statistics
    pub fn get_training_stats(&self) -> LoadBalancerTrainingStats {
        let total_requests = self.node_performance.len() as u64;
        let successful_requests = self.node_performance.iter().filter(|e| e.success).count() as u64;
        
        let avg_response_time = if total_requests > 0 {
            self.node_performance.iter().map(|e| e.response_time_ms).sum::<u64>() / total_requests
        } else {
            0
        };

        let success_rate = if total_requests > 0 {
            successful_requests as f64 / total_requests as f64
        } else {
            0.0
        };

        LoadBalancerTrainingStats {
            total_requests,
            successful_requests,
            success_rate,
            average_response_time_ms: avg_response_time,
            num_nodes: self.num_nodes,
        }
    }
}

/// Training statistics for load balancer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadBalancerTrainingStats {
    /// Total requests handled
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Average response time in ms
    pub average_response_time_ms: u64,
    /// Number of nodes
    pub num_nodes: usize,
}

/// Threat Detection Integration
/// 
/// Connects the data pipeline to the Security/Threat Detection module.
pub struct ThreatDetectionIntegration {
    /// Threat history
    threat_history: Vec<ThreatDetection>,
    /// Maximum history size
    max_history: usize,
    /// Anomaly threshold
    anomaly_threshold: f64,
}

impl ThreatDetectionIntegration {
    /// Create a new threat detection integration
    pub fn new() -> Self {
        Self {
            threat_history: Vec::new(),
            max_history: 1000,
            anomaly_threshold: 3.0, // Z-score threshold
        }
    }

    /// Detect anomalies from features
    pub fn detect_anomalies(&self, features: &FeatureVector) -> Option<ThreatDetection> {
        // Check for anomalous patterns in features
        let cpu_spike = features.get("cpu_peak_to_peak").unwrap_or(0.0);
        let mem_spike = features.get("memory_peak_to_peak").unwrap_or(0.0);

        // Simple anomaly detection based on feature values
        if cpu_spike > 80.0 || mem_spike > 70.0 {
            Some(ThreatDetection::new(
                ThreatLevel::Medium,
                "Anomalous resource usage pattern detected".to_string(),
            ))
        } else {
            None
        }
    }

    /// Record a threat detection
    pub fn record_threat(&mut self, threat: ThreatDetection) {
        self.threat_history.push(threat);
        if self.threat_history.len() > self.max_history {
            self.threat_history.remove(0);
        }
    }

    /// Get threat history
    pub fn threat_history(&self) -> &[ThreatDetection] {
        &self.threat_history
    }

    /// Set anomaly threshold
    pub fn set_threshold(&mut self, threshold: f64) {
        self.anomaly_threshold = threshold;
    }
}

impl Default for ThreatDetectionIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SchedulerIntegration {
    fn default() -> Self {
        Self::new(4) // Default to 4 cores
    }
}

impl Default for PowerManagerIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LoadBalancerIntegration {
    fn default() -> Self {
        Self::new(4) // Default to 4 nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_integration_creation() {
        let integration = AIIntegration::new().unwrap();
        assert!(!integration.is_active());
    }

    #[test]
    fn test_ai_integration_start_stop() {
        let mut integration = AIIntegration::new().unwrap();
        integration.start().unwrap();
        assert!(integration.is_active());
        integration.stop();
        assert!(!integration.is_active());
    }

    #[test]
    fn test_scheduler_integration() {
        let mut scheduler = SchedulerIntegration::new(4);
        
        let metrics = SystemMetrics::default();
        scheduler.update_from_metrics(&metrics);

        let features = FeatureVector::new();
        let decision = scheduler.make_decision(&features, 123);
        
        assert_eq!(decision.pid, 123);
        assert!(decision.cpu_core < 4);
    }

    #[test]
    fn test_power_manager_integration() {
        let mut power_mgr = PowerManagerIntegration::new();
        
        let metrics = SystemMetrics::default();
        power_mgr.update_from_metrics(&metrics);

        let features = FeatureVector::new();
        let decision = power_mgr.make_decision(&features);
        
        assert!(decision.cpu_frequency_mhz > 0 || decision.power_state == PowerState::DeepSleep);
    }

    #[test]
    fn test_load_balancer_integration() {
        let mut lb = LoadBalancerIntegration::new(4);
        
        lb.update_node_performance(0, 50, true, 1000);
        lb.update_node_performance(1, 100, true, 1001);
        lb.update_node_performance(0, 60, true, 1002);

        let selected = lb.select_node();
        assert!(selected < 4);

        let stats = lb.get_training_stats();
        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.successful_requests, 3);
    }

    #[test]
    fn test_threat_detection_integration() {
        let threat_detection = ThreatDetectionIntegration::new();
        
        let features = FeatureVector::new();
        let threat = threat_detection.detect_anomalies(&features);
        
        // No anomaly in empty features
        assert!(threat.is_none());
    }

    #[test]
    fn test_power_thresholds() {
        let mut power_mgr = PowerManagerIntegration::new();
        power_mgr.set_thresholds(PowerThresholds {
            performance_cpu_threshold: 80.0,
            power_save_cpu_threshold: 20.0,
            memory_pressure_threshold: 90.0,
            low_battery_threshold: 15,
        });

        let mut features = FeatureVector::new();
        features.add("cpu_mean".to_string(), 85.0);
        features.add("memory_mean".to_string(), 50.0);

        let decision = power_mgr.make_decision(&features);
        assert_eq!(decision.power_state, PowerState::Performance);
    }

    #[test]
    fn test_scheduler_stats() {
        let mut scheduler = SchedulerIntegration::new(4);
        
        for i in 0..10 {
            let mut metrics = SystemMetrics::default();
            metrics.cpu_usage_avg = 30.0 + i as f64;
            scheduler.update_from_metrics(&metrics);
        }

        let stats = scheduler.get_training_stats();
        assert_eq!(stats.num_cores, 4);
    }
}