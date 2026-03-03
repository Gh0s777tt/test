//! Edge Computing Module
//! 
//! This module provides edge computing functionality for VantisOS including
//! edge computing framework, local processing, cloud synchronization, offline mode,
//! and data aggregation.

pub mod framework;
pub mod processing;
pub mod sync;
pub mod offline;
pub mod aggregation;

pub use framework::*;
pub use processing::*;
pub use sync::*;
pub use offline::*;
pub use aggregation::*;

/// Initialize edge computing
pub fn init() {
    // Initialize edge computing framework
    framework::init();
    
    // Initialize local processing
    processing::init();
    
    // Initialize cloud synchronization
    sync::init();
    
    // Initialize offline mode
    offline::init();
    
    // Initialize data aggregation
    aggregation::init();
}

/// Get edge computing information
pub fn get_info() -> &'static str {
    "Edge Computing v0.7.0"
}