/// Kubernetes Namespace resource management
/// Handles Kubernetes Namespace resources for logical partitioning of cluster resources
/// 
/// Namespaces provide a way to divide cluster resources between multiple users or teams.
/// This module provides comprehensive Namespace management with resource quotas, labels, and annotations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{ObjectMeta, ResourceStatus, KubernetesError, ApiVersion};

/// Kubernetes Namespace resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// Standard object's metadata
    pub metadata: ObjectMeta,
    /// Namespace specification
    #[serde(rename = "spec")]
    pub spec: NamespaceSpec,
    /// Namespace status
    #[serde(rename = "status")]
    pub status: NamespaceStatus,
    /// Kubernetes API version
    pub api_version: Option<ApiVersion>,
    /// Kind of object
    pub kind: Option<String>,
}

impl Namespace {
    /// Create a new Namespace
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: None,
                labels: HashMap::new(),
                annotations: HashMap::new(),
                creation_timestamp: None,
                resource_version: None,
                generation: None,
                uid: None,
                owner_references: Vec::new(),
                finalizers: Vec::new(),
                managed_fields: Vec::new(),
            },
            spec: NamespaceSpec::default(),
            status: NamespaceStatus::default(),
            api_version: Some(ApiVersion::V1),
            kind: Some("Namespace".to_string()),
        }
    }

    /// Add a label to the Namespace
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    /// Add an annotation to the Namespace
    pub fn add_annotation(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.annotations.insert(key.into(), value.into());
        self
    }

    /// Set the finalizers for the Namespace
    pub fn set_finalizers(&mut self, finalizers: Vec<String>) -> &mut Self {
        self.metadata.finalizers = finalizers;
        self
    }

    /// Add a finalizer to the Namespace
    pub fn add_finalizer(&mut self, finalizer: impl Into<String>) -> &mut Self {
        self.metadata.finalizers.push(finalizer.into());
        self
    }

    /// Check if the Namespace is active
    pub fn is_active(&self) -> bool {
        self.status.phase == Some(NamespacePhase::Active)
    }

    /// Check if the Namespace is terminating
    pub fn is_terminating(&self) -> bool {
        self.status.phase == Some(NamespacePhase::Terminating)
    }

    /// Get the current phase of the Namespace
    pub fn get_phase(&self) -> Option<NamespacePhase> {
        self.status.phase
    }

    /// Apply defaults to the Namespace
    pub fn apply_defaults(&mut self) {
        if self.api_version.is_none() {
            self.api_version = Some(ApiVersion::V1);
        }
        if self.kind.is_none() {
            self.kind = Some("Namespace".to_string());
        }
        if self.status.phase.is_none() {
            self.status.phase = Some(NamespacePhase::Active);
        }
    }

    /// Validate the Namespace configuration
    pub fn validate(&self) -> Result<(), KubernetesError> {
        if self.metadata.name.is_none() {
            return Err(KubernetesError::ValidationFailed("Namespace name is required".to_string()));
        }
        
        // Validate namespace name according to Kubernetes naming conventions
        if let Some(name) = &self.metadata.name {
            // Check length (max 253 characters)
            if name.len() > 253 {
                return Err(KubernetesError::ValidationFailed("Namespace name too long (max 253 characters)".to_string()));
            }
            
            // Check for valid characters (lowercase alphanumeric, hyphens)
            if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
                return Err(KubernetesError::ValidationFailed("Namespace name must contain only lowercase alphanumeric characters and hyphens".to_string()));
            }
            
            // Check that it starts and ends with alphanumeric character
            if name.starts_with('-') || name.ends_with('-') {
                return Err(KubernetesError::ValidationFailed("Namespace name must start and end with alphanumeric character".to_string()));
            }
        }
        
        Ok(())
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Namespace specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NamespaceSpec {
    /// Finalizers for the namespace
    pub finalizers: Vec<String>,
}

/// Namespace status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NamespaceStatus {
    /// Current phase of the namespace
    pub phase: Option<NamespacePhase>,
    /// Additional status information
    pub conditions: Vec<NamespaceCondition>,
}

/// Namespace phase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NamespacePhase {
    /// Namespace is active and can accept new resources
    Active,
    /// Namespace is being terminated
    Terminating,
}

/// Namespace condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceCondition {
    /// Type of condition
    #[serde(rename = "type")]
    pub condition_type: String,
    /// Status of the condition
    pub status: ConditionStatus,
    /// Last time the condition transitioned
    pub last_transition_time: Option<String>,
    /// Reason for the condition's last transition
    pub reason: Option<String>,
    /// Human-readable message indicating details about the transition
    pub message: Option<String>,
}

/// Condition status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConditionStatus {
    /// Condition is true
    True,
    /// Condition is false
    False,
    /// Condition is unknown
    Unknown,
}

/// Resource quota for namespace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    /// Metadata for the resource quota
    #[serde(flatten)]
    pub metadata: ObjectMeta,
    /// Resource quota specification
    #[serde(rename = "spec")]
    pub spec: ResourceQuotaSpec,
    /// Resource quota status
    #[serde(rename = "status")]
    pub status: ResourceQuotaStatus,
}

