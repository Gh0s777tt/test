/// Auto-Scaling Module
/// 
/// This module provides auto-scaling capabilities for cloud-native applications,
/// including Horizontal Pod Autoscaler (HPA), Vertical Pod Autoscaler (VPA),
/// and Cluster Autoscaler (CA) support.
/// 
/// Features:
/// - Horizontal Pod Autoscaler (HPA)
/// - Vertical Pod Autoscaler (VPA)
/// - Cluster Autoscaler (CA)
/// - Custom metrics support
/// - Predictive scaling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{CloudError, ScalingStrategy};

/// Horizontal Pod Autoscaler (HPA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalPodAutoscaler {
    /// HPA name
    pub name: String,
    /// Namespace
    pub namespace: String,
    /// Target resource (Deployment, StatefulSet, ReplicaSet)
    #[serde(rename = "targetRef")]
    pub target_ref: CrossVersionObjectReference,
    /// Minimum replicas
    #[serde(rename = "minReplicas")]
    pub min_replicas: i32,
    /// Maximum replicas
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,
    /// Current replicas
    #[serde(rename = "currentReplicas")]
    pub current_replicas: i32,
    /// Metrics for scaling
    pub metrics: Vec<MetricSpec>,
    /// Scaling behavior
    pub behavior: Option<HorizontalPodAutoscalerBehavior>,
    /// HPA status
    pub status: HorizontalPodAutoscalerStatus,
}

impl HorizontalPodAutoscaler {
    /// Create a new HPA
    pub fn new(name: impl Into<String>, target_name: impl Into<String>, target_kind: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: "default".to_string(),
            target_ref: CrossVersionObjectReference {
                api_version: "apps/v1".to_string(),
                kind: target_kind.into(),
                name: target_name.into(),
            },
            min_replicas: 1,
            max_replicas: 10,
            current_replicas: 1,
            metrics: vec![MetricSpec::default_cpu_utilization(80)],
            behavior: None,
            status: HorizontalPodAutoscalerStatus::default(),
        }
    }

    /// Set the namespace
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into();
        self
    }

    /// Set replica range
    pub fn set_replica_range(&mut self, min: i32, max: i32) -> &mut Self {
        self.min_replicas = min;
        self.max_replicas = max;
        self
    }

    /// Add a CPU metric
    pub fn add_cpu_metric(&mut self, target_utilization: i32) -> &mut Self {
        self.metrics.push(MetricSpec::default_cpu_utilization(target_utilization));
        self
    }

    /// Add a memory metric
    pub fn add_memory_metric(&mut self, target_utilization: i32) -> &mut Self {
        self.metrics.push(MetricSpec {
            type_field: MetricSourceType::Resource,
            resource: Some(ResourceMetricSource {
                name: "memory".to_string(),
                target: MetricTarget {
                    type_field: MetricTargetType::Utilization,
                    value: None,
                    average_value: None,
                    average_utilization: Some(target_utilization),
                },
            }),
            pods: None,
            object: None,
            external: None,
        });
        self
    }

    /// Add a custom metric
    pub fn add_custom_metric(&mut self, name: impl Into<String>, target_value: impl Into<String>) -> &mut Self {
        self.metrics.push(MetricSpec {
            type_field: MetricSourceType::Pods,
            resource: None,
            pods: Some(PodsMetricSource {
                metric: MetricIdentifier {
                    name: name.into(),
                    selector: None,
                },
                target: MetricTarget {
                    type_field: MetricTargetType::Value,
                    value: Some(target_value.into()),
                    average_value: None,
                    average_utilization: None,
                },
            }),
            object: None,
            external: None,
        });
        self
    }

    /// Validate HPA configuration
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.min_replicas < 0 {
            return Err(CloudError::ValidationFailed("Min replicas must be non-negative".to_string()));
        }
        if self.max_replicas < self.min_replicas {
            return Err(CloudError::ValidationFailed("Max replicas must be >= min replicas".to_string()));
        }
        if self.metrics.is_empty() {
            return Err(CloudError::ValidationFailed("At least one metric is required".to_string()));
        }
        Ok(())
    }

    /// Calculate desired replicas based on metrics
    pub fn calculate_desired_replicas(&self, current_value: f64, target_value: f64) -> i32 {
        if target_value == 0.0 {
            return self.current_replicas;
        }
        
        let ratio = current_value / target_value;
        let desired = (self.current_replicas as f64 * ratio).ceil() as i32;
        
        desired.max(self.min_replicas).min(self.max_replicas)
    }

    /// Check if scaling is needed
    pub fn needs_scaling(&self, current_value: f64, target_value: f64) -> bool {
        let desired = self.calculate_desired_replicas(current_value, target_value);
        desired != self.current_replicas
    }
}

