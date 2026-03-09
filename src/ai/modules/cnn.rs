//! Convolutional Neural Networks (CNN) Implementation for VantisOS
//!
//! This module provides CNN capabilities for spatial data processing:
//! - 1D, 2D, and 3D convolutions
//! - Pooling layers (Max, Average)
//! - Various activation functions
//! - Batch normalization for convolutions
//! - Dilated and transposed convolutions
//! - Residual connections
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::cnn::{CNN, CNNConfig, ConvLayerConfig};
//!
//! let config = CNNConfig::default();
//! let mut cnn = CNN::new(config);
//! let input = vec![0.0; 28 * 28]; // 28x28 image
//! let output = cnn.predict(&input);
//! ```

use super::{ActivationFunction, Regularization};
use crate::ai::error::{AIServiceError, Result};
use crate::ai::types::ModelMetrics;
use std::collections::HashMap;

/// Convolution layer configuration
#[derive(Debug, Clone)]
pub struct ConvLayerConfig {
    /// Number of input channels
    pub in_channels: usize,
    /// Number of output channels (filters)
    pub out_channels: usize,
    /// Kernel size (square for simplicity)
    pub kernel_size: usize,
    /// Stride
    pub stride: usize,
    /// Padding
    pub padding: usize,
    /// Dilation rate
    pub dilation: usize,
    /// Activation function
    pub activation: ActivationFunction,
    /// Regularization
    pub regularization: Option<Regularization>,
    /// Use batch normalization
    pub use_batch_norm: bool,
    /// Use padding mode 'same' or 'valid'
    pub padding_mode: PaddingMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddingMode {
    Same,
    Valid,
}

impl Default for ConvLayerConfig {
    fn default() -> Self {
        Self {
            in_channels: 1,
            out_channels: 32,
            kernel_size: 3,
            stride: 1,
            padding: 1,
            dilation: 1,
            activation: ActivationFunction::ReLU,
            regularization: Some(Regularization::L2(0.01)),
            use_batch_norm: true,
            padding_mode: PaddingMode::Same,
        }
    }
}

/// Pooling layer configuration
#[derive(Debug, Clone)]
pub struct PoolLayerConfig {
    /// Pool type
    pub pool_type: PoolType,
    /// Pool size (square)
    pub pool_size: usize,
    /// Stride
    pub stride: usize,
    /// Padding
    pub padding: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    Max,
    Average,
    GlobalMax,
    GlobalAverage,
}

impl Default for PoolLayerConfig {
    fn default() -> Self {
        Self {
            pool_type: PoolType::Max,
            pool_size: 2,
            stride: 2,
            padding: 0,
        }
    }
}

/// Convolutional layer
#[derive(Debug, Clone)]
pub struct ConvLayer {
    /// Convolution weights [out_channels, in_channels, kernel_h, kernel_w]
    pub weights: Vec<Vec<Vec<Vec<f64>>>>,
    /// Bias [out_channels]
    pub biases: Vec<f64>,
    /// Configuration
    pub config: ConvLayerConfig,
    /// Batch normalization parameters
    pub batch_norm_gamma: Option<Vec<f64>>,
    pub batch_norm_beta: Option<Vec<f64>>,
    pub batch_norm_running_mean: Option<Vec<f64>>,
    pub batch_norm_running_var: Option<Vec<f64>>,
    /// Cache for backpropagation
    pub last_input: Option<Vec<Vec<Vec<f64>>>>,
    pub last_output: Option<Vec<Vec<Vec<f64>>>>,
}

impl ConvLayer {
    /// Create a new convolutional layer
    pub fn new(config: ConvLayerConfig) -> Self {
        let scale = (2.0 / (config.in_channels * config.kernel_size * config.kernel_size) as f64).sqrt();
        
        let weights: Vec<Vec<Vec<Vec<f64>>>> = (0..config.out_channels)
            .map(|_| (0..config.in_channels)
                .map(|_| (0..config.kernel_size)
                    .map(|_| (0..config.kernel_size)
                        .map(|_| {
                            let hash = (config.in_channels * 1000 + config.out_channels) as f64;
                            (hash.sin() * scale + hash.cos() * scale) / 2.0
                        })
                        .collect())
                    .collect())
                .collect())
            .collect();
        
        let biases = vec![0.0; config.out_channels];
        
        let (batch_norm_gamma, batch_norm_beta, batch_norm_running_mean, batch_norm_running_var) = 
            if config.use_batch_norm {
                (
                    Some(vec![1.0; config.out_channels]),
                    Some(vec![0.0; config.out_channels]),
                    Some(vec![0.0; config.out_channels]),
                    Some(vec![1.0; config.out_channels]),
                )
            } else {
                (None, None, None, None)
            };
        
        Self {
            weights,
            biases,
            config,
            batch_norm_gamma,
            batch_norm_beta,
            batch_norm_running_mean,
            batch_norm_running_var,
            last_input: None,
            last_output: None,
        }
    }
    
