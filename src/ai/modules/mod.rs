//! AI Modules for VantisOS
//! 
//! This module contains sub-modules for the AI data pipeline:
//! - **data_collector**: Collects raw system metrics
//! - **data_processor**: Processes and transforms data for ML
//! - **trainer**: Trains and validates ML models
//! 
//! v1.4.0 adds advanced ML modules:
//! - **neural_networks**: Deep Neural Networks (DNN)
//! - **cnn**: Convolutional Neural Networks
//! - **rnn**: Recurrent Neural Networks (RNN, LSTM, GRU)
//! - **attention**: Attention Mechanisms
//! - **ensembles**: Ensemble framework
//! - **stacking**: Stacking ensembles
//! - **blending**: Blending ensembles
//! - **hyperopt**: Advanced hyperparameter optimization
//! - **multi_objective**: Multi-objective optimization
//! - **nas**: Neural Architecture Search
//! 
//! ## Architecture
//! 
//! ```text
//! ┌─────────────────┐    ┌──────────────────┐    ┌─────────────┐
//! │ Data Collector  │ -> │ Data Processor   │ -> │   Trainer   │
//! │                 │    │                  │    │             │
//! │ - CPU metrics   │    │ - Feature        │    │ - Train     │
//! │ - Memory stats  │    │   extraction     │    │ - Validate  │
//! │ - Disk I/O      │    │ - Normalization  │    │ - Fine-tune │
//! │ - Network stats │    │ - Aggregation    │    │             │
//! └─────────────────┘    └──────────────────┘    └─────────────┘
//! ```
//! 
//! ## Security
//! - All data is processed locally
//! - No external network access
//! - Differential privacy support
//! - Memory-bounded operations

// Core Pipeline Modules
pub mod data_collector;
pub mod data_processor;
pub mod trainer;

// v1.4.0 Neural Network Modules
pub mod neural_networks;
pub mod cnn;
pub mod rnn;
pub mod attention;

// v1.4.0 Ensemble Modules
pub mod ensembles;
pub mod stacking;
pub mod blending;

// v1.4.0 Hyperparameter Optimization Modules
pub mod hyperopt;
pub mod multi_objective;
pub mod nas;

// Re-exports
pub use data_collector::DataCollector;
pub use data_processor::DataProcessor;
pub use trainer::ModelTrainer;

use crate::ai::error::{AIServiceError, Result};
use crate::ai::types::ModelConfig;
use std::collections::HashMap;

/// Common activation functions for neural networks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationFunction {
    /// Rectified Linear Unit
    ReLU,
    /// Gaussian Error Linear Unit
    GELU,
    /// Swish activation
    Swish,
    /// Sigmoid
    Sigmoid,
    /// Hyperbolic Tangent
    Tanh,
    /// Softmax
    Softmax,
}

impl ActivationFunction {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::ReLU => x.max(0.0),
            ActivationFunction::GELU => 0.5 * x * (1.0 + (0.7978845608 * (x + 0.044715 * x.powi(3))).tanh()),
            ActivationFunction::Swish => x / (1.0 + (-x).exp()),
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            ActivationFunction::Tanh => x.tanh(),
            ActivationFunction::Softmax => {
                // Softmax is applied to a vector, not a single value
                1.0 / (1.0 + (-x).exp())
            }
        }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::ReLU => if x > 0.0 { 1.0 } else { 0.0 },
            ActivationFunction::GELU => {
                let pdf = (-0.5 * x.powi(2)).exp() * (2.0 / std::f64::consts::PI).sqrt();
                let cdf = 0.5 * (1.0 + (0.7071067812 * x).erf());
                let x3 = x.powi(3);
                let cdf_prime = pdf * (0.7978845608 + 0.5357224608 * x3);
                0.5 * cdf + 0.5 * x * cdf_prime
            }
            ActivationFunction::Swish => {
                let sigmoid = 1.0 / (1.0 + (-x).exp());
                sigmoid + x * sigmoid * (1.0 - sigmoid)
            }
            ActivationFunction::Sigmoid => {
                let s = 1.0 / (1.0 + (-x).exp());
                s * (1.0 - s)
            }
            ActivationFunction::Tanh => 1.0 - x.tanh().powi(2),
            ActivationFunction::Softmax => {
                1.0 / (1.0 + (-x).exp()) * (1.0 - 1.0 / (1.0 + (-x).exp()))
            }
        }
    }
}

