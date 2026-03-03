/// Kubernetes Pod Resource Management
/// 
/// This module handles Kubernetes Pod resources, which are the smallest deployable
/// units of computing that can be created and managed in Kubernetes.
/// 
/// Features:
/// - Pod lifecycle management
/// - Container configuration and management
/// - Resource limits and requests
/// - Health checks and probes
/// - Volume mounting
/// - Pod networking and DNS
/// - Pod scheduling and affinity

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{ObjectMeta, ResourceStatus, KubernetesError, ApiVersion};

/// Kubernetes Pod resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    /// Standard object's metadata
    pub metadata: ObjectMeta,
    /// Pod specification
    #[serde(rename = "spec")]
    pub spec: PodSpec,
    /// Pod status
    #[serde(rename = "status")]
    pub status: PodStatus,
    /// Kubernetes API version
    pub api_version: Option<ApiVersion>,
    /// Kind of object
    pub kind: Option<String>,
}

impl Pod {
    /// Create a new Pod with default configuration
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
            spec: PodSpec::default(),
            status: PodStatus::default(),
            api_version: Some(ApiVersion::V1),
            kind: Some("Pod".to_string()),
        }
    }

    /// Set the namespace for this Pod
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.metadata.namespace = Some(namespace.into());
        self
    }

    /// Add a container to the Pod
    pub fn add_container(&mut self, container: Container) -> &mut Self {
        self.spec.containers.push(container);
        self
    }

    /// Set the restart policy
    pub fn set_restart_policy(&mut self, policy: RestartPolicy) -> &mut Self {
        self.spec.restart_policy = policy;
        self
    }

    /// Set the service account name
    pub fn set_service_account(&mut self, name: impl Into<String>) -> &mut Self {
        self.spec.service_account_name = Some(name.into());
        self
    }

    /// Add a label to the Pod
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    /// Add an annotation to the Pod
    pub fn add_annotation(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.annotations.insert(key.into(), value.into());
        self
    }

    /// Check if the Pod is ready
    pub fn is_ready(&self) -> bool {
        self.status.conditions
            .iter()
            .any(|c| c.type_field == "Ready" && c.status == "True")
    }

    /// Check if the Pod is running
    pub fn is_running(&self) -> bool {
        self.status.phase == PodPhase::Running
    }

    /// Check if the Pod is scheduled
    pub fn is_scheduled(&self) -> bool {
        self.status.conditions
            .iter()
            .any(|c| c.type_field == "PodScheduled" && c.status == "True")
    }

    /// Get the Pod phase
    pub fn get_phase(&self) -> PodPhase {
        self.status.phase.clone()
    }

    /// Apply defaults to the Pod
    pub fn apply_defaults(&mut self) {
        if self.api_version.is_none() {
            self.api_version = Some(ApiVersion::V1);
        }
        if self.kind.is_none() {
            self.kind = Some("Pod".to_string());
        }
        if self.spec.restart_policy == RestartPolicy::Unknown {
            self.spec.restart_policy = RestartPolicy::Always;
        }
    }

    /// Validate the Pod configuration
    pub fn validate(&self) -> Result<(), KubernetesError> {
        if self.metadata.name.is_none() {
            return Err(KubernetesError::ValidationFailed("Pod name is required".to_string()));
        }
        if self.spec.containers.is_empty() {
            return Err(KubernetesError::ValidationFailed("Pod must have at least one container".to_string()));
        }
        Ok(())
    }
}

impl Default for Pod {
    fn default() -> Self {
        Self::new("default-pod")
    }
}