/// Cross version object reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossVersionObjectReference {
    /// API version
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind
    pub kind: String,
    /// Name
    pub name: String,
}

/// Metric specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSpec {
    /// Metric type
    #[serde(rename = "type")]
    pub type_field: MetricSourceType,
    /// Resource metric
    pub resource: Option<ResourceMetricSource>,
    /// Pods metric
    pub pods: Option<PodsMetricSource>,
    /// Object metric
    pub object: Option<ObjectMetricSource>,
    /// External metric
    pub external: Option<ExternalMetricSource>,
}

impl MetricSpec {
    /// Create default CPU utilization metric
    pub fn default_cpu_utilization(target_utilization: i32) -> Self {
        Self {
            type_field: MetricSourceType::Resource,
            resource: Some(ResourceMetricSource {
                name: "cpu".to_string(),
                target: MetricTarget {
                    type_field: MetricTargetType::Utilization,
                    value: None,
                    average_value: None,
                    average_utilization: Some(target_utilization),
                },
            }),
            pods: None,
            object: None,
            external: None,
        }
    }
}

/// Metric source type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MetricSourceType {
    Resource,
    Pods,
    Object,
    External,
    ContainerResource,
}

/// Resource metric source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetricSource {
    /// Resource name
    pub name: String,
    /// Target
    pub target: MetricTarget,
}

/// Pods metric source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodsMetricSource {
    /// Metric identifier
    pub metric: MetricIdentifier,
    /// Target
    pub target: MetricTarget,
}

/// Object metric source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetricSource {
    /// Described object
    pub described_object: CrossVersionObjectReference,
    /// Metric identifier
    pub metric: MetricIdentifier,
    /// Target
    pub target: MetricTarget,
}

/// External metric source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalMetricSource {
    /// Metric identifier
    pub metric: MetricIdentifier,
    /// Target
    pub target: MetricTarget,
}

/// Metric identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricIdentifier {
    /// Metric name
    pub name: String,
    /// Selector
    pub selector: Option<LabelSelector>,
}

/// Label selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelSelector {
    /// Match labels
    #[serde(rename = "matchLabels")]
    pub match_labels: HashMap<String, String>,
}

/// Metric target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTarget {
    /// Target type
    #[serde(rename = "type")]
    pub type_field: MetricTargetType,
    /// Target value
    pub value: Option<String>,
    /// Average value
    #[serde(rename = "averageValue")]
    pub average_value: Option<String>,
    /// Average utilization
    #[serde(rename = "averageUtilization")]
    pub average_utilization: Option<i32>,
}

/// Metric target type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MetricTargetType {
    Value,
    AverageValue,
    Utilization,
}

/// Horizontal Pod Autoscaler behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalPodAutoscalerBehavior {
    /// Scale down behavior
    #[serde(rename = "scaleDown")]
    pub scale_down: Option<HPAScalingRules>,
    /// Scale up behavior
    #[serde(rename = "scaleUp")]
    pub scale_up: Option<HPAScalingRules>,
}

/// HPA scaling rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HPAScalingRules {
    /// Stabilization window seconds
    #[serde(rename = "stabilizationWindowSeconds")]
    pub stabilization_window_seconds: Option<i32>,
    /// Select policies
    #[serde(rename = "selectPolicy")]
    pub select_policy: Option<String>,
    /// Scaling policies
    pub policies: Vec<HPAScalingPolicy>,
}

