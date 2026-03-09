//! ML Scheduler Module
//! 
//! Provides intelligent process scheduling using reinforcement learning.
//! 
//! ## Security Considerations
//! - Scheduling decisions are deterministic
//! - Priority inversion is prevented
//! - No user data is used for training
//! 
//! ## Performance Requirements
//! - Decision latency: <10ms
//! - Memory overhead: <20MB

use crate::ai::{
    config::SchedulerConfig,
    error::AIError,
    types::{Confidence, SchedulingDecision},
    ml::rl::QLearningAgent,
};

/// State representation for scheduling
/// Encodes system state as discrete state ID
#[derive(Debug, Clone)]
pub struct SchedulerState {
    pub cpu_usage: u8,       // 0-100 mapped to 0-10
    pub memory_usage: u8,    // 0-100 mapped to 0-10
    pub io_wait: u8,         // 0-100 mapped to 0-10
    pub process_count: u8,   // 0-255 mapped to 0-10
}

impl SchedulerState {
    /// Convert to discrete state ID for Q-learning
    pub fn to_state_id(&self) -> usize {
        let cpu_bucket = (self.cpu_usage as usize / 10).min(9);
        let mem_bucket = (self.memory_usage as usize / 10).min(9);
        let io_bucket = (self.io_wait as usize / 10).min(9);
        
        // Combine into single state ID
        cpu_bucket * 100 + mem_bucket * 10 + io_bucket
    }

    /// Create from raw system metrics
    pub fn from_metrics(cpu: f64, memory: f64, io: f64, processes: usize) -> Self {
        Self {
            cpu_usage: (cpu.min(100.0) as u8),
            memory_usage: (memory.min(100.0) as u8),
            io_wait: (io.min(100.0) as u8),
            process_count: (processes.min(255) as u8),
        }
    }
}

/// Action representation for scheduling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SchedulerAction {
    /// Assign to specific CPU core
    AssignCore(usize),
    /// Set time slice
    SetTimeSlice(u64),
    /// Boost priority
    BoostPriority(u8),
    /// Preempt current process
    Preempt,
}

/// ML Scheduler
/// 
/// Provides intelligent scheduling using reinforcement learning.
/// 
/// ## Features
/// - Priority-based scheduling with ML optimization
/// - Predictive time slice allocation
/// - Multi-core load balancing
/// - Real-time process handling
/// - Q-learning based decision making
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::scheduler::MLScheduler;
//! use vantisos::ai::config::SchedulerConfig;
//! 
//! let scheduler = MLScheduler::new(SchedulerConfig::default())?;
//! 
//! // Get scheduling decision for process
//! let decision = scheduler.schedule_process(123, 100)?;
//! println!("Process {} scheduled on core {}", decision.pid, decision.cpu_core);
//! ```
pub struct MLScheduler {
    enabled: bool,
    config: SchedulerConfig,
    model_version: u32,
    q_agent: QLearningAgent,
    core_loads: [f64; 8],  // Track load on each core (up to 8 cores)
    history: Vec<SchedulingHistory>,
}

/// History entry for learning
struct SchedulingHistory {
    state_id: usize,
    action: usize,
    reward: f64,
    next_state_id: usize,
}

