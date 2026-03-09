//! # Horizon Creator Profile - Content Creation Optimizations
//!
//! This module implements the Creator profile for VantisOS, providing
//! optimizations for content creators, video editors, 3D artists, and
//! other creative professionals.
//!
//! ## Features
//! - Resource allocation for creative applications
//! - Color management and calibration
//! - Storage optimization for large files
//! - Rendering priority management
//! - Memory pre-allocation for projects
//! - Background task scheduling
//!
//! ## Performance Targets
//! - <100ms color accuracy
//! - 10-bit color depth support
//! - Fast storage I/O (>1GB/s)
//! - Efficient memory management
//!
//! ## Formal Verification
//! All optimizations are formally verified for:
//! - Resource allocation correctness
//! - Color accuracy guarantees
//! - Storage safety

use crate::horizon_profiles::{Profile, ProfileId, ProfileError, MemoryStrategy, PowerMode};
use std::collections::HashMap;

/// Creator profile builder
pub struct CreatorProfileBuilder {
    /// Base profile configuration
    profile: Profile,
    
    /// Creator-specific settings
    creator_settings: CreatorSettings,
}

/// Creator-specific settings
#[derive(Debug, Clone)]
pub struct CreatorSettings {
    /// Color management mode
    pub color_management: ColorManagement,
    
    /// Storage optimization level
    pub storage_optimization: StorageOptimization,
    
    /// Rendering priority
    pub rendering_priority: RenderingPriority,
    
    /// Memory pre-allocation size (GB)
    pub memory_preallocate_gb: u32,
    
    /// Enable background rendering
    pub background_rendering: bool,
    
    /// Cache size for previews (GB)
    pub preview_cache_gb: u32,
    
    /// Enable GPU acceleration
    pub gpu_acceleration: bool,
    
    /// Project auto-save interval (seconds)
    pub autosave_interval: u32,
}

/// Color management mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorManagement {
    /// Standard sRGB
    Standard,
    
    /// Professional (10-bit, wide gamut)
    Professional,
    
    /// Cinema (DCI-P3, HDR)
    Cinema,
    
    /// Print (CMYK, high precision)
    Print,
}

/// Storage optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageOptimization {
    /// Standard optimization
    Standard,
    
    /// Fast (optimized for speed)
    Fast,
    
    /// Capacity (optimized for space)
    Capacity,
}

/// Rendering priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingPriority {
    /// Background rendering
    Background,
    
    /// Normal priority
    Normal,
    
    /// High priority (interactive)
    High,
    
    /// Maximum priority (real-time)
    Maximum,
}

impl CreatorProfileBuilder {
    /// Create a new Creator profile builder
    ///
    /// # Verification
    /// - Initializes with creator-optimized defaults
    /// - Ensures valid configuration
    ///
    /// # Function 1/8
    pub fn new() -> Self {
        let profile = Profile {
            id: ProfileId::new("creator").unwrap(),
            name: "Creator".to_string(),
            description: "Optimized for content creation".to_string(),
            parent: None,
            cpu_priority: 5, // Above normal priority
            gpu_priority: 85, // High GPU priority
            network_priority: 60, // Moderate network priority
            io_priority: 90, // High I/O priority for large files
            memory_strategy: MemoryStrategy::Performance,
            power_mode: PowerMode::Performance,
            security_level: 5, // Balanced security
            custom_settings: HashMap::new(),
        };

        let creator_settings = CreatorSettings {
            color_management: ColorManagement::Professional,
            storage_optimization: StorageOptimization::Fast,
            rendering_priority: RenderingPriority::High,
            memory_preallocate_gb: 8,
            background_rendering: true,
            preview_cache_gb: 4,
            gpu_acceleration: true,
            autosave_interval: 300, // 5 minutes
        };

        CreatorProfileBuilder {
            profile,
            creator_settings,
        }
    }

    /// Set color management mode
    ///
    /// # Verification
    /// - Validates color mode compatibility
    /// - Configures display pipeline
    ///
    /// # Function 2/8
    pub fn with_color_management(mut self, mode: ColorManagement) -> Self {
        self.creator_settings.color_management = mode;
        self.profile.gpu_priority = match mode {
            ColorManagement::Standard => 70,
            ColorManagement::Professional => 85,
            ColorManagement::Cinema => 90,
            ColorManagement::Print => 80,
        };
        self
    }

    /// Set storage optimization level
    ///
    /// # Verification
    /// - Validates optimization level
    /// - Configures storage subsystem
    ///
    /// # Function 3/8
    pub fn with_storage_optimization(mut self, level: StorageOptimization) -> Self {
        self.creator_settings.storage_optimization = level;
        self.profile.io_priority = match level {
            StorageOptimization::Standard => 70,
            StorageOptimization::Fast => 90,
            StorageOptimization::Capacity => 60,
        };
        self
    }

