// VantisOS USB4/Thunderbolt Security Implementation
//
// This module implements USB4/Thunderbolt security including:
// - PCIe tunneling security
// - DMA attack prevention
// - Device authentication
// - Hot-plug security

#![no_std]
#![allow(dead_code)]

use core::ptr::{NonNull, self};
use core::sync::atomic::{AtomicU64, AtomicU32, AtomicBool, Ordering};

use super::{IommuBackend, IommuError, IommuCapabilities, IommuDomain, DomainId, 
           DeviceId, VirtAddr, PhysAddr};

/// USB4/Thunderbolt register offsets
const USB4_ROUTER_CONTROL: u64 = 0x0000;
const USB4_ROUTER_STATUS: u64 = 0x0004;
const USB4_TOPOLOGY_ID: u64 = 0x0008;
const USB4_DMA_CONTROL: u64 = 0x0010;
const USB4_DMA_STATUS: u64 = 0x0014;
const USB4_SECURITY_LEVEL: u64 = 0x0020;
const USB4_AUTH_CONTROL: u64 = 0x0030;
const USB4_AUTH_STATUS: u64 = 0x0034;
const USB4_HOTPLUG_CONTROL: u64 = 0x0040;
const USB4_HOTPLUG_STATUS: u64 = 0x0044;
const USB4_PCIE_TUNNEL_CONTROL: u64 = 0x0050;
const USB4_PCIE_TUNNEL_STATUS: u64 = 0x0054;

/// USB4/Thunderbolt security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// No security
    None = 0,
    /// User level security
    User = 1,
    /// Secure level
    Secure = 2,
    /// DP only (no PCIe)
    DpOnly = 3,
    /// USB only (no PCIe)
    UsbOnly = 4,
}

/// USB4/Thunderbolt authentication status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthStatus {
    /// Not authenticated
    NotAuthenticated = 0,
    /// Authentication in progress
    InProgress = 1,
    /// Authenticated
    Authenticated = 2,
    /// Authentication failed
    Failed = 3,
}

/// USB4/Thunderbolt device information
#[derive(Debug, Clone, Copy)]
pub struct Usb4DeviceInfo {
    /// Vendor ID
    pub vendor_id: u16,
    /// Device ID
    pub device_id: u16,
    /// Revision
    pub revision: u8,
    /// Security level
    pub security_level: SecurityLevel,
    /// Authentication status
    pub auth_status: AuthStatus,
    /// PCIe tunnel enabled
    pub pcie_tunnel_enabled: bool,
    /// USB tunnel enabled
    pub usb_tunnel_enabled: bool,
}

/// USB4/Thunderbolt DMA control
#[derive(Debug, Clone, Copy)]
pub struct DmaControl {
    /// DMA enabled
    pub enabled: bool,
    /// DMA read allowed
    pub read_allowed: bool,
    /// DMA write allowed
    pub write_allowed: bool,
    /// DMA size limit
    pub size_limit: u64,
    /// DMA address range start
    pub addr_start: PhysAddr,
    /// DMA address range end
    pub addr_end: PhysAddr,
}

impl DmaControl {
    /// Create a new DMA control
    pub fn new() -> Self {
        Self {
            enabled: false,
            read_allowed: false,
            write_allowed: false,
            size_limit: 0,
            addr_start: 0,
            addr_end: 0,
        }
    }

    /// Enable DMA with restrictions
    pub fn enable_restricted(&mut self, read: bool, write: bool, 
                            size_limit: u64, addr_start: PhysAddr, addr_end: PhysAddr) {
        self.enabled = true;
        self.read_allowed = read;
        self.write_allowed = write;
        self.size_limit = size_limit;
        self.addr_start = addr_start;
        self.addr_end = addr_end;
    }

    /// Disable DMA
    pub fn disable(&mut self) {
        self.enabled = false;
        self.read_allowed = false;
        self.write_allowed = false;
    }
}

/// USB4/Thunderbolt IOMMU backend
pub struct Usb4Backend {
    /// Base address of USB4/Thunderbolt registers
    base_addr: PhysAddr,
    /// Device information
    device_info: Usb4DeviceInfo,
    /// DMA control
    dma_control: DmaControl,
    /// Domains
    domains: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<IommuDomain>; 256]>>,
    /// Enabled flag
    enabled: bool,
    /// Hot-plug enabled
    hotplug_enabled: AtomicBool,
    /// Authentication required
    auth_required: bool,
}

