//! HDR Metadata
//! 
//! This module provides HDR metadata parsing and management.

use crate::verified::hdr::{HdrError, HdrType, ColorSpace};

/// HDR metadata
#[derive(Debug, Clone)]
pub struct HdrMetadata {
    pub hdr_type: HdrType,
    pub color_space: ColorSpace,
    pub max_luminance: u32,
    pub min_luminance: u32,
    pub max_cll: u32,
    pub max_fall: u32,
    pub mastering: MasteringDisplayData,
    pub content_light_level: ContentLightLevel,
}

/// Mastering display data
#[derive(Debug, Clone, Copy)]
pub struct MasteringDisplayData {
    pub primary_r: (u16, u16),
    pub primary_g: (u16, u16),
    pub primary_b: (u16, u16),
    pub white_point: (u16, u16),
    pub max_luminance: u32,
    pub min_luminance: u32,
}

/// Content light level
#[derive(Debug, Clone, Copy)]
pub struct ContentLightLevel {
    pub max_cll: u16,
    pub max_fall: u16,
}

impl HdrMetadata {
    pub fn new(hdr_type: HdrType) -> Self {
        let (max_lum, min_lum) = match hdr_type {
            HdrType::Hdr10 => (1000, 1),
            HdrType::Hdr10Plus => (4000, 1),
            HdrType::DolbyVision => (10000, 1),
            HdrType::Hlg => (1000, 1),
        };

        Self {
            hdr_type,
            color_space: ColorSpace::Rec2020,
            max_luminance: max_lum,
            min_luminance: min_lum,
            max_cll: 0,
            max_fall: 0,
            mastering: MasteringDisplayData::default(),
            content_light_level: ContentLightLevel::default(),
        }
    }

    pub fn with_color_space(mut self, color_space: ColorSpace) -> Self {
        self.color_space = color_space;
        self
    }

    pub fn with_luminance(mut self, max_luminance: u32, min_luminance: u32) -> Self {
        self.max_luminance = max_luminance;
        self.min_luminance = min_luminance;
        self
    }

    pub fn parse_from_edid(&mut self, edid: &[u8]) -> Result<(), HdrError> {
        // Placeholder: Parse HDR metadata from EDID
        Ok(())
    }

    pub fn validate(&self) -> Result<(), HdrError> {
        if self.max_luminance < self.min_luminance {
            return Err(HdrError::MetadataError("Invalid luminance range".to_string()));
        }
        Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Placeholder: Serialize to bytes
        Vec::new()
    }
}

impl Default for HdrMetadata {
    fn default() -> Self {
        Self::new(HdrType::Hdr10)
    }
}

impl Default for MasteringDisplayData {
    fn default() -> Self {
        Self {
            primary_r: (34000, 16000),
            primary_g: (13250, 34500),
            primary_b: (7500, 3000),
            white_point: (15635, 16450),
            max_luminance: 1000,
            min_luminance: 1,
        }
    }
}

impl Default for ContentLightLevel {
    fn default() -> Self {
        Self {
            max_cll: 0,
            max_fall: 0,
        }
    }
}