/// Pod specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodSpec {
    /// List of containers in the pod
    pub containers: Vec<Container>,
    /// List of init containers
    #[serde(rename = "initContainers")]
    pub init_containers: Option<Vec<Container>>,
    /// Restart policy for the pod
    #[serde(rename = "restartPolicy")]
    pub restart_policy: RestartPolicy,
    /// Service account name
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// Node selector
    #[serde(rename = "nodeSelector")]
    pub node_selector: Option<HashMap<String, String>>,
    /// Affinity rules
    pub affinity: Option<Affinity>,
    /// Tolerations
    pub tolerations: Option<Vec<Toleration>>,
    /// DNS policy
    #[serde(rename = "dnsPolicy")]
    pub dns_policy: Option<String>,
    /// DNS config
    #[serde(rename = "dnsConfig")]
    pub dns_config: Option<PodDNSConfig>,
    /// Host network
    #[serde(rename = "hostNetwork")]
    pub host_network: Option<bool>,
    /// Volumes
    pub volumes: Option<Vec<Volume>>,
}

/// Pod status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodStatus {
    /// Current phase of the pod
    pub phase: PodPhase,
    /// Conditions
    pub conditions: Vec<PodCondition>,
    /// Message explaining why pod is in this condition
    pub message: Option<String>,
    /// Reason explaining why pod is in this condition
    pub reason: Option<String>,
    /// IP address of the host
    #[serde(rename = "hostIP")]
    pub host_ip: Option<String>,
    /// IP address of the pod
    #[serde(rename = "podIP")]
    pub pod_ip: Option<String>,
    /// Container statuses
    #[serde(rename = "containerStatuses")]
    pub container_statuses: Option<Vec<ContainerStatus>>,
    /// Init container statuses
    #[serde(rename = "initContainerStatuses")]
    pub init_container_statuses: Option<Vec<ContainerStatus>>,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
}

/// Pod phase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PodPhase {
    /// Pod is pending
    Pending,
    /// Pod is running
    Running,
    /// Pod succeeded
    Succeeded,
    /// Pod failed
    Failed,
    /// Pod is unknown
    Unknown,
}

impl Default for PodPhase {
    fn default() -> Self {
        PodPhase::Pending
    }
}

/// Pod condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodCondition {
    /// Type of condition
    #[serde(rename = "type")]
    pub type_field: String,
    /// Status of the condition
    pub status: String,
    /// Last time the condition transitioned
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// Reason for the condition's last transition
    pub reason: Option<String>,
    /// Human-readable message indicating details about the transition
    pub message: Option<String>,
}

/// Container in a pod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    /// Name of the container
    pub name: String,
    /// Container image
    pub image: String,
    /// Container image pull policy
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Container command
    pub command: Option<Vec<String>>,
    /// Container args
    pub args: Option<Vec<String>>,
    /// Container working directory
    #[serde(rename = "workingDir")]
    pub working_dir: Option<String>,
    /// Container ports
    pub ports: Option<Vec<ContainerPort>>,
    /// Environment variables
    pub env: Option<Vec<EnvVar>>,
    /// Environment variables from configmaps/secrets
    #[serde(rename = "envFrom")]
    pub env_from: Option<Vec<EnvFromSource>>,
    /// Resource requirements
    pub resources: Option<ResourceRequirements>,
    /// Volume mounts
    #[serde(rename = "volumeMounts")]
    pub volume_mounts: Option<Vec<VolumeMount>>,
    /// Liveness probe
    #[serde(rename = "livenessProbe")]
    pub liveness_probe: Option<Probe>,
    /// Readiness probe
    #[serde(rename = "readinessProbe")]
    pub readiness_probe: Option<Probe>,
    /// Startup probe
    #[serde(rename = "startupProbe")]
    pub startup_probe: Option<Probe>,
    /// Lifecycle hooks
    pub lifecycle: Option<Lifecycle>,
    /// Termination message path
    #[serde(rename = "terminationMessagePath")]
    pub termination_message_path: Option<String>,
    /// Termination message policy
    #[serde(rename = "terminationMessagePolicy")]
    pub termination_message_policy: Option<String>,
}

