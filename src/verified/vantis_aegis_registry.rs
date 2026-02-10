//! Vantis Aegis - Registry Emulation Layer
//! 
//! This module provides emulation of the Windows Registry to make VantisOS
//! appear as Windows to anti-cheat systems and other software that queries
//! the registry.
//!
//! # Design Philosophy
//! 
//! - In-memory registry structure
//! - Pre-populated with critical Windows keys
//! - Fast query performance
//! - Consistent values across queries
//!
//! # Legal Notice
//! 
//! This implementation uses only publicly documented Windows Registry keys
//! from official Microsoft documentation. No proprietary information is used.

use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Registry error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Key not found
    KeyNotFound(String),
    /// Value not found
    ValueNotFound(String),
    /// Access denied
    AccessDenied,
    /// Invalid parameter
    InvalidParameter,
    /// Buffer too small
    BufferTooSmall,
}

/// Registry result type
pub type RegistryResult<T> = Result<T, RegistryError>;

/// Registry value types
#[derive(Debug, Clone, PartialEq)]
pub enum RegistryValue {
    /// String value (REG_SZ)
    String(String),
    /// Expandable string value (REG_EXPAND_SZ)
    ExpandString(String),
    /// Binary value (REG_BINARY)
    Binary(Vec<u8>),
    /// DWORD value (REG_DWORD)
    DWord(u32),
    /// QWORD value (REG_QWORD)
    QWord(u64),
    /// Multi-string value (REG_MULTI_SZ)
    MultiString(Vec<String>),
}

/// Registry key structure
#[derive(Debug, Clone)]
pub struct RegistryKey {
    /// Key path
    path: String,
    /// Key values
    values: HashMap<String, RegistryValue>,
    /// Subkeys
    subkeys: HashMap<String, RegistryKey>,
}

impl RegistryKey {
    /// Create a new registry key
    pub fn new(path: String) -> Self {
        Self {
            path,
            values: HashMap::new(),
            subkeys: HashMap::new(),
        }
    }
    
    /// Add a value to this key
    pub fn add_value(&mut self, name: String, value: RegistryValue) {
        self.values.insert(name, value);
    }
    
    /// Add a subkey to this key
    pub fn add_subkey(&mut self, name: String, key: RegistryKey) {
        self.subkeys.insert(name, key);
    }
    
    /// Get a value from this key
    pub fn get_value(&self, name: &str) -> Option<&RegistryValue> {
        self.values.get(name)
    }
    
    /// Get a subkey from this key
    pub fn get_subkey(&self, name: &str) -> Option<&RegistryKey> {
        self.subkeys.get(name)
    }
    
    /// Enumerate all value names
    pub fn enumerate_values(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }
    
    /// Enumerate all subkey names
    pub fn enumerate_subkeys(&self) -> Vec<String> {
        self.subkeys.keys().cloned().collect()
    }
}

/// Registry emulator
pub struct RegistryEmulator {
    /// HKEY_LOCAL_MACHINE
    hklm: Arc<RwLock<RegistryKey>>,
    /// HKEY_CURRENT_USER
    hkcu: Arc<RwLock<RegistryKey>>,
    /// HKEY_CLASSES_ROOT
    hkcr: Arc<RwLock<RegistryKey>>,
}

impl RegistryEmulator {
    /// Create a new registry emulator
    pub fn new() -> Self {
        let mut emulator = Self {
            hklm: Arc::new(RwLock::new(RegistryKey::new("HKEY_LOCAL_MACHINE".to_string()))),
            hkcu: Arc::new(RwLock::new(RegistryKey::new("HKEY_CURRENT_USER".to_string()))),
            hkcr: Arc::new(RwLock::new(RegistryKey::new("HKEY_CLASSES_ROOT".to_string()))),
        };
        
        emulator.populate_system_keys();
        emulator
    }
    
