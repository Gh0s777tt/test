// Model Versioning System for VantisOS
// Comprehensive model version management with provenance tracking

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Model version identifier
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionId {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
    /// Pre-release tag
    pub pre_release: Option<String>,
    /// Build metadata
    pub build: Option<String>,
}

impl VersionId {
    /// Create a new version ID
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        VersionId {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }

    /// Parse version from string
    pub fn parse(version: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid version format".to_string());
        }

        let major = parts[0].parse::<u32>().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse::<u32>().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse::<u32>().map_err(|_| "Invalid patch version")?;

        Ok(VersionId::new(major, minor, patch))
    }

    /// Increment major version
    pub fn increment_major(&self) -> Self {
        VersionId::new(self.major + 1, 0, 0)
    }

    /// Increment minor version
    pub fn increment_minor(&self) -> Self {
        VersionId::new(self.major, self.minor + 1, 0)
    }

    /// Increment patch version
    pub fn increment_patch(&self) -> Self {
        VersionId::new(self.major, self.minor, self.patch + 1)
    }
}

impl std::fmt::Display for VersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for VersionId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.major.cmp(&other.major) {
            std::cmp::Ordering::Equal => {
                match self.minor.cmp(&other.minor) {
                    std::cmp::Ordering::Equal => self.patch.cmp(&other.patch),
                    other => other,
                }
            }
            other => other,
        }
    }
}

/// Model metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    /// Model description
    pub description: String,
    /// Model author
    pub author: String,
    /// Model tags
    pub tags: Vec<String>,
    /// Framework (PyTorch, TensorFlow, etc.)
    pub framework: String,
    /// Input shape
    pub input_shape: Vec<usize>,
    /// Output shape
    pub output_shape: Vec<usize>,
    /// Number of parameters
    pub num_parameters: usize,
    /// Model checksum (SHA256)
    pub checksum: String,
}

/// Model version
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    /// Version ID
    pub version: VersionId,
    /// Model metadata
    pub metadata: ModelMetadata,
    /// Model weights path
    pub weights_path: String,
    /// Model configuration
    pub config: HashMap<String, String>,
    /// Performance metrics
    pub metrics: HashMap<String, f64>,
    /// Training metrics
    pub training_metrics: HashMap<String, f64>,
    /// Dependencies
    pub dependencies: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,
    /// Parent version (for lineage tracking)
    pub parent: Option<VersionId>,
    /// Tags
    pub tags: Vec<String>,
    /// Aliases (e.g., "latest", "production")
    pub aliases: Vec<String>,
    /// Is deprecated
    pub deprecated: bool,
}

impl ModelVersion {
    /// Create a new model version
    pub fn new(version: VersionId, metadata: ModelMetadata) -> Self {
        ModelVersion {
            version,
            metadata,
            weights_path: String::new(),
            config: HashMap::new(),
            metrics: HashMap::new(),
            training_metrics: HashMap::new(),
            dependencies: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            parent: None,
            tags: Vec::new(),
            aliases: Vec::new(),
            deprecated: false,
        }
    }

    /// Add an alias
    pub fn add_alias(&mut self, alias: String) {
        if !self.aliases.contains(&alias) {
            self.aliases.push(alias);
        }
    }

    /// Remove an alias
    pub fn remove_alias(&mut self, alias: &str) {
        self.aliases.retain(|a| a != alias);
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Set performance metric
    pub fn set_metric(&mut self, name: String, value: f64) {
        self.metrics.insert(name, value);
        self.updated_at = Utc::now();
    }

    /// Set training metric
    pub fn set_training_metric(&mut self, name: String, value: f64) {
        self.training_metrics.insert(name, value);
    }

    /// Deprecate this version
    pub fn deprecate(&mut self) {
        self.deprecated = true;
        self.updated_at = Utc::now();
    }
}

/// Model registry
#[derive(Clone, Debug)]
pub struct ModelRegistry {
    /// Registry name
    name: String,
    /// Model versions
    versions: HashMap<VersionId, ModelVersion>,
    /// Alias mappings
    aliases: HashMap<String, VersionId>,
    /// Default version
    default_version: Option<VersionId>,
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new(name: String) -> Self {
        ModelRegistry {
            name,
            versions: HashMap::new(),
            aliases: HashMap::new(),
            default_version: None,
        }
    }

