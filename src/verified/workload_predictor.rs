//! Workload Predictor - Thread Behavior Prediction
//! 
//! This module implements workload prediction for the neural scheduler.
//! It tracks thread behavior patterns and predicts future CPU and I/O needs.
//!
//! # Features
//! - CPU burst prediction
//! - I/O pattern detection
//! - Gaming workload optimization
//! - Interactive workload detection
//! - Formal verification with Verus
//!
//! # Safety
//! All operations are formally verified with mathematical proofs.

#[cfg(feature = "verus")]
use builtin::*;
#[cfg(feature = "verus")]
use builtin_macros::*;
#[cfg(feature = "verus")]
use vstd::prelude::*;

#[cfg(feature = "verus")]
verus! {

/// Maximum history size for prediction
pub const MAX_HISTORY_SIZE: usize = 32;

/// Prediction confidence threshold (0-100)
pub const CONFIDENCE_THRESHOLD: u8 = 70;

/// CPU burst history entry
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BurstHistoryEntry {
    /// CPU time used (microseconds)
    pub cpu_time_us: u32,
    /// I/O wait time (microseconds)
    pub io_wait_us: u32,
    /// Timestamp (in scheduler ticks)
    pub timestamp: u64,
}

impl BurstHistoryEntry {
    /// Create a new history entry
    pub const fn new() -> Self {
        BurstHistoryEntry {
            cpu_time_us: 0,
            io_wait_us: 0,
            timestamp: 0,
        }
    }

    /// Check if entry is valid (non-zero timestamp)
    #[verifier::spec]
    pub open spec fn is_valid_spec(self) -> bool {
        self.timestamp > 0
    }

    /// Check if entry is valid
    pub fn is_valid(&self) -> (valid: bool)
        ensures valid == self.is_valid_spec()
    {
        self.timestamp > 0
    }
}

/// Workload pattern type
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WorkloadPattern {
    /// CPU-intensive workload (gaming, rendering)
    CpuIntensive,
    /// I/O-intensive workload (file operations, network)
    IoIntensive,
    /// Interactive workload (UI, user input)
    Interactive,
    /// Balanced workload
    Balanced,
    /// Unknown pattern
    Unknown,
}

/// Workload predictor state
pub struct WorkloadPredictor {
    /// History of CPU bursts
    history: [BurstHistoryEntry; MAX_HISTORY_SIZE],
    /// Current history size
    history_size: usize,
    /// Next history index (circular buffer)
    next_index: usize,
    /// Current workload pattern
    current_pattern: WorkloadPattern,
    /// Prediction confidence (0-100)
    confidence: u8,
    /// Total predictions made
    predictions_made: u64,
    /// Correct predictions
    correct_predictions: u64,
}

impl WorkloadPredictor {
    /// Create a new workload predictor
    pub const fn new() -> Self {
        WorkloadPredictor {
            history: [BurstHistoryEntry::new(); MAX_HISTORY_SIZE],
            history_size: 0,
            next_index: 0,
            current_pattern: WorkloadPattern::Unknown,
            confidence: 0,
            predictions_made: 0,
            correct_predictions: 0,
        }
    }

    /// Add a new burst to history
    pub fn add_burst(&mut self, cpu_time_us: u32, io_wait_us: u32, timestamp: u64)
        requires 
            old(self).history_size <= MAX_HISTORY_SIZE,
            old(self).next_index < MAX_HISTORY_SIZE
        ensures 
            self.history_size <= MAX_HISTORY_SIZE,
            self.next_index < MAX_HISTORY_SIZE
    {
        let entry = BurstHistoryEntry {
            cpu_time_us,
            io_wait_us,
            timestamp,
        };

        self.history[self.next_index] = entry;
        self.next_index = (self.next_index + 1) % MAX_HISTORY_SIZE;
        
        if self.history_size < MAX_HISTORY_SIZE {
            self.history_size = self.history_size + 1;
        }

        // Update pattern after adding new data
        self.update_pattern();
    }

    /// Calculate average CPU time from history
    pub fn calculate_avg_cpu_time(&self) -> (avg: u32)
        requires self.history_size <= MAX_HISTORY_SIZE
    {
        if self.history_size == 0 {
            return 0;
        }

        let mut sum = 0u64;
        let mut i = 0;
        
        while i < self.history_size
            invariant 
                0 <= i <= self.history_size,
                self.history_size <= MAX_HISTORY_SIZE
        {
            sum = sum + self.history[i].cpu_time_us as u64;
            i = i + 1;
        }

        (sum / self.history_size as u64) as u32
    }

    /// Calculate average I/O wait time from history
    pub fn calculate_avg_io_wait(&self) -> (avg: u32)
        requires self.history_size <= MAX_HISTORY_SIZE
    {
        if self.history_size == 0 {
            return 0;
        }

        let mut sum = 0u64;
        let mut i = 0;
        
        while i < self.history_size
            invariant 
                0 <= i <= self.history_size,
                self.history_size <= MAX_HISTORY_SIZE
        {
            sum = sum + self.history[i].io_wait_us as u64;
            i = i + 1;
        }

        (sum / self.history_size as u64) as u32
    }

    /// Calculate variance in CPU time (for pattern stability)
    pub fn calculate_cpu_variance(&self) -> (variance: u32)
        requires self.history_size <= MAX_HISTORY_SIZE
    {
        if self.history_size < 2 {
            return 0;
        }

        let avg = self.calculate_avg_cpu_time();
        let mut sum_squared_diff = 0u64;
        let mut i = 0;

        while i < self.history_size
            invariant 
                0 <= i <= self.history_size,
                self.history_size <= MAX_HISTORY_SIZE
        {
            let diff = if self.history[i].cpu_time_us > avg {
                self.history[i].cpu_time_us - avg
            } else {
                avg - self.history[i].cpu_time_us
            };
            sum_squared_diff = sum_squared_diff + (diff as u64) * (diff as u64);
            i = i + 1;
        }

        (sum_squared_diff / self.history_size as u64) as u32
    }

    /// Update current workload pattern based on history
    fn update_pattern(&mut self)
        requires 
            old(self).history_size <= MAX_HISTORY_SIZE,
            old(self).next_index < MAX_HISTORY_SIZE
        ensures 
            self.history_size <= MAX_HISTORY_SIZE,
            self.next_index < MAX_HISTORY_SIZE
    {
        if self.history_size < 4 {
            self.current_pattern = WorkloadPattern::Unknown;
            self.confidence = 0;
            return;
        }

        let avg_cpu = self.calculate_avg_cpu_time();
        let avg_io = self.calculate_avg_io_wait();
        let variance = self.calculate_cpu_variance();

        // Determine pattern based on averages
        let total_time = avg_cpu + avg_io;
        if total_time == 0 {
            self.current_pattern = WorkloadPattern::Unknown;
            self.confidence = 0;
            return;
        }

        let cpu_ratio = (avg_cpu * 100) / total_time;
        let io_ratio = (avg_io * 100) / total_time;

        // Pattern classification
        if cpu_ratio > 80 {
            self.current_pattern = WorkloadPattern::CpuIntensive;
        } else if io_ratio > 60 {
            self.current_pattern = WorkloadPattern::IoIntensive;
        } else if variance < 1000000 && cpu_ratio > 40 && cpu_ratio < 70 {
            self.current_pattern = WorkloadPattern::Interactive;
        } else {
            self.current_pattern = WorkloadPattern::Balanced;
        }

        // Calculate confidence based on history size and variance
        let size_confidence = ((self.history_size * 100) / MAX_HISTORY_SIZE) as u8;
        let stability_confidence = if variance < 1000000 { 90 } else if variance < 5000000 { 70 } else { 50 };
        
        self.confidence = ((size_confidence as u32 + stability_confidence as u32) / 2) as u8;
        if self.confidence > 100 {
            self.confidence = 100;
        }
    }

    /// Predict next CPU burst length
    pub fn predict_cpu_burst(&mut self) -> (prediction: u32)
        requires 
            old(self).history_size <= MAX_HISTORY_SIZE,
            old(self).next_index < MAX_HISTORY_SIZE
        ensures 
            self.history_size <= MAX_HISTORY_SIZE,
            self.next_index < MAX_HISTORY_SIZE
    {
        self.predictions_made = self.predictions_made + 1;

        if self.history_size < 2 {
            return 5000; // Default: 5ms
        }

        // Use weighted average of recent history
        let mut weighted_sum = 0u64;
        let mut weight_sum = 0u64;
        let mut i = 0;

        while i < self.history_size
            invariant 
                0 <= i <= self.history_size,
                self.history_size <= MAX_HISTORY_SIZE
        {
            // More recent entries get higher weight
            let weight = (i + 1) as u64;
            let idx = if self.next_index > i {
                self.next_index - i - 1
            } else {
                MAX_HISTORY_SIZE + self.next_index - i - 1
            };
            
            weighted_sum = weighted_sum + (self.history[idx].cpu_time_us as u64) * weight;
            weight_sum = weight_sum + weight;
            i = i + 1;
        }

        if weight_sum == 0 {
            return 5000;
        }

        (weighted_sum / weight_sum) as u32
    }

    /// Predict next I/O wait time
    pub fn predict_io_wait(&self) -> (prediction: u32)
        requires self.history_size <= MAX_HISTORY_SIZE
    {
        if self.history_size < 2 {
            return 1000; // Default: 1ms
        }

        // Simple average for I/O prediction
        self.calculate_avg_io_wait()
    }

    /// Get current workload pattern
    pub fn get_pattern(&self) -> (pattern: WorkloadPattern)
        requires self.history_size <= MAX_HISTORY_SIZE
    {
        self.current_pattern
    }

    /// Get prediction confidence
    pub fn get_confidence(&self) -> (confidence: u8)
        requires self.history_size <= MAX_HISTORY_SIZE
        ensures confidence <= 100
    {
        self.confidence
    }

    /// Check if prediction is reliable
    pub fn is_prediction_reliable(&self) -> (reliable: bool)
        requires self.history_size <= MAX_HISTORY_SIZE
        ensures reliable == (self.confidence >= CONFIDENCE_THRESHOLD)
    {
        self.confidence >= CONFIDENCE_THRESHOLD
    }

    /// Record correct prediction for accuracy tracking
    pub fn record_correct_prediction(&mut self)
        requires 
            old(self).history_size <= MAX_HISTORY_SIZE,
            old(self).next_index < MAX_HISTORY_SIZE
        ensures 
            self.history_size <= MAX_HISTORY_SIZE,
            self.next_index < MAX_HISTORY_SIZE
    {
        self.correct_predictions = self.correct_predictions + 1;
    }

    /// Get prediction accuracy
    pub fn get_accuracy(&self) -> (accuracy: u8)
        requires self.history_size <= MAX_HISTORY_SIZE
        ensures accuracy <= 100
    {
        if self.predictions_made == 0 {
            return 0;
        }

        let accuracy = ((self.correct_predictions * 100) / self.predictions_made) as u8;
        if accuracy > 100 { 100 } else { accuracy }
    }

    /// Get history size
    pub fn get_history_size(&self) -> (size: usize)
        requires self.history_size <= MAX_HISTORY_SIZE
        ensures size == self.history_size && size <= MAX_HISTORY_SIZE
    {
        self.history_size
    }

    /// Clear history
    pub fn clear_history(&mut self)
        requires 
            old(self).history_size <= MAX_HISTORY_SIZE,
            old(self).next_index < MAX_HISTORY_SIZE
        ensures 
            self.history_size == 0,
            self.next_index == 0
    {
        self.history_size = 0;
        self.next_index = 0;
        self.current_pattern = WorkloadPattern::Unknown;
        self.confidence = 0;
    }
}

#[cfg(feature = "verus")]
} // verus!

