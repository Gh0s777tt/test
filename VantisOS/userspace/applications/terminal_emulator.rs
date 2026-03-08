//! VantisOS Terminal Emulator
//! 
//! A modern terminal emulator with tabs, profiles, and customization support.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Terminal emulator configuration
#[derive(Clone, Debug)]
pub struct TerminalConfig {
    /// Font configuration
    pub font: TerminalFont,
    /// Color scheme
    pub colors: TerminalColors,
    /// Shell to launch
    pub shell: ShellConfig,
    /// Keyboard shortcuts
    pub shortcuts: KeyboardShortcuts,
    /// Behavior settings
    pub behavior: TerminalBehavior,
}

/// Terminal font configuration
#[derive(Clone, Debug)]
pub struct TerminalFont {
    /// Font family
    pub family: String,
    /// Font size
    pub size: u32,
    /// Line height multiplier
    pub line_height: f32,
    /// Character width
    pub char_width: Option<f32>,
}

impl Default for TerminalFont {
    fn default() -> Self {
        Self {
            family: "monospace".to_string(),
            size: 14,
            line_height: 1.2,
            char_width: None,
        }
    }
}

/// Terminal color scheme
#[derive(Clone, Debug)]
pub struct TerminalColors {
    /// Background color
    pub background: Color,
    /// Foreground color
    pub foreground: Color,
    /// Cursor color
    pub cursor: Color,
    /// Cursor accent (text under cursor)
    pub cursor_accent: Color,
    /// Selection background
    pub selection: Color,
    /// ANSI color palette (16 colors)
    pub palette: [Color; 16],
}

impl Default for TerminalColors {
    fn default() -> Self {
        Self {
            background: Color::rgb(0x1e1e1e),
            foreground: Color::rgb(0xd4d4d4),
            cursor: Color::rgb(0x569cd6),
            cursor_accent: Color::rgb(0xffffff),
            selection: Color::rgba(0x569cd6, 100),
            palette: [
                Color::rgb(0x000000), // Black
                Color::rgb(0xcd3131), // Red
                Color::rgb(0x0dbc79), // Green
                Color::rgb(0xe5e510), // Yellow
                Color::rgb(0x2472c8), // Blue
                Color::rgb(0xbc3fbc), // Magenta
                Color::rgb(0x11a8cd), // Cyan
                Color::rgb(0xe5e5e5), // White
                Color::rgb(0x666666), // Bright Black
                Color::rgb(0xf14c4c), // Bright Red
                Color::rgb(0x23d18b), // Bright Green
                Color::rgb(0xf5f543), // Bright Yellow
                Color::rgb(0x3b8eea), // Bright Blue
                Color::rgb(0xd670d6), // Bright Magenta
                Color::rgb(0x29b8db), // Bright Cyan
                Color::rgb(0xffffff), // Bright White
            ],
        }
    }
}

/// Shell configuration
#[derive(Clone, Debug)]
pub struct ShellConfig {
    /// Shell executable
    pub executable: String,
    /// Shell arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Working directory
    pub working_dir: PathBuf,
}

impl Default for ShellConfig {
    fn default() -> Self {
        let mut env = HashMap::new();
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert("LANG".to_string(), "en_US.UTF-8".to_string());
        
        Self {
            executable: "/bin/bash".to_string(),
            args: vec!["--login".to_string()],
            env,
            working_dir: PathBuf::from("/home/user"),
        }
    }
}

/// Keyboard shortcuts
#[derive(Clone, Debug)]
pub struct KeyboardShortcuts {
    pub copy: String,
    pub paste: String,
    pub new_tab: String,
    pub close_tab: String,
    pub next_tab: String,
    pub previous_tab: String,
    pub clear: String,
    pub find: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self {
            copy: "Ctrl+Shift+C".to_string(),
            paste: "Ctrl+Shift+V".to_string(),
            new_tab: "Ctrl+Shift+T".to_string(),
            close_tab: "Ctrl+Shift+W".to_string(),
            next_tab: "Ctrl+Tab".to_string(),
            previous_tab: "Ctrl+Shift+Tab".to_string(),
            clear: "Ctrl+L".to_string(),
            find: "Ctrl+Shift+F".to_string(),
            zoom_in: "Ctrl++".to_string(),
            zoom_out: "Ctrl+-".to_string(),
            zoom_reset: "Ctrl+0".to_string(),
        }
    }
}