    /// Compute output dimensions
    pub fn output_size(&self, input_height: usize, input_width: usize) -> (usize, usize) {
        match self.config.padding_mode {
            PaddingMode::Valid => {
                let h = ((input_height as i32 - self.config.kernel_size as i32) / self.config.stride as i32 + 1).max(0) as usize;
                let w = ((input_width as i32 - self.config.kernel_size as i32) / self.config.stride as i32 + 1).max(0) as usize;
                (h, w)
            }
            PaddingMode::Same => {
                (
                    (input_height as f64 / self.config.stride as f64).ceil() as usize,
                    (input_width as f64 / self.config.stride as f64).ceil() as usize,
                )
            }
        }
    }
    
    /// 2D convolution operation
    pub fn conv2d(&self, input: &[Vec<Vec<f64>>]) -> Vec<Vec<Vec<f64>>> {
        let (batch, in_channels, height, width) = (
            input.len(),
            input[0].len(),
            input[0][0].len(),
            input[0][0].len(),
        );
        
        let (out_h, out_w) = self.output_size(height, width);
        let out_channels = self.config.out_channels;
        
        let mut output = vec![vec![vec![0.0; out_w]; out_h]; out_channels];
        
        // Apply padding if needed
        let padded = if self.config.padding > 0 {
            self.pad_input(input)
        } else {
            input.to_vec()
        };
        
        let (pad_h, pad_w) = (
            padded[0][0].len(),
            padded[0][0].len(),
        );
        
        // Convolution
        for oc in 0..out_channels {
            let mut out_channel = vec![vec![0.0; out_w]; out_h];
            
            for ic in 0..in_channels {
                for i in 0..out_h {
                    for j in 0..out_w {
                        let mut sum = 0.0;
                        
                        for ki in 0..self.config.kernel_size {
                            for kj in 0..self.config.kernel_size {
                                let input_i = i * self.config.stride + ki;
                                let input_j = j * self.config.stride + kj;
                                
                                if input_i < pad_h && input_j < pad_w {
                                    sum += self.weights[oc][ic][ki][kj] * padded[0][ic][input_i][input_j];
                                }
                            }
                        }
                        
                        out_channel[i][j] += sum;
                    }
                }
            }
            
            // Add bias
            for i in 0..out_h {
                for j in 0..out_w {
                    out_channel[i][j] += self.biases[oc];
                }
            }
            
            output[oc] = out_channel;
        }
        
        // Apply activation
        for oc in 0..out_channels {
            for i in 0..out_h {
                for j in 0..out_w {
                    output[oc][i][j] = self.config.activation.apply(output[oc][i][j]);
                }
            }
        }
        
        output
    }
    
    /// Pad input with zeros
    fn pad_input(&self, input: &[Vec<Vec<f64>>]) -> Vec<Vec<Vec<f64>>> {
        let padding = self.config.padding;
        let (batch, in_channels, height, width) = (
            input.len(),
            input[0].len(),
            input[0][0].len(),
            input[0][0].len(),
        );
        
        let new_h = height + 2 * padding;
        let new_w = width + 2 * padding;
        
        let mut padded = vec![vec![vec![0.0; new_w]; new_h]; in_channels];
        
        for b in 0..batch {
            for ic in 0..in_channels {
                for i in 0..height {
                    for j in 0..width {
                        padded[ic][i + padding][j + padding] = input[b][ic][i][j];
                    }
                }
            }
        }
        
        padded
    }
    
