//! HDR Calibration
//! 
//! This module provides HDR calibration functionality.

use crate::verified::hdr::{HdrError, HdrType};

/// HDR calibration
#[derive(Debug)]
pub struct HdrCalibration {
    white_point: (f32, f32),
    gamma: f32,
    saturation: f32,
    contrast: f32,
}

impl HdrCalibration {
    pub fn new() -> Self {
        Self {
            white_point: (0.3127, 0.3290), // D65
            gamma: 2.2,
            saturation: 1.0,
            contrast: 1.0,
        }
    }

    pub fn with_white_point(mut self, x: f32, y: f32) -> Self {
        self.white_point = (x, y);
        self
    }

    pub fn with_gamma(mut self, gamma: f32) -> Self {
        self.gamma = gamma;
        self
    }

    pub fn with_saturation(mut self, saturation: f32) -> Self {
        self.saturation = saturation;
        self
    }

    pub fn with_contrast(mut self, contrast: f32) -> Self {
        self.contrast = contrast;
        self
    }

    pub fn calibrate(&mut self, hdr_type: HdrType) -> Result<(), HdrError> {
        match hdr_type {
            HdrType::Hdr10 => self.calibrate_hdr10(),
            HdrType::Hdr10Plus => self.calibrate_hdr10_plus(),
            HdrType::DolbyVision => self.calibrate_dolby_vision(),
            HdrType::Hlg => self.calibrate_hlg(),
        }
    }

    fn calibrate_hdr10(&mut self) -> Result<(), HdrError> {
        self.gamma = 2.4; // PQ gamma curve
        self.saturation = 1.0;
        self.contrast = 1.0;
        Ok(())
    }

    fn calibrate_hdr10_plus(&mut self) -> Result<(), HdrError> {
        self.gamma = 2.4;
        self.saturation = 1.0;
        self.contrast = 1.0;
        Ok(())
    }

    fn calibrate_dolby_vision(&mut self) -> Result<(), HdrError> {
        self.gamma = 2.4;
        self.saturation = 1.0;
        self.contrast = 1.0;
        Ok(())
    }

    fn calibrate_hlg(&mut self) -> Result<(), HdrError> {
        self.gamma = 2.0; // HLG gamma curve
        self.saturation = 1.0;
        self.contrast = 1.0;
        Ok(())
    }

    pub fn apply_gamma(&self, value: f32) -> f32 {
        value.powf(1.0 / self.gamma)
    }

    pub fn apply_saturation(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let gray = 0.299 * r + 0.587 * g + 0.114 * b;
        (
            gray + self.saturation * (r - gray),
            gray + self.saturation * (g - gray),
            gray + self.saturation * (b - gray),
        )
    }

    pub fn apply_contrast(&self, value: f32) -> f32 {
        (value - 0.5) * self.contrast + 0.5
    }

    pub fn get_white_point(&self) -> (f32, f32) {
        self.white_point
    }

    pub fn set_white_point(&mut self, x: f32, y: f32) {
        self.white_point = (x, y);
    }

    pub fn get_gamma(&self) -> f32 {
        self.gamma
    }

    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    pub fn get_saturation(&self) -> f32 {
        self.saturation
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation;
    }

    pub fn get_contrast(&self) -> f32 {
        self.contrast
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.contrast = contrast;
    }
}

impl Default for HdrCalibration {
    fn default() -> Self {
        Self::new()
    }
}
