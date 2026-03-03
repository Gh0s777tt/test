//! VantisOS Applications Module
//! 
//! System applications for VantisOS including:
//! - File Manager
//! - Terminal Emulator
//! - Text Editor
//! - System Monitor
//! - Settings Panel

pub mod file_manager;
pub mod terminal_emulator;
pub mod text_editor;
pub mod system_monitor;
pub mod settings_panel;

pub use file_manager::{FileManager, FileManagerError, ViewMode, SortOrder, FileInfo, FileType, Bookmark, Clipboard, ClipboardOperation, ArchiveType, NetworkLocation, NetworkLocationType};
pub use terminal_emulator::{TerminalEmulator, TerminalConfig, TerminalTab, TerminalSession, TerminalError, TerminalFont, TerminalColors, ShellConfig, KeyboardShortcuts, TerminalBehavior, CursorStyle, BellBehavior, Cursor, TerminalBuffer, TerminalLine, TextSegment, SearchResult};
pub use text_editor::{TextEditor, TextEditorConfig, Document, TextEditorError, EditorFont, EditorTheme, EditorBehavior, Keybindings, AutosaveConfig, CursorStyle as EditorCursorStyle, Position, Selection, EditAction, EditActionType, Language, LineEnding, ScrollPosition, FindState, SearchResult as EditorSearchResult, CursorDirection};
pub use system_monitor::{SystemMonitor, MonitorConfig, MonitorError, CpuMonitor, MemoryMonitor, DiskMonitor, NetworkMonitor, ProcessManager, CpuCore, CpuFrequency, Temperature, CpuInfo, Disk, IoStats, NetworkInterface, Process, ProcessState, ProcessSort, ProcessSignal, TemperatureUnit, NetworkUnit};
pub use settings_panel::{SettingsPanel, SettingsConfig, SettingsError, SettingsCategory, Category, Setting, SettingType, SettingValue, SettingOption, Color};
