//! Time Series Anomaly Detection for VantisOS
//!
//! This module provides time series-specific anomaly detection capabilities:
//! - Seasonal anomaly detection
//! - Trend-based detection
//! - Moving window statistics
//! - Forecasting-based anomaly detection
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::time_series_anomaly::{TimeSeriesDetector, DetectionMethod};
//!
//! let mut detector = TimeSeriesDetector::new(DetectionMethod::Seasonal);
//! for value in time_series_data {
//!     if let Some(anomaly) = detector.detect(value) {
//!         println!("Time series anomaly detected: {:?}", anomaly);
//!     }
//! }
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::VecDeque;

/// Time series anomaly detection method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Seasonal decomposition
    Seasonal,
    /// Trend-based detection
    Trend,
    /// Moving window statistics
    MovingWindow,
    /// Exponential smoothing
    ExponentialSmoothing,
    /// ARIMA-based detection
    ARIMA,
    /// Ensemble of methods
    Ensemble,
}

/// Time series anomaly score
#[derive(Debug, Clone)]
pub struct TimeSeriesAnomalyScore {
    pub timestamp: u64,
    pub value: f64,
    pub score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub predicted: f64,
    pub residual: f64,
    pub method: DetectionMethod,
}

/// Time series detector
pub struct TimeSeriesDetector {
    pub method: DetectionMethod,
    pub window: VecDeque<(u64, f64)>,
    pub threshold: f64,
    pub min_window_size: usize,
    pub max_window_size: usize,
    pub seasonality_period: usize,
    pub anomaly_count: usize,
    pub total_samples: usize,
}

impl TimeSeriesDetector {
    pub fn new(method: DetectionMethod) -> Self {
        let seasonality = match method {
            DetectionMethod::Seasonal => 24, // Daily seasonality by default
            _ => 0,
        };

        Self {
            method,
            window: VecDeque::new(),
            threshold: 3.0,
            min_window_size: 50,
            max_window_size: 1000,
            seasonality_period: seasonality,
            anomaly_count: 0,
            total_samples: 0,
        }
    }

    /// Detect time series anomaly
    pub fn detect(&mut self, value: f64) -> Option<TimeSeriesAnomalyScore> {
        self.total_samples += 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Add to window
        self.window.push_back((timestamp, value));

        // Manage window size
        while self.window.len() > self.max_window_size {
            self.window.pop_front();
        }

        // Check if window is ready
        if self.window.len() < self.min_window_size {
            return None;
        }

        // Detect based on method
        let result = match self.method {
            DetectionMethod::Seasonal => self.seasonal_detect(timestamp, value),
            DetectionMethod::Trend => self.trend_detect(timestamp, value),
            DetectionMethod::MovingWindow => self.moving_window_detect(timestamp, value),
            DetectionMethod::ExponentialSmoothing => self.exponential_smoothing_detect(timestamp, value),
            DetectionMethod::ARIMA => self.arima_detect(timestamp, value),
            DetectionMethod::Ensemble => self.ensemble_detect(timestamp, value),
        };

        if let Some(ref anomaly) = result {
            if anomaly.is_anomaly {
                self.anomaly_count += 1;
            }
        }

        result
    }

    /// Seasonal decomposition-based detection
    fn seasonal_detect(&self, _timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        if self.seasonality_period == 0 || self.window.len() < self.seasonality_period * 2 {
            return None;
        }

        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        
        // Simple seasonal decomposition: value = trend + seasonal + residual
        let trend = calculate_moving_average(&values, self.seasonality_period);
        let seasonal = calculate_seasonal_component(&values, self.seasonality_period);
        
        let idx = values.len() - 1;
        let predicted = trend + seasonal;
        let residual = value - predicted;
        
        // Calculate residual std
        let residuals: Vec<f64> = values.iter()
            .enumerate()
            .map(|(i, v)| {
                let t = calculate_moving_average_at(&values, i, self.seasonality_period);
                let s = calculate_seasonal_at(&values, i, self.seasonality_period);
                v - (t + s)
            })
            .collect();
        
        let residual_mean = calculate_mean(&residuals);
        let residual_std = calculate_std(&residuals, residual_mean);
        
        let z_score = if residual_std > 1e-10 { residual / residual_std } else { 0.0 };
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        Some(TimeSeriesAnomalyScore {
            timestamp: _timestamp,
            value,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            predicted,
            residual,
            method: DetectionMethod::Seasonal,
        })
    }

