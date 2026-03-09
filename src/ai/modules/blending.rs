//! Blending Ensemble Implementation for VantisOS
//!
//! This module provides blending ensemble methods:
//! - Simple averaging blending
//! - Weighted blending
//! - Dynamic blending with meta-learner
//! - Geometric blending
//! - Time-weighted blending
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::blending::{BlendingEnsemble, BlendingConfig};
//!
//! let config = BlendingConfig::default();
//! let mut blending = BlendingEnsemble::new(config);
//! blending.add_model(model1, 0.3);
//! blending.add_model(model2, 0.7);
//! let predictions = blending.predict(&features);
//! ```

use crate::ai::error::{AIServiceError, Result};
use crate::ai::modules::ensembles::EnsembleModel;

/// Blending configuration
#[derive(Debug, Clone)]
pub struct BlendingConfig {
    /// Number of models to blend
    pub n_models: usize,
    /// Blending strategy
    pub strategy: BlendingStrategy,
    /// Use adaptive weighting
    pub adaptive_weights: bool,
    /// Weight decay factor for adaptive weights
    pub weight_decay: f64,
    /// Minimum weight
    pub min_weight: f64,
    /// Use meta-learner
    pub use_meta_learner: bool,
}

impl Default for BlendingConfig {
    fn default() -> Self {
        Self {
            n_models: 3,
            strategy: BlendingStrategy::WeightedAverage,
            adaptive_weights: false,
            weight_decay: 0.95,
            min_weight: 0.1,
            use_meta_learner: false,
        }
    }
}

/// Blending strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendingStrategy {
    /// Simple average of predictions
    SimpleAverage,
    /// Weighted average of predictions
    WeightedAverage,
    /// Maximum of predictions
    Max,
    /// Minimum of predictions
    Min,
    /// Geometric mean
    GeometricMean,
    /// Harmonic mean
    HarmonicMean,
    /// Median
    Median,
    /// Meta-learner blending
    MetaLearner,
}

/// Model with weight for blending
#[derive(Debug, Clone)]
pub struct WeightedModel {
    pub model: EnsembleModel,
    pub weight: f64,
    pub performance: f64,
    pub last_update: Option<u64>,
}

impl WeightedModel {
    pub fn new(model: EnsembleModel, weight: f64) -> Self {
        Self {
            model,
            weight,
            performance: 0.0,
            last_update: None,
        }
    }
    
    pub fn with_performance(mut self, performance: f64) -> Self {
        self.performance = performance;
        self
    }
}

/// Blending ensemble
#[derive(Debug, Clone)]
pub struct BlendingEnsemble {
    pub config: BlendingConfig,
    pub models: Vec<WeightedModel>,
    pub meta_weights: Option<Vec<f64>>,
    pub is_fitted: bool,
}

impl BlendingEnsemble {
    pub fn new(config: BlendingConfig) -> Self {
        Self {
            config,
            models: Vec::new(),
            meta_weights: None,
            is_fitted: false,
        }
    }
    
    /// Add a model with its weight
    pub fn add_model(&mut self, model: EnsembleModel, weight: f64) {
        let weighted_model = WeightedModel::new(model, weight);
        self.models.push(weighted_model);
    }
    
    /// Add a weighted model
    pub fn add_weighted_model(&mut self, model: WeightedModel) {
        self.models.push(model);
    }
    
    /// Normalize weights
    pub fn normalize_weights(&mut self) {
        let total: f64 = self.models.iter().map(|m| m.weight).sum();
        if total > 0.0 {
            for model in &mut self.models {
                model.weight /= total;
            }
        }
    }
    
    /// Update weights based on performance
    pub fn update_weights(&mut self, performances: Vec<f64>) {
        if performances.len() != self.models.len() {
            return;
        }
        
        if self.config.adaptive_weights {
            // Update weights based on performance
            let total_perf: f64 = performances.iter().sum();
            
            for (model, &perf) in self.models.iter_mut().zip(performances.iter()) {
                // Decay old weight and add new performance-based weight
                model.weight = model.weight * self.config.weight_decay + 
                               (perf / total_perf) * (1.0 - self.config.weight_decay);
                
                // Ensure minimum weight
                model.weight = model.weight.max(self.config.min_weight);
            }
            
            self.normalize_weights();
        } else {
            // Just update performance tracking
            for (model, &perf) in self.models.iter_mut().zip(performances.iter()) {
                model.performance = perf;
            }
        }
    }
    
    /// Get current weights
    pub fn get_weights(&self) -> Vec<f64> {
        self.models.iter().map(|m| m.weight).collect()
    }
    
