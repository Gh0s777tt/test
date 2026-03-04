//! Attention Mechanisms Implementation for VantisOS
//!
//! This module provides attention mechanisms for sequence modeling:
//! - Scaled Dot-Product Attention
//! - Multi-Head Attention
//! - Self-Attention
//! - Cross-Attention
//! - Causal (Masked) Attention
//! - Linear Attention
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::attention::{MultiHeadAttention, AttentionConfig};
//!
//! let config = AttentionConfig::default();
//! let mut attention = MultiHeadAttention::new(config);
//! let query = vec![vec![0.0; 64]; 10];
//! let key = vec![vec![0.0; 64]; 10];
//! let value = vec![vec![0.0; 64]; 10];
//! let output = attention.forward(&query, &key, &value, None);
//! ```

use super::ActivationFunction;
use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Attention configuration
#[derive(Debug, Clone)]
pub struct AttentionConfig {
    /// Embedding dimension
    pub embed_dim: usize,
    /// Number of attention heads
    pub num_heads: usize,
    /// Dropout rate
    pub dropout: f64,
    /// Use bias in projections
    pub bias: bool,
    /// Key/Value dimension (if different from embed_dim)
    pub kdim: Option<usize>,
    /// Value dimension (if different from embed_dim)
    pub vdim: Option<usize>,
    /// Use causal masking
    pub causal: bool,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            embed_dim: 64,
            num_heads: 8,
            dropout: 0.0,
            bias: true,
            kdim: None,
            vdim: None,
            causal: false,
        }
    }
}

/// Scaled Dot-Product Attention
#[derive(Debug, Clone)]
pub struct ScaledDotProductAttention {
    pub dropout: f64,
    pub scale: f64,
}

impl ScaledDotProductAttention {
    pub fn new(embed_dim: usize, dropout: f64) -> Self {
        Self {
            dropout,
            scale: 1.0 / (embed_dim as f64).sqrt(),
        }
    }
    
    /// Compute attention scores
    pub fn forward(
        &self,
        query: &[Vec<f64>],
        key: &[Vec<f64>],
        value: &[Vec<f64>],
        mask: Option<&[Vec<f64>]>,
    ) -> Result<Vec<Vec<f64>>> {
        let seq_len_q = query.len();
        let seq_len_k = key.len();
        let head_dim = query[0].len();
        
        // Compute attention scores: Q @ K^T / sqrt(d_k)
        let mut scores = vec![vec![0.0; seq_len_k]; seq_len_q];
        
        for i in 0..seq_len_q {
            for j in 0..seq_len_k {
                let mut dot_product = 0.0;
                for k in 0..head_dim {
                    dot_product += query[i][k] * key[j][k];
                }
                scores[i][j] = dot_product * self.scale;
            }
        }
        
        // Apply mask if provided
        if let Some(m) = mask {
            for i in 0..seq_len_q {
                for j in 0..seq_len_k {
                    if m[i][j] == f64::NEG_INFINITY {
                        scores[i][j] = f64::NEG_INFINITY;
                    }
                }
            }
        }
        
        // Apply softmax
        let attn_weights = self.softmax(&scores)?;
        
        // Compute output: attn_weights @ V
        let mut output = vec![vec![0.0; value[0].len()]; seq_len_q];
        
        for i in 0..seq_len_q {
            for j in 0..value[0].len() {
                for k in 0..seq_len_k {
                    output[i][j] += attn_weights[i][k] * value[k][j];
                }
            }
        }
        
        Ok(output)
    }
    