impl Usb4Backend {
    /// Create a new USB4/Thunderbolt backend
    pub fn new(base_addr: PhysAddr) -> Self {
        Self {
            base_addr,
            device_info: Usb4DeviceInfo {
                vendor_id: 0,
                device_id: 0,
                revision: 0,
                security_level: SecurityLevel::Secure,
                auth_status: AuthStatus::NotAuthenticated,
                pcie_tunnel_enabled: false,
                usb_tunnel_enabled: false,
            },
            dma_control: DmaControl::new(),
            domains: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            enabled: false,
            hotplug_enabled: AtomicBool::new(false),
            auth_required: true,
        }
    }

    /// Read USB4/Thunderbolt register
    fn read_reg(&self, offset: u64) -> u32 {
        unsafe {
            let addr = (self.base_addr + offset) as *const u32;
            addr.read_volatile()
        }
    }

    /// Write USB4/Thunderbolt register
    fn write_reg(&self, offset: u64, value: u32) {
        unsafe {
            let addr = (self.base_addr + offset) as *mut u32;
            addr.write_volatile(value);
        }
    }

    /// Read USB4/Thunderbolt register 64-bit
    fn read_reg64(&self, offset: u64) -> u64 {
        unsafe {
            let addr = (self.base_addr + offset) as *const u64;
            addr.read_volatile()
        }
    }

    /// Write USB4/Thunderbolt register 64-bit
    fn write_reg64(&self, offset: u64, value: u64) {
        unsafe {
            let addr = (self.base_addr + offset) as *mut u64;
            addr.write_volatile(value);
        }
    }

    /// Get device information
    fn read_device_info(&mut self) {
        let topology_id = self.read_reg64(USB4_TOPOLOGY_ID);
        self.device_info.vendor_id = ((topology_id >> 48) & 0xFFFF) as u16;
        self.device_info.device_id = ((topology_id >> 32) & 0xFFFF) as u16;
        self.device_info.revision = ((topology_id >> 24) & 0xFF) as u8;

        let security_level = self.read_reg(USB4_SECURITY_LEVEL);
        self.device_info.security_level = match security_level & 0x7 {
            0 => SecurityLevel::None,
            1 => SecurityLevel::User,
            2 => SecurityLevel::Secure,
            3 => SecurityLevel::DpOnly,
            4 => SecurityLevel::UsbOnly,
            _ => SecurityLevel::Secure,
        };

        let auth_status = self.read_reg(USB4_AUTH_STATUS);
        self.device_info.auth_status = match auth_status & 0x3 {
            0 => AuthStatus::NotAuthenticated,
            1 => AuthStatus::InProgress,
            2 => AuthStatus::Authenticated,
            3 => AuthStatus::Failed,
            _ => AuthStatus::NotAuthenticated,
        };

        let pcie_status = self.read_reg(USB4_PCIE_TUNNEL_STATUS);
        self.device_info.pcie_tunnel_enabled = (pcie_status & 0x1) != 0;

        let hotplug_status = self.read_reg(USB4_HOTPLUG_STATUS);
        self.device_info.usb_tunnel_enabled = (hotplug_status & 0x2) != 0;
    }

    /// Authenticate device
    fn authenticate_device(&mut self) -> Result<(), IommuError> {
        // Start authentication
        self.write_reg(USB4_AUTH_CONTROL, 0x1);
        
        // Wait for authentication to complete
        for _ in 0..1000 {
            let status = self.read_reg(USB4_AUTH_STATUS);
            let auth_status = status & 0x3;
            
            if auth_status == 2 {
                // Authenticated
                self.device_info.auth_status = AuthStatus::Authenticated;
                return Ok(());
            } else if auth_status == 3 {
                // Failed
                self.device_info.auth_status = AuthStatus::Failed;
                return Err(IommuError::PermissionDenied);
            }
            
            core::hint::spin_loop();
        }

        Err(IommuError::Timeout)
    }

