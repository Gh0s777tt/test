//! Zero-Trust Network Access (ZTNA) for VANTIS OS
//!
//! Implements a Zero-Trust security model where no entity is trusted by
//! default. Every access request is evaluated based on identity, device
//! posture, location, and behavioral trust factors. Supports policy-based
//! access control with audit logging.

use core::fmt;

// ============================================================================
// Identity & Trust
// ============================================================================

/// Trust factors contributing to the overall trust score
#[derive(Debug, Clone)]
pub struct TrustFactors {
    /// Device health/compliance score (0.0 - 1.0)
    pub device_health: f64,
    /// Authentication strength (0.0 - 1.0)
    pub auth_strength: f64,
    /// Network location trust (0.0 - 1.0)
    pub network_trust: f64,
    /// Behavioral analysis score (0.0 - 1.0)
    pub behavior_score: f64,
    /// Time-based risk factor (0.0 - 1.0)
    pub temporal_risk: f64,
    /// Data sensitivity factor (0.0 - 1.0)
    pub data_sensitivity: f64,
}

impl TrustFactors {
    /// Compute weighted trust score
    pub fn compute_score(&self) -> f64 {
        let weights = [0.25, 0.20, 0.15, 0.20, 0.10, 0.10];
        let factors = [
            self.device_health,
            self.auth_strength,
            self.network_trust,
            self.behavior_score,
            self.temporal_risk,
            self.data_sensitivity,
        ];

        let weighted_sum: f64 = factors.iter()
            .zip(weights.iter())
            .map(|(&f, &w)| f.clamp(0.0, 1.0) * w)
            .sum();

        weighted_sum.clamp(0.0, 1.0)
    }

    /// Create high-trust factors (e.g., corporate device, MFA, on-premises)
    pub fn high_trust() -> Self {
        Self {
            device_health: 0.95,
            auth_strength: 0.95,
            network_trust: 0.90,
            behavior_score: 0.90,
            temporal_risk: 0.85,
            data_sensitivity: 0.80,
        }
    }

    /// Create low-trust factors (e.g., unknown device, basic auth, remote)
    pub fn low_trust() -> Self {
        Self {
            device_health: 0.30,
            auth_strength: 0.40,
            network_trust: 0.20,
            behavior_score: 0.50,
            temporal_risk: 0.60,
            data_sensitivity: 0.70,
        }
    }
}

impl Default for TrustFactors {
    fn default() -> Self {
        Self {
            device_health: 0.5,
            auth_strength: 0.5,
            network_trust: 0.5,
            behavior_score: 0.5,
            temporal_risk: 0.5,
            data_sensitivity: 0.5,
        }
    }
}

/// An authenticated identity in the ZTNA system
#[derive(Debug, Clone)]
pub struct Identity {
    pub id: u64,
    pub principal: String,
    pub roles: Vec<String>,
    pub trust_factors: TrustFactors,
    pub mfa_verified: bool,
    pub session_start_epoch: u64,
    pub last_activity_epoch: u64,
}

impl Identity {
    pub fn new(id: u64, principal: &str) -> Self {
        Self {
            id,
            principal: principal.to_string(),
            roles: Vec::new(),
            trust_factors: TrustFactors::default(),
            mfa_verified: false,
            session_start_epoch: 0,
            last_activity_epoch: 0,
        }
    }

    /// Add a role to this identity
    pub fn add_role(&mut self, role: &str) {
        if !self.roles.contains(&role.to_string()) {
            self.roles.push(role.to_string());
        }
    }

    /// Check if identity has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Current trust score
    pub fn trust_score(&self) -> f64 {
        let base = self.trust_factors.compute_score();
        // MFA bonus
        if self.mfa_verified { (base * 1.1).min(1.0) } else { base * 0.8 }
    }
}

// ============================================================================
// Access Policy
// ============================================================================