    /// Set weights manually
    pub fn set_weights(&mut self, weights: Vec<f64>) -> Result<()> {
        if weights.len() != self.models.len() {
            return Err(AIServiceError::InvalidInput {
                message: format!("Expected {} weights, got {}", self.models.len(), weights.len())
            });
        }
        
        for (model, &weight) in self.models.iter_mut().zip(weights.iter()) {
            model.weight = weight;
        }
        
        Ok(())
    }
    
    /// Blend predictions using configured strategy
    pub fn blend(&self, predictions: &[Vec<f64>]) -> Result<Vec<f64>> {
        if predictions.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "No predictions to blend".to_string()
            });
        }
        
        if predictions.len() != self.models.len() {
            return Err(AIServiceError::InvalidInput {
                message: format!("Expected {} predictions, got {}", self.models.len(), predictions.len())
            });
        }
        
        let output_size = predictions[0].len();
        
        match self.config.strategy {
            BlendingStrategy::SimpleAverage => {
                self.simple_average(predictions, output_size)
            }
            BlendingStrategy::WeightedAverage => {
                self.weighted_average(predictions, output_size)
            }
            BlendingStrategy::Max => {
                self.max_blend(predictions, output_size)
            }
            BlendingStrategy::Min => {
                self.min_blend(predictions, output_size)
            }
            BlendingStrategy::GeometricMean => {
                self.geometric_mean(predictions, output_size)
            }
            BlendingStrategy::HarmonicMean => {
                self.harmonic_mean(predictions, output_size)
            }
            BlendingStrategy::Median => {
                self.median_blend(predictions, output_size)
            }
            BlendingStrategy::MetaLearner => {
                if self.meta_weights.is_some() {
                    self.weighted_average(predictions, output_size)
                } else {
                    self.simple_average(predictions, output_size)
                }
            }
        }
    }
    
    fn simple_average(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![0.0; output_size];
        let n_models = predictions.len() as f64;
        
        for pred in predictions {
            for (i, &p) in pred.iter().enumerate() {
                blended[i] += p / n_models;
            }
        }
        
        Ok(blended)
    }
    
    fn weighted_average(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![0.0; output_size];
        
        for (pred, model) in predictions.iter().zip(self.models.iter()) {
            for (i, &p) in pred.iter().enumerate() {
                blended[i] += p * model.weight;
            }
        }
        
        Ok(blended)
    }
    
    fn max_blend(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![f64::NEG_INFINITY; output_size];
        
        for pred in predictions {
            for (i, &p) in pred.iter().enumerate() {
                blended[i] = blended[i].max(p);
            }
        }
        
        Ok(blended)
    }
    
    fn min_blend(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![f64::INFINITY; output_size];
        
        for pred in predictions {
            for (i, &p) in pred.iter().enumerate() {
                blended[i] = blended[i].min(p);
            }
        }
        
        Ok(blended)
    }
    
    fn geometric_mean(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![1.0; output_size];
        let n_models = predictions.len();
        
        for pred in predictions {
            for (i, &p) in pred.iter().enumerate() {
                if p > 0.0 {
                    blended[i] *= p.powf(1.0 / n_models as f64);
                }
            }
        }
        
        Ok(blended)
    }
    
    fn harmonic_mean(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![0.0; output_size];
        let n_models = predictions.len();
        
        for i in 0..output_size {
            let mut sum_inv = 0.0;
            for pred in predictions {
                if pred[i] > 0.0 {
                    sum_inv += 1.0 / pred[i];
                }
            }
            blended[i] = if sum_inv > 0.0 {
                n_models as f64 / sum_inv
            } else {
                0.0
            };
        }
        
        Ok(blended)
    }
    
    fn median_blend(&self, predictions: &[Vec<f64>], output_size: usize) -> Result<Vec<f64>> {
        let mut blended = vec![0.0; output_size];
        
        for i in 0..output_size {
            let mut values: Vec<f64> = predictions.iter().map(|p| p[i]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let len = values.len();
            if len % 2 == 0 {
                blended[i] = (values[len / 2 - 1] + values[len / 2]) / 2.0;
            } else {
                blended[i] = values[len / 2];
            }
        }
        
        Ok(blended)
    }
    
    /// Fit the blending ensemble (set meta-learner weights)
    pub fn fit(&mut self, predictions: &[Vec<f64>], targets: &[f64]) -> Result<()> {
        if self.config.use_meta_learner {
            // Compute optimal weights by minimizing MSE
            let n_models = self.models.len();
            let n_samples = predictions.len();
            
            // Simplified optimization: set weights inversely proportional to MSE
            let mut mses = Vec::with_capacity(n_models);
            
            for pred in predictions {
                let mse: f64 = pred.iter()
                    .zip(targets.iter())
                    .map(|(p, t)| (p - t).powi(2))
                    .sum::<f64>() / n_samples as f64;
                mses.push(mse);
            }
            
            // Convert MSE to weights (higher accuracy = higher weight)
            let total_inv: f64 = mses.iter().map(|&mse| 1.0 / (mse + 1e-8)).sum();
            let meta_weights: Vec<f64> = mses.iter()
                .map(|&mse| (1.0 / (mse + 1e-8)) / total_inv)
                .collect();
            
            self.meta_weights = Some(meta_weights);
        }
        
        self.is_fitted = true;
        Ok(())
    }
    
    /// Predict using the blending ensemble
    pub fn predict(&self, features: &[Vec<f64>]) -> Result<Vec<f64>> {
        // In production, would get actual model predictions
        // For now, return weighted average of placeholder predictions
        
        let n_samples = features.len();
        let n_models = self.models.len();
        
        let predictions: Vec<Vec<f64>> = (0..n_models)
            .map(|_| (0..n_samples).map(|_| 0.5).collect())
            .collect();
        
        self.blend(&predictions)
    }
    
    /// Get number of models
    pub fn num_models(&self) -> usize {
        self.models.len()
    }
    
    /// Get blending summary
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Blending Ensemble Summary\n");
        summary.push_str("========================\n\n");
        summary.push_str(&format!("Number of models: {}\n", self.models.len()));
        summary.push_str(&format!("Strategy: {:?}\n", self.config.strategy));
        summary.push_str(&format!("Adaptive weights: {}\n", self.config.adaptive_weights));
        summary.push_str(&format!("Weight decay: {}\n", self.config.weight_decay));
        summary.push_str(&format!("Use meta-learner: {}\n", self.config.use_meta_learner));
        summary.push_str(&format!("Is fitted: {}\n", self.is_fitted));
        
        summary.push_str("\nModels:\n");
        for (i, model) in self.models.iter().enumerate() {
            summary.push_str(&format!(
                "  {}: {} (weight={:.3}, performance={:.3})\n",
                i, model.model.name, model.weight, model.performance
            ));
        }
        
        summary
    }
}

