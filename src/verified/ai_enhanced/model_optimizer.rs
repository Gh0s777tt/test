//! Model Optimization Pipeline for VANTIS OS
//!
//! Provides model compression, quantization, and pruning capabilities
//! to optimize neural network models for efficient kernel-level inference.
//! Supports Int8/Int4/Float16 quantization and magnitude-based pruning.

use core::fmt;

// ============================================================================
// Quantization Types
// ============================================================================

/// Quantization methods for model compression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantizationMethod {
    /// 8-bit integer quantization
    Int8,
    /// 4-bit integer quantization (aggressive)
    Int4,
    /// 16-bit floating point
    Float16,
    /// Dynamic quantization (per-tensor scaling)
    DynamicInt8,
    /// No quantization
    None,
}

/// Pruning strategies for weight removal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PruningStrategy {
    /// Remove weights below magnitude threshold
    MagnitudeBased,
    /// Remove entire channels/filters
    Structured,
    /// Gradual magnitude pruning over iterations
    GradualMagnitude,
    /// Random unstructured pruning
    Random,
    /// No pruning
    None,
}

/// Optimization pass types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationPass {
    /// Fuse consecutive operations (e.g., Conv+BN+ReLU)
    OperatorFusion,
    /// Eliminate dead/unused operations
    DeadCodeElimination,
    /// Fold constant expressions
    ConstantFolding,
    /// Optimize memory layout for cache efficiency
    MemoryLayoutOptimization,
    /// Reduce precision of computations
    Quantization,
    /// Remove redundant weights
    Pruning,
}

impl fmt::Display for QuantizationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantizationMethod::Int8 => write!(f, "INT8"),
            QuantizationMethod::Int4 => write!(f, "INT4"),
            QuantizationMethod::Float16 => write!(f, "FP16"),
            QuantizationMethod::DynamicInt8 => write!(f, "DynINT8"),
            QuantizationMethod::None => write!(f, "None"),
        }
    }
}

// ============================================================================
// Weight Tensor
// ============================================================================

/// A weight tensor with statistics for optimization decisions
#[derive(Debug, Clone)]
pub struct WeightTensor {
    pub name: String,
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
    pub is_quantized: bool,
    pub is_pruned: bool,
    pub sparsity: f64,
}

impl WeightTensor {
    pub fn new(name: &str, data: Vec<f32>, shape: Vec<usize>) -> Self {
        Self {
            name: name.to_string(),
            data,
            shape,
            is_quantized: false,
            is_pruned: false,
            sparsity: 0.0,
        }
    }

    /// Total number of elements
    pub fn num_elements(&self) -> usize {
        self.data.len()
    }

    /// Number of non-zero elements
    pub fn num_nonzero(&self) -> usize {
        self.data.iter().filter(|&&x| x.abs() > 1e-10).count()
    }

    /// Compute current sparsity ratio
    pub fn compute_sparsity(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        let zeros = self.data.iter().filter(|&&x| x.abs() <= 1e-10).count();
        zeros as f64 / self.data.len() as f64
    }

    /// Mean absolute value of weights
    pub fn mean_abs(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.data.iter().map(|&x| x.abs() as f64).sum();
        sum / self.data.len() as f64
    }

    /// Maximum absolute value
    pub fn max_abs(&self) -> f32 {
        self.data.iter().map(|x| x.abs()).fold(0.0f32, f32::max)
    }

    /// Minimum absolute value (non-zero)
    pub fn min_abs_nonzero(&self) -> f32 {
        self.data.iter()
            .map(|x| x.abs())
            .filter(|&x| x > 1e-10)
            .fold(f32::MAX, f32::min)
    }