    /// Register a model version
    pub fn register(&mut self, version: ModelVersion) -> Result<(), String> {
        let version_id = version.version.clone();
        
        // Check if version already exists
        if self.versions.contains_key(&version_id) {
            return Err(format!("Version {} already exists", version_id));
        }

        // Register aliases
        for alias in &version.aliases {
            self.aliases.insert(alias.clone(), version_id.clone());
        }

        // Set as default if first version
        if self.versions.is_empty() {
            self.default_version = Some(version_id.clone());
        }

        self.versions.insert(version_id, version);
        Ok(())
    }

    /// Get a version by ID
    pub fn get_version(&self, version: &VersionId) -> Option<&ModelVersion> {
        self.versions.get(version)
    }

    /// Get a version by alias
    pub fn get_by_alias(&self, alias: &str) -> Option<&ModelVersion> {
        self.aliases.get(alias).and_then(|v| self.versions.get(v))
    }

    /// Get the latest version
    pub fn get_latest(&self) -> Option<&ModelVersion> {
        self.versions.keys().max().and_then(|v| self.versions.get(v))
    }

    /// Get the default version
    pub fn get_default(&self) -> Option<&ModelVersion> {
        self.default_version.as_ref().and_then(|v| self.versions.get(v))
    }

    /// List all versions
    pub fn list_versions(&self) -> Vec<&ModelVersion> {
        let mut versions: Vec<_> = self.versions.values().collect();
        versions.sort_by(|a, b| b.version.cmp(&a.version));
        versions
    }

