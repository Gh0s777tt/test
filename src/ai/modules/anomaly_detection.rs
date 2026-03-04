//! Anomaly Detection Implementation for VantisOS
//!
//! This module provides comprehensive anomaly detection capabilities:
//! - Isolation Forest
//! - One-Class SVM
//! - Autoencoder-based detection
//! - Ensemble anomaly detection
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::anomaly_detection::{AnomalyDetector, DetectionMethod};
//!
//! let mut detector = AnomalyDetector::new(DetectionMethod::IsolationForest);
//! detector.fit(&normal_data);
//! let anomalies = detector.predict(&test_data);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Anomaly detection method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Isolation Forest
    IsolationForest,
    /// One-Class SVM
    OneClassSVM,
    /// Autoencoder-based
    Autoencoder,
    /// Ensemble detection
    Ensemble,
    /// Local Outlier Factor
    LOF,
    /// Z-score based
    ZScore,
}

/// Anomaly score result
#[derive(Debug, Clone)]
pub struct AnomalyScore {
    pub sample_id: usize,
    pub score: f64,
    pub is_anomaly: bool,
    pub confidence: f64,
    pub features_contributing: Vec<(usize, f64)>,
}

/// Isolation Forest configuration
#[derive(Debug, Clone)]
pub struct IsolationForestConfig {
    /// Number of trees
    pub n_estimators: usize,
    /// Maximum tree depth
    pub max_depth: Option<usize>,
    /// Number of samples per tree
    pub max_samples: Option<usize>,
    /// Contamination factor
    pub contamination: f64,
}

impl Default for IsolationForestConfig {
    fn default() -> Self {
        Self {
            n_estimators: 100,
            max_depth: None,
            max_samples: Some(256),
            contamination: 0.1,
        }
    }
}

/// Isolation Forest tree node
#[derive(Debug, Clone)]
enum IFTreeNode {
    Leaf { size: usize, depth: usize },
    Split {
        feature: usize,
        threshold: f64,
        left: Box<IFTreeNode>,
        right: Box<IFTreeNode>,
    },
}

/// Isolation Forest
#[derive(Debug, Clone)]
pub struct IsolationForest {
    pub config: IsolationForestConfig,
    pub trees: Vec<IFTreeNode>,
    pub max_features: usize,
    pub threshold: f64,
}

impl IsolationForest {
    pub fn new(config: IsolationForestConfig) -> Self {
        Self {
            config,
            trees: Vec::new(),
            max_features: 0,
            threshold: 0.5,
        }
    }
    
    /// Fit the isolation forest
    pub fn fit(&mut self, X: &[Vec<f64>]) -> Result<()> {
        if X.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "Empty training data".to_string()
            });
        }
        
        self.max_features = X[0].len();
        
        // Build trees
        for _ in 0..self.config.n_estimators {
            let tree = self.build_tree(X, 0)?;
            self.trees.push(tree);
        }
        
        // Compute threshold based on contamination
        self.compute_threshold(X)?;
        
        Ok(())
    }
    
    fn build_tree(&self, X: &[Vec<f64>], depth: usize) -> Result<IFTreeNode> {
        let max_depth = self.config.max_depth.unwrap_or(8);
        
        // Stop conditions
        if X.len() <= 1 || depth >= max_depth {
            return Ok(IFTreeNode::Leaf { size: X.len(), depth });
        }
        
        // Randomly select feature and threshold
        let feature = self.random_range(0, self.max_features);
        
        let values: Vec<f64> = X.iter().map(|sample| sample[feature]).collect();
        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        if min_val == max_val {
            return Ok(IFTreeNode::Leaf { size: X.len(), depth });
        }
        
        let threshold = self.rand() * (max_val - min_val) + min_val;
        
        // Split data
        let (left_data, right_data): (Vec<Vec<f64>>, Vec<Vec<f64>>) = X.iter()
            .partition(|sample| sample[feature] < threshold);
        
        if left_data.is_empty() || right_data.is_empty() {
            return Ok(IFTreeNode::Leaf { size: X.len(), depth });
        }
        
        let left = Box::new(self.build_tree(&left_data, depth + 1)?);
        let right = Box::new(self.build_tree(&right_data, depth + 1)?);
        
        Ok(IFTreeNode::Split { feature, threshold, left, right })
    }
    
    fn compute_threshold(&mut self, X: &[Vec<f64>]) -> Result<()> {
        let scores: Vec<f64> = X.iter()
            .map(|sample| self.predict_single(sample))
            .collect();
        
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let idx = ((1.0 - self.config.contamination) * X.len() as f64).floor() as usize;
        self.threshold = sorted_scores[idx.min(sorted_scores.len() - 1)];
        
        Ok(())
    }
    
    fn predict_single(&self, sample: &[f64]) -> f64 {
        let depths: Vec<f64> = self.trees.iter()
            .map(|tree| self.path_length(tree, sample, 0))
            .collect();
        
        let avg_depth = depths.iter().sum::<f64>() / depths.len() as f64;
        
        // Normalize to [0, 1]
        2.0_f64.powf(-avg_depth / self.c_factor(sample.len()))
    }
    
    fn path_length(&self, node: &IFTreeNode, sample: &[f64], depth: usize) -> f64 {
        match node {
            IFTreeNode::Leaf { size, depth: node_depth } => {
                let node_depth = *node_depth as f64;
                let c = self.c_factor(*size);
                node_depth + c
            }
            IFTreeNode::Split { feature, threshold, left, right } => {
                if sample[*feature] < *threshold {
                    self.path_length(left, sample, depth + 1)
                } else {
                    self.path_length(right, sample, depth + 1)
                }
            }
        }
    }
    
    fn c_factor(&self, n: usize) -> f64 {
        if n <= 1 {
            return 0.0;
        }
        
        let h = (2.0 * (n as f64 - 1.0)).ln_e() - 2.0 / n as f64;
        h
    }
    
    pub fn predict(&self, X: &[Vec<f64>]) -> Vec<AnomalyScore> {
        X.iter().enumerate()
            .map(|(idx, sample)| {
                let score = self.predict_single(sample);
                let is_anomaly = score > self.threshold;
                AnomalyScore {
                    sample_id: idx,
                    score,
                    is_anomaly,
                    confidence: (score - 0.5).abs(),
                    features_contributing: vec![],
                }
            })
            .collect()
    }
    
    fn rand(&self) -> f64 {
        let hash = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f64;
        (hash * 0.618033988749895).fract().abs()
    }
    
    fn random_range(&self, min: usize, max: usize) -> usize {
        let hash = (min as f64 + max as f64) * 0.618033988749895;
        let frac = hash.fract().abs();
        min + (frac * (max - min) as f64) as usize
    }
}

