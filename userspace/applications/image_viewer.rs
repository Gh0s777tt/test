//! # Image Viewer Application
//!
//! A comprehensive image viewer application for VantisOS with support for
//! various image formats, zoom, rotation, slideshow, and basic editing.

use serde::{Deserialize, Serialize};

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    GIF,
    BMP,
    WEBP,
    SVG,
    TIFF,
}

impl ImageFormat {
    /// Get file extension for format
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::PNG => "png",
            ImageFormat::JPEG => "jpg",
            ImageFormat::GIF => "gif",
            ImageFormat::BMP => "bmp",
            ImageFormat::WEBP => "webp",
            ImageFormat::SVG => "svg",
            ImageFormat::TIFF => "tiff",
        }
    }

    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(ImageFormat::PNG),
            "jpg" | "jpeg" => Some(ImageFormat::JPEG),
            "gif" => Some(ImageFormat::GIF),
            "bmp" => Some(ImageFormat::BMP),
            "webp" => Some(ImageFormat::WEBP),
            "svg" => Some(ImageFormat::SVG),
            "tiff" | "tif" => Some(ImageFormat::TIFF),
            _ => None,
        }
    }
}

/// Image metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub size_bytes: u64,
    pub color_depth: u8,
    pub has_alpha: bool,
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    FitToScreen,
    ActualSize,
    FillScreen,
}

/// Slideshow settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlideshowSettings {
    pub enabled: bool,
    pub interval_seconds: u32,
    pub loop_slideshow: bool,
    pub shuffle: bool,
}

impl Default for SlideshowSettings {
    fn default() -> Self {
        SlideshowSettings {
            enabled: false,
            interval_seconds: 5,
            loop_slideshow: true,
            shuffle: false,
        }
    }
}

/// Image viewer state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageViewer {
    pub current_image: Option<ImageInfo>,
    pub image_list: Vec<ImageInfo>,
    pub current_index: usize,
    pub zoom: f32,
    pub rotation: f32,
    pub view_mode: ViewMode,
    pub slideshow_settings: SlideshowSettings,
    pub fullscreen: bool,
    pub show_info: bool,
}

