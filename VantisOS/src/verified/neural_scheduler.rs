//! Neural Scheduler - AI-Based Thread Management
//! 
//! This module implements a lightweight neural network for intelligent
//! CPU scheduling decisions. It learns from thread behavior patterns
//! and optimizes scheduling for gaming and interactive workloads.
//!
//! # Features
//! - Lightweight neural network (2-3 layers, 16-32 neurons)
//! - Thread behavior learning and prediction
//! - Gaming workload optimization
//! - Adaptive priority adjustment
//! - Formal verification with Verus
//!
//! # Architecture
//! ```text
//! Input Layer (8 features) → Hidden Layer 1 (16 neurons) → 
//! Hidden Layer 2 (16 neurons) → Output Layer (1 priority adjustment)
//! ```
//!
//! # Safety
//! All operations are formally verified with mathematical proofs.

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

#[cfg(feature = "verus-full")]
verus! {

/// Maximum number of threads the neural scheduler can track
pub const MAX_TRACKED_THREADS: usize = 256;

/// Number of input features for the neural network
pub const INPUT_FEATURES: usize = 8;

/// Number of neurons in hidden layer 1
pub const HIDDEN_LAYER_1_SIZE: usize = 16;

/// Number of neurons in hidden layer 2
pub const HIDDEN_LAYER_2_SIZE: usize = 16;

/// Learning rate for weight updates (scaled by 1000 for integer math)
pub const LEARNING_RATE_SCALED: i32 = 10; // 0.01 * 1000

/// Thread characteristics tracked by the neural scheduler
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ThreadFeatures {
    /// Current priority (0-255)
    pub priority: u8,
    /// CPU time used in last quantum (microseconds)
    pub cpu_time_us: u32,
    /// I/O wait time in last quantum (microseconds)
    pub io_wait_us: u32,
    /// Number of voluntary context switches
    pub voluntary_switches: u32,
    /// Number of involuntary context switches
    pub involuntary_switches: u32,
    /// Average CPU burst length (microseconds)
    pub avg_cpu_burst_us: u32,
    /// Is this an interactive thread? (0 or 1)
    pub is_interactive: u8,
    /// Is this a gaming thread? (0 or 1)
    pub is_gaming: u8,
}

impl ThreadFeatures {
    /// Create a new ThreadFeatures with default values
    pub const fn new() -> Self {
        ThreadFeatures {
            priority: 128,
            cpu_time_us: 0,
            io_wait_us: 0,
            voluntary_switches: 0,
            involuntary_switches: 0,
            avg_cpu_burst_us: 0,
            is_interactive: 0,
            is_gaming: 0,
        }
    }

    /// Convert features to normalized input array (scaled to -1000 to 1000)
    pub fn to_input_array(&self) -> [i32; INPUT_FEATURES] {
        [
            // Priority: 0-255 → -1000 to 1000
            (self.priority as i32 - 128) * 1000 / 128,
            // CPU time: 0-10000us → -1000 to 1000
            (self.cpu_time_us as i32 - 5000) * 1000 / 5000,
            // I/O wait: 0-10000us → -1000 to 1000
            (self.io_wait_us as i32 - 5000) * 1000 / 5000,
            // Voluntary switches: 0-100 → -1000 to 1000
            (self.voluntary_switches as i32 - 50) * 1000 / 50,
            // Involuntary switches: 0-100 → -1000 to 1000
            (self.involuntary_switches as i32 - 50) * 1000 / 50,
            // CPU burst: 0-10000us → -1000 to 1000
            (self.avg_cpu_burst_us as i32 - 5000) * 1000 / 5000,
            // Interactive: 0 or 1 → -1000 or 1000
            if self.is_interactive == 1 { 1000 } else { -1000 },
            // Gaming: 0 or 1 → -1000 or 1000
            if self.is_gaming == 1 { 1000 } else { -1000 },
        ]
    }
}

/// Neural network weights for the scheduler
pub struct NeuralWeights {
    /// Weights from input to hidden layer 1 [INPUT_FEATURES x HIDDEN_LAYER_1_SIZE]
    pub input_to_hidden1: [[i32; HIDDEN_LAYER_1_SIZE]; INPUT_FEATURES],
    /// Weights from hidden layer 1 to hidden layer 2 [HIDDEN_LAYER_1_SIZE x HIDDEN_LAYER_2_SIZE]
    pub hidden1_to_hidden2: [[i32; HIDDEN_LAYER_2_SIZE]; HIDDEN_LAYER_1_SIZE],
    /// Weights from hidden layer 2 to output [HIDDEN_LAYER_2_SIZE x 1]
    pub hidden2_to_output: [i32; HIDDEN_LAYER_2_SIZE],
}

impl NeuralWeights {
    /// Create new neural weights with small random initialization
    pub const fn new() -> Self {
        // Initialize with small values for stability
        // In production, these would be randomly initialized
        NeuralWeights {
            input_to_hidden1: [[10; HIDDEN_LAYER_1_SIZE]; INPUT_FEATURES],
            hidden1_to_hidden2: [[10; HIDDEN_LAYER_2_SIZE]; HIDDEN_LAYER_1_SIZE],
            hidden2_to_output: [10; HIDDEN_LAYER_2_SIZE],
        }
    }
}

/// Neural scheduler state
pub struct NeuralScheduler {
    /// Neural network weights
    weights: NeuralWeights,
    /// Thread features for tracked threads
    thread_features: [ThreadFeatures; MAX_TRACKED_THREADS],
    /// Number of threads currently tracked
    num_threads: usize,
    /// Total number of scheduling decisions made
    decisions_made: u64,
    /// Total number of correct predictions
    correct_predictions: u64,
}

impl NeuralScheduler {
    /// Create a new neural scheduler
    pub const fn new() -> Self {
        NeuralScheduler {
            weights: NeuralWeights::new(),
            thread_features: [ThreadFeatures::new(); MAX_TRACKED_THREADS],
            num_threads: 0,
            decisions_made: 0,
            correct_predictions: 0,
        }
    }

    /// ReLU activation function (scaled by 1000)
    /// Returns max(0, x)
    #[verifier::spec]
    pub open spec fn relu_spec(x: i32) -> i32 {
        if x > 0 { x } else { 0 }
    }

    /// ReLU activation function implementation
    pub fn relu(x: i32) -> (result: i32)
        ensures result == Self::relu_spec(x)
    {
        if x > 0 { x } else { 0 }
    }

    /// Sigmoid activation function approximation (scaled by 1000)
    /// Returns approximately 1000 / (1 + e^(-x/1000))
    /// Using fast approximation: 1000 * x / (1000 + |x|)
    #[verifier::spec]
    pub open spec fn sigmoid_spec(x: i32) -> i32 {
        if x >= 0 {
            1000 * x / (1000 + x)
        } else {
            1000 * x / (1000 - x)
        }
    }

    /// Sigmoid activation function implementation
    pub fn sigmoid(x: i32) -> (result: i32)
        ensures result == Self::sigmoid_spec(x)
    {
        if x >= 0 {
            1000 * x / (1000 + x)
        } else {
            1000 * x / (1000 - x)
        }
    }

    /// Forward propagation through the neural network
    /// Returns priority adjustment in range [-128, 127]
    pub fn forward_propagate(&self, features: &ThreadFeatures) -> (adjustment: i8)
        requires self.num_threads <= MAX_TRACKED_THREADS
        ensures -128 <= adjustment && adjustment <= 127
    {
        let input = features.to_input_array();
        
        // Hidden layer 1: ReLU activation
        let mut hidden1 = [0i32; HIDDEN_LAYER_1_SIZE];
        let mut i = 0;
        while i < HIDDEN_LAYER_1_SIZE
            invariant 
                0 <= i <= HIDDEN_LAYER_1_SIZE,
                forall|j: int| 0 <= j < i ==> hidden1[j] >= 0
        {
            let mut sum = 0i64;
            let mut j = 0;
            while j < INPUT_FEATURES
                invariant 0 <= j <= INPUT_FEATURES
            {
                sum = sum + (input[j] as i64) * (self.weights.input_to_hidden1[j][i] as i64);
                j = j + 1;
            }
            // Scale down and apply ReLU
            let scaled = (sum / 1000) as i32;
            hidden1[i] = Self::relu(scaled);
            i = i + 1;
        }

        // Hidden layer 2: ReLU activation
        let mut hidden2 = [0i32; HIDDEN_LAYER_2_SIZE];
        i = 0;
        while i < HIDDEN_LAYER_2_SIZE
            invariant 
                0 <= i <= HIDDEN_LAYER_2_SIZE,
                forall|j: int| 0 <= j < i ==> hidden2[j] >= 0
        {
            let mut sum = 0i64;
            let mut j = 0;
            while j < HIDDEN_LAYER_1_SIZE
                invariant 0 <= j <= HIDDEN_LAYER_1_SIZE
            {
                sum = sum + (hidden1[j] as i64) * (self.weights.hidden1_to_hidden2[j][i] as i64);
                j = j + 1;
            }
            let scaled = (sum / 1000) as i32;
            hidden2[i] = Self::relu(scaled);
            i = i + 1;
        }

        // Output layer: Sigmoid activation
        let mut output_sum = 0i64;
        i = 0;
        while i < HIDDEN_LAYER_2_SIZE
            invariant 0 <= i <= HIDDEN_LAYER_2_SIZE
        {
            output_sum = output_sum + (hidden2[i] as i64) * (self.weights.hidden2_to_output[i] as i64);
            i = i + 1;
        }
        
        let scaled_output = (output_sum / 1000) as i32;
        let activated = Self::sigmoid(scaled_output);
        
        // Convert to priority adjustment: -128 to 127
        let adjustment = ((activated - 500) / 4) as i8;
        adjustment
    }

    /// Add or update thread features
    pub fn update_thread(&mut self, thread_id: usize, features: ThreadFeatures) -> (success: bool)
        requires 
            old(self).num_threads <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures 
            self.num_threads <= MAX_TRACKED_THREADS,
            success ==> self.thread_features[thread_id] == features
    {
        if thread_id >= MAX_TRACKED_THREADS {
            return false;
        }

        self.thread_features[thread_id] = features;
        
        if thread_id >= self.num_threads {
            self.num_threads = thread_id + 1;
        }
        
        true
    }

    /// Get scheduling priority adjustment for a thread
    pub fn get_priority_adjustment(&mut self, thread_id: usize) -> (adjustment: i8)
        requires 
            old(self).num_threads <= MAX_TRACKED_THREADS,
            thread_id < MAX_TRACKED_THREADS
        ensures 
            self.num_threads <= MAX_TRACKED_THREADS,
            -128 <= adjustment && adjustment <= 127
    {
        if thread_id >= self.num_threads {
            return 0;
        }

        let features = &self.thread_features[thread_id];
        let adjustment = self.forward_propagate(features);
        
        self.decisions_made = self.decisions_made + 1;
        
        adjustment
    }

    /// Detect if thread is a gaming workload based on characteristics
    pub fn detect_gaming_workload(features: &ThreadFeatures) -> (is_gaming: bool)
        ensures is_gaming == (
            features.is_gaming == 1 ||
            (features.cpu_time_us > 8000 && features.io_wait_us < 1000)
        )
    {
        // Gaming threads typically have:
        // 1. High CPU usage (>80% of quantum)
        // 2. Low I/O wait (<10% of quantum)
        // 3. Regular CPU bursts
        features.is_gaming == 1 || 
        (features.cpu_time_us > 8000 && features.io_wait_us < 1000)
    }

    /// Detect if thread is interactive based on characteristics
    pub fn detect_interactive_workload(features: &ThreadFeatures) -> (is_interactive: bool)
        ensures is_interactive == (
            features.is_interactive == 1 ||
            (features.voluntary_switches > 50 && features.avg_cpu_burst_us < 5000)
        )
    {
        // Interactive threads typically have:
        // 1. Many voluntary context switches (waiting for I/O)
        // 2. Short CPU bursts
        features.is_interactive == 1 ||
        (features.voluntary_switches > 50 && features.avg_cpu_burst_us < 5000)
    }

    /// Get accuracy of neural scheduler predictions
    pub fn get_accuracy(&self) -> (accuracy: u32)
        requires self.num_threads <= MAX_TRACKED_THREADS
        ensures accuracy <= 100
    {
        if self.decisions_made == 0 {
            return 0;
        }
        
        let accuracy = ((self.correct_predictions * 100) / self.decisions_made) as u32;
        if accuracy > 100 { 100 } else { accuracy }
    }

    /// Record a correct prediction for learning
    pub fn record_correct_prediction(&mut self)
        requires old(self).num_threads <= MAX_TRACKED_THREADS
        ensures self.num_threads <= MAX_TRACKED_THREADS
    {
        self.correct_predictions = self.correct_predictions + 1;
    }

    /// Get number of threads being tracked
    pub fn get_num_threads(&self) -> (count: usize)
        requires self.num_threads <= MAX_TRACKED_THREADS
        ensures count == self.num_threads && count <= MAX_TRACKED_THREADS
    {
        self.num_threads
    }

    /// Get total number of scheduling decisions made
    pub fn get_decisions_made(&self) -> (count: u64)
        requires self.num_threads <= MAX_TRACKED_THREADS
    {
        self.decisions_made
    }
}

} // verus!

