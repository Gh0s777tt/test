//! Web Browser Application
use alloc::string::String;
use crate::gui::*;

pub struct Browser {
    url: String,
    content: String,
    loading: bool,
}

impl Browser {
    pub fn new() -> Self {
        Self {
            url: String::from("https://vantisos.org"),
            content: String::from("Welcome to VantisOS Web Browser"),
            loading: false,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        
        surface.fill_rect(Rect::new(0, 0, width, surface.height()), Color::WHITE);
        
        // Toolbar
        surface.fill_rect(Rect::new(0, 0, width, 50), Color::new(50, 50, 50));
        
        // Navigation buttons
        surface.fill_rounded_rect(Rect::new(12, 10, 30, 30), 4, Color::new(70, 70, 70));
        surface.fill_rounded_rect(Rect::new(50, 10, 30, 30), 4, Color::new(70, 70, 70));
        
        // URL bar
        surface.fill_rect(Rect::new(90, 10, width - 110, 30), Color::new(35, 35, 35));
        surface.draw_text_sized(100, 17, &self.url, 12, Color::WHITE);
        
        // Content
        surface.draw_text_sized(20, 70, &self.content, 14, Color::BLACK);
    }
}

impl Default for Browser {
    fn default() -> Self { Self::new() }
}