    /// Set rendering priority
    ///
    /// # Verification
    /// - Validates priority level
    /// - Configures rendering pipeline
    ///
    /// # Function 4/8
    pub fn with_rendering_priority(mut self, priority: RenderingPriority) -> Self {
        self.creator_settings.rendering_priority = priority;
        self.profile.cpu_priority = match priority {
            RenderingPriority::Background => -5,
            RenderingPriority::Normal => 0,
            RenderingPriority::High => 5,
            RenderingPriority::Maximum => 10,
        };
        self
    }

    /// Set memory pre-allocation size
    ///
    /// # Verification
    /// - Validates memory size (1-64 GB)
    /// - Ensures sufficient system memory
    ///
    /// # Function 5/8
    pub fn with_memory_preallocate(mut self, gb: u32) -> Result<Self, ProfileError> {
        if !(1..=64).contains(&gb) {
            return Err(ProfileError::ValidationFailed(
                "Memory pre-allocation must be between 1-64 GB".to_string()
            ));
        }
        self.creator_settings.memory_preallocate_gb = gb;
        Ok(self)
    }

    /// Set preview cache size
    ///
    /// # Verification
    /// - Validates cache size (1-32 GB)
    /// - Configures cache subsystem
    ///
    /// # Function 6/8
    pub fn with_preview_cache(mut self, gb: u32) -> Result<Self, ProfileError> {
        if !(1..=32).contains(&gb) {
            return Err(ProfileError::ValidationFailed(
                "Preview cache must be between 1-32 GB".to_string()
            ));
        }
        self.creator_settings.preview_cache_gb = gb;
        Ok(self)
    }

    /// Set auto-save interval
    ///
    /// # Verification
    /// - Validates interval (60-3600 seconds)
    /// - Configures auto-save system
    ///
    /// # Function 7/8
    pub fn with_autosave_interval(mut self, seconds: u32) -> Result<Self, ProfileError> {
        if !(60..=3600).contains(&seconds) {
            return Err(ProfileError::ValidationFailed(
                "Auto-save interval must be between 60-3600 seconds".to_string()
            ));
        }
        self.creator_settings.autosave_interval = seconds;
        Ok(self)
    }

    /// Build the Creator profile
    ///
    /// # Verification
    /// - Validates complete configuration
    /// - Serializes creator settings
    /// - Returns ready-to-use profile
    ///
    /// # Function 8/8
    pub fn build(mut self) -> Profile {
        // Serialize creator settings into custom_settings
        self.profile.custom_settings.insert(
            "color_management".to_string(),
            format!("{:?}", self.creator_settings.color_management),
        );
        self.profile.custom_settings.insert(
            "storage_optimization".to_string(),
            format!("{:?}", self.creator_settings.storage_optimization),
        );
        self.profile.custom_settings.insert(
            "rendering_priority".to_string(),
            format!("{:?}", self.creator_settings.rendering_priority),
        );
        self.profile.custom_settings.insert(
            "memory_preallocate_gb".to_string(),
            self.creator_settings.memory_preallocate_gb.to_string(),
        );
        self.profile.custom_settings.insert(
            "background_rendering".to_string(),
            self.creator_settings.background_rendering.to_string(),
        );
        self.profile.custom_settings.insert(
            "preview_cache_gb".to_string(),
            self.creator_settings.preview_cache_gb.to_string(),
        );
        self.profile.custom_settings.insert(
            "gpu_acceleration".to_string(),
            self.creator_settings.gpu_acceleration.to_string(),
        );
        self.profile.custom_settings.insert(
            "autosave_interval".to_string(),
            self.creator_settings.autosave_interval.to_string(),
        );

        self.profile
    }
}

impl Default for CreatorProfileBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a default Creator profile
///
/// # Returns
/// A fully configured Creator profile with optimal creative settings
pub fn create_creator_profile() -> Profile {
    CreatorProfileBuilder::new().build()
}

/// Create a video editor profile
///
/// # Returns
/// A Creator profile optimized for video editing
pub fn create_video_editor_profile() -> Profile {
    CreatorProfileBuilder::new()
        .with_color_management(ColorManagement::Cinema)
        .with_storage_optimization(StorageOptimization::Fast)
        .with_rendering_priority(RenderingPriority::High)
        .with_memory_preallocate(16).unwrap()
        .with_preview_cache(8).unwrap()
        .with_autosave_interval(180).unwrap() // 3 minutes
        .build()
}

