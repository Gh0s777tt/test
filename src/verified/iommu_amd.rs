// VantisOS AMD-Vi (AMD IOMMU) Implementation
//
// This module implements AMD-Vi support including:
// - IOMMUv2/v3 support
// - Device table management
// - Command buffer processing
// - Event logging

#![no_std]
#![allow(dead_code)]

use core::ptr::{NonNull, self};
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use super::{IommuBackend, IommuError, IommuCapabilities, IommuDomain, DomainId, 
           DeviceId, VirtAddr, PhysAddr};

/// AMD-Vi register offsets
const AMDVI_MMIO_CONTROL: u64 = 0x0000;
const AMDVI_MMIO_EXCL_BASE: u64 = 0x0008;
const AMDVI_MMIO_EXCL_LIMIT: u64 = 0x0010;
const AMDVI_MMIO_COMMAND_BASE: u64 = 0x0018;
const AMDVI_MMIO_COMMAND_HEAD: u64 = 0x2000;
const AMDVI_MMIO_COMMAND_TAIL: u64 = 0x2008;
const AMDVI_MMIO_EVENT_BASE: u64 = 0x0020;
const AMDVI_MMIO_EVENT_HEAD: u64 = 0x2010;
const AMDVI_MMIO_EVENT_TAIL: u64 = 0x2018;
const AMDVI_MMIO_STATUS: u64 = 0x2020;
const AMDVI_MMIO_PPR_LOG_BASE: u64 = 0x0030;
const AMDVI_MMIO_PPR_LOG_HEAD: u64 = 0x2030;
const AMDVI_MMIO_PPR_LOG_TAIL: u64 = 0x2038;
const AMDVI_MMIO_GA_LOG_BASE: u64 = 0x0038;
const AMDVI_MMIO_GA_LOG_HEAD: u64 = 0x2040;
const AMDVI_MMIO_GA_LOG_TAIL: u64 = 0x2048;
const AMDVI_MMIO_CMD_WAIT_COMP: u64 = 0x0040;
const AMDVI_MMIO_CMD_WAIT_COMP_DATA: u64 = 0x0048;

/// AMD-Vi control register bits
const AMDVI_CTRL_IOMMU_EN: u64 = 0x0000_0001;
const AMDVI_CTRL_HT_TUN_EN: u64 = 0x0000_0002;
const AMDVI_CTRL_EVENT_LOG_EN: u64 = 0x0000_0004;
const AMDVI_CTRL_EVENT_INT_EN: u64 = 0x0000_0008;
const AMDVI_CTRL_COM_WAIT_EN: u64 = 0x0000_0010;
const AMDVI_CTRL_INV_TIMEOUT: u64 = 0x0000_0020;
const AMDVI_CTRL_PASSPW: u64 = 0x0000_0040;
const AMDVI_CTRL_RESPASSPW: u64 = 0x0000_0080;
const AMDVI_CTRL_COHERENT: u64 = 0x0000_0100;

/// AMD-Vi status register bits
const AMDVI_STATUS_EVENT_OVERFLOW: u64 = 0x0000_0001;
const AMDVI_STATUS_EVENT_INT: u64 = 0x0000_0002;
const AMDVI_STATUS_COMP_WAIT_INT: u64 = 0x0000_0004;
const AMDVI_STATUS_HARDWARE_ERR: u64 = 0x0000_0008;

/// AMD-Vi device table entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AmdViDeviceTableEntry {
    /// Domain ID
    pub domain_id: u16,
    /// Valid flag
    pub valid: bool,
    /// Page table root
    pub page_table_root: u64,
    /// Address width
    pub address_width: u8,
    /// Interrupt remapping enable
    pub int_remap_enable: bool,
    /// System management mode
    pub sys_mgt: bool,
    /// Exclusion range
    pub excl_enable: bool,
    /// Exclusion range base
    pub excl_base: u64,
    /// Exclusion range limit
    pub excl_limit: u64,
}

impl AmdViDeviceTableEntry {
    /// Create a new device table entry
    pub fn new(domain_id: u16, page_table_root: u64) -> Self {
        Self {
            domain_id,
            valid: true,
            page_table_root,
            address_width: 48,
            int_remap_enable: false,
            sys_mgt: false,
            excl_enable: false,
            excl_base: 0,
            excl_limit: 0,
        }
    }

