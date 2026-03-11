//! Build System Configuration for VANTIS OS
//!
//! Manages build targets, dependency resolution, and compilation
//! configuration for the VANTIS OS kernel and modules. Supports
//! topological dependency sorting and parallel build scheduling.

use core::fmt;

// ============================================================================
// Build Types
// ============================================================================

/// Build target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    X86_64,
    Aarch64,
    RiscV64,
    Wasm32,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::X86_64 => write!(f, "x86_64"),
            Architecture::Aarch64 => write!(f, "aarch64"),
            Architecture::RiscV64 => write!(f, "riscv64"),
            Architecture::Wasm32 => write!(f, "wasm32"),
        }
    }
}

/// Build optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    /// No optimization (debug)
    Debug,
    /// Basic optimization
    O1,
    /// Full optimization
    O2,
    /// Maximum optimization with LTO
    O3,
    /// Size optimization
    Os,
}

impl fmt::Display for OptLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptLevel::Debug => write!(f, "debug"),
            OptLevel::O1 => write!(f, "O1"),
            OptLevel::O2 => write!(f, "O2"),
            OptLevel::O3 => write!(f, "O3"),
            OptLevel::Os => write!(f, "Os"),
        }
    }
}

/// Build target type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    /// Kernel image
    Kernel,
    /// Kernel module / driver
    Module,
    /// Static library
    StaticLib,
    /// Shared library
    SharedLib,
    /// User-space binary
    Binary,
    /// Test suite
    Test,
    /// Benchmark
    Benchmark,
}

/// Build status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStatus {
    Pending,
    Building,
    Success,
    Failed,
    Skipped,
}

impl fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildStatus::Pending => write!(f, "PENDING"),
            BuildStatus::Building => write!(f, "BUILDING"),
            BuildStatus::Success => write!(f, "SUCCESS"),
            BuildStatus::Failed => write!(f, "FAILED"),
            BuildStatus::Skipped => write!(f, "SKIPPED"),
        }
    }
}

// ============================================================================
// Build Target
// ============================================================================

/// A build target with dependencies
#[derive(Debug, Clone)]
pub struct BuildTarget {
    pub name: String,
    pub target_type: TargetType,
    pub source_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub features: Vec<String>,
    pub architecture: Architecture,
    pub opt_level: OptLevel,
    pub status: BuildStatus,
    pub build_time_ms: Option<u64>,
    pub output_path: Option<String>,
}

impl BuildTarget {
    pub fn new(name: &str, target_type: TargetType, arch: Architecture) -> Self {
        Self {
            name: name.to_string(),
            target_type,
            source_files: Vec::new(),
            dependencies: Vec::new(),
            features: Vec::new(),
            architecture: arch,
            opt_level: OptLevel::O2,
            status: BuildStatus::Pending,
            build_time_ms: None,
            output_path: None,
        }
    }

    /// Add a source file
    pub fn add_source(&mut self, path: &str) {
        self.source_files.push(path.to_string());
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, dep: &str) {
        if !self.dependencies.contains(&dep.to_string()) {
            self.dependencies.push(dep.to_string());
        }
    }

    /// Add a feature flag
    pub fn add_feature(&mut self, feature: &str) {
        if !self.features.contains(&feature.to_string()) {
            self.features.push(feature.to_string());
        }
    }

    /// Check if all dependencies are satisfied
    pub fn deps_satisfied(&self, completed: &[String]) -> bool {
        self.dependencies.iter().all(|d| completed.contains(d))
    }

    /// Number of source files
    pub fn source_count(&self) -> usize {
        self.source_files.len()
    }
}

// ============================================================================
// Build Configuration
// ============================================================================

/// Global build configuration
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub project_name: String,
    pub version: String,
    pub default_arch: Architecture,
    pub default_opt: OptLevel,
    pub parallel_jobs: usize,
    pub enable_lto: bool,
    pub enable_verification: bool,
    pub enable_tests: bool,
    pub extra_flags: Vec<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project_name: "vantis-os".to_string(),
            version: "1.6.0".to_string(),
            default_arch: Architecture::X86_64,
            default_opt: OptLevel::O2,
            parallel_jobs: 4,
            enable_lto: true,
            enable_verification: true,
            enable_tests: true,
            extra_flags: Vec::new(),
        }
    }
}