    /// List versions by tag
    pub fn list_by_tag(&self, tag: &str) -> Vec<&ModelVersion> {
        self.versions.values()
            .filter(|v| v.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Deprecate a version
    pub fn deprecate(&mut self, version: &VersionId) -> Result<(), String> {
        if let Some(v) = self.versions.get_mut(version) {
            v.deprecate();
            Ok(())
        } else {
            Err(format!("Version {} not found", version))
        }
    }

    /// Delete a version
    pub fn delete(&mut self, version: &VersionId) -> Result<ModelVersion, String> {
        if let Some(v) = self.versions.remove(version) {
            // Remove aliases
            for alias in &v.aliases {
                self.aliases.remove(alias);
            }
            Ok(v)
        } else {
            Err(format!("Version {} not found", version))
        }
    }

    /// Set default version
    pub fn set_default(&mut self, version: &VersionId) -> Result<(), String> {
        if self.versions.contains_key(version) {
            self.default_version = Some(version.clone());
            Ok(())
        } else {
            Err(format!("Version {} not found", version))
        }
    }
}

/// Version manager
#[derive(Clone, Debug)]
pub struct VersionManager {
    registries: HashMap<String, ModelRegistry>,
}

impl VersionManager {
    /// Create a new version manager
    pub fn new() -> Self {
        VersionManager {
            registries: HashMap::new(),
        }
    }

    /// Create a new registry
    pub fn create_registry(&mut self, name: String) -> Result<(), String> {
        if self.registries.contains_key(&name) {
            return Err(format!("Registry {} already exists", name));
        }
        self.registries.insert(name.clone(), ModelRegistry::new(name));
        Ok(())
    }

    /// Get a registry
    pub fn get_registry(&self, name: &str) -> Option<&ModelRegistry> {
        self.registries.get(name)
    }

    /// Get a mutable registry
    pub fn get_registry_mut(&mut self, name: &str) -> Option<&mut ModelRegistry> {
        self.registries.get_mut(name)
    }

    /// List all registries
    pub fn list_registries(&self) -> Vec<&String> {
        self.registries.keys().collect()
    }

    /// Register a model version
    pub fn register(&mut self, registry: &str, version: ModelVersion) -> Result<(), String> {
        if let Some(reg) = self.registries.get_mut(registry) {
            reg.register(version)
        } else {
            Err(format!("Registry {} not found", registry))
        }
    }

    /// Get model version
    pub fn get_version(&self, registry: &str, version: &VersionId) -> Option<&ModelVersion> {
        self.registries.get(registry).and_then(|r| r.get_version(version))
    }

    /// Compare two versions
    pub fn compare(&self, registry: &str, v1: &VersionId, v2: &VersionId) -> Option<std::cmp::Ordering> {
        Some(v1.cmp(v2))
    }

    /// Get version lineage
    pub fn get_lineage(&self, registry: &str, version: &VersionId) -> Vec<VersionId> {
        let mut lineage = Vec::new();
        let mut current = Some(version.clone());

        while let Some(v) = current {
            if let Some(reg) = self.registries.get(registry) {
                if let Some(mv) = reg.get_version(&v) {
                    lineage.push(v.clone());
                    current = mv.parent.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        lineage
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_id() {
        let v1 = VersionId::new(1, 0, 0);
        let v2 = VersionId::new(1, 0, 1);
        let v3 = VersionId::new(1, 1, 0);
        let v4 = VersionId::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
    }

    #[test]
    fn test_version_parse() {
        let v = VersionId::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_increment() {
        let v = VersionId::new(1, 2, 3);
        assert_eq!(v.increment_major(), VersionId::new(2, 0, 0));
        assert_eq!(v.increment_minor(), VersionId::new(1, 3, 0));
        assert_eq!(v.increment_patch(), VersionId::new(1, 2, 4));
    }

    #[test]
    fn test_model_version() {
        let metadata = ModelMetadata {
            name: "test_model".to_string(),
            description: "Test model".to_string(),
            author: "test".to_string(),
            tags: vec!["test".to_string()],
            framework: "pytorch".to_string(),
            input_shape: vec![1, 28, 28],
            output_shape: vec![10],
            num_parameters: 1000,
            checksum: "abc123".to_string(),
        };

        let version = ModelVersion::new(VersionId::new(1, 0, 0), metadata);
        assert_eq!(version.version.major, 1);
    }

    #[test]
    fn test_model_registry() {
        let mut registry = ModelRegistry::new("test".to_string());
        
        let metadata = ModelMetadata {
            name: "test_model".to_string(),
            description: "Test".to_string(),
            author: "test".to_string(),
            tags: vec![],
            framework: "pytorch".to_string(),
            input_shape: vec![],
            output_shape: vec![],
            num_parameters: 0,
            checksum: "".to_string(),
        };

        let version = ModelVersion::new(VersionId::new(1, 0, 0), metadata);
        assert!(registry.register(version).is_ok());
        assert!(registry.get_latest().is_some());
    }

    #[test]
    fn test_version_manager() {
        let mut manager = VersionManager::new();
        assert!(manager.create_registry("models".to_string()).is_ok());
        assert!(manager.get_registry("models").is_some());
    }

    #[test]
    fn test_version_lineage() {
        let mut manager = VersionManager::new();
        manager.create_registry("models".to_string()).unwrap();

        let metadata = ModelMetadata {
            name: "test".to_string(),
            description: "".to_string(),
            author: "test".to_string(),
            tags: vec![],
            framework: "pytorch".to_string(),
            input_shape: vec![],
            output_shape: vec![],
            num_parameters: 0,
            checksum: "".to_string(),
        };

        // Register first version
        let v1 = ModelVersion::new(VersionId::new(1, 0, 0), metadata.clone());
        manager.register("models", v1).unwrap();

        // Register second version with parent
        let mut v2 = ModelVersion::new(VersionId::new(1, 1, 0), metadata);
        v2.parent = Some(VersionId::new(1, 0, 0));
        manager.register("models", v2).unwrap();

        let lineage = manager.get_lineage("models", &VersionId::new(1, 1, 0));
        assert_eq!(lineage.len(), 2);
    }
}