/// Time-weighted blending (for temporal models)
#[derive(Debug, Clone)]
pub struct TimeWeightedBlending {
    pub models: Vec<WeightedModel>,
    pub time_weights: Vec<f64>,
    pub decay_rate: f64,
}

impl TimeWeightedBlending {
    pub fn new(decay_rate: f64) -> Self {
        Self {
            models: Vec::new(),
            time_weights: Vec::new(),
            decay_rate,
        }
    }
    
    /// Add model with timestamp
    pub fn add_model(&mut self, model: WeightedModel, timestamp: u64) {
        self.models.push(model);
        // Calculate time weight (exponential decay)
        self.time_weights.push((-self.decay_rate * timestamp as f64).exp());
    }
    
    /// Normalize time weights
    pub fn normalize_time_weights(&mut self) {
        let total: f64 = self.time_weights.iter().sum();
        if total > 0.0 {
            for w in &mut self.time_weights {
                *w /= total;
            }
        }
    }
    
    /// Blend with time weighting
    pub fn blend(&self, predictions: &[Vec<f64>]) -> Result<Vec<f64>> {
        if predictions.is_empty() || self.time_weights.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "No predictions or weights".to_string()
            });
        }
        
        let output_size = predictions[0].len();
        let mut blended = vec![0.0; output_size];
        
        for (pred, &time_weight) in predictions.iter().zip(self.time_weights.iter()) {
            for (i, &p) in pred.iter().enumerate() {
                blended[i] += p * time_weight;
            }
        }
        
        Ok(blended)
    }
}

/// Performance-based adaptive blending
#[derive(Debug, Clone)]
pub struct AdaptiveBlending {
    pub models: Vec<WeightedModel>,
    pub window_size: usize,
    pub performance_history: Vec<Vec<f64>>,
}

impl AdaptiveBlending {
    pub fn new(window_size: usize) -> Self {
        Self {
            models: Vec::new(),
            window_size,
            performance_history: Vec::new(),
        }
    }
    
    /// Add model to adaptive blender
    pub fn add_model(&mut self, model: WeightedModel) {
        self.models.push(model);
        self.performance_history.push(Vec::new());
    }
    
