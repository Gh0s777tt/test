// VantisOS Release Management - V1.0 Release System
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Release Management
//!
//! Complete release management system for VantisOS V1.0, including version
//! management, release planning, build automation, testing, and deployment.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use semver::{Version, VersionReq};

use super::{NexusError};
use super::nexus_storage::NexusStorage;
use super::nexus_engine::NexusEngine;
use super::nexus_updates::UpdateManager;

/// Release version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseVersion {
    /// Major version
    pub major: u64,
    
    /// Minor version
    pub minor: u64,
    
    /// Patch version
    pub patch: u64,
    
    /// Pre-release identifier
    pub pre: Option<String>,
    
    /// Build metadata
    pub build: Option<String>,
}

impl ReleaseVersion {
    /// Create a new release version
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
            build: None,
        }
    }
    
    /// Convert to semver Version
    pub fn to_semver(&self) -> Version {
        let pre = self.pre.as_ref().map(|s| semver::Prerelease::new(s).unwrap());
        let build = self.build.as_ref().map(|s| semver::BuildMetadata::new(s).unwrap());
        
        Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            pre: pre.unwrap_or(semver::Prerelease::EMPTY),
            build: build.unwrap_or(semver::BuildMetadata::EMPTY),
        }
    }
    
    /// Get version string
    pub fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        
        if let Some(pre) = &self.pre {
            version.push_str("-");
            version.push_str(pre);
        }
        
        if let Some(build) = &self.build {
            version.push_str("+");
            version.push_str(build);
        }
        
        version
    }
}

/// Release status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseStatus {
    /// Release is planned
    Planned,
    /// Release is in development
    InDevelopment,
    /// Release is in testing
    InTesting,
    /// Release is in code freeze
    CodeFreeze,
    /// Release is in release candidate phase
    ReleaseCandidate,
    /// Release is released
    Released,
    /// Release is deprecated
    Deprecated,
}

/// Release type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseType {
    /// Major release
    Major,
    /// Minor release
    Minor,
    /// Patch release
    Patch,
    /// Pre-release
    PreRelease,
    /// Hotfix
    Hotfix,
}

/// Release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    /// Release ID
    pub release_id: Uuid,
    
    /// Release version
    pub version: ReleaseVersion,
    
    /// Release name
    pub name: String,
    
    /// Release description
    pub description: String,
    
    /// Release type
    pub release_type: ReleaseType,
    
    /// Release status
    pub status: ReleaseStatus,
    
    /// Release notes
    pub release_notes: String,
    
    /// Changelog
    pub changelog: String,
    
    /// Planned release date
    pub planned_date: u64,
    
    /// Actual release date
    pub actual_date: Option<u64>,
    
    /// Code freeze date
    pub code_freeze_date: Option<u64>,
    
    /// Release candidate date
    pub rc_date: Option<u64>,
    
    /// Features in this release
    pub features: Vec<ReleaseFeature>,
    
    /// Bug fixes in this release
    pub bug_fixes: Vec<BugFix>,
    
    /// Known issues
    pub known_issues: Vec<KnownIssue>,
    
    /// Breaking changes
    pub breaking_changes: Vec<BreakingChange>,
    
    /// Dependencies
    pub dependencies: Vec<Dependency>,
    
    /// Build artifacts
    pub build_artifacts: Vec<BuildArtifact>,
    
    /// Test results
    pub test_results: TestResults,
    
    /// Release metrics
    pub metrics: ReleaseMetrics,
    
    /// Created at timestamp
    pub created_at: u64,
    
    /// Updated at timestamp
    pub updated_at: u64,
}

/// Release feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseFeature {
    /// Feature ID
    pub feature_id: Uuid,
    
    /// Feature name
    pub name: String,
    
    /// Feature description
    pub description: String,
    
    /// Feature status
    pub status: FeatureStatus,
    
    /// Feature priority
    pub priority: FeaturePriority,
    
    /// Assigned to
    pub assigned_to: Option<String>,
    
    /// Estimated hours
    pub estimated_hours: u32,
    
    /// Actual hours
    pub actual_hours: u32,
    
    /// Completed at
    pub completed_at: Option<u64>,
}

