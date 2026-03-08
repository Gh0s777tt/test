// System UI for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework - System UI

use super::framework::{UIElement, UIElementId, UIElementType, UIElementState, UIRect, UIColor, BaseUIElement};
use super::touch_event::TouchEvent;
use super::widgets::{Button, Label, TextAlignment, WidgetStyle};

// Status bar
pub struct StatusBar {
    base: BaseUIElement,
    time_label: Label,
    battery_label: Label,
    network_label: Label,
    height: u32,
}

impl StatusBar {
    pub fn new(id: UIElementId, screen_width: u32) -> Self {
        let rect = UIRect::new(0, 0, screen_width, 32);
        let mut status_bar = StatusBar {
            base: BaseUIElement::new(id, UIElementType::Custom, rect),
            time_label: Label::new(0, UIRect::new(10, 8, 100, 16), "12:00"),
            battery_label: Label::new(1, UIRect::new(screen_width as i32 - 120, 8, 100, 16), "100%"),
            network_label: Label::new(2, UIRect::new(screen_width as i32 - 240, 8, 100, 16), "WiFi"),
            height: 32,
        };

        status_bar.time_label.set_text_color(UIColor::white());
        status_bar.battery_label.set_text_color(UIColor::white());
        status_bar.network_label.set_text_color(UIColor::white());
        status_bar.base.set_background_color(UIColor::rgb(0, 0, 0));

        status_bar
    }

    pub fn set_time(&mut self, time: &str) {
        self.time_label.set_text(time);
    }

    pub fn set_battery(&mut self, battery: &str) {
        self.battery_label.set_text(battery);
    }

    pub fn set_network(&mut self, network: &str) {
        self.network_label.set_text(network);
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl UIElement for StatusBar {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        // Render status bar background
        self.base.render();

        // Render labels
        self.time_label.render();
        self.battery_label.render();
        self.network_label.render();
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        // Status bar doesn't handle touch events
    }
}

// Notification
pub struct Notification {
    id: u32,
    title: [u8; 128],
    title_len: usize,
    message: [u8; 256],
    message_len: usize,
    icon: u32,
    timestamp: u64,
    priority: NotificationPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Notification {
    pub fn new(id: u32, title: &str, message: &str) -> Self {
        let mut notification = Notification {
            id,
            title: [0; 128],
            title_len: 0,
            message: [0; 256],
            message_len: 0,
            icon: 0,
            timestamp: Self::get_timestamp(),
            priority: NotificationPriority::Normal,
        };

        notification.set_title(title);
        notification.set_message(message);
        notification
    }

    pub fn set_title(&mut self, title: &str) {
        self.title_len = title.len().min(127);
        for (i, byte) in title.bytes().enumerate().take(127) {
            self.title[i] = byte;
        }
    }

    pub fn get_title(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.title[..self.title_len])
        }
    }

    pub fn set_message(&mut self, message: &str) {
        self.message_len = message.len().min(255);
        for (i, byte) in message.bytes().enumerate().take(255) {
            self.message[i] = byte;
        }
    }

