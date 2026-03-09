//! # Radial Shell - Gesture-Driven Circular Menu Interface
//! 
//! Modern circular menu interface optimized for touch and gesture input.
//! Provides an intuitive, spatial navigation experience with visual feedback.

use super::Shell;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use crate::ui::flux::{InputManager, InputEvent, GestureEvent, GestureType, ThemeManager, ColorPalette};

/// Menu item in the radial menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialMenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub action: MenuAction,
    pub color: Option<String>,
    pub shortcut: Option<String>,
}

/// Action to perform when menu item is selected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuAction {
    Launch { app: String },
    Command { command: String },
    Submenu { menu_id: String },
    Custom { callback: String },
    None,
}

/// Radial menu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialMenuConfig {
    pub radius: f32,
    pub item_radius: f32,
    pub center_radius: f32,
    pub animation_duration: Duration,
    pub haptic_feedback: bool,
    pub show_labels: bool,
    pub show_icons: bool,
    pub circular_arrangement: bool,
}

impl Default for RadialMenuConfig {
    fn default() -> Self {
        Self {
            radius: 200.0,
            item_radius: 40.0,
            center_radius: 30.0,
            animation_duration: Duration::from_millis(300),
            haptic_feedback: true,
            show_labels: true,
            show_icons: true,
            circular_arrangement: true,
        }
    }
}

/// Radial menu state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadialMenuState {
    Hidden,
    Expanding,
    Expanded,
    Collapsing,
}

/// Menu sector for organizing items
#[derive(Debug, Clone)]
pub struct MenuSector {
    pub id: String,
    pub label: String,
    pub start_angle: f32,
    pub end_angle: f32,
    pub color: String,
    pub items: Vec<RadialMenuItem>,
}

impl MenuSector {
    pub fn new(id: &str, label: &str, start_angle: f32, end_angle: f32, color: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            start_angle,
            end_angle,
            color: color.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: RadialMenuItem) {
        self.items.push(item);
    }

    pub fn contains_angle(&self, angle: f32) -> bool {
        let normalized_angle = angle.rem_euclid(360.0);
        let normalized_start = self.start_angle.rem_euclid(360.0);
        let normalized_end = self.end_angle.rem_euclid(360.0);
        
        if normalized_start < normalized_end {
            normalized_angle >= normalized_start && normalized_angle < normalized_end
        } else {
            normalized_angle >= normalized_start || normalized_angle < normalized_end
        }
    }
}

/// Radial menu
pub struct RadialMenu {
    pub config: RadialMenuConfig,
    pub sectors: Vec<MenuSector>,
    pub state: RadialMenuState,
    pub center_x: f32,
    pub center_y: f32,
    pub animation_progress: f32,
    pub selected_sector: Option<usize>,
    pub selected_item: Option<(usize, usize)>,
    pub last_activation: Option<Instant>,
}

impl RadialMenu {
    pub fn new(config: RadialMenuConfig) -> Self {
        Self {
            config,
            sectors: Vec::new(),
            state: RadialMenuState::Hidden,
            center_x: 0.0,
            center_y: 0.0,
            animation_progress: 0.0,
            selected_sector: None,
            selected_item: None,
            last_activation: None,
        }
    }