impl Container {
    /// Create a new container
    pub fn new(name: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            image: image.into(),
            image_pull_policy: Some("IfNotPresent".to_string()),
            command: None,
            args: None,
            working_dir: None,
            ports: None,
            env: None,
            env_from: None,
            resources: None,
            volume_mounts: None,
            liveness_probe: None,
            readiness_probe: None,
            startup_probe: None,
            lifecycle: None,
            termination_message_path: Some("/dev/termination-log".to_string()),
            termination_message_policy: Some("File".to_string()),
        }
    }

    /// Add an environment variable
    pub fn add_env(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.env.get_or_insert_with(Vec::new).push(EnvVar {
            name: name.into(),
            value: Some(value.into()),
            value_from: None,
        });
        self
    }

    /// Add a port
    pub fn add_port(&mut self, container_port: i32) -> &mut Self {
        self.ports.get_or_insert_with(Vec::new).push(ContainerPort {
            container_port,
            name: None,
            host_port: None,
            protocol: Some("TCP".to_string()),
        });
        self
    }

    /// Set resource limits
    pub fn set_resources(&mut self, resources: ResourceRequirements) -> &mut Self {
        self.resources = Some(resources);
        self
    }

    /// Add a volume mount
    pub fn add_volume_mount(&mut self, name: impl Into<String>, mount_path: impl Into<String>) -> &mut Self {
        self.volume_mounts.get_or_insert_with(Vec::new).push(VolumeMount {
            name: name.into(),
            mount_path: mount_path.into(),
            read_only: None,
            sub_path: None,
        });
        self
    }
}

/// Container status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStatus {
    /// Name of the container
    pub name: String,
    /// Container state
    pub state: Option<ContainerState>,
    /// Last state
    #[serde(rename = "lastState")]
    pub last_state: Option<ContainerState>,
    /// Container ready status
    pub ready: bool,
    /// Restart count
    #[serde(rename = "restartCount")]
    pub restart_count: i32,
    /// Container image
    pub image: String,
    /// Container image ID
    #[serde(rename = "imageID")]
    pub image_id: String,
    /// Container ID
    #[serde(rename = "containerID")]
    pub container_id: Option<String>,
}

/// Container state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerState {
    /// Running state
    pub running: Option<ContainerStateRunning>,
    /// Waiting state
    pub waiting: Option<ContainerStateWaiting>,
    /// Terminated state
    pub terminated: Option<ContainerStateTerminated>,
}

/// Container state running
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateRunning {
    /// Time at which the container was last restarted
    #[serde(rename = "startedAt")]
    pub started_at: Option<String>,
}

/// Container state waiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateWaiting {
    /// Reason for waiting
    pub reason: Option<String>,
    /// Message about waiting
    pub message: Option<String>,
}

/// Container state terminated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStateTerminated {
    /// Exit status
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    /// Signal
    pub signal: Option<i32>,
    /// Reason for termination
    pub reason: Option<String>,
    /// Message about termination
    pub message: Option<String>,
    /// Time at which the container started
    #[serde(rename = "startedAt")]
    pub started_at: Option<String>,
    /// Time at which the container terminated
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<String>,
}

/// Container port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerPort {
    /// Container port number
    #[serde(rename = "containerPort")]
    pub container_port: i32,
    /// Port name
    pub name: Option<String>,
    /// Host port
    #[serde(rename = "hostPort")]
    pub host_port: Option<i32>,
    /// Protocol (TCP/UDP)
    pub protocol: Option<String>,
}

/// Environment variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: Option<String>,
    /// Variable source
    #[serde(rename = "valueFrom")]
    pub value_from: Option<EnvVarSource>,
}

/// Environment variable source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVarSource {
    /// ConfigMap key
    #[serde(rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ConfigMapKeySelector>,
    /// Secret key
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: Option<SecretKeySelector>,
}

/// ConfigMap key selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapKeySelector {
    /// ConfigMap name
    pub name: String,
    /// Key in ConfigMap
    pub key: String,
}

/// Secret key selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeySelector {
    /// Secret name
    pub name: String,
    /// Key in Secret
    pub key: String,
}

