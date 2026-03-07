// AI Research Framework for VantisOS
// Provides distributed training, model versioning, and research tools

pub mod training;
pub mod versioning;
pub mod interfaces;
pub mod distributed;

// Re-export main types
pub use training::{DistributedTrainer, TrainingConfig, OptimizerConfig};
pub use versioning::{ModelVersion, VersionManager, ModelRegistry};
pub use interfaces::{Model, ModelBuilder, ModelConfig};
pub use distributed::{NodeManager, FederatedLearning, SyncStrategy};

/// AI Research configuration
#[derive(Clone, Debug)]
pub struct AIResearchConfig {
    /// Enable distributed training
    pub distributed: bool,
    /// Enable model versioning
    pub versioning: bool,
    /// Number of workers for distributed training
    pub num_workers: usize,
    /// Checkpoint interval (epochs)
    pub checkpoint_interval: usize,
}

impl Default for AIResearchConfig {
    fn default() -> Self {
        AIResearchConfig {
            distributed: false,
            versioning: true,
            num_workers: 4,
            checkpoint_interval: 10,
        }
    }
}

impl AIResearchConfig {
    /// Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable distributed training
    pub fn with_distributed(mut self, num_workers: usize) -> Self {
        self.distributed = true;
        self.num_workers = num_workers;
        self
    }

    /// Set checkpoint interval
    pub fn with_checkpoint_interval(mut self, interval: usize) -> Self {
        self.checkpoint_interval = interval;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AIResearchConfig::default();
        assert!(!config.distributed);
        assert!(config.versioning);
        assert_eq!(config.num_workers, 4);
    }

    #[test]
    fn test_config_builder() {
        let config = AIResearchConfig::new()
            .with_distributed(8)
            .with_checkpoint_interval(5);
        
        assert!(config.distributed);
        assert_eq!(config.num_workers, 8);
        assert_eq!(config.checkpoint_interval, 5);
    }
}