    /// Softmax function
    fn softmax(&self, scores: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let rows = scores.len();
        let cols = scores[0].len();
        let mut result = vec![vec![0.0; cols]; rows];
        
        for i in 0..rows {
            // Find max for numerical stability
            let max_val = scores[i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            
            // Compute exp and sum
            let mut sum = 0.0;
            for j in 0..cols {
                let shifted = scores[i][j] - max_val;
                if shifted > -20.0 { // Avoid underflow
                    result[i][j] = shifted.exp();
                    sum += result[i][j];
                }
            }
            
            // Normalize
            if sum > 0.0 {
                for j in 0..cols {
                    result[i][j] /= sum;
                }
            }
        }
        
        Ok(result)
    }
}

/// Multi-Head Attention
#[derive(Debug, Clone)]
pub struct MultiHeadAttention {
    pub config: AttentionConfig,
    pub w_q: Vec<Vec<f64>>,
    pub w_k: Vec<Vec<f64>>,
    pub w_v: Vec<Vec<f64>>,
    pub w_o: Vec<Vec<f64>>,
    pub attention: ScaledDotProductAttention,
    pub bias_q: Option<Vec<f64>>,
    pub bias_k: Option<Vec<f64>>,
    pub bias_v: Option<Vec<f64>>,
    pub bias_o: Option<Vec<f64>>,
}

impl MultiHeadAttention {
    pub fn new(config: AttentionConfig) -> Self {
        let embed_dim = config.embed_dim;
        let scale = (2.0 / embed_dim as f64).sqrt();
        
        let make_weights = |out_dim: usize, in_dim: usize| -> Vec<Vec<f64>> {
            (0..out_dim)
                .map(|_| (0..in_dim)
                    .map(|_| {
                        let hash = (in_dim * 1000 + out_dim) as f64;
                        (hash.sin() * scale + hash.cos() * scale) / 2.0
                    })
                    .collect())
                .collect()
        };
        
        let (w_q, w_k, w_v, w_o) = (
            make_weights(embed_dim, embed_dim),
            make_weights(embed_dim, config.kdim.unwrap_or(embed_dim)),
            make_weights(embed_dim, config.vdim.unwrap_or(embed_dim)),
            make_weights(embed_dim, embed_dim),
        );
        
        let (bias_q, bias_k, bias_v, bias_o) = if config.bias {
            (
                Some(vec![0.0; embed_dim]),
                Some(vec![0.0; embed_dim]),
                Some(vec![0.0; embed_dim]),
                Some(vec![0.0; embed_dim]),
            )
        } else {
            (None, None, None, None)
        };
        
        let head_dim = embed_dim / config.num_heads;
        let attention = ScaledDotProductAttention::new(head_dim, config.dropout);
        
        Self {
            config,
            w_q,
            w_k,
            w_v,
            w_o,
            attention,
            bias_q,
            bias_k,
            bias_v,
            bias_o,
        }
    }
    
    /// Forward pass
    pub fn forward(
        &self,
        query: &[Vec<f64>],
        key: &[Vec<f64>],
        value: &[Vec<f64>],
        mask: Option<&[Vec<f64>]>,
    ) -> Result<Vec<Vec<f64>>> {
        let embed_dim = self.config.embed_dim;
        let num_heads = self.config.num_heads;
        let head_dim = embed_dim / num_heads;
        
        // Project Q, K, V
        let q = self.project(query, &self.w_q, &self.bias_q)?;
        let k = self.project(key, &self.w_k, &self.bias_k)?;
        let v = self.project(value, &self.w_v, &self.bias_v)?;
        
        // Reshape for multi-head attention: [seq_len, embed_dim] -> [num_heads, seq_len, head_dim]
        let q_heads = self.split_heads(&q, num_heads, head_dim)?;
        let k_heads = self.split_heads(&k, num_heads, head_dim)?;
        let v_heads = self.split_heads(&v, num_heads, head_dim)?;
        
        // Apply attention for each head
        let mut head_outputs = Vec::with_capacity(num_heads);
        for head in 0..num_heads {
            let head_out = self.attention.forward(&q_heads[head], &k_heads[head], &v_heads[head], mask)?;
            head_outputs.push(head_out);
        }
        
        // Concatenate heads: [num_heads, seq_len, head_dim] -> [seq_len, embed_dim]
        let concat = self.concat_heads(&head_outputs)?;
        
        // Output projection
        let output = self.project(&concat, &self.w_o, &self.bias_o)?;
        
        Ok(output)
    }
    
