// VantisFS Features - VantisOS
//
// This module implements VantisFS features including:
// - File permissions (Unix-style)
// - Symbolic links
// - Hard links
// - File attributes

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use spin::Mutex;
use super::vfs::{FileType, FilePermissions};
use super::vantisfs::{Vantisfs, VantisfsInode};

/// VantisFS symbolic link
#[derive(Debug, Clone)]
pub struct VantisfsSymlink {
    pub inode_number: u64,
    pub target: String,
}

impl VantisfsSymlink {
    /// Create a new symbolic link
    pub fn new(inode_number: u64, target: String) -> Self {
        Self {
            inode_number,
            target,
        }
    }
}

/// VantisFS hard link
#[derive(Debug, Clone)]
pub struct VantisfsHardlink {
    pub inode_number: u64,
    pub link_count: u32,
}

impl VantisfsHardlink {
    /// Create a new hard link
    pub fn new(inode_number: u64, link_count: u32) -> Self {
        Self {
            inode_number,
            link_count,
        }
    }
}

/// VantisFS extended attributes
#[derive(Debug, Clone)]
pub struct VantisfsExtendedAttributes {
    pub attributes: BTreeMap<String, Vec<u8>>,
}

impl VantisfsExtendedAttributes {
    /// Create a new extended attributes
    pub fn new() -> Self {
        Self {
            attributes: BTreeMap::new(),
        }
    }
    
    /// Set an attribute
    pub fn set(&mut self, name: String, value: Vec<u8>) {
        self.attributes.insert(name, value);
    }
    
    /// Get an attribute
    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.attributes.get(name)
    }
    
    /// Remove an attribute
    pub fn remove(&mut self, name: &str) -> Option<Vec<u8>> {
        self.attributes.remove(name)
    }
    
    /// List all attributes
    pub fn list(&self) -> Vec<String> {
        self.attributes.keys().cloned().collect()
    }
}

/// VantisFS file permissions manager
pub struct VantisfsPermissionsManager {
    uid: u32,
    gid: u32,
}

impl VantisfsPermissionsManager {
    /// Create a new permissions manager
    pub fn new(uid: u32, gid: u32) -> Self {
        Self { uid, gid }
    }
    
    /// Check if user has read permission
    pub fn can_read(&self, permissions: &FilePermissions, file_uid: u32, file_gid: u32) -> bool {
        // Owner can always read if owner read bit is set
        if self.uid == file_uid && permissions.user_read {
            return true;
        }
        
        // Group can read if group read bit is set
        if self.gid == file_gid && permissions.group_read {
            return true;
        }
        
        // Others can read if other read bit is set
        permissions.other_read
    }
    
    /// Check if user has write permission
    pub fn can_write(&self, permissions: &FilePermissions, file_uid: u32, file_gid: u32) -> bool {
        // Owner can always write if owner write bit is set
        if self.uid == file_uid && permissions.user_write {
            return true;
        }
        
        // Group can write if group write bit is set
        if self.gid == file_gid && permissions.group_write {
            return true;
        }
        
        // Others can write if other write bit is set
        permissions.other_write
    }
    
    /// Check if user has execute permission
    pub fn can_execute(&self, permissions: &FilePermissions, file_uid: u32, file_gid: u32) -> bool {
        // Owner can always execute if owner execute bit is set
        if self.uid == file_uid && permissions.user_execute {
            return true;
        }
        
        // Group can execute if group execute bit is set
        if self.gid == file_gid && permissions.group_execute {
            return true;
        }
        
        // Others can execute if other execute bit is set
        permissions.other_execute
    }
    
    /// Change file permissions
    pub fn chmod(&self, inode: &mut VantisfsInode, mode: u16) {
        inode.permissions = FilePermissions::from_mode(mode);
    }
    
    /// Change file owner
    pub fn chown(&self, inode: &mut VantisfsInode, uid: u32, gid: u32) {
        inode.uid = uid;
        inode.gid = gid;
    }
}

