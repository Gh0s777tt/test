//! Concept Drift Detection for VantisOS
//!
//! This module provides concept drift detection capabilities:
//! - Statistical drift detection (ADWIN, KS test, CUSUM)
//! - Distribution-based drift detection
//! - Model performance monitoring
//! - Adaptive threshold adjustment
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::concept_drift::{ConceptDriftDetector, DriftMethod};
//!
//! let mut detector = ConceptDriftDetector::new(DriftMethod::ADWIN, 0.05);
//! for data_point in data_stream {
//!     if let Some(drift) = detector.detect(data_point) {
//!         println!("Concept drift detected: {:?}", drift);
//!     }
//! }
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::VecDeque;

/// Concept drift detection method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftMethod {
    /// ADWIN (Adaptive Windowing)
    ADWIN,
    /// Kolmogorov-Smirnov test
    KS,
    /// CUSUM (Cumulative Sum)
    CUSUM,
    /// Page-Hinkley test
    PageHinkley,
    /// DDM (Drift Detection Method)
    DDM,
    /// EDDM (Early Drift Detection Method)
    EDDM,
}

/// Concept drift result
#[derive(Debug, Clone)]
pub struct DriftResult {
    pub timestamp: u64,
    pub is_drift: bool,
    pub drift_magnitude: f64,
    pub confidence: f64,
    pub drift_type: DriftType,
    pub p_value: Option<f64>,
    pub statistic: f64,
}

/// Type of concept drift
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftType {
    /// Sudden/abrupt drift
    Sudden,
    /// Gradual drift
    Gradual,
    /// Incremental drift
    Incremental,
    /// Recurring concepts
    Recurring,
    /// No drift
    None,
}

/// Concept drift detector
pub struct ConceptDriftDetector {
    pub method: DriftMethod,
    pub alpha: f64,
    pub window: VecDeque<f64>,
    pub max_window_size: usize,
    pub min_window_size: usize,
    pub drift_count: usize,
    pub total_samples: usize,
    pub drift_history: Vec<DriftResult>,
}

impl ConceptDriftDetector {
    pub fn new(method: DriftMethod, alpha: f64) -> Self {
        Self {
            method,
            alpha,
            window: VecDeque::new(),
            max_window_size: 1000,
            min_window_size: 50,
            drift_count: 0,
            total_samples: 0,
            drift_history: Vec::new(),
        }
    }

    /// Process a single data point and detect drift
    pub fn detect(&mut self, value: f64) -> Option<DriftResult> {
        self.total_samples += 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.window.push_back(value);

        // Manage window size
        while self.window.len() > self.max_window_size {
            self.window.pop_front();
        }

        // Check if window is ready
        if self.window.len() < self.min_window_size {
            return None;
        }

        // Detect drift based on method
        let result = match self.method {
            DriftMethod::ADWIN => self.adwin_detect(timestamp),
            DriftMethod::KS => self.ks_detect(timestamp),
            DriftMethod::CUSUM => self.cusum_detect(timestamp),
            DriftMethod::PageHinkley => self.page_hinkley_detect(timestamp),
            DriftMethod::DDM => self.ddm_detect(timestamp),
            DriftMethod::EDDM => self.eddm_detect(timestamp),
        };

        if let Some(ref drift) = result {
            if drift.is_drift {
                self.drift_count += 1;
                self.drift_history.push(drift.clone());
            }
        }

        result
    }

    /// ADWIN (Adaptive Windowing) drift detection
    fn adwin_detect(&self, timestamp: u64) -> Option<DriftResult> {
        let window: Vec<f64> = self.window.iter().cloned().collect();
        let n = window.len();

        if n < self.min_window_size {
            return None;
        }

        let global_mean = calculate_mean(&window);
        let global_var = calculate_variance(&window, global_mean);

        // Try all possible splits
        for split_pos in (self.min_window_size..=(n - self.min_window_size)).step_by(10) {
            let left_mean = calculate_mean(&window[..split_pos]);
            let right_mean = calculate_mean(&window[split_pos..]);
            
            let m = split_pos;
            let n_minus_m = n - split_pos;
            
            let delta = (left_mean - right_mean).abs();
            let n_cut = (2.0 / (m as f64) + 1.0 / (n_minus_m as f64)) * 
                        (global_var * (1.0 / (m as f64) + 1.0 / (n_minus_m as f64)).ln() * 0.5).sqrt();

            if delta > n_cut {
                return Some(DriftResult {
                    timestamp,
                    is_drift: true,
                    drift_magnitude: delta,
                    confidence: (delta - n_cut) / delta,
                    drift_type: DriftType::Sudden,
                    p_value: Some(self.alpha),
                    statistic: delta,
                });
            }
        }

        None
    }

