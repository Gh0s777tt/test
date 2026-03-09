//! Ensemble Methods Implementation for VantisOS
//!
//! This module provides ensemble methods for combining multiple models:
//! - Base ensemble framework
//! - Voting classifiers and regressors
//! - Bagging and boosting
//! - Model averaging
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::ensembles::{Ensemble, EnsembleConfig};
//!
//! let config = EnsembleConfig::default();
//! let mut ensemble = Ensemble::new(config);
//! ensemble.add_model(model1);
//! ensemble.add_model(model2);
//! let prediction = ensemble.predict(&features);
//! ```

use crate::ai::error::{AIServiceError, Result};
use crate::ai::types::{ModelMetrics, PredictionResult};
use std::collections::HashMap;

/// Ensemble configuration
#[derive(Debug, Clone)]
pub struct EnsembleConfig {
    /// Number of models in ensemble
    pub num_models: usize,
    /// Use cross-validation for weights
    pub use_cross_validation: bool,
    /// Number of CV folds
    pub cv_folds: usize,
    /// Use dynamic selection
    pub dynamic_selection: bool,
    /// Voting strategy
    pub voting: VotingStrategy,
    /// Model weights (optional)
    pub weights: Option<Vec<f64>>,
}

impl Default for EnsembleConfig {
    fn default() -> Self {
        Self {
            num_models: 5,
            use_cross_validation: true,
            cv_folds: 5,
            dynamic_selection: false,
            voting: VotingStrategy::Soft,
            weights: None,
        }
    }
}

/// Voting strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotingStrategy {
    /// Hard voting (majority vote)
    Hard,
    /// Soft voting (weighted average)
    Soft,
}

/// Ensemble method types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnsembleMethod {
    /// Voting ensemble
    Voting,
    /// Bagging
    Bagging,
    /// Boosting
    Boosting,
    /// Random Forest style
    RandomForest,
}

/// Model wrapper for ensemble
#[derive(Debug, Clone)]
pub struct EnsembleModel {
    pub id: usize,
    pub name: String,
    pub model_type: String,
    pub weight: f64,
    pub is_trained: bool,
}

impl EnsembleModel {
    pub fn new(id: usize, name: String, model_type: String) -> Self {
        Self {
            id,
            name,
            model_type,
            weight: 1.0,
            is_trained: false,
        }
    }
    
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

/// Ensemble base structure
#[derive(Debug, Clone)]
pub struct Ensemble {
    pub config: EnsembleConfig,
    pub models: Vec<EnsembleModel>,
    pub model_weights: Vec<f64>,
    pub is_trained: bool,
}

impl Ensemble {
    pub fn new(config: EnsembleConfig) -> Self {
        Self {
            config,
            models: Vec::new(),
            model_weights: Vec::new(),
            is_trained: false,
        }
    }
    
    /// Add a model to the ensemble
    pub fn add_model(&mut self, model: EnsembleModel) {
        let weight = model.weight;
        self.models.push(model);
        self.model_weights.push(weight);
    }
    
    /// Remove a model from the ensemble
    pub fn remove_model(&mut self, model_id: usize) -> Result<()> {
        let idx = self.models.iter().position(|m| m.id == model_id)
            .ok_or_else(|| AIServiceError::InvalidInput {
                message: format!("Model {} not found", model_id)
            })?;
        
        self.models.remove(idx);
        self.model_weights.remove(idx);
        Ok(())
    }
    
    /// Set model weights
    pub fn set_weights(&mut self, weights: Vec<f64>) -> Result<()> {
        if weights.len() != self.models.len() {
            return Err(AIServiceError::InvalidInput {
                message: format!("Expected {} weights, got {}", self.models.len(), weights.len())
            });
        }
        
        self.model_weights = weights.clone();
        for (model, &weight) in self.models.iter_mut().zip(weights.iter()) {
            model.weight = weight;
        }
        
        Ok(())
    }
    
    /// Normalize weights to sum to 1.0
    pub fn normalize_weights(&mut self) {
        let sum: f64 = self.model_weights.iter().sum();
        if sum > 0.0 {
            for w in &mut self.model_weights {
                *w /= sum;
            }
            for model in &mut self.models {
                model.weight /= sum;
            }
        }
    }
    
