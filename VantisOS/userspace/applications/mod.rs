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
pub mod calculator;
pub mod calendar;
pub mod browser;
pub mod image_viewer;
pub mod video_player;

pub use file_manager::{FileManager, FileManagerError, ViewMode, SortOrder, FileInfo, FileType, Bookmark, Clipboard, ClipboardOperation, ArchiveType, NetworkLocation, NetworkLocationType};
pub use terminal_emulator::{TerminalEmulator, TerminalConfig, TerminalTab, TerminalSession, TerminalError, TerminalFont, TerminalColors, ShellConfig, KeyboardShortcuts, TerminalBehavior, CursorStyle, BellBehavior, Cursor, TerminalBuffer, TerminalLine, TextSegment, SearchResult};
pub use text_editor::{TextEditor, TextEditorConfig, Document, TextEditorError, EditorFont, EditorTheme, EditorBehavior, Keybindings, AutosaveConfig, CursorStyle as EditorCursorStyle, Position, Selection, EditAction, EditActionType, Language, LineEnding, ScrollPosition, FindState, SearchResult as EditorSearchResult, CursorDirection};
pub use system_monitor::{SystemMonitor, MonitorConfig, MonitorError, CpuMonitor, MemoryMonitor, DiskMonitor, NetworkMonitor, ProcessManager, CpuCore, CpuFrequency, Temperature, CpuInfo, Disk, IoStats, NetworkInterface, Process, ProcessState, ProcessSort, ProcessSignal, TemperatureUnit, NetworkUnit};
pub use settings_panel::{SettingsPanel, SettingsConfig, SettingsError, SettingsCategory, Category, Setting, SettingType, SettingValue, SettingOption, Color};
pub use calculator::{Calculator, Operation, HistoryEntry};
pub use calendar::{Calendar, Event, EventCategory, Priority, Reminder, ReminderMethod, Recurrence, RecurrencePattern, ViewType, MonthView, DayInfo, SharedCalendar, generate_event_id};
pub use browser::{Browser, Tab, Bookmark as BrowserBookmark, HistoryEntry as BrowserHistoryEntry, ViewType as BrowserViewType};
pub use image_viewer::{ImageViewer, ImageInfo, ImageFormat, ViewMode as ImageViewMode, SlideshowSettings};
pub use video_player::{VideoPlayer, VideoInfo, VideoMetadata, VideoFormat as VideoFormatEnum, PlaybackState, RepeatMode, VideoTrack, SubtitleTrack};
