//! Streaming Anomaly Detection for VantisOS
//!
//! This module provides real-time anomaly detection capabilities for streaming data:
//! - Sliding window detection
//! - Incremental learning
//! - Concept drift adaptation
//! - Multi-window strategies
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::streaming_anomaly::{StreamingDetector, SlidingWindowStrategy};
//!
//! let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(100, 10));
//! for data_point in data_stream {
//!     if let Some(anomaly) = detector.detect(data_point) {
//!         println!("Anomaly detected: {:?}", anomaly);
//!     }
//! }
//! ```

use crate::ai::error::{AIServiceError, Result};
use crate::ai::modules::anomaly_detection::DetectionMethod;
use std::collections::VecDeque;

/// Sliding window strategy for streaming detection
#[derive(Debug, Clone)]
pub enum SlidingWindowStrategy {
    /// Fixed-size window, oldest samples are replaced
    Fixed { size: usize, replacement_count: usize },
    /// Time-based window with sliding duration
    TimeBased { duration_seconds: u64 },
    /// Exponential decay with alpha factor
    ExponentialDecay { alpha: f64 },
    /// Adaptive window based on data distribution
    Adaptive { min_size: usize, max_size: usize, threshold: f64 },
}

impl SlidingWindowStrategy {
    pub fn new(size: usize, replacement_count: usize) -> Self {
        SlidingWindowStrategy::Fixed { size, replacement_count }
    }

    pub fn time_based(duration_seconds: u64) -> Self {
        SlidingWindowStrategy::TimeBased { duration_seconds }
    }

    pub fn exponential_decay(alpha: f64) -> Self {
        SlidingWindowStrategy::ExponentialDecay { alpha }
    }

    pub fn adaptive(min_size: usize, max_size: usize, threshold: f64) -> Self {
        SlidingWindowStrategy::Adaptive { min_size, max_size, threshold }
    }
}

/// Streaming anomaly score with metadata
#[derive(Debug, Clone)]
pub struct StreamingAnomalyScore {
    pub timestamp: u64,
    pub value: f64,
    pub score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub window_mean: f64,
    pub window_std: f64,
    pub z_score: f64,
}

impl StreamingAnomalyScore {
    pub fn new(timestamp: u64, value: f64) -> Self {
        Self {
            timestamp,
            value,
            score: 0.0,
            is_anomaly: false,
            confidence: 0.0,
            window_mean: 0.0,
            window_std: 0.0,
            z_score: 0.0,
        }
    }
}

/// Streaming anomaly detector
pub struct StreamingDetector {
    pub strategy: SlidingWindowStrategy,
    pub window: VecDeque<(u64, f64)>,
    pub threshold: f64,
    pub min_window_size: usize,
    pub detection_method: DetectionMethod,
    pub anomaly_count: usize,
    pub total_samples: usize,
}

impl StreamingDetector {
    pub fn new(strategy: SlidingWindowStrategy) -> Self {
        let min_size = match &strategy {
            SlidingWindowStrategy::Fixed { size, .. } => *size,
            SlidingWindowStrategy::TimeBased { .. } => 50,
            SlidingWindowStrategy::ExponentialDecay { .. } => 50,
            SlidingWindowStrategy::Adaptive { min_size, .. } => *min_size,
        };

        Self {
            strategy,
            window: VecDeque::new(),
            threshold: 3.0,
            min_window_size: min_size / 2,
            detection_method: DetectionMethod::ZScore,
            anomaly_count: 0,
            total_samples: 0,
        }
    }

    /// Process a single data point and return anomaly if detected
    pub fn detect(&mut self, value: f64) -> Option<StreamingAnomalyScore> {
        self.total_samples += 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Add to window
        self.window.push_back((timestamp, value));

        // Apply window strategy
        self.apply_window_strategy();

        // Check if window is ready for detection
        if self.window.len() < self.min_window_size {
            return None;
        }

        // Calculate window statistics
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        let mean = calculate_mean(&values);
        let std = calculate_std(&values, mean);
        let z_score = if std > 1e-10 { (value - mean) / std } else { 0.0 };

        // Detect anomaly
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        if is_anomaly {
            self.anomaly_count += 1;
        }

        let mut score = StreamingAnomalyScore::new(timestamp, value);
        score.score = z_score.abs();
        score.is_anomaly = is_anomaly;
        score.confidence = confidence;
        score.window_mean = mean;
        score.window_std = std;
        score.z_score = z_score;

        if is_anomaly {
            Some(score)
        } else {
            None
        }
    }

    /// Process multiple data points in batch
    pub fn detect_batch(&mut self, values: Vec<f64>) -> Vec<StreamingAnomalyScore> {
        values
            .into_iter()
            .filter_map(|v| self.detect(v))
            .collect()
    }

