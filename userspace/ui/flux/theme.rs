//! # Flux Theming System
//! 
//! Comprehensive theming system supporting color schemes, component styling, and dynamic theme switching.
//! Provides a consistent visual appearance across all Flux UI components.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Color with RGBA components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ThemeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ThemeColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Self::new(r, g, b, a))
        } else {
            None
        }
    }

    pub fn with_alpha(&self, alpha: u8) -> Self {
        Self::new(self.r, self.g, self.b, alpha)
    }

    pub fn blend(&self, other: &ThemeColor, factor: f32) -> Self {
        let f = factor.clamp(0.0, 1.0);
        let r = (self.r as f32 * (1.0 - f) + other.r as f32 * f) as u8;
        let g = (self.g as f32 * (1.0 - f) + other.g as f32 * f) as u8;
        let b = (self.b as f32 * (1.0 - f) + other.b as f32 * f) as u8;
        let a = (self.a as f32 * (1.0 - f) + other.a as f32 * f) as u8;
        Self::new(r, g, b, a)
    }

    pub fn lighten(&self, factor: f32) -> Self {
        self.blend(&ThemeColor::rgb(255, 255, 255), factor)
    }

    pub fn darken(&self, factor: f32) -> Self {
        self.blend(&ThemeColor::rgb(0, 0, 0), factor)
    }
}

/// Gradient type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradientType {
    Linear,
    Radial,
}

/// Gradient definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub gradient_type: GradientType,
    pub colors: Vec<ThemeColor>,
    pub stops: Vec<f32>,
    pub angle: f32,
}

impl Gradient {
    pub fn linear(colors: Vec<ThemeColor>) -> Self {
        let stops: Vec<f32> = (0..colors.len())
            .map(|i| i as f32 / (colors.len() - 1).max(1) as f32)
            .collect();
        
        Self {
            gradient_type: GradientType::Linear,
            colors,
            stops,
            angle: 0.0,
        }
    }

    pub fn radial(colors: Vec<ThemeColor>) -> Self {
        let stops: Vec<f32> = (0..colors.len())
            .map(|i| i as f32 / (colors.len() - 1).max(1) as f32)
            .collect();
        
        Self {
            gradient_type: GradientType::Radial,
            colors,
            stops,
            angle: 0.0,
        }
    }
}

/// Border style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

/// Border definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    pub width: u32,
    pub style: BorderStyle,
    pub color: ThemeColor,
    pub radius: u32,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: 0,
            style: BorderStyle::None,
            color: ThemeColor::rgb(0, 0, 0),
            radius: 0,
        }
    }
}

impl Border {
    pub fn solid(width: u32, color: ThemeColor) -> Self {
        Self {
            width,
            style: BorderStyle::Solid,
            color,
            radius: 0,
        }
    }

    pub fn rounded(width: u32, color: ThemeColor, radius: u32) -> Self {
        Self {
            width,
            style: BorderStyle::Solid,
            color,
            radius,
        }
    }
}

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font weight
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn to_number(&self) -> u32 {
        match self {
            Self::Thin => 100,
            Self::ExtraLight => 200,
            Self::Light => 300,
            Self::Regular => 400,
            Self::Medium => 500,
            Self::SemiBold => 600,
            Self::Bold => 700,
            Self::ExtraBold => 800,
            Self::Black => 900,
        }
    }
}

/// Font definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub family: String,
    pub size: u32,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub color: ThemeColor,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            family: "sans-serif".to_string(),
            size: 14,
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            color: ThemeColor::rgb(0, 0, 0),
        }
    }
}

impl Font {
    pub fn new(family: &str, size: u32) -> Self {
        Self {
            family: family.to_string(),
            size,
            ..Default::default()
        }
    }

    pub fn bold(mut self) -> Self {
        self.weight = FontWeight::Bold;
        self
    }

    pub fn italic(mut self) -> Self {
        self.style = FontStyle::Italic;
        self
    }

    pub fn with_color(mut self, color: ThemeColor) -> Self {
        self.color = color;
        self
    }
}

/// Shadow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    pub offset_x: i32,
    pub offset_y: i32,
    pub blur_radius: u32,
    pub spread_radius: u32,
    pub color: ThemeColor,
}

impl Shadow {
    pub fn new(offset_x: i32, offset_y: i32, blur: u32, color: ThemeColor) -> Self {
        Self {
            offset_x,
            offset_y,
            blur_radius: blur,
            spread_radius: 0,
            color,
        }
    }

