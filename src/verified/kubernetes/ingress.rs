/// Kubernetes Ingress Resource Management
/// 
/// This module handles Kubernetes Ingress resources, which provide HTTP and HTTPS
/// routing from outside the cluster to services within the cluster.
/// 
/// Features:
/// - HTTP and HTTPS routing
/// - Path-based routing
/// - Host-based routing
/// - TLS configuration
/// - Ingress classes and annotations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{ObjectMeta, KubernetesError, ApiVersion};

/// Kubernetes Ingress resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingress {
    /// Standard object's metadata
    pub metadata: ObjectMeta,
    /// Ingress specification
    #[serde(rename = "spec")]
    pub spec: IngressSpec,
    /// Ingress status
    #[serde(rename = "status")]
    pub status: IngressStatus,
    /// Kubernetes API version
    pub api_version: Option<ApiVersion>,
    /// Kind of object
    pub kind: Option<String>,
}

impl Ingress {
    /// Create a new Ingress with default configuration
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
            spec: IngressSpec::default(),
            status: IngressStatus::default(),
            api_version: Some(ApiVersion::NetworkingV1),
            kind: Some("Ingress".to_string()),
        }
    }

    /// Set the namespace for this Ingress
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.metadata.namespace = Some(namespace.into());
        self
    }

    /// Add a rule to the ingress
    pub fn add_rule(&mut self, rule: IngressRule) -> &mut Self {
        self.spec.rules.push(rule);
        self
    }

    /// Add a TLS configuration
    pub fn add_tls(&mut self, tls: IngressTLS) -> &mut Self {
        self.spec.tls.get_or_insert_with(Vec::new).push(tls);
        self
    }

    /// Set the ingress class
    pub fn set_ingress_class(&mut self, class: impl Into<String>) -> &mut Self {
        self.spec.ingress_class_name = Some(class.into());
        self
    }

    /// Add a label to the Ingress
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    /// Add an annotation to the Ingress
    pub fn add_annotation(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.annotations.insert(key.into(), value.into());
        self
    }

    /// Get the external address
    pub fn get_external_address(&self) -> Option<String> {
        self.status.load_balancer.as_ref()
            .and_then(|lb| lb.ingress.as_ref())
            .and_then(|ingress| ingress.first())
            .and_then(|i| i.ip.clone())
            .or_else(|| {
                self.status.load_balancer.as_ref()
                    .and_then(|lb| lb.ingress.as_ref())
                    .and_then(|ingress| ingress.first())
                    .and_then(|i| i.hostname.clone())
            })
    }

    /// Apply defaults to the Ingress
    pub fn apply_defaults(&mut self) {
        if self.api_version.is_none() {
            self.api_version = Some(ApiVersion::NetworkingV1);
        }
        if self.kind.is_none() {
            self.kind = Some("Ingress".to_string());
        }
    }

    /// Validate the Ingress configuration
    pub fn validate(&self) -> Result<(), KubernetesError> {
        if self.metadata.name.is_none() {
            return Err(KubernetesError::ValidationFailed("Ingress name is required".to_string()));
        }
        if self.spec.rules.is_empty() {
            return Err(KubernetesError::ValidationFailed("Ingress must have at least one rule".to_string()));
        }
        Ok(())
    }
}

impl Default for Ingress {
    fn default() -> Self {
        Self::new("default-ingress")
    }
}

/// Ingress specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IngressSpec {
    /// Ingress class name
    #[serde(rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    /// Rules
    pub rules: Vec<IngressRule>,
    /// TLS configuration
    pub tls: Option<Vec<IngressTLS>>,
    /// Default backend
    #[serde(rename = "defaultBackend")]
    pub default_backend: Option<IngressBackend>,
}

/// Ingress status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IngressStatus {
    /// Load balancer status
    #[serde(rename = "loadBalancer")]
    pub load_balancer: Option<IngressLoadBalancerStatus>,
}

/// Ingress load balancer status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressLoadBalancerStatus {
    /// Load balancer ingress points
    pub ingress: Option<Vec<IngressLoadBalancerIngress>>,
}

/// Ingress load balancer ingress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressLoadBalancerIngress {
    /// IP address
    pub ip: Option<String>,
    /// Hostname
    pub hostname: Option<String>,
    /// Ports
    pub ports: Option<Vec<IngressPortStatus>>,
}