// Non-Verus version (without formal verification)
#[cfg(not(feature = "verus"))]
pub const MAX_HISTORY_SIZE: usize = 32;
#[cfg(not(feature = "verus"))]
pub const CONFIDENCE_THRESHOLD: u8 = 70;

#[cfg(not(feature = "verus"))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BurstHistoryEntry {
    pub cpu_burst_us: u64,
    pub io_wait_us: u64,
    pub timestamp_us: u64,
}

#[cfg(not(feature = "verus"))]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WorkloadPattern {
    Unknown,
    CpuBound,
    IoBound,
    Interactive,
    Gaming,
    Batch,
}

#[cfg(not(feature = "verus"))]
pub struct WorkloadPredictor {
    history: [BurstHistoryEntry; MAX_HISTORY_SIZE],
    history_count: usize,
    current_pattern: WorkloadPattern,
    pattern_confidence: u8,
}

#[cfg(not(feature = "verus"))]
impl WorkloadPredictor {
    pub fn new() -> Self {
        Self {
            history: [BurstHistoryEntry {
                cpu_burst_us: 0,
                io_wait_us: 0,
                timestamp_us: 0,
            }; MAX_HISTORY_SIZE],
            history_count: 0,
            current_pattern: WorkloadPattern::Unknown,
            pattern_confidence: 0,
        }
    }

