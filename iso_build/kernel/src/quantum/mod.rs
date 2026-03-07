//! Quantum computing module for VantisOS

use spin::Mutex;

/// Initialize quantum module
pub fn init() {
    crate::arch::serial_println!("[OK] Quantum module initialized");
}