    /// Linear projection
    fn project(&self, input: &[Vec<f64>], weights: &[Vec<f64>], bias: &Option<Vec<f64>>) -> Result<Vec<Vec<f64>>> {
        let seq_len = input.len();
        let out_dim = weights.len();
        let in_dim = weights[0].len();
        
        let mut output = vec![vec![0.0; out_dim]; seq_len];
        
        for i in 0..seq_len {
            if input[i].len() != in_dim {
                return Err(AIServiceError::InvalidInput {
                    message: format!("Expected input dim {}, got {}", in_dim, input[i].len())
                });
            }
            
            for j in 0..out_dim {
                let mut sum: f64 = weights[j].iter().zip(input[i].iter()).map(|(w, x)| w * x).sum();
                if let Some(b) = bias {
                    sum += b[j];
                }
                output[i][j] = sum;
            }
        }
        
        Ok(output)
    }
    
    /// Split into multiple heads
    fn split_heads(&self, input: &[Vec<f64>], num_heads: usize, head_dim: usize) -> Result<Vec<Vec<Vec<f64>>>> {
        let seq_len = input.len();
        let mut heads = vec![vec![vec![0.0; head_dim]; seq_len]; num_heads];
        
        for i in 0..seq_len {
            for h in 0..num_heads {
                for j in 0..head_dim {
                    heads[h][i][j] = input[i][h * head_dim + j];
                }
            }
        }
        
        Ok(heads)
    }
    
    /// Concatenate heads back together
    fn concat_heads(&self, heads: &[Vec<Vec<f64>>]) -> Result<Vec<Vec<f64>>> {
        if heads.is_empty() {
            return Err(AIServiceError::InvalidInput {
                message: "No heads to concatenate".to_string()
            });
        }
        
        let num_heads = heads.len();
        let seq_len = heads[0].len();
        let head_dim = heads[0][0].len();
        let embed_dim = num_heads * head_dim;
        
        let mut output = vec![vec![0.0; embed_dim]; seq_len];
        
        for i in 0..seq_len {
            for h in 0..num_heads {
                for j in 0..head_dim {
                    output[i][h * head_dim + j] = heads[h][i][j];
                }
            }
        }
        
        Ok(output)
    }
}

/// Self-Attention layer
#[derive(Debug, Clone)]
pub struct SelfAttention {
    pub multi_head: MultiHeadAttention,
    pub config: AttentionConfig,
}

impl SelfAttention {
    pub fn new(config: AttentionConfig) -> Self {
        let multi_head = MultiHeadAttention::new(config.clone());
        Self { multi_head, config }
    }
    
    /// Forward pass (query = key = value)
    pub fn forward(&self, input: &[Vec<f64>], mask: Option<&[Vec<f64>]>) -> Result<Vec<Vec<f64>>> {
        self.multi_head.forward(input, input, input, mask)
    }
}

/// Cross-Attention layer
#[derive(Debug, Clone)]
pub struct CrossAttention {
    pub multi_head: MultiHeadAttention,
    pub config: AttentionConfig,
}

impl CrossAttention {
    pub fn new(config: AttentionConfig) -> Self {
        let multi_head = MultiHeadAttention::new(config.clone());
        Self { multi_head, config }
    }
    
    /// Forward pass (query from one source, key and value from another)
    pub fn forward(&self, query: &[Vec<f64>], key_value: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        self.multi_head.forward(query, key_value, key_value, None)
    }
}

/// Causal (Masked) Self-Attention
#[derive(Debug, Clone)]
pub struct CausalSelfAttention {
    pub self_attention: SelfAttention,
    pub config: AttentionConfig,
}

impl CausalSelfAttention {
    pub fn new(config: AttentionConfig) -> Self {
        let causal_config = AttentionConfig { causal: true, ..config };
        let self_attention = SelfAttention::new(causal_config.clone());
        Self { self_attention, config: causal_config }
    }
    