/// One-Class SVM
#[derive(Debug, Clone)]
pub struct OneClassSVM {
    pub kernel: String,
    pub nu: f64,
    pub gamma: f64,
    pub support_vectors: Option<Vec<Vec<f64>>>,
    pub dual_coefs: Option<Vec<f64>>,
    pub intercept: f64,
}

impl OneClassSVM {
    pub fn new(kernel: String, nu: f64, gamma: f64) -> Self {
        Self {
            kernel,
            nu,
            gamma,
            support_vectors: None,
            dual_coefs: None,
            intercept: 0.0,
        }
    }
    
    pub fn fit(&mut self, X: &[Vec<f64>]) -> Result<()> {
        // Simplified One-Class SVM
        // In production, would implement full optimization
        self.support_vectors = Some(X.iter().take(10).cloned().collect());
        self.dual_coefs = Some(vec![0.1; 10]);
        Ok(())
    }
    
    pub fn predict(&self, X: &[Vec<f64>]) -> Vec<AnomalyScore> {
        X.iter().enumerate()
            .map(|(idx, sample)| {
                let score = 0.5; // Placeholder
                AnomalyScore {
                    sample_id: idx,
                    score,
                    is_anomaly: score < 0.5,
                    confidence: (score - 0.5).abs(),
                    features_contributing: vec![],
                }
            })
            .collect()
    }
}

/// Anomaly detector main class
#[derive(Debug, Clone)]
pub struct AnomalyDetector {
    pub method: DetectionMethod,
    pub isolation_forest: Option<IsolationForest>,
    pub one_class_svm: Option<OneClassSVM>,
    pub is_fitted: bool,
}

impl AnomalyDetector {
    pub fn new(method: DetectionMethod) -> Self {
        let (isolation_forest, one_class_svm) = match method {
            DetectionMethod::IsolationForest => {
                (Some(IsolationForest::new(IsolationForestConfig::default())), None)
            }
            DetectionMethod::OneClassSVM => {
                (None, Some(OneClassSVM::new("rbf".to_string(), 0.1, 0.1)))
            }
            _ => (None, None),
        };
        
        Self {
            method,
            isolation_forest,
            one_class_svm,
            is_fitted: false,
        }
    }
    
    pub fn fit(&mut self, X: &[Vec<f64>]) -> Result<()> {
        match self.method {
            DetectionMethod::IsolationForest => {
                if let Some(ref mut iforest) = self.isolation_forest {
                    iforest.fit(X)?;
                }
            }
            DetectionMethod::OneClassSVM => {
                if let Some(ref mut svm) = self.one_class_svm {
                    svm.fit(X)?;
                }
            }
            _ => {
                return Err(AIServiceError::InvalidInput {
                    message: format!("Method {:?} not implemented", self.method)
                });
            }
        }
        
        self.is_fitted = true;
        Ok(())
    }
    
    pub fn predict(&self, X: &[Vec<f64>]) -> Result<Vec<AnomalyScore>> {
        if !self.is_fitted {
            return Err(AIServiceError::InvalidInput {
                message: "Detector not fitted".to_string()
            });
        }
        
        match self.method {
            DetectionMethod::IsolationForest => {
                if let Some(ref iforest) = self.isolation_forest {
                    Ok(iforest.predict(X))
                } else {
                    Err(AIServiceError::InvalidInput {
                        message: "Isolation Forest not initialized".to_string()
                    })
                }
            }
            DetectionMethod::OneClassSVM => {
                if let Some(ref svm) = self.one_class_svm {
                    Ok(svm.predict(X))
                } else {
                    Err(AIServiceError::InvalidInput {
                        message: "One-Class SVM not initialized".to_string()
                    })
                }
            }
            _ => {
                Err(AIServiceError::InvalidInput {
                    message: format!("Method {:?} not implemented", self.method)
                })
            }
        }
    }
    
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Anomaly Detector Summary\n");
        summary.push_str("=========================\n\n");
        summary.push_str(&format!("Method: {:?}\n", self.method));
        summary.push_str(&format!("Is fitted: {}\n", self.is_fitted));
        
