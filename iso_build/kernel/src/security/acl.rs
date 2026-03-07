//! Access Control Lists

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// ACL entry
#[derive(Debug, Clone)]
pub struct AclEntry {
    /// Subject (user or group)
    pub subject: AclSubject,
    /// Permissions
    pub permissions: Permissions,
    /// Flags
    pub flags: AclFlags,
}

/// ACL subject
#[derive(Debug, Clone)]
pub enum AclSubject {
    /// User by ID
    User(u32),
    /// Group by ID
    Group(u32),
    /// All users
    All,
    /// Owner
    Owner,
    /// Group owner
    GroupOwner,
}

/// ACL permissions
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self { read, write, execute }
    }
    
    pub fn none() -> Self {
        Self::new(false, false, false)
    }
    
    pub fn read_only() -> Self {
        Self::new(true, false, false)
    }
    
    pub fn read_write() -> Self {
        Self::new(true, true, false)
    }
    
    pub fn full() -> Self {
        Self::new(true, true, true)
    }
}

/// ACL flags
#[derive(Debug, Clone, Copy)]
pub struct AclFlags {
    /// Inherit to new files
    pub inherit: bool,
    /// Default ACL
    pub default: bool,
}

impl AclFlags {
    pub fn new() -> Self {
        Self {
            inherit: false,
            default: false,
        }
    }
}

/// ACL
#[derive(Debug, Clone)]
pub struct Acl {
    /// ACL entries
    pub entries: Vec<AclEntry>,
}

impl Acl {
    /// Create a new empty ACL
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    /// Create a simple ACL with user/group/other permissions
    pub fn simple(owner: Permissions, group: Permissions, other: Permissions) -> Self {
        let mut acl = Self::new();
        acl.entries.push(AclEntry {
            subject: AclSubject::Owner,
            permissions: owner,
            flags: AclFlags::new(),
        });
        acl.entries.push(AclEntry {
            subject: AclSubject::GroupOwner,
            permissions: group,
            flags: AclFlags::new(),
        });
        acl.entries.push(AclEntry {
            subject: AclSubject::All,
            permissions: other,
            flags: AclFlags::new(),
        });
        acl
    }
    
    /// Check if a user has access
    pub fn check_access(&self, uid: u32, gid: u32, _owner_uid: u32, owner_gid: u32, requested: Permissions) -> bool {
        for entry in &self.entries {
            match &entry.subject {
                AclSubject::User(u) if *u == uid => {
                    if entry.permissions.read >= requested.read
                        && entry.permissions.write >= requested.write
                        && entry.permissions.execute >= requested.execute
                    {
                        return true;
                    }
                }
                AclSubject::Group(g) if *g == gid => {
                    if entry.permissions.read >= requested.read
                        && entry.permissions.write >= requested.write
                        && entry.permissions.execute >= requested.execute
                    {
                        return true;
                    }
                }
                AclSubject::Owner if uid == _owner_uid => {
                    if entry.permissions.read >= requested.read
                        && entry.permissions.write >= requested.write
                        && entry.permissions.execute >= requested.execute
                    {
                        return true;
                    }
                }
                AclSubject::GroupOwner if gid == owner_gid => {
                    if entry.permissions.read >= requested.read
                        && entry.permissions.write >= requested.write
                        && entry.permissions.execute >= requested.execute
                    {
                        return true;
                    }
                }
                AclSubject::All => {
                    if entry.permissions.read >= requested.read
                        && entry.permissions.write >= requested.write
                        && entry.permissions.execute >= requested.execute
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}