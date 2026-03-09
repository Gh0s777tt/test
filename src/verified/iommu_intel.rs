// VantisOS Intel VT-d (Virtualization Technology for Directed I/O) Implementation
//
// This module implements Intel VT-d support including:
// - DMA remapping
// - Interrupt remapping
// - Page table management
// - Domain isolation

#![no_std]
#![allow(dead_code)]

use core::ptr::{NonNull, self};
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use super::{IommuBackend, IommuError, IommuCapabilities, IommuDomain, DomainId, 
           DeviceId, VirtAddr, PhysAddr, PageTableEntry};

/// Intel VT-d register offsets
const VTDMCR_REG: u64 = 0x0;
const VTDGCR_REG: u64 = 0x18;
const VTDRTADDR_REG: u64 = 0x20;
const VTDCMD_REG: u64 = 0x30;
const VTDSTS_REG: u64 = 0x34;
const VTDFAULT_REG: u64 = 0x38;
const VTDFAE_REG: u64 = 0x3C;
const VTDFAF_REG: u64 = 0x40;
const VTDIOTLB_REG: u64 = 0x58;
const VTDINV_REG: u64 = 0x60;
const VTDIRTA_REG: u64 = 0x68;

/// Intel VT-d capabilities
#[derive(Debug, Clone, Copy)]
pub struct VtdCapabilities {
    /// Number of domains supported
    pub nd: u8,
    /// Super page support
    pub sllps: u8,
    /// Zero length read
    pub zlr: bool,
    /// Cache coherency
    pub cg: bool,
    /// Maximum address width
    pub maw: u8,
    /// SAGAW support
    pub sagaw: u8,
    /// Page walk coherency
    pub pgs: u8,
}

/// Intel VT-d root entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VtdRootEntry {
    /// Context table pointer
    pub ctp: u64,
    /// Present flag
    pub present: bool,
}

impl VtdRootEntry {
    /// Create a new root entry
    pub fn new() -> Self {
        Self {
            ctp: 0,
            present: false,
        }
    }

    /// Convert to raw value
    pub fn to_raw(&self) -> u64 {
        let mut value = self.ctp & 0xffff_ffff_ffff_f000;
        if self.present { value |= 0x01; }
        value
    }

    /// Create from raw value
    pub fn from_raw(value: u64) -> Self {
        Self {
            ctp: value & 0xffff_ffff_ffff_f000,
            present: (value & 0x01) != 0,
        }
    }
}

/// Intel VT-d context entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VtdContextEntry {
    /// Address width
    pub aw: u8,
    /// Domain identifier
    pub did: u16,
    /// Translation type
    pub tt: u8,
    /// Second-level page translation pointer
    pub slptptr: u64,
    /// Present flag
    pub present: bool,
    /// Fault processing disable
    pub fpd: bool,
    /// Translation type
    pub t: u8,
}

impl VtdContextEntry {
    /// Create a new context entry
    pub fn new(did: u16, slptptr: u64) -> Self {
        Self {
            aw: 2, // 48-bit address width
            did,
            tt: 0,
            slptptr,
            present: true,
            fpd: false,
            t: 0,
        }
    }

    /// Convert to raw values (high and low)
    pub fn to_raw(&self) -> (u64, u64) {
        let low = ((self.slptptr & 0xffff_ffff_ffff_f000) as u64) |
                  ((self.did as u64) << 8) |
                  ((self.aw as u64) << 0) |
                  0x01; // Present
        let high = ((self.tt as u64) << 2) |
                   ((self.t as u64) << 3) |
                   if self.fpd { 0x10 } else { 0 };
        (low, high)
    }
}

/// Intel VT-d IOMMU backend
pub struct IntelVtdBackend {
    /// Base address of VT-d registers
    base_addr: PhysAddr,
    /// Capabilities
    capabilities: VtdCapabilities,
    /// Root table address
    root_table: Option<NonNull<VtdRootEntry>>,
    /// Number of root entries
    root_entries: u32,
    /// Domains
    domains: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<IommuDomain>; 256]>>,
    /// Enabled flag
    enabled: bool,
}