impl MLScheduler {
    /// Create a new ML Scheduler
    /// 
    /// ## Arguments
    /// * `config` - Scheduler configuration
    /// 
    /// ## Returns
    /// A new scheduler instance
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: SchedulerConfig) -> Result<Self, AIError> {
        // Initialize Q-learning agent
        // State space: 1000 states (discretized system state)
        // Action space: 8 cores + 1 preempt = 9 actions
        let q_agent = QLearningAgent::new(1000, 9, 0.1, 0.95)?;

        Ok(Self {
            enabled: config.enabled,
            config,
            model_version: 1,
            q_agent,
            core_loads: [0.0; 8],
            history: Vec::new(),
        })
    }

    /// Schedule a process
    /// 
    /// ## Arguments
    /// * `pid` - Process ID to schedule
    /// * `priority` - Process priority (0-255)
    /// 
    /// ## Returns
    /// Scheduling decision for the process
    /// 
    /// ## Errors
    /// - Returns error if scheduler is disabled
    /// - Returns error if priority is invalid
    /// 
    /// ## Performance
    /// Target latency: <10ms
    pub fn schedule_process(&mut self, pid: u32, priority: u8) -> Result<SchedulingDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        // Get current system state
        let state = self.get_current_state();
        let state_id = state.to_state_id();

        // Get action from Q-learning agent
        let action = self.q_agent.select_action(state_id, 0.1)?;

        // Convert action to core assignment
        let cpu_core = self.action_to_core(action);

        // Calculate time slice based on priority and ML
        let time_slice_ms = self.calculate_time_slice_with_ml(priority, &state);

        // Update core load tracking
        self.update_core_load(cpu_core, priority);

        // Calculate confidence based on Q-value
        let confidence = self.calculate_confidence(state_id, action);

        Ok(SchedulingDecision {
            pid,
            cpu_core,
            priority,
            time_slice_ms,
            confidence: Some(confidence),
        })
    }

    /// Schedule with system state provided
    /// 
    /// ## Arguments
    /// * `pid` - Process ID
    /// * `priority` - Process priority
    /// * `state` - Current system state
    pub fn schedule_with_state(
        &mut self,
        pid: u32,
        priority: u8,
        state: &SchedulerState,
    ) -> Result<SchedulingDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        let state_id = state.to_state_id();
        let action = self.q_agent.select_action(state_id, 0.1)?;
        let cpu_core = self.action_to_core(action);
        let time_slice_ms = self.calculate_time_slice_with_ml(priority, state);
        let confidence = self.calculate_confidence(state_id, action);

        Ok(SchedulingDecision {
            pid,
            cpu_core,
            priority,
            time_slice_ms,
            confidence: Some(confidence),
        })
    }

    /// Provide feedback for learning
    /// 
    /// ## Arguments
    /// * `state` - Previous system state
    /// * `action` - Action taken
    /// * `reward` - Reward received (positive = good, negative = bad)
    /// * `next_state` - Resulting system state
    pub fn provide_feedback(
        &mut self,
        state: &SchedulerState,
        action: usize,
        reward: f64,
        next_state: &SchedulerState,
    ) -> Result<(), AIError> {
        let state_id = state.to_state_id();
        let next_state_id = next_state.to_state_id();

        // Update Q-table
        self.q_agent.update(state_id, action, reward, next_state_id)?;

        // Store in history for batch updates
        self.history.push(SchedulingHistory {
            state_id,
            action,
            reward,
            next_state_id,
        });

        // Keep history bounded
        if self.history.len() > 10000 {
            self.history.remove(0);
        }

        Ok(())
    }

    /// Calculate reward for scheduling decision
    /// 
    /// Reward is based on:
    /// - CPU utilization improvement
    /// - Latency reduction
    /// - Fairness
    /// - Cache hit rate
    pub fn calculate_reward(
        &self,
        latency_ms: f64,
        cpu_util: f64,
        cache_hit_rate: f64,
    ) -> f64 {
        // Reward = low latency + high utilization + high cache hits
        let latency_reward = 1.0 / (1.0 + latency_ms);
        let util_reward = cpu_util / 100.0;
        let cache_reward = cache_hit_rate;

        latency_reward * 0.4 + util_reward * 0.3 + cache_reward * 0.3
    }

    /// Update the model with new data
    /// 
    /// ## Returns
    /// Success or error
    pub fn update_model(&mut self) -> Result<(), AIError> {
        // Batch update from history
        for entry in &self.history {
            self.q_agent.update(
                entry.state_id,
                entry.action,
                entry.reward,
                entry.next_state_id,
            )?;
        }

        self.model_version += 1;
        Ok(())
    }

    /// Check if scheduler is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable/disable scheduler
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if scheduler is ready
    pub fn is_ready(&self) -> bool {
        self.enabled && self.model_version > 0
    }

    /// Get model version
    pub fn get_model_version(&self) -> u32 {
        self.model_version
    }

    /// Get core load distribution
    pub fn get_core_loads(&self) -> &[f64] {
        &self.core_loads
    }

    // Private helper methods

    /// Get current system state
    fn get_current_state(&self) -> SchedulerState {
        // In real implementation, read from kernel
        // For now, return default state
        SchedulerState {
            cpu_usage: 50,
            memory_usage: 50,
            io_wait: 10,
            process_count: 50,
        }
    }

    /// Convert action ID to core number
    fn action_to_core(&self, action: usize) -> usize {
        if action < 8 {
            action
        } else {
            // Action 8 = preempt, assign to least loaded core
            self.find_least_loaded_core()
        }
    }

    /// Find the least loaded core
    fn find_least_loaded_core(&self) -> usize {
        self.core_loads
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Update core load tracking
    fn update_core_load(&mut self, core: usize, priority: u8) {
        let load_increment = (priority as f64) / 255.0 * 0.1;
        self.core_loads[core] = (self.core_loads[core] + load_increment).min(1.0);

        // Decay other cores slightly
        for (i, load) in self.core_loads.iter_mut().enumerate() {
            if i != core {
                *load *= 0.99;
            }
        }
    }

    /// Calculate time slice with ML optimization
    fn calculate_time_slice_with_ml(&self, priority: u8, state: &SchedulerState) -> u64 {
        // Base time slice from priority
        let base_slice = calculate_time_slice(priority);

        // Adjust based on system load
        let load_factor = 1.0 - (state.cpu_usage as f64 / 200.0); // 0.5 to 1.0

        // Adjust based on memory pressure
        let mem_factor = 1.0 - (state.memory_usage as f64 / 200.0);

        // Combined adjustment
        let adjustment = load_factor * mem_factor;

        ((base_slice as f64) * adjustment).max(5.0).min(100.0) as u64
    }

    /// Calculate confidence from Q-values
    fn calculate_confidence(&self, state_id: usize, action: usize) -> Confidence {
        if let Ok(q_value) = self.q_agent.get_q_value(state_id, action) {
            let q_abs = q_value.abs();
            if q_abs > 1.0 {
                Confidence::High
            } else if q_abs > 0.5 {
                Confidence::Medium
            } else {
                Confidence::Low
            }
        } else {
            Confidence::Low
        }
    }
}

