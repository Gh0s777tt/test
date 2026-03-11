//! Runtime Integrity Verification for VANTIS OS
//!
//! Provides continuous runtime integrity monitoring by measuring critical
//! system resources against known-good baselines. Supports hash-based
//! verification with severity-based remediation policies.

use core::fmt;

// ============================================================================
// Resource Types
// ============================================================================

/// Types of system resources that can be integrity-checked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Kernel module / driver
    KernelModule,
    /// System binary / executable
    SystemBinary,
    /// Shared library
    SharedLibrary,
    /// Boot configuration
    BootConfig,
    /// Security policy file
    SecurityPolicy,
    /// Configuration file
    ConfigFile,
    /// Firmware image
    Firmware,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceKind::KernelModule => write!(f, "KernelModule"),
            ResourceKind::SystemBinary => write!(f, "SystemBinary"),
            ResourceKind::SharedLibrary => write!(f, "SharedLibrary"),
            ResourceKind::BootConfig => write!(f, "BootConfig"),
            ResourceKind::SecurityPolicy => write!(f, "SecurityPolicy"),
            ResourceKind::ConfigFile => write!(f, "ConfigFile"),
            ResourceKind::Firmware => write!(f, "Firmware"),
        }
    }
}

/// Severity of an integrity violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationSeverity {
    /// Informational - minor change detected
    Low,
    /// Warning - unexpected modification
    Medium,
    /// Critical - security-sensitive resource modified
    High,
    /// Emergency - kernel or boot integrity compromised
    Critical,
}

impl fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViolationSeverity::Low => write!(f, "LOW"),
            ViolationSeverity::Medium => write!(f, "MEDIUM"),
            ViolationSeverity::High => write!(f, "HIGH"),
            ViolationSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Default severity for each resource kind
fn default_severity(kind: ResourceKind) -> ViolationSeverity {
    match kind {
        ResourceKind::KernelModule | ResourceKind::Firmware => ViolationSeverity::Critical,
        ResourceKind::SystemBinary | ResourceKind::BootConfig => ViolationSeverity::High,
        ResourceKind::SharedLibrary | ResourceKind::SecurityPolicy => ViolationSeverity::Medium,
        ResourceKind::ConfigFile => ViolationSeverity::Low,
    }
}

// ============================================================================
// Measurement & Baseline
// ============================================================================

/// A hash measurement of a resource (simplified as a 32-byte array)
pub type HashValue = [u8; 32];

/// A measurement record for a single resource
#[derive(Debug, Clone)]
pub struct Measurement {
    pub resource_path: String,
    pub resource_kind: ResourceKind,
    pub expected_hash: HashValue,
    pub current_hash: Option<HashValue>,
    pub last_verified_epoch: u64,
    pub is_valid: bool,
}

impl Measurement {
    pub fn new(path: &str, kind: ResourceKind, expected_hash: HashValue) -> Self {
        Self {
            resource_path: path.to_string(),
            resource_kind: kind,
            expected_hash,
            current_hash: None,
            last_verified_epoch: 0,
            is_valid: false,
        }
    }

    /// Verify the current hash against the expected baseline
    pub fn verify(&mut self, current_hash: HashValue, epoch: u64) -> bool {
        self.current_hash = Some(current_hash);
        self.last_verified_epoch = epoch;
        self.is_valid = current_hash == self.expected_hash;
        self.is_valid
    }

    /// Update the expected hash (re-baseline)
    pub fn update_baseline(&mut self, new_hash: HashValue) {
        self.expected_hash = new_hash;
        self.current_hash = Some(new_hash);
        self.is_valid = true;
    }
}

// ============================================================================
// Violation Record
// ============================================================================

/// A recorded integrity violation
#[derive(Debug, Clone)]
pub struct IntegrityViolation {
    pub epoch: u64,
    pub resource_path: String,
    pub resource_kind: ResourceKind,
    pub severity: ViolationSeverity,
    pub expected_hash: HashValue,
    pub actual_hash: HashValue,
    pub remediation: RemediationAction,
}

/// Remediation actions for violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationAction {
    /// Log the violation only
    LogOnly,
    /// Alert administrators
    Alert,
    /// Quarantine the resource
    Quarantine,
    /// Restore from known-good backup
    Restore,
    /// Halt the system (critical violation)
    SystemHalt,
}