    pub fn get_message(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.message[..self.message_len])
        }
    }

    pub fn set_icon(&mut self, icon: u32) {
        self.icon = icon;
    }

    pub fn set_priority(&mut self, priority: NotificationPriority) {
        self.priority = priority;
    }

    fn get_timestamp() -> u64 {
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

// Notification system
pub struct NotificationSystem {
    notifications: [Option<Notification>; 50],
    num_notifications: usize,
    next_id: u32,
}

impl NotificationSystem {
    pub const fn new() -> Self {
        NotificationSystem {
            notifications: [None; 50],
            num_notifications: 0,
            next_id: 1,
        }
    }

    pub fn add_notification(&mut self, title: &str, message: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let notification = Notification::new(id, title, message);

        if self.num_notifications < 50 {
            for i in 0..50 {
                if self.notifications[i].is_none() {
                    self.notifications[i] = Some(notification);
                    self.num_notifications += 1;
                    return id;
                }
            }
        }

        id
    }

    pub fn remove_notification(&mut self, id: u32) -> bool {
        for i in 0..self.num_notifications {
            if let Some(ref notification) = self.notifications[i] {
                if notification.id == id {
                    self.notifications[i] = None;
                    self.num_notifications -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn get_notifications(&self) -> Vec<&Notification> {
        let mut result = Vec::new();
        for notification in &self.notifications {
            if let Some(ref n) = notification {
                result.push(n);
            }
        }
        result
    }

    pub fn clear(&mut self) {
        for i in 0..50 {
            self.notifications[i] = None;
        }
        self.num_notifications = 0;
    }

    pub fn len(&self) -> usize {
        self.num_notifications
    }

    pub fn is_empty(&self) -> bool {
        self.num_notifications == 0
    }
}

impl Default for NotificationSystem {
    fn default() -> Self {
        Self::new()
    }
}

// Quick settings panel
pub struct QuickSettingsPanel {
    base: BaseUIElement,
    wifi_button: Button,
    bluetooth_button: Button,
    airplane_button: Button,
    brightness_slider: u8,
    is_visible: bool,
}

impl QuickSettingsPanel {
    pub fn new(id: UIElementId, screen_width: u32) -> Self {
        let rect = UIRect::new(0, 32, screen_width, 400);
        let mut panel = QuickSettingsPanel {
            base: BaseUIElement::new(id, UIElementType::Custom, rect),
            wifi_button: Button::new(0, UIRect::new(20, 50, 150, 50), "WiFi"),
            bluetooth_button: Button::new(1, UIRect::new(190, 50, 150, 50), "Bluetooth"),
            airplane_button: Button::new(2, UIRect::new(360, 50, 150, 50), "Airplane"),
            brightness_slider: 100,
            is_visible: false,
        };

        panel.wifi_button.set_style(WidgetStyle::Primary);
        panel.bluetooth_button.set_style(WidgetStyle::Primary);
        panel.airplane_button.set_style(WidgetStyle::Secondary);
        panel.base.set_background_color(UIColor::rgb(240, 240, 240));

        panel
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        self.base.set_state(UIElementState::Normal);
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.base.set_state(UIElementState::Hidden);
    }

    pub fn toggle(&mut self) {
        if self.is_visible {
            self.hide();
        } else {
            self.show();
        }
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness_slider = brightness;
    }

    pub fn get_brightness(&self) -> u8 {
        self.brightness_slider
    }
}

impl UIElement for QuickSettingsPanel {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible {
            return;
        }

        // Render panel background
        self.base.render();

        // Render buttons
        self.wifi_button.render();
        self.bluetooth_button.render();
        self.airplane_button.render();

        // TODO: Render brightness slider
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        if !self.is_visible || !self.is_enabled() {
            return;
        }

        self.wifi_button.handle_touch_event(event);
        self.bluetooth_button.handle_touch_event(event);
        self.airplane_button.handle_touch_event(event);
    }
}

// Lock screen
pub struct LockScreen {
    base: BaseUIElement,
    pin_field: super::widgets::TextField,
    unlock_button: Button,
    time_label: Label,
    date_label: Label,
    is_locked: bool,
}

impl LockScreen {
    pub fn new(id: UIElementId, screen_width: u32, screen_height: u32) -> Self {
        let rect = UIRect::new(0, 0, screen_width, screen_height);
        let mut lock_screen = LockScreen {
            base: BaseUIElement::new(id, UIElementType::Custom, rect),
            pin_field: super::widgets::TextField::new(0, UIRect::new(screen_width as i32 / 2 - 100, screen_height as i32 / 2, 200, 50), "Enter PIN"),
            unlock_button: Button::new(1, UIRect::new(screen_width as i32 / 2 - 75, screen_height as i32 / 2 + 70, 150, 50), "Unlock"),
            time_label: Label::new(2, UIRect::new(screen_width as i32 / 2 - 100, screen_height as i32 / 2 - 150, 200, 50), "12:00"),
            date_label: Label::new(3, UIRect::new(screen_width as i32 / 2 - 100, screen_height as i32 / 2 - 100, 200, 30), "March 1, 2025"),
            is_locked: true,
        };

        lock_screen.time_label.set_alignment(TextAlignment::Center);
        lock_screen.date_label.set_alignment(TextAlignment::Center);
        lock_screen.time_label.set_text_color(UIColor::white());
        lock_screen.date_label.set_text_color(UIColor::white());
        lock_screen.unlock_button.set_style(WidgetStyle::Primary);
        lock_screen.base.set_background_color(UIColor::rgb(30, 30, 30));

        lock_screen
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
        self.base.set_state(UIElementState::Normal);
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.base.set_state(UIElementState::Hidden);
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn set_time(&mut self, time: &str) {
        self.time_label.set_text(time);
    }

    pub fn set_date(&mut self, date: &str) {
        self.date_label.set_text(date);
    }

    pub fn get_pin(&self) -> &str {
        self.pin_field.get_text()
    }
}

impl UIElement for LockScreen {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        // Render lock screen background
        self.base.render();

        // Render time and date
        self.time_label.render();
        self.date_label.render();

        // Render PIN field and unlock button
        self.pin_field.render();
        self.unlock_button.render();
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        if !self.is_visible || !self.is_enabled() {
            return;
        }

        self.pin_field.handle_touch_event(event);
        self.unlock_button.handle_touch_event(event);

        // Check if unlock button was pressed
        if event.event_type == super::touch_event::TouchEventType::Up {
            if let Some(point) = event.get_primary_point() {
                if self.unlock_button.get_rect().contains(point.x, point.y) {
                    // TODO: Validate PIN and unlock
                    self.unlock();
                }
            }
        }
    }
}

// Home screen
pub struct HomeScreen {
    base: BaseUIElement,
    app_grid: [Option<AppIcon>; 24],
    num_apps: usize,
    dock: [Option<AppIcon>; 4],
    num_dock_apps: usize,
}

pub struct AppIcon {
    id: u32,
    name: [u8; 64],
    name_len: usize,
    icon: u32,
    rect: UIRect,
}

impl AppIcon {
    pub fn new(id: u32, name: &str, icon: u32, rect: UIRect) -> Self {
        let mut app_icon = AppIcon {
            id,
            name: [0; 64],
            name_len: 0,
            icon,
            rect,
        };

        app_icon.set_name(name);
        app_icon
    }

    pub fn set_name(&mut self, name: &str) {
        self.name_len = name.len().min(63);
        for (i, byte) in name.bytes().enumerate().take(63) {
            self.name[i] = byte;
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.name[..self.name_len])
        }
    }
}

impl HomeScreen {
    pub fn new(id: UIElementId, screen_width: u32, screen_height: u32) -> Self {
        let rect = UIRect::new(0, 0, screen_width, screen_height);
        let mut home_screen = HomeScreen {
            base: BaseUIElement::new(id, UIElementType::Custom, rect),
            app_grid: [None; 24],
            num_apps: 0,
            dock: [None; 4],
            num_dock_apps: 0,
        };

        home_screen.base.set_background_color(UIColor::rgb(245, 245, 245));
        home_screen
    }

    pub fn add_app(&mut self, name: &str, icon: u32) {
        if self.num_apps >= 24 {
            return;
        }

        let row = self.num_apps / 4;
        let col = self.num_apps % 4;
        let x = 20 + (col * 120) as i32;
        let y = 100 + (row * 140) as i32;
        let rect = UIRect::new(x, y, 100, 120);

        let app_icon = AppIcon::new(self.num_apps as u32, name, icon, rect);
        self.app_grid[self.num_apps] = Some(app_icon);
        self.num_apps += 1;
    }

    pub fn add_dock_app(&mut self, name: &str, icon: u32) {
        if self.num_dock_apps >= 4 {
            return;
        }

        let x = 20 + (self.num_dock_apps * 120) as i32;
        let y = 800;
        let rect = UIRect::new(x, y, 100, 120);

        let app_icon = AppIcon::new(self.num_dock_apps as u32, name, icon, rect);
        self.dock[self.num_dock_apps] = Some(app_icon);
        self.num_dock_apps += 1;
    }

    pub fn get_apps(&self) -> Vec<&AppIcon> {
        let mut result = Vec::new();
        for app in &self.app_grid {
            if let Some(ref a) = app {
                result.push(a);
            }
        }
        result
    }

    pub fn get_dock_apps(&self) -> Vec<&AppIcon> {
        let mut result = Vec::new();
        for app in &self.dock {
            if let Some(ref a) = app {
                result.push(a);
            }
        }
        result
    }
}

impl UIElement for HomeScreen {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        // Render home screen background
        self.base.render();

        // TODO: Render app icons
        // TODO: Render dock
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        if !self.is_visible || !self.is_enabled() {
            return;
        }

        // TODO: Handle app icon touches
    }
}