    /// Generate causal mask
    fn generate_causal_mask(seq_len: usize) -> Vec<Vec<f64>> {
        let mut mask = vec![vec![0.0; seq_len]; seq_len];
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask[i][j] = f64::NEG_INFINITY;
            }
        }
        mask
    }
    
    /// Forward pass with causal masking
    pub fn forward(&self, input: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let mask = Self::generate_causal_mask(input.len());
        self.self_attention.forward(input, Some(&mask))
    }
}

/// Linear Attention (for efficient long sequences)
#[derive(Debug, Clone)]
pub struct LinearAttention {
    pub embed_dim: usize,
    pub num_heads: usize,
    pub w_q: Vec<Vec<f64>>,
    pub w_k: Vec<Vec<f64>>,
    pub w_v: Vec<Vec<f64>>,
    pub w_o: Vec<Vec<f64>>,
}

impl LinearAttention {
    pub fn new(embed_dim: usize, num_heads: usize) -> Self {
        let scale = (2.0 / embed_dim as f64).sqrt();
        
        let make_weights = |out_dim: usize, in_dim: usize| -> Vec<Vec<f64>> {
            (0..out_dim)
                .map(|_| (0..in_dim)
                    .map(|_| {
                        let hash = (in_dim * 1000 + out_dim) as f64;
                        (hash.sin() * scale + hash.cos() * scale) / 2.0
                    })
                    .collect())
                .collect()
        };
        
        Self {
            embed_dim,
            num_heads,
            w_q: make_weights(embed_dim, embed_dim),
            w_k: make_weights(embed_dim, embed_dim),
            w_v: make_weights(embed_dim, embed_dim),
            w_o: make_weights(embed_dim, embed_dim),
        }
    }
    
    /// Linear attention forward (O(n) complexity instead of O(n^2))
    pub fn forward(&self, query: &[Vec<f64>], key: &[Vec<f64>], value: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let seq_len = query.len();
        
        // Project Q, K, V
        let q = self.linear_transform(query, &self.w_q)?;
        let k = self.linear_transform(key, &self.w_k)?;
        let v = self.linear_transform(value, &self.w_v)?;
        
        // Apply feature map (elu + 1) to K and Q
        let q_mapped = self.feature_map(&q);
        let k_mapped = self.feature_map(&k);
        
        // Compute K^T @ V (constant size)
        let kt_v = self.outer_product_sum(&k_mapped, v)?;
        
        // Compute Q @ (K^T @ V)
        let output = self.matrix_vector_product(&q_mapped, &kt_v)?;
        
        // Normalize
        let k_sum: Vec<f64> = (0..self.embed_dim)
            .map(|i| k_mapped.iter().map(|k| k[i]).sum())
            .collect();
        
        let mut normalized_output = vec![vec![0.0; self.embed_dim]; seq_len];
        for i in 0..seq_len {
            let normalizer: f64 = q_mapped[i].iter().zip(k_sum.iter()).map(|(q, k)| q * k).sum();
            for j in 0..self.embed_dim {
                normalized_output[i][j] = if normalizer > 1e-8 {
                    output[i][j] / normalizer
                } else {
                    output[i][j]
                };
            }
        }
        
        // Output projection
        self.linear_transform(&normalized_output, &self.w_o)
    }
    
    fn linear_transform(&self, input: &[Vec<f64>], weights: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let seq_len = input.len();
        let out_dim = weights.len();
        
        let mut output = vec![vec![0.0; out_dim]; seq_len];
        
        for i in 0..seq_len {
            for j in 0..out_dim {
                output[i][j] = weights[j].iter().zip(input[i].iter()).map(|(w, x)| w * x).sum();
            }
        }
        
        Ok(output)
    }
    
