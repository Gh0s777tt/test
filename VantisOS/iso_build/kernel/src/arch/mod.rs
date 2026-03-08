//! Architecture-specific code for x86_64

pub mod gdt;
pub mod serial;
pub mod x86_64;

pub use gdt::*;
pub use serial::*;
pub use x86_64::*;