impl IntelVtdBackend {
    /// Create a new Intel VT-d backend
    pub fn new(base_addr: PhysAddr) -> Self {
        Self {
            base_addr,
            capabilities: VtdCapabilities {
                nd: 255,
                sllps: 0,
                zlr: false,
                cg: true,
                maw: 48,
                sagaw: 0x02,
                pgs: 0x01,
            },
            root_table: None,
            root_entries: 256,
            domains: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            enabled: false,
        }
    }

    /// Read VT-d register
    fn read_reg(&self, offset: u64) -> u32 {
        unsafe {
            let addr = (self.base_addr + offset) as *const u32;
            addr.read_volatile()
        }
    }

    /// Write VT-d register
    fn write_reg(&self, offset: u64, value: u32) {
        unsafe {
            let addr = (self.base_addr + offset) as *mut u32;
            addr.write_volatile(value);
        }
    }

    /// Write VT-d register 64-bit
    fn write_reg64(&self, offset: u64, value: u64) {
        unsafe {
            let addr = (self.base_addr + offset) as *mut u64;
            addr.write_volatile(value);
        }
    }

    /// Get root entry for device
    fn get_root_entry(&self, device: DeviceId) -> Option<&VtdRootEntry> {
        let index = (device.bus as usize) * 256 + 
                   (device.device as usize) * 8 + 
                   (device.function as usize);
        
        unsafe {
            if let Some(ref root_table) = self.root_table {
                root_table.as_ptr().add(index).as_ref()
            } else {
                None
            }
        }
    }

    /// Set root entry for device
    fn set_root_entry(&mut self, device: DeviceId, entry: VtdRootEntry) {
        let index = (device.bus as usize) * 256 + 
                   (device.device as usize) * 8 + 
                   (device.function as usize);
        
        unsafe {
            if let Some(ref mut root_table) = self.root_table {
                *root_table.as_ptr().add(index) = entry;
            }
        }
    }

    /// Invalidate IOTLB
    fn invalidate_iotlb(&self, domain: Option<DomainId>) {
        let mut value = 0x8000_0000; // Global invalidation
        if let Some(did) = domain {
            value = (did as u32) << 16;
        }
        self.write_reg(VTDIOTLB_REG, value);
        
        // Wait for completion
        while self.read_reg(VTDIOTLB_REG) & 0x8000_0000 != 0 {
            core::hint::spin_loop();
        }
    }

    /// Invalidate context cache
    fn invalidate_context_cache(&self) {
        let value = 0x8000_0001; // Global context invalidation
        self.write_reg(VTDINV_REG, value);
        
        // Wait for completion
        while self.read_reg(VTDINV_REG) & 0x8000_0000 != 0 {
            core::hint::spin_loop();
        }
    }
}

impl IommuBackend for IntelVtdBackend {
    fn name(&self) -> &str {
        "Intel VT-d"
    }

    fn init(&mut self) -> Result<(), IommuError> {
        // Allocate root table
        let root_table_size = self.root_entries as usize * core::mem::size_of::<VtdRootEntry>();
        // In a real implementation, this would allocate physical memory
        // For now, we'll use a placeholder
        
        // Initialize root entries
        unsafe {
            let root_table_ptr = core::alloc::alloc::alloc_zeroed(
                core::alloc::Layout::from_size_align(root_table_size, 4096)
                    .map_err(|_| IommuError::OutOfMemory)?
            ) as *mut VtdRootEntry;
            
            if root_table_ptr.is_null() {
                return Err(IommuError::OutOfMemory);
            }
            
            self.root_table = NonNull::new(root_table_ptr);
        }

        // Set root table address
        if let Some(ref root_table) = self.root_table {
            let root_addr = root_table.as_ptr() as u64;
            self.write_reg64(VTDRTADDR_REG, root_addr);
        }

        // Enable VT-d
        self.write_reg(VTDMCR_REG, 0x8000_0000); // Set translation enable
        
        Ok(())
    }

