//! File System Module (Phase 3)
//! 
//! Provides UEFI Simple File System Protocol access for loading the kernel

use uefi::{
    proto::media::file::{File, FileAttribute, FileInfo, FileMode},
    proto::media::fs::SimpleFileSystem,
    table::boot::{AllocateType, MemoryType, ScopedProtocol},
    CStr16,
};

/// File system errors
#[derive(Debug, Clone, Copy)]
pub enum FsError {
    /// Failed to open the file system protocol
    NoFileSystem,
    /// Failed to open the file
    FileNotFound,
    /// Failed to read the file
    ReadError,
    /// Failed to get file information
    InfoError,
    /// Failed to allocate memory
    AllocationFailed,
    /// Invalid file path
    InvalidPath,
    /// File is too large
    FileTooLarge,
}

/// Result type for file system operations
pub type FsResult<T> = core::result::Result<T, FsError>;

/// Kernel file loader using UEFI Simple File System Protocol
pub struct KernelLoader {
    /// The loaded kernel buffer
    buffer: Option<&'static mut [u8]>,
    /// Size of the loaded kernel
    size: usize,
}

impl KernelLoader {
    /// Create a new kernel loader
    pub const fn new() -> Self {
        Self {
            buffer: None,
            size: 0,
        }
    }
    
    /// Load the kernel from the EFI system partition
    /// 
    /// # Arguments
    /// * `boot_services` - UEFI boot services for memory allocation
    /// * `fs_protocol` - Simple file system protocol
    /// * `path` - Path to the kernel file (UTF-16 string)
    /// 
    /// # Returns
    /// * `Ok(&[u8])` with the kernel data on success
    /// * `Err(FsError)` on failure
    pub fn load_kernel(
        &mut self,
        boot_services: &uefi::table::boot::BootServices,
        mut fs_protocol: ScopedProtocol<SimpleFileSystem>,
        path: &CStr16,
    ) -> FsResult<()> {
        // Open the root directory
        let mut root = fs_protocol.open_volume().map_err(|_| FsError::NoFileSystem)?;
        
        // Open the kernel file
        let file = root
            .open(path, FileMode::Read, FileAttribute::empty())
            .map_err(|_| FsError::FileNotFound)?;
        
        // Get the file type - we need a regular file
        let mut file = file.into_regular_file().ok_or(FsError::FileNotFound)?;
        
        // Get file information to determine size
        let mut info_buffer = [0u8; 256];
        let info = file
            .get_info::<FileInfo>(&mut info_buffer)
            .map_err(|_| FsError::InfoError)?;
        
        let file_size = info.file_size() as usize;
        
        if file_size == 0 || file_size > 100 * 1024 * 1024 {
            // 100MB limit
            return Err(FsError::FileTooLarge);
        }
        
        // Allocate memory for the kernel
        let buffer = boot_services
            .allocate_pages(
                AllocateType::AnyPages,
                MemoryType::LOADER_DATA,
                (file_size + 4095) / 4096, // Number of 4KB pages
            )
            .map_err(|_| FsError::AllocationFailed)?;
        
        let buffer_ptr = buffer as *mut u8;
        self.buffer = Some(unsafe {
            core::slice::from_raw_parts_mut(buffer_ptr, file_size)
        });
        
        // Read the file
        let buffer_slice = self.buffer.as_mut().unwrap();
        let mut file = file;
        file.read(buffer_slice).map_err(|_| FsError::ReadError)?;
        
        self.size = file_size;
        
        Ok(())
    }
    
    /// Get the loaded kernel buffer
    pub fn kernel_buffer(&self) -> Option<&[u8]> {
        self.buffer.as_ref().map(|b| &**b)
    }
    
    /// Get the size of the loaded kernel
    pub fn kernel_size(&self) -> usize {
        self.size
    }
    
    /// Take ownership of the kernel buffer
    /// The caller is responsible for freeing the memory
    pub fn take_buffer(&mut self) -> Option<&'static mut [u8]> {
        self.buffer.take()
    }
}

/// Convert a Rust string slice to a UEFI-compatible UTF-16 string
/// This is a helper function for creating file paths
pub fn str_to_uefi_path<'a>(s: &str, buffer: &'a mut [u16]) -> Option<&'a CStr16> {
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= buffer.len() - 1 {
            return None;
        }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0; // Null terminator
    
    CStr16::from_u16_with_nul(&buffer[..=i]).ok()
}