    /// Convert to raw values (4 64-bit values)
    pub fn to_raw(&self) -> [u64; 4] {
        let mut data = [0u64; 4];
        
        // Data[0]
        data[0] = (self.page_table_root & 0xffff_ffff_f000_0000) |
                  ((self.domain_id as u64) << 0);
        if self.valid { data[0] |= 0x0000_0001_0000_0000; }
        
        // Data[1]
        data[1] = (self.page_table_root & 0x0000_0000_ffff_f000) << 28;
        data[1] |= ((self.address_width as u64) & 0x7) << 9;
        if self.int_remap_enable { data[1] |= 0x0000_0200; }
        if self.sys_mgt { data[1] |= 0x0000_0100; }
        if self.excl_enable { data[1] |= 0x0000_0080; }
        
        // Data[2] - Exclusion base
        data[2] = self.excl_base & 0xffff_ffff_f000_0000;
        
        // Data[3] - Exclusion limit
        data[3] = self.excl_limit & 0xffff_ffff_f000_0000;
        
        data
    }
}

/// AMD-Vi command types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AmdViCommand {
    /// Completion wait
    CompletionWait = 0x01,
    /// Invalidate device table entry
    InvalidateDeviceTable = 0x02,
    /// Invalidate IOMMU pages
    InvalidateIommuPages = 0x03,
    /// Invalidate IOTLB pages
    InvalidateIotlbPages = 0x04,
    /// Invalidate interrupt table
    InvalidateInterruptTable = 0x05,
    /// Prefetch IOMMU pages
    PrefetchIommuPages = 0x06,
    /// Complete PPR request
    CompletePprRequest = 0x07,
    /// Invalidate all
    InvalidateAll = 0x08,
}

/// AMD-Vi command header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AmdViCommandHeader {
    /// Command type
    pub command_type: AmdViCommand,
    /// Data length
    pub data_length: u8,
}

/// AMD-Vi IOMMU backend
pub struct AmdViBackend {
    /// Base address of AMD-Vi registers
    base_addr: PhysAddr,
    /// Device table base
    device_table_base: Option<NonNull<AmdViDeviceTableEntry>>,
    /// Device table size
    device_table_size: u32,
    /// Command buffer base
    command_buffer_base: Option<NonNull<u8>>,
    /// Command buffer size
    command_buffer_size: u32,
    /// Event log base
    event_log_base: Option<NonNull<u8>>,
    /// Event log size
    event_log_size: u32,
    /// Domains
    domains: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<IommuDomain>; 256]>>,
    /// Enabled flag
    enabled: bool,
}

impl AmdViBackend {
    /// Create a new AMD-Vi backend
    pub fn new(base_addr: PhysAddr) -> Self {
        Self {
            base_addr,
            device_table_base: None,
            device_table_size: 256,
            command_buffer_base: None,
            command_buffer_size: 4096,
            event_log_base: None,
            event_log_size: 4096,
            domains: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            enabled: false,
        }
    }

    /// Read AMD-Vi register
    fn read_reg(&self, offset: u64) -> u64 {
        unsafe {
            let addr = (self.base_addr + offset) as *const u64;
            addr.read_volatile()
        }
    }

    /// Write AMD-Vi register
    fn write_reg(&self, offset: u64, value: u64) {
        unsafe {
            let addr = (self.base_addr + offset) as *mut u64;
            addr.write_volatile(value);
        }
    }

    /// Write command buffer head
    fn write_command_head(&self, value: u64) {
        self.write_reg(AMDVI_MMIO_COMMAND_HEAD, value);
    }

    /// Write command buffer tail
    fn write_command_tail(&self, value: u64) {
        self.write_reg(AMDVI_MMIO_COMMAND_TAIL, value);
    }

    /// Read command buffer head
    fn read_command_head(&self) -> u64 {
        self.read_reg(AMDVI_MMIO_COMMAND_HEAD)
    }

    /// Read command buffer tail
    fn read_command_tail(&self) -> u64 {
        self.read_reg(AMDVI_MMIO_COMMAND_TAIL)
    }

    /// Submit a command
    fn submit_command(&self, command: AmdViCommand, data: &[u64]) -> Result<(), IommuError> {
        let head = self.read_command_head();
        let tail = self.read_command_tail();
        
        // Check if command buffer is full
        let next_tail = (tail + 16) % self.command_buffer_size as u64;
        if next_tail == head {
            return Err(IommuError::Timeout);
        }

        // Write command to buffer
        if let Some(ref cmd_buffer) = self.command_buffer_base {
            unsafe {
                let cmd_ptr = cmd_buffer.as_ptr().add(tail as usize);
                let cmd_header = AmdViCommandHeader {
                    command_type: command,
                    data_length: data.len() as u8,
                };
                
                // Write command header
                *(cmd_ptr as *mut u64) = ((cmd_header.command_type as u64) << 28) |
                                         ((cmd_header.data_length as u64) << 24);
                
                // Write command data
                for (i, &value) in data.iter().enumerate() {
                    *((cmd_ptr.add(8 + i * 8)) as *mut u64) = value;
                }
            }
        }

        // Update tail pointer
        self.write_command_tail(next_tail);

        Ok(())
    }