    /// Trend-based detection
    fn trend_detect(&self, timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        
        // Calculate linear trend
        let n = values.len() as f64;
        let sum_x: f64 = (0..values.len()).map(|i| i as f64).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = values.iter().enumerate()
            .map(|(i, v)| i as f64 * v)
            .sum();
        let sum_x2: f64 = (0..values.len()).map(|i| (i as f64).powi(2)).sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;
        
        // Predict current value
        let idx = values.len() - 1;
        let predicted = slope * idx as f64 + intercept;
        let residual = value - predicted;
        
        // Calculate residual std
        let residuals: Vec<f64> = values.iter()
            .enumerate()
            .map(|(i, v)| v - (slope * i as f64 + intercept))
            .collect();
        
        let residual_mean = calculate_mean(&residuals);
        let residual_std = calculate_std(&residuals, residual_mean);
        
        let z_score = if residual_std > 1e-10 { residual / residual_std } else { 0.0 };
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        Some(TimeSeriesAnomalyScore {
            timestamp,
            value,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            predicted,
            residual,
            method: DetectionMethod::Trend,
        })
    }

    /// Moving window detection
    fn moving_window_detect(&self, timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        
        let window_size = 20;
        if values.len() < window_size {
            return None;
        }
        
        let recent: Vec<f64> = values.iter().rev().take(window_size).cloned().collect();
        let mean = calculate_mean(&recent);
        let std = calculate_std(&recent, mean);
        
        let predicted = mean;
        let residual = value - mean;
        
        let z_score = if std > 1e-10 { residual / std } else { 0.0 };
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        Some(TimeSeriesAnomalyScore {
            timestamp,
            value,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            predicted,
            residual,
            method: DetectionMethod::MovingWindow,
        })
    }

    /// Exponential smoothing detection
    fn exponential_smoothing_detect(&self, timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        let alpha = 0.3;
        
        // Simple exponential smoothing
        let mut smoothed = values[0];
        for i in 1..values.len() {
            smoothed = alpha * values[i] + (1.0 - alpha) * smoothed;
        }
        
        let predicted = smoothed;
        let residual = value - smoothed;
        
        // Calculate residual std
        let residuals: Vec<f64> = values.windows(2)
            .map(|w| w[1] - (alpha * w[1] + (1.0 - alpha) * w[0]))
            .collect();
        
        if residuals.is_empty() {
            return None;
        }
        
        let residual_mean = calculate_mean(&residuals);
        let residual_std = calculate_std(&residuals, residual_mean);
        
        let z_score = if residual_std > 1e-10 { residual / residual_std } else { 0.0 };
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        Some(TimeSeriesAnomalyScore {
            timestamp,
            value,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            predicted,
            residual,
            method: DetectionMethod::ExponentialSmoothing,
        })
    }

    /// Simple ARIMA-like detection (simplified)
    fn arima_detect(&self, timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        let values: Vec<f64> = self.window.iter().map(|(_, v)| *v).collect();
        
        if values.len() < 10 {
            return None;
        }
        
        // Simple AR(1) model
        let n = values.len();
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;
        
        for i in 1..n {
            sum_xy += values[i] * values[i-1];
            sum_x2 += values[i-1].powi(2);
        }
        
        let ar_coef = sum_xy / sum_x2;
        
        // Predict using AR(1)
        let predicted = ar_coef * values[n-1];
        let residual = value - predicted;
        
        // Calculate residual std
        let residuals: Vec<f64> = values.windows(2)
            .map(|w| w[1] - ar_coef * w[0])
            .collect();
        
        let residual_mean = calculate_mean(&residuals);
        let residual_std = calculate_std(&residuals, residual_mean);
        
        let z_score = if residual_std > 1e-10 { residual / residual_std } else { 0.0 };
        let is_anomaly = z_score.abs() > self.threshold;
        let confidence = (z_score.abs() - self.threshold).min(1.0).max(0.0);

        Some(TimeSeriesAnomalyScore {
            timestamp,
            value,
            score: z_score.abs(),
            is_anomaly,
            confidence,
            predicted,
            residual,
            method: DetectionMethod::ARIMA,
        })
    }