/// Terminal behavior settings
#[derive(Clone, Debug)]
pub struct TerminalBehavior {
    /// Scrollback buffer size (lines)
    pub scrollback_size: usize,
    /// Auto-scroll on output
    pub auto_scroll: bool,
    /// Blink cursor
    pub blink_cursor: bool,
    /// Cursor style
    pub cursor_style: CursorStyle,
    /// Bell sound
    pub bell: BellBehavior,
    /// Text wrapping
    pub word_wrap: bool,
    /// Allow bold text
    pub allow_bold: bool,
    /// Confirm close with running processes
    pub confirm_close: bool,
}

/// Cursor style
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Block
    }
}

/// Bell behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BellBehavior {
    None,
    Visual,
    Audible,
    Both,
}

impl Default for TerminalBehavior {
    fn default() -> Self {
        Self {
            scrollback_size: 10000,
            auto_scroll: true,
            blink_cursor: true,
            cursor_style: CursorStyle::Block,
            bell: BellBehavior::Both,
            word_wrap: true,
            allow_bold: true,
            confirm_close: true,
        }
    }
}

/// Terminal tab
#[derive(Clone, Debug)]
pub struct TerminalTab {
    /// Tab ID
    pub id: String,
    /// Tab title
    pub title: String,
    /// Terminal session
    pub session: Arc<Mutex<TerminalSession>>,
    /// Is modified
    pub modified: bool,
}

/// Terminal session
#[derive(Debug)]
pub struct TerminalSession {
    /// Terminal buffer
    pub buffer: TerminalBuffer,
    /// Cursor position
    pub cursor: Cursor,
    /// Process ID
    pub pid: Option<u32>,
    /// Is running
    pub running: bool,
    /// Exit code
    pub exit_code: Option<i32>,
}

/// Terminal buffer
#[derive(Clone, Debug)]
pub struct TerminalBuffer {
    /// Lines in the buffer
    pub lines: VecDeque<TerminalLine>,
    /// Maximum scrollback size
    pub scrollback_size: usize,
    /// Current scroll position (0 = bottom)
    pub scroll_position: usize,
    /// Current line
    pub current_line: TerminalLine,
}

/// Terminal line
#[derive(Clone, Debug)]
pub struct TerminalLine {
    /// Segments in the line (with different colors/styles)
    pub segments: Vec<TextSegment>,
    /// Is wrapped from previous line
    pub wrapped: bool,
}

impl Default for TerminalLine {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            wrapped: false,
        }
    }
}

/// Text segment with styling
#[derive(Clone, Debug)]
pub struct TextSegment {
    /// Text content
    pub text: String,
    /// Foreground color
    pub foreground: Color,
    /// Background color
    pub background: Color,
    /// Is bold
    pub bold: bool,
    /// Is italic
    pub italic: bool,
    /// Is underlined
    pub underlined: bool,
}

impl TextSegment {
    pub fn new(text: &str, foreground: Color, background: Color) -> Self {
        Self {
            text: text.to_string(),
            foreground,
            background,
            bold: false,
            italic: false,
            underlined: false,
        }
    }
    
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    
    pub fn underlined(mut self) -> Self {
        self.underlined = true;
        self
    }
}

/// Cursor
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    /// Line position
    pub line: usize,
    /// Column position
    pub column: usize,
    /// Is visible
    pub visible: bool,
    /// Cursor style
    pub style: CursorStyle,
    /// Last blink toggle
    pub last_blink: Option<Instant>,
    /// Blink state
    pub blink_state: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            visible: true,
            style: CursorStyle::Block,
            last_blink: None,
            blink_state: true,
        }
    }
}

impl Cursor {
    pub fn toggle_blink(&mut self) {
        if let Some(last) = self.last_blink {
            if last.elapsed() >= Duration::from_millis(500) {
                self.blink_state = !self.blink_state;
                self.last_blink = Some(Instant::now());
            }
        } else {
            self.last_blink = Some(Instant::now());
        }
    }
}

/// Terminal emulator
pub struct TerminalEmulator {
    /// Configuration
    config: TerminalConfig,
    /// Terminal tabs
    tabs: Vec<TerminalTab>,
    /// Active tab index
    active_tab: usize,
    /// Profiles
    profiles: HashMap<String, TerminalConfig>,
    /// Active profile
    active_profile: String,
    /// Search query
    search_query: Option<String>,
    /// Search results
    search_results: Vec<SearchResult>,
    /// Current search index
    search_index: usize,
}