/// Common optimization algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationAlgorithm {
    /// Stochastic Gradient Descent
    SGD,
    /// Adam optimizer
    Adam,
    /// AdamW optimizer (Adam with decoupled weight decay)
    AdamW,
    /// RMSprop optimizer
    RMSprop,
    /// Rectified Adam
    RAdam,
}

/// Regularization techniques
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Regularization {
    /// Dropout regularization
    Dropout(f64),
    /// L1 regularization
    L1(f64),
    /// L2 regularization
    L2(f64),
    /// Elastic Net (combination of L1 and L2)
    ElasticNet(f64, f64),
}

/// Layer configuration for neural networks
#[derive(Debug, Clone)]
pub struct LayerConfig {
    pub input_size: usize,
    pub output_size: usize,
    pub activation: ActivationFunction,
    pub regularization: Option<Regularization>,
    pub use_batch_norm: bool,
}

impl Default for LayerConfig {
    fn default() -> Self {
        Self {
            input_size: 128,
            output_size: 64,
            activation: ActivationFunction::ReLU,
            regularization: None,
            use_batch_norm: false,
        }
    }
}

/// Neural network architecture
#[derive(Debug, Clone)]
pub struct NeuralNetworkArchitecture {
    pub layers: Vec<LayerConfig>,
    pub optimizer: OptimizationAlgorithm,
    pub learning_rate: f64,
    pub input_size: usize,
    pub output_size: usize,
}

impl Default for NeuralNetworkArchitecture {
    fn default() -> Self {
        Self {
            layers: vec![
                LayerConfig {
                    input_size: 128,
                    output_size: 256,
                    activation: ActivationFunction::ReLU,
                    regularization: Some(Regularization::L2(0.01)),
                    use_batch_norm: true,
                },
                LayerConfig {
                    input_size: 256,
                    output_size: 128,
                    activation: ActivationFunction::ReLU,
                    regularization: Some(Regularization::L2(0.01)),
                    use_batch_norm: true,
                },
                LayerConfig {
                    input_size: 128,
                    output_size: 64,
                    activation: ActivationFunction::GELU,
                    regularization: Some(Regularization::L2(0.01)),
                    use_batch_norm: true,
                },
            ],
            optimizer: OptimizationAlgorithm::Adam,
            learning_rate: 0.001,
            input_size: 128,
            output_size: 64,
        }
    }
}

/// Ensemble configuration
#[derive(Debug, Clone)]
pub struct EnsembleConfig {
    pub models: Vec<ModelConfig>,
    pub weights: Option<Vec<f64>>,
    pub use_cross_validation: bool,
    pub cv_folds: usize,
    pub dynamic_selection: bool,
}

impl Default for EnsembleConfig {
    fn default() -> Self {
        Self {
            models: vec![],
            weights: None,
            use_cross_validation: true,
            cv_folds: 5,
            dynamic_selection: false,
        }
    }
}

/// Hyperparameter optimization configuration
#[derive(Debug, Clone)]
pub struct HyperoptConfig {
    pub max_iterations: usize,
    pub n_random_starts: usize,
    pub early_stopping: Option<usize>,
    pub optimization_type: OptimizationType,
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationType {
    SingleObjective,
    MultiObjective,
}

impl Default for HyperoptConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            n_random_starts: 10,
            early_stopping: None,
            optimization_type: OptimizationType::SingleObjective,
            timeout_seconds: None,
        }
    }
}

/// Result of hyperparameter optimization
#[derive(Debug, Clone)]
pub struct HyperoptResult {
    pub best_params: HashMap<String, f64>,
    pub best_score: f64,
    pub iteration: usize,
    pub convergence_history: Vec<f64>,
    pub optimization_time: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activation_functions() {
        assert_eq!(ActivationFunction::ReLU.apply(-1.0), 0.0);
        assert_eq!(ActivationFunction::ReLU.apply(1.0), 1.0);
        
        let sigmoid_result = ActivationFunction::Sigmoid.apply(0.0);
        assert!((sigmoid_result - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_layer_config_default() {
        let config = LayerConfig::default();
        assert_eq!(config.input_size, 128);
        assert_eq!(config.output_size, 64);
    }

    #[test]
    fn test_neural_network_architecture_default() {
        let arch = NeuralNetworkArchitecture::default();
        assert_eq!(arch.layers.len(), 3);
        assert_eq!(arch.learning_rate, 0.001);
    }
}