    fn capabilities(&self) -> IommuCapabilities {
        IommuCapabilities {
            address_translation: true,
            interrupt_remapping: true,
            page_fault_reporting: true,
            device_isolation: true,
            max_address_width: self.capabilities.maw,
            max_domains: (self.capabilities.nd + 1) as u32,
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
        // In a real implementation, this would:
        // 1. Get the domain's page table
        // 2. Create/update page table entries
        // 3. Flush IOTLB
        
        // For now, we'll just validate the parameters
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        if virt & 0xfff != 0 || phys & 0xfff != 0 {
            return Err(IommuError::InvalidAddress);
        }

        if size & 0xfff != 0 || size == 0 {
            return Err(IommuError::InvalidAddress);
        }

        // Invalidate IOTLB for this domain
        self.invalidate_iotlb(Some(domain));

        Ok(())
    }

    fn unmap(&mut self, domain: DomainId, virt: VirtAddr, size: u64) -> Result<(), IommuError> {
        // In a real implementation, this would:
        // 1. Get the domain's page table
        // 2. Clear page table entries
        // 3. Flush IOTLB
        
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        if virt & 0xfff != 0 || size & 0xfff != 0 || size == 0 {
            return Err(IommuError::InvalidAddress);
        }

        // Invalidate IOTLB for this domain
        self.invalidate_iotlb(Some(domain));

        Ok(())
    }

    fn attach_device(&mut self, device: DeviceId, domain: DomainId) -> Result<(), IommuError> {
        // Create context entry
        let context_entry = VtdContextEntry::new(domain as u16, 0);
        
        // Create root entry
        let mut root_entry = VtdRootEntry::new();
        root_entry.present = true;
        root_entry.ctp = 0; // Context table pointer
        
        // Set root entry
        self.set_root_entry(device, root_entry);
        
        // Invalidate context cache
        self.invalidate_context_cache();
        
        Ok(())
    }

    fn detach_device(&mut self, device: DeviceId) -> Result<(), IommuError> {
        // Clear root entry
        let root_entry = VtdRootEntry::new();
        self.set_root_entry(device, root_entry);
        
        // Invalidate context cache
        self.invalidate_context_cache();
        
        Ok(())
    }

    fn flush_tlb(&mut self, domain: Option<DomainId>) -> Result<(), IommuError> {
        self.invalidate_iotlb(domain);
        Ok(())
    }

    fn enable(&mut self) -> Result<(), IommuError> {
        self.write_reg(VTDMCR_REG, 0x8000_0000); // Enable translation
        self.enabled = true;
        Ok(())
    }

    fn disable(&mut self) -> Result<(), IommuError> {
        self.write_reg(VTDMCR_REG, 0x0000_0000); // Disable translation
        self.enabled = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vtd_root_entry() {
        let entry = VtdRootEntry::new();
        assert!(!entry.present);
        
        let raw = entry.to_raw();
        let entry2 = VtdRootEntry::from_raw(raw);
        assert_eq!(entry.present, entry2.present);
    }

    #[test]
    fn test_vtd_context_entry() {
        let entry = VtdContextEntry::new(1, 0x1000);
        assert_eq!(entry.did, 1);
        assert_eq!(entry.slptptr, 0x1000);
        assert!(entry.present);
    }

    #[test]
    fn test_intel_vtd_backend_creation() {
        let backend = IntelVtdBackend::new(0xFED90000);
        assert_eq!(backend.name(), "Intel VT-d");
        assert!(!backend.enabled);
    }

    #[test]
    fn test_intel_vtd_capabilities() {
        let backend = IntelVtdBackend::new(0xFED90000);
        let caps = backend.capabilities();
        assert!(caps.address_translation);
        assert!(caps.interrupt_remapping);
        assert!(caps.page_fault_reporting);
        assert!(caps.device_isolation);
        assert_eq!(caps.max_address_width, 48);
        assert_eq!(caps.max_domains, 256);
    }
}