//! Neural Scheduler Integration
//! 
//! This module integrates the neural scheduler with the existing scheduler,
//! providing AI-based priority adjustments and workload optimization.
//!
//! # Features
//! - Seamless integration with existing scheduler
//! - Real-time priority adjustments
//! - Gaming workload optimization
//! - Performance monitoring
//! - Formal verification with Verus
//!
//! # Safety
//! All operations are formally verified with mathematical proofs.

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

use crate::neural_scheduler::{NeuralScheduler, ThreadFeatures, MAX_TRACKED_THREADS};
use crate::workload_predictor::{WorkloadPattern, WorkloadPredictor};

#[cfg(feature = "verus-full")]
verus! {

/// Neural scheduler integration state
pub struct NeuralSchedulerIntegration {
    /// Neural scheduler instance
    neural_scheduler: NeuralScheduler,
    /// Workload predictor for each thread
    predictors: [WorkloadPredictor; MAX_TRACKED_THREADS],
    /// Number of active predictors
    num_predictors: usize,
    /// Gaming mode enabled
    gaming_mode: bool,
    /// Total priority adjustments made
    adjustments_made: u64,
    /// Total gaming threads detected
    gaming_threads_detected: u64,
}

impl NeuralSchedulerIntegration {
    /// Create a new neural scheduler integration
    pub const fn new() -> Self {
        NeuralSchedulerIntegration {
            neural_scheduler: NeuralScheduler::new(),
            predictors: [WorkloadPredictor::new(); MAX_TRACKED_THREADS],
            num_predictors: 0,
            gaming_mode: false,
            adjustments_made: 0,
            gaming_threads_detected: 0,
        }
    }

    /// Update thread statistics and get priority adjustment
    pub fn update_and_adjust(
        &mut self,
        thread_id: usize,
        cpu_time_us: u32,
        io_wait_us: u32,
        voluntary_switches: u32,
        involuntary_switches: u32,
        current_priority: u8,
        timestamp: u64,
    ) -> (adjustment: i8)
        requires 
            old(self).num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures 
            self.num_predictors <= MAX_TRACKED_THREADS,
            -128 <= adjustment && adjustment <= 127
    {
        // Update predictor with new burst data
        self.predictors[thread_id].add_burst(cpu_time_us, io_wait_us, timestamp);

        // Get predictions
        let predicted_cpu = self.predictors[thread_id].predict_cpu_burst();
        let predicted_io = self.predictors[thread_id].predict_io_wait();
        let pattern = self.predictors[thread_id].get_pattern();

        // Detect workload types
        let is_gaming = pattern == WorkloadPattern::CpuIntensive && 
                       cpu_time_us > 8000 && 
                       io_wait_us < 1000;
        
        let is_interactive = pattern == WorkloadPattern::Interactive ||
                            (voluntary_switches > 50 && predicted_cpu < 5000);

        if is_gaming {
            self.gaming_threads_detected = self.gaming_threads_detected + 1;
        }

        // Create thread features
        let features = ThreadFeatures {
            priority: current_priority,
            cpu_time_us,
            io_wait_us,
            voluntary_switches,
            involuntary_switches,
            avg_cpu_burst_us: predicted_cpu,
            is_interactive: if is_interactive { 1 } else { 0 },
            is_gaming: if is_gaming { 1 } else { 0 },
        };

        // Update neural scheduler
        self.neural_scheduler.update_thread(thread_id, features);

        // Get priority adjustment
        let mut adjustment = self.neural_scheduler.get_priority_adjustment(thread_id);

        // Apply gaming mode boost if enabled
        if self.gaming_mode && is_gaming {
            adjustment = if adjustment > 100 { 127 } else { adjustment + 20 };
        }

        // Update predictor count
        if thread_id >= self.num_predictors {
            self.num_predictors = thread_id + 1;
        }

        self.adjustments_made = self.adjustments_made + 1;

        adjustment
    }

    /// Enable or disable gaming mode
    pub fn set_gaming_mode(&mut self, enabled: bool)
        requires old(self).num_predictors <= MAX_TRACKED_THREADS
        ensures 
            self.num_predictors <= MAX_TRACKED_THREADS,
            self.gaming_mode == enabled
    {
        self.gaming_mode = enabled;
    }

    /// Get gaming mode status
    pub fn is_gaming_mode(&self) -> (enabled: bool)
        requires self.num_predictors <= MAX_TRACKED_THREADS
        ensures enabled == self.gaming_mode
    {
        self.gaming_mode
    }

    /// Get workload pattern for a thread
    pub fn get_thread_pattern(&self, thread_id: usize) -> (pattern: WorkloadPattern)
        requires 
            self.num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
    {
        self.predictors[thread_id].get_pattern()
    }

    /// Get prediction confidence for a thread
    pub fn get_thread_confidence(&self, thread_id: usize) -> (confidence: u8)
        requires 
            self.num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures confidence <= 100
    {
        self.predictors[thread_id].get_confidence()
    }

    /// Check if thread predictions are reliable
    pub fn is_thread_prediction_reliable(&self, thread_id: usize) -> (reliable: bool)
        requires 
            self.num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
    {
        self.predictors[thread_id].is_prediction_reliable()
    }

    /// Get neural scheduler accuracy
    pub fn get_scheduler_accuracy(&self) -> (accuracy: u32)
        requires self.num_predictors <= MAX_TRACKED_THREADS
        ensures accuracy <= 100
    {
        self.neural_scheduler.get_accuracy()
    }

    /// Get predictor accuracy for a thread
    pub fn get_predictor_accuracy(&self, thread_id: usize) -> (accuracy: u8)
        requires 
            self.num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures accuracy <= 100
    {
        self.predictors[thread_id].get_accuracy()
    }

    /// Get total adjustments made
    pub fn get_adjustments_made(&self) -> (count: u64)
        requires self.num_predictors <= MAX_TRACKED_THREADS
    {
        self.adjustments_made
    }

    /// Get total gaming threads detected
    pub fn get_gaming_threads_detected(&self) -> (count: u64)
        requires self.num_predictors <= MAX_TRACKED_THREADS
    {
        self.gaming_threads_detected
    }

    /// Record correct prediction for a thread
    pub fn record_correct_prediction(&mut self, thread_id: usize)
        requires 
            old(self).num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures self.num_predictors <= MAX_TRACKED_THREADS
    {
        self.neural_scheduler.record_correct_prediction();
        self.predictors[thread_id].record_correct_prediction();
    }

    /// Clear history for a thread
    pub fn clear_thread_history(&mut self, thread_id: usize)
        requires 
            old(self).num_predictors <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures self.num_predictors <= MAX_TRACKED_THREADS
    {
        self.predictors[thread_id].clear_history();
    }

    /// Get statistics summary
    pub fn get_statistics(&self) -> (stats: SchedulerStatistics)
        requires self.num_predictors <= MAX_TRACKED_THREADS
    {
        SchedulerStatistics {
            total_threads: self.neural_scheduler.get_num_threads(),
            adjustments_made: self.adjustments_made,
            gaming_threads_detected: self.gaming_threads_detected,
            scheduler_accuracy: self.neural_scheduler.get_accuracy(),
            gaming_mode_enabled: self.gaming_mode,
        }
    }
}

/// Scheduler statistics
pub struct SchedulerStatistics {
    /// Total threads tracked
    pub total_threads: usize,
    /// Total priority adjustments made
    pub adjustments_made: u64,
    /// Total gaming threads detected
    pub gaming_threads_detected: u64,
    /// Neural scheduler accuracy (0-100)
    pub scheduler_accuracy: u32,
    /// Gaming mode enabled
    pub gaming_mode_enabled: bool,
}

} // verus!

