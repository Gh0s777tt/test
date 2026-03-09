/// Kubernetes Integration Module
/// 
/// This module provides comprehensive Kubernetes integration for VantisOS,
/// enabling management of Kubernetes clusters, resources, and applications.
/// 
/// Features:
/// - Kubernetes API client with full CRUD operations
/// - Resource management for Pods, Deployments, Services, etc.
/// - Authentication and authorization support
/// - ConfigMap and Secret management
/// - Namespace and resource quota management
/// - Watch and event streaming
/// - Load balancing and ingress support
/// - Rolling updates and scaling
/// - Health checks and monitoring

pub mod client;
pub mod config;
pub mod auth;
pub mod pod;
pub mod deployment;
pub mod service;
pub mod replicaset;
pub mod ingress;
pub mod configmap;
pub mod secret;
pub mod namespace;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kubernetes error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KubernetesError {
    /// Connection error
    ConnectionError(String),
    /// Authentication failed
    AuthenticationFailed(String),
    /// Resource not found
    NotFound(String),
    /// Already exists
    AlreadyExists(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Validation failed
    ValidationFailed(String),
    /// API error
    ApiError {
        status_code: u16,
        message: String,
    },
    /// Timeout
    Timeout(String),
    /// Internal error
    InternalError(String),
}

impl std::fmt::Display for KubernetesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KubernetesError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            KubernetesError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            KubernetesError::NotFound(msg) => write!(f, "Not found: {}", msg),
            KubernetesError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            KubernetesError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            KubernetesError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            KubernetesError::ApiError { status_code, message } => {
                write!(f, "API error ({}): {}", status_code, message)
            }
            KubernetesError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            KubernetesError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for KubernetesError {}

/// Kubernetes API version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiVersion {
    V1,
    AppsV1,
    NetworkingV1,
    BatchV1,
    StorageV1,
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::V1
    }
}

/// Kubernetes object metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    /// Name of the object
    pub name: Option<String>,
    /// Namespace of the object
    pub namespace: Option<String>,
    /// Labels for the object
    pub labels: HashMap<String, String>,
    /// Annotations for the object
    pub annotations: HashMap<String, String>,
    /// Creation timestamp
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
    /// Resource version
    #[serde(rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// Generation
    pub generation: Option<i64>,
    /// UID
    pub uid: Option<String>,
    /// Owner references
    #[serde(rename = "ownerReferences")]
    pub owner_references: Vec<OwnerReference>,
    /// Finalizers
    pub finalizers: Vec<String>,
    /// Managed fields
    #[serde(rename = "managedFields")]
    pub managed_fields: Vec<ManagedFieldsEntry>,
}

/// Owner reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerReference {
    /// API version of the referent
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind of the referent
    pub kind: String,
    /// Name of the referent
    pub name: String,
    /// UID of the referent
    pub uid: String,
    /// If true, this reference points to the managing controller
    pub controller: Option<bool>,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer,
    /// then the owner cannot be deleted from the key-value store until this
    /// reference is removed
    #[serde(rename = "blockOwnerDeletion")]
    pub block_owner_deletion: Option<bool>,
}

/// Managed fields entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFieldsEntry {
    /// Manager is an identifier of the workflow managing this field
    pub manager: Option<String>,
    /// Operation is the type of operation which lead to this ManagedFieldsEntry
    pub operation: Option<String>,
    /// APIVersion defines the version of this schema that this field applies to
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Time is the timestamp of when the ManagedFieldsEntry was added
    pub time: Option<String>,
    /// Fields is the set of fields comprising this object
    pub fields: Option<HashMap<String, serde_json::Value>>,
}

/// Resource status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResourceStatus {
    /// Resource is being created
    Pending,
    /// Resource is running and available
    Running,
    /// Resource completed successfully
    Succeeded,
    /// Resource failed
    Failed,
    /// Resource is being terminated
    Terminating,
    /// Resource is terminated
    Terminated,
    /// Resource is unknown
    Unknown,
}

impl Default for ResourceStatus {
    fn default() -> Self {
        ResourceStatus::Unknown
    }
}

/// Kubernetes server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server version
    pub version: String,
    /// Git version
    pub git_version: String,
    /// Platform
    pub platform: String,
    /// Go version
    pub go_version: String,
}

/// Kubernetes configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KubernetesConfig {
    /// API server URL
    #[serde(rename = "apiServer")]
    pub api_server: String,
    /// Authentication method
    #[serde(rename = "authMethod")]
    pub auth_method: Option<String>,
    /// Authentication token
    #[serde(rename = "authToken")]
    pub auth_token: Option<String>,
    /// CA certificate
    #[serde(rename = "caCert")]
    pub ca_cert: Option<String>,
    /// Client certificate
    #[serde(rename = "clientCert")]
    pub client_cert: Option<String>,
    /// Client key
    #[serde(rename = "clientKey")]
    pub client_key: Option<String>,
    /// Namespace
    pub namespace: Option<String>,
    /// Timeout in seconds
    pub timeout: Option<u64>,
    /// Insecure skip TLS verify
    #[serde(rename = "insecureSkipTLS")]
    pub insecure_skip_tls: Option<bool>,
}

/// Apply defaults to object metadata
pub fn apply_defaults(meta: &mut ObjectMeta) {
    if meta.creation_timestamp.is_none() {
        meta.creation_timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kubernetes_error_display() {
        let err = KubernetesError::ConnectionError("Failed to connect".to_string());
        assert!(err.to_string().contains("Connection error"));
    }

    #[test]
    fn test_object_meta_defaults() {
        let mut meta = ObjectMeta::default();
        apply_defaults(&mut meta);
        assert!(meta.creation_timestamp.is_some());
    }

    #[test]
    fn test_resource_status() {
        let status = ResourceStatus::Running;
        assert_eq!(status, ResourceStatus::Running);
    }
}