    pub fn with_spread(mut self, spread: u32) -> Self {
        self.spread_radius = spread;
        self
    }
}

/// Component style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    pub background: Option<ThemeColor>,
    pub background_gradient: Option<Gradient>,
    pub foreground: Option<ThemeColor>,
    pub border: Option<Border>,
    pub font: Option<Font>,
    pub shadow: Option<Shadow>,
    pub opacity: f32,
}

impl Default for ComponentStyle {
    fn default() -> Self {
        Self {
            background: None,
            background_gradient: None,
            foreground: None,
            border: None,
            font: None,
            shadow: None,
            opacity: 1.0,
        }
    }
}

impl ComponentStyle {
    pub fn background(mut self, color: ThemeColor) -> Self {
        self.background = Some(color);
        self
    }

    pub fn foreground(mut self, color: ThemeColor) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

/// Color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: ThemeColor,
    pub secondary: ThemeColor,
    pub accent: ThemeColor,
    pub success: ThemeColor,
    pub warning: ThemeColor,
    pub error: ThemeColor,
    pub info: ThemeColor,
    pub background: ThemeColor,
    pub surface: ThemeColor,
    pub on_primary: ThemeColor,
    pub on_secondary: ThemeColor,
    pub on_accent: ThemeColor,
    pub on_background: ThemeColor,
    pub on_surface: ThemeColor,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: ThemeColor::rgb(0, 122, 255),
            secondary: ThemeColor::rgb(88, 86, 214),
            accent: ThemeColor::rgb(255, 45, 85),
            success: ThemeColor::rgb(52, 199, 89),
            warning: ThemeColor::rgb(255, 204, 0),
            error: ThemeColor::rgb(255, 59, 48),
            info: ThemeColor::rgb(0, 199, 190),
            background: ThemeColor::rgb(242, 242, 247),
            surface: ThemeColor::rgb(255, 255, 255),
            on_primary: ThemeColor::rgb(255, 255, 255),
            on_secondary: ThemeColor::rgb(255, 255, 255),
            on_accent: ThemeColor::rgb(255, 255, 255),
            on_background: ThemeColor::rgb(0, 0, 0),
            on_surface: ThemeColor::rgb(0, 0, 0),
        }
    }
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            primary: ThemeColor::rgb(10, 132, 255),
            secondary: ThemeColor::rgb(94, 92, 230),
            accent: ThemeColor::rgb(255, 55, 95),
            success: ThemeColor::rgb(48, 209, 88),
            warning: ThemeColor::rgb(255, 214, 10),
            error: ThemeColor::rgb(255, 69, 58),
            info: ThemeColor::rgb(100, 210, 255),
            background: ThemeColor::rgb(28, 28, 30),
            surface: ThemeColor::rgb(44, 44, 46),
            on_primary: ThemeColor::rgb(255, 255, 255),
            on_secondary: ThemeColor::rgb(255, 255, 255),
            on_accent: ThemeColor::rgb(255, 255, 255),
            on_background: ThemeColor::rgb(255, 255, 255),
            on_surface: ThemeColor::rgb(255, 255, 255),
        }
    }
}

/// Complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub version: String,
    pub palette: ColorPalette,
    pub styles: HashMap<String, ComponentStyle>,
}

