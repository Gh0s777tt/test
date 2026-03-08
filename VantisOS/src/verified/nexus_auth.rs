// VantisOS Nexus Authentication - OAuth 2.0 and RBAC
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Nexus Authentication
//!
//! OAuth 2.0 authentication and Role-Based Access Control (RBAC) for Nexus Server.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Algorithm, Header, Validation, EncodingKey, DecodingKey};

use super::{NexusError};
use super::storage::NexusStorage;

/// OAuth 2.0 provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Provider {
    /// Provider name (google, github, azure, etc.)
    pub name: String,
    
    /// Client ID
    pub client_id: String,
    
    /// Client secret
    pub client_secret: String,
    
    /// Authorization endpoint
    pub auth_url: String,
    
    /// Token endpoint
    pub token_url: String,
    
    /// User info endpoint
    pub user_info_url: String,
    
    /// Scopes to request
    pub scopes: Vec<String>,
}

/// Role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Super administrator
    SuperAdmin,
    /// Administrator
    Admin,
    /// Compliance officer
    ComplianceOfficer,
    /// Security analyst
    SecurityAnalyst,
    /// Operator
    Operator,
    /// Auditor
    Auditor,
    /// Viewer
    Viewer,
    /// Custom role
    Custom(String),
}

impl Role {
    /// Get all permissions for this role
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::SuperAdmin => vec![
                Permission::All,
            ],
            Role::Admin => vec![
                Permission::ReadNodes,
                Permission::WriteNodes,
                Permission::ControlNodes,
                Permission::ReadCompliance,
                Permission::WriteCompliance,
                Permission::ReadMetrics,
                Permission::WriteMetrics,
                Permission::ReadUsers,
                Permission::WriteUsers,
                Permission::ManageUpdates,
            ],
            Role::ComplianceOfficer => vec![
                Permission::ReadNodes,
                Permission::ReadCompliance,
                Permission::WriteCompliance,
                Permission::ReadMetrics,
            ],
            Role::SecurityAnalyst => vec![
                Permission::ReadNodes,
                Permission::ReadCompliance,
                Permission::ReadMetrics,
                Permission::ReadAlerts,
            ],
            Role::Operator => vec![
                Permission::ReadNodes,
                Permission::ControlNodes,
                Permission::ReadMetrics,
            ],
            Role::Auditor => vec![
                Permission::ReadNodes,
                Permission::ReadCompliance,
                Permission::ReadMetrics,
                Permission::ReadAuditLogs,
            ],
            Role::Viewer => vec![
                Permission::ReadNodes,
                Permission::ReadCompliance,
                Permission::ReadMetrics,
            ],
            Role::Custom(_) => vec![],
        }
    }
}

/// Permission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// All permissions
    All,
    /// Read nodes
    ReadNodes,
    /// Write nodes
    WriteNodes,
    /// Control nodes
    ControlNodes,
    /// Read compliance
    ReadCompliance,
    /// Write compliance
    WriteCompliance,
    /// Read metrics
    ReadMetrics,
    /// Write metrics
    WriteMetrics,
    /// Read users
    ReadUsers,
    /// Write users
    WriteUsers,
    /// Manage updates
    ManageUpdates,
    /// Read alerts
    ReadAlerts,
    /// Write alerts
    WriteAlerts,
    /// Read audit logs
    ReadAuditLogs,
    /// Custom permission
    Custom(String),
}

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub user_id: Uuid,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: String,
    
    /// Password hash (for local users)
    pub password_hash: Option<String>,
    
    /// OAuth provider (if applicable)
    pub oauth_provider: Option<String>,
    
    /// OAuth subject ID (if applicable)
    pub oauth_subject: Option<String>,
    
    /// User roles
    pub roles: Vec<Role>,
    
    /// User permissions
    pub permissions: HashSet<Permission>,
    
    /// Created at timestamp
    pub created_at: u64,
    
    /// Last login timestamp
    pub last_login: Option<u64>,
    
    /// User metadata
    pub metadata: HashMap<String, String>,
}

impl User {
    /// Check if user has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        // Check for All permission
        if self.permissions.contains(&Permission::All) {
            return true;
        }
        