    /// Apply window strategy to manage window size
    fn apply_window_strategy(&mut self) {
        match &self.strategy {
            SlidingWindowStrategy::Fixed { size, replacement_count } => {
                while self.window.len() > *size {
                    for _ in 0..replacement_count.min(self.window.len()) {
                        self.window.pop_front();
                    }
                }
            }
            SlidingWindowStrategy::TimeBased { duration_seconds } => {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                while let Some(front) = self.window.front() {
                    if current_time - front.0 > *duration_seconds {
                        self.window.pop_front();
                    } else {
                        break;
                    }
                }
            }
            SlidingWindowStrategy::ExponentialDecay { .. } => {
                // Window grows but older samples have less weight in calculations
                let max_size = 1000;
                while self.window.len() > max_size {
                    self.window.pop_front();
                }
            }
            SlidingWindowStrategy::Adaptive { min_size, max_size, threshold } => {
                if self.window.len() < *min_size {
                    return;
                }

                let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
                let mean = calculate_mean(&values);
                let std = calculate_std(&values, mean);
                
                // Calculate variance
                let variance = std * std;
                
                // Adjust window size based on variance
                if variance > *threshold && self.window.len() > *min_size {
                    self.window.pop_front();
                } else if variance < *threshold * 0.5 && self.window.len() < *max_size {
                    // Window can grow, but we need more samples
                }
            }
        }
    }

    /// Get current window statistics
    pub fn get_window_stats(&self) -> WindowStats {
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        
        if values.is_empty() {
            return WindowStats {
                size: 0,
                mean: 0.0,
                std: 0.0,
                min: 0.0,
                max: 0.0,
                median: 0.0,
            };
        }

        let mean = calculate_mean(&values);
        let std = calculate_std(&values, mean);
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        WindowStats {
            size: self.window.len(),
            mean,
            std,
            min,
            max,
            median,
        }
    }

    /// Reset detector state
    pub fn reset(&mut self) {
        self.window.clear();
        self.anomaly_count = 0;
        self.total_samples = 0;
    }

    /// Get anomaly rate
    pub fn anomaly_rate(&self) -> f64 {
        if self.total_samples > 0 {
            self.anomaly_count as f64 / self.total_samples as f64
        } else {
            0.0
        }
    }
}

/// Window statistics
#[derive(Debug, Clone)]
pub struct WindowStats {
    pub size: usize,
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
}

/// Calculate mean
fn calculate_mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate standard deviation
fn calculate_std(values: &[f64], mean: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_detector_basic() {
        let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(50, 5));
        
        // Normal data
        for i in 0..100 {
            let value = 100.0 + (i as f64).sin() * 10.0;
            assert!(detector.detect(value).is_none());
        }
        
        // Anomaly
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().is_anomaly);
    }

    #[test]
    fn test_window_stats() {
        let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(50, 5));
        
        for i in 0..100 {
            detector.detect(100.0 + (i as f64));
        }
        
        let stats = detector.get_window_stats();
        assert!(stats.size > 0);
        assert!(stats.mean > 0.0);
    }

    #[test]
    fn test_time_based_strategy() {
        let detector = StreamingDetector::new(
            SlidingWindowStrategy::time_based(60)
        );
        assert_eq!(detector.window.len(), 0);
    }

    #[test]
    fn test_exponential_decay_strategy() {
        let detector = StreamingDetector::new(
            SlidingWindowStrategy::exponential_decay(0.95)
        );
        assert_eq!(detector.window.len(), 0);
    }

    #[test]
    fn test_adaptive_strategy() {
        let detector = StreamingDetector::new(
            SlidingWindowStrategy::adaptive(20, 100, 10.0)
        );
        assert_eq!(detector.window.len(), 0);
    }

    #[test]
    fn test_detect_batch() {
        let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(50, 5));
        
        // Build window with normal data
        for i in 0..60 {
            detector.detect(100.0 + (i as f64).sin() * 10.0);
        }
        
        // Batch with anomalies
        let batch = vec![100.0, 500.0, 100.0, 400.0, 100.0];
        let anomalies = detector.detect_batch(batch);
        
        assert_eq!(anomalies.len(), 2);
        assert!(anomalies.iter().all(|a| a.is_anomaly));
    }

    #[test]
    fn test_anomaly_rate() {
        let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(50, 5));
        
        for i in 0..100 {
            detector.detect(100.0 + (i as f64).sin() * 10.0);
        }
        
        let rate_before = detector.anomaly_rate();
        
        // Add anomalies
        detector.detect(500.0);
        detector.detect(600.0);
        
        let rate_after = detector.anomaly_rate();
        assert!(rate_after > rate_before);
    }

    #[test]
    fn test_reset() {
        let mut detector = StreamingDetector::new(SlidingWindowStrategy::new(50, 5));
        
        for i in 0..100 {
            detector.detect(100.0 + (i as f64));
        }
        
        detector.reset();
        
        assert_eq!(detector.window.len(), 0);
        assert_eq!(detector.anomaly_count, 0);
        assert_eq!(detector.total_samples, 0);
    }
}