/// VantisFS link manager
pub struct VantisfsLinkManager {
    symlinks: Mutex<BTreeMap<u64, VantisfsSymlink>>,
    hardlinks: Mutex<BTreeMap<u64, VantisfsHardlink>>,
}

impl VantisfsLinkManager {
    /// Create a new link manager
    pub fn new() -> Self {
        Self {
            symlinks: Mutex::new(BTreeMap::new()),
            hardlinks: Mutex::new(BTreeMap::new()),
        }
    }
    
    /// Create a symbolic link
    pub fn create_symlink(&self, inode_number: u64, target: String) -> Result<(), ()> {
        let mut symlinks = self.symlinks.lock();
        symlinks.insert(inode_number, VantisfsSymlink::new(inode_number, target));
        Ok(())
    }
    
    /// Read a symbolic link
    pub fn read_symlink(&self, inode_number: u64) -> Result<String, ()> {
        let symlinks = self.symlinks.lock();
        symlinks.get(&inode_number)
            .map(|s| s.target.clone())
            .ok_or(())
    }
    
    /// Create a hard link
    pub fn create_hardlink(&self, inode_number: u64) -> Result<(), ()> {
        let mut hardlinks = self.hardlinks.lock();
        
        if let Some(hardlink) = hardlinks.get_mut(&inode_number) {
            hardlink.link_count += 1;
        } else {
            hardlinks.insert(inode_number, VantisfsHardlink::new(inode_number, 2));
        }
        
        Ok(())
    }
    
    /// Remove a hard link
    pub fn remove_hardlink(&self, inode_number: u64) -> Result<(), ()> {
        let mut hardlinks = self.hardlinks.lock();
        
        if let Some(hardlink) = hardlinks.get_mut(&inode_number) {
            if hardlink.link_count > 1 {
                hardlink.link_count -= 1;
            } else {
                hardlinks.remove(&inode_number);
            }
            Ok(())
        } else {
            Err(())
        }
    }
    
    /// Get hard link count
    pub fn get_hardlink_count(&self, inode_number: u64) -> u32 {
        let hardlinks = self.hardlinks.lock();
        hardlinks.get(&inode_number)
            .map(|h| h.link_count)
            .unwrap_or(1)
    }
    
    /// Check if inode is a symbolic link
    pub fn is_symlink(&self, inode_number: u64) -> bool {
        self.symlinks.lock().contains_key(&inode_number)
    }
}

/// VantisFS attributes manager
pub struct VantisfsAttributesManager {
    extended_attributes: Mutex<BTreeMap<u64, VantisfsExtendedAttributes>>,
}

impl VantisfsAttributesManager {
    /// Create a new attributes manager
    pub fn new() -> Self {
        Self {
            extended_attributes: Mutex::new(BTreeMap::new()),
        }
    }
    
    /// Set extended attribute
    pub fn set_extended_attribute(&self, inode_number: u64, name: String, value: Vec<u8>) -> Result<(), ()> {
        let mut attrs = self.extended_attributes.lock();
        
        if !attrs.contains_key(&inode_number) {
            attrs.insert(inode_number, VantisfsExtendedAttributes::new());
        }
        
        attrs.get_mut(&inode_number).unwrap().set(name, value);
        Ok(())
    }
    
    /// Get extended attribute
    pub fn get_extended_attribute(&self, inode_number: u64, name: &str) -> Result<Vec<u8>, ()> {
        let attrs = self.extended_attributes.lock();
        attrs.get(&inode_number)
            .and_then(|a| a.get(name))
            .map(|v| v.clone())
            .ok_or(())
    }
    
    /// Remove extended attribute
    pub fn remove_extended_attribute(&self, inode_number: u64, name: &str) -> Result<(), ()> {
        let mut attrs = self.extended_attributes.lock();
        
        if let Some(ext_attrs) = attrs.get_mut(&inode_number) {
            ext_attrs.remove(name);
            Ok(())
        } else {
            Err(())
        }
    }
    
