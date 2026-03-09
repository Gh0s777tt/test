//! Display Mode Management
//! 
//! This module provides display mode (resolution/refresh rate) management.

use crate::verified::multi_monitor::{MultiMonitorError};

/// Display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub preferred: bool,
    pub interlaced: bool,
}

impl DisplayMode {
    pub fn new(width: u32, height: u32, refresh_rate: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate,
            preferred: false,
            interlaced: false,
        }
    }

    pub fn preferred(mut self) -> Self {
        self.preferred = true;
        self
    }

    pub fn interlaced(mut self) -> Self {
        self.interlaced = true;
        self
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn is_high_dpi(&self) -> bool {
        self.width >= 3840 || self.height >= 2160
    }

    pub fn aspect_ratio(&self) -> (u32, u32) {
        let gcd = self.width.gcd(self.height);
        (self.width / gcd, self.height / gcd)
    }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::new(1920, 1080, 60)
    }
}

/// Common display modes
pub const MODE_1920x1080_60: DisplayMode = DisplayMode {
    width: 1920,
    height: 1080,
    refresh_rate: 60,
    preferred: true,
    interlaced: false,
};

pub const MODE_2560x1440_60: DisplayMode = DisplayMode {
    width: 2560,
    height: 1440,
    refresh_rate: 60,
    preferred: false,
    interlaced: false,
};

pub const MODE_3840x2160_60: DisplayMode = DisplayMode {
    width: 3840,
    height: 2160,
    refresh_rate: 60,
    preferred: false,
    interlaced: false,
};

pub const MODE_1920x1080_144: DisplayMode = DisplayMode {
    width: 1920,
    height: 1080,
    refresh_rate: 144,
    preferred: false,
    interlaced: false,
};

/// Display mode manager
#[derive(Debug)]
pub struct ModeManager {
    modes: Vec<DisplayMode>,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            modes: Vec::new(),
        }
    }

    pub fn add_mode(&mut self, mode: DisplayMode) {
        self.modes.push(mode);
    }

    pub fn remove_mode(&mut self, width: u32, height: u32) {
        self.modes.retain(|m| m.width != width || m.height != height);
    }

    pub fn get_modes(&self) -> &[DisplayMode] {
        &self.modes
    }

    pub fn get_preferred_mode(&self) -> Option<&DisplayMode> {
        self.modes.iter().find(|m| m.preferred)
    }

    pub fn get_best_mode(&self) -> Option<&DisplayMode> {
        self.modes.iter()
            .max_by_key(|m| (m.width, m.height, m.refresh_rate))
    }

    pub fn find_mode(&self, width: u32, height: u32, refresh_rate: u32) -> Option<&DisplayMode> {
        self.modes.iter()
            .find(|m| m.width == width && m.height == height && m.refresh_rate == refresh_rate)
    }

    pub fn set_mode(&mut self, mode: DisplayMode) -> Result<(), MultiMonitorError> {
        // Placeholder: Set display mode
        Ok(())
    }
}

impl Default for ModeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Common EDID modes
pub fn get_common_modes() -> Vec<DisplayMode> {
    vec![
        DisplayMode::new(1920, 1080, 60).preferred(),
        DisplayMode::new(1920, 1080, 144),
        DisplayMode::new(1920, 1080, 120),
        DisplayMode::new(2560, 1440, 60),
        DisplayMode::new(2560, 1440, 144),
        DisplayMode::new(3840, 2160, 60),
        DisplayMode::new(3840, 2160, 120),
        DisplayMode::new(1366, 768, 60),
        DisplayMode::new(1280, 720, 60),
    ]
}
