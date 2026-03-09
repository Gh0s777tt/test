/// Kubernetes Secret resource management
/// Handles Kubernetes Secret resources including authentication secrets, TLS certificates, and sensitive data
/// 
/// Secrets are used to store sensitive information such as passwords, OAuth tokens, and SSH keys.
/// This module provides comprehensive Secret management with support for different types of secrets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{ObjectMeta, ResourceStatus, KubernetesError, ApiVersion, apply_defaults};

/// Kubernetes Secret resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    /// Standard object's metadata
    pub metadata: ObjectMeta,
    /// Secret specification
    #[serde(rename = "spec")]
    pub spec: SecretSpec,
    /// Secret status
    #[serde(rename = "status")]
    pub status: Option<SecretStatus>,
    /// Kubernetes API version
    pub api_version: Option<ApiVersion>,
    /// Kind of object
    pub kind: Option<String>,
}

impl Secret {
    /// Create a new Secret with default configuration
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
            spec: SecretSpec::default(),
            status: None,
            api_version: Some(ApiVersion::V1),
            kind: Some("Secret".to_string()),
        }
    }

    /// Set the namespace for this Secret
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.metadata.namespace = Some(namespace.into());
        self
    }

    /// Add data to the Secret (base64 encoded)
    pub fn add_data(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.spec.data.insert(key.into(), value.into());
        self
    }

    /// Get data from the Secret
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.spec.data.get(key)
    }

    /// Add string data to the Secret (automatically base64 encoded)
    pub fn add_string_data(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.spec.string_data.insert(key.into(), value.into());
        self
    }

    /// Get string data from the Secret
    pub fn get_string_data(&self, key: &str) -> Option<&String> {
        self.spec.string_data.get(key)
    }

    /// Set the Secret type
    pub fn set_type(&mut self, secret_type: impl Into<String>) -> &mut Self {
        self.spec.type_field = Some(secret_type.into());
        self
    }

    /// Make the Secret immutable
    pub fn set_immutable(&mut self, immutable: bool) -> &mut Self {
        self.spec.immutable = Some(immutable);
        self
    }

    /// Add a label to the Secret
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.labels.insert(key.into(), value.into());
        self
    }

    /// Add an annotation to the Secret
    pub fn add_annotation(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.annotations.insert(key.into(), value.into());
        self
    }

    /// Apply defaults to the Secret
    pub fn apply_defaults(&mut self) {
        if self.api_version.is_none() {
            self.api_version = Some(ApiVersion::V1);
        }
        if self.kind.is_none() {
            self.kind = Some("Secret".to_string());
        }
        if self.spec.type_field.is_none() {
            self.spec.type_field = Some("Opaque".to_string());
        }
    }

    /// Validate the Secret configuration
    pub fn validate(&self) -> Result<(), KubernetesError> {
        if self.metadata.name.is_none() {
            return Err(KubernetesError::ValidationFailed("Secret name is required".to_string()));
        }
        if self.spec.data.is_empty() && self.spec.string_data.is_empty() {
            return Err(KubernetesError::ValidationFailed("Secret must contain data or string_data".to_string()));
        }
        Ok(())
    }
}

impl Default for Secret {
    fn default() -> Self {
        Self::new("default-secret")
    }
}

/// Secret specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretSpec {
    /// Secret data (base64 encoded)
    pub data: HashMap<String, String>,
    /// Secret string data (not base64 encoded)
    pub string_data: HashMap<String, String>,
    /// Secret type (e.g., "Opaque", "kubernetes.io/tls", "kubernetes.io/dockerconfigjson")
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    /// Whether the Secret is immutable
    pub immutable: Option<bool>,
}

/// Secret status (mostly informational)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretStatus {
    /// List of secrets managed by this controller
    #[serde(rename = "secrets")]
    pub managed_secrets: Vec<String>,
}

/// Common Secret types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecretType {
    /// Arbitrary user-defined data
    Opaque,
    /// TLS certificate and key
    Tls,
    /// Docker registry credentials
    DockerConfigJson,
    /// Service account token
    ServiceAccountToken,
    /// Basic authentication
    BasicAuth,
    /// SSH authentication
    SshAuth,
    /// Bootstrap token
    BootstrapToken,
}

impl SecretType {
    /// Convert to string representation
    pub fn as_str(&self) -> &str {
        match self {
            SecretType::Opaque => "Opaque",
            SecretType::Tls => "kubernetes.io/tls",
            SecretType::DockerConfigJson => "kubernetes.io/dockerconfigjson",
            SecretType::ServiceAccountToken => "kubernetes.io/service-account-token",
            SecretType::BasicAuth => "kubernetes.io/basic-auth",
            SecretType::SshAuth => "kubernetes.io/ssh-auth",
            SecretType::BootstrapToken => "bootstrap.kubernetes.io/token",
        }
    }