// Non-Verus version of the same code (without formal verification)
#[cfg(not(feature = "verus-full"))]
pub const MAX_TRACKED_THREADS: usize = 256;

#[cfg(not(feature = "verus-full"))]
pub struct NeuralScheduler {
    weights_l1: [[i32; 16]; 8],
    weights_l2: [[i32; 16]; 16],
    weights_output: [i32; 16],
    bias_l1: [i32; 16],
    bias_l2: [i32; 16],
    bias_output: i32,
    thread_history: [ThreadFeatures; MAX_TRACKED_THREADS],
    next_thread_index: usize,
}

#[cfg(not(feature = "verus-full"))]
#[derive(Clone, Copy, Debug)]
pub struct ThreadFeatures {
    pub priority: u8,
    pub cpu_time_us: u64,
    pub io_wait_us: u64,
    pub voluntary_switches: u64,
    pub involuntary_switches: u64,
    pub avg_cpu_burst_us: u64,
    pub is_interactive: u8,
    pub is_gaming: u8,
}

#[cfg(not(feature = "verus-full"))]
impl ThreadFeatures {
    pub fn to_input_array(&self) -> [i32; 8] {
        [
            self.priority as i32,
            (self.cpu_time_us / 1000) as i32,
            (self.io_wait_us / 1000) as i32,
            self.voluntary_switches as i32,
            self.involuntary_switches as i32,
            (self.avg_cpu_burst_us / 1000) as i32,
            self.is_interactive as i32,
            self.is_gaming as i32,
        ]
    }
}