        // Check for specific permission
        self.permissions.contains(permission)
    }
    
    /// Check if user has any of the specified permissions
    pub fn has_any_permission(&self, permissions: &[Permission]) -> bool {
        permissions.iter().any(|p| self.has_permission(p))
    }
    
    /// Check if user has all of the specified permissions
    pub fn has_all_permissions(&self, permissions: &[Permission]) -> bool {
        permissions.iter().all(|p| self.has_permission(p))
    }
    
    /// Check if user has a specific role
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }
}

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    
    /// Issuer
    pub iss: String,
    
    /// Audience
    pub aud: String,
    
    /// Expiration time
    pub exp: usize,
    
    /// Issued at
    pub iat: usize,
    
    /// User roles
    pub roles: Vec<String>,
    
    /// User permissions
    pub permissions: Vec<String>,
}

/// Authentication manager
pub struct AuthManager {
    /// Storage backend
    storage: Arc<NexusStorage>,
    
    /// OAuth 2.0 providers
    oauth_providers: Vec<OAuth2Provider>,
    
    /// JWT secret key
    jwt_secret: String,
    
    /// JWT expiration time in seconds
    jwt_expiration: u64,
    
    /// In-memory user cache
    user_cache: Arc<RwLock<HashMap<Uuid, User>>>,
    
    /// Token blacklist (for logout)
    token_blacklist: Arc<RwLock<HashSet<String>>>,
}

impl AuthManager {
    /// Create a new Auth Manager instance
    pub fn new(
        storage: Arc<NexusStorage>,
        oauth_providers: Vec<OAuth2Provider>,
    ) -> Result<Self, NexusError> {
        Ok(Self {
            storage,
            oauth_providers,
            jwt_secret: Self::generate_jwt_secret(),
            jwt_expiration: 86400, // 24 hours
            user_cache: Arc::new(RwLock::new(HashMap::new())),
            token_blacklist: Arc::new(RwLock::new(HashSet::new())),
        })
    }
    