/// Search result
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// Tab ID
    pub tab_id: String,
    /// Line number
    pub line: usize,
    /// Column number
    pub column: usize,
    /// Matched text
    pub text: String,
}

/// Terminal errors
#[derive(Debug, Clone)]
pub enum TerminalError {
    /// Tab not found
    TabNotFound,
    /// Process failed to start
    ProcessStartFailed(String),
    /// Process already running
    ProcessAlreadyRunning,
    /// Invalid configuration
    InvalidConfig(String),
    /// Profile not found
    ProfileNotFound(String),
    /// Shell not found
    ShellNotFound(String),
    /// Write failed
    WriteFailed(String),
}

impl std::fmt::Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TerminalError::TabNotFound => write!(f, "Tab not found"),
            TerminalError::ProcessStartFailed(msg) => write!(f, "Process start failed: {}", msg),
            TerminalError::ProcessAlreadyRunning => write!(f, "Process already running"),
            TerminalError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            TerminalError::ProfileNotFound(name) => write!(f, "Profile not found: {}", name),
            TerminalError::ShellNotFound(path) => write!(f, "Shell not found: {}", path),
            TerminalError::WriteFailed(msg) => write!(f, "Write failed: {}", msg),
        }
    }
}

impl std::error::Error for TerminalError {}

impl TerminalEmulator {
    /// Create a new terminal emulator
    pub fn new() -> Self {
        let config = TerminalConfig::default();
        let mut profiles = HashMap::new();
        profiles.insert("Default".to_string(), config.clone());
        
        Self {
            config,
            tabs: Vec::new(),
            active_tab: 0,
            profiles,
            active_profile: "Default".to_string(),
            search_query: None,
            search_results: Vec::new(),
            search_index: 0,
        }
    }
    
    /// Create terminal emulator with custom config
    pub fn with_config(config: TerminalConfig) -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("Default".to_string(), config.clone());
        
