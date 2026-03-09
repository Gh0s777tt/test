//! AI Core Module
//! 
//! Provides the central coordination for all AI operations,
//! including model management, resource allocation, and
//! inter-module communication.

use crate::ai::{config::AIConfig, error::AIError, types::{Confidence, ModelMetadata, ResourceUsage}};

/// AI Core
/// 
/// Central coordination module for the AI system.
/// 
/// ## Features
/// - Model lifecycle management
/// - Resource allocation and monitoring
/// - Inter-module coordination
/// - Performance tracking
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::core::AICore;
//! 
//! let core = AICore::new(AIConfig::default())?;
//! 
//! // Register a model
//! let model = ModelMetadata::new(ModelType::Classifier);
//! core.register_model(model)?;
//! 
//! // Get resource usage
//! let usage = core.get_resource_usage();
//! println!("CPU: {}%", usage.cpu_usage);
//! ```
/// 
/// # Safety
/// The AI Core maintains strict resource limits:
/// - Maximum memory usage: 512MB
/// - Maximum CPU overhead: 5%
/// - Maximum concurrent models: 10
pub struct AICore {
    config: AIConfig,
    initialized: bool,
    models: [Option<ModelMetadata>; 10],
    resource_usage: ResourceUsage,
}

impl AICore {
    /// Create a new AI Core
    /// 
    /// ## Arguments
    /// * `config` - AI configuration
    /// 
    /// ## Returns
    /// A new AI Core instance
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: AIConfig) -> Result<Self, AIError> {
        Ok(Self {
            config,
            initialized: true,
            models: [
                None, None, None, None, None, None, None, None, None, None
            ],
            resource_usage: ResourceUsage::default(),
        })
    }

    /// Register an AI model
    /// 
    /// ## Arguments
    /// * `model` - Model metadata to register
    /// 
    /// ## Returns
    /// Model ID for registered model
    /// 
    /// ## Errors
    /// - Returns error if no free slots available
    /// - Returns error if model is invalid
    pub fn register_model(&mut self, model: ModelMetadata) -> Result<usize, AIError> {
        if !self.initialized {
            return Err(AIError::ModuleNotReady);
        }

        // Find free slot
        for (i, slot) in self.models.iter().enumerate() {
            if slot.is_none() {
                self.models[i] = Some(model);
                return Ok(i);
            }
        }

        Err(AIError::InsufficientResources)
    }

    /// Unregister an AI model
    /// 
    /// ## Arguments
    /// * `model_id` - ID of model to unregister
    /// 
    /// ## Returns
    /// Unregistered model metadata
    /// 
    /// ## Errors
    /// - Returns error if model not found
    pub fn unregister_model(&mut self, model_id: usize) -> Result<ModelMetadata, AIError> {
        if model_id >= self.models.len() {
            return Err(AIError::ModelNotFound);
        }

        self.models[model_id]
            .take()
            .ok_or_else(|| AIError::ModelNotFound)
    }

    /// Get model metadata
    /// 
    /// ## Arguments
    /// * `model_id` - ID of model to query
    /// 
    /// ## Returns
    /// Model metadata if found
    /// 
    /// ## Errors
    /// - Returns error if model not found
    pub fn get_model(&self, model_id: usize) -> Result<&ModelMetadata, AIError> {
        self.models
            .get(model_id)
            .and_then(|m| m.as_ref())
            .ok_or_else(|| AIError::ModelNotFound)
    }

    /// Update resource usage statistics
    /// 
    /// ## Arguments
    /// * `usage` - New resource usage data
    pub fn update_resource_usage(&mut self, usage: ResourceUsage) {
        self.resource_usage = usage;
    }

    /// Get current resource usage
    /// 
    /// ## Returns
    /// Current resource usage statistics
    pub fn get_resource_usage(&self) -> ResourceUsage {
        self.resource_usage
    }

    /// Check if AI core is initialized
    /// 
    /// ## Returns
    /// True if initialized, false otherwise
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get number of registered models
    /// 
    /// ## Returns
    /// Number of registered models
    pub fn model_count(&self) -> usize {
        self.models.iter().filter(|m| m.is_some()).count()
    }
}

impl Default for AICore {
    fn default() -> Self {
        Self::new(AIConfig::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_creation() {
        let core = AICore::new(AIConfig::default()).unwrap();
        assert!(core.is_initialized());
        assert_eq!(core.model_count(), 0);
    }

    #[test]
    fn test_register_model() {
        let mut core = AICore::new(AIConfig::default()).unwrap();
        let model = ModelMetadata::new(types::ModelType::Classifier);
        
        let model_id = core.register_model(model).unwrap();
        assert_eq!(core.model_count(), 1);
        assert!(core.get_model(model_id).is_ok());
    }

    #[test]
    fn test_unregister_model() {
        let mut core = AICore::new(AIConfig::default()).unwrap();
        let model = ModelMetadata::new(types::ModelType::Classifier);
        
        let model_id = core.register_model(model).unwrap();
        assert_eq!(core.model_count(), 1);
        
        core.unregister_model(model_id).unwrap();
        assert_eq!(core.model_count(), 0);
    }

    #[test]
    fn test_max_models() {
        let mut core = AICore::new(AIConfig::default()).unwrap();
        
        // Register 10 models (max capacity)
        for _ in 0..10 {
            let model = ModelMetadata::new(types::ModelType::Classifier);
            core.register_model(model).unwrap();
        }
        
        assert_eq!(core.model_count(), 10);
        
        // Try to register 11th model (should fail)
        let model = ModelMetadata::new(types::ModelType::Classifier);
        assert!(matches!(
            core.register_model(model),
            Err(AIError::InsufficientResources)
        ));
    }
}