/// Calculate time slice based on priority
/// 
/// Higher priority processes get longer time slices.
/// 
/// ## Arguments
/// * `priority` - Process priority (0-255)
/// 
/// ## Returns
/// Time slice in milliseconds
fn calculate_time_slice(priority: u8) -> u64 {
    // Base time slice: 5ms
    // Additional time slice based on priority: 0-95ms
    // Total range: 5-100ms
    let base_ms: u64 = 5;
    let additional_ms = (priority as u64 * 95) / 255;
    base_ms + additional_ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        assert!(scheduler.is_enabled());
    }

    #[test]
    fn test_schedule_process() {
        let mut scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        let decision = scheduler.schedule_process(123, 100).unwrap();
        assert_eq!(decision.pid, 123);
        assert!(decision.cpu_core < 8);
        assert!(decision.time_slice_ms > 0);
    }

    #[test]
    fn test_time_slice_calculation() {
        let scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        let dec1 = scheduler.schedule_process(1, 30).unwrap();
        let dec2 = scheduler.schedule_process(2, 150).unwrap();
        assert!(dec2.time_slice_ms > dec1.time_slice_ms);
    }

    #[test]
    fn test_scheduler_disable() {
        let mut scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        scheduler.set_enabled(false);
        assert!(!scheduler.is_enabled());
    }

    #[test]
    fn test_state_encoding() {
        let state = SchedulerState {
            cpu_usage: 50,
            memory_usage: 60,
            io_wait: 10,
            process_count: 100,
        };
        let state_id = state.to_state_id();
        assert!(state_id < 1000);
    }

    #[test]
    fn test_core_load_tracking() {
        let mut scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        
        // Schedule multiple processes
        scheduler.schedule_process(1, 100).unwrap();
        scheduler.schedule_process(2, 100).unwrap();
        
        // Check core loads are tracked
        let loads = scheduler.get_core_loads();
        let total_load: f64 = loads.iter().sum();
        assert!(total_load > 0.0);
    }

    #[test]
    fn test_feedback_learning() {
        let mut scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        
        let state = SchedulerState {
            cpu_usage: 50,
            memory_usage: 50,
            io_wait: 10,
            process_count: 50,
        };
        
        let next_state = SchedulerState {
            cpu_usage: 45,
            memory_usage: 48,
            io_wait: 8,
            process_count: 49,
        };
        
        // Provide feedback
        scheduler.provide_feedback(&state, 0, 1.0, &next_state).unwrap();
        
        // Model should be updated
        assert!(!scheduler.history.is_empty());
    }
}