/// Environment from source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvFromSource {
    /// ConfigMap ref
    #[serde(rename = "configMapRef")]
    pub config_map_ref: Option<ConfigMapEnvSource>,
    /// Secret ref
    #[serde(rename = "secretRef")]
    pub secret_ref: Option<SecretEnvSource>,
}

/// ConfigMap env source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapEnvSource {
    /// ConfigMap name
    pub name: String,
}

/// Secret env source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretEnvSource {
    /// Secret name
    pub name: String,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Resource limits
    pub limits: Option<HashMap<String, String>>,
    /// Resource requests
    pub requests: Option<HashMap<String, String>>,
}

/// Volume mount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    /// Volume name
    pub name: String,
    /// Mount path
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    /// Read only
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Sub path
    #[serde(rename = "subPath")]
    pub sub_path: Option<String>,
}

/// Volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    /// Volume name
    pub name: String,
    /// EmptyDir volume
    #[serde(rename = "emptyDir")]
    pub empty_dir: Option<EmptyDirVolumeSource>,
    /// HostPath volume
    #[serde(rename = "hostPath")]
    pub host_path: Option<HostPathVolumeSource>,
    /// ConfigMap volume
    #[serde(rename = "configMap")]
    pub config_map: Option<ConfigMapVolumeSource>,
    /// Secret volume
    #[serde(rename = "secret")]
    pub secret: Option<SecretVolumeSource>,
    /// Persistent volume claim
    #[serde(rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<PersistentVolumeClaimVolumeSource>,
}

/// EmptyDir volume source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyDirVolumeSource {
    /// Medium of the volume
    pub medium: Option<String>,
    /// Size limit
    #[serde(rename = "sizeLimit")]
    pub size_limit: Option<String>,
}

/// HostPath volume source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostPathVolumeSource {
    /// Path on the host
    pub path: String,
    /// Type of host path
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// ConfigMap volume source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMapVolumeSource {
    /// ConfigMap name
    pub name: String,
}

/// Secret volume source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVolumeSource {
    /// Secret name
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// Persistent volume claim volume source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentVolumeClaimVolumeSource {
    /// Claim name
    #[serde(rename = "claimName")]
    pub claim_name: String,
    /// Read only
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

/// Probe for container health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Probe {
    /// Exec action
    pub exec: Option<ExecAction>,
    /// HTTP GET action
    #[serde(rename = "httpGet")]
    pub http_get: Option<HTTPGetAction>,
    /// TCP socket action
    #[serde(rename = "tcpSocket")]
    pub tcp_socket: Option<TCPSocketAction>,
    /// Initial delay seconds
    #[serde(rename = "initialDelaySeconds")]
    pub initial_delay_seconds: Option<i32>,
    /// Timeout seconds
    #[serde(rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
    /// Period seconds
    #[serde(rename = "periodSeconds")]
    pub period_seconds: Option<i32>,
    /// Success threshold
    #[serde(rename = "successThreshold")]
    pub success_threshold: Option<i32>,
    /// Failure threshold
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: Option<i32>,
}

/// Exec action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecAction {
    /// Command to execute
    pub command: Vec<String>,
}

/// HTTP get action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPGetAction {
    /// Path
    pub path: Option<String>,
    /// Port
    pub port: i32,
    /// Host
    pub host: Option<String>,
    /// Scheme
    pub scheme: Option<String>,
}

/// TCP socket action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPSocketAction {
    /// Port
    pub port: i32,
    /// Host
    pub host: Option<String>,
}

/// Lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifecycle {
    /// Pre-stop hook
    #[serde(rename = "preStop")]
    pub pre_stop: Option<Handler>,
    /// Post-start hook
    #[serde(rename = "postStart")]
    pub post_start: Option<Handler>,
}

/// Handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handler {
    /// Exec action
    pub exec: Option<ExecAction>,
    /// HTTP GET action
    #[serde(rename = "httpGet")]
    pub http_get: Option<HTTPGetAction>,
    /// TCP socket action
    #[serde(rename = "tcpSocket")]
    pub tcp_socket: Option<TCPSocketAction>,
}