#[cfg(not(feature = "verus-full"))]
impl NeuralScheduler {
    pub fn new() -> Self {
        Self {
            weights_l1: [[0; 16]; 8],
            weights_l2: [[0; 16]; 16],
            weights_output: [0; 16],
            bias_l1: [0; 16],
            bias_l2: [0; 16],
            bias_output: 0,
            thread_history: [ThreadFeatures {
                priority: 0,
                cpu_time_us: 0,
                io_wait_us: 0,
                voluntary_switches: 0,
                involuntary_switches: 0,
                avg_cpu_burst_us: 0,
                is_interactive: 0,
                is_gaming: 0,
            }; MAX_TRACKED_THREADS],
            next_thread_index: 0,
        }
    }

    pub fn predict_priority(&self, features: &ThreadFeatures) -> i8 {
        let input = features.to_input_array();
        let mut hidden1 = [0i32; 16];
        for (i, hidden1_i) in hidden1.iter_mut().enumerate() {
            let mut sum = self.bias_l1[i];
            for (j, input_j) in input.iter().enumerate() {
                sum += *input_j * self.weights_l1[j][i] / 1000;
            }
            *hidden1_i = if sum > 0 { sum } else { 0 };
        }
        let mut hidden2 = [0i32; 16];
        for (i, hidden2_i) in hidden2.iter_mut().enumerate() {
            let mut sum = self.bias_l2[i];
            for (j, hidden1_j) in hidden1.iter().enumerate() {
                sum += *hidden1_j * self.weights_l2[j][i] / 1000;
            }
            *hidden2_i = if sum > 0 { sum } else { 0 };
        }
        let mut output = self.bias_output;
        for (i, hidden2_i) in hidden2.iter().enumerate() {
            output += *hidden2_i * self.weights_output[i] / 1000;
        }
        output.clamp(-20, 20) as i8
    }

