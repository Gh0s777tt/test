/// Kubernetes Service Resource Management
/// 
/// This module handles Kubernetes Service resources, which provide an abstract
/// way to expose an application running on a set of Pods as a network service.
/// 
/// Features:
/// - Service discovery and load balancing
/// - ClusterIP, NodePort, LoadBalancer, and ExternalName services
/// - Service selectors and endpoints
/// - Headless services
/// - Session affinity

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{ObjectMeta, KubernetesError, ApiVersion};

/// Kubernetes Service resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Standard object's metadata
    pub metadata: ObjectMeta,
    /// Service specification
    #[serde(rename = "spec")]
    pub spec: ServiceSpec,
    /// Service status
    #[serde(rename = "status")]
    pub status: ServiceStatus,
    /// Kubernetes API version
    pub api_version: Option<ApiVersion>,
    /// Kind of object
    pub kind: Option<String>,
}

impl Service {
    /// Create a new Service with default configuration
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
            spec: ServiceSpec::default(),
            status: ServiceStatus::default(),
            api_version: Some(ApiVersion::V1),
            kind: Some("Service".to_string()),
        }
    }

    /// Set the namespace for this Service
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.metadata.namespace = Some(namespace.into());
        self
    }

    /// Set the service type
    pub fn set_type(&mut self, service_type: ServiceType) -> &mut Self {
        self.spec.type_field = Some(service_type);
        self
    }

    /// Set the selector for pods
    pub fn add_selector(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.spec.selector.insert(key.into(), value.into());
        self
    }

    /// Add a port to the service
    pub fn add_port(&mut self, port: i32, target_port: i32) -> &mut Self {
        self.spec.ports.push(ServicePort {
            port,
            target_port: Some(target_port),
            name: None,
            protocol: Some("TCP".to_string()),
            node_port: None,
            app_protocol: None,
        });
        self
    }

    /// Add a named port to the service
    pub fn add_named_port(&mut self, name: impl Into<String>, port: i32, target_port: i32) -> &mut Self {
        self.spec.ports.push(ServicePort {
            port,
            target_port: Some(target_port),
            name: Some(name.into()),
            protocol: Some("TCP".to_string()),
            node_port: None,
            app_protocol: None,
        });
        self
    }

    /// Add a label to the Service
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    /// Get the external IP (if available)
    pub fn get_external_ip(&self) -> Option<String> {
        self.status.load_balancer.as_ref()
            .and_then(|lb| lb.ingress.as_ref())
            .and_then(|ingress| ingress.first())
            .and_then(|i| i.ip.clone())
    }

    /// Get the external hostname (if available)
    pub fn get_external_hostname(&self) -> Option<String> {
        self.status.load_balancer.as_ref()
            .and_then(|lb| lb.ingress.as_ref())
            .and_then(|ingress| ingress.first())
            .and_then(|i| i.hostname.clone())
    }

    /// Check if the service is ready (has an external IP/hostname for LoadBalancer type)
    pub fn is_ready(&self) -> bool {
        if self.spec.type_field == Some(ServiceType::LoadBalancer) {
            self.get_external_ip().is_some() || self.get_external_hostname().is_some()
        } else {
            true
        }
    }

    /// Apply defaults to the Service
    pub fn apply_defaults(&mut self) {
        if self.api_version.is_none() {
            self.api_version = Some(ApiVersion::V1);
        }
        if self.kind.is_none() {
            self.kind = Some("Service".to_string());
        }
        if self.spec.type_field.is_none() {
            self.spec.type_field = Some(ServiceType::ClusterIP);
        }
    }

    /// Validate the Service configuration
    pub fn validate(&self) -> Result<(), KubernetesError> {
        if self.metadata.name.is_none() {
            return Err(KubernetesError::ValidationFailed("Service name is required".to_string()));
        }
        if self.spec.ports.is_empty() {
            return Err(KubernetesError::ValidationFailed("Service must have at least one port".to_string()));
        }
        Ok(())
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new("default-service")
    }
}