/// Feature status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureStatus {
    /// Feature is proposed
    Proposed,
    /// Feature is approved
    Approved,
    /// Feature is in development
    InDevelopment,
    /// Feature is in testing
    InTesting,
    /// Feature is complete
    Complete,
    /// Feature is cancelled
    Cancelled,
}

/// Feature priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum FeaturePriority {
    /// Critical priority
    Critical = 4,
    /// High priority
    High = 3,
    /// Medium priority
    Medium = 2,
    /// Low priority
    Low = 1,
}

/// Bug fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugFix {
    /// Bug ID
    pub bug_id: Uuid,
    
    /// Bug title
    pub title: String,
    
    /// Bug description
    pub description: String,
    
    /// Bug severity
    pub severity: BugSeverity,
    
    /// Bug status
    pub status: BugStatus,
    
    /// Assigned to
    pub assigned_to: Option<String>,
    
    /// Fixed at
    pub fixed_at: Option<u64>,
}

/// Bug severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum BugSeverity {
    /// Critical bug
    Critical = 4,
    /// High severity
    High = 3,
    /// Medium severity
    Medium = 2,
    /// Low severity
    Low = 1,
}

/// Bug status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BugStatus {
    /// Bug is open
    Open,
    /// Bug is in progress
    InProgress,
    /// Bug is fixed
    Fixed,
    /// Bug is verified
    Verified,
    /// Bug is closed
    Closed,
}

/// Known issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownIssue {
    /// Issue ID
    pub issue_id: Uuid,
    
    /// Issue title
    pub title: String,
    
    /// Issue description
    pub description: String,
    
    /// Issue severity
    pub severity: BugSeverity,
    
    /// Workaround
    pub workaround: Option<String>,
    
    /// Planned fix version
    pub planned_fix_version: Option<String>,
}

/// Breaking change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    /// Change ID
    pub change_id: Uuid,
    
    /// Change title
    pub title: String,
    
    /// Change description
    pub description: String,
    
    /// Migration guide
    pub migration_guide: String,
    
    /// Affected components
    pub affected_components: Vec<String>,
}

/// Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    
    /// Dependency version requirement
    pub version_requirement: String,
    
    /// Dependency type
    pub dependency_type: DependencyType,
    
    /// Is optional
    pub optional: bool,
}

/// Dependency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    /// Runtime dependency
    Runtime,
    /// Build dependency
    Build,
    /// Development dependency
    Dev,
    /// Test dependency
    Test,
}

/// Build artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    /// Artifact ID
    pub artifact_id: Uuid,
    
    /// Artifact name
    pub name: String,
    
    /// Artifact type
    pub artifact_type: ArtifactType,
    
    /// Artifact path
    pub path: String,
    
    /// Artifact checksum (SHA256)
    pub checksum: String,
    
    /// Artifact size in bytes
    pub size_bytes: u64,
    
    /// Build timestamp
    pub build_timestamp: u64,
    
    /// Build number
    pub build_number: u32,
}

/// Artifact type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactType {
    /// Source tarball
    SourceTarball,
    /// Binary package
    Binary,
    /// ISO image
    IsoImage,
    /// Docker image
    DockerImage,
    /// Documentation
    Documentation,
    /// Checksums file
    Checksums,
    /// Signature file
    Signature,
}

/// Test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    /// Total tests run
    pub total_tests: u32,
    
    /// Passed tests
    pub passed: u32,
    
    /// Failed tests
    pub failed: u32,
    
    /// Skipped tests
    pub skipped: u32,
    
    /// Pass rate (0-100)
    pub pass_rate: f64,
    
    /// Test coverage (0-100)
    pub coverage: f64,
    
    /// Test execution time in seconds
    pub execution_time_seconds: u64,
    
    /// Test suites
    pub test_suites: Vec<TestSuite>,
}