/// Access decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessDecision {
    /// Access granted
    Allow,
    /// Access denied
    Deny,
    /// Access requires additional verification (step-up auth)
    StepUp,
    /// Access granted with reduced privileges
    AllowRestricted,
}

impl fmt::Display for AccessDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessDecision::Allow => write!(f, "ALLOW"),
            AccessDecision::Deny => write!(f, "DENY"),
            AccessDecision::StepUp => write!(f, "STEP-UP"),
            AccessDecision::AllowRestricted => write!(f, "ALLOW-RESTRICTED"),
        }
    }
}

/// An access policy rule
#[derive(Debug, Clone)]
pub struct AccessPolicy {
    pub id: u64,
    pub name: String,
    /// Resource pattern (supports wildcard *)
    pub resource_pattern: String,
    /// Required roles (any match)
    pub required_roles: Vec<String>,
    /// Minimum trust score required
    pub min_trust_score: f64,
    /// Whether MFA is required
    pub require_mfa: bool,
    /// Allowed network locations (empty = any)
    pub allowed_locations: Vec<String>,
    /// Maximum session age in seconds (0 = no limit)
    pub max_session_age_secs: u64,
    /// Whether the policy is active
    pub is_active: bool,
}

impl AccessPolicy {
    pub fn new(id: u64, name: &str, resource_pattern: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            resource_pattern: resource_pattern.to_string(),
            required_roles: Vec::new(),
            min_trust_score: 0.5,
            require_mfa: false,
            allowed_locations: Vec::new(),
            max_session_age_secs: 0,
            is_active: true,
        }
    }

    /// Check if a resource matches this policy's pattern
    pub fn matches_resource(&self, resource: &str) -> bool {
        if self.resource_pattern == "*" {
            return true;
        }
        if self.resource_pattern.ends_with('*') {
            let prefix = &self.resource_pattern[..self.resource_pattern.len() - 1];
            return resource.starts_with(prefix);
        }
        if self.resource_pattern.starts_with('*') {
            let suffix = &self.resource_pattern[1..];
            return resource.ends_with(suffix);
        }
        self.resource_pattern == resource
    }

    /// Check if identity has any of the required roles
    pub fn check_roles(&self, identity: &Identity) -> bool {
        if self.required_roles.is_empty() {
            return true;
        }
        self.required_roles.iter().any(|r| identity.has_role(r))
    }
}

// ============================================================================
// Audit Log
// ============================================================================

/// An audit log entry for access decisions
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub identity_id: u64,
    pub principal: String,
    pub resource: String,
    pub decision: AccessDecision,
    pub trust_score: f64,
    pub reason: String,
}

// ============================================================================
// ZTNA Controller
// ============================================================================

/// Error types for the ZTNA controller
#[derive(Debug, Clone, PartialEq)]
pub enum ZtnaError {
    IdentityNotFound(u64),
    PolicyNotFound(u64),
    DuplicateIdentity(u64),
    DuplicatePolicy(u64),
    InvalidTrustScore(f64),
}

impl fmt::Display for ZtnaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZtnaError::IdentityNotFound(id) => write!(f, "Identity {} not found", id),
            ZtnaError::PolicyNotFound(id) => write!(f, "Policy {} not found", id),
            ZtnaError::DuplicateIdentity(id) => write!(f, "Identity {} already exists", id),
            ZtnaError::DuplicatePolicy(id) => write!(f, "Policy {} already exists", id),
            ZtnaError::InvalidTrustScore(s) => write!(f, "Invalid trust score: {}", s),
        }
    }
}

/// The main Zero-Trust Network Access controller
pub struct ZtnaController {
    identities: Vec<Identity>,
    policies: Vec<AccessPolicy>,
    audit_log: Vec<AuditEntry>,
    /// Default decision when no policy matches
    default_decision: AccessDecision,
    /// Global minimum trust score
    global_min_trust: f64,
    /// Current timestamp for session checks
    current_epoch: u64,
}

