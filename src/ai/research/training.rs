// Distributed Training Framework for VantisOS
// High-performance distributed training with fault tolerance

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Distributed trainer configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of epochs
    pub epochs: usize,
    /// Batch size
    pub batch_size: usize,
    /// Learning rate
    pub learning_rate: f64,
    /// Gradient accumulation steps
    pub gradient_accumulation_steps: usize,
    /// Enable mixed precision training
    pub mixed_precision: bool,
    /// Enable gradient checkpointing
    pub gradient_checkpointing: bool,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        TrainingConfig {
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.001,
            gradient_accumulation_steps: 1,
            mixed_precision: false,
            gradient_checkpointing: false,
            seed: None,
        }
    }
}

impl TrainingConfig {
    /// Create a new training configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set number of epochs
    pub fn epochs(mut self, epochs: usize) -> Self {
        self.epochs = epochs;
        self
    }

    /// Set batch size
    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Set learning rate
    pub fn learning_rate(mut self, lr: f64) -> Self {
        self.learning_rate = lr;
        self
    }

    /// Enable mixed precision training
    pub fn mixed_precision(mut self, enable: bool) -> Self {
        self.mixed_precision = enable;
        self
    }
}

/// Optimizer configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptimizerConfig {
    /// Optimizer type (adam, sgd, rmsprop, etc.)
    pub optimizer_type: String,
    /// Learning rate
    pub learning_rate: f64,
    /// Beta1 for Adam
    pub beta1: f64,
    /// Beta2 for Adam
    pub beta2: f64,
    /// Epsilon for numerical stability
    pub epsilon: f64,
    /// Weight decay (L2 regularization)
    pub weight_decay: f64,
    /// Gradient clipping threshold
    pub gradient_clipping: Option<f64>,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        OptimizerConfig {
            optimizer_type: "adam".to_string(),
            learning_rate: 0.001,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.0,
            gradient_clipping: None,
        }
    }
}

impl OptimizerConfig {
    /// Create Adam optimizer config
    pub fn adam(learning_rate: f64) -> Self {
        OptimizerConfig {
            optimizer_type: "adam".to_string(),
            learning_rate,
            ..Default::default()
        }
    }

    /// Create SGD optimizer config
    pub fn sgd(learning_rate: f64) -> Self {
        OptimizerConfig {
            optimizer_type: "sgd".to_string(),
            learning_rate,
            ..Default::default()
        }
    }

    /// Set gradient clipping
    pub fn with_gradient_clipping(mut self, threshold: f64) -> Self {
        self.gradient_clipping = Some(threshold);
        self
    }

    /// Set weight decay
    pub fn with_weight_decay(mut self, decay: f64) -> Self {
        self.weight_decay = decay;
        self
    }
}

/// Gradient accumulator for distributed training
#[derive(Clone, Debug)]
pub struct GradientAccumulator {
    gradients: Vec<f64>,
    count: usize,
}

impl GradientAccumulator {
    /// Create a new gradient accumulator
    pub fn new(size: usize) -> Self {
        GradientAccumulator {
            gradients: vec![0.0; size],
            count: 0,
        }
    }

    /// Accumulate gradients
    pub fn accumulate(&mut self, gradients: &[f64]) {
        for (i, &g) in gradients.iter().enumerate() {
            if i < self.gradients.len() {
                self.gradients[i] += g;
            }
        }
        self.count += 1;
    }

    /// Get averaged gradients
    pub fn average(&self) -> Vec<f64> {
        if self.count == 0 {
            return vec![0.0; self.gradients.len()];
        }
        self.gradients.iter().map(|&g| g / self.count as f64).collect()
    }

    /// Reset accumulator
    pub fn reset(&mut self) {
        self.gradients.fill(0.0);
        self.count = 0;
    }
}

/// Learning rate scheduler
#[derive(Clone, Debug)]
pub enum LearningRateScheduler {
    /// Constant learning rate
    Constant,
    /// Step decay
    Step { step_size: usize, gamma: f64 },
    /// Exponential decay
    Exponential { gamma: f64 },
    /// Cosine annealing
    Cosine { t_max: usize, eta_min: f64 },
    /// Linear warmup
    Warmup { warmup_steps: usize, target_lr: f64 },
    /// One cycle policy
    OneCycle { max_lr: f64, total_steps: usize },
}

impl LearningRateScheduler {
    /// Get learning rate for current step
    pub fn get_lr(&self, step: usize, base_lr: f64) -> f64 {
        match self {
            LearningRateScheduler::Constant => base_lr,
            LearningRateScheduler::Step { step_size, gamma } => {
                let num_steps = step / step_size;
                base_lr * gamma.powi(num_steps as i32)
            }
            LearningRateScheduler::Exponential { gamma } => {
                base_lr * gamma.powi(step as i32)
            }
            LearningRateScheduler::Cosine { t_max, eta_min } => {
                let progress = (step % t_max) as f64 / *t_max as f64;
                eta_min + (base_lr - eta_min) * (1.0 + (std::f64::consts::PI * progress).cos()) / 2.0
            }
            LearningRateScheduler::Warmup { warmup_steps, target_lr } => {
                if step < *warmup_steps {
                    target_lr * step as f64 / *warmup_steps as f64
                } else {
                    *target_lr
                }
            }
            LearningRateScheduler::OneCycle { max_lr, total_steps } => {
                let progress = step as f64 / *total_steps as f64;
                if progress < 0.5 {
                    max_lr * (1.0 + 2.0 * progress)
                } else {
                    max_lr * (2.0 - 2.0 * progress)
                }
            }
        }
    }
}