/// HPA scaling policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HPAScalingPolicy {
    /// Type
    #[serde(rename = "type")]
    pub type_field: HPAScalingPolicyType,
    /// Value
    pub value: i32,
    /// Period seconds
    #[serde(rename = "periodSeconds")]
    pub period_seconds: i32,
}

/// HPA scaling policy type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HPAScalingPolicyType {
    Pods,
    Percent,
}

/// Horizontal Pod Autoscaler status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HorizontalPodAutoscalerStatus {
    /// Observed generation
    #[serde(rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Last scale time
    #[serde(rename = "lastScaleTime")]
    pub last_scale_time: Option<String>,
    /// Current replicas
    #[serde(rename = "currentReplicas")]
    pub current_replicas: i32,
    /// Desired replicas
    #[serde(rename = "desiredReplicas")]
    pub desired_replicas: i32,
    /// Current metrics
    #[serde(rename = "currentMetrics")]
    pub current_metrics: Vec<MetricStatus>,
    /// Conditions
    pub conditions: Vec<HorizontalPodAutoscalerCondition>,
}

/// Metric status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatus {
    /// Metric type
    #[serde(rename = "type")]
    pub type_field: MetricSourceType,
    /// Resource metric
    pub resource: Option<ResourceMetricStatus>,
    /// Pods metric
    pub pods: Option<PodsMetricStatus>,
}

/// Resource metric status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetricStatus {
    /// Resource name
    pub name: String,
    /// Current value
    pub current: MetricValueStatus,
}

/// Pods metric status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodsMetricStatus {
    /// Metric identifier
    pub metric: MetricIdentifier,
    /// Current value
    pub current: MetricValueStatus,
}

/// Metric value status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValueStatus {
    /// Current value
    pub value: Option<String>,
    /// Average value
    #[serde(rename = "averageValue")]
    pub average_value: Option<String>,
    /// Average utilization
    #[serde(rename = "averageUtilization")]
    pub average_utilization: Option<i32>,
}

/// Horizontal Pod Autoscaler condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalPodAutoscalerCondition {
    /// Condition type
    #[serde(rename = "type")]
    pub type_field: String,
    /// Status
    pub status: String,
    /// Last transition time
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Reason
    pub reason: Option<String>,
    /// Message
    pub message: Option<String>,
}

/// Vertical Pod Autoscaler (VPA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalPodAutoscaler {
    /// VPA name
    pub name: String,
    /// Namespace
    pub namespace: String,
    /// Target resource
    #[serde(rename = "targetRef")]
    pub target_ref: CrossVersionObjectReference,
    /// Update policy
    #[serde(rename = "updatePolicy")]
    pub update_policy: Option<VPAUpdatePolicy>,
    /// Resource policy
    #[serde(rename = "resourcePolicy")]
    pub resource_policy: Option<VPAResourcePolicy>,
    /// VPA status
    pub status: Option<VPAStatus>,
}

impl VerticalPodAutoscaler {
    /// Create a new VPA
    pub fn new(name: impl Into<String>, target_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: "default".to_string(),
            target_ref: CrossVersionObjectReference {
                api_version: "apps/v1".to_string(),
                kind: "Deployment".to_string(),
                name: target_name.into(),
            },
            update_policy: Some(VPAUpdatePolicy::default()),
            resource_policy: None,
            status: None,
        }
    }

    /// Set update mode
    pub fn set_update_mode(&mut self, mode: VPAUpdateMode) -> &mut Self {
        self.update_policy = Some(VPAUpdatePolicy {
            update_mode: Some(mode),
        });
        self
    }

    /// Validate VPA configuration
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.name.is_empty() {
            return Err(CloudError::ValidationFailed("VPA name is required".to_string()));
        }
        if self.target_ref.name.is_empty() {
            return Err(CloudError::ValidationFailed("Target name is required".to_string()));
        }
        Ok(())
    }
}

/// VPA update policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPAUpdatePolicy {
    /// Update mode
    #[serde(rename = "updateMode")]
    pub update_mode: Option<VPAUpdateMode>,
}

