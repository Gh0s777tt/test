//! Image Viewer Application
use alloc::string::String;
use crate::gui::*;

pub struct ImageViewer {
    filename: String,
    zoom: u32,
}

impl ImageViewer {
    pub fn new() -> Self {
        Self {
            filename: String::from("No image loaded"),
            zoom: 100,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(30, 30, 30));
        
        // Image area
        surface.fill_rect(Rect::new(20, 60, width - 40, height - 120), Color::BLACK);
        surface.draw_text_sized(width / 2 - 50, height / 2 - 30, "No Image", 16, Color::GRAY);
        
        // Toolbar
        surface.fill_rect(Rect::new(0, 0, width, 50), Color::new(40, 40, 40));
        surface.draw_text_sized(20, 17, &self.filename, 13, Color::WHITE);
        surface.draw_text_sized(width - 80, 17, &alloc::format!("{}%", self.zoom), 12, Color::LIGHT_GRAY);
    }
}

impl Default for ImageViewer {
    fn default() -> Self { Self::new() }
}