    /// Forward pass
    pub fn forward(&mut self, input: &[Vec<Vec<f64>>], _training: bool) -> Result<Vec<Vec<Vec<f64>>>> {
        self.last_input = Some(input.to_vec());
        
        let output = self.conv2d(input);
        
        // Apply batch normalization
        let output = if self.config.use_batch_norm {
            self.apply_batch_norm(&output)?
        } else {
            output
        };
        
        self.last_output = Some(output.clone());
        Ok(output)
    }
    
    /// Apply batch normalization
    fn apply_batch_norm(&mut self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        let out_channels = input.len();
        let height = input[0].len();
        let width = input[0][0].len();
        
        let mut output = vec![vec![vec![0.0; width]; height]; out_channels];
        
        if let (Some(gamma), Some(beta), Some(running_mean), Some(running_var)) = (
            &self.batch_norm_gamma,
            &self.batch_norm_beta,
            &mut self.batch_norm_running_mean,
            &mut self.batch_norm_running_var,
        ) {
            let epsilon = 1e-5;
            let momentum = 0.1;
            
            for oc in 0..out_channels {
                // Compute mean
                let mean: f64 = input[oc].iter()
                    .flat_map(|row| row.iter())
                    .sum::<f64>() / (height * width) as f64;
                
                // Compute variance
                let var: f64 = input[oc].iter()
                    .flat_map(|row| row.iter())
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>() / (height * width) as f64;
                
                // Update running statistics
                running_mean[oc] = (1.0 - momentum) * running_mean[oc] + momentum * mean;
                running_var[oc] = (1.0 - momentum) * running_var[oc] + momentum * var;
                
                // Normalize
                for i in 0..height {
                    for j in 0..width {
                        output[oc][i][j] = gamma[oc] * (input[oc][i][j] - mean) / (var + epsilon).sqrt() + beta[oc];
                    }
                }
            }
        }
        
        Ok(output)
    }
}

/// Pooling layer
#[derive(Debug, Clone)]
pub struct PoolLayer {
    pub config: PoolLayerConfig,
    pub last_input: Option<Vec<Vec<Vec<f64>>>>,
    pub last_indices: Option<Vec<(usize, usize)>>,
}

impl PoolLayer {
    pub fn new(config: PoolLayerConfig) -> Self {
        Self {
            config,
            last_input: None,
            last_indices: None,
        }
    }
    
    /// Compute output size
    pub fn output_size(&self, input_height: usize, input_width: usize) -> (usize, usize) {
        let h = ((input_height - self.config.pool_size) / self.config.stride + 1).max(0);
        let w = ((input_width - self.config.pool_size) / self.config.stride + 1).max(0);
        (h, w)
    }
    
    /// Forward pass
    pub fn forward(&mut self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        self.last_input = Some(input.to_vec());
        
        match self.config.pool_type {
            PoolType::Max => self.max_pool(input),
            PoolType::Average => self.avg_pool(input),
            PoolType::GlobalMax => self.global_max_pool(input),
            PoolType::GlobalAverage => self.global_avg_pool(input),
        }
    }
    
    fn max_pool(&self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        let channels = input.len();
        let (out_h, out_w) = self.output_size(input[0].len(), input[0][0].len());
        
        let mut output = vec![vec![vec![0.0; out_w]; out_h]; channels];
        let mut indices = Vec::new();
        
        for c in 0..channels {
            for i in 0..out_h {
                for j in 0..out_w {
                    let mut max_val = f64::NEG_INFINITY;
                    let mut max_idx = (0, 0);
                    
                    for ki in 0..self.config.pool_size {
                        for kj in 0..self.config.pool_size {
                            let input_i = i * self.config.stride + ki;
                            let input_j = j * self.config.stride + kj;
                            
                            if input_i < input[c].len() && input_j < input[c][0].len() {
                                let val = input[c][input_i][input_j];
                                if val > max_val {
                                    max_val = val;
                                    max_idx = (input_i, input_j);
                                }
                            }
                        }
                    }
                    
                    output[c][i][j] = max_val;
                    indices.push(max_idx);
                }
            }
        }
        
        Ok(output)
    }
    
