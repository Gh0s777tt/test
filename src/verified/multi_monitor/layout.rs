//! Layout Management
//! 
//! This module provides display layout management functionality.

use crate::verified::multi_monitor::{DisplayLayout, DisplayInfo, MultiMonitorError};

/// Layout manager
#[derive(Debug)]
pub struct LayoutManager {
    layout: DisplayLayout,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            layout: DisplayLayout::Extend,
        }
    }

    pub fn set_layout(&mut self, layout: DisplayLayout) {
        self.layout = layout;
    }

    pub fn get_layout(&self) -> DisplayLayout {
        self.layout
    }

    pub fn apply_layout(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        match self.layout {
            DisplayLayout::Extend => self.apply_extend(displays),
            DisplayLayout::Mirror => self.apply_mirror(displays),
            DisplayLayout::Span => self.apply_span(displays),
            DisplayLayout::OnlyPrimary => self.apply_only_primary(displays),
        }
    }

    fn apply_extend(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        let mut x_offset = 0;
        for display in displays.iter_mut() {
            if display.enabled {
                display.position = (x_offset, 0);
                x_offset += display.width as i32;
            }
        }
        Ok(())
    }

    fn apply_mirror(&self, displays: &mut [DisplayInfo>) -> Result<(), MultiMonitorError> {
        let primary = displays.iter()
            .find(|d| d.primary)
            .ok_or(MultiMonitorError::InvalidConfiguration)?;
        
        for display in displays.iter_mut() {
            if display.enabled {
                display.position = (0, 0);
            }
        }
        Ok(())
    }

    fn apply_span(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        let mut x_offset = 0;
        let mut y_offset = 0;
        
        for display in displays.iter_mut() {
            if display.enabled {
                display.position = (x_offset, y_offset);
                x_offset += display.width as i32;
                if x_offset >= 4000 {
                    x_offset = 0;
                    y_offset += display.height as i32;
                }
            }
        }
        Ok(())
    }

    fn apply_only_primary(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        for display in displays.iter_mut() {
            if !display.primary {
                display.enabled = false;
            }
        }
        Ok(())
    }

    pub fn optimize_layout(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        // Optimize layout based on display capabilities
        Ok(())
    }

    pub fn auto_arrange(&self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        let mut sorted_displays: Vec<_> = displays.iter_mut().collect();
        sorted_displays.sort_by(|a, b| b.height.cmp(&a.height));
        
        let mut x_offset = 0;
        for display in sorted_displays {
            if display.enabled {
                display.position = (x_offset, 0);
                x_offset += display.width as i32;
            }
        }
        Ok(())
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