    /// List extended attributes
    pub fn list_extended_attributes(&self, inode_number: u64) -> Result<Vec<String>, ()> {
        let attrs = self.extended_attributes.lock();
        attrs.get(&inode_number)
            .map(|a| a.list())
            .ok_or(())
    }
}

/// VantisFS features extension
pub struct VantisfsFeatures {
    permissions_manager: VantisfsPermissionsManager,
    link_manager: VantisfsLinkManager,
    attributes_manager: VantisfsAttributesManager,
}

impl VantisfsFeatures {
    /// Create a new VantisFS features extension
    pub fn new(uid: u32, gid: u32) -> Self {
        Self {
            permissions_manager: VantisfsPermissionsManager::new(uid, gid),
            link_manager: VantisfsLinkManager::new(),
            attributes_manager: VantisfsAttributesManager::new(),
        }
    }
    
    /// Get permissions manager
    pub fn get_permissions_manager(&self) -> &VantisfsPermissionsManager {
        &self.permissions_manager
    }
    
    /// Get link manager
    pub fn get_link_manager(&self) -> &VantisfsLinkManager {
        &self.link_manager
    }
    
    /// Get attributes manager
    pub fn get_attributes_manager(&self) -> &VantisfsAttributesManager {
        &self.attributes_manager
    }
}

/// Initialize VantisFS features
pub fn init() {
    // TODO: Initialize VantisFS features
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symlink_creation() {
        let symlink = VantisfsSymlink::new(1, "/target/path".to_string());
        
        assert_eq!(symlink.inode_number, 1);
        assert_eq!(symlink.target, "/target/path");
    }
    
    #[test]
    fn test_hardlink_creation() {
        let hardlink = VantisfsHardlink::new(1, 2);
        
        assert_eq!(hardlink.inode_number, 1);
        assert_eq!(hardlink.link_count, 2);
    }
    
    #[test]
    fn test_extended_attributes() {
        let mut attrs = VantisfsExtendedAttributes::new();
        
        attrs.set("user.comment".to_string(), b"Test comment".to_vec());
        
        assert!(attrs.get("user.comment").is_some());
        assert_eq!(attrs.get("user.comment").unwrap(), b"Test comment");
    }
    
    #[test]
    fn test_permissions_manager() {
        let manager = VantisfsPermissionsManager::new(0, 0);
        
        let mut perms = FilePermissions::new();
        perms.user_read = true;
        perms.group_read = false;
        perms.other_read = false;
        
        assert!(manager.can_read(&perms, 0, 0));
        assert!(!manager.can_read(&perms, 1, 0));
    }
    
    #[test]
    fn test_link_manager_symlink() {
        let manager = VantisfsLinkManager::new();
        
        manager.create_symlink(1, "/target".to_string()).unwrap();
        
        assert!(manager.is_symlink(1));
        assert_eq!(manager.read_symlink(1).unwrap(), "/target");
    }
    
    #[test]
    fn test_link_manager_hardlink() {
        let manager = VantisfsLinkManager::new();
        
        manager.create_hardlink(1).unwrap();
        assert_eq!(manager.get_hardlink_count(1), 2);
        
        manager.create_hardlink(1).unwrap();
        assert_eq!(manager.get_hardlink_count(1), 3);
        
        manager.remove_hardlink(1).unwrap();
        assert_eq!(manager.get_hardlink_count(1), 2);
    }
    
    #[test]
    fn test_attributes_manager() {
        let manager = VantisfsAttributesManager::new();
        
        manager.set_extended_attribute(1, "user.comment".to_string(), b"Test".to_vec()).unwrap();
        
        assert_eq!(manager.get_extended_attribute(1, "user.comment").unwrap(), b"Test");
        
        let attrs = manager.list_extended_attributes(1).unwrap();
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0], "user.comment");
        
        manager.remove_extended_attribute(1, "user.comment").unwrap();
        assert!(manager.list_extended_attributes(1).unwrap().is_empty());
    }
}