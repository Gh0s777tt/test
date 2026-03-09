//! Kubernetes ConfigMap Management
//! 
//! Provides functionality for managing Kubernetes ConfigMaps

use super::{KubernetesError, ObjectMeta};
use alloc::string::String;
use alloc::vec::Vec;

/// ConfigMap entry
#[derive(Debug, Clone)]
pub struct ConfigMapEntry {
    /// Entry name
    pub name: String,
    /// Entry value
    pub value: String,
}

/// ConfigMap spec
#[derive(Debug, Clone, Default)]
pub struct ConfigMapSpec {
    /// Data entries
    pub data: Vec<(String, String)>,
    /// Binary data entries
    pub binary_data: Vec<(String, Vec<u8>)>,
    /// Immutable flag
    pub immutable: bool,
}

/// ConfigMap resource
#[derive(Debug, Clone)]
pub struct ConfigMap {
    /// Metadata
    pub metadata: ObjectMeta,
    /// Spec
    pub spec: ConfigMapSpec,
}

impl ConfigMap {
    /// Create a new ConfigMap
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            metadata: ObjectMeta {
                name: name.into(),
                namespace: Some(namespace.into()),
                ..Default::default()
            },
            spec: ConfigMapSpec::default(),
        }
    }
    
    /// Add data entry
    pub fn add_data(&mut self, key: &str, value: &str) {
        self.spec.data.push((key.into(), value.into()));
    }
    
    /// Add binary data entry
    pub fn add_binary_data(&mut self, key: &str, value: Vec<u8>) {
        self.spec.binary_data.push((key.into(), value));
    }
    
    /// Set immutable flag
    pub fn set_immutable(&mut self, immutable: bool) {
        self.spec.immutable = immutable;
    }
    
    /// Get data value
    pub fn get_data(&self, key: &str) -> Option<String> {
        self.spec.data.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }
    
    /// Remove data entry
    pub fn remove_data(&mut self, key: &str) {
        self.spec.data.retain(|(k, _)| k != key);
    }
}