/// Resource quota specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceQuotaSpec {
    /// Hard resource limits
    pub hard: HashMap<String, String>,
    /// Selector for scoping the quota
    pub scopes: Vec<String>,
    /// Scope selector for scoping the quota
    pub scope_selector: Option<ScopeSelector>,
}

/// Resource quota status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceQuotaStatus {
    /// Hard resource limits
    pub hard: HashMap<String, String>,
    /// Current resource usage
    pub used: HashMap<String, String>,
}

/// Scope selector for resource quotas
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScopeSelector {
    /// Match expressions for scope selector
    #[serde(rename = "matchExpressions")]
    pub match_expressions: Vec<ScopeSelectorRequirement>,
}

/// Scope selector requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeSelectorRequirement {
    /// Operator for the requirement
    pub operator: String,
    /// Scope name for the requirement
    pub scope_name: String,
    /// Values for the requirement
    pub values: Option<Vec<String>>,
}

/// Network policy for namespace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Metadata for the network policy
    #[serde(flatten)]
    pub metadata: ObjectMeta,
    /// Network policy specification
    #[serde(rename = "spec")]
    pub spec: NetworkPolicySpec,
}

/// Network policy specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicySpec {
    /// Pod selector
    #[serde(rename = "podSelector")]
    pub pod_selector: Option<LabelSelector>,
    /// Policy types
    #[serde(rename = "policyTypes")]
    pub policy_types: Vec<String>,
    /// Ingress rules
    pub ingress: Vec<NetworkPolicyIngressRule>,
    /// Egress rules
    pub egress: Vec<NetworkPolicyEgressRule>,
}

/// Label selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelSelector {
    /// Match labels
    #[serde(rename = "matchLabels")]
    pub match_labels: Option<HashMap<String, String>>,
    /// Match expressions
    #[serde(rename = "matchExpressions")]
    pub match_expressions: Vec<LabelSelectorRequirement>,
}

/// Label selector requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelSelectorRequirement {
    /// Key for the requirement
    pub key: String,
    /// Operator for the requirement
    pub operator: String,
    /// Values for the requirement
    pub values: Option<Vec<String>>,
}

/// Network policy ingress rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyIngressRule {
    /// List of sources
    pub from: Vec<NetworkPolicyPeer>,
    /// List of ports
    pub ports: Vec<NetworkPolicyPort>,
}

/// Network policy egress rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyEgressRule {
    /// List of destinations
    pub to: Vec<NetworkPolicyPeer>,
    /// List of ports
    pub ports: Vec<NetworkPolicyPort>,
}

/// Network policy peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyPeer {
    /// Pod selector
    #[serde(rename = "podSelector")]
    pub pod_selector: Option<LabelSelector>,
    /// Namespace selector
    #[serde(rename = "namespaceSelector")]
    pub namespace_selector: Option<LabelSelector>,
    /// IP block
    #[serde(rename = "ipBlock")]
    pub ip_block: Option<IPBlock>,
}

/// Network policy port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyPort {
    /// Protocol
    pub protocol: Option<String>,
    /// Port number or name
    pub port: Option<String>,
    /// End port
    #[serde(rename = "endPort")]
    pub end_port: Option<i32>,
}

/// IP block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPBlock {
    /// CIDR range
    pub cidr: String,
    /// Exceptions to CIDR range
    pub except: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let namespace = Namespace::new("test-namespace");
        assert_eq!(namespace.metadata.name, Some("test-namespace".to_string()));
    }

    #[test]
    fn test_namespace_labels() {
        let mut namespace = Namespace::new("test-namespace");
        namespace.add_label("environment", "production");
        namespace.add_label("team", "platform");
        
        assert_eq!(namespace.metadata.labels.get("environment"), Some(&"production".to_string()));
        assert_eq!(namespace.metadata.labels.get("team"), Some(&"platform".to_string()));
    }

    #[test]
    fn test_namespace_phase() {
        let mut namespace = Namespace::new("test-namespace");
        assert!(namespace.is_active());
        
        namespace.status.phase = Some(NamespacePhase::Terminating);
        assert!(namespace.is_terminating());
    }

    #[test]
    fn test_namespace_validation() {
        let namespace = Namespace::new("test-namespace");
        assert!(namespace.validate().is_ok());
        
        // Test invalid name (uppercase)
        let namespace = Namespace::new("Test-Namespace");
        assert!(namespace.validate().is_err());
        
        // Test invalid name (starts with hyphen)
        let namespace = Namespace::new("-invalid");
        assert!(namespace.validate().is_err());
        
        // Test invalid name (too long)
        let long_name = "a".repeat(254);
        let namespace = Namespace::new(long_name);
        assert!(namespace.validate().is_err());
    }

    #[test]
    fn test_namespace_finalizers() {
        let mut namespace = Namespace::new("test-namespace");
        namespace.add_finalizer("kubernetes");
        
        assert_eq!(namespace.metadata.finalizers.len(), 1);
        assert!(namespace.metadata.finalizers.contains(&"kubernetes".to_string()));
    }

    #[test]
    fn test_default_namespace() {
        let namespace = Namespace::default();
        assert_eq!(namespace.metadata.name, Some("default".to_string()));
    }
}