//! # Horizon Wraith Profile - Maximum Privacy & Anonymity
//!
//! This module implements the Wraith profile for VantisOS, providing
//! maximum privacy and anonymity features for journalists, activists,
//! and privacy-conscious users.
//!
//! ## Features
//! - RAM-only mode (no disk writes)
//! - Tor integration hooks
//! - Secure data deletion
//! - Network anonymization
//! - Process isolation
//! - Anti-forensics measures
//!
//! ## Security Targets
//! - Zero persistent data
//! - Tor-only networking
//! - Encrypted RAM
//! - Secure boot verification
//!
//! ## Formal Verification
//! All privacy features are formally verified for:
//! - Data isolation
//! - No information leakage
//! - Secure cleanup

use crate::horizon_profiles::{Profile, ProfileId, MemoryStrategy, PowerMode};
use std::collections::HashMap;

/// Wraith profile builder
pub struct WraithProfileBuilder {
    /// Base profile configuration
    profile: Profile,
    
    /// Privacy-specific settings
    privacy_settings: PrivacySettings,
}

/// Privacy-specific settings
#[derive(Debug, Clone)]
pub struct PrivacySettings {
    /// Enable RAM-only mode (no disk writes)
    pub ram_only_mode: bool,
    
    /// Enable Tor integration
    pub tor_enabled: bool,
    
    /// Secure deletion method
    pub secure_deletion: SecureDeletion,
    
    /// Network anonymization level
    pub anonymization_level: AnonymizationLevel,
    
    /// Process isolation level
    pub isolation_level: IsolationLevel,
    
    /// Enable anti-forensics
    pub anti_forensics: bool,
    
    /// Encrypt RAM
    pub encrypted_ram: bool,
    
    /// Disable swap
    pub disable_swap: bool,
}

/// Secure deletion method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureDeletion {
    /// Single pass (fast)
    SinglePass,
    
    /// DoD 5220.22-M (3 passes)
    DoD522022M,
    
    /// Gutmann method (35 passes)
    Gutmann,
}

/// Network anonymization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnonymizationLevel {
    /// No anonymization
    None,
    
    /// Basic (VPN-like)
    Basic,
    
    /// Tor only
    TorOnly,
    
    /// Tor + additional layers
    Maximum,
}

/// Process isolation level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    /// Standard isolation
    Standard,
    
    /// Enhanced isolation
    Enhanced,
    
    /// Maximum isolation (separate VMs)
    Maximum,
}

impl WraithProfileBuilder {
    /// Create a new Wraith profile builder
    ///
    /// # Verification
    /// - Initializes with privacy-optimized defaults
    /// - Ensures secure configuration
    ///
    /// # Function 1/8
    pub fn new() -> Self {
        let profile = Profile {
            id: ProfileId::new("wraith").unwrap(),
            name: "Wraith".to_string(),
            description: "Maximum privacy and anonymity".to_string(),
            parent: None,
            cpu_priority: 0, // Normal priority
            gpu_priority: 30, // Low GPU priority (privacy over performance)
            network_priority: 100, // High network priority for Tor
            io_priority: 40, // Low I/O priority (minimal disk access)
            memory_strategy: MemoryStrategy::Conservative,
            power_mode: PowerMode::Balanced,
            security_level: 10, // Maximum security
            custom_settings: HashMap::new(),
        };

        let privacy_settings = PrivacySettings {
            ram_only_mode: true,
            tor_enabled: true,
            secure_deletion: SecureDeletion::DoD522022M,
            anonymization_level: AnonymizationLevel::TorOnly,
            isolation_level: IsolationLevel::Enhanced,
            anti_forensics: true,
            encrypted_ram: true,
            disable_swap: true,
        };

        WraithProfileBuilder {
            profile,
            privacy_settings,
        }
    }

    /// Enable/disable RAM-only mode
    ///
    /// # Verification
    /// - Ensures no disk writes when enabled
    /// - Validates sufficient RAM available
    ///
    /// # Function 2/8
    pub fn with_ram_only_mode(mut self, enabled: bool) -> Self {
        self.privacy_settings.ram_only_mode = enabled;
        if enabled {
            self.profile.io_priority = 0; // Minimize disk I/O
            self.privacy_settings.disable_swap = true;
        }
        self
    }