impl Default for VPAUpdatePolicy {
    fn default() -> Self {
        Self {
            update_mode: Some(VPAUpdateMode::Auto),
        }
    }
}

/// VPA update mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VPAUpdateMode {
    Off,
    Initial,
    Recreate,
    Auto,
}

/// VPA resource policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPAResourcePolicy {
    /// Container policies
    #[serde(rename = "containerPolicies")]
    pub container_policies: Vec<VPAContainerResourcePolicy>,
}

/// VPA container resource policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPAContainerResourcePolicy {
    /// Container name
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Mode
    pub mode: Option<VPAContainerMode>,
    /// Min allowed resources
    #[serde(rename = "minAllowed")]
    pub min_allowed: Option<ResourceRequirements>,
    /// Max allowed resources
    #[serde(rename = "maxAllowed")]
    pub max_allowed: Option<ResourceRequirements>,
    /// Controlled resources
    #[serde(rename = "controlledResources")]
    pub controlled_resources: Option<Vec<String>>,
}

/// VPA container mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VPAContainerMode {
    Off,
    Auto,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU
    pub cpu: Option<String>,
    /// Memory
    pub memory: Option<String>,
}

/// VPA status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPAStatus {
    /// Conditions
    pub conditions: Vec<VPACondition>,
    /// Recommendation
    pub recommendation: Option<RecommendedPodResources>,
}

/// VPA condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPACondition {
    /// Condition type
    #[serde(rename = "type")]
    pub type_field: String,
    /// Status
    pub status: String,
    /// Last transition time
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Reason
    pub reason: Option<String>,
    /// Message
    pub message: Option<String>,
}

/// Recommended pod resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedPodResources {
    /// Container recommendations
    #[serde(rename = "containerRecommendations")]
    pub container_recommendations: Vec<RecommendedContainerResources>,
}

/// Recommended container resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedContainerResources {
    /// Container name
    #[serde(rename = "containerName")]
    pub container_name: String,
    /// Target resources
    pub target: ResourceRequirements,
    /// Lower bound
    #[serde(rename = "lowerBound")]
    pub lower_bound: ResourceRequirements,
    /// Upper bound
    #[serde(rename = "upperBound")]
    pub upper_bound: ResourceRequirements,
    /// Uncapped target
    #[serde(rename = "uncappedTarget")]
    pub uncapped_target: Option<ResourceRequirements>,
}

/// Cluster Autoscaler (CA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAutoscaler {
    /// CA name
    pub name: String,
    /// Minimum nodes
    #[serde(rename = "minNodes")]
    pub min_nodes: i32,
    /// Maximum nodes
    #[serde(rename = "maxNodes")]
    pub max_nodes: i32,
    /// Current nodes
    #[serde(rename = "currentNodes")]
    pub current_nodes: i32,
    /// Scale down enabled
    #[serde(rename = "scaleDownEnabled")]
    pub scale_down_enabled: bool,
    /// Scale down utilization threshold
    #[serde(rename = "scaleDownUtilizationThreshold")]
    pub scale_down_utilization_threshold: f64,
    /// Scale down unneeded time
    #[serde(rename = "scaleDownUnneededTime")]
    pub scale_down_unneeded_time: String,
    /// Balance similar node groups
    #[serde(rename = "balanceSimilarNodeGroups")]
    pub balance_similar_node_groups: bool,
    /// Skip nodes with system pods
    #[serde(rename = "skipNodesWithSystemPods")]
    pub skip_nodes_with_system_pods: bool,
    /// CA status
    pub status: ClusterAutoscalerStatus,
}