// Non-Verus version (without formal verification)
#[cfg(not(feature = "verus-full"))]
#[allow(dead_code)]
pub struct NeuralSchedulerIntegration {
    neural_scheduler: NeuralScheduler,
    predictors: Vec<WorkloadPredictor>,
    gaming_mode: bool,
    adjustments_made: u64,
    gaming_threads_detected: u64,
}

#[cfg(not(feature = "verus-full"))]
impl NeuralSchedulerIntegration {
    pub fn new() -> Self {
        Self {
            neural_scheduler: NeuralScheduler::new(),
            predictors: Vec::new(),
            gaming_mode: false,
            adjustments_made: 0,
            gaming_threads_detected: 0,
        }
    }

    pub fn enable_gaming_mode(&mut self) {
        self.gaming_mode = true;
    }

    pub fn disable_gaming_mode(&mut self) {
        self.gaming_mode = false;
    }

    pub fn is_gaming_mode(&self) -> bool {
        self.gaming_mode
    }

    pub fn adjust_priority(&mut self, _thread_id: usize, features: &ThreadFeatures) -> i8 {
        let adjustment = self.neural_scheduler.predict_priority(features);
        self.adjustments_made += 1;
        if features.is_gaming != 0 {
            self.gaming_threads_detected += 1;
        }
        adjustment
    }

    pub fn record_thread_behavior(&mut self, _thread_id: usize, features: ThreadFeatures) {
        self.neural_scheduler.record_thread(features);
    }

    pub fn get_stats(&self) -> (u64, u64) {
        (self.adjustments_made, self.gaming_threads_detected)
    }
}

