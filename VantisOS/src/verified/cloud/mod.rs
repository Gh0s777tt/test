/// Cloud-Native Applications Module
/// 
/// This module provides cloud-native application support for VantisOS,
/// enabling deployment, scaling, and management of containerized applications.
/// 
/// Features:
/// - Cloud deployment tools
/// - Auto-scaling (HPA, VPA, Cluster Autoscaler)
/// - Load balancing
/// - Service mesh integration (Istio, Linkerd)

pub mod deployment;
pub mod autoscaling;
pub mod loadbalancer;
pub mod service_mesh;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cloud error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudError {
    /// Deployment error
    DeploymentError(String),
    /// Scaling error
    ScalingError(String),
    /// Load balancer error
    LoadBalancerError(String),
    /// Service mesh error
    ServiceMeshError(String),
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

impl std::fmt::Display for CloudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudError::DeploymentError(msg) => write!(f, "Deployment error: {}", msg),
            CloudError::ScalingError(msg) => write!(f, "Scaling error: {}", msg),
            CloudError::LoadBalancerError(msg) => write!(f, "Load balancer error: {}", msg),
            CloudError::ServiceMeshError(msg) => write!(f, "Service mesh error: {}", msg),
            CloudError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            CloudError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            CloudError::ApiError { status_code, message } => {
                write!(f, "API error ({}): {}", status_code, message)
            }
            CloudError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            CloudError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CloudError {}

/// Cloud provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    AWS,
    Azure,
    GCP,
    OnPremise,
    Custom(String),
}

impl Default for CloudProvider {
    fn default() -> Self {
        CloudProvider::OnPremise
    }
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStrategy {
    RollingUpdate,
    BlueGreen,
    Canary,
    A_BTesting,
    Recreate,
}

impl Default for DeploymentStrategy {
    fn default() -> Self {
        DeploymentStrategy::RollingUpdate
    }
}

/// Scaling strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScalingStrategy {
    Horizontal,
    Vertical,
    Both,
}

impl Default for ScalingStrategy {
    fn default() -> Self {
        ScalingStrategy::Horizontal
    }
}

/// Cloud configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudConfig {
    /// Cloud provider
    #[serde(rename = "cloudProvider")]
    pub cloud_provider: CloudProvider,
    /// Region
    pub region: Option<String>,
    /// Zone
    pub zone: Option<String>,
    /// Cluster name
    #[serde(rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// Auto-scaling enabled
    #[serde(rename = "autoScalingEnabled")]
    pub auto_scaling_enabled: bool,
    /// Service mesh enabled
    #[serde(rename = "serviceMeshEnabled")]
    pub service_mesh_enabled: bool,
    /// Labels
    pub labels: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_error_display() {
        let err = CloudError::DeploymentError("Failed to deploy".to_string());
        assert!(err.to_string().contains("Deployment error"));
    }

    #[test]
    fn test_cloud_provider() {
        let provider = CloudProvider::AWS;
        assert_eq!(provider, CloudProvider::AWS);
    }

    #[test]
    fn test_deployment_strategy() {
        let strategy = DeploymentStrategy::RollingUpdate;
        assert_eq!(strategy, DeploymentStrategy::RollingUpdate);
    }
}