impl fmt::Display for RemediationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemediationAction::LogOnly => write!(f, "LOG"),
            RemediationAction::Alert => write!(f, "ALERT"),
            RemediationAction::Quarantine => write!(f, "QUARANTINE"),
            RemediationAction::Restore => write!(f, "RESTORE"),
            RemediationAction::SystemHalt => write!(f, "HALT"),
        }
    }
}

/// Map severity to default remediation action
fn default_remediation(severity: ViolationSeverity) -> RemediationAction {
    match severity {
        ViolationSeverity::Low => RemediationAction::LogOnly,
        ViolationSeverity::Medium => RemediationAction::Alert,
        ViolationSeverity::High => RemediationAction::Quarantine,
        ViolationSeverity::Critical => RemediationAction::SystemHalt,
    }
}

// ============================================================================
// Integrity Monitor
// ============================================================================

/// Error types for the integrity monitor
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrityError {
    ResourceNotFound(String),
    DuplicateResource(String),
    VerificationFailed(String),
    NoMeasurements,
}

impl fmt::Display for IntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrityError::ResourceNotFound(p) => write!(f, "Resource not found: {}", p),
            IntegrityError::DuplicateResource(p) => write!(f, "Duplicate resource: {}", p),
            IntegrityError::VerificationFailed(p) => write!(f, "Verification failed: {}", p),
            IntegrityError::NoMeasurements => write!(f, "No measurements registered"),
        }
    }
}

/// Result of a full integrity check
#[derive(Debug, Clone)]
pub struct IntegrityReport {
    pub epoch: u64,
    pub total_resources: usize,
    pub verified_ok: usize,
    pub violations_found: usize,
    pub violations: Vec<IntegrityViolation>,
    pub highest_severity: Option<ViolationSeverity>,
}

impl IntegrityReport {
    /// Whether all resources passed verification
    pub fn is_clean(&self) -> bool {
        self.violations_found == 0
    }

    /// Verification success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_resources == 0 {
            return 100.0;
        }
        self.verified_ok as f64 / self.total_resources as f64 * 100.0
    }
}

/// The main integrity monitoring engine
pub struct IntegrityMonitor {
    measurements: Vec<Measurement>,
    violation_log: Vec<IntegrityViolation>,
    total_checks: u64,
    total_violations: u64,
    current_epoch: u64,
}