    /// Get number of models
    pub fn num_models(&self) -> usize {
        self.models.len()
    }
    
    /// Combine predictions using voting
    pub fn combine_predictions(&self, predictions: &[Vec<f64>]) -> Result<Vec<f64>> {
        if predictions.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "No predictions to combine".to_string()
            });
        }
        
        if predictions.len() != self.models.len() {
            return Err(AIServiceError::InvalidInput {
                message: format!("Expected {} predictions, got {}", self.models.len(), predictions.len())
            });
        }
        
        let output_size = predictions[0].len();
        
        match self.config.voting {
            VotingStrategy::Hard => {
                // Hard voting: take mode of predictions
                let mut combined = vec![0.0; output_size];
                for i in 0..output_size {
                    let mut counts: HashMap<i64, usize> = HashMap::new();
                    for (pred, &weight) in predictions.iter().zip(self.model_weights.iter()) {
                        let class = (pred[i] * 100.0) as i64; // Discretize
                        *counts.entry(class).or_insert(0) += (weight * 100.0) as usize;
                    }
                    
                    let best_class = counts.into_iter()
                        .max_by_key(|&(_, count)| count)
                        .map(|(class, _)| class)
                        .unwrap_or(0);
                    combined[i] = best_class as f64 / 100.0;
                }
                Ok(combined)
            }
            VotingStrategy::Soft => {
                // Soft voting: weighted average
                let mut combined = vec![0.0; output_size];
                for (pred, &weight) in predictions.iter().zip(self.model_weights.iter()) {
                    for (i, &p) in pred.iter().enumerate() {
                        combined[i] += p * weight;
                    }
                }
                Ok(combined)
            }
        }
    }
    
    /// Get ensemble summary
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Ensemble Summary\n");
        summary.push_str("================\n\n");
        summary.push_str(&format!("Number of models: {}\n", self.models.len()));
        summary.push_str(&format!("Voting strategy: {:?}\n", self.config.voting));
        summary.push_str(&format!("Cross-validation: {}\n", self.config.use_cross_validation));
        summary.push_str(&format!("Dynamic selection: {}\n", self.config.dynamic_selection));
        
        summary.push_str("\nModels:\n");
        for model in &self.models {
            summary.push_str(&format!(
                "  {} ({}): weight={:.3}\n",
                model.name,
                model.model_type,
                model.weight
            ));
        }
        
        summary
    }
}

/// Voting classifier
#[derive(Debug, Clone)]
pub struct VotingClassifier {
    pub ensemble: Ensemble,
    pub classes: Vec<String>,
}

impl VotingClassifier {
    pub fn new(config: EnsembleConfig, classes: Vec<String>) -> Self {
        let ensemble = Ensemble::new(config);
        Self { ensemble, classes }
    }
    
    /// Add model to voting classifier
    pub fn add_model(&mut self, model: EnsembleModel) {
        self.ensemble.add_model(model);
    }
    
    /// Predict class
    pub fn predict(&self, predictions: &[Vec<f64>]) -> Result<Vec<String>> {
        let combined = self.ensemble.combine_predictions(predictions)?;
        
        // Convert probabilities to class labels
        let class_indices: Vec<usize> = combined.iter()
            .map(|&prob| {
                (prob * (self.classes.len() - 1) as f64).round() as usize
                    .min(self.classes.len() - 1)
            })
            .collect();
        
        let labels: Vec<String> = class_indices.iter()
            .map(|&i| self.classes[i].clone())
            .collect();
        
        Ok(labels)
    }
}

/// Voting regressor
#[derive(Debug, Clone)]
pub struct VotingRegressor {
    pub ensemble: Ensemble,
}

impl VotingRegressor {
    pub fn new(config: EnsembleConfig) -> Self {
        let ensemble = Ensemble::new(config);
        Self { ensemble }
    }
    
    /// Add model to voting regressor
    pub fn add_model(&mut self, model: EnsembleModel) {
        self.ensemble.add_model(model);
    }
    
