//! Primary Display Management
//! 
//! This module provides primary display selection and management.

use crate::verified::multi_monitor::{DisplayInfo, MultiMonitorError};

/// Primary display manager
#[derive(Debug)]
pub struct PrimaryDisplay {
    primary_id: Option<u32>,
}

impl PrimaryDisplay {
    pub fn new() -> Self {
        Self {
            primary_id: None,
        }
    }

    pub fn set_primary(&mut self, display_id: u32, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        let display = displays.iter_mut()
            .find(|d| d.id == display_id)
            .ok_or(MultiMonitorError::DisplayNotFound(display_id))?;
        
        // Clear previous primary
        for d in displays.iter_mut() {
            d.primary = false;
        }
        
        display.primary = true;
        self.primary_id = Some(display_id);
        Ok(())
    }

    pub fn get_primary(&self, displays: &[DisplayInfo]) -> Option<&DisplayInfo> {
        displays.iter().find(|d| d.primary)
    }

    pub fn get_primary_id(&self) -> Option<u32> {
        self.primary_id
    }

    pub fn auto_select_primary(&mut self, displays: &mut [DisplayInfo]) -> Result<(), MultiMonitorError> {
        if displays.is_empty() {
            return Err(MultiMonitorError::InvalidConfiguration);
        }

        // Prefer built-in displays, then largest display
        let primary_idx = displays.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let a_score = a.width * a.height;
                let b_score = b.width * b.height;
                a_score.cmp(&b_score)
            })
            .map(|(i, _)| i);

        if let Some(idx) = primary_idx {
            for d in displays.iter_mut() {
                d.primary = false;
            }
            displays[idx].primary = true;
            self.primary_id = Some(displays[idx].id);
        }
        Ok(())
    }

    pub fn is_primary(&self, display_id: u32, displays: &[DisplayInfo]) -> bool {
        displays.iter()
            .find(|d| d.id == display_id)
            .map(|d| d.primary)
            .unwrap_or(false)
    }
}

impl Default for PrimaryDisplay {
    fn default() -> Self {
        Self::new()
    }
}