/// Checkpoint manager for saving and loading training state
#[derive(Clone, Debug)]
pub struct CheckpointManager {
    checkpoints: Vec<Checkpoint>,
    max_checkpoints: usize,
}

/// Training checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Epoch number
    pub epoch: usize,
    /// Global step
    pub global_step: usize,
    /// Model weights
    pub weights: Vec<f64>,
    /// Optimizer state
    pub optimizer_state: HashMap<String, Vec<f64>>,
    /// Training metrics
    pub metrics: HashMap<String, f64>,
    /// Timestamp
    pub timestamp: u64,
}

impl CheckpointManager {
    /// Create a new checkpoint manager
    pub fn new(max_checkpoints: usize) -> Self {
        CheckpointManager {
            checkpoints: Vec::new(),
            max_checkpoints,
        }
    }

    /// Save a checkpoint
    pub fn save(&mut self, checkpoint: Checkpoint) {
        self.checkpoints.push(checkpoint);
        
        // Remove old checkpoints if needed
        while self.checkpoints.len() > self.max_checkpoints {
            self.checkpoints.remove(0);
        }
    }

    /// Load the latest checkpoint
    pub fn load_latest(&self) -> Option<&Checkpoint> {
        self.checkpoints.last()
    }

    /// Load a specific checkpoint by epoch
    pub fn load_by_epoch(&self, epoch: usize) -> Option<&Checkpoint> {
        self.checkpoints.iter().find(|c| c.epoch == epoch)
    }

    /// List all checkpoints
    pub fn list(&self) -> &[Checkpoint] {
        &self.checkpoints
    }
}

/// Early stopping callback
#[derive(Clone, Debug)]
pub struct EarlyStopping {
    patience: usize,
    min_delta: f64,
    best_loss: f64,
    counter: usize,
    should_stop: bool,
}

impl EarlyStopping {
    /// Create a new early stopping instance
    pub fn new(patience: usize, min_delta: f64) -> Self {
        EarlyStopping {
            patience,
            min_delta,
            best_loss: f64::MAX,
            counter: 0,
            should_stop: false,
        }
    }

    /// Check if training should stop
    pub fn step(&mut self, loss: f64) -> bool {
        if loss < self.best_loss - self.min_delta {
            self.best_loss = loss;
            self.counter = 0;
        } else {
            self.counter += 1;
            if self.counter >= self.patience {
                self.should_stop = true;
            }
        }
        self.should_stop
    }

    /// Check if early stopping triggered
    pub fn should_stop(&self) -> bool {
        self.should_stop
    }

    /// Reset early stopping
    pub fn reset(&mut self) {
        self.best_loss = f64::MAX;
        self.counter = 0;
        self.should_stop = false;
    }
}

/// Distributed trainer
#[derive(Clone, Debug)]
pub struct DistributedTrainer {
    config: TrainingConfig,
    optimizer_config: OptimizerConfig,
    gradient_accumulator: GradientAccumulator,
    checkpoint_manager: CheckpointManager,
    early_stopping: Option<EarlyStopping>,
    metrics: HashMap<String, Vec<f64>>,
    current_epoch: usize,
    global_step: usize,
}

impl DistributedTrainer {
    /// Create a new distributed trainer
    pub fn new(config: TrainingConfig, optimizer_config: OptimizerConfig) -> Self {
        DistributedTrainer {
            gradient_accumulator: GradientAccumulator::new(1000), // Default size
            checkpoint_manager: CheckpointManager::new(5),
            config,
            optimizer_config,
            early_stopping: None,
            metrics: HashMap::new(),
            current_epoch: 0,
            global_step: 0,
        }
    }

    /// Enable early stopping
    pub fn with_early_stopping(mut self, patience: usize, min_delta: f64) -> Self {
        self.early_stopping = Some(EarlyStopping::new(patience, min_delta));
        self
    }

    /// Train for one step
    pub fn train_step(&mut self, gradients: &[f64]) -> Result<Vec<f64>, String> {
        // Accumulate gradients
        self.gradient_accumulator.accumulate(gradients);
        
        // Check if we should update
        if self.gradient_accumulator.count >= self.config.gradient_accumulation_steps {
            let averaged = self.gradient_accumulator.average();
            self.gradient_accumulator.reset();
            
            // Apply gradient clipping if configured
            let clipped = if let Some(threshold) = self.optimizer_config.gradient_clipping {
                Self::clip_gradients(&averaged, threshold)
            } else {
                averaged
            };
            
            // Apply weight decay if configured
            let decayed = if self.optimizer_config.weight_decay > 0.0 {
                clipped.iter()
                    .map(|&g| g * (1.0 - self.optimizer_config.weight_decay))
                    .collect()
            } else {
                clipped
            };
            
            self.global_step += 1;
            Ok(decayed)
        } else {
            Ok(vec![])
        }
    }

