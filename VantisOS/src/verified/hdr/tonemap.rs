//! Tone Mapping
//! 
//! This module provides tone mapping functionality for HDR to SDR conversion.

use crate::verified::hdr::{HdrError, HdrType};

/// Tone mapper for HDR to SDR conversion
#[derive(Debug)]
pub struct ToneMapper {
    peak_luminance: u32,
    target_luminance: u32,
    algorithm: ToneMapAlgorithm,
}

/// Tone mapping algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneMapAlgorithm {
    Reinhard,
    Filmic,
    Aces,
    Mobius,
    Clip,
    ReinhardJodie,
}

impl ToneMapper {
    pub fn new() -> Self {
        Self {
            peak_luminance: 1000,
            target_luminance: 100,
            algorithm: ToneMapAlgorithm::Reinhard,
        }
    }

    pub fn with_peak_luminance(mut self, peak: u32) -> Self {
        self.peak_luminance = peak;
        self
    }

    pub fn with_target_luminance(mut self, target: u32) -> Self {
        self.target_luminance = target;
        self
    }

    pub fn with_algorithm(mut self, algorithm: ToneMapAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn tone_map(&self, value: f32) -> Result<f32, HdrError> {
        if value < 0.0 || value > 1.0 {
            return Err(HdrError::ToneMappingError("Invalid input value".to_string()));
        }

        let result = match self.algorithm {
            ToneMapAlgorithm::Reinhard => self.reinhard(value),
            ToneMapAlgorithm::Filmic => self.filmic(value),
            ToneMapAlgorithm::Aces => self.aces(value),
            ToneMapAlgorithm::Mobius => self.mobius(value),
            ToneMapAlgorithm::Clip => value.min(1.0),
            ToneMapAlgorithm::ReinhardJodie => self.reinhard_jodie(value),
        };

        Ok(result)
    }

    fn reinhard(&self, value: f32) -> f32 {
        value / (value + 1.0)
    }

    fn filmic(&self, value: f32) -> f32 {
        let a = 0.15;
        let b = 0.50;
        let c = 0.10;
        let d = 0.20;
        let e = 0.02;
        let f = 0.30;

        ((value * (a * value + c * b) + d * e) / (value * (a * value + b) + d * f)) - e / f
    }

    fn aces(&self, value: f32) -> f32 {
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;

        ((value * (a * value + b)) / (value * (c * value + d) + e)).clamp(0.0, 1.0)
    }

    fn mobius(&self, value: f32) -> f32 {
        let l = value * 2.0;
        if l > 1.0 {
            let j = 1.0 / l;
            l.min(1.0)
        } else {
            l * (1.0 + l) / (1.0 + l + l * l)
        }
    }

    fn reinhard_jodie(&self, value: f32) -> f32 {
        let luminance = value;
        let reinhard = luminance / (1.0 + luminance);
        reinhard
    }

    pub fn set_algorithm(&mut self, algorithm: ToneMapAlgorithm) {
        self.algorithm = algorithm;
    }

    pub fn get_algorithm(&self) -> ToneMapAlgorithm {
        self.algorithm
    }
}

impl Default for ToneMapper {
    fn default() -> Self {
        Self::new()
    }
}
