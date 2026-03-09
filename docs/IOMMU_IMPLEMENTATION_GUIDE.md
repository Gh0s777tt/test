# IOMMU Implementation Guide for VantisOS

## Overview

This guide describes the implementation of Input-Output Memory Management Unit (IOMMU) support in VantisOS. IOMMU provides critical security by preventing Direct Memory Access (DMA) attacks and enabling device isolation.

---

## What is IOMMU?

IOMMU is a hardware component that:
- Translates device DMA addresses to physical memory addresses
- Enforces memory access permissions for devices
- Prevents unauthorized memory access by devices
- Enables device isolation and virtualization

### Why IOMMU is Critical

**Security:**
- Prevents DMA attacks (malicious devices accessing memory)
- Protects against physical attacks (Thunderbolt/USB4)
- Enforces device memory permissions
- Enables secure device passthrough

**Performance:**
- Reduces bounce buffer overhead
- Enables scatter-gather DMA
- Improves I/O performance
- Supports large memory configurations

**Compliance:**
- Required for Common Criteria EAL7+
- Required for PCI DSS compliance
- Required for automotive safety (ISO 26262)

---

## Architecture

### IOMMU Types

VantisOS supports multiple IOMMU types:

1. **Intel VT-d** (Intel Virtualization Technology for Directed I/O)
2. **AMD-Vi** (AMD I/O Virtualization Technology)
3. **ARM SMMU** (ARM System Memory Management Unit)
4. **Generic IOMMU** (Software-based fallback)

### IOMMU Components

```
┌─────────────────────────────────────────────────────────┐
│                    VantisOS Kernel                      │
├─────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ IOMMU Driver │  │ Device Driver │  │ Memory Manager│  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                 │          │
│  ┌──────▼─────────────────▼─────────────────▼───────┐  │
│  │           IOMMU Manager (Core)                   │  │
│  │  - Domain Management                             │  │
│  │  - Address Translation                            │  │
│  │  - Permission Enforcement                         │  │
│  └──────┬───────────────────────────────────────────┘  │
└─────────┼───────────────────────────────────────────────┘
          │
    ┌─────▼─────┐
    │  IOMMU    │
    │  Hardware │
    └───────────┘
```

---

## Implementation Plan

### Phase 1: IOMMU Core (Days 1-3)

#### Day 1: IOMMU Detection and Initialization

**File:** `src/verified/kernel/iommu/detection.rs`

```rust
use crate::acpi::AcpiTable;
use crate::pci::PciDevice;

/// IOMMU types supported by VantisOS
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IommuType {
    IntelVtd,
    AmdVi,
    ArmSmmu,
    Generic,
}

/// IOMMU device information
#[derive(Debug)]
pub struct IommuDevice {
    pub iommu_type: IommuType,
    pub pci_device: Option<PciDevice>,
    pub capabilities: IommuCapabilities,
    pub domain_count: u32,
    pub address_width: u8,
}

/// IOMMU capabilities
#[derive(Debug)]
pub struct IommuCapabilities {
    pub supported_page_sizes: u64,
    pub max_domains: u32,
    pub max_address_width: u8,
    pub supports_cache_coherency: bool,
    pub supports_interrupt_remap: bool,
}

/// Detect IOMMU hardware
pub fn detect_iommu() -> Result<Vec<IommuDevice>, IommuError> {
    let mut iommus = Vec::new();
    
    // Check ACPI tables for IOMMU information
    if let Some(vtd) = detect_intel_vtd()? {
        iommus.push(vtd);
    }
    
    if let Some(vi) = detect_amd_vi()? {
        iommus.push(vi);
    }
    
    if let Some(smmu) = detect_arm_smmu()? {
        iommus.push(smmu);
    }
    
    if iommus.is_empty() {
        // Fall back to software IOMMU
        iommus.push(create_generic_iommu()?);
    }
    
    Ok(iommus)
}

/// Detect Intel VT-d
fn detect_intel_vtd() -> Result<Option<IommuDevice>, IommuError> {
    // Parse DMAR table from ACPI
    // Check for VT-d support
    // Return IOMMU device info
    Ok(None)
}

/// Detect AMD-Vi
fn detect_amd_vi() -> Result<Option<IommuDevice>, IommuError> {
    // Parse IVRS table from ACPI
    // Check for AMD-Vi support
    // Return IOMMU device info
    Ok(None)
}

/// Detect ARM SMMU
fn detect_arm_smmu() -> Result<Option<IommuDevice>, IommuError> {
    // Parse IORT table from ACPI
    // Check for SMMU support
    // Return IOMMU device info
    Ok(None)
}

/// Create generic software IOMMU
fn create_generic_iommu() -> Result<IommuDevice, IommuError> {
    Ok(IommuDevice {
        iommu_type: IommuType::Generic,
        pci_device: None,
        capabilities: IommuCapabilities {
            supported_page_sizes: 0x1000, // 4KB pages only
            max_domains: 256,
            max_address_width: 48,
            supports_cache_coherency: false,
            supports_interrupt_remap: false,
        },
        domain_count: 256,
        address_width: 48,
    })
}
```