    /// Feature map: elu(x) + 1
    fn feature_map(&self, x: &[Vec<f64>]) -> Vec<Vec<f64>> {
        x.iter()
            .map(|row| {
                row.iter()
                    .map(|&v| {
                        if v >= 0.0 {
                            v + 1.0
                        } else {
                            v.exp() + 1.0
                        }
                    })
                    .collect()
            })
            .collect()
    }
    
    /// Compute sum of outer products
    fn outer_product_sum(&self, a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let m = a[0].len();
        let n = b[0].len();
        
        let mut result = vec![vec![0.0; n]; m];
        
        for (ai, bi) in a.iter().zip(b.iter()) {
            for i in 0..m {
                for j in 0..n {
                    result[i][j] += ai[i] * bi[j];
                }
            }
        }
        
        Ok(result)
    }
    
    /// Matrix-vector product
    fn matrix_vector_product(&self, a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>> {
        let seq_len = a.len();
        let m = a[0].len();
        let n = b[0].len();
        
        let mut result = vec![vec![0.0; n]; seq_len];
        
        for i in 0..seq_len {
            for j in 0..n {
                for k in 0..m {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        
        Ok(result)
    }
}

/// Attention summary helper
pub fn attention_summary(config: &AttentionConfig) -> String {
    let mut summary = String::new();
    summary.push_str("Attention Layer Summary\n");
    summary.push_str("=======================\n\n");
    summary.push_str(&format!("Embedding dimension: {}\n", config.embed_dim));
    summary.push_str(&format!("Number of heads: {}\n", config.num_heads));
    summary.push_str(&format!("Head dimension: {}\n", config.embed_dim / config.num_heads));
    summary.push_str(&format!("Dropout: {}\n", config.dropout));
    summary.push_str(&format!("Bias: {}\n", config.bias));
    summary.push_str(&format!("Causal: {}\n", config.causal));
    
    let num_params = config.embed_dim * config.embed_dim * 4; // Q, K, V, O projections
    summary.push_str(&format!("Parameters: {}\n", num_params));
    
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_dot_product_attention() {
        let attention = ScaledDotProductAttention::new(64, 0.0);
        let query = vec![vec![0.0; 64]; 10];
        let key = vec![vec![0.0; 64]; 10];
        let value = vec![vec![0.0; 64]; 10];
        
        let output = attention.forward(&query, &key, &value, None).unwrap();
        assert_eq!(output.len(), 10);
        assert_eq!(output[0].len(), 64);
    }

    #[test]
    fn test_multi_head_attention() {
        let config = AttentionConfig::default();
        let mha = MultiHeadAttention::new(config);
        
        let query = vec![vec![0.0; 64]; 10];
        let key = vec![vec![0.0; 64]; 10];
        let value = vec![vec![0.0; 64]; 10];
        
        let output = mha.forward(&query, &key, &value, None).unwrap();
        assert_eq!(output.len(), 10);
        assert_eq!(output[0].len(), 64);
    }

    #[test]
    fn test_self_attention() {
        let config = AttentionConfig::default();
        let self_attn = SelfAttention::new(config);
        
        let input = vec![vec![0.0; 64]; 10];
        let output = self_attn.forward(&input, None).unwrap();
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn test_causal_attention() {
        let config = AttentionConfig::default();
        let causal_attn = CausalSelfAttention::new(config);
        
        let input = vec![vec![0.0; 64]; 10];
        let output = causal_attn.forward(&input).unwrap();
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn test_linear_attention() {
        let attention = LinearAttention::new(64, 8);
        
        let query = vec![vec![0.0; 64]; 100];
        let key = vec![vec![0.0; 64]; 100];
        let value = vec![vec![0.0; 64]; 100];
        
        let output = attention.forward(&query, &key, &value).unwrap();
        assert_eq!(output.len(), 100);
    }

    #[test]
    fn test_attention_summary() {
        let config = AttentionConfig::default();
        let summary = attention_summary(&config);
        assert!(summary.contains("Embedding dimension: 64"));
        assert!(summary.contains("Number of heads: 8"));
    }
}