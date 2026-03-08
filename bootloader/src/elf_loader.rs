//! ELF Loader Module (Phase 4)
//! 
//! Parses and loads ELF format kernel binaries

use xmas_elf::{
    ElfFile,
    header,
    program::Type,
};

/// ELF parsing errors
#[derive(Debug, Clone, Copy)]
pub enum ElfError {
    /// Invalid ELF magic number
    InvalidMagic,
    /// Unsupported ELF class (not 64-bit)
    UnsupportedClass,
    /// Unsupported endianness
    UnsupportedEndian,
    /// No program headers found
    NoProgramHeaders,
    /// Invalid program header
    InvalidProgramHeader,
    /// No loadable segments found
    NoLoadableSegments,
    /// Memory allocation failed
    AllocationFailed,
    /// Invalid entry point
    InvalidEntryPoint,
}

/// Result type for ELF operations
pub type ElfResult<T> = core::result::Result<T, ElfError>;

/// ELF loader state
pub struct ElfLoader {
    /// The ELF file being loaded
    elf_file: Option<ElfFile<'static>>,
    /// Entry point address
    entry_point: u64,
    /// Total size of loaded segments
    loaded_size: u64,
}

impl ElfLoader {
    /// Create a new ELF loader
    pub const fn new() -> Self {
        Self {
            elf_file: None,
            entry_point: 0,
            loaded_size: 0,
        }
    }
    
    /// Parse an ELF file from a byte buffer
    /// 
    /// # Arguments
    /// * `buffer` - The raw ELF file bytes
    /// 
    /// # Returns
    /// * `Ok(())` if parsing succeeded
    /// * `Err(ElfError)` if parsing failed
    pub fn parse(&mut self, buffer: &'static [u8]) -> ElfResult<()> {
        // Validate the ELF file
        let elf_file = ElfFile::new(buffer).map_err(|_| ElfError::InvalidMagic)?;
        
        // Check the ELF header
        let header = elf_file.header;
        
        // Verify magic number using the is_valid function
        if !header::sanity_check(&elf_file).is_ok() {
            return Err(ElfError::InvalidMagic);
        }
        
        // Verify it's a 64-bit ELF (use the pointee type directly)
        if header.pt1.class() != header::Class::SixtyFour {
            return Err(ElfError::UnsupportedClass);
        }
        
        // Verify it's little-endian
        if header.pt1.data() != header::Data::LittleEndian {
            return Err(ElfError::UnsupportedEndian);
        }
        
        // Store the entry point
        self.entry_point = header.pt2.entry_point();
        
        // Count loadable segments and calculate total size
        let program_headers = elf_file.program_iter();
        let mut load_count = 0;
        let mut total_size = 0u64;
        
        for ph in program_headers {
            if ph.get_type() == Ok(Type::Load) {
                load_count += 1;
                let mem_size = ph.mem_size();
                let vaddr = ph.virtual_addr();
                
                // Track the maximum address for size calculation
                total_size = total_size.max(vaddr + mem_size);
            }
        }
        
        if load_count == 0 {
            return Err(ElfError::NoLoadableSegments);
        }
        
        self.loaded_size = total_size;
        self.elf_file = Some(elf_file);
        
        Ok(())
    }
    
    /// Load all PT_LOAD segments into memory
    /// 
    /// # Arguments
    /// * `allocate` - Function to allocate memory at a specific address
    /// * `copy` - Function to copy data to memory
    /// 
    /// # Returns
    /// * `Ok(entry_point)` on success
    /// * `Err(ElfError)` on failure
    pub fn load_segments<F, G>(&self, mut allocate: F, mut copy: G) -> ElfResult<u64>
    where
        F: FnMut(u64, u64) -> core::result::Result<*mut u8, ()>,
        G: FnMut(*mut u8, &[u8]),
    {
        let elf_file = self.elf_file.as_ref().ok_or(ElfError::NoLoadableSegments)?;
        
        for ph in elf_file.program_iter() {
            if ph.get_type() == Ok(Type::Load) {
                let vaddr = ph.virtual_addr();
                let mem_size = ph.mem_size();
                let file_size = ph.file_size();
                let offset = ph.offset();
                
                // Allocate memory for this segment
                let dest = allocate(vaddr, mem_size).map_err(|_| ElfError::AllocationFailed)?;
                
                // Clear the memory region (BSS needs to be zeroed)
                unsafe {
                    core::ptr::write_bytes(dest, 0, mem_size as usize);
                }
                
                // Copy the segment data
                let data = &elf_file.input[offset as usize..(offset + file_size) as usize];
                copy(dest, data);
            }
        }
        
        Ok(self.entry_point)
    }
    
    /// Get the entry point address
    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }
    
    /// Get the total loaded size
    pub fn loaded_size(&self) -> u64 {
        self.loaded_size
    }
}

/// Parse ELF magic number from raw bytes
/// Returns true if the magic number is valid
#[inline]
pub fn is_valid_elf_magic(buffer: &[u8]) -> bool {
    if buffer.len() < 4 {
        return false;
    }
    buffer[0] == 0x7F && buffer[1] == b'E' && buffer[2] == b'L' && buffer[3] == b'F'
}