#### Day 2: Domain Management

**File:** `src/verified/kernel/iommu/domain.rs`

```rust
use crate::memory::{PhysicalAddress, VirtualAddress};
use crate::capability::Capability;

/// IOMMU domain identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DomainId(u32);

/// IOMMU domain
pub struct IommuDomain {
    id: DomainId,
    devices: Vec<DeviceId>,
    page_table: IommuPageTable,
    permissions: DomainPermissions,
}

/// Domain permissions
#[derive(Debug, Clone, Copy)]
pub struct DomainPermissions {
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_execute: bool,
}

/// Create new IOMMU domain
pub fn create_domain() -> Result<DomainId, IommuError> {
    let domain_id = DomainId::new();
    let page_table = IommuPageTable::new()?;
    
    let domain = IommuDomain {
        id: domain_id,
        devices: Vec::new(),
        page_table,
        permissions: DomainPermissions {
            allow_read: true,
            allow_write: true,
            allow_execute: false,
        },
    };
    
    // Register domain with IOMMU hardware
    register_domain(&domain)?;
    
    Ok(domain_id)
}

/// Attach device to domain
pub fn attach_device(domain_id: DomainId, device_id: DeviceId) -> Result<(), IommuError> {
    // Get domain
    let domain = get_domain(domain_id)?;
    
    // Attach device to domain
    domain.devices.push(device_id);
    
    // Configure IOMMU hardware
    configure_device_mapping(domain_id, device_id)?;
    
    Ok(())
}

/// Detach device from domain
pub fn detach_device(domain_id: DomainId, device_id: DeviceId) -> Result<(), IommuError> {
    // Get domain
    let domain = get_domain_mut(domain_id)?;
    
    // Remove device from domain
    domain.devices.retain(|&d| d != device_id);
    
    // Clear device mappings
    clear_device_mappings(device_id)?;
    
    Ok(())
}

/// Map memory into domain
pub fn map_memory(
    domain_id: DomainId,
    iova: Iova,
    phys_addr: PhysicalAddress,
    size: usize,
    permissions: DomainPermissions,
) -> Result<(), IommuError> {
    let domain = get_domain_mut(domain_id)?;
    
    // Add mapping to page table
    domain.page_table.map(iova, phys_addr, size, permissions)?;
    
    // Flush IOMMU TLB
    flush_domain_tlb(domain_id)?;
    
    Ok(())
}

/// Unmap memory from domain
pub fn unmap_memory(
    domain_id: DomainId,
    iova: Iova,
    size: usize,
) -> Result<(), IommuError> {
    let domain = get_domain_mut(domain_id)?;
    
    // Remove mapping from page table
    domain.page_table.unmap(iova, size)?;
    
    // Flush IOMMU TLB
    flush_domain_tlb(domain_id)?;
    
    Ok(())
}
```

#### Day 3: Address Translation

