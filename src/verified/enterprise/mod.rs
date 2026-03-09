//! # Enterprise Features Module
//! 
//! Ten moduł zawiera funkcje enterprise dla integracji z systemami korporacyjnymi.

pub mod ad;
pub mod ldap;
pub mod kerberos;
pub mod sso;
pub mod mfa;
pub mod rbac;

pub use ad::{ActiveDirectory, AdConfig, AdUser};
pub use ldap::{LdapClient, LdapConfig, LdapEntry};
pub use kerberos::{KerberosClient, KerberosConfig, KerberosTicket};
pub use sso::{SsoProvider, SsoConfig, SsoToken};
pub use mfa::{MfaProvider, MfaConfig, MfaChallenge};
pub use rbac::{RbacManager, Role, Permission, Policy};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu enterprise
static ENTERPRISE_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł enterprise
pub fn init() -> Result<(), EnterpriseError> {
    // Inicjalizuj Active Directory
    ad::init()?;
    
    // Inicjalizuj LDAP
    ldap::init()?;
    
    // Inicjalizuj Kerberos
    kerberos::init()?;
    
    // Inicjalizuj SSO
    sso::init()?;
    
    // Inicjalizuj MFA
    mfa::init()?;
    
    // Inicjalizuj RBAC
    rbac::init()?;
    
    ENTERPRISE_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu enterprise
pub fn enterprise_state() -> bool {
    ENTERPRISE_STATE.load(Ordering::Acquire) == 1
}

/// Błędy enterprise
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterpriseError {
    /// Błąd Active Directory
    AdError,
    /// Błąd LDAP
    LdapError,
    /// Błąd Kerberos
    KerberosError,
    /// Błąd SSO
    SsoError,
    /// Błąd MFA
    MfaError,
    /// Błąd RBAC
    RbacError,
    /// Błąd autentykacji
    AuthenticationError,
    /// Błąd autoryzacji
    AuthorizationError,
}

impl core::fmt::Display for EnterpriseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EnterpriseError::AdError => write!(f, "Active Directory error"),
            EnterpriseError::LdapError => write!(f, "LDAP error"),
            EnterpriseError::KerberosError => write!(f, "Kerberos error"),
            EnterpriseError::SsoError => write!(f, "SSO error"),
            EnterpriseError::MfaError => write!(f, "MFA error"),
            EnterpriseError::RbacError => write!(f, "RBAC error"),
            EnterpriseError::AuthenticationError => write!(f, "Authentication error"),
            EnterpriseError::AuthorizationError => write!(f, "Authorization error"),
        }
    }
}

impl core::error::Error for EnterpriseError {}