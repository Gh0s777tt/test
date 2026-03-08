//! Contextual Anomaly Detection for VantisOS
//!
//! This module provides contextual anomaly detection capabilities:
//! - Time-based context (hour of day, day of week)
//! - Feature-based context
//! - Ensemble of contextual detectors
//! - Adaptive context windows
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::contextual_anomaly::{ContextualDetector, ContextType};
//!
//! let mut detector = ContextualDetector::new(ContextType::TimeBased);
//! for (value, context) in data_stream {
//!     if let Some(anomaly) = detector.detect(value, context) {
//!         println!("Contextual anomaly detected: {:?}", anomaly);
//!     }
//! }
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Context type for contextual anomaly detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextType {
    /// Time-based context (hour, day of week, month)
    TimeBased,
    /// Feature-based context
    FeatureBased,
    /// Spatial context
    Spatial,
    /// User behavior context
    UserBehavior,
    /// Composite context (multiple types)
    Composite,
}

/// Contextual information
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Context {
    pub context_type: ContextType,
    pub features: Vec<String>,
    pub values: Vec<i64>,
}

impl Context {
    pub fn new(context_type: ContextType) -> Self {
        Self {
            context_type,
            features: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn with_time() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let datetime = chrono::DateTime::from_timestamp(now as i64, 0).unwrap();
        
        Self {
            context_type: ContextType::TimeBased,
            features: vec![
                "hour".to_string(),
                "day_of_week".to_string(),
                "day_of_month".to_string(),
                "month".to_string(),
            ],
            values: vec![
                datetime.hour() as i64,
                datetime.weekday().num_days_from_monday() as i64,
                datetime.day() as i64,
                datetime.month() as i64,
            ],
        }
    }

    pub fn with_features(feature_names: Vec<String>, feature_values: Vec<i64>) -> Self {
        Self {
            context_type: ContextType::FeatureBased,
            features: feature_names,
            values: feature_values,
        }
    }

    pub fn key(&self) -> String {
        self.features.join(":") + "|" + &self.values.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(":")
    }
}

/// Contextual anomaly score
#[derive(Debug, Clone)]
pub struct ContextualAnomalyScore {
    pub value: f64,
    pub context: Context,
    pub score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub baseline_mean: f64,
    pub baseline_std: f64,
    pub context_size: usize,
}

/// Contextual anomaly detector
pub struct ContextualDetector {
    pub context_type: ContextType,
    pub context_windows: HashMap<String, VecDeque<f64>>,
    pub threshold: f64,
    pub min_context_samples: usize,
    pub max_context_size: usize,
    pub anomaly_count: usize,
    pub total_samples: usize,
}

impl ContextualDetector {
    pub fn new(context_type: ContextType) -> Self {
        Self {
            context_type,
            context_windows: HashMap::new(),
            threshold: 3.0,
            min_context_samples: 10,
            max_context_size: 1000,
            anomaly_count: 0,
            total_samples: 0,
        }
    }

    /// Detect contextual anomaly
    pub fn detect(&mut self, value: f64, context: Context) -> Option<ContextualAnomalyScore> {
        self.total_samples += 1;
        let context_key = context.key();

        // Get or create context window
        let window = self.context_windows
            .entry(context_key)
            .or_insert_with(VecDeque::new);

        // Add value to window
        window.push_back(value);

        // Manage window size
        while window.len() > self.max_context_size {
            window.pop_front();
        }

        // Check if we have enough samples for this context
        if window.len() < self.min_context_samples {
            return None;
        }

        // Calculate statistics for this context
        let values: Vec<f64> = window.iter().cloned().collect();
        let mean = calculate_mean(&values);
        let std = calculate_std(&values, mean);
        let z_score = if std > 1e-10 { (value - mean) / std } else { 0.0 };

        // Detect anomaly
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        if is_anomaly {
            self.anomaly_count += 1;
        }

        Some(ContextualAnomalyScore {
            value,
            context,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            baseline_mean: mean,
            baseline_std: std,
            context_size: window.len(),
        })
    }

    /// Detect with automatic time context
    pub fn detect_with_time(&mut self, value: f64) -> Option<ContextualAnomalyScore> {
        let context = Context::with_time();
        self.detect(value, context)
    }