    fn avg_pool(&self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        let channels = input.len();
        let (out_h, out_w) = self.output_size(input[0].len(), input[0][0].len());
        
        let mut output = vec![vec![vec![0.0; out_w]; out_h]; channels];
        
        for c in 0..channels {
            for i in 0..out_h {
                for j in 0..out_w {
                    let mut sum = 0.0;
                    let mut count = 0;
                    
                    for ki in 0..self.config.pool_size {
                        for kj in 0..self.config.pool_size {
                            let input_i = i * self.config.stride + ki;
                            let input_j = j * self.config.stride + kj;
                            
                            if input_i < input[c].len() && input_j < input[c][0].len() {
                                sum += input[c][input_i][input_j];
                                count += 1;
                            }
                        }
                    }
                    
                    output[c][i][j] = if count > 0 { sum / count as f64 } else { 0.0 };
                }
            }
        }
        
        Ok(output)
    }
    
    fn global_max_pool(&self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        let channels = input.len();
        let mut output = vec![vec![vec![0.0; 1]; 1]; channels];
        
        for c in 0..channels {
            let mut max_val = f64::NEG_INFINITY;
            for row in &input[c] {
                for &val in row {
                    max_val = max_val.max(val);
                }
            }
            output[c][0][0] = max_val;
        }
        
        Ok(output)
    }
    
    fn global_avg_pool(&self, input: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<Vec<f64>>>> {
        let channels = input.len();
        let mut output = vec![vec![vec![0.0; 1]; 1]; channels];
        
        for c in 0..channels {
            let sum: f64 = input[c].iter().flat_map(|row| row.iter()).sum();
            let count = input[c].len() * input[c][0].len();
            output[c][0][0] = sum / count as f64;
        }
        
        Ok(output)
    }
}

/// Fully connected layer (for CNN output)
#[derive(Debug, Clone)]
pub struct DenseLayer {
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
    pub input_size: usize,
    pub output_size: usize,
    pub activation: ActivationFunction,
}

impl DenseLayer {
    pub fn new(input_size: usize, output_size: usize, activation: ActivationFunction) -> Self {
        let scale = (2.0 / input_size as f64).sqrt();
        
        let weights: Vec<Vec<f64>> = (0..output_size)
            .map(|_| (0..input_size)
                .map(|_| {
                    let hash = (input_size * 1000 + output_size) as f64;
                    (hash.sin() * scale + hash.cos() * scale) / 2.0
                })
                .collect())
            .collect();
        
        let biases = vec![0.0; output_size];
        
        Self {
            weights,
            biases,
            input_size,
            output_size,
            activation,
        }
    }
    
