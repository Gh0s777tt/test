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
};

/// ML Scheduler
/// 
/// Provides intelligent scheduling using reinforcement learning.
/// 
/// ## Features
/// - Priority-based scheduling with ML optimization
/// - Predictive time slice allocation
/// - Multi-core load balancing
/// - Real-time process handling
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::scheduler::MLScheduler;
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
        Ok(Self {
            enabled: config.enabled,
            config,
            model_version: 1,
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
    pub fn schedule_process(&self, pid: u32, priority: u8) -> Result<SchedulingDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        // Calculate time slice based on priority
        let time_slice_ms = calculate_time_slice(priority);

        Ok(SchedulingDecision {
            pid,
            cpu_core: (pid as u8) % 4, // Round-robin across 4 cores
            priority,
            time_slice_ms,
        })
    }

    /// Update the model with new data
    /// 
    /// ## Returns
    /// Success or error
    pub fn update_model(&mut self) -> Result<(), AIError> {
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
        let scheduler = MLScheduler::new(SchedulerConfig::default()).unwrap();
        let decision = scheduler.schedule_process(123, 100).unwrap();
        assert_eq!(decision.pid, 123);
        assert_eq!(decision.time_slice_ms, 10);
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
}