impl ZtnaController {
    /// Create a new ZTNA controller
    pub fn new() -> Self {
        Self {
            identities: Vec::new(),
            policies: Vec::new(),
            audit_log: Vec::new(),
            default_decision: AccessDecision::Deny, // deny by default
            global_min_trust: 0.3,
            current_epoch: 0,
        }
    }

    /// Set the current epoch time
    pub fn set_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
    }

    /// Register an identity
    pub fn register_identity(&mut self, identity: Identity) -> Result<(), ZtnaError> {
        if self.identities.iter().any(|i| i.id == identity.id) {
            return Err(ZtnaError::DuplicateIdentity(identity.id));
        }
        self.identities.push(identity);
        Ok(())
    }

    /// Remove an identity
    pub fn remove_identity(&mut self, id: u64) -> Result<(), ZtnaError> {
        let idx = self.identities.iter().position(|i| i.id == id)
            .ok_or(ZtnaError::IdentityNotFound(id))?;
        self.identities.remove(idx);
        Ok(())
    }

    /// Add an access policy
    pub fn add_policy(&mut self, policy: AccessPolicy) -> Result<(), ZtnaError> {
        if self.policies.iter().any(|p| p.id == policy.id) {
            return Err(ZtnaError::DuplicatePolicy(policy.id));
        }
        self.policies.push(policy);
        Ok(())
    }

    /// Remove a policy
    pub fn remove_policy(&mut self, id: u64) -> Result<(), ZtnaError> {
        let idx = self.policies.iter().position(|p| p.id == id)
            .ok_or(ZtnaError::PolicyNotFound(id))?;
        self.policies.remove(idx);
        Ok(())
    }

    /// Evaluate access request
    pub fn evaluate(
        &mut self,
        identity_id: u64,
        resource: &str,
    ) -> Result<AccessDecision, ZtnaError> {
        let identity = self.identities.iter()
            .find(|i| i.id == identity_id)
            .ok_or(ZtnaError::IdentityNotFound(identity_id))?
            .clone();

        let trust_score = identity.trust_score();

        // Global minimum trust check
        if trust_score < self.global_min_trust {
            let decision = AccessDecision::Deny;
            self.log_access(&identity, resource, decision, trust_score, "Below global minimum trust");
            return Ok(decision);
        }

        // Find matching policies (most specific first)
        let matching_policies: Vec<&AccessPolicy> = self.policies.iter()
            .filter(|p| p.is_active && p.matches_resource(resource))
            .collect();

        if matching_policies.is_empty() {
            let decision = self.default_decision;
            self.log_access(&identity, resource, decision, trust_score, "No matching policy");
            return Ok(decision);
        }

        // Evaluate each matching policy
        for policy in &matching_policies {
            // Check trust score
            if trust_score < policy.min_trust_score {
                if trust_score >= policy.min_trust_score * 0.8 {
                    let decision = AccessDecision::StepUp;
                    self.log_access(&identity, resource, decision, trust_score, "Trust score marginal - step-up required");
                    return Ok(decision);
                }
                let decision = AccessDecision::Deny;
                self.log_access(&identity, resource, decision, trust_score, "Insufficient trust score");
                return Ok(decision);
            }

            // Check roles
            if !policy.check_roles(&identity) {
                let decision = AccessDecision::Deny;
                self.log_access(&identity, resource, decision, trust_score, "Missing required role");
                return Ok(decision);
            }

            // Check MFA
            if policy.require_mfa && !identity.mfa_verified {
                let decision = AccessDecision::StepUp;
                self.log_access(&identity, resource, decision, trust_score, "MFA required");
                return Ok(decision);
            }

            // Check session age
            if policy.max_session_age_secs > 0 && self.current_epoch > 0 {
                let session_age = self.current_epoch.saturating_sub(identity.session_start_epoch);
                if session_age > policy.max_session_age_secs {
                    let decision = AccessDecision::StepUp;
                    self.log_access(&identity, resource, decision, trust_score, "Session expired");
                    return Ok(decision);
                }
            }
        }

        // All checks passed
        let decision = if trust_score >= 0.8 {
            AccessDecision::Allow
        } else {
            AccessDecision::AllowRestricted
        };

        self.log_access(&identity, resource, decision, trust_score, "Policy checks passed");
        Ok(decision)
    }

    /// Log an access decision
    fn log_access(
        &mut self,
        identity: &Identity,
        resource: &str,
        decision: AccessDecision,
        trust_score: f64,
        reason: &str,
    ) {
        self.audit_log.push(AuditEntry {
            timestamp: self.current_epoch,
            identity_id: identity.id,
            principal: identity.principal.clone(),
            resource: resource.to_string(),
            decision,
            trust_score,
            reason: reason.to_string(),
        });
    }

    /// Get the audit log
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }

    /// Get audit entries for a specific identity
    pub fn audit_for_identity(&self, identity_id: u64) -> Vec<&AuditEntry> {
        self.audit_log.iter().filter(|e| e.identity_id == identity_id).collect()
    }

    /// Get number of registered identities
    pub fn identity_count(&self) -> usize {
        self.identities.len()
    }

    /// Get number of policies
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Get an identity by ID
    pub fn get_identity(&self, id: u64) -> Option<&Identity> {
        self.identities.iter().find(|i| i.id == id)
    }

    /// Clear the audit log
    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }
}

