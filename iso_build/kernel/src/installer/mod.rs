//! Installation Wizard
//! Complete GUI installer for VantisOS

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use crate::gui::*;

/// Installer state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallerState {
    Welcome,
    Language,
    Keyboard,
    Network,
    Partition,
    Users,
    Summary,
    Installing,
    Complete,
    Error,
}

/// Disk information
#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub size: u64,
    pub path: String,
    pub selected: bool,
}

/// Partition info
#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub name: String,
    pub size: u64,
    pub fstype: String,
    pub mount_point: String,
}

/// User account
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub full_name: String,
    pub password: String,
    pub admin: bool,
}

/// Installer
pub struct Installer {
    pub state: InstallerState,
    pub progress: u32,
    pub progress_text: String,
    pub error_message: String,
    
    // Configuration
    pub language: String,
    pub keyboard_layout: String,
    pub timezone: String,
    pub hostname: String,
    
    // Network
    pub network_auto: bool,
    pub wifi_ssid: String,
    pub wifi_password: String,
    
    // Disks
    pub disks: Vec<DiskInfo>,
    pub selected_disk: Option<usize>,
    pub partitions: Vec<PartitionInfo>,
    pub auto_partition: bool,
    pub use_encryption: bool,
    pub encryption_password: String,
    
    // Users
    pub users: Vec<UserAccount>,
    pub root_password: String,
    
    // Options
    pub install_bootloader: bool,
    pub bootloader_location: String,
    pub install_extras: bool,
    
    // Step tracking
    pub current_step: u32,
    pub total_steps: u32,
}

impl Installer {
    pub fn new() -> Self {
        Self {
            state: InstallerState::Welcome,
            progress: 0,
            progress_text: String::new(),
            error_message: String::new(),
            
            language: String::from("en_US"),
            keyboard_layout: String::from("us"),
            timezone: String::from("UTC"),
            hostname: String::from("vantis"),
            
            network_auto: true,
            wifi_ssid: String::new(),
            wifi_password: String::new(),
            
            disks: vec![
                DiskInfo {
                    name: String::from("/dev/sda - 256GB SSD"),
                    size: 256 * 1024 * 1024 * 1024,
                    path: String::from("/dev/sda"),
                    selected: false,
                },
                DiskInfo {
                    name: String::from("/dev/sdb - 1TB HDD"),
                    size: 1024 * 1024 * 1024 * 1024,
                    path: String::from("/dev/sdb"),
                    selected: false,
                },
            ],
            selected_disk: None,
            partitions: Vec::new(),
            auto_partition: true,
            use_encryption: false,
            encryption_password: String::new(),
            
            users: Vec::new(),
            root_password: String::new(),
            
            install_bootloader: true,
            bootloader_location: String::from("/dev/sda"),
            install_extras: true,
            
            current_step: 1,
            total_steps: 8,
        }
    }
    
    pub fn next_step(&mut self) {
        self.current_step += 1;
        self.state = match self.state {
            InstallerState::Welcome => InstallerState::Language,
            InstallerState::Language => InstallerState::Keyboard,
            InstallerState::Keyboard => InstallerState::Network,
            InstallerState::Network => InstallerState::Partition,
            InstallerState::Partition => InstallerState::Users,
            InstallerState::Users => InstallerState::Summary,
            InstallerState::Summary => InstallerState::Installing,
            InstallerState::Installing => InstallerState::Complete,
            InstallerState::Complete => InstallerState::Complete,
            InstallerState::Error => InstallerState::Error,
        };
    }
    