    /// Generate a random JWT secret
    fn generate_jwt_secret() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Register a new user
    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password_hash: String,
        roles: Vec<Role>,
    ) -> Result<User, NexusError> {
        let user_id = Uuid::new_v4();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Calculate permissions from roles
        let permissions: HashSet<Permission> = roles.iter()
            .flat_map(|role| role.permissions())
            .collect();
        
        let user = User {
            user_id,
            username: username.clone(),
            email,
            password_hash: Some(password_hash),
            oauth_provider: None,
            oauth_subject: None,
            roles,
            permissions,
            created_at,
            last_login: None,
            metadata: HashMap::new(),
        };
        
        // Store in database
        self.storage.store_user(&user).await?;
        
        // Add to cache
        let mut cache = self.user_cache.write()
            .map_err(|_| NexusError::LockError)?;
        cache.insert(user_id, user.clone());
        
        log::info!("User registered: {}", username);
        
        Ok(user)
    }
    
    /// Authenticate a user with username and password
    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, NexusError> {
        // Load user from database
        let user = self.storage.load_user_by_username(username).await?
            .ok_or_else(|| NexusError::AuthError("User not found".to_string()))?;
        
        // Verify password
        if let Some(ref password_hash) = user.password_hash {
            // TODO: Implement password verification (bcrypt, argon2, etc.)
            // For now, just check if password_hash matches
            if password_hash != password {
                return Err(NexusError::AuthError("Invalid password".to_string()));
            }
        } else {
            return Err(NexusError::AuthError("User has no password".to_string()));
        }
        
        // Update last login
        let mut cache = self.user_cache.write()
            .map_err(|_| NexusError::LockError)?;
        if let Some(cached_user) = cache.get_mut(&user.user_id) {
            cached_user.last_login = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        
        log::info!("User authenticated: {}", username);
        
        Ok(user)
    }
    
    /// Authenticate a user with OAuth 2.0
    pub async fn authenticate_oauth(
        &self,
        provider_name: &str,
        auth_code: &str,
    ) -> Result<User, NexusError> {
        // Find OAuth provider
        let provider = self.oauth_providers.iter()
            .find(|p| p.name == provider_name)
            .ok_or_else(|| NexusError::AuthError("OAuth provider not found".to_string()))?;
        
        // TODO: Exchange auth code for access token
        // TODO: Get user info from provider
        // TODO: Create or update user in database
        
        Err(NexusError::AuthError("OAuth authentication not implemented".to_string()))
    }
    
    /// Generate a JWT token for a user
    pub fn generate_token(&self, user: &User) -> Result<String, NexusError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let exp = now + self.jwt_expiration as usize;
        
        let roles: Vec<String> = user.roles.iter()
            .map(|r| format!("{:?}", r))
            .collect();
        
        let permissions: Vec<String> = user.permissions.iter()
            .map(|p| format!("{:?}", p))
            .collect();
        
        let claims = Claims {
            sub: user.user_id.to_string(),
            iss: "vantis-nexus".to_string(),
            aud: "vantis-nexus-api".to_string(),
            exp,
            iat: now,
            roles,
            permissions,
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        ).map_err(|e| NexusError::AuthError(format!("Failed to generate token: {}", e)))?;
        
        Ok(token)
    }
    
    /// Validate a JWT token
    pub fn validate_token(&self, token: &str) -> Result<User, NexusError> {
        // Check if token is blacklisted
        let blacklist = self.token_blacklist.read()
            .map_err(|_| NexusError::LockError)?;
        if blacklist.contains(token) {
            return Err(NexusError::AuthError("Token is blacklisted".to_string()));
        }
        drop(blacklist);
        
        // Decode and validate token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|e| NexusError::AuthError(format!("Invalid token: {}", e)))?;
        
        // Extract user ID from claims
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| NexusError::AuthError("Invalid user ID in token".to_string()))?;
        
        // Load user from cache or database
        let cache = self.user_cache.read()
            .map_err(|_| NexusError::LockError)?;
        if let Some(user) = cache.get(&user_id) {
            return Ok(user.clone());
        }
        drop(cache);
        
        // TODO: Load from database
        Err(NexusError::AuthError("User not found".to_string()))
    }
    
    /// Logout a user (blacklist token)
    pub fn logout(&self, token: String) -> Result<(), NexusError> {
        let mut blacklist = self.token_blacklist.write()
            .map_err(|_| NexusError::LockError)?;
        blacklist.insert(token);
        
        Ok(())
    }
    
    /// Refresh a token
    pub fn refresh_token(&self, token: &str) -> Result<String, NexusError> {
        // Validate old token
        let user = self.validate_token(token)?;
        
        // Generate new token
        self.generate_token(&user)
    }
    
    /// Get a user by ID
    pub async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, NexusError> {
        // Check cache first
        {
            let cache = self.user_cache.read()
                .map_err(|_| NexusError::LockError)?;
            if let Some(user) = cache.get(&user_id) {
                return Ok(Some(user.clone()));
            }
        }
        
        // Load from database
        self.storage.load_user(user_id).await
    }
    
    /// Update user roles
    pub async fn update_user_roles(
        &self,
        user_id: Uuid,
        roles: Vec<Role>,
    ) -> Result<(), NexusError> {
        // Load user
        let mut user = self.storage.load_user(user_id).await?
            .ok_or_else(|| NexusError::AuthError("User not found".to_string()))?;
        
        // Update roles
        user.roles = roles.clone();
        
        // Recalculate permissions
        user.permissions = roles.iter()
            .flat_map(|role| role.permissions())
            .collect();
        
        // Store in database
        self.storage.store_user(&user).await?;
        
        // Update cache
        let mut cache = self.user_cache.write()
            .map_err(|_| NexusError::LockError)?;
        cache.insert(user_id, user);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_role_permissions() {
        let admin_perms = Role::Admin.permissions();
        assert!(admin_perms.contains(&Permission::ReadNodes));
        assert!(admin_perms.contains(&Permission::WriteNodes));
        assert!(admin_perms.contains(&Permission::ControlNodes));
        
        let viewer_perms = Role::Viewer.permissions();
        assert!(viewer_perms.contains(&Permission::ReadNodes));
        assert!(!viewer_perms.contains(&Permission::WriteNodes));
    }
    
    #[test]
    fn test_user_has_permission() {
        let user = User {
            user_id: Uuid::new_v4(),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: None,
            oauth_provider: None,
            oauth_subject: None,
            roles: vec![Role::Admin],
            permissions: Role::Admin.permissions().into_iter().collect(),
            created_at: 0,
            last_login: None,
            metadata: HashMap::new(),
        };
        
        assert!(user.has_permission(&Permission::ReadNodes));
        assert!(user.has_permission(&Permission::WriteNodes));
        assert!(!user.has_permission(&Permission::All));
    }
}