    /// Ensemble detection
    fn ensemble_detect(&self, timestamp: u64, value: f64) -> Option<TimeSeriesAnomalyScore> {
        // Combine multiple methods
        let seasonal = self.seasonal_detect(timestamp, value);
        let trend = self.trend_detect(timestamp, value);
        let moving_window = self.moving_window_detect(timestamp, value);
        
        let methods = vec![seasonal, trend, moving_window];
        let valid_methods: Vec<_> = methods.iter().filter_map(|m| m.as_ref()).collect();
        
        if valid_methods.is_empty() {
            return None;
        }
        
        // Average scores
        let avg_score = valid_methods.iter()
            .map(|m| m.score)
            .sum::<f64>() / valid_methods.len() as f64;
        
        let avg_confidence = valid_methods.iter()
            .map(|m| m.confidence)
            .sum::<f64>() / valid_methods.len() as f64;
        
        let is_anomaly = avg_score > self.threshold;
        
        let avg_predicted = valid_methods.iter()
            .map(|m| m.predicted)
            .sum::<f64>() / valid_methods.len() as f64;
        
        let residual = value - avg_predicted;

        Some(TimeSeriesAnomalyScore {
            timestamp,
            value,
            score: avg_score,
            is_anomaly,
            confidence: avg_confidence,
            predicted: avg_predicted,
            residual,
            method: DetectionMethod::Ensemble,
        })
    }

    /// Reset detector
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

/// Calculate moving average
fn calculate_moving_average(values: &[f64], window: usize) -> f64 {
    if values.len() < window {
        return calculate_mean(values);
    }
    
    let recent: Vec<f64> = values.iter().rev().take(window).cloned().collect();
    calculate_mean(&recent)
}

/// Calculate moving average at specific index
fn calculate_moving_average_at(values: &[f64], idx: usize, window: usize) -> f64 {
    let start = idx.saturating_sub(window - 1);
    let end = idx + 1;
    let slice = &values[start..end.min(values.len())];
    calculate_mean(slice)
}

/// Calculate seasonal component
fn calculate_seasonal_component(values: &[f64], period: usize) -> f64 {
    if values.len() < period {
        return 0.0;
    }
    
    let idx = values.len() - 1;
    let seasonal_idx = idx % period;
    
    // Average of values at same seasonal position
    let seasonal_values: Vec<f64> = values.iter()
        .enumerate()
        .filter(|(i, _)| i % period == seasonal_idx)
        .map(|(_, v)| *v)
        .collect();
    
    let seasonal_mean = calculate_mean(&seasonal_values);
    let overall_mean = calculate_mean(values);
    
    seasonal_mean - overall_mean
}

/// Calculate seasonal component at specific index
fn calculate_seasonal_at(values: &[f64], idx: usize, period: usize) -> f64 {
    if values.len() < period {
        return 0.0;
    }
    
    let seasonal_idx = idx % period;
    
    let seasonal_values: Vec<f64> = values.iter()
        .enumerate()
        .filter(|(i, _)| i % period == seasonal_idx)
        .map(|(_, v)| *v)
        .collect();
    
    let seasonal_mean = calculate_mean(&seasonal_values);
    let overall_mean = calculate_mean(values);
    
    seasonal_mean - overall_mean
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
    fn test_seasonal_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::Seasonal);
        
        // Build seasonal pattern
        for i in 0..200 {
            let value = 100.0 + (i as f64).sin() * 20.0;
            detector.detect(value);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().is_anomaly);
    }

    #[test]
    fn test_trend_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::Trend);
        
        // Build trend
        for i in 0..100 {
            detector.detect(100.0 + i as f64);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().is_anomaly);
    }

    #[test]
    fn test_moving_window_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::MovingWindow);
        
        for _ in 0..50 {
            detector.detect(100.0);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().is_anomaly);
    }

    #[test]
    fn test_exponential_smoothing_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::ExponentialSmoothing);
        
        for _ in 0..50 {
            detector.detect(100.0);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
    }

    #[test]
    fn test_arima_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::ARIMA);
        
        for _ in 0..20 {
            detector.detect(100.0);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
    }

    #[test]
    fn test_ensemble_detector() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::Ensemble);
        
        for i in 0..100 {
            detector.detect(100.0 + (i as f64).sin() * 10.0);
        }
        
        let anomaly = detector.detect(500.0);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().is_anomaly);
    }

    #[test]
    fn test_anomaly_rate() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::MovingWindow);
        
        for _ in 0..100 {
            detector.detect(100.0);
        }
        
        let rate_before = detector.anomaly_rate();
        
        detector.detect(500.0);
        detector.detect(600.0);
        
        let rate_after = detector.anomaly_rate();
        assert!(rate_after > rate_before);
    }

    #[test]
    fn test_reset() {
        let mut detector = TimeSeriesDetector::new(DetectionMethod::MovingWindow);
        
        for _ in 0..50 {
            detector.detect(100.0);
        }
        
        detector.reset();
        
        assert_eq!(detector.window.len(), 0);
        assert_eq!(detector.anomaly_count, 0);
        assert_eq!(detector.total_samples, 0);
    }
}