    /// Get the global registry emulator instance
    pub fn instance() -> &'static RegistryEmulator {
        static INSTANCE: OnceLock<RegistryEmulator> = OnceLock::new();
        INSTANCE.get_or_init(RegistryEmulator::new)
    }
    
    /// Populate system registry keys
    fn populate_system_keys(&mut self) {
        self.populate_system_information();
        self.populate_windows_version();
        self.populate_hardware_information();
        self.populate_network_information();
    }
    
    /// Populate system information keys
    fn populate_system_information(&mut self) {
        let mut hklm = self.hklm.write().unwrap();
        
        // HKLM\SYSTEM\CurrentControlSet\Control\SystemInformation
        let mut sys_info = RegistryKey::new(
            "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\SystemInformation".to_string()
        );
        sys_info.add_value("SystemProductName".to_string(), 
                          RegistryValue::String("VantisOS Gaming Edition".to_string()));
        sys_info.add_value("SystemManufacturer".to_string(),
                          RegistryValue::String("VantisCorp".to_string()));
        sys_info.add_value("SystemSKU".to_string(),
                          RegistryValue::String("VOS-001".to_string()));
        
        // Navigate to the correct location and add the key
        self.add_key_path(&mut hklm, "SYSTEM\\CurrentControlSet\\Control\\SystemInformation", sys_info);
    }
    
    /// Populate Windows version keys
    fn populate_windows_version(&mut self) {
        let mut hklm = self.hklm.write().unwrap();
        
        // HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion
        let mut version = RegistryKey::new(
            "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion".to_string()
        );
        version.add_value("ProductName".to_string(),
                         RegistryValue::String("Windows 11 Pro".to_string()));
        version.add_value("EditionID".to_string(),
                         RegistryValue::String("Professional".to_string()));
        version.add_value("CurrentVersion".to_string(),
                         RegistryValue::String("10.0".to_string()));
        version.add_value("CurrentBuild".to_string(),
                         RegistryValue::String("22631".to_string()));
        version.add_value("CurrentBuildNumber".to_string(),
                         RegistryValue::String("22631".to_string()));
        version.add_value("BuildLabEx".to_string(),
                         RegistryValue::String("22631.1.amd64fre.ni_release.220506-1250".to_string()));
        version.add_value("CurrentMajorVersionNumber".to_string(),
                         RegistryValue::DWord(10));
        version.add_value("CurrentMinorVersionNumber".to_string(),
                         RegistryValue::DWord(0));
        version.add_value("InstallationType".to_string(),
                         RegistryValue::String("Client".to_string()));
        version.add_value("ProductId".to_string(),
                         RegistryValue::String("00000-00000-00000-AAOEM".to_string()));
        version.add_value("RegisteredOwner".to_string(),
                         RegistryValue::String("VantisOS User".to_string()));
        version.add_value("RegisteredOrganization".to_string(),
                         RegistryValue::String("".to_string()));
        
        self.add_key_path(&mut hklm, "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", version);
    }
    
    /// Populate hardware information keys
    fn populate_hardware_information(&mut self) {
        let mut hklm = self.hklm.write().unwrap();
        
        // Get actual CPU information
        let cpu_name = Self::get_cpu_name();
        let cpu_vendor = Self::get_cpu_vendor();
        
        // HKLM\HARDWARE\DESCRIPTION\System\CentralProcessor\0
        let mut cpu = RegistryKey::new(
            "HKEY_LOCAL_MACHINE\\HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0".to_string()
        );
        cpu.add_value("ProcessorNameString".to_string(),
                     RegistryValue::String(cpu_name));
        cpu.add_value("VendorIdentifier".to_string(),
                     RegistryValue::String(cpu_vendor));
        cpu.add_value("Identifier".to_string(),
                     RegistryValue::String("Intel64 Family 6 Model 158 Stepping 10".to_string()));
        cpu.add_value("~MHz".to_string(),
                     RegistryValue::DWord(3600)); // 3.6 GHz
        
        self.add_key_path(&mut hklm, "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", cpu);
        
        // HKLM\HARDWARE\DESCRIPTION\System\BIOS
        let mut bios = RegistryKey::new(
            "HKEY_LOCAL_MACHINE\\HARDWARE\\DESCRIPTION\\System\\BIOS".to_string()
        );
        bios.add_value("SystemManufacturer".to_string(),
                      RegistryValue::String("VantisCorp".to_string()));
        bios.add_value("SystemProductName".to_string(),
                      RegistryValue::String("VantisOS Gaming PC".to_string()));
        bios.add_value("BIOSVendor".to_string(),
                      RegistryValue::String("American Megatrends Inc.".to_string()));
        bios.add_value("BIOSVersion".to_string(),
                      RegistryValue::String("1.0.0".to_string()));
        
        self.add_key_path(&mut hklm, "HARDWARE\\DESCRIPTION\\System\\BIOS", bios);
    }
    
    /// Populate network information keys
    fn populate_network_information(&mut self) {
        let mut hklm = self.hklm.write().unwrap();
        
        // HKLM\SYSTEM\CurrentControlSet\Control\ComputerName\ActiveComputerName
        let mut computer_name = RegistryKey::new(
            "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName".to_string()
        );
        computer_name.add_value("ComputerName".to_string(),
                               RegistryValue::String(Self::get_hostname()));
        
        self.add_key_path(&mut hklm, "SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName", computer_name);
    }
    
    /// Add a key at a specific path
    fn add_key_path(&self, root: &mut RegistryKey, path: &str, key: RegistryKey) {
        let parts: Vec<&str> = path.split('\\').collect();
        let mut current = root;
        
        // Navigate/create path
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part - add the key
                current.add_subkey(part.to_string(), key.clone());
                break;
            } else {
                // Intermediate part - ensure it exists
                if !current.subkeys.contains_key(*part) {
                    current.add_subkey(part.to_string(), RegistryKey::new(part.to_string()));
                }
                current = current.subkeys.get_mut(*part).unwrap();
            }
        }
    }
    
    /// Get CPU name from /proc/cpuinfo
    fn get_cpu_name() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        return name.trim().to_string();
                    }
                }
            }
        }
        "Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz".to_string()
    }
    
    /// Get CPU vendor from /proc/cpuinfo
    fn get_cpu_vendor() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("vendor_id") {
                    if let Some(vendor) = line.split(':').nth(1) {
                        let vendor = vendor.trim();
                        return match vendor {
                            "GenuineIntel" => "GenuineIntel".to_string(),
                            "AuthenticAMD" => "AuthenticAMD".to_string(),
                            _ => vendor.to_string(),
                        };
                    }
                }
            }
        }
        "GenuineIntel".to_string()
    }
    
    /// Get hostname
    fn get_hostname() -> String {
        if let Ok(hostname) = std::fs::read_to_string("/etc/hostname") {
            return hostname.trim().to_string();
        }
        "VANTIS-PC".to_string()
    }
    
    // ========================================================================
    // PUBLIC API FUNCTIONS
    // ========================================================================
    
    /// Open a registry key
    /// 
    /// # Arguments
    /// * `path` - Full registry path (e.g., "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
    pub fn open_key(&self, path: &str) -> RegistryResult<Arc<RwLock<RegistryKey>>> {
        let (root_name, subpath) = self.parse_path(path)?;
        
        let root = match root_name.as_str() {
            "HKEY_LOCAL_MACHINE" | "HKLM" => self.hklm.clone(),
            "HKEY_CURRENT_USER" | "HKCU" => self.hkcu.clone(),
            "HKEY_CLASSES_ROOT" | "HKCR" => self.hkcr.clone(),
            _ => return Err(RegistryError::KeyNotFound(root_name.to_string())),
        };
        
        if subpath.is_empty() {
            return Ok(root);
        }
        
        // Navigate to the key
        {
            let root_lock = root.read().unwrap();
            let mut current = &*root_lock;
            
            for part in subpath.split('\\') {
                if let Some(subkey) = current.get_subkey(part) {
                    current = subkey;
                } else {
                    return Err(RegistryError::KeyNotFound(path.to_string()));
                }
            }
        } // Drop the lock here
        
        // We can't return a reference to a subkey, so we'll return the root
        // In a real implementation, we'd need a more sophisticated approach
        Ok(root)
    }
    
    /// Query a registry value
    /// 
    /// # Arguments
    /// * `key_path` - Full registry key path
    /// * `value_name` - Name of the value to query
    pub fn query_value(&self, key_path: &str, value_name: &str) -> RegistryResult<RegistryValue> {
        let (root_name, subpath) = self.parse_path(key_path)?;
        
        let root = match root_name.as_str() {
            "HKEY_LOCAL_MACHINE" | "HKLM" => self.hklm.read().unwrap(),
            "HKEY_CURRENT_USER" | "HKCU" => self.hkcu.read().unwrap(),
            "HKEY_CLASSES_ROOT" | "HKCR" => self.hkcr.read().unwrap(),
            _ => return Err(RegistryError::KeyNotFound(root_name.to_string())),
        };
        
        let mut current = &*root;
        
        // Navigate to the key
        if !subpath.is_empty() {
            for part in subpath.split('\\') {
                if let Some(subkey) = current.get_subkey(part) {
                    current = subkey;
                } else {
                    return Err(RegistryError::KeyNotFound(key_path.to_string()));
                }
            }
        }
        
        // Get the value
        current.get_value(value_name)
            .cloned()
            .ok_or_else(|| RegistryError::ValueNotFound(value_name.to_string()))
    }
    
    /// Enumerate subkeys of a registry key
    /// 
    /// # Arguments
    /// * `key_path` - Full registry key path
    pub fn enumerate_subkeys(&self, key_path: &str) -> RegistryResult<Vec<String>> {
        let (root_name, subpath) = self.parse_path(key_path)?;
        
        let root = match root_name.as_str() {
            "HKEY_LOCAL_MACHINE" | "HKLM" => self.hklm.read().unwrap(),
            "HKEY_CURRENT_USER" | "HKCU" => self.hkcu.read().unwrap(),
            "HKEY_CLASSES_ROOT" | "HKCR" => self.hkcr.read().unwrap(),
            _ => return Err(RegistryError::KeyNotFound(root_name.to_string())),
        };
        
        let mut current = &*root;
        
        // Navigate to the key
        if !subpath.is_empty() {
            for part in subpath.split('\\') {
                if let Some(subkey) = current.get_subkey(part) {
                    current = subkey;
                } else {
                    return Err(RegistryError::KeyNotFound(key_path.to_string()));
                }
            }
        }
        
        Ok(current.enumerate_subkeys())
    }
    
    /// Enumerate values of a registry key
    /// 
    /// # Arguments
    /// * `key_path` - Full registry key path
    pub fn enumerate_values(&self, key_path: &str) -> RegistryResult<Vec<String>> {
        let (root_name, subpath) = self.parse_path(key_path)?;
        
        let root = match root_name.as_str() {
            "HKEY_LOCAL_MACHINE" | "HKLM" => self.hklm.read().unwrap(),
            "HKEY_CURRENT_USER" | "HKCU" => self.hkcu.read().unwrap(),
            "HKEY_CLASSES_ROOT" | "HKCR" => self.hkcr.read().unwrap(),
            _ => return Err(RegistryError::KeyNotFound(root_name.to_string())),
        };
        
        let mut current = &*root;
        
        // Navigate to the key
        if !subpath.is_empty() {
            for part in subpath.split('\\') {
                if let Some(subkey) = current.get_subkey(part) {
                    current = subkey;
                } else {
                    return Err(RegistryError::KeyNotFound(key_path.to_string()));
                }
            }
        }
        
        Ok(current.enumerate_values())
    }
    
    // ========================================================================
    // HELPER FUNCTIONS
    // ========================================================================
    
    /// Parse a registry path into root and subpath
    fn parse_path(&self, path: &str) -> RegistryResult<(String, String)> {
        let parts: Vec<&str> = path.split('\\').collect();
        if parts.is_empty() {
            return Err(RegistryError::InvalidParameter);
        }
        
        let root = parts[0].to_string();
        let subpath = if parts.len() > 1 {
            parts[1..].join("\\")
        } else {
            String::new()
        };
        
        Ok((root, subpath))
    }
}

impl Default for RegistryEmulator {
    fn default() -> Self {
        Self::new()
    }
}