impl Default for ZtnaController {
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

    fn setup_controller() -> ZtnaController {
        let mut ctrl = ZtnaController::new();
        ctrl.set_epoch(1000);

        // Register identities
        let mut admin = Identity::new(1, "admin@vantis.os");
        admin.add_role("admin");
        admin.trust_factors = TrustFactors::high_trust();
        admin.mfa_verified = true;
        admin.session_start_epoch = 900;
        ctrl.register_identity(admin).unwrap();

        let mut user = Identity::new(2, "user@vantis.os");
        user.add_role("user");
        user.trust_factors = TrustFactors::default();
        user.mfa_verified = false;
        user.session_start_epoch = 950;
        ctrl.register_identity(user).unwrap();

        let mut guest = Identity::new(3, "guest@external.com");
        guest.trust_factors = TrustFactors::low_trust();
        guest.session_start_epoch = 990;
        ctrl.register_identity(guest).unwrap();

        // Add policies
        let mut admin_policy = AccessPolicy::new(1, "admin_access", "/admin/*");
        admin_policy.required_roles = vec!["admin".to_string()];
        admin_policy.min_trust_score = 0.7;
        admin_policy.require_mfa = true;
        ctrl.add_policy(admin_policy).unwrap();

        let mut user_policy = AccessPolicy::new(2, "user_access", "/app/*");
        user_policy.required_roles = vec!["user".to_string(), "admin".to_string()];
        user_policy.min_trust_score = 0.4;
        ctrl.add_policy(user_policy).unwrap();

        let mut public_policy = AccessPolicy::new(3, "public_access", "/public/*");
        public_policy.min_trust_score = 0.2;
        ctrl.add_policy(public_policy).unwrap();

        ctrl
    }

    #[test]
    fn test_trust_factors_score() {
        let high = TrustFactors::high_trust();
        let low = TrustFactors::low_trust();
        assert!(high.compute_score() > 0.8);
        assert!(low.compute_score() < 0.5);
    }

