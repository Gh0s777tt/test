//! Settings Application
//! System settings for VantisOS

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use crate::gui::*;

/// Settings categories
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsCategory {
    Personalization,
    Display,
    Network,
    Sound,
    Power,
    Users,
    Privacy,
    System,
    About,
}

/// Settings application
pub struct Settings {
    pub current_category: SettingsCategory,
    pub theme: String,
    pub wallpaper: u32,
    pub brightness: u32,
    pub volume: u32,
    pub wifi_enabled: bool,
    pub bluetooth_enabled: bool,
    pub notifications: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            current_category: SettingsCategory::Personalization,
            theme: String::from("Dark"),
            wallpaper: 0,
            brightness: 80,
            volume: 50,
            wifi_enabled: true,
            bluetooth_enabled: false,
            notifications: true,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        // Sidebar
        surface.fill_rect(Rect::new(0, 0, 220, height), Color::new(35, 35, 35));
        
        let categories = [
            ("Personalization", SettingsCategory::Personalization),
            ("Display", SettingsCategory::Display),
            ("Network", SettingsCategory::Network),
            ("Sound", SettingsCategory::Sound),
            ("Power", SettingsCategory::Power),
            ("Users", SettingsCategory::Users),
            ("Privacy", SettingsCategory::Privacy),
            ("System", SettingsCategory::System),
            ("About", SettingsCategory::About),
        ];
        
        // Title
        surface.draw_text_sized(20, 20, "Settings", 18, Color::WHITE);
        
        for (i, (name, cat)) in categories.iter().enumerate() {
            let y = 60 + (i as u32) * 40;
            let is_selected = *cat == self.current_category;
            
            if is_selected {
                surface.fill_rect(Rect::new(0, y as i32 - 8, 220, 36), Color::ACCENT);
            }
            
            surface.draw_text_sized(20, y, name, 13, Color::WHITE);
        }
        
        // Content area
        let content_x = 240;
        let content_y = 40;
        