    pub fn with_center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x;
        self.center_y = y;
        self
    }

    pub fn add_sector(&mut self, sector: MenuSector) {
        self.sectors.push(sector);
    }

    pub fn show(&mut self, x: f32, y: f32) {
        self.center_x = x;
        self.center_y = y;
        self.state = RadialMenuState::Expanding;
        self.animation_progress = 0.0;
    }

    pub fn hide(&mut self) {
        self.state = RadialMenuState::Collapsing;
        self.animation_progress = 0.0;
    }

    pub fn toggle(&mut self, x: f32, y: f32) {
        match self.state {
            RadialMenuState::Hidden => self.show(x, y),
            RadialMenuState::Expanded => self.hide(),
            _ => {}
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let delta = dt.as_secs_f32();
        
        match self.state {
            RadialMenuState::Expanding => {
                self.animation_progress += delta / self.config.animation_duration.as_secs_f32();
                if self.animation_progress >= 1.0 {
                    self.animation_progress = 1.0;
                    self.state = RadialMenuState::Expanded;
                }
            }
            RadialMenuState::Collapsing => {
                self.animation_progress += delta / self.config.animation_duration.as_secs_f32();
                if self.animation_progress >= 1.0 {
                    self.animation_progress = 1.0;
                    self.state = RadialMenuState::Hidden;
                    self.selected_sector = None;
                    self.selected_item = None;
                }
            }
            _ => {}
        }
    }

    pub fn handle_gesture(&mut self, gesture: &GestureEvent) -> Option<MenuAction> {
        if self.state != RadialMenuState::Expanded {
            return None;
        }

        let dx = gesture.center_x - self.center_x;
        let dy = gesture.center_y - self.center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees() + 90.0;

        // Check if gesture is within menu radius
        if distance < self.config.center_radius || distance > self.config.radius {
            self.selected_sector = None;
            self.selected_item = None;
            return None;
        }

        // Find which sector the gesture is in
        for (sector_idx, sector) in self.sectors.iter().enumerate() {
            if sector.contains_angle(angle) {
                self.selected_sector = Some(sector_idx);
                
                // Find which item is being pointed to
                let item_angle_step = (sector.end_angle - sector.start_angle) / sector.items.len() as f32;
                let relative_angle = angle - sector.start_angle;
                let item_idx = (relative_angle / item_angle_step) as usize;
                
                if item_idx < sector.items.len() {
                    self.selected_item = Some((sector_idx, item_idx));
                    
                    // If it's a tap gesture, return the action
                    if gesture.gesture_type == GestureType::Tap {
                        if let Some(cooldown) = self.last_activation {
                            if cooldown.elapsed() < Duration::from_millis(500) {
                                return None;
                            }
                        }
                        self.last_activation = Some(Instant::now());
                        return Some(sector.items[item_idx].action.clone());
                    }
                }
                break;
            }
        }

        None
    }

    pub fn is_visible(&self) -> bool {
        self.state != RadialMenuState::Hidden
    }

    pub fn get_current_radius(&self) -> f32 {
        let progress = if self.state == RadialMenuState::Collapsing {
            1.0 - self.animation_progress
        } else {
            self.animation_progress
        };
        self.config.radius * ease_out_cubic(progress)
    }

    pub fn get_item_position(&self, sector_idx: usize, item_idx: usize) -> Option<(f32, f32)> {
        let sector = self.sectors.get(sector_idx)?;
        let item_angle_step = (sector.end_angle - sector.start_angle) / sector.items.len().max(1) as f32;
        let angle_rad = (sector.start_angle + item_angle_step * (item_idx as f32 + 0.5) - 90.0).to_radians();
        let radius = self.get_current_radius();
        
        let x = self.center_x + angle_rad.cos() * radius;
        let y = self.center_y + angle_rad.sin() * radius;
        
        Some((x, y))
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Gesture hint
#[derive(Debug, Clone)]
pub struct GestureHint {
    pub gesture_type: GestureType,
    pub description: String,
    pub animation_duration: Duration,
}

impl GestureHint {
    pub fn new(gesture_type: GestureType, description: &str) -> Self {
        Self {
            gesture_type,
            description: description.to_string(),
            animation_duration: Duration::from_millis(1000),
        }
    }
}

/// Radial shell
pub struct RadialShell {
    input_manager: InputManager,
    theme_manager: ThemeManager,
    main_menu: RadialMenu,
    submenus: HashMap<String, RadialMenu>,
    active_menu: Option<String>,
    gesture_hints: Vec<GestureHint>,
    running: bool,
}

impl RadialShell {
    pub fn new() -> Self {
        let config = RadialMenuConfig::default();
        let mut main_menu = RadialMenu::new(config);
        
        // Create default sectors
        main_menu.add_sector(MenuSector::new("apps", "Apps", 0.0, 90.0, "#007AFF"));
        main_menu.add_sector(MenuSector::new("system", "System", 90.0, 180.0, "#5856D6"));
        main_menu.add_sector(MenuSector::new("settings", "Settings", 180.0, 270.0, "#FF9500"));
        main_menu.add_sector(MenuSector::new("tools", "Tools", 270.0, 360.0, "#34C759"));
        
        Self {
            input_manager: InputManager::new(),
            theme_manager: ThemeManager::new(),
            main_menu,
            submenus: HashMap::new(),
            active_menu: None,
            gesture_hints: Vec::new(),
            running: false,
        }
    }

    pub fn add_submenu(&mut self, id: &str, menu: RadialMenu) {
        self.submenus.insert(id.to_string(), menu);
    }

    pub fn add_gesture_hint(&mut self, hint: GestureHint) {
        self.gesture_hints.push(hint);
    }

    pub fn add_menu_item(&mut self, sector_id: &str, item: RadialMenuItem) {
        for sector in &mut self.main_menu.sectors {
            if sector.id == sector_id {
                sector.add_item(item);
                break;
            }
        }
    }

    fn process_input(&mut self) -> Vec<MenuAction> {
        let mut actions = Vec::new();
        
        // Process events and check for gestures
        let events = self.input_manager.process_event(InputEvent::MouseMove(
            crate::ui::flux::MousePosition::new(0.0, 0.0)
        ));
        
        for event in events {
            if let InputEvent::Gesture(gesture) = event {
                let action = if let Some(menu_id) = &self.active_menu {
                    if let Some(submenu) = self.submenus.get_mut(menu_id) {
                        submenu.handle_gesture(&gesture)
                    } else {
                        None
                    }
                } else {
                    self.main_menu.handle_gesture(&gesture)
                };
                
                if let Some(action) = action {
                    actions.push(action);
                    
                    // Handle submenu navigation
                    if let MenuAction::Submenu { menu_id } = &action {
                        self.active_menu = Some(menu_id.clone());
                    }
                }
            }
        }
        
        actions
    }

    pub fn update(&mut self, dt: Duration) -> Vec<MenuAction> {
        self.main_menu.update(dt);
        for menu in self.submenus.values_mut() {
            menu.update(dt);
        }
        
        self.process_input()
    }
}

impl Shell for RadialShell {
    fn run(&self) {
        println!("Starting Radial Shell - Gesture-Driven Circular Menu Interface");
        println!("Use gestures to navigate the circular menu");
        println!("Available gestures: tap, double-tap, long-press, pan, pinch, rotate, swipe");
    }
}

impl Default for RadialShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radial_menu_creation() {
        let config = RadialMenuConfig::default();
        let menu = RadialMenu::new(config);
        assert_eq!(menu.state, RadialMenuState::Hidden);
    }

    #[test]
    fn test_radial_menu_show_hide() {
        let config = RadialMenuConfig::default();
        let mut menu = RadialMenu::new(config);
        
        menu.show(100.0, 100.0);
        assert_eq!(menu.state, RadialMenuState::Expanding);
        
        menu.hide();
        assert_eq!(menu.state, RadialMenuState::Collapsing);
    }

    #[test]
    fn test_menu_sector_creation() {
        let sector = MenuSector::new("test", "Test", 0.0, 90.0, "#FF0000");
        assert_eq!(sector.id, "test");
        assert_eq!(sector.label, "Test");
    }

    #[test]
    fn test_menu_sector_contains_angle() {
        let sector = MenuSector::new("test", "Test", 0.0, 90.0, "#FF0000");
        assert!(sector.contains_angle(45.0));
        assert!(!sector.contains_angle(135.0));
    }

    #[test]
    fn test_radial_menu_item_creation() {
        let item = RadialMenuItem {
            id: "test".to_string(),
            label: "Test".to_string(),
            icon: Some("icon.png".to_string()),
            action: MenuAction::Launch { app: "test_app".to_string() },
            color: Some("#FF0000".to_string()),
            shortcut: Some("Ctrl+T".to_string()),
        };
        assert_eq!(item.id, "test");
    }

    #[test]
    fn test_radial_shell_creation() {
        let shell = RadialShell::new();
        assert_eq!(shell.main_menu.sectors.len(), 4);
    }
}