    /// Convert from string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Opaque" => Some(SecretType::Opaque),
            "kubernetes.io/tls" => Some(SecretType::Tls),
            "kubernetes.io/dockerconfigjson" => Some(SecretType::DockerConfigJson),
            "kubernetes.io/service-account-token" => Some(SecretType::ServiceAccountToken),
            "kubernetes.io/basic-auth" => Some(SecretType::BasicAuth),
            "kubernetes.io/ssh-auth" => Some(SecretType::SshAuth),
            "bootstrap.kubernetes.io/token" => Some(SecretType::BootstrapToken),
            _ => None,
        }
    }
}

impl Default for SecretType {
    fn default() -> Self {
        SecretType::Opaque
    }
}

/// TLS Secret builder for certificate management
pub struct TlsSecretBuilder {
    secret: Secret,
}

impl TlsSecretBuilder {
    /// Create a new TLS Secret builder
    pub fn new(name: impl Into<String>) -> Self {
        let mut secret = Secret::new(name);
        secret.set_type(SecretType::Tls.as_str());
        Self { secret }
    }

    /// Set TLS certificate
    pub fn set_certificate(&mut self, certificate: impl Into<String>) -> &mut Self {
        self.secret.add_data("tls.crt", certificate.into());
        self
    }

    /// Set TLS key
    pub fn set_key(&mut self, key: impl Into<String>) -> &mut Self {
        self.secret.add_data("tls.key", key.into());
        self
    }

    /// Build the TLS Secret
    pub fn build(self) -> Secret {
        self.secret
    }
}

/// Docker registry secret builder
pub struct DockerRegistrySecretBuilder {
    secret: Secret,
}

impl DockerRegistrySecretBuilder {
    /// Create a new Docker registry secret builder
    pub fn new(name: impl Into<String>) -> Self {
        let mut secret = Secret::new(name);
        secret.set_type(SecretType::DockerConfigJson.as_str());
        Self { secret }
    }

    /// Set Docker registry credentials
    pub fn set_credentials(&mut self, registry: impl Into<String>, username: impl Into<String>, password: impl Into<String>) -> &mut Self {
        let auth = format!("{}:{}", username.into(), password.into());
        let encoded = base64::encode(&auth);
        let config = serde_json::json!({
            "auths": {
                registry.into(): {
                    "auth": encoded,
                    "username": username,
                    "password": password,
                    "auth": encoded
                }
            }
        });
        self.secret.add_string_data(".dockerconfigjson", config.to_string());
        self
    }

    /// Build the Docker registry secret
    pub fn build(self) -> Secret {
        self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_creation() {
        let secret = Secret::new("test-secret");
        assert_eq!(secret.metadata.name, Some("test-secret".to_string()));
    }

    #[test]
    fn test_secret_data() {
        let mut secret = Secret::new("test-secret");
        secret.add_data("username", "YWRtaW4="); // "admin" base64 encoded
        secret.add_data("password", "cGFzc3dvcmQ="); // "password" base64 encoded
        
        assert_eq!(secret.get_data("username"), Some(&"YWRtaW4=".to_string()));
        assert_eq!(secret.get_data("password"), Some(&"cGFzc3dvcmQ=".to_string()));
    }

    #[test]
    fn test_secret_string_data() {
        let mut secret = Secret::new("test-secret");
        secret.add_string_data("username", "admin");
        secret.add_string_data("password", "password");
        
        assert_eq!(secret.get_string_data("username"), Some(&"admin".to_string()));
        assert_eq!(secret.get_string_data("password"), Some(&"password".to_string()));
    }

    #[test]
    fn test_secret_type() {
        let mut secret = Secret::new("test-secret");
        secret.set_type(SecretType::Tls.as_str());
        
        assert_eq!(secret.spec.type_field, Some("kubernetes.io/tls".to_string()));
    }

    #[test]
    fn test_tls_secret_builder() {
        let mut builder = TlsSecretBuilder::new("tls-secret");
        builder.set_certificate("-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----");
        builder.set_key("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----");
        let secret = builder.build();
        
        assert!(secret.get_data("tls.crt").is_some());
        assert!(secret.get_data("tls.key").is_some());
    }

    #[test]
    fn test_docker_registry_secret_builder() {
        let mut builder = DockerRegistrySecretBuilder::new("docker-secret");
        builder.set_credentials("docker.io", "username", "password");
        let secret = builder.build();
        
        assert!(secret.get_string_data(".dockerconfigjson").is_some());
    }

    #[test]
    fn test_secret_validation() {
        let secret = Secret::new("test-secret");
        assert!(secret.validate().is_err()); // No data
        
        let mut secret = Secret::new("test-secret");
        secret.add_data("key", "value");
        assert!(secret.validate().is_ok());
    }
}