/// Service specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceSpec {
    /// Type of service
    #[serde(rename = "type")]
    pub type_field: Option<ServiceType>,
    /// Selector for pods
    pub selector: HashMap<String, String>,
    /// Ports
    pub ports: Vec<ServicePort>,
    /// Cluster IP (can be "None" for headless services)
    #[serde(rename = "clusterIP")]
    pub cluster_ip: Option<String>,
    /// Cluster IP range
    #[serde(rename = "clusterIPs")]
    pub cluster_ips: Option<Vec<String>>,
    /// External IPs
    #[serde(rename = "externalIPs")]
    pub external_ips: Option<Vec<String>>,
    /// External name (for ExternalName type)
    #[serde(rename = "externalName")]
    pub external_name: Option<String>,
    /// External traffic policy
    #[serde(rename = "externalTrafficPolicy")]
    pub external_traffic_policy: Option<String>,
    /// Session affinity
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    /// Load balancer IP
    #[serde(rename = "loadBalancerIP")]
    pub load_balancer_ip: Option<String>,
    /// Load balancer source ranges
    #[serde(rename = "loadBalancerSourceRanges")]
    pub load_balancer_source_ranges: Option<Vec<String>>,
    /// Publish not ready addresses
    #[serde(rename = "publishNotReadyAddresses")]
    pub publish_not_ready_addresses: Option<bool>,
    /// IP families
    #[serde(rename = "ipFamilies")]
    pub ip_families: Option<Vec<String>>,
    /// IP family policy
    #[serde(rename = "ipFamilyPolicy")]
    pub ip_family_policy: Option<String>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceStatus {
    /// Load balancer status
    #[serde(rename = "loadBalancer")]
    pub load_balancer: Option<LoadBalancerStatus>,
}

/// Load balancer status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancerStatus {
    /// Load balancer ingress points
    pub ingress: Option<Vec<LoadBalancerIngress>>,
}

/// Load balancer ingress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerIngress {
    /// IP address
    pub ip: Option<String>,
    /// Hostname
    pub hostname: Option<String>,
    /// Ports
    pub ports: Option<Vec<PortStatus>>,
}

/// Port status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortStatus {
    /// Port number
    pub port: i32,
    /// Protocol
    pub protocol: String,
    /// Error message
    pub error: Option<String>,
}

/// Service port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePort {
    /// Port number
    pub port: i32,
    /// Target port
    #[serde(rename = "targetPort")]
    pub target_port: Option<i32>,
    /// Port name
    pub name: Option<String>,
    /// Protocol
    pub protocol: Option<String>,
    /// Node port (for NodePort type)
    #[serde(rename = "nodePort")]
    pub node_port: Option<i32>,
    /// Application protocol
    #[serde(rename = "appProtocol")]
    pub app_protocol: Option<String>,
}

/// Service type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
    ExternalName,
}

impl Default for ServiceType {
    fn default() -> Self {
        ServiceType::ClusterIP
    }
}

/// Service builder
pub struct ServiceBuilder {
    service: Service,
}

impl ServiceBuilder {
    /// Create a new ServiceBuilder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            service: Service::new(name),
        }
    }

    /// Set namespace
    pub fn namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.service.set_namespace(namespace);
        self
    }

    /// Set service type
    pub fn service_type(&mut self, service_type: ServiceType) -> &mut Self {
        self.service.set_type(service_type);
        self
    }

    /// Add selector
    pub fn selector(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.service.add_selector(key, value);
        self
    }

    /// Add port
    pub fn port(&mut self, port: i32, target_port: i32) -> &mut Self {
        self.service.add_port(port, target_port);
        self
    }

    /// Add named port
    pub fn named_port(&mut self, name: impl Into<String>, port: i32, target_port: i32) -> &mut Self {
        self.service.add_named_port(name, port, target_port);
        self
    }

    /// Add label
    pub fn label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.service.add_label(key, value);
        self
    }

    /// Build the service
    pub fn build(self) -> Service {
        self.service
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = Service::new("test-service");
        assert_eq!(service.metadata.name, Some("test-service".to_string()));
    }

    #[test]
    fn test_service_port() {
        let mut service = Service::new("test-service");
        service.add_port(80, 8080);
        
        assert_eq!(service.spec.ports.len(), 1);
        assert_eq!(service.spec.ports[0].port, 80);
        assert_eq!(service.spec.ports[0].target_port, Some(8080));
    }

    #[test]
    fn test_service_selector() {
        let mut service = Service::new("test-service");
        service.add_selector("app", "myapp");
        
        assert_eq!(service.spec.selector.get("app"), Some(&"myapp".to_string()));
    }

    #[test]
    fn test_service_type() {
        let mut service = Service::new("test-service");
        service.set_type(ServiceType::LoadBalancer);
        
        assert_eq!(service.spec.type_field, Some(ServiceType::LoadBalancer));
    }

    #[test]
    fn test_service_validation() {
        let service = Service::new("test-service");
        assert!(service.validate().is_err()); // No ports
        
        let mut service = Service::new("test-service");
        service.add_port(80, 8080);
        assert!(service.validate().is_ok());
    }

    #[test]
    fn test_service_builder() {
        let mut builder = ServiceBuilder::new("test-service");
        builder
            .namespace("default")
            .service_type(ServiceType::NodePort)
            .selector("app", "myapp")
            .port(80, 8080);
        let service = builder.build();
        
        assert_eq!(service.metadata.namespace, Some("default".to_string()));
        assert_eq!(service.spec.type_field, Some(ServiceType::NodePort));
        assert_eq!(service.spec.selector.get("app"), Some(&"myapp".to_string()));
    }
}