    /// Enable PCIe tunnel
    fn enable_pcie_tunnel(&mut self) -> Result<(), IommuError> {
        if self.device_info.auth_status != AuthStatus::Authenticated {
            return Err(IommuError::PermissionDenied);
        }

        self.write_reg(USB4_PCIE_TUNNEL_CONTROL, 0x1);
        
        // Wait for tunnel to be enabled
        for _ in 0..100 {
            let status = self.read_reg(USB4_PCIE_TUNNEL_STATUS);
            if status & 0x1 != 0 {
                self.device_info.pcie_tunnel_enabled = true;
                return Ok(());
            }
            core::hint::spin_loop();
        }

        Err(IommuError::Timeout)
    }

    /// Disable PCIe tunnel
    fn disable_pcie_tunnel(&mut self) {
        self.write_reg(USB4_PCIE_TUNNEL_CONTROL, 0x0);
        self.device_info.pcie_tunnel_enabled = false;
    }

    /// Enable hot-plug
    fn enable_hotplug(&mut self) {
        self.write_reg(USB4_HOTPLUG_CONTROL, 0x1);
        self.hotplug_enabled.store(true, Ordering::SeqCst);
    }

    /// Disable hot-plug
    fn disable_hotplug(&mut self) {
        self.write_reg(USB4_HOTPLUG_CONTROL, 0x0);
        self.hotplug_enabled.store(false, Ordering::SeqCst);
    }

    /// Configure DMA restrictions
    fn configure_dma(&mut self, control: &DmaControl) {
        let mut value = 0u64;
        
        if control.enabled { value |= 0x1; }
        if control.read_allowed { value |= 0x2; }
        if control.write_allowed { value |= 0x4; }
        
        value |= (control.size_limit & 0xFFFF_FFFF) << 32;
        value |= control.addr_start & 0xFFFF_FFFF_F000;
        
        self.write_reg64(USB4_DMA_CONTROL, value);
        self.write_reg64(USB4_DMA_CONTROL + 8, control.addr_end & 0xFFFF_FFFF_F000);
        
        self.dma_control = *control;
    }

    /// Check if DMA is allowed
    fn is_dma_allowed(&self, addr: PhysAddr, size: u64, is_write: bool) -> bool {
        if !self.dma_control.enabled {
            return false;
        }

        if is_write && !self.dma_control.write_allowed {
            return false;
        }

        if !is_write && !self.dma_control.read_allowed {
            return false;
        }

        if size > self.dma_control.size_limit {
            return false;
        }

        if addr < self.dma_control.addr_start || addr >= self.dma_control.addr_end {
            return false;
        }

        true
    }
}

impl IommuBackend for Usb4Backend {
    fn name(&self) -> &str {
        "USB4/Thunderbolt"
    }

    fn init(&mut self) -> Result<(), IommuError> {
        // Read device information
        self.read_device_info();
        
        // Authenticate device if required
        if self.auth_required {
            self.authenticate_device()?;
        }

        // Disable DMA by default
        self.dma_control.disable();
        self.configure_dma(&self.dma_control);

        // Disable hot-plug by default
        self.disable_hotplug();

        Ok(())
    }

    fn capabilities(&self) -> IommuCapabilities {
        IommuCapabilities {
            address_translation: true,
            interrupt_remapping: false,
            page_fault_reporting: true,
            device_isolation: true,
            max_address_width: 48,
            max_domains: 256,
        }
    }

    fn create_domain(&mut self, id: DomainId) -> Result<IommuDomain, IommuError> {
        if id > 255 {
            return Err(IommuError::InvalidAddress);
        }

        let domain = IommuDomain::new(id)?;
        
        unsafe {
            let domains = &mut *self.domains.get();
            domains.assume_init_mut()[id as usize] = Some(domain);
        }

        Ok(domain)
    }

    fn destroy_domain(&mut self, id: DomainId) -> Result<(), IommuError> {
        if id > 255 {
            return Err(IommuError::InvalidAddress);
        }

        unsafe {
            let domains = &mut *self.domains.get();
            domains.assume_init_mut()[id as usize] = None;
        }

        Ok(())
    }