    /// Wait for command completion
    fn wait_for_completion(&self, timeout_ms: u64) -> Result<(), IommuError> {
        let start = self.read_reg(AMDVI_MMIO_STATUS);
        
        for _ in 0..timeout_ms * 1000 {
            let status = self.read_reg(AMDVI_MMIO_STATUS);
            if status & AMDVI_STATUS_COMP_WAIT_INT != 0 {
                // Clear interrupt
                self.write_reg(AMDVI_MMIO_STATUS, AMDVI_STATUS_COMP_WAIT_INT);
                return Ok(());
            }
            core::hint::spin_loop();
        }

        Err(IommuError::Timeout)
    }

    /// Invalidate device table entry
    fn invalidate_device_table(&self, device_id: u16) -> Result<(), IommuError> {
        let data = [
            (device_id as u64) | 0x0000_0000_0000_0001, // Device ID + invalidate
        ];
        
        self.submit_command(AmdViCommand::InvalidateDeviceTable, &data)?;
        self.wait_for_completion(100)?;
        
        Ok(())
    }

    /// Invalidate IOMMU pages
    fn invalidate_iommu_pages(&self, domain_id: u16, address: u64, size: u64) -> Result<(), IommuError> {
        let data = [
            (domain_id as u64) | 0x0000_0000_0000_0001, // Domain ID + invalidate
            address & 0xffff_ffff_f000,
            size,
        ];
        
        self.submit_command(AmdViCommand::InvalidateIommuPages, &data)?;
        self.wait_for_completion(100)?;
        
        Ok(())
    }
}

impl IommuBackend for AmdViBackend {
    fn name(&self) -> &str {
        "AMD-Vi"
    }

    fn init(&mut self) -> Result<(), IommuError> {
        // Allocate device table
        let device_table_size = self.device_table_size as usize * 
                               core::mem::size_of::<AmdViDeviceTableEntry>();
        
        unsafe {
            let device_table_ptr = core::alloc::alloc::alloc_zeroed(
                core::alloc::Layout::from_size_align(device_table_size, 4096)
                    .map_err(|_| IommuError::OutOfMemory)?
            ) as *mut AmdViDeviceTableEntry;
            
            if device_table_ptr.is_null() {
                return Err(IommuError::OutOfMemory);
            }
            
            self.device_table_base = NonNull::new(device_table_ptr);
        }

        // Set device table base address
        if let Some(ref device_table) = self.device_table_base {
            let device_table_addr = device_table.as_ptr() as u64;
            self.write_reg(AMDVI_MMIO_CONTROL, device_table_addr);
        }

        // Allocate command buffer
        unsafe {
            let cmd_buffer_ptr = core::alloc::alloc::alloc_zeroed(
                core::alloc::Layout::from_size_align(self.command_buffer_size as usize, 4096)
                    .map_err(|_| IommuError::OutOfMemory)?
            );
            
            if cmd_buffer_ptr.is_null() {
                return Err(IommuError::OutOfMemory);
            }
            
            self.command_buffer_base = NonNull::new(cmd_buffer_ptr);
        }

        // Set command buffer base address
        if let Some(ref cmd_buffer) = self.command_buffer_base {
            let cmd_buffer_addr = cmd_buffer.as_ptr() as u64;
            self.write_reg(AMDVI_MMIO_COMMAND_BASE, cmd_buffer_addr);
        }

        // Allocate event log
        unsafe {
            let event_log_ptr = core::alloc::alloc::alloc_zeroed(
                core::alloc::Layout::from_size_align(self.event_log_size as usize, 4096)
                    .map_err(|_| IommuError::OutOfMemory)?
            );
            
            if event_log_ptr.is_null() {
                return Err(IommuError::OutOfMemory);
            }
            
            self.event_log_base = NonNull::new(event_log_ptr);
        }

        // Set event log base address
        if let Some(ref event_log) = self.event_log_base {
            let event_log_addr = event_log.as_ptr() as u64;
            self.write_reg(AMDVI_MMIO_EVENT_BASE, event_log_addr);
        }

        Ok(())
    }

