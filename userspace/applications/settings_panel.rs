//! VantisOS Settings Panel
//! 
//! A comprehensive settings panel for system configuration.

use std::collections::HashMap;
use std::path::PathBuf;

/// Settings panel application
pub struct SettingsPanel {
    /// Configuration
    config: SettingsConfig,
    /// Settings categories
    categories: Vec<SettingsCategory>,
    /// Current category
    current_category: Category,
    /// Settings values
    values: HashMap<String, SettingValue>,
    /// Unsaved changes
    has_unsaved_changes: bool,
}

/// Settings configuration
#[derive(Clone, Debug)]
pub struct SettingsConfig {
    /// Show advanced settings
    pub show_advanced: bool,
    /// Confirm changes
    pub confirm_changes: bool,
    /// Auto-save on change
    pub auto_save: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            show_advanced: false,
            confirm_changes: true,
            auto_save: false,
        }
    }
}

/// Settings category
#[derive(Clone, Debug)]
pub struct SettingsCategory {
    /// Category type
    pub category: Category,
    /// Display name
    pub name: String,
    /// Icon
    pub icon: String,
    /// Description
    pub description: String,
    /// Settings in this category
    pub settings: Vec<Setting>,
}

/// Settings category type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    /// Display settings
    Display,
    /// Sound settings
    Sound,
    /// Network settings
    Network,
    /// Bluetooth settings
    Bluetooth,
    /// Keyboard settings
    Keyboard,
    /// Mouse settings
    Mouse,
    /// Touchpad settings
    Touchpad,
    /// Power settings
    Power,
    /// Storage settings
    Storage,
    /// Appearance settings
    Appearance,
    /// Accessibility settings
    Accessibility,
    /// Privacy settings
    Privacy,
    /// Security settings
    Security,
    /// System settings
    System,
    /// About
    About,
    /// Advanced settings
    Advanced,
}

impl Category {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Display => "Display",
            Self::Sound => "Sound",
            Self::Network => "Network",
            Self::Bluetooth => "Bluetooth",
            Self::Keyboard => "Keyboard",
            Self::Mouse => "Mouse",
            Self::Touchpad => "Touchpad",
            Self::Power => "Power",
            Self::Storage => "Storage",
            Self::Appearance => "Appearance",
            Self::Accessibility => "Accessibility",
            Self::Privacy => "Privacy",
            Self::Security => "Security",
            Self::System => "System",
            Self::About => "About",
            Self::Advanced => "Advanced",
        }
    }
    
    pub fn icon(&self) -> &str {
        match self {
            Self::Display => "display",
            Self::Sound => "volume-high",
            Self::Network => "wifi",
            Self::Bluetooth => "bluetooth",
            Self::Keyboard => "keyboard",
            Self::Mouse => "mouse",
            Self::Touchpad => "touchpad",
            Self::Power => "power",
            Self::Storage => "hard-drive",
            Self::Appearance => "palette",
            Self::Accessibility => "accessibility",
            Self::Privacy => "lock",
            Self::Security => "shield",
            Self::System => "cog",
            Self::About => "info",
            Self::Advanced => "settings",
        }
    }
}

/// Setting
#[derive(Clone, Debug)]
pub struct Setting {
    /// Setting key (unique identifier)
    pub key: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Setting type
    pub setting_type: SettingType,
    /// Current value
    pub value: SettingValue,
    /// Default value
    pub default: SettingValue,
    /// Is advanced
    pub advanced: bool,
    /// Requires restart
    pub requires_restart: bool,
    /// Options (for select/enum types)
    pub options: Vec<SettingOption>,
    /// Minimum value (for numeric types)
    pub min: Option<SettingValue>,
    /// Maximum value (for numeric types)
    pub max: Option<SettingValue>,
}

/// Setting type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingType {
    /// Boolean toggle
    Boolean,
    /// Integer number
    Integer,
    /// Float number
    Float,
    /// String
    String,
    /// Path
    Path,
    /// Select from options
    Select,
    /// Color picker
    Color,
    /// Slider (integer)
    Slider,
    /// Enum
    Enum,
    /// List of strings
    List,
    /// Map of key-value pairs
    Map,
}

/// Setting value
#[derive(Clone, Debug, PartialEq)]
pub enum SettingValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Path(PathBuf),
    Color(Color),
    List(Vec<String>),
    Map(HashMap<String, String>),
}