    /// Update model performance
    pub fn update_performance(&mut self, performances: Vec<f64>) {
        for (i, &perf) in performances.iter().enumerate() {
            if i < self.performance_history.len() {
                self.performance_history[i].push(perf);
                
                // Keep only recent performances
                if self.performance_history[i].len() > self.window_size {
                    self.performance_history[i].remove(0);
                }
            }
        }
        
        self.update_weights();
    }
    
    /// Update weights based on recent performance
    fn update_weights(&mut self) {
        for (model, history) in self.models.iter_mut().zip(self.performance_history.iter()) {
            if !history.is_empty() {
                // Use average performance over window
                let avg_perf: f64 = history.iter().sum::<f64>() / history.len() as f64;
                model.weight = avg_perf;
                model.performance = avg_perf;
            }
        }
        
        // Normalize
        let total: f64 = self.models.iter().map(|m| m.weight).sum();
        if total > 0.0 {
            for model in &mut self.models {
                model.weight /= total;
            }
        }
    }
    
    /// Blend predictions
    pub fn blend(&self, predictions: &[Vec<f64>]) -> Result<Vec<f64>> {
        let mut blender = BlendingEnsemble {
            config: BlendingConfig::default(),
            models: self.models.clone(),
            meta_weights: None,
            is_fitted: true,
        };
        blender.blend(predictions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blending_config_default() {
        let config = BlendingConfig::default();
        assert_eq!(config.n_models, 3);
        assert_eq!(config.strategy, BlendingStrategy::WeightedAverage);
    }

    #[test]
    fn test_blending_ensemble_creation() {
        let config = BlendingConfig::default();
        let blending = BlendingEnsemble::new(config);
        assert_eq!(blending.num_models(), 0);
    }

    #[test]
    fn test_add_model() {
        let mut blending = BlendingEnsemble::new(BlendingConfig::default());
        let model = EnsembleModel::new(0, "model1".to_string(), "linear".to_string());
        blending.add_model(model, 0.5);
        assert_eq!(blending.num_models(), 1);
        assert_eq!(blending.models[0].weight, 0.5);
    }

    #[test]
    fn test_normalize_weights() {
        let mut blending = BlendingEnsemble::new(BlendingConfig::default());
        let model1 = EnsembleModel::new(0, "m1".to_string(), "linear".to_string());
        let model2 = EnsembleModel::new(1, "m2".to_string(), "linear".to_string());
        
        blending.add_model(model1, 2.0);
        blending.add_model(model2, 4.0);
        
        blending.normalize_weights();
        
        let weights = blending.get_weights();
        assert!((weights[0] - 0.333).abs() < 0.01);
        assert!((weights[1] - 0.667).abs() < 0.01);
    }

    #[test]
    fn test_simple_average() {
        let mut blending = BlendingEnsemble::new(BlendingConfig {
            strategy: BlendingStrategy::SimpleAverage,
            ..Default::default()
        });
        
        blending.add_model(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()), 1.0);
        blending.add_model(EnsembleModel::new(1, "m2".to_string(), "linear".to_string()), 1.0);
        
        let predictions = vec![
            vec![0.2, 0.8],
            vec![0.4, 0.6],
        ];
        
        let blended = blending.blend(&predictions).unwrap();
        assert_eq!(blended[0], 0.3);
        assert_eq!(blended[1], 0.7);
    }

    #[test]
    fn test_weighted_average() {
        let mut blending = BlendingEnsemble::new(BlendingConfig::default());
        
        blending.add_model(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()), 0.3);
        blending.add_model(EnsembleModel::new(1, "m2".to_string(), "linear".to_string()), 0.7);
        blending.normalize_weights();
        
        let predictions = vec![
            vec![0.2, 0.8],
            vec![0.4, 0.6],
        ];
        
        let blended = blending.blend(&predictions).unwrap();
        assert!((blended[0] - 0.34).abs() < 0.01);
        assert!((blended[1] - 0.66).abs() < 0.01);
    }

    #[test]
    fn test_time_weighted_blending() {
        let mut twb = TimeWeightedBlending::new(0.1);
        let model = WeightedModel::new(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()), 1.0);
        twb.add_model(model, 0);
        twb.normalize_time_weights();
        assert_eq!(twb.time_weights[0], 1.0);
    }

    #[test]
    fn test_adaptive_blending() {
        let mut ab = AdaptiveBlending::new(10);
        let model = WeightedModel::new(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()), 1.0);
        ab.add_model(model);
        
        ab.update_performance(vec![0.8]);
        assert_eq!(ab.models[0].performance, 0.8);
    }
}