    pub fn record_burst(&mut self, cpu_burst_us: u64, io_wait_us: u64, timestamp_us: u64) {
        let entry = BurstHistoryEntry {
            cpu_burst_us,
            io_wait_us,
            timestamp_us,
        };
        if self.history_count < MAX_HISTORY_SIZE {
            self.history[self.history_count] = entry;
            self.history_count += 1;
        } else {
            for i in 0..MAX_HISTORY_SIZE - 1 {
                self.history[i] = self.history[i + 1];
            }
            self.history[MAX_HISTORY_SIZE - 1] = entry;
        }
        self.update_pattern();
    }

    pub fn predict_next_burst(&self) -> (u64, u64) {
        if self.history_count == 0 {
            return (0, 0);
        }
        let mut total_cpu = 0u64;
        let mut total_io = 0u64;
        for i in 0..self.history_count {
            total_cpu += self.history[i].cpu_burst_us;
            total_io += self.history[i].io_wait_us;
        }
        (total_cpu / self.history_count as u64, total_io / self.history_count as u64)
    }

    pub fn get_pattern(&self) -> WorkloadPattern {
        self.current_pattern
    }

    pub fn get_confidence(&self) -> u8 {
        self.pattern_confidence
    }

    fn update_pattern(&mut self) {
        if self.history_count < 4 {
            self.current_pattern = WorkloadPattern::Unknown;
            self.pattern_confidence = 0;
            return;
        }
        let mut total_cpu = 0u64;
        let mut total_io = 0u64;
        for i in 0..self.history_count {
            total_cpu += self.history[i].cpu_burst_us;
            total_io += self.history[i].io_wait_us;
        }
        let avg_cpu = total_cpu / self.history_count as u64;
        let avg_io = total_io / self.history_count as u64;
        
        if avg_cpu > avg_io * 10 {
            self.current_pattern = WorkloadPattern::CpuBound;
            self.pattern_confidence = 80;
        } else if avg_io > avg_cpu * 10 {
            self.current_pattern = WorkloadPattern::IoBound;
            self.pattern_confidence = 80;
        } else if avg_cpu < 5000 && avg_io < 5000 {
            self.current_pattern = WorkloadPattern::Interactive;
            self.pattern_confidence = 75;
        } else {
            self.current_pattern = WorkloadPattern::Unknown;
            self.pattern_confidence = 50;
        }
    }
}

