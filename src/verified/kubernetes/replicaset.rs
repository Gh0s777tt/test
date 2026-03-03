//! Kubernetes ReplicaSet Management
//! 
//! Provides functionality for managing Kubernetes ReplicaSets

use super::{KubernetesError, ObjectMeta};
use super::pod::PodTemplateSpec;
use alloc::string::String;
use alloc::vec::Vec;

/// ReplicaSet spec
#[derive(Debug, Clone)]
pub struct ReplicaSetSpec {
    /// Replicas
    pub replicas: u32,
    /// Minimum ready seconds
    pub min_ready_seconds: u32,
    /// Selector labels
    pub selector: Vec<(String, String)>,
    /// Pod template
    pub template: Option<PodTemplateSpec>,
}

impl Default for ReplicaSetSpec {
    fn default() -> Self {
        Self {
            replicas: 1,
            min_ready_seconds: 0,
            selector: Vec::new(),
            template: None,
        }
    }
}

/// ReplicaSet condition
#[derive(Debug, Clone)]
pub struct ReplicaSetCondition {
    /// Condition type
    pub condition_type: String,
    /// Status
    pub status: bool,
    /// Last transition time
    pub last_transition_time: Option<String>,
    /// Reason
    pub reason: Option<String>,
    /// Message
    pub message: Option<String>,
}

/// ReplicaSet status
#[derive(Debug, Clone, Default)]
pub struct ReplicaSetStatus {
    /// Replicas
    pub replicas: u32,
    /// Fully labeled replicas
    pub fully_labeled_replicas: u32,
    /// Ready replicas
    pub ready_replicas: u32,
    /// Available replicas
    pub available_replicas: u32,
    /// Observed generation
    pub observed_generation: Option<u64>,
    /// Conditions
    pub conditions: Vec<ReplicaSetCondition>,
}

/// ReplicaSet resource
#[derive(Debug, Clone)]
pub struct ReplicaSet {
    /// Metadata
    pub metadata: ObjectMeta,
    /// Spec
    pub spec: ReplicaSetSpec,
    /// Status
    pub status: ReplicaSetStatus,
}

impl ReplicaSet {
    /// Create a new ReplicaSet
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: Some(namespace.into()),
                ..Default::default()
            },
            spec: ReplicaSetSpec::default(),
            status: ReplicaSetStatus::default(),
        }
    }
    
    /// Set replicas
    pub fn set_replicas(&mut self, replicas: u32) {
        self.spec.replicas = replicas;
    }
    
    /// Add selector
    pub fn add_selector(&mut self, key: &str, value: &str) {
        self.spec.selector.push((key.into(), value.into()));
    }
    
    /// Check if fully available
    pub fn is_fully_available(&self) -> bool {
        self.status.available_replicas == self.spec.replicas
    }
    
    /// Scale ReplicaSet
    pub fn scale(&mut self, replicas: u32) {
        self.spec.replicas = replicas;
    }
}