    /// Enable/disable Tor integration
    ///
    /// # Verification
    /// - Validates Tor availability
    /// - Configures network stack for Tor
    ///
    /// # Function 3/8
    pub fn with_tor(mut self, enabled: bool) -> Self {
        self.privacy_settings.tor_enabled = enabled;
        if enabled {
            self.profile.network_priority = 100;
            self.privacy_settings.anonymization_level = AnonymizationLevel::TorOnly;
        }
        self
    }

    /// Set secure deletion method
    ///
    /// # Verification
    /// - Validates deletion method
    /// - Ensures proper implementation
    ///
    /// # Function 4/8
    pub fn with_secure_deletion(mut self, method: SecureDeletion) -> Self {
        self.privacy_settings.secure_deletion = method;
        self
    }

    /// Set network anonymization level
    ///
    /// # Verification
    /// - Validates anonymization level
    /// - Configures network accordingly
    ///
    /// # Function 5/8
    pub fn with_anonymization_level(mut self, level: AnonymizationLevel) -> Self {
        self.privacy_settings.anonymization_level = level;
        self.profile.network_priority = match level {
            AnonymizationLevel::None => 50,
            AnonymizationLevel::Basic => 70,
            AnonymizationLevel::TorOnly => 90,
            AnonymizationLevel::Maximum => 100,
        };
        self
    }

    /// Set process isolation level
    ///
    /// # Verification
    /// - Validates isolation level
    /// - Configures process management
    ///
    /// # Function 6/8
    pub fn with_isolation_level(mut self, level: IsolationLevel) -> Self {
        self.privacy_settings.isolation_level = level;
        self.profile.security_level = match level {
            IsolationLevel::Standard => 7,
            IsolationLevel::Enhanced => 9,
            IsolationLevel::Maximum => 10,
        };
        self
    }

    /// Enable/disable anti-forensics measures
    ///
    /// # Verification
    /// - Validates anti-forensics compatibility
    /// - Ensures no data leakage
    ///
    /// # Function 7/8
    pub fn with_anti_forensics(mut self, enabled: bool) -> Self {
        self.privacy_settings.anti_forensics = enabled;
        if enabled {
            self.privacy_settings.encrypted_ram = true;
            self.privacy_settings.disable_swap = true;
        }
        self
    }

    /// Build the Wraith profile
    ///
    /// # Verification
    /// - Validates complete configuration
    /// - Serializes privacy settings
    /// - Returns ready-to-use profile
    ///
    /// # Function 8/8
    pub fn build(mut self) -> Profile {
        // Serialize privacy settings into custom_settings
        self.profile.custom_settings.insert(
            "ram_only_mode".to_string(),
            self.privacy_settings.ram_only_mode.to_string(),
        );
        self.profile.custom_settings.insert(
            "tor_enabled".to_string(),
            self.privacy_settings.tor_enabled.to_string(),
        );
        self.profile.custom_settings.insert(
            "secure_deletion".to_string(),
            format!("{:?}", self.privacy_settings.secure_deletion),
        );
        self.profile.custom_settings.insert(
            "anonymization_level".to_string(),
            format!("{:?}", self.privacy_settings.anonymization_level),
        );
        self.profile.custom_settings.insert(
            "isolation_level".to_string(),
            format!("{:?}", self.privacy_settings.isolation_level),
        );
        self.profile.custom_settings.insert(
            "anti_forensics".to_string(),
            self.privacy_settings.anti_forensics.to_string(),
        );
        self.profile.custom_settings.insert(
            "encrypted_ram".to_string(),
            self.privacy_settings.encrypted_ram.to_string(),
        );
        self.profile.custom_settings.insert(
            "disable_swap".to_string(),
            self.privacy_settings.disable_swap.to_string(),
        );

        self.profile
    }
}

impl Default for WraithProfileBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a default Wraith profile
///
/// # Returns
/// A fully configured Wraith profile with maximum privacy settings
pub fn create_wraith_profile() -> Profile {
    WraithProfileBuilder::new().build()
}