impl IntegrityMonitor {
    /// Create a new integrity monitor
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
            violation_log: Vec::new(),
            total_checks: 0,
            total_violations: 0,
            current_epoch: 0,
        }
    }

    /// Set the current epoch
    pub fn set_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
    }

    /// Register a resource for integrity monitoring
    pub fn register_resource(
        &mut self,
        path: &str,
        kind: ResourceKind,
        baseline_hash: HashValue,
    ) -> Result<(), IntegrityError> {
        if self.measurements.iter().any(|m| m.resource_path == path) {
            return Err(IntegrityError::DuplicateResource(path.to_string()));
        }
        self.measurements.push(Measurement::new(path, kind, baseline_hash));
        Ok(())
    }

    /// Remove a resource from monitoring
    pub fn unregister_resource(&mut self, path: &str) -> Result<(), IntegrityError> {
        let idx = self.measurements.iter().position(|m| m.resource_path == path)
            .ok_or_else(|| IntegrityError::ResourceNotFound(path.to_string()))?;
        self.measurements.remove(idx);
        Ok(())
    }

    /// Verify a single resource
    pub fn verify_resource(
        &mut self,
        path: &str,
        current_hash: HashValue,
    ) -> Result<bool, IntegrityError> {
        let measurement = self.measurements.iter_mut()
            .find(|m| m.resource_path == path)
            .ok_or_else(|| IntegrityError::ResourceNotFound(path.to_string()))?;

        self.total_checks += 1;
        let valid = measurement.verify(current_hash, self.current_epoch);

        if !valid {
            let severity = default_severity(measurement.resource_kind);
            let violation = IntegrityViolation {
                epoch: self.current_epoch,
                resource_path: path.to_string(),
                resource_kind: measurement.resource_kind,
                severity,
                expected_hash: measurement.expected_hash,
                actual_hash: current_hash,
                remediation: default_remediation(severity),
            };
            self.violation_log.push(violation);
            self.total_violations += 1;
        }

        Ok(valid)
    }

    /// Run a full integrity check using a hash provider callback
    pub fn full_check<F>(&mut self, hash_provider: F) -> Result<IntegrityReport, IntegrityError>
    where
        F: Fn(&str) -> Option<HashValue>,
    {
        if self.measurements.is_empty() {
            return Err(IntegrityError::NoMeasurements);
        }

        let mut verified_ok = 0;
        let mut violations = Vec::new();
        let mut highest_severity: Option<ViolationSeverity> = None;

        let paths_and_kinds: Vec<(String, ResourceKind, HashValue)> = self.measurements.iter()
            .map(|m| (m.resource_path.clone(), m.resource_kind, m.expected_hash))
            .collect();

        for (path, kind, expected) in &paths_and_kinds {
            self.total_checks += 1;

            if let Some(current_hash) = hash_provider(path) {
                // Update measurement
                if let Some(m) = self.measurements.iter_mut().find(|m| m.resource_path == *path) {
                    m.verify(current_hash, self.current_epoch);
                }

                if current_hash == *expected {
                    verified_ok += 1;
                } else {
                    let severity = default_severity(*kind);
                    let violation = IntegrityViolation {
                        epoch: self.current_epoch,
                        resource_path: path.clone(),
                        resource_kind: *kind,
                        severity,
                        expected_hash: *expected,
                        actual_hash: current_hash,
                        remediation: default_remediation(severity),
                    };

                    if highest_severity.map_or(true, |s| severity > s) {
                        highest_severity = Some(severity);
                    }

                    violations.push(violation.clone());
                    self.violation_log.push(violation);
                    self.total_violations += 1;
                }
            } else {
                // Resource not accessible - treat as violation
                let severity = default_severity(*kind);
                let violation = IntegrityViolation {
                    epoch: self.current_epoch,
                    resource_path: path.clone(),
                    resource_kind: *kind,
                    severity,
                    expected_hash: *expected,
                    actual_hash: [0u8; 32],
                    remediation: default_remediation(severity),
                };

                if highest_severity.map_or(true, |s| severity > s) {
                    highest_severity = Some(severity);
                }

                violations.push(violation.clone());
                self.violation_log.push(violation);
                self.total_violations += 1;
            }
        }

        let total_resources = paths_and_kinds.len();
        let violations_found = violations.len();

        Ok(IntegrityReport {
            epoch: self.current_epoch,
            total_resources,
            verified_ok,
            violations_found,
            violations,
            highest_severity,
        })
    }

    /// Update baseline for a resource
    pub fn update_baseline(
        &mut self,
        path: &str,
        new_hash: HashValue,
    ) -> Result<(), IntegrityError> {
        let measurement = self.measurements.iter_mut()
            .find(|m| m.resource_path == path)
            .ok_or_else(|| IntegrityError::ResourceNotFound(path.to_string()))?;
        measurement.update_baseline(new_hash);
        Ok(())
    }

    /// Get the number of monitored resources
    pub fn resource_count(&self) -> usize {
        self.measurements.len()
    }

    /// Get total checks performed
    pub fn total_checks(&self) -> u64 {
        self.total_checks
    }

    /// Get total violations detected
    pub fn total_violations(&self) -> u64 {
        self.total_violations
    }

    /// Get the violation log
    pub fn violation_log(&self) -> &[IntegrityViolation] {
        &self.violation_log
    }

    /// Clear the violation log
    pub fn clear_log(&mut self) {
        self.violation_log.clear();
    }

    /// Get a measurement by path
    pub fn get_measurement(&self, path: &str) -> Option<&Measurement> {
        self.measurements.iter().find(|m| m.resource_path == path)
    }
}