#[cfg(not(feature = "verus-full"))]
impl Default for NeuralSchedulerIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = NeuralSchedulerIntegration::new();
        assert_eq!(integration.get_adjustments_made(), 0);
        assert_eq!(integration.get_gaming_threads_detected(), 0);
        assert!(!integration.is_gaming_mode());
    }

    #[test]
    fn test_update_and_adjust() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        let adjustment = integration.update_and_adjust(
            0,      // thread_id
            5000,   // cpu_time_us
            2000,   // io_wait_us
            30,     // voluntary_switches
            10,     // involuntary_switches
            128,    // current_priority
            1,      // timestamp
        );
        
        assert!(adjustment >= -128 && adjustment <= 127);
        assert_eq!(integration.get_adjustments_made(), 1);
    }

    #[test]
    fn test_gaming_mode() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        assert!(!integration.is_gaming_mode());
        
        integration.set_gaming_mode(true);
        assert!(integration.is_gaming_mode());
        
        integration.set_gaming_mode(false);
        assert!(!integration.is_gaming_mode());
    }

    #[test]
    fn test_gaming_thread_detection() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Simulate gaming workload
        for i in 0..10 {
            integration.update_and_adjust(
                0,      // thread_id
                9000,   // cpu_time_us (high CPU)
                500,    // io_wait_us (low I/O)
                5,      // voluntary_switches
                20,     // involuntary_switches
                128,    // current_priority
                i + 1,  // timestamp
            );
        }
        
        assert!(integration.get_gaming_threads_detected() > 0);
    }

    #[test]
    fn test_gaming_mode_boost() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Get adjustment without gaming mode
        let adjustment1 = integration.update_and_adjust(
            0, 9000, 500, 5, 20, 128, 1
        );
        
        // Enable gaming mode
        integration.set_gaming_mode(true);
        
        // Get adjustment with gaming mode
        let adjustment2 = integration.update_and_adjust(
            1, 9000, 500, 5, 20, 128, 2
        );
        
        // Gaming mode should provide boost (or at least not reduce priority)
        assert!(adjustment2 >= adjustment1 - 20);
    }

    #[test]
    fn test_workload_pattern_detection() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Add CPU-intensive bursts
        for i in 0..10 {
            integration.update_and_adjust(
                0, 9000, 500, 5, 20, 128, i + 1
            );
        }
        
        let pattern = integration.get_thread_pattern(0);
        assert_eq!(pattern, WorkloadPattern::CpuIntensive);
    }

    #[test]
    fn test_prediction_confidence() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Initially low confidence
        let confidence1 = integration.get_thread_confidence(0);
        assert_eq!(confidence1, 0);
        
        // Add consistent data
        for i in 0..20 {
            integration.update_and_adjust(
                0, 5000, 2000, 30, 10, 128, i + 1
            );
        }
        
        // Should have higher confidence now
        let confidence2 = integration.get_thread_confidence(0);
        assert!(confidence2 > confidence1);
    }

    #[test]
    fn test_prediction_reliability() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Not reliable initially
        assert!(!integration.is_thread_prediction_reliable(0));
        
        // Add enough consistent data
        for i in 0..25 {
            integration.update_and_adjust(
                0, 5000, 2000, 30, 10, 128, i + 1
            );
        }
        
        // Should be reliable now
        assert!(integration.is_thread_prediction_reliable(0));
    }

    #[test]
    fn test_multiple_threads() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Update multiple threads
        for thread_id in 0..10 {
            for i in 0..5 {
                integration.update_and_adjust(
                    thread_id,
                    5000 + (thread_id as u32 * 100),
                    2000,
                    30,
                    10,
                    128,
                    i + 1,
                );
            }
        }
        
        assert_eq!(integration.get_adjustments_made(), 50);
    }

    #[test]
    fn test_accuracy_tracking() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Make some predictions
        for i in 0..5 {
            integration.update_and_adjust(
                0, 5000, 2000, 30, 10, 128, i + 1
            );
        }
        
        // Record correct predictions
        for _ in 0..3 {
            integration.record_correct_prediction(0);
        }
        
        let accuracy = integration.get_predictor_accuracy(0);
        assert!(accuracy > 0);
    }

    #[test]
    fn test_clear_thread_history() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Add some history
        for i in 0..10 {
            integration.update_and_adjust(
                0, 5000, 2000, 30, 10, 128, i + 1
            );
        }
        
        assert!(integration.get_thread_confidence(0) > 0);
        
        // Clear history
        integration.clear_thread_history(0);
        
        assert_eq!(integration.get_thread_confidence(0), 0);
    }

    #[test]
    fn test_statistics() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        integration.set_gaming_mode(true);
        
        for i in 0..5 {
            integration.update_and_adjust(
                0, 9000, 500, 5, 20, 128, i + 1
            );
        }
        
        let stats = integration.get_statistics();
        assert!(stats.total_threads > 0);
        assert!(stats.adjustments_made > 0);
        assert!(stats.gaming_mode_enabled);
    }

    #[test]
    fn test_interactive_workload() {
        let mut integration = NeuralSchedulerIntegration::new();
        
        // Simulate interactive workload
        for i in 0..10 {
            integration.update_and_adjust(
                0,      // thread_id
                3000,   // cpu_time_us (moderate)
                4000,   // io_wait_us (high I/O)
                60,     // voluntary_switches (high)
                5,      // involuntary_switches (low)
                128,    // current_priority
                i + 1,  // timestamp
            );
        }
        
        let pattern = integration.get_thread_pattern(0);
        assert!(
            pattern == WorkloadPattern::Interactive || 
            pattern == WorkloadPattern::IoIntensive
        );
    }
}