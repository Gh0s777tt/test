/// Multi-Cloud Support Module
/// 
/// This module provides multi-cloud capabilities for VantisOS,
/// supporting AWS, Azure, GCP, and a cloud abstraction layer.
/// 
/// Features:
/// - AWS integration (EC2, S3, VPC, EKS)
/// - Azure integration (VM, Storage, VNet, AKS)
/// - GCP integration (Compute Engine, Cloud Storage, VPC, GKE)
/// - Cloud abstraction layer for multi-cloud operations

pub mod aws;
pub mod azure;
pub mod gcp;
pub mod abstract;

// Re-export key types for convenience
pub use aws::{
    AwsClient, AwsCredentials, EC2InstanceConfig, S3BucketConfig, VPCConfig, 
    SecurityGroupConfig, BucketCannedAcl, Tenancy
};
pub use azure::{
    AzureClient, AzureCredentials, AzureConfig, AzureEnvironment, AzureToken,
    VirtualMachineConfig as AzureVMConfig, StorageAccountConfig, StorageAccountKind,
    AksClusterConfig, ImageReference, VmSize
};
pub use gcp::{
    GcpClient, GcpCredentials, GcpConfig, GcpToken, ComputeInstanceConfig,
    StorageBucketConfig as GcpStorageBucketConfig, GkeClusterConfig, MachineType
};
pub use abstract::{
    CloudProviderTrait, VirtualMachineConfig, StorageConfig, KubernetesClusterConfig,
    LoadBalancerConfig, NetworkConfig, InstanceType, OperatingSystem, DiskType,
    StorageClass, HealthStatus, ResourceType, CostEstimate, LoadBalancerType,
    MultiCloudManager, AwsProviderAdapter, AzureProviderAdapter, GcpProviderAdapter
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Cloud result type alias
pub type CloudResult<T> = Result<T, CloudError>;

/// Multi-cloud error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudError {
    /// AWS error
    AwsError(String),
    /// Azure error
    AzureError(String),
    /// GCP error
    GcpError(String),
    /// Provider not supported
    ProviderNotSupported(String),
    /// Provider not registered
    ProviderNotRegistered(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// Validation failed
    ValidationFailed(String),
    /// Authentication failed
    AuthenticationFailed(String),
    /// Resource not found
    NotFound(String),
    /// Rate limit exceeded
    RateLimitExceeded(String),
    /// Timeout
    Timeout(String),
    /// API error
    ApiError(String),
    /// Internal error
    InternalError(String),
}

impl std::fmt::Display for CloudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudError::AwsError(msg) => write!(f, "AWS error: {}", msg),
            CloudError::AzureError(msg) => write!(f, "Azure error: {}", msg),
            CloudError::GcpError(msg) => write!(f, "GCP error: {}", msg),
            CloudError::ProviderNotSupported(msg) => write!(f, "Provider not supported: {}", msg),
            CloudError::ProviderNotRegistered(msg) => write!(f, "Provider not registered: {}", msg),
            CloudError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            CloudError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            CloudError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            CloudError::NotFound(msg) => write!(f, "Not found: {}", msg),
            CloudError::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
            CloudError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            CloudError::ApiError(msg) => write!(f, "API error: {}", msg),
            CloudError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CloudError {}

/// Cloud provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    Aws,
    Azure,
    Gcp,
    OnPremise,
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudProvider::Aws => write!(f, "AWS"),
            CloudProvider::Azure => write!(f, "Azure"),
            CloudProvider::Gcp => write!(f, "GCP"),
            CloudProvider::OnPremise => write!(f, "OnPremise"),
        }
    }
}

/// Cloud resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudResource {
    /// Resource ID
    pub id: String,
    /// Resource name
    pub name: String,
    /// Resource type
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// Provider
    pub provider: CloudProvider,
    /// Region/Location
    pub location: Option<String>,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Additional properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Cloud credentials trait
#[async_trait]
pub trait CloudCredentials: Send + Sync {
    /// Get the provider type
    fn provider(&self) -> CloudProvider;
    
    /// Get as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Resource status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceStatus {
    Creating,
    Available,
    Running,
    Stopped,
    Terminated,
    Error,
    Unknown,
}

impl std::fmt::Display for ResourceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceStatus::Creating => write!(f, "CREATING"),
            ResourceStatus::Available => write!(f, "AVAILABLE"),
            ResourceStatus::Running => write!(f, "RUNNING"),
            ResourceStatus::Stopped => write!(f, "STOPPED"),
            ResourceStatus::Terminated => write!(f, "TERMINATED"),
            ResourceStatus::Error => write!(f, "ERROR"),
            ResourceStatus::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_error_display() {
        let err = CloudError::AwsError("Failed to create instance".to_string());
        assert!(err.to_string().contains("AWS error"));
    }

    #[test]
    fn test_cloud_provider() {
        let provider = CloudProvider::Aws;
        assert_eq!(provider, CloudProvider::Aws);
        assert_eq!(provider.to_string(), "AWS");
    }

    #[test]
    fn test_resource_status() {
        let status = ResourceStatus::Running;
        assert_eq!(status, ResourceStatus::Running);
        assert_eq!(status.to_string(), "RUNNING");
    }

    #[test]
    fn test_cloud_resource() {
        let resource = CloudResource {
            id: "i-1234567890".to_string(),
            name: "test-instance".to_string(),
            resource_type: "ec2-instance".to_string(),
            provider: CloudProvider::Aws,
            location: Some("us-east-1".to_string()),
            tags: HashMap::new(),
            properties: HashMap::new(),
        };

        assert_eq!(resource.id, "i-1234567890");
        assert_eq!(resource.provider, CloudProvider::Aws);
    }
}