impl Default for IntegrityMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_a() -> HashValue {
        let mut h = [0u8; 32];
        h[0] = 0xAA;
        h[31] = 0xBB;
        h
    }

    fn hash_b() -> HashValue {
        let mut h = [0u8; 32];
        h[0] = 0xCC;
        h[31] = 0xDD;
        h
    }

    #[test]
    fn test_measurement_verify_ok() {
        let mut m = Measurement::new("/boot/kernel", ResourceKind::KernelModule, hash_a());
        assert!(m.verify(hash_a(), 100));
        assert!(m.is_valid);
    }

    #[test]
    fn test_measurement_verify_fail() {
        let mut m = Measurement::new("/boot/kernel", ResourceKind::KernelModule, hash_a());
        assert!(!m.verify(hash_b(), 100));
        assert!(!m.is_valid);
    }

    #[test]
    fn test_measurement_update_baseline() {
        let mut m = Measurement::new("/boot/kernel", ResourceKind::KernelModule, hash_a());
        m.update_baseline(hash_b());
        assert_eq!(m.expected_hash, hash_b());
        assert!(m.is_valid);
    }

    #[test]
    fn test_register_resource() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a()).unwrap();
        assert_eq!(monitor.resource_count(), 1);
    }

    #[test]
    fn test_duplicate_resource() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a()).unwrap();
        let result = monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a());
        assert!(matches!(result, Err(IntegrityError::DuplicateResource(_))));
    }

    #[test]
    fn test_verify_resource_ok() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/bin/ls", ResourceKind::SystemBinary, hash_a()).unwrap();
        let valid = monitor.verify_resource("/bin/ls", hash_a()).unwrap();
        assert!(valid);
        assert_eq!(monitor.total_violations(), 0);
    }

    #[test]
    fn test_verify_resource_violation() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/bin/ls", ResourceKind::SystemBinary, hash_a()).unwrap();
        let valid = monitor.verify_resource("/bin/ls", hash_b()).unwrap();
        assert!(!valid);
        assert_eq!(monitor.total_violations(), 1);
        assert_eq!(monitor.violation_log().len(), 1);
        assert_eq!(monitor.violation_log()[0].severity, ViolationSeverity::High);
    }

    #[test]
    fn test_full_check_clean() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a()).unwrap();
        monitor.register_resource("/bin/ls", ResourceKind::SystemBinary, hash_b()).unwrap();

        let report = monitor.full_check(|path| {
            match path {
                "/boot/kernel" => Some(hash_a()),
                "/bin/ls" => Some(hash_b()),
                _ => None,
            }
        }).unwrap();

        assert!(report.is_clean());
        assert_eq!(report.verified_ok, 2);
        assert_eq!(report.violations_found, 0);
        assert!((report.success_rate() - 100.0).abs() < 1e-5);
    }

    #[test]
    fn test_full_check_with_violation() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a()).unwrap();
        monitor.register_resource("/bin/ls", ResourceKind::SystemBinary, hash_a()).unwrap();

        let report = monitor.full_check(|path| {
            match path {
                "/boot/kernel" => Some(hash_a()),  // OK
                "/bin/ls" => Some(hash_b()),        // MODIFIED
                _ => None,
            }
        }).unwrap();

        assert!(!report.is_clean());
        assert_eq!(report.verified_ok, 1);
        assert_eq!(report.violations_found, 1);
        assert_eq!(report.highest_severity, Some(ViolationSeverity::High));
    }

    #[test]
    fn test_full_check_missing_resource() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/boot/kernel", ResourceKind::KernelModule, hash_a()).unwrap();

        let report = monitor.full_check(|_| None).unwrap();

        assert!(!report.is_clean());
        assert_eq!(report.violations_found, 1);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(ViolationSeverity::Critical > ViolationSeverity::High);
        assert!(ViolationSeverity::High > ViolationSeverity::Medium);
        assert!(ViolationSeverity::Medium > ViolationSeverity::Low);
    }

    #[test]
    fn test_default_severity_mapping() {
        assert_eq!(default_severity(ResourceKind::KernelModule), ViolationSeverity::Critical);
        assert_eq!(default_severity(ResourceKind::SystemBinary), ViolationSeverity::High);
        assert_eq!(default_severity(ResourceKind::SharedLibrary), ViolationSeverity::Medium);
        assert_eq!(default_severity(ResourceKind::ConfigFile), ViolationSeverity::Low);
    }

    #[test]
    fn test_unregister_resource() {
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/bin/ls", ResourceKind::SystemBinary, hash_a()).unwrap();
        monitor.unregister_resource("/bin/ls").unwrap();
        assert_eq!(monitor.resource_count(), 0);
    }

    #[test]
    fn test_no_measurements_error() {
        let mut monitor = IntegrityMonitor::new();
        let result = monitor.full_check(|_| None);
        assert!(matches!(result, Err(IntegrityError::NoMeasurements)));
    }
}