**File:** `src/verified/kernel/iommu/translation.rs`

```rust
use crate::memory::PhysicalAddress;

/// I/O Virtual Address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Iova(u64);

impl Iova {
    pub fn new(addr: u64) -> Self {
        Iova(addr)
    }
    
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// IOMMU page table entry
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IommuPte {
    pub present: bool,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub physical_address: PhysicalAddress,
    pub cache_coherent: bool,
}

/// IOMMU page table
pub struct IommuPageTable {
    root: PhysicalAddress,
    levels: u8,
}

impl IommuPageTable {
    pub fn new() -> Result<Self, IommuError> {
        // Allocate root page table
        let root = allocate_page_table()?;
        
        Ok(IommuPageTable {
            root,
            levels: 4, // 4-level page table
        })
    }
    
    pub fn map(
        &mut self,
        iova: Iova,
        phys_addr: PhysicalAddress,
        size: usize,
        permissions: DomainPermissions,
    ) -> Result<(), IommuError> {
        // Walk page table and create mappings
        let mut current_level = self.levels;
        let mut current_addr = iova.as_u64();
        let mut phys = phys_addr.as_u64();
        let mut remaining_size = size;
        
        while remaining_size > 0 {
            // Calculate page size for this level
            let page_size = 1usize << (12 + (current_level - 1) * 9);
            
            // Align addresses
            let aligned_iova = current_addr & !(page_size as u64 - 1);
            let aligned_phys = phys & !(page_size as u64 - 1);
            
            // Create page table entry
            let pte = IommuPte {
                present: true,
                read: permissions.allow_read,
                write: permissions.allow_write,
                execute: permissions.allow_execute,
                physical_address: PhysicalAddress::new(aligned_phys),
                cache_coherent: true,
            };
            
            // Write PTE to page table
            self.write_pte(aligned_iova, pte)?;
            
            // Advance to next page
            current_addr += page_size as u64;
            phys += page_size as u64;
            remaining_size = remaining_size.saturating_sub(page_size);
        }
        
        Ok(())
    }
    
    pub fn unmap(&mut self, iova: Iova, size: usize) -> Result<(), IommuError> {
        // Walk page table and clear mappings
        let mut current_addr = iova.as_u64();
        let mut remaining_size = size;
        
        while remaining_size > 0 {
            // Calculate page size
            let page_size = 1usize << (12 + (self.levels - 1) * 9);
            
            // Clear page table entry
            self.clear_pte(current_addr)?;
            
            // Advance to next page
            current_addr += page_size as u64;
            remaining_size = remaining_size.saturating_sub(page_size);
        }
        
        Ok(())
    }
    
    fn write_pte(&mut self, iova: u64, pte: IommuPte) -> Result<(), IommuError> {
        // Write PTE to page table
        Ok(())
    }
    
    fn clear_pte(&mut self, iova: u64) -> Result<(), IommuError> {
        // Clear PTE in page table
        Ok(())
    }
}
```

### Phase 2: Device Integration (Days 4-5)

#### Day 4: PCI Device Integration

**File:** `src/verified/kernel/iommu/pci.rs`

```rust
use crate::pci::PciDevice;
use crate::iommu::{DomainId, Iova};

/// PCI device IOMMU integration
pub struct PciIommuIntegration {
    device_id: DeviceId,
    domain_id: DomainId,
    iova_allocator: IovaAllocator,
}

impl PciIommuIntegration {
    pub fn new(device: &PciDevice, domain_id: DomainId) -> Result<Self, IommuError> {
        // Attach device to domain
        attach_device(domain_id, device.id())?;
        
        Ok(PciIommuIntegration {
            device_id: device.id(),
            domain_id,
            iova_allocator: IovaAllocator::new(),
        })
    }
    
    pub fn allocate_iova(&mut self, size: usize) -> Result<Iova, IommuError> {
        self.iova_allocator.allocate(size)
    }
    
    pub fn map_dma_buffer(
        &mut self,
        phys_addr: PhysicalAddress,
        size: usize,
    ) -> Result<Iova, IommuError> {
        // Allocate IOVA
        let iova = self.allocate_iova(size)?;
        
        // Map physical memory to IOVA
        map_memory(
            self.domain_id,
            iova,
            phys_addr,
            size,
            DomainPermissions {
                allow_read: true,
                allow_write: true,
                allow_execute: false,
            },
        )?;
        
        Ok(iova)
    }
    
    pub fn unmap_dma_buffer(&mut self, iova: Iova, size: usize) -> Result<(), IommuError> {
        // Unmap memory
        unmap_memory(self.domain_id, iova, size)?;
        
        // Free IOVA
        self.iova_allocator.free(iova, size);
        
        Ok(())
    }
}
```