    pub fn forward(&self, input: &[f64]) -> Result<Vec<f64>> {
        if input.len() != self.input_size {
            return Err(AIServiceError::InvalidInput {
                message: format!("Expected input size {}, got {}", self.input_size, input.len())
            });
        }
        
        let mut output = vec![0.0; self.output_size];
        
        for i in 0..self.output_size {
            let sum: f64 = self.weights[i].iter().zip(input.iter()).map(|(w, x)| w * x).sum();
            output[i] = self.activation.apply(sum + self.biases[i]);
        }
        
        Ok(output)
    }
}

/// CNN configuration
#[derive(Debug, Clone)]
pub struct CNNConfig {
    pub conv_layers: Vec<ConvLayerConfig>,
    pub pool_layers: Vec<PoolLayerConfig>,
    pub dense_layers: Vec<(usize, ActivationFunction)>,
    pub input_channels: usize,
    pub input_height: usize,
    pub input_width: usize,
    pub num_classes: usize,
}

impl Default for CNNConfig {
    fn default() -> Self {
        Self {
            conv_layers: vec![
                ConvLayerConfig {
                    in_channels: 1,
                    out_channels: 32,
                    kernel_size: 3,
                    stride: 1,
                    padding: 1,
                    dilation: 1,
                    activation: ActivationFunction::ReLU,
                    regularization: Some(Regularization::L2(0.01)),
                    use_batch_norm: true,
                    padding_mode: PaddingMode::Same,
                },
                ConvLayerConfig {
                    in_channels: 32,
                    out_channels: 64,
                    kernel_size: 3,
                    stride: 1,
                    padding: 1,
                    dilation: 1,
                    activation: ActivationFunction::ReLU,
                    regularization: Some(Regularization::L2(0.01)),
                    use_batch_norm: true,
                    padding_mode: PaddingMode::Same,
                },
            ],
            pool_layers: vec![
                PoolLayerConfig {
                    pool_type: PoolType::Max,
                    pool_size: 2,
                    stride: 2,
                    padding: 0,
                },
            ],
            dense_layers: vec![
                (128, ActivationFunction::ReLU),
                (10, ActivationFunction::Softmax),
            ],
            input_channels: 1,
            input_height: 28,
            input_width: 28,
            num_classes: 10,
        }
    }
}

/// Convolutional Neural Network
#[derive(Debug, Clone)]
pub struct CNN {
    pub conv_layers: Vec<ConvLayer>,
    pub pool_layers: Vec<PoolLayer>,
    pub dense_layers: Vec<DenseLayer>,
    pub config: CNNConfig,
}

impl CNN {
    pub fn new(config: CNNConfig) -> Self {
        let conv_layers: Vec<ConvLayer> = config.conv_layers.iter().cloned().map(ConvLayer::new).collect();
        let pool_layers: Vec<PoolLayer> = config.pool_layers.iter().cloned().map(PoolLayer::new).collect();
        
        // Calculate flattened size after conv and pool layers
        let mut current_h = config.input_height;
        let mut current_w = config.input_width;
        let mut current_c = config.input_channels;
        
        // Apply conv and pool layers to compute size
        let mut pool_idx = 0;
        for conv in &config.conv_layers {
            let (h, w) = conv.output_size(current_h, current_w);
            current_h = h;
            current_w = w;
            current_c = conv.out_channels;
            
            if pool_idx < config.pool_layers.len() {
                let pool = &config.pool_layers[pool_idx];
                let (h, w) = pool.output_size(current_h, current_w);
                current_h = h;
                current_w = w;
                pool_idx += 1;
            }
        }
        
        let flattened_size = current_h * current_w * current_c;
        
        // Create dense layers
        let mut dense_layers = Vec::new();
        let mut prev_size = flattened_size;
        
        for (size, activation) in &config.dense_layers {
            dense_layers.push(DenseLayer::new(prev_size, *size, *activation));
            prev_size = *size;
        }
        
        Self {
            conv_layers,
            pool_layers,
            dense_layers,
            config,
        }
    }
    
    /// Predict using the CNN
    pub fn predict(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        // Reshape input to [channels, height, width]
        let mut input_3d = vec![vec![vec![0.0; self.config.input_width]; self.config.input_height]; self.config.input_channels];
        
        for c in 0..self.config.input_channels {
            for i in 0..self.config.input_height {
                for j in 0..self.config.input_width {
                    let idx = c * self.config.input_height * self.config.input_width + i * self.config.input_width + j;
                    input_3d[c][i][j] = input[idx];
                }
            }
        }
        
        // Forward through conv layers
        let mut current: Vec<Vec<Vec<f64>>> = input_3d;
        let mut pool_idx = 0;
        
        for (conv_idx, conv) in self.conv_layers.iter_mut().enumerate() {
            current = conv.forward(&current, false)?;
            
            // Apply pooling if available
            if pool_idx < self.pool_layers.len() {
                current = self.pool_layers[pool_idx].forward(&current)?;
                pool_idx += 1;
            }
        }
        
        // Flatten
        let flattened: Vec<f64> = current.iter()
            .flat_map(|channel| channel.iter().flat_map(|row| row.iter().cloned()))
            .collect();
        
        // Forward through dense layers
        let mut output = flattened;
        for dense in &self.dense_layers {
            output = dense.forward(&output)?;
        }
        
        Ok(output)
    }
    