/// Test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Suite name
    pub name: String,
    
    /// Total tests
    pub total_tests: u32,
    
    /// Passed tests
    pub passed: u32,
    
    /// Failed tests
    pub failed: u32,
    
    /// Skipped tests
    pub skipped: u32,
    
    /// Execution time in seconds
    pub execution_time_seconds: u64,
}

/// Release metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseMetrics {
    /// Total lines of code
    pub total_loc: u64,
    
    /// New lines of code
    pub new_loc: u64,
    
    /// Modified lines of code
    pub modified_loc: u64,
    
    /// Deleted lines of code
    pub deleted_loc: u64,
    
    /// Number of contributors
    pub contributors: u32,
    
    /// Number of commits
    pub commits: u32,
    
    /// Number of pull requests
    pub pull_requests: u32,
    
    /// Number of issues closed
    pub issues_closed: u32,
    
    /// Build time in seconds
    pub build_time_seconds: u64,
    
    /// Package size in bytes
    pub package_size_bytes: u64,
}

/// Release criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCriteria {
    /// Minimum test pass rate (0-100)
    pub min_test_pass_rate: f64,
    
    /// Minimum code coverage (0-100)
    pub min_code_coverage: f64,
    
    /// Maximum critical bugs allowed
    pub max_critical_bugs: u32,
    
    /// Maximum high severity bugs allowed
    pub max_high_bugs: u32,
    
    /// All features must be complete
    pub all_features_complete: bool,
    
    /// All documentation must be complete
    pub all_documentation_complete: bool,
    
    /// Security audit must pass
    pub security_audit_passed: bool,
    
    /// Performance benchmarks must pass
    pub performance_benchmarks_passed: bool,
}

impl Default for ReleaseCriteria {
    fn default() -> Self {
        Self {
            min_test_pass_rate: 95.0,
            min_code_coverage: 80.0,
            max_critical_bugs: 0,
            max_high_bugs: 0,
            all_features_complete: true,
            all_documentation_complete: true,
            security_audit_passed: true,
            performance_benchmarks_passed: true,
        }
    }
}

/// Release Manager
pub struct ReleaseManager {
    /// Storage backend
    storage: Arc<NexusStorage>,
    
    /// Core engine
    engine: Arc<NexusEngine>,
    
    /// Update manager
    update_manager: Arc<UpdateManager>,
    
    /// Releases
    releases: Arc<RwLock<HashMap<Uuid, Release>>>,
    
