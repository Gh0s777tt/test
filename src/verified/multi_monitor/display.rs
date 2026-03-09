//! Display Management
//! 
//! This module provides display detection and management functionality.

use crate::verified::multi_monitor::{DisplayInfo, MultiMonitorError};

/// Display manager
#[derive(Debug)]
pub struct DisplayManager {
    displays: Vec<DisplayInfo>,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self {
            displays: Vec::new(),
        }
    }

    pub fn detect_displays(&mut self) -> Result<(), MultiMonitorError> {
        // Placeholder: Detect all connected displays
        Ok(())
    }

    pub fn add_display(&mut self, display: DisplayInfo) -> Result<(), MultiMonitorError> {
        if self.displays.iter().any(|d| d.id == display.id) {
            return Err(MultiMonitorError::AlreadyConnected);
        }
        self.displays.push(display);
        Ok(())
    }

    pub fn remove_display(&mut self, display_id: u32) -> Result<(), MultiMonitorError> {
        let pos = self.displays.iter().position(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        self.displays.remove(pos);
        Ok(())
    }

    pub fn get_display(&self, display_id: u32) -> Result<&DisplayInfo, MultiMonitorError> {
        self.displays.iter()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))
    }

    pub fn get_displays(&self) -> &[DisplayInfo] {
        &self.displays
    }

    pub fn enable_display(&mut self, display_id: u32) -> Result<(), MultiMonitorError> {
        let display = self.displays.iter_mut()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        display.enabled = true;
        Ok(())
    }

    pub fn disable_display(&mut self, display_id: u32) -> Result<(), MultiMonitorError> {
        let display = self.displays.iter_mut()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        display.enabled = false;
        Ok(())
    }

    pub fn set_display_scale(&mut self, display_id: u32, scale: f32) -> Result<(), MultiMonitorError> {
        let display = self.displays.iter_mut()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        display.scale = scale;
        Ok(())
    }

    pub fn set_display_position(&mut self, display_id: u32, x: i32, y: i32) -> Result<(), MultiMonitorError> {
        let display = self.displays.iter_mut()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        display.position = (x, y);
        Ok(())
    }

    pub fn get_display_count(&self) -> usize {
        self.displays.len()
    }

    pub fn get_active_display_count(&self) -> usize {
        self.displays.iter().filter(|d| d.enabled).count()
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}
