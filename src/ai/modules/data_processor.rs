//! Data Processor Module for VantisOS AI
//! 
//! Processes raw collected data for ML training and inference.
//! Performs feature extraction, normalization, and transformation.
//! 
//! ## Security Considerations
//! - All processing is local (no cloud dependencies)
//! - No data exfiltration
//! - Memory-bounded processing
//! - Input validation

use crate::ai::{error::AIError, types::ResourceUsage};

/// Data Processor
/// 
/// Processes raw system metrics into features suitable for ML models.
/// 
/// ## Features
//! - Feature extraction
//! - Data normalization
//! - Feature scaling
//! - Time window aggregation
//! - Outlier detection
pub struct DataProcessor {
    window_size: usize,
    enabled: bool,
}

impl DataProcessor {
    /// Create a new data processor
    pub fn new() -> Result<Self, AIError> {
        Ok(Self {
            window_size: 10,
            enabled: true,
        })
    }

    /// Process raw metrics into features
    pub fn process_features(&self, raw_metrics: &[ResourceUsage]) -> Result<Vec<f64>, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }
        
        if raw_metrics.is_empty() {
            return Err(AIError::InsufficientData);
        }
        
        // STUB: Will implement full feature processing in v1.3.0
        let mut features = Vec::new();
        
        // Extract basic statistics
        let cpu_mean: f64 = raw_metrics.iter().map(|m| m.cpu_usage).sum::<f64>() / raw_metrics.len() as f64;
        let mem_mean: f64 = raw_metrics.iter().map(|m| m.memory_usage).sum::<f64>() / raw_metrics.len() as f64;
        
        features.push(cpu_mean);
        features.push(mem_mean);
        
        Ok(features)
    }

    /// Normalize features
    pub fn normalize(&self, features: &[f64]) -> Result<Vec<f64>, AIError> {
        if features.is_empty() {
            return Err(AIError::InsufficientData);
        }
        
        let min = features.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = features.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if max == min {
            return Ok(features.to_vec());
        }
        
        Ok(features.iter().map(|&f| (f - min) / (max - min)).collect())
    }

    /// Get window size
    pub fn window_size(&self) -> usize {
        self.window_size
    }

    /// Check if processor is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = DataProcessor::new().unwrap();
        assert!(processor.is_enabled());
        assert_eq!(processor.window_size(), 10);
    }

    #[test]
    fn test_process_features() {
        let processor = DataProcessor::new().unwrap();
        let metrics = vec![
            ResourceUsage {
                cpu_usage: 20.0,
                memory_usage: 40.0,
                disk_usage: 50.0,
                network_usage: 10.0,
            },
            ResourceUsage {
                cpu_usage: 30.0,
                memory_usage: 50.0,
                disk_usage: 60.0,
                network_usage: 15.0,
            },
        ];
        
        let features = processor.process_features(&metrics).unwrap();
        assert_eq!(features.len(), 2); // cpu_mean, mem_mean
        assert_eq!(features[0], 25.0); // (20 + 30) / 2
    }

    #[test]
    fn test_normalize() {
        let processor = DataProcessor::new().unwrap();
        let features = vec![0.0, 0.5, 1.0];
        let normalized = processor.normalize(&features).unwrap();
        
        assert_eq!(normalized[0], 0.0);
        assert_eq!(normalized[1], 0.5);
        assert_eq!(normalized[2], 1.0);
    }
}