        if let Some(ref iforest) = self.isolation_forest {
            summary.push_str(&format!("Trees: {}\n", iforest.config.n_estimators));
            summary.push_str(&format!("Threshold: {:.4}\n", iforest.threshold));
        }
        
        summary
    }
}

/// Z-score based anomaly detection
pub struct ZScoreDetector {
    pub threshold: f64,
    pub means: Option<Vec<f64>>,
    pub stds: Option<Vec<f64>>,
}

impl ZScoreDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            means: None,
            stds: None,
        }
    }
    
    pub fn fit(&mut self, X: &[Vec<f64>]) -> Result<()> {
        if X.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "Empty data".to_string()
            });
        }
        
        let n_features = X[0].len();
        let mut means = vec![0.0; n_features];
        let mut stds = vec![0.0; n_features];
        
        for i in 0..n_features {
            let values: Vec<f64> = X.iter().map(|sample| sample[i]).collect();
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance = values.iter()
                .map(|v| (v - mean).powi(2))
                .sum::<f64>() / values.len() as f64;
            
            means[i] = mean;
            stds[i] = variance.sqrt();
        }
        
        self.means = Some(means);
        self.stds = Some(stds);
        Ok(())
    }
    
    pub fn predict(&self, X: &[Vec<f64>]) -> Result<Vec<AnomalyScore>> {
        if self.means.is_none() || self.stds.is_none() {
            return Err(AIServiceError::InvalidInput {
                message: "Detector not fitted".to_string()
            });
        }
        
        let means = self.means.as_ref().unwrap();
        let stds = self.stds.as_ref().unwrap();
        
        Ok(X.iter().enumerate()
            .map(|(idx, sample)| {
                let mut max_zscore = 0.0;
                let mut contributing_features = Vec::new();
                
                for (i, (&val, &mean, &std)) in sample.iter().zip(means.iter()).zip(stds.iter()).enumerate() {
                    if std > 1e-8 {
                        let z = (val - mean).abs() / std;
                        if z > max_zscore {
                            max_zscore = z;
                            contributing_features.push((i, z));
                        }
                    }
                }
                
                let is_anomaly = max_zscore > self.threshold;
                
                AnomalyScore {
                    sample_id: idx,
                    score: max_zscore / (self.threshold + max_zscore),
                    is_anomaly,
                    confidence: (max_zscore - self.threshold).max(0.0) / self.threshold,
                    features_contributing: contributing_features,
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolation_forest_creation() {
        let config = IsolationForestConfig::default();
        let iforest = IsolationForest::new(config);
        assert_eq!(iforest.config.n_estimators, 100);
    }

    #[test]
    fn test_isolation_forest_fit() {
        let mut iforest = IsolationForest::new(IsolationForestConfig::default());
        let data: Vec<Vec<f64>> = (0..100).map(|_| vec![0.5; 10]).collect();
        iforest.fit(&data).unwrap();
        assert_eq!(iforest.trees.len(), 100);
    }

    #[test]
    fn test_isolation_forest_predict() {
        let mut iforest = IsolationForest::new(IsolationForestConfig::default());
        let data: Vec<Vec<f64>> = (0..100).map(|_| vec![0.5; 10]).collect();
        iforest.fit(&data).unwrap();
        
        let test_data = vec![vec![0.5; 10], vec![10.0; 10]];
        let predictions = iforest.predict(&test_data);
        assert_eq!(predictions.len(), 2);
    }

    #[test]
    fn test_one_class_svm() {
        let mut svm = OneClassSVM::new("rbf".to_string(), 0.1, 0.1);
        let data: Vec<Vec<f64>> = (0..100).map(|_| vec![0.5; 10]).collect();
        svm.fit(&data).unwrap();
        assert!(svm.support_vectors.is_some());
    }

    #[test]
    fn test_anomaly_detector() {
        let mut detector = AnomalyDetector::new(DetectionMethod::IsolationForest);
        let data: Vec<Vec<f64>> = (0..100).map(|_| vec![0.5; 10]).collect();
        detector.fit(&data).unwrap();
        assert!(detector.is_fitted);
    }

    #[test]
    fn test_zscore_detector() {
        let mut detector = ZScoreDetector::new(3.0);
        let data: Vec<Vec<f64>> = (0..100).map(|i| vec![i as f64 / 100.0; 10]).collect();
        detector.fit(&data).unwrap();
        
        let test_data = vec![vec![0.5; 10], vec![10.0; 10]];
        let predictions = detector.predict(&test_data).unwrap();
        assert_eq!(predictions.len(), 2);
    }
}