    fn map(&mut self, domain: DomainId, virt: VirtAddr, phys: PhysAddr, size: u64,
            read: bool, write: bool, execute: bool) -> Result<(), IommuError> {
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        if virt & 0xfff != 0 || phys & 0xfff != 0 {
            return Err(IommuError::InvalidAddress);
        }

        if size & 0xfff != 0 || size == 0 {
            return Err(IommuError::InvalidAddress);
        }

        // Configure DMA restrictions for this mapping
        let mut dma_control = DmaControl::new();
        dma_control.enable_restricted(read, write, size, phys, phys + size);
        self.configure_dma(&dma_control);

        Ok(())
    }

    fn unmap(&mut self, domain: DomainId, _virt: VirtAddr, _size: u64) -> Result<(), IommuError> {
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        // Disable DMA for this domain
        self.dma_control.disable();
        self.configure_dma(&self.dma_control);

        Ok(())
    }

    fn attach_device(&mut self, device: DeviceId, domain: DomainId) -> Result<(), IommuError> {
        // Enable PCIe tunnel for the device
        self.enable_pcie_tunnel()?;
        
        // Attach device to domain
        unsafe {
            if let Some(ref domains) = &*self.domains.get() {
                let domains = domains.assume_init_ref();
                if let Some(ref domain) = domains[domain as usize] {
                    domain.add_device(device)?;
                }
            }
        }

        Ok(())
    }

    fn detach_device(&mut self, device: DeviceId) -> Result<(), IommuError> {
        // Disable PCIe tunnel
        self.disable_pcie_tunnel();
        
        // Detach device from all domains
        unsafe {
            if let Some(ref domains) = &*self.domains.get() {
                let domains = domains.assume_init_ref();
                for domain in domains.iter().flatten() {
                    let _ = domain.remove_device(device);
                }
            }
        }

        Ok(())
    }

    fn flush_tlb(&mut self, _domain: Option<DomainId>) -> Result<(), IommuError> {
        // USB4/Thunderbolt doesn't have a traditional TLB
        // DMA restrictions are applied immediately
        Ok(())
    }

    fn enable(&mut self) -> Result<(), IommuError> {
        // Enable router
        self.write_reg(USB4_ROUTER_CONTROL, 0x1);
        self.enabled = true;
        Ok(())
    }

    fn disable(&mut self) -> Result<(), IommuError> {
        // Disable router
        self.write_reg(USB4_ROUTER_CONTROL, 0x0);
        self.enabled = false;
        
        // Disable all tunnels
        self.disable_pcie_tunnel();
        self.disable_hotplug();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level() {
        assert_eq!(SecurityLevel::None as u8, 0);
        assert_eq!(SecurityLevel::User as u8, 1);
        assert_eq!(SecurityLevel::Secure as u8, 2);
    }

    #[test]
    fn test_dma_control() {
        let mut control = DmaControl::new();
        assert!(!control.enabled);
        
        control.enable_restricted(true, false, 4096, 0x1000, 0x2000);
        assert!(control.enabled);
        assert!(control.read_allowed);
        assert!(!control.write_allowed);
        assert_eq!(control.size_limit, 4096);
        assert_eq!(control.addr_start, 0x1000);
        assert_eq!(control.addr_end, 0x2000);
    }

    #[test]
    fn test_usb4_backend_creation() {
        let backend = Usb4Backend::new(0xFED90000);
        assert_eq!(backend.name(), "USB4/Thunderbolt");
        assert!(!backend.enabled);
        assert!(backend.auth_required);
    }

    #[test]
    fn test_usb4_capabilities() {
        let backend = Usb4Backend::new(0xFED90000);
        let caps = backend.capabilities();
        assert!(caps.address_translation);
        assert!(!caps.interrupt_remapping);
        assert!(caps.page_fault_reporting);
        assert!(caps.device_isolation);
        assert_eq!(caps.max_address_width, 48);
        assert_eq!(caps.max_domains, 256);
    }

    #[test]
    fn test_is_dma_allowed() {
        let backend = Usb4Backend::new(0xFED90000);
        
        // DMA should not be allowed when disabled
        assert!(!backend.is_dma_allowed(0x1000, 4096, false));
    }
}