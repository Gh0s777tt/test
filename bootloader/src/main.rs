//! VantisOS UEFI Bootloader
//! 
//! A 6-phase UEFI bootloader for VantisOS that:
//! 1. Initializes UEFI environment (Phase 1-2)
//! 2. Loads kernel.elf from EFI system partition (Phase 3)
//! 3. Parses ELF format and loads segments (Phase 4)
//! 4. Collects hardware information (Phase 5)
//! 5. Exits boot services and jumps to kernel (Phase 6)

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use uefi::{
    prelude::*,
    proto::media::fs::SimpleFileSystem,
};

// Module declarations
mod boot_info;
mod elf_loader;
mod file_system;

use elf_loader::{ElfLoader, is_valid_elf_magic};
use file_system::KernelLoader;

/// Print a message to the UEFI console
fn print(st: &mut SystemTable<Boot>, msg: &str) {
    let msg_cstring = uefi::CString16::try_from(msg).unwrap_or_else(|_| uefi::CString16::try_from("Print error").unwrap());
    st.stdout().output_string(msg_cstring.as_ref()).unwrap_or(());
}

/// Print a message with a newline
fn println(st: &mut SystemTable<Boot>, msg: &str) {
    print(st, msg);
    print(st, "\r\n");
}

/// Entry point for the UEFI bootloader
#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    // Initialize UEFI services
    uefi_services::init(&mut st).expect("Failed to initialize UEFI services");
    
    // Clear the screen
    st.stdout().clear().unwrap_or(());
    
    // Print banner
    println(&mut st, "========================================");
    println(&mut st, "    VantisOS UEFI Bootloader v0.1.0");
    println(&mut st, "========================================");
    println(&mut st, "");
    
    // Phase 3: Locate and load kernel.elf
    println(&mut st, "[Phase 3] Loading kernel.elf...");
    
    // Get the device handle we were loaded from
    let device_handle = {
        let boot_services = st.boot_services();
        let loaded_image = boot_services
            .open_protocol_exclusive::<uefi::proto::loaded_image::LoadedImage>(image)
            .expect("Failed to open Loaded Image protocol");
        loaded_image.device().expect("No device handle")
    };
    
    // Load the kernel
    let (kernel_size, kernel_data) = {
        let boot_services = st.boot_services();
        let fs_protocol = boot_services
            .open_protocol_exclusive::<SimpleFileSystem>(device_handle)
            .expect("Failed to open Simple File System protocol");
        
        let mut kernel_loader = KernelLoader::new();
        let kernel_path = uefi::CString16::try_from("\\EFI\\VANTIS\\kernel.elf").unwrap();
        
        match kernel_loader.load_kernel(boot_services, fs_protocol, kernel_path.as_ref()) {
            Ok(()) => {
                println(&mut st, "  Kernel loaded successfully!");
                let size = kernel_loader.kernel_size();
                println(&mut st, &format!("  Size: {} bytes", size));
                (size, kernel_loader.take_buffer())
            }
            Err(e) => {
                println(&mut st, &format!("  ERROR: Failed to load kernel: {:?}", e));
                return Status::LOAD_ERROR;
            }
        }
    };
    
    // Phase 4: Parse ELF
    println(&mut st, "");
    println(&mut st, "[Phase 4] Parsing ELF binary...");
    
    let kernel_data = kernel_data.expect("No kernel data");
    
    if !is_valid_elf_magic(&kernel_data) {
        println(&mut st, "  ERROR: Invalid ELF magic number!");
        return Status::LOAD_ERROR;
    }
    
    let (entry_point, loaded_size) = {
        let mut elf_loader = ElfLoader::new();
        match elf_loader.parse(unsafe { 
            core::slice::from_raw_parts(kernel_data.as_ptr(), kernel_data.len()) 
        }) {
            Ok(()) => {
                println(&mut st, "  ELF parsed successfully!");
                let entry = elf_loader.entry_point();
                let size = elf_loader.loaded_size();
                println(&mut st, &format!("  Entry point: {:#x}", entry));
                println(&mut st, &format!("  Loaded size: {:#x} bytes", size));
                (entry, size)
            }
            Err(e) => {
                println(&mut st, &format!("  ERROR: ELF parsing failed: {:?}", e));
                return Status::LOAD_ERROR;
            }
        }
    };
    
    // Phase 5: Collect hardware information
    println(&mut st, "");
    println(&mut st, "[Phase 5] Collecting hardware information...");
    
    // Get memory map size
    let memory_map_size = {
        let boot_services = st.boot_services();
        boot_services.memory_map_size().map_size
    };
    println(&mut st, &format!("  Memory map size: {} bytes", memory_map_size));
    
    // Get framebuffer info via GOP
    // GOP GUID: 9042a9de-23dc-4a38-96fb-7aded080516a
    let gop_guid = uefi::Guid::parse_or_panic("9042a9de-23dc-4a38-96fb-7aded080516a");
    
    // Extract framebuffer info first to avoid borrow conflicts
    let fb_info: Option<(u64, u64, u32, u32)> = {
        let boot_services = st.boot_services();
        let gop_handles = boot_services.locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(&gop_guid));
        match gop_handles {
            Ok(handles) => {
                if !handles.is_empty() {
                    let gop = boot_services.open_protocol_exclusive::<uefi::proto::console::gop::GraphicsOutput>(handles[0]);
                    match gop {
                        Ok(mut gop) => {
                            let mode = gop.current_mode_info();
                            let fb_base = gop.frame_buffer().as_mut_ptr() as u64;
                            let fb_size = gop.frame_buffer().size() as u64;
                            Some((fb_base, fb_size, mode.resolution().0 as u32, mode.resolution().1 as u32))
                        }
                        Err(_) => None
                    }
                } else {
                    None
                }
            }
            Err(_) => None
        }
    };
    
    // Now print the framebuffer info
    match fb_info {
        Some((fb_base, fb_size, width, height)) => {
            println(&mut st, &format!("  Framebuffer: {:#x} ({} bytes)", fb_base, fb_size));
            println(&mut st, &format!("  Resolution: {}x{}", width, height));
        }
        None => {
            println(&mut st, "  No GOP available (headless mode)");
        }
    }
    
    // Phase 6: Exit boot services and jump to kernel
    println(&mut st, "");
    println(&mut st, "[Phase 6] Exiting boot services...");
    
    // Create BootInfo structure
    let mut boot_info = boot_info::BootInfo::new();
    boot_info.kernel_entry = entry_point;
    boot_info.kernel_size = loaded_size;
    
    // Store framebuffer info if available
    if let Some((fb_base, fb_size, width, height)) = fb_info {
        boot_info.has_framebuffer = true;
        boot_info.framebuffer.write(boot_info::FramebufferInfo {
            base_address: fb_base,
            size: fb_size,
            width,
            height,
            pixels_per_scan_line: width, // Assume 1:1 for now
            pixel_format: 0, // RGB
            bits_per_pixel: 32,
        });
    }
    
    println(&mut st, "  Boot info prepared");
    println(&mut st, &format!("  Kernel entry: {:#x}", entry_point));
    println(&mut st, "");
    println(&mut st, "Goodbye from bootloader!");
    
    // Exit boot services and jump to kernel
    // This is the final step - after this we cannot use any UEFI services
    
    // Exit boot services - this returns the runtime system table and memory map
    let (_runtime_st, memory_map) = {
        st.exit_boot_services(uefi::table::boot::MemoryType::LOADER_DATA)
    };
    
    // Count memory map entries
    let mem_entries = memory_map.entries().count();
    boot_info.memory_map_size = mem_entries as u64;
    
    // Jump to kernel
    // The kernel entry point expects a pointer to BootInfo
    let kernel_entry: fn(*const boot_info::BootInfo) -> ! = unsafe {
        core::mem::transmute(entry_point as *const ())
    };
    
    // Call the kernel with boot info
    kernel_entry(&boot_info);
}