        match self.current_category {
            SettingsCategory::Personalization => self.render_personalization(surface, content_x, content_y),
            SettingsCategory::Display => self.render_display(surface, content_x, content_y, width),
            SettingsCategory::Network => self.render_network(surface, content_x, content_y),
            SettingsCategory::Sound => self.render_sound(surface, content_x, content_y, width),
            SettingsCategory::Power => self.render_power(surface, content_x, content_y),
            SettingsCategory::Users => self.render_users(surface, content_x, content_y),
            SettingsCategory::Privacy => self.render_privacy(surface, content_x, content_y),
            SettingsCategory::System => self.render_system(surface, content_x, content_y),
            SettingsCategory::About => self.render_about(surface, content_x, content_y),
        }
    }
    
    fn render_personalization(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "Personalization", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "Theme:", 14, Color::LIGHT_GRAY);
        
        let themes = ["Dark", "Light", "High Contrast"];
        for (i, theme) in themes.iter().enumerate() {
            let is_selected = *theme == self.theme;
            let bg = if is_selected { Color::ACCENT } else { Color::new(60, 60, 60) };
            surface.fill_rounded_rect(Rect::new(x as i32 + (i as i32) * 120, y as i32 + 80, 100, 80), 8, bg);
            surface.draw_text_sized(x + 20 + (i as u32) * 120, y + 125, theme, 12, Color::WHITE);
        }
        
        surface.draw_text_sized(x, y + 190, "Wallpaper:", 14, Color::LIGHT_GRAY);
        
        let wallpapers = ["Gradient", "Abstract", "Nature", "Minimal"];
        for (i, wp) in wallpapers.iter().enumerate() {
            let is_selected = i as u32 == self.wallpaper;
            let border = if is_selected { Color::ACCENT } else { Color::GRAY };
            surface.fill_rect(Rect::new(x as i32 + (i as i32) * 110, y as i32 + 220, 90, 60), Color::new(60, 60, 60));
            surface.draw_rect(Rect::new(x as i32 + (i as i32) * 110, y as i32 + 220, 90, 60), border, 2);
            surface.draw_text_sized(x + 20 + (i as u32) * 110, y + 285, wp, 11, Color::LIGHT_GRAY);
        }
    }
    
    fn render_display(&self, surface: &mut dyn Surface, x: u32, y: u32, width: u32) {
        surface.draw_text_sized(x, y, "Display", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "Brightness:", 14, Color::LIGHT_GRAY);
        
        // Brightness slider
        surface.fill_rect(Rect::new(x as i32, y as i32 + 80, width - x - 50, 8), Color::new(60, 60, 60));
        let fill_width = (width - x - 50) * self.brightness / 100;
        surface.fill_rect(Rect::new(x as i32, y as i32 + 80, fill_width, 8), Color::ACCENT);
        surface.draw_text_sized(x + 310, y + 55, &format!("{}%", self.brightness), 12, Color::WHITE);
        
        surface.draw_text_sized(x, y + 120, "Resolution:", 14, Color::LIGHT_GRAY);
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 150, 200, 35), 4, Color::new(50, 50, 50));
        surface.draw_text_sized(x + 10, y + 158, "1920 x 1080", 12, Color::WHITE);
        
        surface.draw_text_sized(x, y + 210, "Scaling:", 14, Color::LIGHT_GRAY);
        let scales = ["100%", "125%", "150%"];
        for (i, scale) in scales.iter().enumerate() {
            let is_selected = *scale == "100%";
            let bg = if is_selected { Color::ACCENT } else { Color::new(50, 50, 50) };
            surface.fill_rounded_rect(Rect::new(x as i32 + (i as i32) * 80, y as i32 + 240, 70, 35), 4, bg);
            surface.draw_text_sized(x + 15 + (i as u32) * 80, y + 248, scale, 11, Color::WHITE);
        }
    }
    
    fn render_network(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "Network", 20, Color::WHITE);
        
        // WiFi toggle
        let wifi_bg = if self.wifi_enabled { Color::ACCENT } else { Color::new(60, 60, 60) };
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 50, 50, 26), 13, wifi_bg);
        let knob_x = if self.wifi_enabled { x + 24 } else { x + 4 };
        surface.fill_rounded_rect(Rect::new(knob_x as i32, y as i32 + 53, 20, 20), 10, Color::WHITE);
        surface.draw_text_sized(x + 60, y + 53, "WiFi", 14, Color::WHITE);
        
        if self.wifi_enabled {
            surface.draw_text_sized(x, y + 100, "Available Networks:", 13, Color::LIGHT_GRAY);
            
            let networks = ["VantisNet", "Guest-WiFi", "Neighbor_5G"];
            for (i, network) in networks.iter().enumerate() {
                surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 130 + (i as i32) * 45, 300, 40), 4, Color::new(50, 50, 50));
                surface.draw_text_sized(x + 10, y + 140 + (i as u32) * 45, network, 12, Color::WHITE);
                surface.draw_icon(x + 270, y + 140 + (i as u32) * 45, Icon::Wifi, 16);
            }
        }
    }
    
    fn render_sound(&self, surface: &mut dyn Surface, x: u32, y: u32, width: u32) {
        surface.draw_text_sized(x, y, "Sound", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "Volume:", 14, Color::LIGHT_GRAY);
        
        // Volume slider
        surface.fill_rect(Rect::new(x as i32, y as i32 + 80, width - x - 50, 8), Color::new(60, 60, 60));
        let fill_width = (width - x - 50) * self.volume / 100;
        surface.fill_rect(Rect::new(x as i32, y as i32 + 80, fill_width, 8), Color::ACCENT);
        surface.draw_text_sized(x + 310, y + 55, &format!("{}%", self.volume), 12, Color::WHITE);
        
        surface.draw_text_sized(x, y + 120, "Output Device:", 14, Color::LIGHT_GRAY);
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 150, 300, 35), 4, Color::new(50, 50, 50));
        surface.draw_text_sized(x + 10, y + 158, "Speakers (Built-in)", 12, Color::WHITE);
        
        surface.draw_text_sized(x, y + 210, "Input Device:", 14, Color::LIGHT_GRAY);
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 240, 300, 35), 4, Color::new(50, 50, 50));
        surface.draw_text_sized(x + 10, y + 248, "Microphone (Built-in)", 12, Color::WHITE);
    }
    
    fn render_power(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "Power", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "Power Mode:", 14, Color::LIGHT_GRAY);
        
        let modes = ["Balanced", "Performance", "Power Saver"];
        for (i, mode) in modes.iter().enumerate() {
            let is_selected = *mode == "Balanced";
            let bg = if is_selected { Color::ACCENT } else { Color::new(50, 50, 50) };
            surface.fill_rounded_rect(Rect::new(x as i32 + (i as i32) * 120, y as i32 + 80, 100, 60), 8, bg);
            surface.draw_text_sized(x + 10 + (i as u32) * 120, y + 105, mode, 11, Color::WHITE);
        }
        
        surface.draw_text_sized(x, y + 170, "Battery:", 14, Color::LIGHT_GRAY);
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 200, 200, 35), 4, Color::new(50, 50, 50));
        surface.draw_text_sized(x + 10, y + 208, "100% (Charging)", 12, Color::WHITE);
    }
    
    fn render_users(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "Users", 20, Color::WHITE);
        
        // Current user
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 50, 300, 80), 8, Color::new(50, 50, 50));
        surface.draw_icon(x + 15, y + 65, Icon::User, 40);
        surface.draw_text_sized(x + 70, y + 65, "user", 16, Color::WHITE);
        surface.draw_text_sized(x + 70, y + 90, "Administrator", 12, Color::LIGHT_GRAY);
        
        // Add user button
        surface.fill_rounded_rect(Rect::new(x as i32, y as i32 + 150, 150, 40), 4, Color::ACCENT);
        surface.draw_text_sized(x + 30, y + 160, "Add User", 13, Color::WHITE);
    }
    
    fn render_privacy(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "Privacy", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "Notifications:", 14, Color::LIGHT_GRAY);
        let notif_bg = if self.notifications { Color::ACCENT } else { Color::new(60, 60, 60) };
        surface.fill_rounded_rect(Rect::new(x as i32 + 150, y as i32 + 47, 50, 26), 13, notif_bg);
        let knob_x = if self.notifications { x + 174 } else { x + 154 };
        surface.fill_rounded_rect(Rect::new(knob_x as i32, y as i32 + 50, 20, 20), 10, Color::WHITE);
        
        surface.draw_text_sized(x, y + 100, "Location Services:", 14, Color::LIGHT_GRAY);
        surface.fill_rounded_rect(Rect::new(x as i32 + 180, y as i32 + 97, 50, 26), 13, Color::new(60, 60, 60));
        surface.fill_rounded_rect(Rect::new((x + 154) as i32, y as i32 + 100, 20, 20), 10, Color::WHITE);
    }
    
    fn render_system(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "System", 20, Color::WHITE);
        
        let info = [
            ("OS Version:", "VantisOS 1.5.0"),
            ("Kernel:", "Vantis 1.5.0"),
            ("Architecture:", "x86_64"),
            ("Memory:", "16 GB"),
            ("Processor:", "Virtual CPU"),
            ("Uptime:", "0 hours"),
        ];
        
        for (i, (label, value)) in info.iter().enumerate() {
            surface.draw_text_sized(x, y + 50 + (i as u32) * 35, label, 13, Color::LIGHT_GRAY);
            surface.draw_text_sized(x + 130, y + 50 + (i as u32) * 35, value, 13, Color::WHITE);
        }
    }
    
    fn render_about(&self, surface: &mut dyn Surface, x: u32, y: u32) {
        surface.draw_text_sized(x, y, "About VantisOS", 20, Color::WHITE);
        
        surface.draw_text_sized(x, y + 50, "VantisOS", 28, Color::WHITE);
        surface.draw_text_sized(x, y + 85, "Version 1.5.0 &quot;Quantum Ready&quot;", 14, Color::LIGHT_GRAY);
        
        surface.draw_text_sized(x, y + 130, "A modern, quantum-ready operating system.", 13, Color::LIGHT_GRAY);
        surface.draw_text_sized(x, y + 155, "Built with Rust for security and performance.", 13, Color::LIGHT_GRAY);
        
        surface.draw_text_sized(x, y + 200, "© 2025 VantisOS Team", 12, Color::GRAY);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}