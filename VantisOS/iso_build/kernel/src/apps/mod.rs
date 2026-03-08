//! Desktop Applications
//! Built-in applications for VantisOS

pub mod file_manager;
pub mod terminal;
pub mod settings;
pub mod text_editor;
pub mod system_monitor;
pub mod calculator;
pub mod browser;
pub mod media_player;
pub mod image_viewer;
pub mod calendar;
pub mod notes;

use alloc::string::String;
use alloc::vec::Vec;
use crate::gui::*;

/// Application category
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppCategory {
    System,
    Utilities,
    Multimedia,
    Network,
    Office,
    Development,
    Games,
}

/// Application information
pub struct AppInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: Icon,
    pub category: AppCategory,
}

/// Application manager
pub struct AppManager {
    apps: Vec<AppInfo>,
}

impl AppManager {
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
        }
    }
    
    pub fn register(&mut self, app: AppInfo) {
        self.apps.push(app);
    }
    
    pub fn apps(&self) -> &[AppInfo] {
        &self.apps
    }
    
    pub fn apps_by_category(&self, category: AppCategory) -> impl Iterator<Item = &AppInfo> {
        self.apps.iter().filter(move |app| app.category == category)
    }
}

impl Default for AppManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in applications
pub const BUILTIN_APPS: &[AppInfo] = &[
    AppInfo {
        id: "file_manager",
        name: "Files",
        description: "File manager for browsing and managing files",
        icon: Icon::Folder,
        category: AppCategory::System,
    },
    AppInfo {
        id: "terminal",
        name: "Terminal",
        description: "Command-line terminal emulator",
        icon: Icon::Computer,
        category: AppCategory::System,
    },
    AppInfo {
        id: "settings",
        name: "Settings",
        description: "System configuration and preferences",
        icon: Icon::Settings,
        category: AppCategory::System,
    },
    AppInfo {
        id: "text_editor",
        name: "Text Editor",
        description: "Simple text editor",
        icon: Icon::File,
        category: AppCategory::Utilities,
    },
    AppInfo {
        id: "system_monitor",
        name: "System Monitor",
        description: "Monitor system resources and processes",
        icon: Icon::Computer,
        category: AppCategory::System,
    },
    AppInfo {
        id: "calculator",
        name: "Calculator",
        description: "Desktop calculator",
        icon: Icon::Application,
        category: AppCategory::Utilities,
    },
    AppInfo {
        id: "browser",
        name: "Web Browser",
        description: "Browse the internet",
        icon: Icon::Network,
        category: AppCategory::Network,
    },
    AppInfo {
        id: "media_player",
        name: "Media Player",
        description: "Play audio and video files",
        icon: Icon::Play,
        category: AppCategory::Multimedia,
    },
    AppInfo {
        id: "image_viewer",
        name: "Image Viewer",
        description: "View images and photos",
        icon: Icon::File,
        category: AppCategory::Multimedia,
    },
    AppInfo {
        id: "calendar",
        name: "Calendar",
        description: "View calendar and schedule events",
        icon: Icon::Calendar,
        category: AppCategory::Office,
    },
    AppInfo {
        id: "notes",
        name: "Notes",
        description: "Quick notes and reminders",
        icon: Icon::File,
        category: AppCategory::Office,
    },
];