    /// Get statistics for a specific context
    pub fn get_context_stats(&self, context: &Context) -> Option<ContextStats> {
        let context_key = context.key();
        let window = self.context_windows.get(&context_key)?;

        if window.is_empty() {
            return None;
        }

        let values: Vec<f64> = window.iter().cloned().collect();
        let mean = calculate_mean(&values);
        let std = calculate_std(&values, mean);
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        Some(ContextStats {
            context_key,
            size: window.len(),
            mean,
            std,
            min,
            max,
        })
    }

    /// Get all contexts
    pub fn get_all_contexts(&self) -> Vec<ContextStats> {
        self.context_windows
            .iter()
            .filter_map(|(key, window)| {
                if window.is_empty() {
                    return None;
                }

                let values: Vec<f64> = window.iter().cloned().collect();
                let mean = calculate_mean(&values);
                let std = calculate_std(&values, mean);
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                Some(ContextStats {
                    context_key: key.clone(),
                    size: window.len(),
                    mean,
                    std,
                    min,
                    max,
                })
            })
            .collect()
    }

    /// Reset detector
    pub fn reset(&mut self) {
        self.context_windows.clear();
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

    /// Prune old contexts
    pub fn prune_old_contexts(&mut self, min_size: usize) {
        self.context_windows
            .retain(|_, window| window.len() >= min_size);
    }
}

/// Context statistics
#[derive(Debug, Clone)]
pub struct ContextStats {
    pub context_key: String,
    pub size: usize,
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
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
    fn test_contextual_detector() {
        let mut detector = ContextualDetector::new(ContextType::TimeBased);
        
        // Normal data for same context
        for _ in 0..50 {
            let context = Context::new(ContextType::TimeBased);
            detector.detect(100.0, context);
        }
        
        // Anomaly
        let context = Context::new(ContextType::TimeBased);
        let result = detector.detect(500.0, context);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_anomaly);
    }

    #[test]
    fn test_detect_with_time() {
        let mut detector = ContextualDetector::new(ContextType::TimeBased);
        
        for _ in 0..50 {
            detector.detect_with_time(100.0);
        }
        
        let result = detector.detect_with_time(500.0);
        assert!(result.is_some());
        assert!(result.unwrap().is_anomaly);
    }

    #[test]
    fn test_context_stats() {
        let mut detector = ContextualDetector::new(ContextType::TimeBased);
        
        let context = Context::new(ContextType::TimeBased);
        for _ in 0..50 {
            detector.detect(100.0, context.clone());
        }
        
        let stats = detector.get_context_stats(&context);
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().size, 50);
    }

    #[test]
    fn test_get_all_contexts() {
        let mut detector = ContextualDetector::new(ContextType::FeatureBased);
        
        let context1 = Context::with_features(
            vec!["feature1".to_string()],
            vec![1]
        );
        let context2 = Context::with_features(
            vec!["feature1".to_string()],
            vec![2]
        );
        
        for _ in 0..30 {
            detector.detect(100.0, context1.clone());
            detector.detect(200.0, context2.clone());
        }
        
        let all_contexts = detector.get_all_contexts();
        assert_eq!(all_contexts.len(), 2);
    }

    #[test]
    fn test_prune_old_contexts() {
        let mut detector = ContextualDetector::new(ContextType::FeatureBased);
        
        let context1 = Context::with_features(
            vec!["feature1".to_string()],
            vec![1]
        );
        let context2 = Context::with_features(
            vec!["feature1".to_string()],
            vec![2]
        );
        
        for _ in 0..5 {
            detector.detect(100.0, context1.clone());
        }
        for _ in 0..50 {
            detector.detect(200.0, context2.clone());
        }
        
        detector.prune_old_contexts(10);
        let all_contexts = detector.get_all_contexts();
        
        assert_eq!(all_contexts.len(), 1);
    }

    #[test]
    fn test_anomaly_rate() {
        let mut detector = ContextualDetector::new(ContextType::TimeBased);
        
        for _ in 0..100 {
            detector.detect_with_time(100.0);
        }
        
        let rate_before = detector.anomaly_rate();
        assert!(rate_before < 0.1);
        
        // Add anomalies
        for _ in 0..10 {
            detector.detect_with_time(500.0);
        }
        
        let rate_after = detector.anomaly_rate();
        assert!(rate_after > rate_before);
    }

    #[test]
    fn test_reset() {
        let mut detector = ContextualDetector::new(ContextType::TimeBased);
        
        for _ in 0..50 {
            detector.detect_with_time(100.0);
        }
        
        detector.reset();
        
        assert_eq!(detector.context_windows.len(), 0);
        assert_eq!(detector.anomaly_count, 0);
        assert_eq!(detector.total_samples, 0);
    }
}