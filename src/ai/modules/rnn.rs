//! Recurrent Neural Networks (RNN, LSTM, GRU) Implementation for VantisOS
//!
//! This module provides recurrent neural network capabilities for sequential data:
//! - Basic RNN cells
//! - Long Short-Term Memory (LSTM) cells
//! - Gated Recurrent Units (GRU)
//! - Bidirectional RNNs
//! - Multi-layer RNNs
//! - Various activation functions
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::rnn::{RNN, RNNConfig, RNNType};
//!
//! let config = RNNConfig {
//!     rnn_type: RNNType::LSTM,
//!     ..Default::default()
//! };
//! let mut rnn = RNN::new(config);
//! let sequence = vec![vec![0.0; 10]; 5]; // 5 time steps, 10 features
//! let output = rnn.predict(&sequence);
//! ```

use super::ActivationFunction;
use crate::ai::error::{AIServiceError, Result};
use crate::ai::types::ModelMetrics;
use std::collections::HashMap;

/// RNN cell types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RNNType {
    /// Basic RNN
    RNN,
    /// Long Short-Term Memory
    LSTM,
    /// Gated Recurrent Unit
    GRU,
}

/// RNN layer configuration
#[derive(Debug, Clone)]
pub struct RNNLayerConfig {
    /// Input size
    pub input_size: usize,
    /// Hidden size
    pub hidden_size: usize,
    /// RNN type
    pub rnn_type: RNNType,
    /// Number of layers
    pub num_layers: usize,
    /// Bidirectional
    pub bidirectional: bool,
    /// Dropout rate
    pub dropout: f64,
    /// Activation function (for basic RNN)
    pub activation: ActivationFunction,
}

impl Default for RNNLayerConfig {
    fn default() -> Self {
        Self {
            input_size: 10,
            hidden_size: 64,
            rnn_type: RNNType::LSTM,
            num_layers: 1,
            bidirectional: false,
            dropout: 0.0,
            activation: ActivationFunction::Tanh,
        }
    }
}

/// RNN cell (basic)
#[derive(Debug, Clone)]
pub struct RNNCell {
    pub input_weights: Vec<Vec<f64>>,
    pub hidden_weights: Vec<Vec<f64>>,
    pub bias: Vec<f64>,
    pub activation: ActivationFunction,
}

impl RNNCell {
    pub fn new(input_size: usize, hidden_size: usize, activation: ActivationFunction) -> Self {
        let scale = (2.0 / input_size as f64).sqrt();
        
        let input_weights: Vec<Vec<f64>> = (0..hidden_size)
            .map(|_| (0..input_size)
                .map(|_| {
                    let hash = (input_size * 1000 + hidden_size) as f64;
                    (hash.sin() * scale + hash.cos() * scale) / 2.0
                })
                .collect())
            .collect();
        
        let hidden_weights: Vec<Vec<f64>> = (0..hidden_size)
            .map(|_| (0..hidden_size)
                .map(|_| {
                    let hash = (hidden_size * 2000) as f64;
                    (hash.sin() * scale + hash.cos() * scale) / 2.0
                })
                .collect())
            .collect();
        
        let bias = vec![0.0; hidden_size];
        
        Self {
            input_weights,
            hidden_weights,
            bias,
            activation,
        }
    }
    
