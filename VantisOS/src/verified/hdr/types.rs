//! HDR Types
//! 
//! This module defines HDR type constants and related structures.

/// HDR type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdrType {
    /// HDR10 (static metadata)
    Hdr10,
    /// HDR10+ (dynamic metadata)
    Hdr10Plus,
    /// Dolby Vision (dynamic metadata)
    DolbyVision,
    /// Hybrid Log-Gamma (broadcast HDR)
    Hlg,
}

impl HdrType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HdrType::Hdr10 => "HDR10",
            HdrType::Hdr10Plus => "HDR10+",
            HdrType::DolbyVision => "Dolby Vision",
            HdrType::Hlg => "HLG",
        }
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self, HdrType::Hdr10Plus | HdrType::DolbyVision)
    }

    pub fn max_nits(&self) -> u32 {
        match self {
            HdrType::Hdr10 => 1000,
            HdrType::Hdr10Plus => 4000,
            HdrType::DolbyVision => 10000,
            HdrType::Hlg => 1000,
        }
    }

    pub fn bit_depth(&self) -> u32 {
        match self {
            HdrType::Hdr10 => 10,
            HdrType::Hdr10Plus => 10,
            HdrType::DolbyVision => 12,
            HdrType::Hlg => 10,
        }
    }
}

impl std::fmt::Display for HdrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Color space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Srgb,
    Rec709,
    Rec2020,
    DciP3,
}

impl ColorSpace {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorSpace::Srgb => "sRGB",
            ColorSpace::Rec709 => "Rec.709",
            ColorSpace::Rec2020 => "Rec.2020",
            ColorSpace::DciP3 => "DCI-P3",
        }
    }

    pub fn is_wide(&self) -> bool {
        matches!(self, ColorSpace::Rec2020 | ColorSpace::DciP3)
    }
}

impl Default for HdrType {
    fn default() -> Self {
        HdrType::Hdr10
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        ColorSpace::Srgb
    }
}