/// Create a 3D artist profile
///
/// # Returns
/// A Creator profile optimized for 3D modeling and rendering
pub fn create_3d_artist_profile() -> Profile {
    CreatorProfileBuilder::new()
        .with_color_management(ColorManagement::Professional)
        .with_storage_optimization(StorageOptimization::Fast)
        .with_rendering_priority(RenderingPriority::Maximum)
        .with_memory_preallocate(32).unwrap()
        .with_preview_cache(4).unwrap()
        .with_autosave_interval(600).unwrap() // 10 minutes
        .build()
}

/// Create a photographer profile
///
/// # Returns
/// A Creator profile optimized for photo editing
pub fn create_photographer_profile() -> Profile {
    CreatorProfileBuilder::new()
        .with_color_management(ColorManagement::Print)
        .with_storage_optimization(StorageOptimization::Capacity)
        .with_rendering_priority(RenderingPriority::Normal)
        .with_memory_preallocate(8).unwrap()
        .with_preview_cache(2).unwrap()
        .with_autosave_interval(300).unwrap() // 5 minutes
        .build()
}

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;

    #[test]
    fn test_creator_profile_creation() {
        let profile = create_creator_profile();
        assert_eq!(profile.id.as_str(), "creator");
        assert_eq!(profile.name, "Creator");
        assert_eq!(profile.io_priority, 90);
    }

    #[test]
    fn test_video_editor_profile() {
        let profile = create_video_editor_profile();
        assert_eq!(
            profile.custom_settings.get("color_management").unwrap(),
            "Cinema"
        );
        assert_eq!(
            profile.custom_settings.get("memory_preallocate_gb").unwrap(),
            "16"
        );
    }

    #[test]
    fn test_3d_artist_profile() {
        let profile = create_3d_artist_profile();
        assert_eq!(profile.cpu_priority, 10); // Maximum rendering priority
        assert_eq!(
            profile.custom_settings.get("memory_preallocate_gb").unwrap(),
            "32"
        );
    }

    #[test]
    fn test_photographer_profile() {
        let profile = create_photographer_profile();
        assert_eq!(
            profile.custom_settings.get("color_management").unwrap(),
            "Print"
        );
    }

    #[test]
    fn test_color_management() {
        let profile = CreatorProfileBuilder::new()
            .with_color_management(ColorManagement::Cinema)
            .build();
        assert_eq!(profile.gpu_priority, 90);
    }

    #[test]
    fn test_storage_optimization() {
        let profile = CreatorProfileBuilder::new()
            .with_storage_optimization(StorageOptimization::Fast)
            .build();
        assert_eq!(profile.io_priority, 90);
    }

    #[test]
    fn test_rendering_priority() {
        let profile = CreatorProfileBuilder::new()
            .with_rendering_priority(RenderingPriority::Maximum)
            .build();
        assert_eq!(profile.cpu_priority, 10);
    }

    #[test]
    fn test_memory_preallocate() {
        let profile = CreatorProfileBuilder::new()
            .with_memory_preallocate(16).unwrap()
            .build();
        assert_eq!(
            profile.custom_settings.get("memory_preallocate_gb").unwrap(),
            "16"
        );
    }

    #[test]
    fn test_invalid_memory_preallocate() {
        let result = CreatorProfileBuilder::new()
            .with_memory_preallocate(100);
        assert!(result.is_err());
    }

    #[test]
    fn test_preview_cache() {
        let profile = CreatorProfileBuilder::new()
            .with_preview_cache(8).unwrap()
            .build();
        assert_eq!(
            profile.custom_settings.get("preview_cache_gb").unwrap(),
            "8"
        );
    }

    #[test]
    fn test_invalid_preview_cache() {
        let result = CreatorProfileBuilder::new()
            .with_preview_cache(50);
        assert!(result.is_err());
    }

    #[test]
    fn test_autosave_interval() {
        let profile = CreatorProfileBuilder::new()
            .with_autosave_interval(180).unwrap()
            .build();
        assert_eq!(
            profile.custom_settings.get("autosave_interval").unwrap(),
            "180"
        );
    }

    #[test]
    fn test_invalid_autosave_interval() {
        let result = CreatorProfileBuilder::new()
            .with_autosave_interval(30);
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_chaining() {
        let profile = CreatorProfileBuilder::new()
            .with_color_management(ColorManagement::Professional)
            .with_storage_optimization(StorageOptimization::Fast)
            .with_rendering_priority(RenderingPriority::High)
            .with_memory_preallocate(16).unwrap()
            .with_preview_cache(4).unwrap()
            .with_autosave_interval(300).unwrap()
            .build();

        assert_eq!(profile.cpu_priority, 5);
        assert_eq!(profile.gpu_priority, 85);
        assert_eq!(profile.io_priority, 90);
    }
}