    /// Standard deviation of weights
    pub fn std_dev(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }
        let mean = self.mean_abs();
        let variance: f64 = self.data.iter()
            .map(|&x| {
                let diff = x.abs() as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / (self.data.len() - 1) as f64;
        variance.sqrt()
    }

    /// Size in bytes at given precision
    pub fn size_bytes(&self, method: QuantizationMethod) -> usize {
        let bits_per_element = match method {
            QuantizationMethod::Int8 | QuantizationMethod::DynamicInt8 => 8,
            QuantizationMethod::Int4 => 4,
            QuantizationMethod::Float16 => 16,
            QuantizationMethod::None => 32,
        };
        (self.data.len() * bits_per_element + 7) / 8
    }
}

// ============================================================================
// Optimization Configuration
// ============================================================================

/// Configuration for the optimization pipeline
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub quantization: QuantizationMethod,
    pub pruning: PruningStrategy,
    pub pruning_ratio: f64,
    pub passes: Vec<OptimizationPass>,
    pub calibration_samples: usize,
    pub preserve_accuracy_threshold: f64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            quantization: QuantizationMethod::Int8,
            pruning: PruningStrategy::MagnitudeBased,
            pruning_ratio: 0.3,
            passes: vec![
                OptimizationPass::ConstantFolding,
                OptimizationPass::DeadCodeElimination,
                OptimizationPass::OperatorFusion,
                OptimizationPass::Pruning,
                OptimizationPass::Quantization,
            ],
            calibration_samples: 100,
            preserve_accuracy_threshold: 0.99,
        }
    }
}

// ============================================================================
// Optimization Results
// ============================================================================

/// Results from the optimization pipeline
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub original_size_bytes: usize,
    pub optimized_size_bytes: usize,
    pub compression_ratio: f64,
    pub layers_pruned: usize,
    pub layers_quantized: usize,
    pub total_layers: usize,
    pub average_sparsity: f64,
    pub passes_applied: Vec<OptimizationPass>,
    pub estimated_speedup: f64,
}

impl OptimizationResult {
    /// Size reduction as a percentage
    pub fn size_reduction_pct(&self) -> f64 {
        if self.original_size_bytes == 0 {
            return 0.0;
        }
        (1.0 - self.optimized_size_bytes as f64 / self.original_size_bytes as f64) * 100.0
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizerError {
    /// No weights loaded
    NoWeightsLoaded,
    /// Invalid pruning ratio
    InvalidPruningRatio(f64),
    /// Quantization failed for a layer
    QuantizationFailed(String),
    /// Accuracy degradation too high
    AccuracyDegradation { threshold: f64, actual: f64 },
    /// Empty model
    EmptyModel,
}

impl fmt::Display for OptimizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizerError::NoWeightsLoaded => write!(f, "No weights loaded"),
            OptimizerError::InvalidPruningRatio(r) => {
                write!(f, "Invalid pruning ratio: {} (must be 0.0-1.0)", r)
            }
            OptimizerError::QuantizationFailed(layer) => {
                write!(f, "Quantization failed for layer: {}", layer)
            }
            OptimizerError::AccuracyDegradation { threshold, actual } => {
                write!(f, "Accuracy degradation: {:.4} < threshold {:.4}", actual, threshold)
            }
            OptimizerError::EmptyModel => write!(f, "Empty model"),
        }
    }
}

// ============================================================================
// Model Optimizer
// ============================================================================

/// The main model optimization engine
pub struct ModelOptimizer {
    weights: Vec<WeightTensor>,
    config: OptimizationConfig,
    optimization_history: Vec<OptimizationResult>,
}