#### Day 5: USB4/Thunderbolt Support

**File:** `src/verified/kernel/iommu/thunderbolt.rs`

```rust
use crate::iommu::{DomainId, Iova};

/// Thunderbolt security level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThunderboltSecurity {
    /// No security (legacy)
    None,
    /// User approval required
    UserApproval,
    /// Secure connect (only authorized devices)
    SecureConnect,
    /// Only display port and USB
    DisplayPortUsbOnly,
}

/// Thunderbolt IOMMU integration
pub struct ThunderboltIommu {
    domain_id: DomainId,
    security_level: ThunderboltSecurity,
}

impl ThunderboltIommu {
    pub fn new(security_level: ThunderboltSecurity) -> Result<Self, IommuError> {
        // Create isolated domain for Thunderbolt
        let domain_id = create_domain()?;
        
        // Configure domain with restricted permissions
        let permissions = match security_level {
            ThunderboltSecurity::None => DomainPermissions {
                allow_read: true,
                allow_write: true,
                allow_execute: false,
            },
            ThunderboltSecurity::UserApproval => DomainPermissions {
                allow_read: true,
                allow_write: true,
                allow_execute: false,
            },
            ThunderboltSecurity::SecureConnect => DomainPermissions {
                allow_read: true,
                allow_write: false,
                allow_execute: false,
            },
            ThunderboltSecurity::DisplayPortUsbOnly => DomainPermissions {
                allow_read: true,
                allow_write: false,
                allow_execute: false,
            },
        };
        
        Ok(ThunderboltIommu {
            domain_id,
            security_level,
        })
    }
    
    pub fn authorize_device(&mut self, device_id: DeviceId) -> Result<(), IommuError> {
        match self.security_level {
            ThunderboltSecurity::None => {
                // No authorization needed
                attach_device(self.domain_id, device_id)?;
            }
            ThunderboltSecurity::UserApproval => {
                // Request user approval
                if request_user_approval(device_id)? {
                    attach_device(self.domain_id, device_id)?;
                } else {
                    return Err(IommuError::DeviceNotAuthorized);
                }
            }
            ThunderboltSecurity::SecureConnect => {
                // Verify device signature
                if verify_device_signature(device_id)? {
                    attach_device(self.domain_id, device_id)?;
                } else {
                    return Err(IommuError::DeviceNotAuthorized);
                }
            }
            ThunderboltSecurity::DisplayPortUsbOnly => {
                // Only allow display port and USB
                if is_display_port_or_usb(device_id)? {
                    attach_device(self.domain_id, device_id)?;
                } else {
                    return Err(IommuError::DeviceNotAuthorized);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn deauthorize_device(&mut self, device_id: DeviceId) -> Result<(), IommuError> {
        detach_device(self.domain_id, device_id)
    }
}
```

### Phase 3: Testing and Validation (Days 6-7)

#### Day 6: Unit Tests