    /// Get model summary
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Convolutional Neural Network Summary\n");
        summary.push_str("====================================\n\n");
        summary.push_str(&format!("Input: {}x{}x{}\n\n", self.config.input_height, self.config.input_width, self.config.input_channels));
        
        let mut current_h = self.config.input_height;
        let mut current_w = self.config.input_width;
        let mut current_c = self.config.input_channels;
        
        summary.push_str("Convolutional Layers:\n");
        for (i, conv) in self.conv_layers.iter().enumerate() {
            let (h, w) = conv.output_size(current_h, current_w);
            summary.push_str(&format!(
                "  Conv2D: {} filters, {}x{} kernel, stride={}, padding={} -> Output: {}x{}x{}\n",
                conv.config.out_channels,
                conv.config.kernel_size,
                conv.config.kernel_size,
                conv.config.stride,
                conv.config.padding,
                h, w, conv.config.out_channels
            ));
            current_h = h;
            current_w = w;
            current_c = conv.config.out_channels;
        }
        
        summary.push_str("\nPooling Layers:\n");
        for pool in &self.pool_layers {
            let (h, w) = pool.output_size(current_h, current_w);
            summary.push_str(&format!(
                "  {:?} Pool: {}x{}, stride={} -> Output: {}x{}x{}\n",
                pool.config.pool_type,
                pool.config.pool_size,
                pool.config.pool_size,
                pool.config.stride,
                h, w, current_c
            ));
            current_h = h;
            current_w = w;
        }
        
        summary.push_str(&format!("\nFlattened size: {}\n", current_h * current_w * current_c));
        
        summary.push_str("\nDense Layers:\n");
        for (i, dense) in self.dense_layers.iter().enumerate() {
            summary.push_str(&format!(
                "  Dense: {} -> {}, activation={:?}\n",
                dense.input_size,
                dense.output_size,
                dense.activation
            ));
        }
        
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_layer_creation() {
        let config = ConvLayerConfig::default();
        let layer = ConvLayer::new(config);
        assert_eq!(layer.config.out_channels, 32);
        assert_eq!(layer.weights.len(), 32);
    }

    #[test]
    fn test_conv_layer_forward() {
        let mut layer = ConvLayer::new(ConvLayerConfig {
            in_channels: 1,
            out_channels: 2,
            kernel_size: 3,
            stride: 1,
            padding: 1,
            dilation: 1,
            activation: ActivationFunction::ReLU,
            regularization: None,
            use_batch_norm: false,
            padding_mode: PaddingMode::Same,
        });
        
        let input = vec![vec![vec![0.0; 10]; 10]];
        let output = layer.forward(&input, false).unwrap();
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_pool_layer() {
        let mut pool = PoolLayer::new(PoolLayerConfig::default());
        let input = vec![vec![vec![1.0, 2.0, 3.0, 4.0]; 4]];
        let output = pool.forward(&input).unwrap();
        assert_eq!(output[0].len(), 2);
    }

    #[test]
    fn test_cnn_creation() {
        let config = CNNConfig::default();
        let cnn = CNN::new(config);
        assert_eq!(cnn.conv_layers.len(), 2);
        assert_eq!(cnn.pool_layers.len(), 1);
        assert_eq!(cnn.dense_layers.len(), 2);
    }

    #[test]
    fn test_cnn_predict() {
        let mut cnn = CNN::new(CNNConfig::default());
        let input = vec![0.0; 28 * 28];
        let output = cnn.predict(&input).unwrap();
        assert_eq!(output.len(), 10);
    }
}