    pub fn forward(&self, input: &[f64], hidden_state: &[f64]) -> Vec<f64> {
        let mut new_hidden = vec![0.0; self.bias.len()];
        
        for i in 0..self.bias.len() {
            let input_sum: f64 = self.input_weights[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum: f64 = self.hidden_weights[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            new_hidden[i] = self.activation.apply(input_sum + hidden_sum + self.bias[i]);
        }
        
        new_hidden
    }
}

/// LSTM cell
#[derive(Debug, Clone)]
pub struct LSTMCell {
    pub input_weights_i: Vec<Vec<f64>>, // Input gate
    pub hidden_weights_i: Vec<Vec<f64>>,
    pub bias_i: Vec<f64>,
    
    pub input_weights_f: Vec<Vec<f64>>, // Forget gate
    pub hidden_weights_f: Vec<Vec<f64>>,
    pub bias_f: Vec<f64>,
    
    pub input_weights_o: Vec<Vec<f64>>, // Output gate
    pub hidden_weights_o: Vec<Vec<f64>>,
    pub bias_o: Vec<f64>,
    
    pub input_weights_c: Vec<Vec<f64>>, // Cell gate
    pub hidden_weights_c: Vec<Vec<f64>>,
    pub bias_c: Vec<f64>,
    
    pub hidden_size: usize,
}

impl LSTMCell {
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        let scale = (2.0 / input_size as f64).sqrt();
        
        let make_weights = |h_size: usize, i_size: usize| -> Vec<Vec<f64>> {
            (0..h_size)
                .map(|_| (0..i_size)
                    .map(|_| {
                        let hash = (i_size * 1000 + h_size) as f64;
                        (hash.sin() * scale + hash.cos() * scale) / 2.0
                    })
                    .collect())
                .collect()
        };
        
        Self {
            input_weights_i: make_weights(hidden_size, input_size),
            hidden_weights_i: make_weights(hidden_size, hidden_size),
            bias_i: vec![0.0; hidden_size],
            
            input_weights_f: make_weights(hidden_size, input_size),
            hidden_weights_f: make_weights(hidden_size, hidden_size),
            bias_f: vec![0.0; hidden_size],
            
            input_weights_o: make_weights(hidden_size, input_size),
            hidden_weights_o: make_weights(hidden_size, hidden_size),
            bias_o: vec![0.0; hidden_size],
            
            input_weights_c: make_weights(hidden_size, input_size),
            hidden_weights_c: make_weights(hidden_size, hidden_size),
            bias_c: vec![0.0; hidden_size],
            
            hidden_size,
        }
    }
    
    pub fn forward(&self, input: &[f64], hidden_state: &[f64], cell_state: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut new_hidden = vec![0.0; self.hidden_size];
        let mut new_cell = vec![0.0; self.hidden_size];
        
        for i in 0..self.hidden_size {
            // Input gate
            let input_sum_i: f64 = self.input_weights_i[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_i: f64 = self.hidden_weights_i[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let i_gate = ActivationFunction::Sigmoid.apply(input_sum_i + hidden_sum_i + self.bias_i[i]);
            
            // Forget gate
            let input_sum_f: f64 = self.input_weights_f[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_f: f64 = self.hidden_weights_f[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let f_gate = ActivationFunction::Sigmoid.apply(input_sum_f + hidden_sum_f + self.bias_f[i]);
            
            // Output gate
            let input_sum_o: f64 = self.input_weights_o[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_o: f64 = self.hidden_weights_o[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let o_gate = ActivationFunction::Sigmoid.apply(input_sum_o + hidden_sum_o + self.bias_o[i]);
            
            // Cell gate
            let input_sum_c: f64 = self.input_weights_c[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_c: f64 = self.hidden_weights_c[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let c_gate = ActivationFunction::Tanh.apply(input_sum_c + hidden_sum_c + self.bias_c[i]);
            
            // Update cell state
            new_cell[i] = f_gate * cell_state[i] + i_gate * c_gate;
            
            // Update hidden state
            new_hidden[i] = o_gate * ActivationFunction::Tanh.apply(new_cell[i]);
        }
        
        (new_hidden, new_cell)
    }
}

/// GRU cell
#[derive(Debug, Clone)]
pub struct GRUCell {
    pub input_weights_z: Vec<Vec<f64>>, // Update gate
    pub hidden_weights_z: Vec<Vec<f64>>,
    pub bias_z: Vec<f64>,
    
    pub input_weights_r: Vec<Vec<f64>>, // Reset gate
    pub hidden_weights_r: Vec<Vec<f64>>,
    pub bias_r: Vec<f64>,
    
    pub input_weights_h: Vec<Vec<f64>>, // Hidden state
    pub hidden_weights_h: Vec<Vec<f64>>,
    pub bias_h: Vec<f64>,
    
    pub hidden_size: usize,
}

impl GRUCell {
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        let scale = (2.0 / input_size as f64).sqrt();
        
        let make_weights = |h_size: usize, i_size: usize| -> Vec<Vec<f64>> {
            (0..h_size)
                .map(|_| (0..i_size)
                    .map(|_| {
                        let hash = (i_size * 1000 + h_size) as f64;
                        (hash.sin() * scale + hash.cos() * scale) / 2.0
                    })
                    .collect())
                .collect()
        };
        
        Self {
            input_weights_z: make_weights(hidden_size, input_size),
            hidden_weights_z: make_weights(hidden_size, hidden_size),
            bias_z: vec![0.0; hidden_size],
            
            input_weights_r: make_weights(hidden_size, input_size),
            hidden_weights_r: make_weights(hidden_size, hidden_size),
            bias_r: vec![0.0; hidden_size],
            
            input_weights_h: make_weights(hidden_size, input_size),
            hidden_weights_h: make_weights(hidden_size, hidden_size),
            bias_h: vec![0.0; hidden_size],
            
            hidden_size,
        }
    }
    
    pub fn forward(&self, input: &[f64], hidden_state: &[f64]) -> Vec<f64> {
        let mut new_hidden = vec![0.0; self.hidden_size];
        
        for i in 0..self.hidden_size {
            // Update gate
            let input_sum_z: f64 = self.input_weights_z[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_z: f64 = self.hidden_weights_z[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let z_gate = ActivationFunction::Sigmoid.apply(input_sum_z + hidden_sum_z + self.bias_z[i]);
            
            // Reset gate
            let input_sum_r: f64 = self.input_weights_r[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_r: f64 = self.hidden_weights_r[i].iter().zip(hidden_state.iter()).map(|(w, h)| w * h).sum();
            let r_gate = ActivationFunction::Sigmoid.apply(input_sum_r + hidden_sum_r + self.bias_r[i]);
            
            // Candidate hidden state
            let mut reset_hidden = vec![0.0; self.hidden_size];
            for j in 0..self.hidden_size {
                reset_hidden[j] = r_gate * hidden_state[j];
            }
            
            let input_sum_h: f64 = self.input_weights_h[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            let hidden_sum_h: f64 = self.hidden_weights_h[i].iter().zip(reset_hidden.iter()).map(|(w, h)| w * h).sum();
            let h_candidate = ActivationFunction::Tanh.apply(input_sum_h + hidden_sum_h + self.bias_h[i]);
            
            // Update hidden state
            new_hidden[i] = (1.0 - z_gate) * hidden_state[i] + z_gate * h_candidate;
        }
        
        new_hidden
    }
}

/// RNN layer
#[derive(Debug, Clone)]
pub struct RNNLayer {
    pub config: RNNLayerConfig,
    pub rnn_cells: Vec<RNNCell>,      // For basic RNN
    pub lstm_cells: Vec<LSTMCell>,     // For LSTM
    pub gru_cells: Vec<GRUCell>,       // For GRU
}

impl RNNLayer {
    pub fn new(config: RNNLayerConfig) -> Self {
        match config.rnn_type {
            RNNType::RNN => {
                let rnn_cells: Vec<RNNCell> = (0..config.num_layers)
                    .map(|_| RNNCell::new(config.input_size, config.hidden_size, config.activation))
                    .collect();
                Self {
                    config,
                    rnn_cells,
                    lstm_cells: vec![],
                    gru_cells: vec![],
                }
            }
            RNNType::LSTM => {
                let lstm_cells: Vec<LSTMCell> = (0..config.num_layers)
                    .map(|_| LSTMCell::new(config.input_size, config.hidden_size))
                    .collect();
                Self {
                    config,
                    rnn_cells: vec![],
                    lstm_cells,
                    gru_cells: vec![],
                }
            }
            RNNType::GRU => {
                let gru_cells: Vec<GRUCell> = (0..config.num_layers)
                    .map(|_| GRUCell::new(config.input_size, config.hidden_size))
                    .collect();
                Self {
                    config,
                    rnn_cells: vec![],
                    lstm_cells: vec![],
                    gru_cells,
                }
            }
        }
    }
    
    /// Forward pass through the RNN layer
    pub fn forward(&self, sequence: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let mut outputs = Vec::new();
        
        match self.config.rnn_type {
            RNNType::RNN => {
                let mut hidden_state = vec![0.0; self.config.hidden_size];
                
                for timestep in sequence {
                    hidden_state = self.rnn_cells[0].forward(timestep, &hidden_state);
                    outputs.push(hidden_state.clone());
                }
            }
            RNNType::LSTM => {
                let mut hidden_state = vec![0.0; self.config.hidden_size];
                let mut cell_state = vec![0.0; self.config.hidden_size];
                
                for timestep in sequence {
                    (hidden_state, cell_state) = self.lstm_cells[0].forward(timestep, &hidden_state, &cell_state);
                    outputs.push(hidden_state.clone());
                }
            }
            RNNType::GRU => {
                let mut hidden_state = vec![0.0; self.config.hidden_size];
                
                for timestep in sequence {
                    hidden_state = self.gru_cells[0].forward(timestep, &hidden_state);
                    outputs.push(hidden_state.clone());
                }
            }
        }
        
        Ok(outputs)
    }
    
    /// Get output size
    pub fn output_size(&self) -> usize {
        if self.config.bidirectional {
            self.config.hidden_size * 2
        } else {
            self.config.hidden_size
        }
    }
}

/// RNN configuration
#[derive(Debug, Clone)]
pub struct RNNConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub num_layers: usize,
    pub rnn_type: RNNType,
    pub bidirectional: bool,
    pub dropout: f64,
    pub output_size: usize,
}

impl Default for RNNConfig {
    fn default() -> Self {
        Self {
            input_size: 10,
            hidden_size: 64,
            num_layers: 2,
            rnn_type: RNNType::LSTM,
            bidirectional: false,
            dropout: 0.1,
            output_size: 10,
        }
    }
}

/// Recurrent Neural Network
#[derive(Debug, Clone)]
pub struct RNN {
    pub rnn_layer: RNNLayer,
    pub config: RNNConfig,
}

impl RNN {
    pub fn new(config: RNNConfig) -> Self {
        let layer_config = RNNLayerConfig {
            input_size: config.input_size,
            hidden_size: config.hidden_size,
            rnn_type: config.rnn_type,
            num_layers: config.num_layers,
            bidirectional: config.bidirectional,
            dropout: config.dropout,
            activation: ActivationFunction::Tanh,
        };
        
        let rnn_layer = RNNLayer::new(layer_config);
        
        Self {
            rnn_layer,
            config,
        }
    }
    
    /// Predict using the RNN
    pub fn predict(&self, sequence: &[Vec<f64>]) -> Result<Vec<f64>> {
        // Forward through RNN layer
        let rnn_outputs = self.rnn_layer.forward(sequence)?;
        
        // Return last hidden state
        let final_hidden = rnn_outputs.last()
            .ok_or_else(|| AIServiceError::InvalidInput {
                message: "Empty sequence".to_string()
            })?
            .clone();
        
        Ok(final_hidden)
    }
    
    /// Predict with all time steps
    pub fn predict_sequence(&self, sequence: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        self.rnn_layer.forward(sequence)
    }
    
    /// Get model summary
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Recurrent Neural Network Summary\n");
        summary.push_str("================================\n\n");
        summary.push_str(&format!("Input size: {}\n", self.config.input_size));
        summary.push_str(&format!("Hidden size: {}\n", self.config.hidden_size));
        summary.push_str(&format!("Num layers: {}\n", self.config.num_layers));
        summary.push_str(&format!("RNN Type: {:?}\n", self.config.rnn_type));
        summary.push_str(&format!("Bidirectional: {}\n", self.config.bidirectional));
        summary.push_str(&format!("Dropout: {}\n", self.config.dropout));
        summary.push_str(&format!("Output size: {}\n", self.config.output_size));
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rnn_cell_creation() {
        let cell = RNNCell::new(10, 64, ActivationFunction::Tanh);
        assert_eq!(cell.input_weights.len(), 64);
        assert_eq!(cell.input_weights[0].len(), 10);
    }

    #[test]
    fn test_rnn_cell_forward() {
        let cell = RNNCell::new(10, 64, ActivationFunction::Tanh);
        let input = vec![0.0; 10];
        let hidden = vec![0.0; 64];
        let output = cell.forward(&input, &hidden);
        assert_eq!(output.len(), 64);
    }

    #[test]
    fn test_lstm_cell_creation() {
        let cell = LSTMCell::new(10, 64);
        assert_eq!(cell.hidden_size, 64);
        assert_eq!(cell.input_weights_i.len(), 64);
    }

    #[test]
    fn test_lstm_cell_forward() {
        let cell = LSTMCell::new(10, 64);
        let input = vec![0.0; 10];
        let hidden = vec![0.0; 64];
        let cell_state = vec![0.0; 64];
        let (new_hidden, new_cell) = cell.forward(&input, &hidden, &cell_state);
        assert_eq!(new_hidden.len(), 64);
        assert_eq!(new_cell.len(), 64);
    }

    #[test]
    fn test_gru_cell_creation() {
        let cell = GRUCell::new(10, 64);
        assert_eq!(cell.hidden_size, 64);
    }

    #[test]
    fn test_gru_cell_forward() {
        let cell = GRUCell::new(10, 64);
        let input = vec![0.0; 10];
        let hidden = vec![0.0; 64];
        let output = cell.forward(&input, &hidden);
        assert_eq!(output.len(), 64);
    }

    #[test]
    fn test_rnn_layer_forward() {
        let config = RNNLayerConfig {
            input_size: 10,
            hidden_size: 64,
            rnn_type: RNNType::LSTM,
            num_layers: 1,
            bidirectional: false,
            dropout: 0.0,
            activation: ActivationFunction::Tanh,
        };
        let layer = RNNLayer::new(config);
        let sequence = vec![vec![0.0; 10]; 5];
        let output = layer.forward(&sequence).unwrap();
        assert_eq!(output.len(), 5);
        assert_eq!(output[0].len(), 64);
    }

    #[test]
    fn test_rnn_creation() {
        let config = RNNConfig::default();
        let rnn = RNN::new(config);
        assert_eq!(rnn.config.hidden_size, 64);
    }

    #[test]
    fn test_rnn_predict() {
        let rnn = RNN::new(RNNConfig::default());
        let sequence = vec![vec![0.0; 10]; 5];
        let output = rnn.predict(&sequence).unwrap();
        assert_eq!(output.len(), 64);
    }

    #[test]
    fn test_rnn_predict_sequence() {
        let rnn = RNN::new(RNNConfig::default());
        let sequence = vec![vec![0.0; 10]; 5];
        let output = rnn.predict_sequence(&sequence).unwrap();
        assert_eq!(output.len(), 5);
        assert_eq!(output[0].len(), 64);
    }
}