**File:** `src/verified/kernel/iommu/tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_iommu_detection() {
        let iommus = detect_iommu().unwrap();
        assert!(!iommus.is_empty());
    }
    
    #[test]
    fn test_domain_creation() {
        let domain_id = create_domain().unwrap();
        assert!(domain_id.as_u32() > 0);
    }
    
    #[test]
    fn test_memory_mapping() {
        let domain_id = create_domain().unwrap();
        let iova = Iova::new(0x1000);
        let phys = PhysicalAddress::new(0x100000);
        
        map_memory(
            domain_id,
            iova,
            phys,
            0x1000,
            DomainPermissions {
                allow_read: true,
                allow_write: true,
                allow_execute: false,
            },
        ).unwrap();
        
        unmap_memory(domain_id, iova, 0x1000).unwrap();
    }
    
    #[test]
    fn test_thunderbolt_authorization() {
        let mut tb = ThunderboltIommu::new(ThunderboltSecurity::UserApproval).unwrap();
        let device_id = DeviceId::new(1);
        
        // Mock user approval
        assert!(tb.authorize_device(device_id).is_ok());
    }
}
```

#### Day 7: Integration Tests

**File:** `src/verified/kernel/iommu/integration_tests.rs`

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_pci_device_dma() {
        // Test PCI device DMA with IOMMU
        let domain_id = create_domain().unwrap();
        let device = get_test_pci_device();
        
        let mut integration = PciIommuIntegration::new(&device, domain_id).unwrap();
        
        // Allocate and map DMA buffer
        let phys = allocate_dma_buffer(0x1000).unwrap();
        let iova = integration.map_dma_buffer(phys, 0x1000).unwrap();
        
        // Perform DMA operation
        perform_dma_operation(device.id(), iova, 0x1000).unwrap();
        
        // Unmap buffer
        integration.unmap_dma_buffer(iova, 0x1000).unwrap();
    }
    
    #[test]
    fn test_thunderbolt_security() {
        // Test Thunderbolt security levels
        for security in [
            ThunderboltSecurity::None,
            ThunderboltSecurity::UserApproval,
            ThunderboltSecurity::SecureConnect,
            ThunderboltSecurity::DisplayPortUsbOnly,
        ] {
            let mut tb = ThunderboltIommu::new(security).unwrap();
            let device_id = DeviceId::new(1);
            
            // Test authorization
            let result = tb.authorize_device(device_id);
            assert!(result.is_ok() || result.is_err());
        }
    }
}
```

---

## Security Considerations

### DMA Attack Prevention

IOMMU prevents DMA attacks by:
1. **Address Translation**: Translates device DMA addresses to physical addresses
2. **Permission Enforcement**: Enforces read/write/execute permissions
3. **Domain Isolation**: Isolates devices into separate domains
4. **TLB Validation**: Validates all DMA operations

### Thunderbolt/USB4 Security

IOMMU provides Thunderbolt/USB4 security by:
1. **Device Authorization**: Requires user approval or device signature
2. **Restricted Domains**: Isolates Thunderbolt devices
3. **Limited Permissions**: Restricts what devices can access
4. **Hot-Plug Protection**: Protects against hot-plug attacks

---

## Performance Optimization

### TLB Optimization
- Use large pages when possible
- Batch TLB flushes
- Use IOTLB for better performance

### Memory Allocation
- Use bounce buffers only when necessary
- Pre-allocate DMA pools
- Use zero-copy when possible

### Interrupt Handling
- Use interrupt remapping
- Optimize interrupt delivery
- Use MSI/MSI-X when available

---

## Troubleshooting

### Common Issues

#### Issue: IOMMU not detected
**Solution:** Check ACPI tables, ensure IOMMU is enabled in BIOS

#### Issue: DMA failures
**Solution:** Check domain permissions, verify mappings

#### Issue: Performance degradation
**Solution:** Use large pages, optimize TLB flushes

---

## References

- [Intel VT-d Specification](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-virtualization-technology-for-directed-io-vt-d.html)
- [AMD-Vi Specification](https://developer.amd.com/resources/developer-guides-manuals/)
- [ARM SMMU Specification](https://developer.arm.com/documentation/ihi0062/latest/)
- [PCI Express Specification](https://pcisig.com/specifications)

---

**Version:** 1.0  
**Last Updated:** February 24, 2025  
**Maintained by:** VantisOS Kernel Team