impl ModelOptimizer {
    /// Create a new optimizer with given configuration
    pub fn new(config: OptimizationConfig) -> Self {
        Self {
            weights: Vec::new(),
            config,
            optimization_history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(OptimizationConfig::default())
    }

    /// Add a weight tensor to the model
    pub fn add_weights(&mut self, tensor: WeightTensor) {
        self.weights.push(tensor);
    }

    /// Get number of weight tensors
    pub fn num_layers(&self) -> usize {
        self.weights.len()
    }

    /// Total model size in bytes (at current precision)
    pub fn total_size_bytes(&self) -> usize {
        self.weights.iter()
            .map(|w| w.size_bytes(if w.is_quantized { self.config.quantization } else { QuantizationMethod::None }))
            .sum()
    }

    /// Original model size in bytes (float32)
    pub fn original_size_bytes(&self) -> usize {
        self.weights.iter()
            .map(|w| w.size_bytes(QuantizationMethod::None))
            .sum()
    }

    /// Run the full optimization pipeline
    pub fn optimize(&mut self) -> Result<OptimizationResult, OptimizerError> {
        if self.weights.is_empty() {
            return Err(OptimizerError::EmptyModel);
        }

        if self.config.pruning_ratio < 0.0 || self.config.pruning_ratio > 1.0 {
            return Err(OptimizerError::InvalidPruningRatio(self.config.pruning_ratio));
        }

        let original_size = self.original_size_bytes();
        let mut passes_applied = Vec::new();
        let mut layers_pruned = 0;
        let mut layers_quantized = 0;

        for pass in &self.config.passes.clone() {
            match pass {
                OptimizationPass::Pruning => {
                    layers_pruned = self.apply_pruning();
                    passes_applied.push(*pass);
                }
                OptimizationPass::Quantization => {
                    layers_quantized = self.apply_quantization();
                    passes_applied.push(*pass);
                }
                OptimizationPass::ConstantFolding
                | OptimizationPass::DeadCodeElimination
                | OptimizationPass::OperatorFusion
                | OptimizationPass::MemoryLayoutOptimization => {
                    // These passes are structural and don't modify weights directly
                    passes_applied.push(*pass);
                }
            }
        }

        let optimized_size = self.total_size_bytes();
        let average_sparsity = if self.weights.is_empty() {
            0.0
        } else {
            self.weights.iter().map(|w| w.compute_sparsity()).sum::<f64>() / self.weights.len() as f64
        };

        let compression_ratio = if optimized_size > 0 {
            original_size as f64 / optimized_size as f64
        } else {
            1.0
        };

        // Estimate speedup based on sparsity and quantization
        let sparsity_speedup = 1.0 / (1.0 - average_sparsity * 0.7).max(0.1);
        let quant_speedup = match self.config.quantization {
            QuantizationMethod::Int8 | QuantizationMethod::DynamicInt8 => 2.0,
            QuantizationMethod::Int4 => 3.5,
            QuantizationMethod::Float16 => 1.5,
            QuantizationMethod::None => 1.0,
        };
        let estimated_speedup = sparsity_speedup * quant_speedup;

        let result = OptimizationResult {
            original_size_bytes: original_size,
            optimized_size_bytes: optimized_size,
            compression_ratio,
            layers_pruned,
            layers_quantized,
            total_layers: self.weights.len(),
            average_sparsity,
            passes_applied,
            estimated_speedup,
        };

        self.optimization_history.push(result.clone());
        Ok(result)
    }

    /// Apply pruning to all weight tensors
    fn apply_pruning(&mut self) -> usize {
        let ratio = self.config.pruning_ratio;
        let strategy = self.config.pruning;
        let mut pruned_count = 0;

        for weight in &mut self.weights {
            if weight.is_pruned {
                continue;
            }

            match strategy {
                PruningStrategy::MagnitudeBased => {
                    Self::magnitude_prune(weight, ratio);
                    pruned_count += 1;
                }
                PruningStrategy::Random => {
                    Self::random_prune(weight, ratio);
                    pruned_count += 1;
                }
                PruningStrategy::Structured | PruningStrategy::GradualMagnitude => {
                    // Structured pruning: zero out smallest magnitude elements
                    Self::magnitude_prune(weight, ratio);
                    pruned_count += 1;
                }
                PruningStrategy::None => {}
            }
        }

        pruned_count
    }

    /// Magnitude-based pruning: zero out weights below threshold
    fn magnitude_prune(weight: &mut WeightTensor, ratio: f64) {
        if weight.data.is_empty() || ratio <= 0.0 {
            return;
        }

        // Find the threshold value at the given percentile
        let mut sorted_abs: Vec<f32> = weight.data.iter().map(|x| x.abs()).collect();
        sorted_abs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

        let threshold_idx = ((sorted_abs.len() as f64 * ratio) as usize).min(sorted_abs.len() - 1);
        let threshold = sorted_abs[threshold_idx];

        // Zero out weights below threshold
        for val in &mut weight.data {
            if val.abs() <= threshold {
                *val = 0.0;
            }
        }

        weight.is_pruned = true;
        weight.sparsity = weight.compute_sparsity();
    }

    /// Random pruning: zero out random weights
    fn random_prune(weight: &mut WeightTensor, ratio: f64) {
        if weight.data.is_empty() || ratio <= 0.0 {
            return;
        }

        // Deterministic pseudo-random for reproducibility
        for (i, val) in weight.data.iter_mut().enumerate() {
            let pseudo_random = ((i as f64 * 0.618033988749895) % 1.0) as f64;
            if pseudo_random < ratio {
                *val = 0.0;
            }
        }

        weight.is_pruned = true;
        weight.sparsity = weight.compute_sparsity();
    }

    /// Apply quantization to all weight tensors
    fn apply_quantization(&mut self) -> usize {
        let method = self.config.quantization;
        let mut quantized_count = 0;

        if method == QuantizationMethod::None {
            return 0;
        }

        for weight in &mut self.weights {
            if weight.is_quantized {
                continue;
            }

            match method {
                QuantizationMethod::Int8 | QuantizationMethod::DynamicInt8 => {
                    Self::quantize_int8(weight);
                    quantized_count += 1;
                }
                QuantizationMethod::Int4 => {
                    Self::quantize_int4(weight);
                    quantized_count += 1;
                }
                QuantizationMethod::Float16 => {
                    Self::quantize_fp16(weight);
                    quantized_count += 1;
                }
                QuantizationMethod::None => {}
            }
        }

        quantized_count
    }

    /// Simulate INT8 quantization (scale + zero-point)
    fn quantize_int8(weight: &mut WeightTensor) {
        let max_val = weight.max_abs();
        if max_val < 1e-10 {
            weight.is_quantized = true;
            return;
        }
        let scale = max_val / 127.0;
        for val in &mut weight.data {
            let quantized = (*val / scale).round().clamp(-128.0, 127.0);
            *val = quantized * scale; // dequantize back to f32 representation
        }
        weight.is_quantized = true;
    }

    /// Simulate INT4 quantization
    fn quantize_int4(weight: &mut WeightTensor) {
        let max_val = weight.max_abs();
        if max_val < 1e-10 {
            weight.is_quantized = true;
            return;
        }
        let scale = max_val / 7.0;
        for val in &mut weight.data {
            let quantized = (*val / scale).round().clamp(-8.0, 7.0);
            *val = quantized * scale;
        }
        weight.is_quantized = true;
    }

    /// Simulate FP16 quantization (reduce precision)
    fn quantize_fp16(weight: &mut WeightTensor) {
        for val in &mut weight.data {
            // Simulate FP16 precision loss by rounding to ~3 decimal places
            *val = (*val * 1024.0).round() / 1024.0;
        }
        weight.is_quantized = true;
    }

    /// Get optimization history
    pub fn history(&self) -> &[OptimizationResult] {
        &self.optimization_history
    }

    /// Get a reference to the weight tensors
    pub fn weights(&self) -> &[WeightTensor] {
        &self.weights
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_weights() -> Vec<WeightTensor> {
        vec![
            WeightTensor::new("layer1", vec![0.5, -0.3, 0.8, -0.1, 0.02, -0.7, 0.4, 0.9], vec![2, 4]),
            WeightTensor::new("layer2", vec![0.1, -0.2, 0.3, -0.4], vec![2, 2]),
            WeightTensor::new("layer3", vec![1.0, -0.5, 0.25, -0.125, 0.0625, -0.03125], vec![2, 3]),
        ]
    }

    #[test]
    fn test_weight_tensor_stats() {
        let w = WeightTensor::new("test", vec![1.0, -2.0, 0.0, 3.0, -1.0], vec![5]);
        assert_eq!(w.num_elements(), 5);
        assert_eq!(w.num_nonzero(), 4);
        assert!((w.compute_sparsity() - 0.2).abs() < 1e-5);
        assert!((w.max_abs() - 3.0).abs() < 1e-5);
    }

    #[test]
    fn test_weight_size_bytes() {
        let w = WeightTensor::new("test", vec![0.0; 100], vec![100]);
        assert_eq!(w.size_bytes(QuantizationMethod::None), 400);   // 100 * 32 / 8
        assert_eq!(w.size_bytes(QuantizationMethod::Int8), 100);   // 100 * 8 / 8
        assert_eq!(w.size_bytes(QuantizationMethod::Int4), 50);    // 100 * 4 / 8
        assert_eq!(w.size_bytes(QuantizationMethod::Float16), 200); // 100 * 16 / 8
    }

    #[test]
    fn test_magnitude_pruning() {
        let mut optimizer = ModelOptimizer::new(OptimizationConfig {
            pruning: PruningStrategy::MagnitudeBased,
            pruning_ratio: 0.5,
            quantization: QuantizationMethod::None,
            passes: vec![OptimizationPass::Pruning],
            ..Default::default()
        });

        for w in sample_weights() {
            optimizer.add_weights(w);
        }

        let result = optimizer.optimize().unwrap();
        assert!(result.layers_pruned > 0);
        assert!(result.average_sparsity > 0.0);
    }

    #[test]
    fn test_int8_quantization() {
        let mut optimizer = ModelOptimizer::new(OptimizationConfig {
            quantization: QuantizationMethod::Int8,
            pruning: PruningStrategy::None,
            passes: vec![OptimizationPass::Quantization],
            ..Default::default()
        });

        for w in sample_weights() {
            optimizer.add_weights(w);
        }

        let result = optimizer.optimize().unwrap();
        assert!(result.layers_quantized > 0);
        assert!(result.compression_ratio > 1.0);
    }

    #[test]
    fn test_full_pipeline() {
        let mut optimizer = ModelOptimizer::with_defaults();
        for w in sample_weights() {
            optimizer.add_weights(w);
        }

        let result = optimizer.optimize().unwrap();
        assert!(result.passes_applied.len() >= 2);
        assert!(result.compression_ratio > 1.0);
        assert!(result.estimated_speedup > 1.0);
        assert!(result.size_reduction_pct() > 0.0);
    }

    #[test]
    fn test_empty_model_error() {
        let mut optimizer = ModelOptimizer::with_defaults();
        let result = optimizer.optimize();
        assert!(matches!(result, Err(OptimizerError::EmptyModel)));
    }

    #[test]
    fn test_invalid_pruning_ratio() {
        let mut optimizer = ModelOptimizer::new(OptimizationConfig {
            pruning_ratio: 1.5,
            ..Default::default()
        });
        optimizer.add_weights(WeightTensor::new("test", vec![1.0; 10], vec![10]));
        let result = optimizer.optimize();
        assert!(matches!(result, Err(OptimizerError::InvalidPruningRatio(_))));
    }

    #[test]
    fn test_int4_quantization() {
        let mut optimizer = ModelOptimizer::new(OptimizationConfig {
            quantization: QuantizationMethod::Int4,
            pruning: PruningStrategy::None,
            passes: vec![OptimizationPass::Quantization],
            ..Default::default()
        });

        optimizer.add_weights(WeightTensor::new("layer", vec![0.5, -0.3, 0.8, -0.1], vec![4]));
        let result = optimizer.optimize().unwrap();
        assert_eq!(result.layers_quantized, 1);
    }

    #[test]
    fn test_optimization_history() {
        let mut optimizer = ModelOptimizer::with_defaults();
        optimizer.add_weights(WeightTensor::new("layer", vec![1.0; 100], vec![10, 10]));
        optimizer.optimize().unwrap();
        assert_eq!(optimizer.history().len(), 1);
    }

    #[test]
    fn test_sparsity_after_pruning() {
        let mut optimizer = ModelOptimizer::new(OptimizationConfig {
            pruning: PruningStrategy::MagnitudeBased,
            pruning_ratio: 0.5,
            quantization: QuantizationMethod::None,
            passes: vec![OptimizationPass::Pruning],
            ..Default::default()
        });

        optimizer.add_weights(WeightTensor::new(
            "dense",
            (0..100).map(|i| (i as f32 - 50.0) / 100.0).collect(),
            vec![10, 10],
        ));

        optimizer.optimize().unwrap();
        let weights = optimizer.weights();
        assert!(weights[0].sparsity >= 0.4); // At least 40% sparse after 50% pruning target
    }
}