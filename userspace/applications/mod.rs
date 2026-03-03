//! VantisOS Applications Module
//! 
//! System applications for VantisOS including:
//! - File Manager
//! - Terminal Emulator
//! - Text Editor
//! - System Monitor
//! - Settings Panel

pub mod file_manager;

pub use file_manager::{FileManager, FileManagerError, ViewMode, SortOrder, FileInfo, FileType, Bookmark, Clipboard, ClipboardOperation, ArchiveType, NetworkLocation, NetworkLocationType};
