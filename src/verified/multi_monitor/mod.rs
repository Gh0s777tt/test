//! Multi-Monitor Support Module
//! 
//! This module provides comprehensive multi-monitor support for VantisOS,
//! including display management, configuration, and layout optimization.

mod display;
mod layout;
mod primary;
mod docking;
mod mode;

pub use display::DisplayManager;
pub use layout::LayoutManager;
pub use primary::PrimaryDisplay;
pub use docking::DockingManager;
pub use mode::DisplayMode;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// Multi-monitor manager
#[derive(Debug)]
pub struct MultiMonitorManager {
    state: Arc<AtomicU32>,
    displays: Vec<DisplayInfo>,
}

impl MultiMonitorManager {
    pub fn new() -> Result<Self, MultiMonitorError> {
        Ok(Self {
            state: Arc::new(AtomicU32::new(0)),
            displays: Vec::new(),
        })
    }

    pub fn initialize(&mut self) -> Result<(), MultiMonitorError> {
        let state = self.state.load(Ordering::SeqCst);
        if state == 0 {
            self.detect_displays()?;
            self.state.store(1, Ordering::SeqCst);
        }
        Ok(())
    }

    fn detect_displays(&mut self) -> Result<(), MultiMonitorError> {
        // Placeholder: Detect all connected displays
        Ok(())
    }

    pub fn get_displays(&self) -> &[DisplayInfo] {
        &self.displays
    }

    pub fn set_layout(&mut self, layout: DisplayLayout) -> Result<(), MultiMonitorError> {
        // Placeholder: Set display layout
        Ok(())
    }

    pub fn set_primary(&mut self, display_id: u32) -> Result<(), MultiMonitorError> {
        // Placeholder: Set primary display
        Ok(())
    }
}

/// Display information
#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub id: u32,
    pub name: String,
    pub edid: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub position: (i32, i32),
    pub scale: f32,
    pub primary: bool,
    pub enabled: bool,
}

/// Display layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayLayout {
    Extend,
    Mirror,
    Span,
    OnlyPrimary,
}

/// Multi-monitor errors
#[derive(Debug, thiserror::Error)]
pub enum MultiMonitorError {
    #[error("Display not found: {0}")]
    DisplayNotFound(u32),
    #[error("Invalid display configuration")]
    InvalidConfiguration,
    #[error("Display initialization failed")]
    InitializationFailed,
    #[error("Mode not supported")]
    ModeNotSupported,
    #[error("Hardware error: {0}")]
    HardwareError(String),
    #[error("Display already connected")]
    AlreadyConnected,
    #[error("Display disconnected")]
    Disconnected,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Display busy")]
    DisplayBusy,
    #[error("Operation not supported")]
    NotSupported,
}

impl Default for MultiMonitorManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create MultiMonitorManager"))
    }
}