/// Pod DNS config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodDNSConfig {
    /// DNS options
    pub options: Option<Vec<PodDNSConfigOption>>,
    /// DNS nameservers
    #[serde(rename = "nameservers")]
    pub nameservers: Option<Vec<String>>,
    /// DNS searches
    pub searches: Option<Vec<String>>,
}

/// Pod DNS config option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodDNSConfigOption {
    /// Option name
    pub name: String,
    /// Option value
    pub value: Option<String>,
}

/// Restart policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
    Unknown,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::Unknown
    }
}

/// Affinity rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affinity {
    /// Node affinity
    #[serde(rename = "nodeAffinity")]
    pub node_affinity: Option<NodeAffinity>,
    /// Pod affinity
    #[serde(rename = "podAffinity")]
    pub pod_affinity: Option<PodAffinity>,
    /// Pod anti-affinity
    #[serde(rename = "podAntiAffinity")]
    pub pod_anti_affinity: Option<PodAffinity>,
}

/// Node affinity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAffinity {
    /// Required node selector
    #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required: Option<NodeSelector>,
    /// Preferred node selectors
    #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred: Option<Vec<PreferredSchedulingTerm>>,
}

/// Node selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelector {
    /// Node selector terms
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<NodeSelectorTerm>,
}

/// Node selector term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelectorTerm {
    /// Match expressions
    #[serde(rename = "matchExpressions")]
    pub match_expressions: Vec<NodeSelectorRequirement>,
}

/// Node selector requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSelectorRequirement {
    /// Key
    pub key: String,
    /// Operator
    pub operator: String,
    /// Values
    pub values: Option<Vec<String>>,
}

/// Preferred scheduling term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredSchedulingTerm {
    /// Weight
    pub weight: i32,
    /// Preference
    pub preference: NodeSelectorTerm,
}

/// Pod affinity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodAffinity {
    /// Required pod affinity
    #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required: Option<Vec<PodAffinityTerm>>,
    /// Preferred pod affinity
    #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred: Option<Vec<WeightedPodAffinityTerm>>,
}

/// Pod affinity term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodAffinityTerm {
    /// Label selector
    #[serde(rename = "labelSelector")]
    pub label_selector: Option<LabelSelector>,
    /// Namespaces
    pub namespaces: Option<Vec<String>>,
    /// Topology key
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
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
    /// Key
    pub key: String,
    /// Operator
    pub operator: String,
    /// Values
    pub values: Option<Vec<String>>,
}

/// Weighted pod affinity term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedPodAffinityTerm {
    /// Weight
    pub weight: i32,
    /// Pod affinity term
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: PodAffinityTerm,
}

/// Toleration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toleration {
    /// Key
    pub key: Option<String>,
    /// Operator
    pub operator: Option<String>,
    /// Effect
    pub effect: Option<String>,
    /// Toleration seconds
    #[serde(rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_creation() {
        let pod = Pod::new("test-pod");
        assert_eq!(pod.metadata.name, Some("test-pod".to_string()));
    }

    #[test]
    fn test_container_creation() {
        let container = Container::new("test-container", "nginx:latest");
        assert_eq!(container.name, "test-container");
        assert_eq!(container.image, "nginx:latest");
    }

    #[test]
    fn test_container_env() {
        let mut container = Container::new("test-container", "nginx:latest");
        container.add_env("VAR1", "value1");
        container.add_env("VAR2", "value2");
        
        assert_eq!(container.env.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_pod_phase() {
        let mut pod = Pod::new("test-pod");
        assert_eq!(pod.get_phase(), PodPhase::Pending);
        
        pod.status.phase = PodPhase::Running;
        assert!(pod.is_running());
    }

    #[test]
    fn test_pod_validation() {
        let pod = Pod::new("test-pod");
        assert!(pod.validate().is_err()); // No containers
        
        let mut pod = Pod::new("test-pod");
        pod.add_container(Container::new("container", "nginx:latest"));
        assert!(pod.validate().is_ok());
    }
}