    pub fn prev_step(&mut self) {
        if self.current_step > 1 {
            self.current_step -= 1;
        }
        self.state = match self.state {
            InstallerState::Welcome => InstallerState::Welcome,
            InstallerState::Language => InstallerState::Welcome,
            InstallerState::Keyboard => InstallerState::Language,
            InstallerState::Network => InstallerState::Keyboard,
            InstallerState::Partition => InstallerState::Network,
            InstallerState::Users => InstallerState::Partition,
            InstallerState::Summary => InstallerState::Users,
            InstallerState::Installing => InstallerState::Summary,
            InstallerState::Complete => InstallerState::Complete,
            InstallerState::Error => InstallerState::Summary,
        };
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        // Background gradient
        surface.fill_rect(Rect::new(0, 0, width, height), Color::DESKTOP_BG);
        
        // Installer window
        let window_rect = Rect::new(
            (width as i32 - 800) / 2,
            (height as i32 - 600) / 2,
            800,
            600,
        );
        
        // Window background
        surface.fill_rect(window_rect, Color::new(32, 32, 32));
        surface.draw_rect(window_rect, Color::new(60, 60, 60), 1);
        
        // Title bar
        surface.fill_rect(
            Rect::new(window_rect.x, window_rect.y, window_rect.width, 48),
            Color::ACCENT,
        );
        
        // Title
        surface.draw_text_sized(
            window_rect.x as u32 + 20,
            window_rect.y as u32 + 14,
            "VantisOS Installer",
            20,
            Color::WHITE,
        );
        
        // Progress indicator
        let progress_y = window_rect.y + 60;
        let step_width = (window_rect.width - 60) / self.total_steps;
        for i in 0..self.total_steps {
            let step_x = window_rect.x + 30 + (i as i32) * (step_width as i32);
            let step_color = if i < self.current_step {
                Color::ACCENT
            } else if i == self.current_step - 1 {
                Color::new(100, 180, 255)
            } else {
                Color::new(60, 60, 60)
            };
            surface.fill_rounded_rect(
                Rect::new(step_x, progress_y, step_width - 4, 4),
                2,
                step_color,
            );
        }
        
        // Content area
        match self.state {
            InstallerState::Welcome => self.render_welcome(surface, window_rect),
            InstallerState::Language => self.render_language(surface, window_rect),
            InstallerState::Keyboard => self.render_keyboard(surface, window_rect),
            InstallerState::Network => self.render_network(surface, window_rect),
            InstallerState::Partition => self.render_partition(surface, window_rect),
            InstallerState::Users => self.render_users(surface, window_rect),
            InstallerState::Summary => self.render_summary(surface, window_rect),
            InstallerState::Installing => self.render_installing(surface, window_rect),
            InstallerState::Complete => self.render_complete(surface, window_rect),
            InstallerState::Error => self.render_error(surface, window_rect),
        }
        
        // Navigation buttons
        self.render_nav_buttons(surface, window_rect);
    }
    