    /// Predict values
    pub fn predict(&self, predictions: &[Vec<f64>]) -> Result<Vec<f64>> {
        self.ensemble.combine_predictions(predictions)
    }
}

/// Bagging configuration
#[derive(Debug, Clone)]
pub struct BaggingConfig {
    /// Number of base estimators
    pub n_estimators: usize,
    /// Fraction of samples for each estimator
    pub max_samples: f64,
    /// Fraction of features for each estimator
    pub max_features: f64,
    /// Use bootstrap sampling
    pub bootstrap: bool,
    /// Use out-of-bag evaluation
    pub oob_score: bool,
    /// Number of parallel jobs
    pub n_jobs: usize,
}

impl Default for BaggingConfig {
    fn default() -> Self {
        Self {
            n_estimators: 10,
            max_samples: 1.0,
            max_features: 1.0,
            bootstrap: true,
            oob_score: true,
            n_jobs: 1,
        }
    }
}

/// Bagging ensemble
#[derive(Debug, Clone)]
pub struct BaggingEnsemble {
    pub config: BaggingConfig,
    pub estimators: Vec<EnsembleModel>,
    pub oob_score: Option<f64>,
}

impl BaggingEnsemble {
    pub fn new(config: BaggingConfig) -> Self {
        Self {
            config,
            estimators: Vec::new(),
            oob_score: None,
        }
    }
    
    /// Generate bootstrap sample indices
    pub fn bootstrap_sample(&self, n_samples: usize) -> Vec<usize> {
        let sample_size = (n_samples as f64 * self.config.max_samples) as usize;
        let mut indices = Vec::with_capacity(sample_size);
        
        // Simple deterministic bootstrap for reproducibility
        for i in 0..sample_size {
            indices.push((i * 7 + 13) % n_samples); // Simple pseudo-random
        }
        
        indices
    }
    
    /// Generate feature subset indices
    pub fn feature_subset(&self, n_features: usize) -> Vec<usize> {
        let feature_count = (n_features as f64 * self.config.max_features) as usize;
        (0..feature_count).collect()
    }
    
    /// Calculate out-of-bag score
    pub fn calculate_oob_score(&mut self, predictions: &[Vec<f64>], targets: &[f64]) {
        let mse: f64 = predictions.iter()
            .zip(targets.iter())
            .map(|(pred, &target)| {
                let pred_val = pred.get(0).copied().unwrap_or(0.0);
                (pred_val - target).powi(2)
            })
            .sum::<f64>() / predictions.len() as f64;
        
        self.oob_score = Some(mse.sqrt());
    }
    
    /// Add estimator
    pub fn add_estimator(&mut self, estimator: EnsembleModel) {
        self.estimators.push(estimator);
    }
    
    /// Get number of estimators
    pub fn num_estimators(&self) -> usize {
        self.estimators.len()
    }
}

/// Boosting configuration
#[derive(Debug, Clone)]
pub struct BoostingConfig {
    /// Number of estimators
    pub n_estimators: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Maximum depth of trees
    pub max_depth: usize,
    /// Minimum samples split
    pub min_samples_split: usize,
    /// Loss function
    pub loss: LossFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossFunction {
    /// Squared error for regression
    SquaredError,
    /// Absolute error for regression
    AbsoluteError,
    /// Logistic loss for classification
    Logistic,
    /// Exponential loss (AdaBoost)
    Exponential,
}

impl Default for BoostingConfig {
    fn default() -> Self {
        Self {
            n_estimators: 100,
            learning_rate: 0.1,
            max_depth: 3,
            min_samples_split: 2,
            loss: LossFunction::SquaredError,
        }
    }
}

/// Boosting ensemble
#[derive(Debug, Clone)]
pub struct BoostingEnsemble {
    pub config: BoostingConfig,
    pub estimators: Vec<EnsembleModel>,
    pub estimator_weights: Vec<f64>,
    pub training_scores: Vec<f64>,
}

impl BoostingEnsemble {
    pub fn new(config: BoostingConfig) -> Self {
        Self {
            config,
            estimators: Vec::new(),
            estimator_weights: Vec::new(),
            training_scores: Vec::new(),
        }
    }
    