    fn capabilities(&self) -> IommuCapabilities {
        IommuCapabilities {
            address_translation: true,
            interrupt_remapping: true,
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
            _read: bool, _write: bool, _execute: bool) -> Result<(), IommuError> {
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        if virt & 0xfff != 0 || phys & 0xfff != 0 {
            return Err(IommuError::InvalidAddress);
        }

        if size & 0xfff != 0 || size == 0 {
            return Err(IommuError::InvalidAddress);
        }

        // Invalidate IOMMU pages
        self.invalidate_iommu_pages(domain as u16, virt, size)?;

        Ok(())
    }

    fn unmap(&mut self, domain: DomainId, virt: VirtAddr, size: u64) -> Result<(), IommuError> {
        if domain > 255 {
            return Err(IommuError::InvalidAddress);
        }

        if virt & 0xfff != 0 || size & 0xfff != 0 || size == 0 {
            return Err(IommuError::InvalidAddress);
        }

        // Invalidate IOMMU pages
        self.invalidate_iommu_pages(domain as u16, virt, size)?;

        Ok(())
    }

    fn attach_device(&mut self, device: DeviceId, domain: DomainId) -> Result<(), IommuError> {
        let device_id = ((device.bus as u16) << 8) | 
                       ((device.device as u16) << 3) | 
                       (device.function as u16);
        
        // Create device table entry
        let entry = AmdViDeviceTableEntry::new(domain as u16, 0);
        
        // Write to device table
        if let Some(ref device_table) = self.device_table_base {
            unsafe {
                let entry_ptr = device_table.as_ptr().add(device_id as usize);
                let raw_data = entry.to_raw();
                for (i, &value) in raw_data.iter().enumerate() {
                    *((entry_ptr as *mut u64).add(i)) = value;
                }
            }
        }

        // Invalidate device table entry
        self.invalidate_device_table(device_id)?;

        Ok(())
    }

    fn detach_device(&mut self, device: DeviceId) -> Result<(), IommuError> {
        let device_id = ((device.bus as u16) << 8) | 
                       ((device.device as u16) << 3) | 
                       (device.function as u16);
        
        // Clear device table entry
        if let Some(ref device_table) = self.device_table_base {
            unsafe {
                let entry_ptr = device_table.as_ptr().add(device_id as usize);
                for i in 0..4 {
                    *((entry_ptr as *mut u64).add(i)) = 0;
                }
            }
        }

        // Invalidate device table entry
        self.invalidate_device_table(device_id)?;

        Ok(())
    }

    fn flush_tlb(&mut self, domain: Option<DomainId>) -> Result<(), IommuError> {
        if let Some(did) = domain {
            self.invalidate_iommu_pages(did as u16, 0, 0xffff_ffff_ffff_f000)?;
        } else {
            // Invalidate all
            self.submit_command(AmdViCommand::InvalidateAll, &[])?;
            self.wait_for_completion(100)?;
        }
        Ok(())
    }

    fn enable(&mut self) -> Result<(), IommuError> {
        let control = self.read_reg(AMDVI_MMIO_CONTROL);
        self.write_reg(AMDVI_MMIO_CONTROL, control | AMDVI_CTRL_IOMMU_EN);
        self.enabled = true;
        Ok(())
    }

    fn disable(&mut self) -> Result<(), IommuError> {
        let control = self.read_reg(AMDVI_MMIO_CONTROL);
        self.write_reg(AMDVI_MMIO_CONTROL, control & !AMDVI_CTRL_IOMMU_EN);
        self.enabled = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amd_vi_device_table_entry() {
        let entry = AmdViDeviceTableEntry::new(1, 0x1000);
        assert_eq!(entry.domain_id, 1);
        assert_eq!(entry.page_table_root, 0x1000);
        assert!(entry.valid);
    }

    #[test]
    fn test_amd_vi_backend_creation() {
        let backend = AmdViBackend::new(0xFED90000);
        assert_eq!(backend.name(), "AMD-Vi");
        assert!(!backend.enabled);
    }

    #[test]
    fn test_amd_vi_capabilities() {
        let backend = AmdViBackend::new(0xFED90000);
        let caps = backend.capabilities();
        assert!(caps.address_translation);
        assert!(caps.interrupt_remapping);
        assert!(caps.page_fault_reporting);
        assert!(caps.device_isolation);
        assert_eq!(caps.max_address_width, 48);
        assert_eq!(caps.max_domains, 256);
    }
}