    fn render_welcome(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Welcome to VantisOS!",
            28,
            Color::WHITE,
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 50) as u32,
            "This installer will guide you through the setup process.",
            14,
            Color::LIGHT_GRAY,
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 80) as u32,
            "VantisOS is a modern, quantum-ready operating system with:",
            14,
            Color::LIGHT_GRAY,
        );
        
        let features = [
            "Modern microkernel architecture",
            "Built-in post-quantum cryptography",
            "Beautiful, intuitive desktop environment",
            "Secure and private by design",
            "Quantum computing simulation support",
        ];
        
        for (i, feature) in features.iter().enumerate() {
            surface.draw_text_sized(
                window.x as u32 + 60,
                (y + 110 + i as i32 * 28) as u32,
                &format!("• {}", feature),
                13,
                Color::WHITE,
            );
        }
    }
    
    fn render_language(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Select Language",
            24,
            Color::WHITE,
        );
        
        let languages = ["English (US)", "Polski", "Deutsch", "Français", "Español", "日本語", "中文"];
        for (i, lang) in languages.iter().enumerate() {
            let selected = *lang == "English (US)";
            let bg = if selected { Color::ACCENT } else { Color::new(50, 50, 50) };
            surface.fill_rounded_rect(Rect::new(window.x + 40, y + 50 + (i as i32) * 40, 300, 35), 4, bg);
            surface.draw_text_sized(window.x as u32 + 50, (y + 58 + (i as i32) * 40) as u32, lang, 14, Color::WHITE);
        }
    }
    
    fn render_keyboard(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Keyboard Layout",
            24,
            Color::WHITE,
        );
        
        let layouts = ["US - English", "PL - Polish", "DE - German", "FR - French", "ES - Spanish"];
        for (i, layout) in layouts.iter().enumerate() {
            let selected = *layout == "US - English";
            let bg = if selected { Color::ACCENT } else { Color::new(50, 50, 50) };
            surface.fill_rounded_rect(Rect::new(window.x + 40, y + 50 + (i as i32) * 40, 300, 35), 4, bg);
            surface.draw_text_sized(window.x as u32 + 50, (y + 58 + (i as i32) * 40) as u32, layout, 14, Color::WHITE);
        }
        
        // Test input
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 280) as u32,
            "Test your keyboard:",
            14,
            Color::LIGHT_GRAY,
        );
        
        surface.fill_rect(
            Rect::new(window.x + 40, y + 310, 400, 35),
            Color::new(40, 40, 40),
        );
        surface.draw_rect(
            Rect::new(window.x + 40, y + 310, 400, 35),
            Color::GRAY,
            1,
        );
    }
    
    fn render_network(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Network Configuration",
            24,
            Color::WHITE,
        );
        
        // Auto configure
        let bg = if self.network_auto { Color::ACCENT } else { Color::new(50, 50, 50) };
        surface.fill_rounded_rect(Rect::new(window.x + 40, y + 50, 20, 20), 4, bg);
        surface.draw_text_sized(
            window.x as u32 + 70,
            (y + 52) as u32,
            "Configure automatically (DHCP)",
            14,
            Color::WHITE,
        );
        
        // WiFi section
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 100) as u32,
            "WiFi Network (optional):",
            14,
            Color::LIGHT_GRAY,
        );
        
        surface.fill_rect(
            Rect::new(window.x + 40, y + 130, 300, 35),
            Color::new(40, 40, 40),
        );
        surface.draw_rect(
            Rect::new(window.x + 40, y + 130, 300, 35),
            Color::GRAY,
            1,
        );
        surface.draw_text_sized(
            window.x as u32 + 50,
            (y + 138) as u32,
            "Network name (SSID)",
            12,
            Color::GRAY,
        );
        
        surface.fill_rect(
            Rect::new(window.x + 40, y + 180, 300, 35),
            Color::new(40, 40, 40),
        );
        surface.draw_rect(
            Rect::new(window.x + 40, y + 180, 300, 35),
            Color::GRAY,
            1,
        );
        surface.draw_text_sized(
            window.x as u32 + 50,
            (y + 188) as u32,
            "Password",
            12,
            Color::GRAY,
        );
    }
    
    fn render_partition(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Disk Configuration",
            24,
            Color::WHITE,
        );
        
        // Disks
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 40) as u32,
            "Available disks:",
            14,
            Color::LIGHT_GRAY,
        );
        
        for (i, disk) in self.disks.iter().enumerate() {
            let bg = if disk.selected { Color::ACCENT } else { Color::new(50, 50, 50) };
            surface.fill_rounded_rect(Rect::new(window.x + 40, y + 70 + (i as i32) * 50, 500, 40), 4, bg);
            surface.draw_text_sized(
                window.x as u32 + 50,
                (y + 80 + (i as i32) * 50) as u32,
                &disk.name,
                14,
                Color::WHITE,
            );
        }
        
        // Encryption option
        let enc_bg = if self.use_encryption { Color::ACCENT } else { Color::new(50, 50, 50) };
        surface.fill_rounded_rect(Rect::new(window.x + 40, y + 200, 20, 20), 4, enc_bg);
        surface.draw_text_sized(
            window.x as u32 + 70,
            (y + 202) as u32,
            "Encrypt disk (LUKS)",
            14,
            Color::WHITE,
        );
    }
    
    fn render_users(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "User Configuration",
            24,
            Color::WHITE,
        );
        
        // Root password
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 50) as u32,
            "Root password:",
            14,
            Color::LIGHT_GRAY,
        );
        surface.fill_rect(
            Rect::new(window.x + 40, y + 75, 300, 35),
            Color::new(40, 40, 40),
        );
        surface.draw_rect(
            Rect::new(window.x + 40, y + 75, 300, 35),
            Color::GRAY,
            1,
        );
        
        // Create user section
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 130) as u32,
            "Create user account:",
            14,
            Color::LIGHT_GRAY,
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 160) as u32,
            "Username:",
            12,
            Color::GRAY,
        );
        surface.fill_rect(
            Rect::new(window.x + 40, y + 180, 300, 30),
            Color::new(40, 40, 40),
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 220) as u32,
            "Full name:",
            12,
            Color::GRAY,
        );
        surface.fill_rect(
            Rect::new(window.x + 40, y + 240, 300, 30),
            Color::new(40, 40, 40),
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 280) as u32,
            "Password:",
            12,
            Color::GRAY,
        );
        surface.fill_rect(
            Rect::new(window.x + 40, y + 300, 300, 30),
            Color::new(40, 40, 40),
        );
        
        let admin_bg = if true { Color::ACCENT } else { Color::new(50, 50, 50) };
        surface.fill_rounded_rect(Rect::new(window.x + 40, y + 340, 20, 20), 4, admin_bg);
        surface.draw_text_sized(
            window.x as u32 + 70,
            (y + 342) as u32,
            "Administrator account",
            14,
            Color::WHITE,
        );
    }
    
    fn render_summary(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 100;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Installation Summary",
            24,
            Color::WHITE,
        );
        
        let summary_items: [(&str, &str); 7] = [
            ("Language:", &self.language),
            ("Keyboard:", &self.keyboard_layout),
            ("Timezone:", &self.timezone),
            ("Hostname:", &self.hostname),
            ("Target disk:", self.disks.get(0).map(|d| d.name.as_str()).unwrap_or("None")),
            ("Encryption:", if self.use_encryption { "Yes" } else { "No" }),
            ("Bootloader:", if self.install_bootloader { "Yes" } else { "No" }),
        ];
        
        for (i, (label, value)) in summary_items.iter().enumerate() {
            surface.draw_text_sized(
                window.x as u32 + 60,
                (y + 50 + (i as i32) * 30) as u32,
                label,
                13,
                Color::LIGHT_GRAY,
            );
            surface.draw_text_sized(
                window.x as u32 + 200,
                (y + 50 + (i as i32) * 30) as u32,
                value,
                13,
                Color::WHITE,
            );
        }
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 300) as u32,
            "Click 'Install' to begin the installation.",
            14,
            Color::LIGHT_GRAY,
        );
    }
    
    fn render_installing(&self, surface: &mut dyn Surface, window: Rect) {
        let y = window.y + 200;
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            y as u32,
            "Installing VantisOS...",
            24,
            Color::WHITE,
        );
        
        // Progress bar
        surface.fill_rect(
            Rect::new(window.x + 40, y + 50, 720, 30),
            Color::new(50, 50, 50),
        );
        if self.progress > 0 {
            let fill_width = 720 * self.progress / 100;
            surface.fill_rect(
                Rect::new(window.x + 40, y + 50, fill_width, 30),
                Color::ACCENT,
            );
        }
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 100) as u32,
            &self.progress_text,
            14,
            Color::LIGHT_GRAY,
        );
    }
    
    fn render_complete(&self, surface: &mut dyn Surface, window: Rect) {
        let width = surface.width();
        let y = window.y + 150;
        
        surface.draw_text_sized(
            (width / 2 - 100) as u32,
            y as u32,
            "Installation Complete!",
            28,
            Color::new(100, 255, 100),
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 60) as u32,
            "VantisOS has been successfully installed on your system.",
            14,
            Color::LIGHT_GRAY,
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 90) as u32,
            "Click 'Restart' to reboot and start using VantisOS.",
            14,
            Color::LIGHT_GRAY,
        );
    }
    
    fn render_error(&self, surface: &mut dyn Surface, window: Rect) {
        let width = surface.width();
        let y = window.y + 150;
        
        surface.draw_text_sized(
            (width / 2 - 80) as u32,
            y as u32,
            "Installation Error",
            28,
            Color::RED,
        );
        
        surface.draw_text_sized(
            window.x as u32 + 40,
            (y + 60) as u32,
            &self.error_message,
            14,
            Color::LIGHT_GRAY,
        );
    }
    
    fn render_nav_buttons(&self, surface: &mut dyn Surface, window: Rect) {
        let btn_y = window.y + window.height as i32 - 60;
        
        // Back button
        if self.state != InstallerState::Welcome && self.state != InstallerState::Installing {
            surface.fill_rounded_rect(Rect::new(window.x + 40, btn_y, 100, 40), 4, Color::new(60, 60, 60));
            surface.draw_text_sized(
                window.x as u32 + 75,
                (btn_y + 12) as u32,
                "Back",
                14,
                Color::WHITE,
            );
        }
        
        // Next/Install/Restart button
        let (btn_text, btn_color) = match self.state {
            InstallerState::Summary => ("Install", Color::new(0, 150, 0)),
            InstallerState::Installing => ("Wait...", Color::GRAY),
            InstallerState::Complete => ("Restart", Color::ACCENT),
            InstallerState::Error => ("Retry", Color::ACCENT),
            _ => ("Next", Color::ACCENT),
        };
        
        surface.fill_rounded_rect(
            Rect::new(window.x + window.width as i32 - 140, btn_y, 100, 40),
            4,
            btn_color,
        );
        surface.draw_text_sized(
            (window.x + window.width as i32 - 140 + 50 - 20) as u32,
            (btn_y + 12) as u32,
            btn_text,
            14,
            Color::WHITE,
        );
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}