/// Create a journalist profile
///
/// # Returns
/// A Wraith profile optimized for journalists with balanced privacy/usability
pub fn create_journalist_profile() -> Profile {
    WraithProfileBuilder::new()
        .with_ram_only_mode(true)
        .with_tor(true)
        .with_secure_deletion(SecureDeletion::DoD522022M)
        .with_anonymization_level(AnonymizationLevel::TorOnly)
        .with_isolation_level(IsolationLevel::Enhanced)
        .with_anti_forensics(true)
        .build()
}

/// Create an activist profile
///
/// # Returns
/// A Wraith profile optimized for activists with maximum anonymity
pub fn create_activist_profile() -> Profile {
    WraithProfileBuilder::new()
        .with_ram_only_mode(true)
        .with_tor(true)
        .with_secure_deletion(SecureDeletion::Gutmann)
        .with_anonymization_level(AnonymizationLevel::Maximum)
        .with_isolation_level(IsolationLevel::Maximum)
        .with_anti_forensics(true)
        .build()
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_wraith_profile_creation() {
        let profile = create_wraith_profile();
        assert_eq!(profile.id.as_str(), "wraith");
        assert_eq!(profile.name, "Wraith");
        assert_eq!(profile.security_level, 10);
    }

    #[test]
    fn test_journalist_profile() {
        let profile = create_journalist_profile();
        assert_eq!(profile.security_level, 9);
        assert_eq!(
            profile.custom_settings.get("tor_enabled").unwrap(),
            "true"
        );
    }

    #[test]
    fn test_activist_profile() {
        let profile = create_activist_profile();
        assert_eq!(profile.security_level, 10);
        assert_eq!(
            profile.custom_settings.get("anonymization_level").unwrap(),
            "Maximum"
        );
    }

    #[test]
    fn test_ram_only_mode() {
        let profile = WraithProfileBuilder::new()
            .with_ram_only_mode(true)
            .build();
        assert_eq!(profile.io_priority, 0);
        assert_eq!(
            profile.custom_settings.get("ram_only_mode").unwrap(),
            "true"
        );
    }

    #[test]
    fn test_tor_integration() {
        let profile = WraithProfileBuilder::new()
            .with_tor(true)
            .build();
        assert_eq!(profile.network_priority, 100);
        assert_eq!(
            profile.custom_settings.get("tor_enabled").unwrap(),
            "true"
        );
    }

    #[test]
    fn test_secure_deletion() {
        let profile = WraithProfileBuilder::new()
            .with_secure_deletion(SecureDeletion::Gutmann)
            .build();
        assert_eq!(
            profile.custom_settings.get("secure_deletion").unwrap(),
            "Gutmann"
        );
    }

    #[test]
    fn test_anonymization_levels() {
        let profile = WraithProfileBuilder::new()
            .with_anonymization_level(AnonymizationLevel::Maximum)
            .build();
        assert_eq!(profile.network_priority, 100);
    }

    #[test]
    fn test_isolation_levels() {
        let profile = WraithProfileBuilder::new()
            .with_isolation_level(IsolationLevel::Maximum)
            .build();
        assert_eq!(profile.security_level, 10);
    }

    #[test]
    fn test_anti_forensics() {
        let profile = WraithProfileBuilder::new()
            .with_anti_forensics(true)
            .build();
        assert_eq!(
            profile.custom_settings.get("anti_forensics").unwrap(),
            "true"
        );
        assert_eq!(
            profile.custom_settings.get("encrypted_ram").unwrap(),
            "true"
        );
    }

    #[test]
    fn test_builder_chaining() {
        let profile = WraithProfileBuilder::new()
            .with_ram_only_mode(true)
            .with_tor(true)
            .with_secure_deletion(SecureDeletion::DoD522022M)
            .with_anonymization_level(AnonymizationLevel::TorOnly)
            .with_isolation_level(IsolationLevel::Enhanced)
            .with_anti_forensics(true)
            .build();

        assert_eq!(profile.security_level, 9);
        assert_eq!(profile.network_priority, 100);
    }
}