    #[test]
    fn test_trust_factors_default() {
        let default = TrustFactors::default();
        let score = default.compute_score();
        assert!((score - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_identity_trust_with_mfa() {
        let mut id = Identity::new(1, "test");
        id.trust_factors = TrustFactors::high_trust();
        id.mfa_verified = true;
        let with_mfa = id.trust_score();

        id.mfa_verified = false;
        let without_mfa = id.trust_score();

        assert!(with_mfa > without_mfa);
    }

    #[test]
    fn test_policy_resource_matching() {
        let policy = AccessPolicy::new(1, "test", "/api/*");
        assert!(policy.matches_resource("/api/users"));
        assert!(policy.matches_resource("/api/data/export"));
        assert!(!policy.matches_resource("/admin/settings"));
    }

    #[test]
    fn test_policy_wildcard_all() {
        let policy = AccessPolicy::new(1, "test", "*");
        assert!(policy.matches_resource("/anything"));
        assert!(policy.matches_resource(""));
    }

    #[test]
    fn test_policy_exact_match() {
        let policy = AccessPolicy::new(1, "test", "/specific/resource");
        assert!(policy.matches_resource("/specific/resource"));
        assert!(!policy.matches_resource("/specific/other"));
    }

    #[test]
    fn test_admin_access_allowed() {
        let mut ctrl = setup_controller();
        let decision = ctrl.evaluate(1, "/admin/settings").unwrap();
        assert_eq!(decision, AccessDecision::Allow);
    }

    #[test]
    fn test_user_denied_admin() {
        let mut ctrl = setup_controller();
        let decision = ctrl.evaluate(2, "/admin/settings").unwrap();
        assert_eq!(decision, AccessDecision::Deny); // missing admin role
    }

    #[test]
    fn test_guest_denied_low_trust() {
        let mut ctrl = setup_controller();
        let decision = ctrl.evaluate(3, "/app/dashboard").unwrap();
        // Guest has low trust and no roles
        assert!(decision == AccessDecision::Deny || decision == AccessDecision::StepUp);
    }

    #[test]
    fn test_public_access() {
        let mut ctrl = setup_controller();
        let decision = ctrl.evaluate(3, "/public/docs").unwrap();
        // Public policy has low trust requirement
        assert!(decision == AccessDecision::Allow || decision == AccessDecision::AllowRestricted);
    }

    #[test]
    fn test_no_matching_policy() {
        let mut ctrl = setup_controller();
        let decision = ctrl.evaluate(1, "/unknown/resource").unwrap();
        assert_eq!(decision, AccessDecision::Deny); // default deny
    }

    #[test]
    fn test_audit_log() {
        let mut ctrl = setup_controller();
        ctrl.evaluate(1, "/admin/settings").unwrap();
        ctrl.evaluate(2, "/app/dashboard").unwrap();

        assert_eq!(ctrl.audit_log().len(), 2);
        let admin_log = ctrl.audit_for_identity(1);
        assert_eq!(admin_log.len(), 1);
        assert_eq!(admin_log[0].principal, "admin@vantis.os");
    }

    #[test]
    fn test_duplicate_identity() {
        let mut ctrl = ZtnaController::new();
        ctrl.register_identity(Identity::new(1, "test")).unwrap();
        let result = ctrl.register_identity(Identity::new(1, "test2"));
        assert!(matches!(result, Err(ZtnaError::DuplicateIdentity(1))));
    }

    #[test]
    fn test_remove_identity() {
        let mut ctrl = setup_controller();
        assert_eq!(ctrl.identity_count(), 3);
        ctrl.remove_identity(3).unwrap();
        assert_eq!(ctrl.identity_count(), 2);
    }

    #[test]
    fn test_mfa_step_up() {
        let mut ctrl = setup_controller();
        // User 2 doesn't have MFA, try to access admin resource
        // First, give user admin role to pass role check
        let mut admin_user = Identity::new(4, "admin_no_mfa@vantis.os");
        admin_user.add_role("admin");
        admin_user.trust_factors = TrustFactors::high_trust();
        admin_user.mfa_verified = false; // no MFA
        admin_user.session_start_epoch = 900;
        ctrl.register_identity(admin_user).unwrap();

        let decision = ctrl.evaluate(4, "/admin/settings").unwrap();
        assert_eq!(decision, AccessDecision::StepUp); // MFA required
    }
}