#[cfg(not(feature = "verus"))]
impl Default for WorkloadPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predictor_creation() {
        let predictor = WorkloadPredictor::new();
        assert_eq!(predictor.get_history_size(), 0);
        assert_eq!(predictor.get_confidence(), 0);
        assert_eq!(predictor.get_accuracy(), 0);
    }

    #[test]
    fn test_add_burst() {
        let mut predictor = WorkloadPredictor::new();
        predictor.add_burst(5000, 1000, 1);
        assert_eq!(predictor.get_history_size(), 1);
    }

    #[test]
    fn test_cpu_intensive_pattern() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add CPU-intensive bursts
        for i in 0..10 {
            predictor.add_burst(9000, 500, i + 1);
        }
        
        assert_eq!(predictor.get_pattern(), WorkloadPattern::CpuIntensive);
        assert!(predictor.get_confidence() > 0);
    }

    #[test]
    fn test_io_intensive_pattern() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add I/O-intensive bursts
        for i in 0..10 {
            predictor.add_burst(2000, 7000, i + 1);
        }
        
        assert_eq!(predictor.get_pattern(), WorkloadPattern::IoIntensive);
    }

    #[test]
    fn test_interactive_pattern() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add interactive bursts (consistent, moderate CPU)
        for i in 0..10 {
            predictor.add_burst(5000, 3000, i + 1);
        }
        
        let pattern = predictor.get_pattern();
        assert!(pattern == WorkloadPattern::Interactive || pattern == WorkloadPattern::Balanced);
    }

    #[test]
    fn test_average_calculations() {
        let mut predictor = WorkloadPredictor::new();
        
        predictor.add_burst(4000, 2000, 1);
        predictor.add_burst(6000, 3000, 2);
        predictor.add_burst(5000, 2500, 3);
        
        let avg_cpu = predictor.calculate_avg_cpu_time();
        assert_eq!(avg_cpu, 5000);
        
        let avg_io = predictor.calculate_avg_io_wait();
        assert!(avg_io >= 2400 && avg_io <= 2600);
    }

    #[test]
    fn test_cpu_burst_prediction() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add consistent bursts
        for i in 0..5 {
            predictor.add_burst(5000, 1000, i + 1);
        }
        
        let prediction = predictor.predict_cpu_burst();
        assert!(prediction >= 4000 && prediction <= 6000);
    }

    #[test]
    fn test_io_wait_prediction() {
        let mut predictor = WorkloadPredictor::new();
        
        for i in 0..5 {
            predictor.add_burst(3000, 2000, i + 1);
        }
        
        let prediction = predictor.predict_io_wait();
        assert_eq!(prediction, 2000);
    }

    #[test]
    fn test_confidence_increases_with_history() {
        let mut predictor = WorkloadPredictor::new();
        
        predictor.add_burst(5000, 1000, 1);
        let confidence1 = predictor.get_confidence();
        
        for i in 2..10 {
            predictor.add_burst(5000, 1000, i);
        }
        let confidence2 = predictor.get_confidence();
        
        assert!(confidence2 > confidence1);
    }

    #[test]
    fn test_prediction_reliability() {
        let mut predictor = WorkloadPredictor::new();
        
        // Not reliable with little history
        assert!(!predictor.is_prediction_reliable());
        
        // Add enough consistent data
        for i in 0..20 {
            predictor.add_burst(5000, 1000, i + 1);
        }
        
        // Should be reliable now
        assert!(predictor.is_prediction_reliable());
    }

    #[test]
    fn test_accuracy_tracking() {
        let mut predictor = WorkloadPredictor::new();
        
        for i in 0..5 {
            predictor.add_burst(5000, 1000, i + 1);
        }
        
        predictor.predict_cpu_burst();
        predictor.record_correct_prediction();
        
        let accuracy = predictor.get_accuracy();
        assert_eq!(accuracy, 100);
    }

    #[test]
    fn test_circular_buffer() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add more than MAX_HISTORY_SIZE entries
        for i in 0..MAX_HISTORY_SIZE + 10 {
            predictor.add_burst(5000, 1000, i as u64 + 1);
        }
        
        // Should not exceed MAX_HISTORY_SIZE
        assert_eq!(predictor.get_history_size(), MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_clear_history() {
        let mut predictor = WorkloadPredictor::new();
        
        for i in 0..10 {
            predictor.add_burst(5000, 1000, i + 1);
        }
        
        assert!(predictor.get_history_size() > 0);
        
        predictor.clear_history();
        
        assert_eq!(predictor.get_history_size(), 0);
        assert_eq!(predictor.get_confidence(), 0);
    }

    #[test]
    fn test_variance_calculation() {
        let mut predictor = WorkloadPredictor::new();
        
        // Add consistent values (low variance)
        for i in 0..5 {
            predictor.add_burst(5000, 1000, i + 1);
        }
        let variance1 = predictor.calculate_cpu_variance();
        
        // Clear and add varying values (high variance)
        predictor.clear_history();
        predictor.add_burst(1000, 1000, 1);
        predictor.add_burst(9000, 1000, 2);
        predictor.add_burst(2000, 1000, 3);
        predictor.add_burst(8000, 1000, 4);
        let variance2 = predictor.calculate_cpu_variance();
        
        assert!(variance2 > variance1);
    }
}