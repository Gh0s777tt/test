//! HDR Support Module
//! 
//! This module provides HDR (High Dynamic Range) display support for VantisOS,
//! including HDR10, HDR10+, and Dolby Vision support.

mod types;
mod tonemap;
mod metadata;
mod calibration;

pub use types::HdrType;
pub use tonemap::ToneMapper;
pub use metadata::HdrMetadata;
pub use calibration::HdrCalibration;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// HDR manager
#[derive(Debug)]
pub struct HdrManager {
    state: Arc<AtomicU32>,
    supported_types: Vec<HdrType>,
    enabled: bool,
    current_type: Option<HdrType>,
}

impl HdrManager {
    pub fn new() -> Result<Self, HdrError> {
        Ok(Self {
            state: Arc::new(AtomicU32::new(0)),
            supported_types: Vec::new(),
            enabled: false,
            current_type: None,
        })
    }

    pub fn initialize(&mut self) -> Result<(), HdrError> {
        self.detect_hdr_support()?;
        self.state.store(1, Ordering::SeqCst);
        Ok(())
    }

    fn detect_hdr_support(&mut self) -> Result<(), HdrError> {
        // Placeholder: Detect HDR support from display EDID
        self.supported_types.push(HdrType::Hdr10);
        self.supported_types.push(HdrType::Hlg);
        Ok(())
    }

    pub fn is_supported(&self) -> bool {
        !self.supported_types.is_empty()
    }

    pub fn get_supported_types(&self) -> &[HdrType] {
        &self.supported_types
    }

    pub fn enable(&mut self, hdr_type: HdrType) -> Result<(), HdrError> {
        if !self.supported_types.contains(&hdr_type) {
            return Err(HdrError::NotSupported);
        }
        self.enabled = true;
        self.current_type = Some(hdr_type);
        Ok(())
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.current_type = None;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_current_type(&self) -> Option<HdrType> {
        self.current_type
    }
}

impl Default for HdrManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create HdrManager"))
    }
}

/// HDR errors
#[derive(Debug, thiserror::Error)]
pub enum HdrError {
    #[error("HDR not supported")]
    NotSupported,
    #[error("Invalid HDR type")]
    InvalidType,
    #[error("Calibration failed")]
    CalibrationFailed,
    #[error("Display error: {0}")]
    DisplayError(String),
    #[error("Metadata error: {0}")]
    MetadataError(String),
    #[error("Tone mapping error: {0}")]
    ToneMappingError(String),
    #[error("EDID parse error: {0}")]
    EdidError(String),
    #[error("Permission denied")]
    PermissionDenied,
}