// ============================================================================
// Build System
// ============================================================================

/// Error types for the build system
#[derive(Debug, Clone, PartialEq)]
pub enum BuildError {
    TargetNotFound(String),
    DuplicateTarget(String),
    CyclicDependency(Vec<String>),
    UnsatisfiedDependency { target: String, missing: String },
    BuildFailed { target: String, reason: String },
    NoTargets,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::TargetNotFound(name) => write!(f, "Target not found: {}", name),
            BuildError::DuplicateTarget(name) => write!(f, "Duplicate target: {}", name),
            BuildError::CyclicDependency(cycle) => write!(f, "Cyclic dependency: {:?}", cycle),
            BuildError::UnsatisfiedDependency { target, missing } => {
                write!(f, "Target {} depends on missing target {}", target, missing)
            }
            BuildError::BuildFailed { target, reason } => {
                write!(f, "Build failed for {}: {}", target, reason)
            }
            BuildError::NoTargets => write!(f, "No build targets defined"),
        }
    }
}

/// Build result summary
#[derive(Debug, Clone)]
pub struct BuildSummary {
    pub total_targets: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
    pub total_time_ms: u64,
    pub build_order: Vec<String>,
}

impl BuildSummary {
    /// Whether the entire build succeeded
    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    /// Success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_targets == 0 {
            return 100.0;
        }
        self.succeeded as f64 / self.total_targets as f64 * 100.0
    }
}

/// The main build system
pub struct BuildSystem {
    config: BuildConfig,
    targets: Vec<BuildTarget>,
}

