//! Media Player Application
use alloc::string::String;
use crate::gui::*;

pub struct MediaPlayer {
    filename: String,
    playing: bool,
    progress: u32,
    duration: u32,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            filename: String::from("No file selected"),
            playing: false,
            progress: 0,
            duration: 180,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(20, 20, 20));
        
        // Video area
        surface.fill_rect(Rect::new(20, 20, width - 40, height - 140), Color::BLACK);
        surface.draw_text_sized(width / 2 - 50, height / 2 - 40, "No Video", 18, Color::GRAY);
        
        // Controls
        surface.fill_rect(Rect::new(0, (height - 100) as i32, width, 100), Color::new(30, 30, 30));
        
        // Progress bar
        surface.fill_rect(Rect::new(20, (height - 90) as i32, width - 40, 8), Color::new(50, 50, 50));
        if self.duration > 0 {
            let prog_width = (width - 40) * self.progress / self.duration;
            surface.fill_rect(Rect::new(20, (height - 90) as i32, prog_width, 8), Color::ACCENT);
        }
        
        // Play/Pause button
        let play_icon = if self.playing { Icon::Pause } else { Icon::Play };
        surface.draw_icon(width / 2 - 16, height - 60, play_icon, 32);
        
        // File name
        surface.draw_text_sized(20, height - 50, &self.filename, 12, Color::LIGHT_GRAY);
    }
}

impl Default for MediaPlayer {
    fn default() -> Self { Self::new() }
}