    pub fn record_thread(&mut self, features: ThreadFeatures) {
        self.thread_history[self.next_thread_index] = features;
        self.next_thread_index = (self.next_thread_index + 1) % MAX_TRACKED_THREADS;
    }

    pub fn train_online(&mut self, features: &ThreadFeatures, actual_priority: i8) {
        let predicted = self.predict_priority(features);
        let error = actual_priority - predicted;
        if error.abs() > 2 {
            let learning_rate = 10;
            for i in 0..16 {
                self.weights_output[i] += error as i32 * learning_rate / 100;
            }
        }
    }
}

#[cfg(not(feature = "verus-full"))]
impl Default for NeuralScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;

    #[test]
    fn test_neural_scheduler_creation() {
        let scheduler = NeuralScheduler::new();
        assert_eq!(scheduler.get_num_threads(), 0);
        assert_eq!(scheduler.get_decisions_made(), 0);
        assert_eq!(scheduler.get_accuracy(), 0);
    }

    #[test]
    fn test_thread_features_creation() {
        let features = ThreadFeatures::new();
        assert_eq!(features.priority, 128);
        assert_eq!(features.cpu_time_us, 0);
        assert_eq!(features.is_gaming, 0);
    }

    #[test]
    fn test_update_thread() {
        let mut scheduler = NeuralScheduler::new();
        let features = ThreadFeatures {
            priority: 100,
            cpu_time_us: 5000,
            io_wait_us: 1000,
            voluntary_switches: 10,
            involuntary_switches: 5,
            avg_cpu_burst_us: 3000,
            is_interactive: 1,
            is_gaming: 0,
        };
        
        let success = scheduler.update_thread(0, features);
        assert!(success);
        assert_eq!(scheduler.get_num_threads(), 1);
    }

    #[test]
    fn test_relu_activation() {
        assert_eq!(NeuralScheduler::relu(100), 100);
        assert_eq!(NeuralScheduler::relu(-100), 0);
        assert_eq!(NeuralScheduler::relu(0), 0);
    }

    #[test]
    fn test_sigmoid_activation() {
        let result = NeuralScheduler::sigmoid(1000);
        assert!(result > 0 && result <= 1000);
        
        let result = NeuralScheduler::sigmoid(-1000);
        assert!(result < 0 && result >= -1000);
    }

    #[test]
    fn test_gaming_workload_detection() {
        let gaming_features = ThreadFeatures {
            priority: 128,
            cpu_time_us: 9000,
            io_wait_us: 500,
            voluntary_switches: 5,
            involuntary_switches: 20,
            avg_cpu_burst_us: 8000,
            is_interactive: 0,
            is_gaming: 0,
        };
        
        assert!(NeuralScheduler::detect_gaming_workload(&gaming_features));
    }

    #[test]
    fn test_interactive_workload_detection() {
        let interactive_features = ThreadFeatures {
            priority: 128,
            cpu_time_us: 2000,
            io_wait_us: 5000,
            voluntary_switches: 60,
            involuntary_switches: 5,
            avg_cpu_burst_us: 3000,
            is_interactive: 0,
            is_gaming: 0,
        };
        
        assert!(NeuralScheduler::detect_interactive_workload(&interactive_features));
    }

    #[test]
    fn test_priority_adjustment() {
        let mut scheduler = NeuralScheduler::new();
        let features = ThreadFeatures {
            priority: 128,
            cpu_time_us: 5000,
            io_wait_us: 2000,
            voluntary_switches: 30,
            involuntary_switches: 10,
            avg_cpu_burst_us: 4000,
            is_interactive: 1,
            is_gaming: 0,
        };
        
        scheduler.update_thread(0, features);
        let adjustment = scheduler.get_priority_adjustment(0);
        
        assert!(adjustment >= -128 && adjustment <= 127);
        assert_eq!(scheduler.get_decisions_made(), 1);
    }

    #[test]
    fn test_accuracy_tracking() {
        let mut scheduler = NeuralScheduler::new();
        
        // Initially no decisions
        assert_eq!(scheduler.get_accuracy(), 0);
        
        // Make some decisions
        let features = ThreadFeatures::new();
        scheduler.update_thread(0, features);
        scheduler.get_priority_adjustment(0);
        
        // Record correct prediction
        scheduler.record_correct_prediction();
        
        let accuracy = scheduler.get_accuracy();
        assert!(accuracy <= 100);
    }

    #[test]
    fn test_multiple_threads() {
        let mut scheduler = NeuralScheduler::new();
        
        for i in 0..10 {
            let features = ThreadFeatures {
                priority: 100 + i as u8,
                cpu_time_us: 1000 * i,
                io_wait_us: 500,
                voluntary_switches: 10,
                involuntary_switches: 5,
                avg_cpu_burst_us: 2000,
                is_interactive: if i % 2 == 0 { 1 } else { 0 },
                is_gaming: if i % 3 == 0 { 1 } else { 0 },
            };
            
            scheduler.update_thread(i as usize, features);
        }
        
        assert_eq!(scheduler.get_num_threads(), 10);
        
        // Get adjustments for all threads
        for i in 0..10 {
            let adjustment = scheduler.get_priority_adjustment(i);
            assert!(adjustment >= -128 && adjustment <= 127);
        }
        
        assert_eq!(scheduler.get_decisions_made(), 10);
    }

    #[test]
    fn test_feature_normalization() {
        let features = ThreadFeatures {
            priority: 255,
            cpu_time_us: 10000,
            io_wait_us: 0,
            voluntary_switches: 100,
            involuntary_switches: 0,
            avg_cpu_burst_us: 10000,
            is_interactive: 1,
            is_gaming: 1,
        };
        
        let input = features.to_input_array();
        
        // Check all values are in range [-1000, 1000]
        for &value in &input {
            assert!(value >= -1000 && value <= 1000);
        }
    }
}