/// Image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub path: String,
    pub name: String,
    pub metadata: Option<ImageMetadata>,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageViewer {
    /// Create a new image viewer instance
    pub fn new() -> Self {
        ImageViewer {
            current_image: None,
            image_list: Vec::new(),
            current_index: 0,
            zoom: 1.0,
            rotation: 0.0,
            view_mode: ViewMode::FitToScreen,
            slideshow_settings: SlideshowSettings::default(),
            fullscreen: false,
            show_info: true,
        }
    }

    /// Load image from path
    pub fn load_image(&mut self, path: String) -> Result<(), String> {
        let name = path.split('/').last().unwrap_or("Unknown").to_string();
        
        // In a real implementation, this would load the actual image
        let metadata = Self::load_metadata(&path)?;
        
        let image_info = ImageInfo {
            path: path.clone(),
            name,
            metadata: Some(metadata),
        };
        
        self.current_image = Some(image_info.clone());
        
        // Update image list
        if let Some(index) = self.image_list.iter().position(|i| i.path == path) {
            self.current_index = index;
        } else {
            self.image_list.push(image_info);
            self.current_index = self.image_list.len() - 1;
        }
        
        self.reset_view();
        Ok(())
    }

    /// Load multiple images
    pub fn load_images(&mut self, paths: Vec<String>) {
        for path in paths {
            if let Ok(image_info) = Self::create_image_info(&path) {
                self.image_list.push(image_info);
            }
        }
        
        if !self.image_list.is_empty() && self.current_image.is_none() {
            self.current_index = 0;
            self.load_image(self.image_list[0].path.clone()).ok();
        }
    }

    /// Navigate to next image
    pub fn next_image(&mut self) -> bool {
        if self.image_list.is_empty() {
            return false;
        }
        
        self.current_index = (self.current_index + 1) % self.image_list.len();
        if let Some(image) = self.image_list.get(self.current_index) {
            self.current_image = Some(image.clone());
            self.reset_view();
            true
        } else {
            false
        }
    }

    /// Navigate to previous image
    pub fn previous_image(&mut self) -> bool {
        if self.image_list.is_empty() {
            return false;
        }
        
        self.current_index = if self.current_index == 0 {
            self.image_list.len() - 1
        } else {
            self.current_index - 1
        };
        
        if let Some(image) = self.image_list.get(self.current_index) {
            self.current_image = Some(image.clone());
            self.reset_view();
            true
        } else {
            false
        }
    }

    /// Go to specific image
    pub fn goto_image(&mut self, index: usize) -> bool {
        if index < self.image_list.len() {
            self.current_index = index;
            if let Some(image) = self.image_list.get(index) {
                self.current_image = Some(image.clone());
                self.reset_view();
                return true;
            }
        }
        false
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).min(50.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).max(0.02);
    }

    /// Reset zoom
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    /// Rotate image
    pub fn rotate(&mut self, degrees: f32) {
        self.rotation = (self.rotation + degrees) % 360.0;
    }

    /// Reset rotation
    pub fn reset_rotation(&mut self) {
        self.rotation = 0.0;
    }

    /// Set view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
        self.reset_zoom();
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    /// Toggle info display
    pub fn toggle_info(&mut self) {
        self.show_info = !self.show_info;
    }

    /// Start slideshow
    pub fn start_slideshow(&mut self, interval_seconds: u32) {
        self.slideshow_settings.enabled = true;
        self.slideshow_settings.interval_seconds = interval_seconds;
    }

    /// Stop slideshow
    pub fn stop_slideshow(&mut self) {
        self.slideshow_settings.enabled = false;
    }

    /// Toggle slideshow
    pub fn toggle_slideshow(&mut self) {
        self.slideshow_settings.enabled = !self.slideshow_settings.enabled;
    }

    /// Get current zoom level as percentage
    pub fn zoom_percentage(&self) -> u32 {
        (self.zoom * 100.0) as u32
    }

    /// Get current rotation degrees
    pub fn rotation_degrees(&self) -> f32 {
        self.rotation
    }

    /// Check if next image exists
    pub fn has_next(&self) -> bool {
        !self.image_list.is_empty() && self.current_index < self.image_list.len() - 1
    }

    /// Check if previous image exists
    pub fn has_previous(&self) -> bool {
        !self.image_list.is_empty() && self.current_index > 0
    }

    /// Get image count
    pub fn image_count(&self) -> usize {
        self.image_list.len()
    }

    /// Get current position (1-based)
    pub fn current_position(&self) -> usize {
        self.current_index + 1
    }

    /// Reset view to defaults
    fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.rotation = 0.0;
    }

    /// Create image info from path
    fn create_image_info(path: &str) -> Result<ImageInfo, String> {
        let name = path.split('/').last().unwrap_or("Unknown").to_string();
        let metadata = Self::load_metadata(path)?;
        
        Ok(ImageInfo {
            path: path.to_string(),
            name,
            metadata: Some(metadata),
        })
    }

    /// Load image metadata (simplified)
    fn load_metadata(path: &str) -> Result<ImageMetadata, String> {
        let ext = path.split('.').last().unwrap_or("");
        let format = ImageFormat::from_extension(ext)
            .ok_or_else(|| format!("Unsupported format: {}", ext))?;
        
        // In a real implementation, this would read actual image metadata
        Ok(ImageMetadata {
            width: 1920,
            height: 1080,
            format,
            size_bytes: 1024 * 1024, // 1MB default
            color_depth: 24,
            has_alpha: format == ImageFormat::PNG,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_viewer_creation() {
        let viewer = ImageViewer::new();
        assert!(viewer.current_image.is_none());
        assert_eq!(viewer.zoom, 1.0);
    }

    #[test]
    fn test_zoom_in() {
        let mut viewer = ImageViewer::new();
        viewer.zoom_in();
        assert!(viewer.zoom > 1.0);
    }

    #[test]
    fn test_zoom_out() {
        let mut viewer = ImageViewer::new();
        viewer.zoom_in();
        viewer.zoom_out();
        assert!((viewer.zoom - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_rotate() {
        let mut viewer = ImageViewer::new();
        viewer.rotate(90.0);
        assert_eq!(viewer.rotation, 90.0);
    }

    #[test]
    fn test_image_format_from_extension() {
        assert_eq!(ImageFormat::from_extension("png"), Some(ImageFormat::PNG));
        assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::JPEG));
        assert_eq!(ImageFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_slideshow_settings_default() {
        let settings = SlideshowSettings::default();
        assert!(!settings.enabled);
        assert_eq!(settings.interval_seconds, 5);
        assert!(settings.loop_slideshow);
        assert!(!settings.shuffle);
    }
}