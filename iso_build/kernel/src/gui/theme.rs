//! Theme System
//! Comprehensive theming for VantisOS GUI

use alloc::vec;
use super::*;
use alloc::string::String;
use alloc::vec::Vec;

/// Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Background color
    pub background: Color,
    /// Foreground/text color
    pub foreground: Color,
    /// Accent color
    pub accent: Color,
    /// Window background
    pub window_bg: Color,
    /// Window border
    pub window_border: Color,
    /// Title bar background
    pub title_bar: Color,
    /// Title bar active
    pub title_bar_active: Color,
    /// Title text color
    pub title_text: Color,
    /// Button background
    pub button: Color,
    /// Button hover
    pub button_hover: Color,
    /// Button pressed
    pub button_pressed: Color,
    /// Button text
    pub button_text: Color,
    /// Input background
    pub input_bg: Color,
    /// Input border
    pub input_border: Color,
    /// Input focus border
    pub input_focus: Color,
    /// Selection background
    pub selection: Color,
    /// Menu background
    pub menu_bg: Color,
    /// Menu item hover
    pub menu_hover: Color,
    /// Scrollbar track
    pub scroll_track: Color,
    /// Scrollbar thumb
    pub scroll_thumb: Color,
    /// Desktop background
    pub desktop: Color,
    /// Taskbar background
    pub taskbar: Color,
    /// Panel background
    pub panel: Color,
}

impl Theme {
    /// Create dark theme (default)
    pub fn dark() -> Self {
        Self {
            name: String::from("Dark"),
            background: Color::new(32, 32, 32),
            foreground: Color::WHITE,
            accent: Color::ACCENT,
            window_bg: Color::new(45, 45, 45),
            window_border: Color::new(60, 60, 60),
            title_bar: Color::new(32, 32, 32),
            title_bar_active: Color::ACCENT,
            title_text: Color::WHITE,
            button: Color::new(60, 60, 60),
            button_hover: Color::new(80, 80, 80),
            button_pressed: Color::new(50, 50, 50),
            button_text: Color::WHITE,
            input_bg: Color::new(40, 40, 40),
            input_border: Color::new(80, 80, 80),
            input_focus: Color::ACCENT,
            selection: Color::SELECTED,
            menu_bg: Color::new(40, 40, 40),
            menu_hover: Color::new(60, 60, 60),
            scroll_track: Color::new(40, 40, 40),
            scroll_thumb: Color::new(80, 80, 80),
            desktop: Color::DESKTOP_BG,
            taskbar: Color::new(24, 24, 24),
            panel: Color::new(40, 40, 40),
        }
    }
    
    /// Create light theme
    pub fn light() -> Self {
        Self {
            name: String::from("Light"),
            background: Color::new(240, 240, 240),
            foreground: Color::BLACK,
            accent: Color::ACCENT,
            window_bg: Color::WHITE,
            window_border: Color::new(200, 200, 200),
            title_bar: Color::new(240, 240, 240),
            title_bar_active: Color::ACCENT,
            title_text: Color::BLACK,
            button: Color::new(225, 225, 225),
            button_hover: Color::new(229, 243, 255),
            button_pressed: Color::new(204, 228, 247),
            button_text: Color::BLACK,
            input_bg: Color::WHITE,
            input_border: Color::new(180, 180, 180),
            input_focus: Color::ACCENT,
            selection: Color::SELECTED,
            menu_bg: Color::WHITE,
            menu_hover: Color::new(240, 240, 240),
            scroll_track: Color::new(230, 230, 230),
            scroll_thumb: Color::new(180, 180, 180),
            desktop: Color::new(0, 120, 212),
            taskbar: Color::new(243, 243, 243),
            panel: Color::WHITE,
        }
    }
    
    /// Create high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            name: String::from("High Contrast"),
            background: Color::BLACK,
            foreground: Color::WHITE,
            accent: Color::new(0, 255, 255),
            window_bg: Color::BLACK,
            window_border: Color::WHITE,
            title_bar: Color::BLACK,
            title_bar_active: Color::new(0, 0, 128),
            title_text: Color::WHITE,
            button: Color::BLACK,
            button_hover: Color::new(0, 0, 128),
            button_pressed: Color::new(0, 0, 255),
            button_text: Color::WHITE,
            input_bg: Color::BLACK,
            input_border: Color::WHITE,
            input_focus: Color::new(0, 255, 255),
            selection: Color::new(0, 0, 128),
            menu_bg: Color::BLACK,
            menu_hover: Color::new(0, 0, 128),
            scroll_track: Color::BLACK,
            scroll_thumb: Color::WHITE,
            desktop: Color::new(0, 0, 64),
            taskbar: Color::BLACK,
            panel: Color::BLACK,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Theme manager
pub struct ThemeManager {
    themes: Vec<Theme>,
    current: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            themes: vec![
                Theme::dark(),
                Theme::light(),
                Theme::high_contrast(),
            ],
            current: 0,
        }
    }
    
    pub fn current(&self) -> &Theme {
        &self.themes[self.current]
    }
    
    pub fn current_mut(&mut self) -> &mut Theme {
        &mut self.themes[self.current]
    }
    
    pub fn set_theme(&mut self, index: usize) -> bool {
        if index < self.themes.len() {
            self.current = index;
            true
        } else {
            false
        }
    }
    
    pub fn available_themes(&self) -> &[Theme] {
        &self.themes
    }
    
    pub fn theme_names(&self) -> Vec<&str> {
        self.themes.iter().map(|t| t.name.as_str()).collect()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}