    /// Release criteria
    release_criteria: Arc<RwLock<ReleaseCriteria>>,
    
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl ReleaseManager {
    /// Create a new Release Manager instance
    pub fn new(
        storage: Arc<NexusStorage>,
        engine: Arc<NexusEngine>,
        update_manager: Arc<UpdateManager>,
    ) -> Result<Self, NexusError> {
        Ok(Self {
            storage,
            engine,
            update_manager,
            releases: Arc::new(RwLock::new(HashMap::new())),
            release_criteria: Arc::new(RwLock::new(ReleaseCriteria::default())),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Create a new release
    pub async fn create_release(
        &self,
        version: ReleaseVersion,
        name: String,
        description: String,
        release_type: ReleaseType,
        planned_date: u64,
    ) -> Result<Uuid, NexusError> {
        let release_id = Uuid::new_v4();
        
        let release = Release {
            release_id,
            version,
            name,
            description,
            release_type,
            status: ReleaseStatus::Planned,
            release_notes: String::new(),
            changelog: String::new(),
            planned_date,
            actual_date: None,
            code_freeze_date: None,
            rc_date: None,
            features: Vec::new(),
            bug_fixes: Vec::new(),
            known_issues: Vec::new(),
            breaking_changes: Vec::new(),
            dependencies: Vec::new(),
            build_artifacts: Vec::new(),
            test_results: TestResults {
                total_tests: 0,
                passed: 0,
                failed: 0,
                skipped: 0,
                pass_rate: 0.0,
                coverage: 0.0,
                execution_time_seconds: 0,
                test_suites: Vec::new(),
            },
            metrics: ReleaseMetrics {
                total_loc: 0,
                new_loc: 0,
                modified_loc: 0,
                deleted_loc: 0,
                contributors: 0,
                commits: 0,
                pull_requests: 0,
                issues_closed: 0,
                build_time_seconds: 0,
                package_size_bytes: 0,
            },
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        releases.insert(release_id, release);
        
        log::info!("Created release: {} ({})", release_id, name);
        
        Ok(release_id)
    }
    
    /// Get release by ID
    pub fn get_release(&self, release_id: Uuid) -> Option<Release> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        releases.get(&release_id).cloned()
    }
    
    /// Get all releases
    pub fn get_all_releases(&self) -> Vec<Release> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        releases.values().cloned().collect()
    }
    
    /// Get latest release
    pub fn get_latest_release(&self) -> Option<Release> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        releases.values()
            .filter(|r| r.status == ReleaseStatus::Released)
            .max_by_key(|r| r.actual_date.unwrap_or(0))
            .cloned()
    }
    
    /// Update release status
    pub async fn update_release_status(
        &self,
        release_id: Uuid,
        status: ReleaseStatus,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.status = status;
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Set actual date if released
        if status == ReleaseStatus::Released {
            release.actual_date = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        
        log::info!("Updated release {} status to {:?}", release_id, status);
        
        Ok(())
    }
    
    /// Add feature to release
    pub async fn add_feature(
        &self,
        release_id: Uuid,
        feature: ReleaseFeature,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.features.push(feature);
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Add bug fix to release
    pub async fn add_bug_fix(
        &self,
        release_id: Uuid,
        bug_fix: BugFix,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.bug_fixes.push(bug_fix);
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Add known issue to release
    pub async fn add_known_issue(
        &self,
        release_id: Uuid,
        issue: KnownIssue,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.known_issues.push(issue);
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Add breaking change to release
    pub async fn add_breaking_change(
        &self,
        release_id: Uuid,
        change: BreakingChange,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.breaking_changes.push(change);
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Update test results
    pub async fn update_test_results(
        &self,
        release_id: Uuid,
        test_results: TestResults,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.test_results = test_results;
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Update release metrics
    pub async fn update_metrics(
        &self,
        release_id: Uuid,
        metrics: ReleaseMetrics,
    ) -> Result<(), NexusError> {
        let mut releases = self.releases.write()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get_mut(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        release.metrics = metrics;
        release.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
    
    /// Check if release meets criteria
    pub async fn check_release_criteria(&self, release_id: Uuid) -> Result<bool, NexusError> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        let criteria = self.release_criteria.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        // Check test pass rate
        if release.test_results.pass_rate < criteria.min_test_pass_rate {
            return Ok(false);
        }
        
        // Check code coverage
        if release.test_results.coverage < criteria.min_code_coverage {
            return Ok(false);
        }
        
        // Check critical bugs
        let critical_bugs = release.bug_fixes.iter()
            .filter(|b| b.severity == BugSeverity::Critical && b.status != BugStatus::Fixed)
            .count() as u32;
        
        if critical_bugs > criteria.max_critical_bugs {
            return Ok(false);
        }
        
        // Check high severity bugs
        let high_bugs = release.bug_fixes.iter()
            .filter(|b| b.severity == BugSeverity::High && b.status != BugStatus::Fixed)
            .count() as u32;
        
        if high_bugs > criteria.max_high_bugs {
            return Ok(false);
        }
        
        // Check if all features are complete
        if criteria.all_features_complete {
            let incomplete_features = release.features.iter()
                .filter(|f| f.status != FeatureStatus::Complete)
                .count();
            
            if incomplete_features > 0 {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Generate release notes
    pub async fn generate_release_notes(&self, release_id: Uuid) -> Result<String, NexusError> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        let mut notes = format!("# {} v{}\n\n", release.name, release.version.to_string());
        notes.push_str(&format!("{}\n\n", release.description));
        
        // Features
        if !release.features.is_empty() {
            notes.push_str("## Features\n\n");
            for feature in &release.features {
                notes.push_str(&format!("- **{}**: {}\n", feature.name, feature.description));
            }
            notes.push_str("\n");
        }
        
        // Bug fixes
        if !release.bug_fixes.is_empty() {
            notes.push_str("## Bug Fixes\n\n");
            for bug_fix in &release.bug_fixes {
                notes.push_str(&format!("- **{}**: {}\n", bug_fix.title, bug_fix.description));
            }
            notes.push_str("\n");
        }
        
        // Breaking changes
        if !release.breaking_changes.is_empty() {
            notes.push_str("## Breaking Changes\n\n");
            for change in &release.breaking_changes {
                notes.push_str(&format!("- **{}**: {}\n", change.title, change.description));
                notes.push_str(&format!("  Migration: {}\n", change.migration_guide));
            }
            notes.push_str("\n");
        }
        
        // Known issues
        if !release.known_issues.is_empty() {
            notes.push_str("## Known Issues\n\n");
            for issue in &release.known_issues {
                notes.push_str(&format!("- **{}**: {}\n", issue.title, issue.description));
                if let Some(workaround) = &issue.workaround {
                    notes.push_str(&format!("  Workaround: {}\n", workaround));
                }
            }
            notes.push_str("\n");
        }
        
        // Test results
        notes.push_str("## Test Results\n\n");
        notes.push_str(&format!("- Total Tests: {}\n", release.test_results.total_tests));
        notes.push_str(&format!("- Passed: {}\n", release.test_results.passed));
        notes.push_str(&format!("- Failed: {}\n", release.test_results.failed));
        notes.push_str(&format!("- Pass Rate: {:.1}%\n", release.test_results.pass_rate));
        notes.push_str(&format!("- Coverage: {:.1}%\n", release.test_results.coverage));
        notes.push_str("\n");
        
        Ok(notes)
    }
    
    /// Generate changelog
    pub async fn generate_changelog(&self, release_id: Uuid) -> Result<String, NexusError> {
        let releases = self.releases.read()
            .map_err(|_| NexusError::LockError)
            .unwrap();
        
        let release = releases.get(&release_id)
            .ok_or(NexusError::NotFound)?;
        
        let mut changelog = format!("## [{}] - {}\n\n", release.version.to_string(), {
            let date = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let datetime = chrono::DateTime::<chrono::Utc>::from(UNIX_EPOCH + Duration::from_secs(date));
            datetime.format("%Y-%m-%d").to_string()
        });
        
        changelog.push_str(&format!("### {}\n\n", release.name));
        changelog.push_str(&format!("{}\n\n", release.description));
        
        // Features
        if !release.features.is_empty() {
            changelog.push_str("### Added\n\n");
            for feature in &release.features {
                changelog.push_str(&format!("- {}\n", feature.description));
            }
            changelog.push_str("\n");
        }
        
        // Bug fixes
        if !release.bug_fixes.is_empty() {
            changelog.push_str("### Fixed\n\n");
            for bug_fix in &release.bug_fixes {
                changelog.push_str(&format!("- {}\n", bug_fix.description));
            }
            changelog.push_str("\n");
        }
        
        // Breaking changes
        if !release.breaking_changes.is_empty() {
            changelog.push_str("### Changed\n\n");
            for change in &release.breaking_changes {
                changelog.push_str(&format!("- {}\n", change.description));
            }
            changelog.push_str("\n");
        }
        
        Ok(changelog)
    }
}

/// Nexus error type (re-exported for convenience)
pub use super::nexus_server::NexusError;