impl SettingValue {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }
    
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
}

/// Setting option
#[derive(Clone, Debug)]
pub struct SettingOption {
    /// Option value
    pub value: SettingValue,
    /// Display name
    pub name: String,
    /// Description
    pub description: Option<String>,
}

/// Color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }
    
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 24) |
        ((self.g as u32) << 16) |
        ((self.b as u32) << 8) |
        (self.a as u32)
    }
}

/// Settings error
#[derive(Debug, Clone)]
pub enum SettingsError {
    /// Setting not found
    SettingNotFound(String),
    /// Invalid value type
    InvalidType(String),
    /// Value out of range
    OutOfRange(String),
    /// Read failed
    ReadFailed(String),
    /// Write failed
    WriteFailed(String),
    /// Permission denied
    PermissionDenied(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SettingsError::SettingNotFound(key) => write!(f, "Setting not found: {}", key),
            SettingsError::InvalidType(msg) => write!(f, "Invalid value type: {}", msg),
            SettingsError::OutOfRange(msg) => write!(f, "Value out of range: {}", msg),
            SettingsError::ReadFailed(msg) => write!(f, "Read failed: {}", msg),
            SettingsError::WriteFailed(msg) => write!(f, "Write failed: {}", msg),
            SettingsError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for SettingsError {}

impl SettingsPanel {
    /// Create a new settings panel
    pub fn new() -> Self {
        let mut panel = Self {
            config: SettingsConfig::default(),
            categories: Vec::new(),
            current_category: Category::Display,
            values: HashMap::new(),
            has_unsaved_changes: false,
        };
        
        panel.init_categories();
        panel
    }
    
    /// Create with configuration
    pub fn with_config(config: SettingsConfig) -> Self {
        let mut panel = Self {
            config,
            categories: Vec::new(),
            current_category: Category::Display,
            values: HashMap::new(),
            has_unsaved_changes: false,
        };
        
        panel.init_categories();
        panel
    }
    
    /// Initialize settings categories
    fn init_categories(&mut self) {
        self.categories = vec![
            self.create_display_category(),
            self.create_sound_category(),
            self.create_network_category(),
            self.create_bluetooth_category(),
            self.create_keyboard_category(),
            self.create_mouse_category(),
            self.create_touchpad_category(),
            self.create_power_category(),
            self.create_storage_category(),
            self.create_appearance_category(),
            self.create_accessibility_category(),
            self.create_privacy_category(),
            self.create_security_category(),
            self.create_system_category(),
            self.create_about_category(),
            self.create_advanced_category(),
        ];
        
        // Initialize values from settings
        for category in &self.categories {
            for setting in &category.settings {
                self.values.insert(setting.key.clone(), setting.value.clone());
            }
        }
    }
    
    /// Create display category
    fn create_display_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Display,
            name: "Display".to_string(),
            icon: "display".to_string(),
            description: "Configure display settings".to_string(),
            settings: vec![
                Setting {
                    key: "display.brightness".to_string(),
                    name: "Brightness".to_string(),
                    description: "Display brightness level".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(80),
                    default: SettingValue::Integer(80),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(0)),
                    max: Some(SettingValue::Integer(100)),
                },
                Setting {
                    key: "display.resolution".to_string(),
                    name: "Resolution".to_string(),
                    description: "Display resolution".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("1920x1080".to_string()),
                    default: SettingValue::String("1920x1080".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("1920x1080".to_string()), name: "1920x1080".to_string(), description: None },
                        SettingOption { value: SettingValue::String("2560x1440".to_string()), name: "2560x1440".to_string(), description: None },
                        SettingOption { value: SettingValue::String("3840x2160".to_string()), name: "3840x2160 (4K)".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "display.refresh_rate".to_string(),
                    name: "Refresh Rate".to_string(),
                    description: "Display refresh rate in Hz".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("60".to_string()),
                    default: SettingValue::String("60".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("60".to_string()), name: "60 Hz".to_string(), description: None },
                        SettingOption { value: SettingValue::String("120".to_string()), name: "120 Hz".to_string(), description: None },
                        SettingOption { value: SettingValue::String("144".to_string()), name: "144 Hz".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "display.scale".to_string(),
                    name: "Scale".to_string(),
                    description: "Display scaling factor".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("1.0".to_string()),
                    default: SettingValue::String("1.0".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("1.0".to_string()), name: "100%".to_string(), description: None },
                        SettingOption { value: SettingValue::String("1.25".to_string()), name: "125%".to_string(), description: None },
                        SettingOption { value: SettingValue::String("1.5".to_string()), name: "150%".to_string(), description: None },
                        SettingOption { value: SettingValue::String("2.0".to_string()), name: "200%".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create sound category
    fn create_sound_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Sound,
            name: "Sound".to_string(),
            icon: "volume-high".to_string(),
            description: "Configure sound settings".to_string(),
            settings: vec![
                Setting {
                    key: "sound.output_volume".to_string(),
                    name: "Output Volume".to_string(),
                    description: "Master output volume".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(75),
                    default: SettingValue::Integer(75),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(0)),
                    max: Some(SettingValue::Integer(100)),
                },
                Setting {
                    key: "sound.input_volume".to_string(),
                    name: "Input Volume".to_string(),
                    description: "Microphone input volume".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(100),
                    default: SettingValue::Integer(100),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(0)),
                    max: Some(SettingValue::Integer(100)),
                },
                Setting {
                    key: "sound.output_device".to_string(),
                    name: "Output Device".to_string(),
                    description: "Primary audio output device".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("default".to_string()),
                    default: SettingValue::String("default".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "sound.mute_notifications".to_string(),
                    name: "Mute Notifications".to_string(),
                    description: "Mute notification sounds".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create network category
    fn create_network_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Network,
            name: "Network".to_string(),
            icon: "wifi".to_string(),
            description: "Configure network settings".to_string(),
            settings: vec![
                Setting {
                    key: "network.auto_connect".to_string(),
                    name: "Auto Connect".to_string(),
                    description: "Automatically connect to known networks".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "network.airplane_mode".to_string(),
                    name: "Airplane Mode".to_string(),
                    description: "Disable all wireless connections".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "network.proxy_enabled".to_string(),
                    name: "Proxy Enabled".to_string(),
                    description: "Enable proxy server".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: true,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "network.proxy_url".to_string(),
                    name: "Proxy URL".to_string(),
                    description: "Proxy server URL".to_string(),
                    setting_type: SettingType::String,
                    value: SettingValue::String(String::new()),
                    default: SettingValue::String(String::new()),
                    advanced: true,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create bluetooth category
    fn create_bluetooth_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Bluetooth,
            name: "Bluetooth".to_string(),
            icon: "bluetooth".to_string(),
            description: "Configure Bluetooth settings".to_string(),
            settings: vec![
                Setting {
                    key: "bluetooth.enabled".to_string(),
                    name: "Bluetooth".to_string(),
                    description: "Enable or disable Bluetooth".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "bluetooth.visible".to_string(),
                    name: "Visible".to_string(),
                    description: "Make device visible to other devices".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create keyboard category
    fn create_keyboard_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Keyboard,
            name: "Keyboard".to_string(),
            icon: "keyboard".to_string(),
            description: "Configure keyboard settings".to_string(),
            settings: vec![
                Setting {
                    key: "keyboard.layout".to_string(),
                    name: "Keyboard Layout".to_string(),
                    description: "Primary keyboard layout".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("us".to_string()),
                    default: SettingValue::String("us".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("us".to_string()), name: "English (US)".to_string(), description: None },
                        SettingOption { value: SettingValue::String("uk".to_string()), name: "English (UK)".to_string(), description: None },
                        SettingOption { value: SettingValue::String("de".to_string()), name: "German".to_string(), description: None },
                        SettingOption { value: SettingValue::String("fr".to_string()), name: "French".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "keyboard.repeat_delay".to_string(),
                    name: "Repeat Delay".to_string(),
                    description: "Delay before key repeat in milliseconds".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(500),
                    default: SettingValue::Integer(500),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(100)),
                    max: Some(SettingValue::Integer(2000)),
                },
                Setting {
                    key: "keyboard.repeat_rate".to_string(),
                    name: "Repeat Rate".to_string(),
                    description: "Key repeat rate in characters per second".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(20),
                    default: SettingValue::Integer(20),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(1)),
                    max: Some(SettingValue::Integer(50)),
                },
            ],
        }
    }
    
    /// Create mouse category
    fn create_mouse_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Mouse,
            name: "Mouse".to_string(),
            icon: "mouse".to_string(),
            description: "Configure mouse settings".to_string(),
            settings: vec![
                Setting {
                    key: "mouse.primary_button".to_string(),
                    name: "Primary Button".to_string(),
                    description: "Primary mouse button".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("left".to_string()),
                    default: SettingValue::String("left".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("left".to_string()), name: "Left".to_string(), description: None },
                        SettingOption { value: SettingValue::String("right".to_string()), name: "Right".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "mouse.acceleration".to_string(),
                    name: "Pointer Acceleration".to_string(),
                    description: "Enable pointer acceleration".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "mouse.sensitivity".to_string(),
                    name: "Pointer Speed".to_string(),
                    description: "Mouse pointer speed".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(50),
                    default: SettingValue::Integer(50),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(1)),
                    max: Some(SettingValue::Integer(100)),
                },
            ],
        }
    }
    
    /// Create touchpad category
    fn create_touchpad_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Touchpad,
            name: "Touchpad".to_string(),
            icon: "touchpad".to_string(),
            description: "Configure touchpad settings".to_string(),
            settings: vec![
                Setting {
                    key: "touchpad.enabled".to_string(),
                    name: "Touchpad".to_string(),
                    description: "Enable or disable touchpad".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "touchpad.natural_scrolling".to_string(),
                    name: "Natural Scrolling".to_string(),
                    description: "Enable natural scrolling direction".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "touchpad.tap_to_click".to_string(),
                    name: "Tap to Click".to_string(),
                    description: "Enable tap to click".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create power category
    fn create_power_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Power,
            name: "Power".to_string(),
            icon: "power".to_string(),
            description: "Configure power management".to_string(),
            settings: vec![
                Setting {
                    key: "power.sleep_after".to_string(),
                    name: "Sleep After".to_string(),
                    description: "Time before system sleep".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("30".to_string()),
                    default: SettingValue::String("30".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("5".to_string()), name: "5 minutes".to_string(), description: None },
                        SettingOption { value: SettingValue::String("10".to_string()), name: "10 minutes".to_string(), description: None },
                        SettingOption { value: SettingValue::String("30".to_string()), name: "30 minutes".to_string(), description: None },
                        SettingOption { value: SettingValue::String("never".to_string()), name: "Never".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "power.battery_saver".to_string(),
                    name: "Battery Saver".to_string(),
                    description: "Enable battery saver mode".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create storage category
    fn create_storage_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Storage,
            name: "Storage".to_string(),
            icon: "hard-drive".to_string(),
            description: "Configure storage settings".to_string(),
            settings: vec![
                Setting {
                    key: "storage.default_download_location".to_string(),
                    name: "Default Download Location".to_string(),
                    description: "Default location for downloaded files".to_string(),
                    setting_type: SettingType::Path,
                    value: SettingValue::Path(PathBuf::from("/home/user/Downloads")),
                    default: SettingValue::Path(PathBuf::from("/home/user/Downloads")),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "storage.auto_cleanup".to_string(),
                    name: "Auto Cleanup".to_string(),
                    description: "Automatically clean temporary files".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: true,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create appearance category
    fn create_appearance_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Appearance,
            name: "Appearance".to_string(),
            icon: "palette".to_string(),
            description: "Configure appearance settings".to_string(),
            settings: vec![
                Setting {
                    key: "appearance.theme".to_string(),
                    name: "Theme".to_string(),
                    description: "System theme".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("dark".to_string()),
                    default: SettingValue::String("dark".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("light".to_string()), name: "Light".to_string(), description: None },
                        SettingOption { value: SettingValue::String("dark".to_string()), name: "Dark".to_string(), description: None },
                        SettingOption { value: SettingValue::String("auto".to_string()), name: "Auto".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "appearance.accent_color".to_string(),
                    name: "Accent Color".to_string(),
                    description: "System accent color".to_string(),
                    setting_type: SettingType::Color,
                    value: SettingValue::Color(Color::rgb(0x569cd6)),
                    default: SettingValue::Color(Color::rgb(0x569cd6)),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "appearance.window_decorations".to_string(),
                    name: "Window Decorations".to_string(),
                    description: "Show window decorations".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create accessibility category
    fn create_accessibility_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Accessibility,
            name: "Accessibility".to_string(),
            icon: "accessibility".to_string(),
            description: "Configure accessibility features".to_string(),
            settings: vec![
                Setting {
                    key: "accessibility.high_contrast".to_string(),
                    name: "High Contrast".to_string(),
                    description: "Enable high contrast mode".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "accessibility.text_scaling".to_string(),
                    name: "Text Scaling".to_string(),
                    description: "Scale text size".to_string(),
                    setting_type: SettingType::Slider,
                    value: SettingValue::Integer(100),
                    default: SettingValue::Integer(100),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: Some(SettingValue::Integer(50)),
                    max: Some(SettingValue::Integer(200)),
                },
                Setting {
                    key: "accessibility.screen_reader".to_string(),
                    name: "Screen Reader".to_string(),
                    description: "Enable screen reader".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create privacy category
    fn create_privacy_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Privacy,
            name: "Privacy".to_string(),
            icon: "lock".to_string(),
            description: "Configure privacy settings".to_string(),
            settings: vec![
                Setting {
                    key: "privacy.send_analytics".to_string(),
                    name: "Send Analytics".to_string(),
                    description: "Send anonymous usage analytics".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "privacy.location_services".to_string(),
                    name: "Location Services".to_string(),
                    description: "Enable location services".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create security category
    fn create_security_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Security,
            name: "Security".to_string(),
            icon: "shield".to_string(),
            description: "Configure security settings".to_string(),
            settings: vec![
                Setting {
                    key: "security.auto_lock".to_string(),
                    name: "Auto Lock".to_string(),
                    description: "Automatically lock screen after inactivity".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "security.lock_after".to_string(),
                    name: "Lock After".to_string(),
                    description: "Time before auto-lock".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("5".to_string()),
                    default: SettingValue::String("5".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("1".to_string()), name: "1 minute".to_string(), description: None },
                        SettingOption { value: SettingValue::String("5".to_string()), name: "5 minutes".to_string(), description: None },
                        SettingOption { value: SettingValue::String("15".to_string()), name: "15 minutes".to_string(), description: None },
                        SettingOption { value: SettingValue::String("30".to_string()), name: "30 minutes".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create system category
    fn create_system_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::System,
            name: "System".to_string(),
            icon: "cog".to_string(),
            description: "Configure system settings".to_string(),
            settings: vec![
                Setting {
                    key: "system.language".to_string(),
                    name: "Language".to_string(),
                    description: "System language".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("en_US".to_string()),
                    default: SettingValue::String("en_US".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("en_US".to_string()), name: "English (US)".to_string(), description: None },
                        SettingOption { value: SettingValue::String("pl_PL".to_string()), name: "Polish".to_string(), description: None },
                        SettingOption { value: SettingValue::String("de_DE".to_string()), name: "German".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "system.timezone".to_string(),
                    name: "Timezone".to_string(),
                    description: "System timezone".to_string(),
                    setting_type: SettingType::Select,
                    value: SettingValue::String("UTC".to_string()),
                    default: SettingValue::String("UTC".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![
                        SettingOption { value: SettingValue::String("UTC".to_string()), name: "UTC".to_string(), description: None },
                        SettingOption { value: SettingValue::String("Europe/Warsaw".to_string()), name: "Warsaw".to_string(), description: None },
                        SettingOption { value: SettingValue::String("America/New_York".to_string()), name: "New York".to_string(), description: None },
                    ],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "system.auto_update".to_string(),
                    name: "Auto Update".to_string(),
                    description: "Automatically install system updates".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(true),
                    default: SettingValue::Boolean(true),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create about category
    fn create_about_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::About,
            name: "About".to_string(),
            icon: "info".to_string(),
            description: "System information".to_string(),
            settings: vec![
                Setting {
                    key: "about.name".to_string(),
                    name: "System Name".to_string(),
                    description: "VantisOS".to_string(),
                    setting_type: SettingType::String,
                    value: SettingValue::String("VantisOS".to_string()),
                    default: SettingValue::String("VantisOS".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "about.version".to_string(),
                    name: "Version".to_string(),
                    description: "System version".to_string(),
                    setting_type: SettingType::String,
                    value: SettingValue::String("1.1.0".to_string()),
                    default: SettingValue::String("1.1.0".to_string()),
                    advanced: false,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Create advanced category
    fn create_advanced_category(&self) -> SettingsCategory {
        SettingsCategory {
            category: Category::Advanced,
            name: "Advanced".to_string(),
            icon: "settings".to_string(),
            description: "Advanced system settings".to_string(),
            settings: vec![
                Setting {
                    key: "advanced.debug_mode".to_string(),
                    name: "Debug Mode".to_string(),
                    description: "Enable debug logging".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: true,
                    requires_restart: false,
                    options: vec![],
                    min: None,
                    max: None,
                },
                Setting {
                    key: "advanced.developer_mode".to_string(),
                    name: "Developer Mode".to_string(),
                    description: "Enable developer features".to_string(),
                    setting_type: SettingType::Boolean,
                    value: SettingValue::Boolean(false),
                    default: SettingValue::Boolean(false),
                    advanced: true,
                    requires_restart: true,
                    options: vec![],
                    min: None,
                    max: None,
                },
            ],
        }
    }
    
    /// Get all categories
    pub fn categories(&self) -> &[SettingsCategory] {
        &self.categories
    }
    
    /// Get current category
    pub fn current_category(&self) -> Category {
        self.current_category
    }
    
    /// Switch to a category
    pub fn switch_category(&mut self, category: Category) {
        self.current_category = category;
    }
    
    /// Get settings for current category
    pub fn current_settings(&self) -> Vec<&Setting> {
        if let Some(category) = self.categories.iter().find(|c| c.category == self.current_category) {
            category.settings.iter().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get a setting by key
    pub fn get_setting(&self, key: &str) -> Option<&Setting> {
        for category in &self.categories {
            if let Some(setting) = category.settings.iter().find(|s| s.key == key) {
                return Some(setting);
            }
        }
        None
    }
    
    /// Get a setting by key (mutable)
    pub fn get_setting_mut(&mut self, key: &str) -> Option<&mut Setting> {
        for category in &mut self.categories {
            if let Some(setting) = category.settings.iter_mut().find(|s| s.key == key) {
                return Some(setting);
            }
        }
        None
    }
    
    /// Set a setting value
    pub fn set_value(&mut self, key: &str, value: SettingValue) -> Result<(), SettingsError> {
        let setting = self.get_setting_mut(key)
            .ok_or_else(|| SettingsError::SettingNotFound(key.to_string()))?;
        
        // Validate type
        if std::mem::discriminant(&value) != std::mem::discriminant(&setting.value) {
            return Err(SettingsError::InvalidType(format!("Invalid type for setting {}", key)));
        }
        
        // Validate range
        if let Some(min) = &setting.min {
            // TODO: Add range validation
        }
        
        setting.value = value.clone();
        self.values.insert(key.to_string(), value);
        self.has_unsaved_changes = true;
        
        Ok(())
    }
    
    /// Reset a setting to default
    pub fn reset_to_default(&mut self, key: &str) -> Result<(), SettingsError> {
        let setting = self.get_setting_mut(key)
            .ok_or_else(|| SettingsError::SettingNotFound(key.to_string()))?;
        
        setting.value = setting.default.clone();
        self.values.insert(key.to_string(), setting.default.clone());
        self.has_unsaved_changes = true;
        
        Ok(())
    }
    
    /// Reset all settings to default
    pub fn reset_all_to_default(&mut self) {
        for category in &mut self.categories {
            for setting in &mut category.settings {
                setting.value = setting.default.clone();
                self.values.insert(setting.key.clone(), setting.default.clone());
            }
        }
        self.has_unsaved_changes = true;
    }
    
    /// Save all settings
    pub fn save(&mut self) -> Result<(), SettingsError> {
        // In a real implementation, this would persist settings to disk
        self.has_unsaved_changes = false;
        Ok(())
    }
    
    /// Discard unsaved changes
    pub fn discard_changes(&mut self) {
        for category in &mut self.categories {
            for setting in &mut category.settings {
                if let Some(value) = self.values.get(&setting.key) {
                    setting.value = value.clone();
                }
            }
        }
        self.has_unsaved_changes = false;
    }
    
    /// Check if there are unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.has_unsaved_changes
    }
    
    /// Get configuration
    pub fn config(&self) -> &SettingsConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: SettingsConfig) {
        self.config = config;
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}