    /// Train for one epoch
    pub fn train_epoch(&mut self, num_batches: usize) -> Result<(), String> {
        for _ in 0..num_batches {
            // Simulate training step
            let gradients = vec![0.1; 1000];
            self.train_step(&gradients)?;
        }
        
        self.current_epoch += 1;
        Ok(())
    }

    /// Log metric
    pub fn log_metric(&mut self, name: &str, value: f64) {
        self.metrics.entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }

    /// Get metrics
    pub fn get_metrics(&self) -> &HashMap<String, Vec<f64>> {
        &self.metrics
    }

    /// Save checkpoint
    pub fn save_checkpoint(&mut self, weights: Vec<f64>) {
        let checkpoint = Checkpoint {
            epoch: self.current_epoch,
            global_step: self.global_step,
            weights,
            optimizer_state: HashMap::new(),
            metrics: self.metrics.iter()
                .filter_map(|(k, v)| v.last().map(|&last| (k.clone(), last)))
                .collect(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.checkpoint_manager.save(checkpoint);
    }

    /// Load checkpoint
    pub fn load_checkpoint(&mut self) -> Option<Vec<f64>> {
        self.checkpoint_manager.load_latest().map(|c| {
            self.current_epoch = c.epoch;
            self.global_step = c.global_step;
            c.weights.clone()
        })
    }

    /// Get current epoch
    pub fn current_epoch(&self) -> usize {
        self.current_epoch
    }

    /// Get global step
    pub fn global_step(&self) -> usize {
        self.global_step
    }

    /// Check if training should stop
    pub fn should_stop(&self) -> bool {
        self.early_stopping.as_ref().map_or(false, |es| es.should_stop())
    }

    /// Clip gradients
    fn clip_gradients(gradients: &[f64], threshold: f64) -> Vec<f64> {
        let norm: f64 = gradients.iter().map(|g| g * g).sum::<f64>().sqrt();
        if norm > threshold {
            let scale = threshold / norm;
            gradients.iter().map(|g| g * scale).collect()
        } else {
            gradients.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_config() {
        let config = TrainingConfig::new()
            .epochs(100)
            .batch_size(64)
            .learning_rate(0.01)
            .mixed_precision(true);
        
        assert_eq!(config.epochs, 100);
        assert_eq!(config.batch_size, 64);
        assert_eq!(config.learning_rate, 0.01);
        assert!(config.mixed_precision);
    }

    #[test]
    fn test_optimizer_config() {
        let config = OptimizerConfig::adam(0.001)
            .with_gradient_clipping(1.0)
            .with_weight_decay(0.01);
        
        assert_eq!(config.optimizer_type, "adam");
        assert_eq!(config.gradient_clipping, Some(1.0));
        assert_eq!(config.weight_decay, 0.01);
    }

    #[test]
    fn test_gradient_accumulator() {
        let mut acc = GradientAccumulator::new(10);
        acc.accumulate(&[1.0; 10]);
        acc.accumulate(&[2.0; 10]);
        
        let avg = acc.average();
        assert_eq!(avg[0], 1.5);
    }

    #[test]
    fn test_lr_scheduler_constant() {
        let scheduler = LearningRateScheduler::Constant;
        assert_eq!(scheduler.get_lr(0, 0.001), 0.001);
        assert_eq!(scheduler.get_lr(100, 0.001), 0.001);
    }

    #[test]
    fn test_lr_scheduler_warmup() {
        let scheduler = LearningRateScheduler::Warmup {
            warmup_steps: 100,
            target_lr: 0.001,
        };
        assert!((scheduler.get_lr(0, 0.0) - 0.0).abs() < 1e-10);
        assert!((scheduler.get_lr(100, 0.0) - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_checkpoint_manager() {
        let mut manager = CheckpointManager::new(3);
        
        manager.save(Checkpoint {
            epoch: 1,
            global_step: 100,
            weights: vec![1.0],
            optimizer_state: HashMap::new(),
            metrics: HashMap::new(),
            timestamp: 0,
        });
        
        assert!(manager.load_latest().is_some());
        assert_eq!(manager.list().len(), 1);
    }

    #[test]
    fn test_early_stopping() {
        let mut es = EarlyStopping::new(3, 0.001);
        
        assert!(!es.step(0.1));
        assert!(!es.step(0.09));
        assert!(!es.step(0.095)); // No improvement
        assert!(!es.step(0.096)); // No improvement
        assert!(es.step(0.097)); // Patience exceeded
    }

    #[test]
    fn test_distributed_trainer() {
        let config = TrainingConfig::new();
        let opt_config = OptimizerConfig::default();
        let mut trainer = DistributedTrainer::new(config, opt_config);
        
        let gradients = vec![0.1; 1000];
        let result = trainer.train_step(&gradients);
        assert!(result.is_ok());
    }
}