    /// Calculate loss
    pub fn calculate_loss(&self, prediction: f64, target: f64) -> f64 {
        match self.config.loss {
            LossFunction::SquaredError => (prediction - target).powi(2),
            LossFunction::AbsoluteError => (prediction - target).abs(),
            LossFunction::Logistic => {
                let p = 1.0 / (1.0 + (-prediction).exp());
                if target > 0.5 {
                    -p.ln()
                } else {
                    -(1.0 - p).ln()
                }
            }
            LossFunction::Exponential => (-target * prediction).exp(),
        }
    }
    
    /// Calculate gradient
    pub fn calculate_gradient(&self, prediction: f64, target: f64) -> f64 {
        match self.config.loss {
            LossFunction::SquaredError => 2.0 * (prediction - target),
            LossFunction::AbsoluteError => if prediction > target { 1.0 } else { -1.0 },
            LossFunction::Logistic => {
                let p = 1.0 / (1.0 + (-prediction).exp());
                p - target
            }
            LossFunction::Exponential => -target * (-target * prediction).exp(),
        }
    }
    
    /// Update weights after adding an estimator
    pub fn update_weights(&mut self, estimator_weight: f64) {
        self.estimator_weights.push(estimator_weight);
    }
    
    /// Add estimator
    pub fn add_estimator(&mut self, estimator: EnsembleModel, weight: f64) {
        self.estimators.push(estimator);
        self.estimator_weights.push(weight);
    }
    
    /// Combine predictions from all estimators
    pub fn combine_predictions(&self, predictions: &[Vec<f64>]) -> Vec<f64> {
        if predictions.is_empty() || self.estimator_weights.is_empty() {
            return vec![0.0];
        }
        
        let output_size = predictions[0].len();
        let mut combined = vec![0.0; output_size];
        
        for (pred, &weight) in predictions.iter().zip(self.estimator_weights.iter()) {
            for (i, &p) in pred.iter().enumerate() {
                combined[i] += p * weight * self.config.learning_rate;
            }
        }
        
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensemble_creation() {
        let config = EnsembleConfig::default();
        let ensemble = Ensemble::new(config);
        assert_eq!(ensemble.num_models(), 0);
    }

    #[test]
    fn test_add_model() {
        let mut ensemble = Ensemble::new(EnsembleConfig::default());
        let model = EnsembleModel::new(0, "model1".to_string(), "linear".to_string());
        ensemble.add_model(model);
        assert_eq!(ensemble.num_models(), 1);
    }

    #[test]
    fn test_combine_predictions_soft() {
        let mut ensemble = Ensemble::new(EnsembleConfig::default());
        ensemble.add_model(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()));
        ensemble.add_model(EnsembleModel::new(1, "m2".to_string(), "linear".to_string()));
        ensemble.normalize_weights();
        
        let predictions = vec![
            vec![0.3, 0.7],
            vec![0.4, 0.6],
        ];
        
        let combined = ensemble.combine_predictions(&predictions).unwrap();
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn test_voting_classifier() {
        let classes = vec!["A".to_string(), "B".to_string()];
        let mut clf = VotingClassifier::new(EnsembleConfig::default(), classes);
        
        clf.add_model(EnsembleModel::new(0, "m1".to_string(), "linear".to_string()));
        clf.add_model(EnsembleModel::new(1, "m2".to_string(), "linear".to_string()));
        
        let predictions = vec![
            vec![0.3, 0.7],
            vec![0.4, 0.6],
        ];
        
        let labels = clf.predict(&predictions).unwrap();
        assert_eq!(labels.len(), 2);
    }

    #[test]
    fn test_bagging_ensemble() {
        let bagging = BaggingEnsemble::new(BaggingConfig::default());
        assert_eq!(bagging.num_estimators(), 0);
    }

    #[test]
    fn test_boosting_ensemble() {
        let boosting = BoostingEnsemble::new(BoostingConfig::default());
        assert!(boosting.estimators.is_empty());
    }
}