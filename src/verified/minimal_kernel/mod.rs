//! # Minimal Kernel
//!
//! A streamlined, production-ready microkernel for VantisOS.
//!
//! This module implements the core kernel functionality including:
//! - Memory management
//! - Process management
//! - Thread scheduling
//! - I/O system
//! - IPC integration

pub mod init;
pub mod memory;
pub mod process;
pub mod thread;
pub mod io;
pub mod ipc;

use init::kernel_init;

/// Kernel entry point
///
/// This is the main entry point for the minimal kernel.
/// It initializes all kernel subsystems and starts the scheduler.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    kernel_init();
}

/// Kernel version
pub const KERNEL_VERSION: &str = "0.4.1";
pub const KERNEL_NAME: &str = "VantisOS Minimal Kernel";

/// Kernel configuration
pub struct KernelConfig {
    /// Maximum number of processes
    pub max_processes: usize,
    /// Maximum number of threads per process
    pub max_threads_per_process: usize,
    /// Page size in bytes
    pub page_size: usize,
    /// Kernel stack size
    pub kernel_stack_size: usize,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_processes: 1024,
            max_threads_per_process: 64,
            page_size: 4096,
            kernel_stack_size: 8192,
        }
    }
}

/// Global kernel configuration
static mut KERNEL_CONFIG: Option<KernelConfig> = None;

/// Get kernel configuration
pub fn get_config() -> &'static KernelConfig {
    unsafe {
        KERNEL_CONFIG.as_ref().expect("Kernel config not initialized")
    }
}

/// Initialize kernel configuration
pub fn init_config(config: KernelConfig) {
    unsafe {
        KERNEL_CONFIG = Some(config);
    }
}