impl ClusterAutoscaler {
    /// Create a new cluster autoscaler
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min_nodes: 1,
            max_nodes: 10,
            current_nodes: 1,
            scale_down_enabled: true,
            scale_down_utilization_threshold: 0.5,
            scale_down_unneeded_time: "10m".to_string(),
            balance_similar_node_groups: false,
            skip_nodes_with_system_pods: true,
            status: ClusterAutoscalerStatus::default(),
        }
    }

    /// Set node range
    pub fn set_node_range(&mut self, min: i32, max: i32) -> &mut Self {
        self.min_nodes = min;
        self.max_nodes = max;
        self
    }

    /// Calculate desired nodes based on pending pods
    pub fn calculate_desired_nodes(&self, pending_pods: i32, pods_per_node: i32) -> i32 {
        if pending_pods == 0 || pods_per_node == 0 {
            return self.current_nodes;
        }
        
        let needed_nodes = (pending_pods + pods_per_node - 1) / pods_per_node;
        (self.current_nodes + needed_nodes).max(self.min_nodes).min(self.max_nodes)
    }

    /// Check if scale up is needed
    pub fn needs_scale_up(&self, pending_pods: i32, pods_per_node: i32) -> bool {
        let desired = self.calculate_desired_nodes(pending_pods, pods_per_node);
        desired > self.current_nodes
    }

    /// Check if scale down is needed
    pub fn needs_scale_down(&self, utilization: f64) -> bool {
        self.scale_down_enabled && utilization < self.scale_down_utilization_threshold
    }

    /// Validate CA configuration
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.min_nodes < 0 {
            return Err(CloudError::ValidationFailed("Min nodes must be non-negative".to_string()));
        }
        if self.max_nodes < self.min_nodes {
            return Err(CloudError::ValidationFailed("Max nodes must be >= min nodes".to_string()));
        }
        Ok(())
    }
}

/// Cluster Autoscaler status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterAutoscalerStatus {
    /// Health status
    pub health: ClusterAutoscalerHealth,
    /// Last scale up time
    #[serde(rename = "lastScaleUpTime")]
    pub last_scale_up_time: Option<String>,
    /// Last scale down time
    #[serde(rename = "lastScaleDownTime")]
    pub last_scale_down_time: Option<String>,
}

/// Cluster Autoscaler health
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterAutoscalerHealth {
    /// Healthy
    pub healthy: bool,
    /// Health status
    pub status: String,
    /// Last probe time
    #[serde(rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    /// Errors
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpa_creation() {
        let hpa = HorizontalPodAutoscaler::new("test-hpa", "test-deployment", "Deployment");
        assert_eq!(hpa.name, "test-hpa");
        assert_eq!(hpa.min_replicas, 1);
        assert_eq!(hpa.max_replicas, 10);
    }

    #[test]
    fn test_hpa_replica_calculation() {
        let hpa = HorizontalPodAutoscaler::new("test-hpa", "test-deployment", "Deployment");
        
        // CPU at 100% with target 80% should scale up
        let desired = hpa.calculate_desired_replicas(100.0, 80.0);
        assert!(desired > hpa.current_replicas);
    }

    #[test]
    fn test_hpa_validation() {
        let hpa = HorizontalPodAutoscaler::new("test-hpa", "test-deployment", "Deployment");
        assert!(hpa.validate().is_ok());
    }

    #[test]
    fn test_vpa_creation() {
        let vpa = VerticalPodAutoscaler::new("test-vpa", "test-deployment");
        assert_eq!(vpa.name, "test-vpa");
    }

    #[test]
    fn test_vpa_validation() {
        let vpa = VerticalPodAutoscaler::new("test-vpa", "test-deployment");
        assert!(vpa.validate().is_ok());
    }

    #[test]
    fn test_cluster_autoscaler_creation() {
        let ca = ClusterAutoscaler::new("test-ca");
        assert_eq!(ca.name, "test-ca");
        assert_eq!(ca.min_nodes, 1);
        assert_eq!(ca.max_nodes, 10);
    }

    #[test]
    fn test_cluster_autoscaler_scale_up() {
        let ca = ClusterAutoscaler::new("test-ca");
        
        // With 5 pending pods and 2 pods per node, need 3 more nodes
        assert!(ca.needs_scale_up(5, 2));
    }

    #[test]
    fn test_cluster_autoscaler_scale_down() {
        let ca = ClusterAutoscaler::new("test-ca");
        
        // With low utilization, should scale down
        assert!(ca.needs_scale_down(0.3));
    }
}