impl Theme {
    pub fn new(name: &str, palette: ColorPalette) -> Self {
        Self {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            palette,
            styles: HashMap::new(),
        }
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn add_style(mut self, component: &str, style: ComponentStyle) -> Self {
        self.styles.insert(component.to_string(), style);
        self
    }

    pub fn get_style(&self, component: &str) -> Option<&ComponentStyle> {
        self.styles.get(component)
    }

    pub fn light() -> Self {
        let palette = ColorPalette::default();
        let mut theme = Self::new("Light", palette);

        // Add default component styles
        theme.styles.insert(
            "button".to_string(),
            ComponentStyle::default()
                .background(palette.primary)
                .foreground(palette.on_primary)
                .border(Border::rounded(1, palette.primary.darken(0.1), 8))
                .shadow(Shadow::new(0, 2, 4, ThemeColor::rgb(0, 0, 0).with_alpha(51)))
        );

        theme.styles.insert(
            "input".to_string(),
            ComponentStyle::default()
                .background(palette.surface)
                .foreground(palette.on_surface)
                .border(Border::solid(1, ThemeColor::rgb(209, 209, 214)))
        );

        theme.styles.insert(
            "card".to_string(),
            ComponentStyle::default()
                .background(palette.surface)
                .foreground(palette.on_surface)
                .border(Border::rounded(1, ThemeColor::rgb(229, 229, 234), 12))
                .shadow(Shadow::new(0, 1, 3, ThemeColor::rgb(0, 0, 0).with_alpha(26)))
        );

        theme
    }

    pub fn dark() -> Self {
        let palette = ColorPalette::dark();
        let mut theme = Self::new("Dark", palette);

        // Add default component styles
        theme.styles.insert(
            "button".to_string(),
            ComponentStyle::default()
                .background(palette.primary)
                .foreground(palette.on_primary)
                .border(Border::rounded(1, palette.primary.lighten(0.1), 8))
                .shadow(Shadow::new(0, 2, 4, ThemeColor::rgb(0, 0, 0).with_alpha(128)))
        );

        theme.styles.insert(
            "input".to_string(),
            ComponentStyle::default()
                .background(palette.surface)
                .foreground(palette.on_surface)
                .border(Border::solid(1, ThemeColor::rgb(99, 99, 102)))
        );

        theme.styles.insert(
            "card".to_string(),
            ComponentStyle::default()
                .background(palette.surface)
                .foreground(palette.on_surface)
                .border(Border::rounded(1, ThemeColor::rgb(58, 58, 60), 12))
                .shadow(Shadow::new(0, 1, 3, ThemeColor::rgb(0, 0, 0).with_alpha(76)))
        );

        theme
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize theme: {}", e))?;
        
        std::fs::write(path, json)
            .map_err(|e| format!("Failed to write theme file: {}", e))?;
        
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read theme file: {}", e))?;
        
        let theme = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse theme: {}", e))?;
        
        Ok(theme)
    }
}

/// Theme manager
pub struct ThemeManager {
    current_theme: Theme,
    themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_theme: Theme::light(),
            themes: HashMap::new(),
        };
        
        // Register default themes
        manager.register_theme(Theme::light());
        manager.register_theme(Theme::dark());
        
        manager
    }

    pub fn register_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    pub fn set_theme(&mut self, name: &str) -> Result<(), String> {
        if let Some(theme) = self.themes.get(name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.current_theme
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    pub fn get_all_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_palette(&self) -> &ColorPalette {
        &self.current_theme.palette
    }

    pub fn get_style(&self, component: &str) -> Option<&ComponentStyle> {
        self.current_theme.get_style(component)
    }

    pub fn toggle_light_dark(&mut self) -> bool {
        let new_theme = if self.current_theme.name == "Light" {
            "Dark"
        } else {
            "Light"
        };
        
        self.set_theme(new_theme).is_ok()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_color_creation() {
        let color = ThemeColor::rgb(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_theme_color_hex() {
        let color = ThemeColor::rgb(255, 0, 0);
        assert_eq!(color.to_hex(), "#FF0000FF");
    }

    #[test]
    fn test_theme_color_from_hex() {
        let color = ThemeColor::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();
        assert_eq!(palette.primary.r, 0);
        assert_eq!(palette.primary.g, 122);
        assert_eq!(palette.primary.b, 255);
    }

    #[test]
    fn test_color_palette_dark() {
        let palette = ColorPalette::dark();
        assert_eq!(palette.background.r, 28);
        assert_eq!(palette.background.g, 28);
        assert_eq!(palette.background.b, 30);
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("Custom", ColorPalette::default());
        assert_eq!(theme.name, "Custom");
        assert_eq!(theme.version, "1.0.0");
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new();
        assert_eq!(manager.get_current_theme().name, "Light");
        
        manager.set_theme("Dark").unwrap();
        assert_eq!(manager.get_current_theme().name, "Dark");
    }

    #[test]
    fn test_font_weight() {
        assert_eq!(FontWeight::Bold.to_number(), 700);
        assert_eq!(FontWeight::Regular.to_number(), 400);
    }

    #[test]
    fn test_component_style_builder() {
        let style = ComponentStyle::default()
            .background(ThemeColor::rgb(255, 0, 0))
            .foreground(ThemeColor::rgb(0, 0, 0));
        
        assert!(style.background.is_some());
        assert!(style.foreground.is_some());
    }
}