impl BuildSystem {
    /// Create a new build system
    pub fn new(config: BuildConfig) -> Self {
        Self {
            config,
            targets: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(BuildConfig::default())
    }

    /// Add a build target
    pub fn add_target(&mut self, target: BuildTarget) -> Result<(), BuildError> {
        if self.targets.iter().any(|t| t.name == target.name) {
            return Err(BuildError::DuplicateTarget(target.name));
        }
        self.targets.push(target);
        Ok(())
    }

    /// Remove a build target
    pub fn remove_target(&mut self, name: &str) -> Result<(), BuildError> {
        let idx = self.targets.iter().position(|t| t.name == name)
            .ok_or_else(|| BuildError::TargetNotFound(name.to_string()))?;
        self.targets.remove(idx);
        Ok(())
    }

    /// Get the number of targets
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Validate all dependencies exist
    pub fn validate_dependencies(&self) -> Result<(), BuildError> {
        let target_names: Vec<&str> = self.targets.iter().map(|t| t.name.as_str()).collect();
        for target in &self.targets {
            for dep in &target.dependencies {
                if !target_names.contains(&dep.as_str()) {
                    return Err(BuildError::UnsatisfiedDependency {
                        target: target.name.clone(),
                        missing: dep.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Compute topological build order using Kahn's algorithm
    pub fn compute_build_order(&self) -> Result<Vec<String>, BuildError> {
        if self.targets.is_empty() {
            return Err(BuildError::NoTargets);
        }

        self.validate_dependencies()?;

        let n = self.targets.len();
        let name_to_idx: std::collections::HashMap<&str, usize> = self.targets.iter()
            .enumerate()
            .map(|(i, t)| (t.name.as_str(), i))
            .collect();

        // Compute in-degrees
        let mut in_degree = vec![0usize; n];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

        for (i, target) in self.targets.iter().enumerate() {
            for dep in &target.dependencies {
                if let Some(&dep_idx) = name_to_idx.get(dep.as_str()) {
                    adj[dep_idx].push(i);
                    in_degree[i] += 1;
                }
            }
        }

        // Kahn's algorithm
        let mut queue: Vec<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::new();

        while let Some(node) = queue.pop() {
            order.push(self.targets[node].name.clone());
            for &neighbor in &adj[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push(neighbor);
                }
            }
        }

        if order.len() != n {
            // Cycle detected - find the cycle
            let remaining: Vec<String> = (0..n)
                .filter(|&i| in_degree[i] > 0)
                .map(|i| self.targets[i].name.clone())
                .collect();
            return Err(BuildError::CyclicDependency(remaining));
        }

        Ok(order)
    }

    /// Execute the build (simulated)
    pub fn build(&mut self) -> Result<BuildSummary, BuildError> {
        let build_order = self.compute_build_order()?;
        let mut completed: Vec<String> = Vec::new();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut skipped = 0;
        let mut total_time: u64 = 0;

        for target_name in &build_order {
            let target = self.targets.iter_mut()
                .find(|t| t.name == *target_name)
                .unwrap();

            // Check if dependencies are satisfied
            if !target.deps_satisfied(&completed) {
                target.status = BuildStatus::Skipped;
                skipped += 1;
                continue;
            }

            target.status = BuildStatus::Building;

            // Simulate build time based on source count and opt level
            let base_time = (target.source_count() as u64 + 1) * 100;
            let opt_multiplier = match target.opt_level {
                OptLevel::Debug => 1,
                OptLevel::O1 => 2,
                OptLevel::O2 => 3,
                OptLevel::O3 => 5,
                OptLevel::Os => 4,
            };
            let build_time = base_time * opt_multiplier;

            target.build_time_ms = Some(build_time);
            target.status = BuildStatus::Success;
            target.output_path = Some(format!("build/{}/{}", target.architecture, target.name));
            total_time += build_time;
            succeeded += 1;
            completed.push(target_name.clone());
        }

        Ok(BuildSummary {
            total_targets: self.targets.len(),
            succeeded,
            failed,
            skipped,
            total_time_ms: total_time,
            build_order,
        })
    }

    /// Get a target by name
    pub fn get_target(&self, name: &str) -> Option<&BuildTarget> {
        self.targets.iter().find(|t| t.name == name)
    }

    /// Get the build configuration
    pub fn config(&self) -> &BuildConfig {
        &self.config
    }

    /// Get all targets with a specific status
    pub fn targets_with_status(&self, status: BuildStatus) -> Vec<&BuildTarget> {
        self.targets.iter().filter(|t| t.status == status).collect()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_build_system() -> BuildSystem {
        let mut bs = BuildSystem::with_defaults();

        let mut core = BuildTarget::new("core", TargetType::StaticLib, Architecture::X86_64);
        core.add_source("src/core/main.rs");
        core.add_source("src/core/types.rs");
        bs.add_target(core).unwrap();

        let mut memory = BuildTarget::new("memory", TargetType::StaticLib, Architecture::X86_64);
        memory.add_source("src/memory/allocator.rs");
        memory.add_dependency("core");
        bs.add_target(memory).unwrap();

        let mut scheduler = BuildTarget::new("scheduler", TargetType::Module, Architecture::X86_64);
        scheduler.add_source("src/scheduler/mod.rs");
        scheduler.add_dependency("core");
        scheduler.add_dependency("memory");
        bs.add_target(scheduler).unwrap();

        let mut kernel = BuildTarget::new("kernel", TargetType::Kernel, Architecture::X86_64);
        kernel.add_source("src/main.rs");
        kernel.add_dependency("core");
        kernel.add_dependency("memory");
        kernel.add_dependency("scheduler");
        bs.add_target(kernel).unwrap();

        bs
    }

    #[test]
    fn test_add_target() {
        let bs = setup_build_system();
        assert_eq!(bs.target_count(), 4);
    }

    #[test]
    fn test_duplicate_target() {
        let mut bs = setup_build_system();
        let target = BuildTarget::new("core", TargetType::StaticLib, Architecture::X86_64);
        let result = bs.add_target(target);
        assert!(matches!(result, Err(BuildError::DuplicateTarget(_))));
    }

    #[test]
    fn test_remove_target() {
        let mut bs = setup_build_system();
        bs.remove_target("scheduler").unwrap();
        assert_eq!(bs.target_count(), 3);
    }

    #[test]
    fn test_validate_dependencies() {
        let bs = setup_build_system();
        assert!(bs.validate_dependencies().is_ok());
    }

    #[test]
    fn test_unsatisfied_dependency() {
        let mut bs = BuildSystem::with_defaults();
        let mut target = BuildTarget::new("broken", TargetType::Binary, Architecture::X86_64);
        target.add_dependency("nonexistent");
        bs.add_target(target).unwrap();
        let result = bs.validate_dependencies();
        assert!(matches!(result, Err(BuildError::UnsatisfiedDependency { .. })));
    }

    #[test]
    fn test_build_order() {
        let bs = setup_build_system();
        let order = bs.compute_build_order().unwrap();

        // core must come before memory, scheduler, kernel
        let core_pos = order.iter().position(|n| n == "core").unwrap();
        let memory_pos = order.iter().position(|n| n == "memory").unwrap();
        let scheduler_pos = order.iter().position(|n| n == "scheduler").unwrap();
        let kernel_pos = order.iter().position(|n| n == "kernel").unwrap();

        assert!(core_pos < memory_pos);
        assert!(core_pos < scheduler_pos);
        assert!(memory_pos < scheduler_pos);
        assert!(scheduler_pos < kernel_pos);
    }

    #[test]
    fn test_cyclic_dependency() {
        let mut bs = BuildSystem::with_defaults();

        let mut a = BuildTarget::new("a", TargetType::StaticLib, Architecture::X86_64);
        a.add_dependency("b");
        bs.add_target(a).unwrap();

        let mut b = BuildTarget::new("b", TargetType::StaticLib, Architecture::X86_64);
        b.add_dependency("a");
        bs.add_target(b).unwrap();

        let result = bs.compute_build_order();
        assert!(matches!(result, Err(BuildError::CyclicDependency(_))));
    }

    #[test]
    fn test_build_execution() {
        let mut bs = setup_build_system();
        let summary = bs.build().unwrap();

        assert!(summary.is_success());
        assert_eq!(summary.succeeded, 4);
        assert_eq!(summary.failed, 0);
        assert!(summary.total_time_ms > 0);
    }

    #[test]
    fn test_build_status() {
        let mut bs = setup_build_system();
        bs.build().unwrap();

        let succeeded = bs.targets_with_status(BuildStatus::Success);
        assert_eq!(succeeded.len(), 4);
    }

    #[test]
    fn test_build_output_paths() {
        let mut bs = setup_build_system();
        bs.build().unwrap();

        let kernel = bs.get_target("kernel").unwrap();
        assert!(kernel.output_path.is_some());
        assert!(kernel.output_path.as_ref().unwrap().contains("kernel"));
    }

    #[test]
    fn test_no_targets_error() {
        let bs = BuildSystem::with_defaults();
        let result = bs.compute_build_order();
        assert!(matches!(result, Err(BuildError::NoTargets)));
    }

    #[test]
    fn test_target_features() {
        let mut target = BuildTarget::new("test", TargetType::Binary, Architecture::X86_64);
        target.add_feature("gpu");
        target.add_feature("networking");
        target.add_feature("gpu"); // duplicate
        assert_eq!(target.features.len(), 2);
    }

    #[test]
    fn test_build_summary_rate() {
        let summary = BuildSummary {
            total_targets: 10,
            succeeded: 8,
            failed: 2,
            skipped: 0,
            total_time_ms: 5000,
            build_order: Vec::new(),
        };
        assert!(!summary.is_success());
        assert!((summary.success_rate() - 80.0).abs() < 1e-5);
    }

    #[test]
    fn test_architecture_display() {
        assert_eq!(format!("{}", Architecture::X86_64), "x86_64");
        assert_eq!(format!("{}", Architecture::Aarch64), "aarch64");
        assert_eq!(format!("{}", Architecture::RiscV64), "riscv64");
    }
}