    /// Kolmogorov-Smirnov test for drift detection
    fn ks_detect(&self, timestamp: u64) -> Option<DriftResult> {
        let window: Vec<f64> = self.window.iter().cloned().collect();
        let n = window.len();

        if n < self.min_window_size * 2 {
            return None;
        }

        let split_pos = n / 2;
        let left = &window[..split_pos];
        let right = &window[split_pos..];

        // Sort both halves
        let mut left_sorted = left.to_vec();
        let mut right_sorted = right.to_vec();
        left_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        right_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate KS statistic
        let mut max_diff = 0.0;
        let mut i = 0;
        let mut j = 0;
        let n1 = left_sorted.len() as f64;
        let n2 = right_sorted.len() as f64;

        while i < left_sorted.len() && j < right_sorted.len() {
            let f1 = (i + 1) as f64 / n1;
            let f2 = (j + 1) as f64 / n2;
            
            let diff = (f1 - f2).abs();
            if diff > max_diff {
                max_diff = diff;
            }

            if left_sorted[i] < right_sorted[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        // Critical value approximation
        let critical_value = 1.36 * ((n1 + n2) / (n1 * n2)).sqrt();

        let is_drift = max_diff > critical_value;

        Some(DriftResult {
            timestamp,
            is_drift,
            drift_magnitude: max_diff,
            confidence: (max_diff - critical_value).max(0.0) / max_diff,
            drift_type: if is_drift { DriftType::Gradual } else { DriftType::None },
            p_value: None,
            statistic: max_diff,
        })
    }

    /// CUSUM (Cumulative Sum) drift detection
    fn cusum_detect(&self, _timestamp: u64) -> Option<DriftResult> {
        let window: Vec<f64> = self.window.iter().cloned().collect();

        if window.len() < self.min_window_size {
            return None;
        }

        let mean = calculate_mean(&window);
        let std = calculate_std(&window, mean);

        if std < 1e-10 {
            return None;
        }

        // CUSUM statistic
        let threshold = self.alpha * 5.0;
        let mut cusum_pos = 0.0;
        let mut cusum_neg = 0.0;

        for &value in &window {
            let deviation = (value - mean) / std;
            cusum_pos = (0.0, cusum_pos + deviation - 0.5).max(0.0);
            cusum_neg = (0.0, cusum_neg - deviation - 0.5).max(0.0);

            if cusum_pos > threshold || cusum_neg > threshold {
                return Some(DriftResult {
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    is_drift: true,
                    drift_magnitude: cusum_pos.max(cusum_neg),
                    confidence: (cusum_pos.max(cusum_neg) - threshold) / threshold,
                    drift_type: DriftType::Incremental,
                    p_value: None,
                    statistic: cusum_pos.max(cusum_neg),
                });
            }
        }

        None
    }

    /// Page-Hinkley test for drift detection
    fn page_hinkley_detect(&self, _timestamp: u64) -> Option<DriftResult> {
        let window: Vec<f64> = self.window.iter().cloned().collect();

        if window.len() < self.min_window_size {
            return None;
        }

        let mean = calculate_mean(&window);
        let std = calculate_std(&window, mean);

        if std < 1e-10 {
            return None;
        }

        let alpha = self.alpha * 10.0;
        let threshold = 4.0 * std;
        let mut sum = 0.0;
        let mut min_sum = f64::INFINITY;

        for &value in &window {
            sum += value - mean;
            min_sum = min_sum.min(sum);

            if sum - min_sum > threshold {
                return Some(DriftResult {
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    is_drift: true,
                    drift_magnitude: sum - min_sum,
                    confidence: (sum - min_sum - threshold) / threshold,
                    drift_type: DriftType::Gradual,
                    p_value: None,
                    statistic: sum - min_sum,
                });
            }
        }

        None
    }

    /// DDM (Drift Detection Method)
    fn ddm_detect(&self, _timestamp: u64) -> Option<DriftResult> {
        // DDM is typically used with classification errors
        // Here we adapt it for numerical values using deviation from mean
        let window: Vec<f64> = self.window.iter().cloned().collect();

        if window.len() < self.min_window_size {
            return None;
        }

        let mean = calculate_mean(&window);
        let std = calculate_std(&window, mean);

        if std < 1e-10 {
            return None;
        }

        // Calculate error rate as deviation
        let error_rate = window.iter()
            .map(|v| (v - mean).abs())
            .sum::<f64>() / (std * window.len() as f64);

        let threshold = 2.0;
        if error_rate > threshold {
            return Some(DriftResult {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_drift: true,
                drift_magnitude: error_rate,
                confidence: (error_rate - threshold) / threshold,
                drift_type: DriftType::Sudden,
                p_value: None,
                statistic: error_rate,
            });
        }

        None
    }

    /// EDDM (Early Drift Detection Method)
    fn eddm_detect(&self, _timestamp: u64) -> Option<DriftResult> {
        let window: Vec<f64> = self.window.iter().cloned().collect();

        if window.len() < self.min_window_size {
            return None;
        }

        let mean = calculate_mean(&window);
        let std = calculate_std(&window, mean);

        if std < 1e-10 {
            return None;
        }

        // EDDM monitors distance between errors
        let deviations: Vec<f64> = window.iter()
            .map(|v| (v - mean).abs())
            .collect();

        let max_deviation = deviations.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_deviation = deviations.iter().cloned().fold(f64::INFINITY, f64::min);

        let ratio = max_deviation / (min_deviation + 1e-10);
        let threshold = 2.0;

        if ratio > threshold {
            return Some(DriftResult {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                is_drift: true,
                drift_magnitude: ratio,
                confidence: (ratio - threshold) / threshold,
                drift_type: DriftType::Gradual,
                p_value: None,
                statistic: ratio,
            });
        }

        None
    }

    /// Reset detector state
    pub fn reset(&mut self) {
        self.window.clear();
        self.drift_count = 0;
        self.total_samples = 0;
        self.drift_history.clear();
    }

    /// Get drift rate
    pub fn drift_rate(&self) -> f64 {
        if self.total_samples > 0 {
            self.drift_count as f64 / self.total_samples as f64
        } else {
            0.0
        }
    }

    /// Get recent drift history
    pub fn get_recent_drifts(&self, n: usize) -> Vec<DriftResult> {
        self.drift_history.iter()
            .rev()
            .take(n)
            .cloned()
            .collect()
    }
}

/// Calculate mean
fn calculate_mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate variance
fn calculate_variance(values: &[f64], mean: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64
}

/// Calculate standard deviation
fn calculate_std(values: &[f64], mean: f64) -> f64 {
    calculate_variance(values, mean).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adwin_detector() {
        let mut detector = ConceptDriftDetector::new(DriftMethod::ADWIN, 0.05);
        
        // Normal data
        for i in 0..100 {
            detector.detect(100.0 + (i as f64).sin() * 10.0);
        }
        
        // Drift data
        for i in 0..50 {
            detector.detect(200.0 + (i as f64).sin() * 10.0);
        }
        
        let drift = detector.detect(250.0);
        assert!(drift.is_some());
        assert!(drift.unwrap().is_drift);
    }

    #[test]
    fn test_ks_detector() {
        let mut detector = ConceptDriftDetector::new(DriftMethod::KS, 0.05);
        
        // Build window
        for i in 0..200 {
            detector.detect(i as f64);
        }
        
        let drift = detector.detect(500.0);
        assert!(drift.is_some());
    }

    #[test]
    fn test_cusum_detector() {
        let mut detector = ConceptDriftDetector::new(DriftMethod::CUSUM, 0.05);
        
        for i in 0..100 {
            detector.detect(100.0);
        }
        
        for i in 0..20 {
            detector.detect(150.0 + i as f64);
        }
        
        let drift = detector.detect(200.0);
        assert!(drift.is_some());
    }

    #[test]
    fn test_drift_rate() {
        let mut detector = ConceptDriftDetector::new(DriftMethod::ADWIN, 0.05);
        
        for i in 0..100 {
            detector.detect(100.0 + (i as f64).sin() * 10.0);
        }
        
        let rate_before = detector.drift_rate();
        
        // Induce drift
        for i in 0..50 {
            detector.detect(200.0 + (i as f64).sin() * 10.0);
        }
        
        let rate_after = detector.drift_rate();
        assert!(rate_after >= rate_before);
    }

    #[test]
    fn test_reset() {
        let mut detector = ConceptDriftDetector::new(DriftMethod::ADWIN, 0.05);
        
        for i in 0..100 {
            detector.detect(i as f64);
        }
        
        detector.reset();
        
        assert_eq!(detector.window.len(), 0);
        assert_eq!(detector.drift_count, 0);
        assert_eq!(detector.drift_history.len(), 0);
    }

    #[test]
    fn test_all_methods() {
        let methods = vec![
            DriftMethod::ADWIN,
            DriftMethod::KS,
            DriftMethod::CUSUM,
            DriftMethod::PageHinkley,
            DriftMethod::DDM,
            DriftMethod::EDDM,
        ];

        for method in methods {
            let mut detector = ConceptDriftDetector::new(method, 0.05);
            
            for i in 0..100 {
                detector.detect(100.0 + (i as f64).sin() * 10.0);
            }
            
            // Should not panic
            let _result = detector.detect(500.0);
        }
    }
}