/// Ingress port status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressPortStatus {
    /// Port number
    pub port: i32,
    /// Protocol
    pub protocol: String,
    /// Error message
    pub error: Option<String>,
}

/// Ingress rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressRule {
    /// Host (optional)
    pub host: Option<String>,
    /// HTTP rule
    pub http: Option<HTTPIngressRuleValue>,
}

/// HTTP ingress rule value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPIngressRuleValue {
    /// Paths
    pub paths: Vec<HTTPIngressPath>,
}

/// HTTP ingress path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPIngressPath {
    /// Path
    pub path: Option<String>,
    /// Path type
    #[serde(rename = "pathType")]
    pub path_type: PathType,
    /// Backend
    pub backend: IngressBackend,
}

/// Path type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PathType {
    Exact,
    Prefix,
    ImplementationSpecific,
}

impl Default for PathType {
    fn default() -> Self {
        PathType::Prefix
    }
}

/// Ingress backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressBackend {
    /// Service reference
    pub service: Option<IngressServiceBackend>,
    /// Resource reference
    pub resource: Option<HashMap<String, String>>,
}

/// Ingress service backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressServiceBackend {
    /// Service name
    pub name: String,
    /// Service port
    pub port: ServiceBackendPort,
}

/// Service backend port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBackendPort {
    /// Port number
    pub number: Option<i32>,
    /// Port name
    pub name: Option<String>,
}

/// Ingress TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressTLS {
    /// Hosts
    pub hosts: Option<Vec<String>>,
    /// Secret name
    #[serde(rename = "secretName")]
    pub secret_name: Option<String>,
}

/// Ingress builder
pub struct IngressBuilder {
    ingress: Ingress,
}

impl IngressBuilder {
    /// Create a new IngressBuilder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ingress: Ingress::new(name),
        }
    }

    /// Set namespace
    pub fn namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.ingress.set_namespace(namespace);
        self
    }

    /// Set ingress class
    pub fn ingress_class(&mut self, class: impl Into<String>) -> &mut Self {
        self.ingress.set_ingress_class(class);
        self
    }

    /// Add rule
    pub fn rule(&mut self, rule: IngressRule) -> &mut Self {
        self.ingress.add_rule(rule);
        self
    }

    /// Add TLS
    pub fn tls(&mut self, tls: IngressTLS) -> &mut Self {
        self.ingress.add_tls(tls);
        self
    }

    /// Add label
    pub fn label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.ingress.add_label(key, value);
        self
    }

    /// Add annotation
    pub fn annotation(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.ingress.add_annotation(key, value);
        self
    }

    /// Build the ingress
    pub fn build(self) -> Ingress {
        self.ingress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingress_creation() {
        let ingress = Ingress::new("test-ingress");
        assert_eq!(ingress.metadata.name, Some("test-ingress".to_string()));
    }

    #[test]
    fn test_ingress_rule() {
        let mut ingress = Ingress::new("test-ingress");
        let rule = IngressRule {
            host: Some("example.com".to_string()),
            http: None,
        };
        ingress.add_rule(rule);
        
        assert_eq!(ingress.spec.rules.len(), 1);
        assert_eq!(ingress.spec.rules[0].host, Some("example.com".to_string()));
    }

    #[test]
    fn test_ingress_tls() {
        let mut ingress = Ingress::new("test-ingress");
        let tls = IngressTLS {
            hosts: Some(vec!["example.com".to_string()]),
            secret_name: Some("tls-secret".to_string()),
        };
        ingress.add_tls(tls);
        
        assert_eq!(ingress.spec.tls.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_ingress_validation() {
        let ingress = Ingress::new("test-ingress");
        assert!(ingress.validate().is_err()); // No rules
        
        let mut ingress = Ingress::new("test-ingress");
        let rule = IngressRule {
            host: Some("example.com".to_string()),
            http: None,
        };
        ingress.add_rule(rule);
        assert!(ingress.validate().is_ok());
    }

    #[test]
    fn test_path_type() {
        assert_eq!(PathType::Exact, PathType::Exact);
        assert_eq!(PathType::Prefix, PathType::Prefix);
        assert_eq!(PathType::ImplementationSpecific, PathType::ImplementationSpecific);
    }
}