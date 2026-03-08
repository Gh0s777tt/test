//! Boot Information Module (Phase 5)
//! 
//! Contains structures for passing hardware information to the kernel

use core::mem::MaybeUninit;

/// Memory map entry describing a region of physical memory
#[repr(C)]
pub struct MemoryMapEntry {
    /// Physical start address of the memory region
    pub physical_start: u64,
    /// Virtual start address (if mapped)
    pub virtual_start: u64,
    /// Number of 4KB pages in this region
    pub number_of_pages: u64,
    /// Memory attributes (cacheable, writable, etc.)
    pub attributes: u64,
    /// Type of memory (conventional, reserved, etc.)
    pub memory_type: u32,
}

/// Framebuffer information for graphical output
#[repr(C)]
pub struct FramebufferInfo {
    /// Base address of the framebuffer
    pub base_address: u64,
    /// Size of the framebuffer in bytes
    pub size: u64,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixels per scan line
    pub pixels_per_scan_line: u32,
    /// Pixel format (RGB, BGR, etc.)
    pub pixel_format: u32,
    /// Bits per pixel
    pub bits_per_pixel: u32,
}

/// Boot information passed to the kernel
/// This structure contains all hardware information collected during boot
#[repr(C)]
pub struct BootInfo {
    /// Magic number for validation (0x56414E544953 = "VANTIS")
    pub magic: u64,
    /// Version of the BootInfo structure
    pub version: u32,
    /// Reserved for alignment
    pub reserved: u32,
    /// Pointer to the memory map
    pub memory_map: *mut MemoryMapEntry,
    /// Number of entries in the memory map
    pub memory_map_size: u64,
    /// Memory map descriptor size
    pub memory_descriptor_size: u64,
    /// Framebuffer information
    pub framebuffer: MaybeUninit<FramebufferInfo>,
    /// Whether framebuffer is available
    pub has_framebuffer: bool,
    /// ACPI RSDP address (if available)
    pub acpi_rsdp_address: u64,
    /// Kernel virtual entry point
    pub kernel_entry: u64,
    /// Kernel image size
    pub kernel_size: u64,
}

impl BootInfo {
    /// Magic number constant for validation
    pub const MAGIC: u64 = 0x56414E544953; // "VANTIS" in hex
    
    /// Current version of the BootInfo structure
    pub const VERSION: u32 = 1;
    
    /// Create a new BootInfo with default values
    pub const fn new() -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            reserved: 0,
            memory_map: core::ptr::null_mut(),
            memory_map_size: 0,
            memory_descriptor_size: 0,
            framebuffer: MaybeUninit::uninit(),
            has_framebuffer: false,
            acpi_rsdp_address: 0,
            kernel_entry: 0,
            kernel_size: 0,
        }
    }
}

/// Memory types matching UEFI memory types
#[repr(u32)]
pub enum MemoryType {
    /// Unusable memory
    Reserved = 0,
    /// Code for the loader
    LoaderCode = 1,
    /// Data for the loader
    LoaderData = 2,
    /// Runtime services code
    BootServicesCode = 3,
    /// Runtime services data
    BootServicesData = 4,
    /// Runtime driver code
    RuntimeServicesCode = 5,
    /// Runtime driver data
    RuntimeServicesData = 6,
    /// Free memory
    ConventionalMemory = 7,
    /// Memory with errors
    UnusableMemory = 8,
    /// ACPI reclaimable memory
    ACPIReclaimMemory = 9,
    /// ACPI NVS memory
    ACPIMemoryNVS = 10,
    /// Memory mapped I/O
    MemoryMappedIO = 11,
    /// Memory mapped I/O port space
    MemoryMappedIOPortSpace = 12,
    /// Pal code
    PalCode = 13,
    /// Persistent memory
    PersistentMemory = 14,
}