        Self {
            config,
            tabs: Vec::new(),
            active_tab: 0,
            profiles,
            active_profile: "Default".to_string(),
            search_query: None,
            search_results: Vec::new(),
            search_index: 0,
        }
    }
    
    /// Add a profile
    pub fn add_profile(&mut self, name: String, config: TerminalConfig) {
        self.profiles.insert(name, config);
    }
    
    /// Switch to a profile
    pub fn switch_profile(&mut self, name: &str) -> Result<(), TerminalError> {
        if let Some(config) = self.profiles.get(name).cloned() {
            self.config = config;
            self.active_profile = name.to_string();
            Ok(())
        } else {
            Err(TerminalError::ProfileNotFound(name.to_string()))
        }
    }
    
    /// Create a new tab
    pub fn new_tab(&mut self) -> Result<(), TerminalError> {
        let id = format!("tab-{}", self.tabs.len());
        let title = format!("Tab {}", self.tabs.len() + 1);
        
        let session = TerminalSession {
            buffer: TerminalBuffer {
                lines: VecDeque::with_capacity(self.config.behavior.scrollback_size + 1),
                scrollback_size: self.config.behavior.scrollback_size,
                scroll_position: 0,
                current_line: TerminalLine::default(),
            },
            cursor: Cursor {
                style: self.config.behavior.cursor_style,
                ..Default::default()
            },
            pid: None,
            running: false,
            exit_code: None,
        };
        
        let tab = TerminalTab {
            id: id.clone(),
            title,
            session: Arc::new(Mutex::new(session)),
            modified: false,
        };
        
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        
        Ok(())
    }
    
    /// Close a tab
    pub fn close_tab(&mut self, index: usize) -> Result<(), TerminalError> {
        if index >= self.tabs.len() {
            return Err(TerminalError::TabNotFound);
        }
        
        let session = self.tabs[index].session.lock().unwrap();
        if session.running && self.config.behavior.confirm_close {
            return Err(TerminalError::ProcessAlreadyRunning);
        }
        drop(session);
        
        self.tabs.remove(index);
        
        if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_tab = self.tabs.len() - 1;
        }
        
        Ok(())
    }
    
    /// Switch to a tab
    pub fn switch_tab(&mut self, index: usize) -> Result<(), TerminalError> {
        if index >= self.tabs.len() {
            return Err(TerminalError::TabNotFound);
        }
        
        self.active_tab = index;
        Ok(())
    }
    
    /// Get active tab
    pub fn active_tab(&self) -> Option<&TerminalTab> {
        self.tabs.get(self.active_tab)
    }
    
    /// Get all tabs
    pub fn tabs(&self) -> &[TerminalTab] {
        &self.tabs
    }
    
    /// Start shell in active tab
    pub fn start_shell(&mut self) -> Result<(), TerminalError> {
        let tab = self.tabs.get(self.active_tab).ok_or(TerminalError::TabNotFound)?;
        let mut session = tab.session.lock().unwrap();
        
        if session.running {
            return Err(TerminalError::ProcessAlreadyRunning);
        }
        
        // In a real implementation, this would spawn the shell process
        session.running = true;
        session.pid = Some(1); // Placeholder PID
        
        Ok(())
    }
    
    /// Write text to terminal
    pub fn write(&self, text: &str) -> Result<(), TerminalError> {
        let tab = self.tabs.get(self.active_tab).ok_or(TerminalError::TabNotFound)?;
        let mut session = tab.session.lock().unwrap();
        
        let colors = &self.config.colors;
        let segment = TextSegment::new(text, colors.foreground, colors.background);
        session.buffer.current_line.segments.push(segment);
        
        Ok(())
    }
    
    /// Write styled text to terminal
    pub fn write_styled(&self, text: &str, foreground: Color, background: Color, bold: bool) -> Result<(), TerminalError> {
        let tab = self.tabs.get(self.active_tab).ok_or(TerminalError::TabNotFound)?;
        let mut session = tab.session.lock().unwrap();
        
        let mut segment = TextSegment::new(text, foreground, background);
        segment.bold = bold;
        session.buffer.current_line.segments.push(segment);
        
        Ok(())
    }
    
    /// New line
    pub fn newline(&self) -> Result<(), TerminalError> {
        let tab = self.tabs.get(self.active_tab).ok_or(TerminalError::TabNotFound)?;
        let mut session = tab.session.lock().unwrap();
        
        // Move current line to buffer
        session.buffer.lines.push_back(session.buffer.current_line.clone());
        session.buffer.current_line = TerminalLine::default();
        
        // Trim scrollback if needed
        while session.buffer.lines.len() > session.buffer.scrollback_size {
            session.buffer.lines.pop_front();
        }
        
        Ok(())
    }
    
    /// Clear buffer
    pub fn clear(&self) -> Result<(), TerminalError> {
        let tab = self.tabs.get(self.active_tab).ok_or(TerminalError::TabNotFound)?;
        let mut session = tab.session.lock().unwrap();
        
        session.buffer.lines.clear();
        session.buffer.current_line = TerminalLine::default();
        session.cursor.line = 0;
        session.cursor.column = 0;
        
        Ok(())
    }
    
    /// Search in all tabs
    pub fn search(&mut self, query: &str) {
        self.search_query = Some(query.to_string());
        self.search_results.clear();
        self.search_index = 0;
        
        for tab in &self.tabs {
            let session = tab.session.lock().unwrap();
            
            for (line_idx, line) in session.buffer.lines.iter().enumerate() {
                for segment in &line.segments {
                    if let Some(pos) = segment.text.find(query) {
                        self.search_results.push(SearchResult {
                            tab_id: tab.id.clone(),
                            line: line_idx,
                            column: pos,
                            text: segment.text.clone(),
                        });
                    }
                }
            }
        }
    }
    
    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query = None;
        self.search_results.clear();
        self.search_index = 0;
    }
    
    /// Get search results
    pub fn search_results(&self) -> &[SearchResult] {
        &self.search_results
    }
    
    /// Get config
    pub fn config(&self) -> &TerminalConfig {
        &self.config
    }
    
    /// Set config
    pub fn set_config(&mut self, config: TerminalConfig) {
        self.config = config;
    }
    
    /// Get profiles
    pub fn profiles(&self) -> &HashMap<String, TerminalConfig> {
        &self.profiles
    }
    
    /// Get active profile name
    pub fn active_profile(&self) -> &str {
        &self.active_profile
    }
}

impl Default for TerminalEmulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Color representation (placeholder - would use Flux graphics Color)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }
    
    pub fn rgba(hex: u32, a: u8) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a,
        }
    }
}