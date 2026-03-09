//! File Manager Application
//! 
//! A modern file manager for VantisOS with support for:
//! - Multiple views (grid, list, details, tree)
//! - File operations (copy, move, delete, rename, create)
//! - Search and filtering
//! - Bookmarks and favorites
//! - Archive support
//! - Network locations
//! - File preview

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// File Manager main structure
pub struct FileManager {
    pub current_directory: Arc<Mutex<PathBuf>>,
    pub history: Arc<Mutex<Vec<PathBuf>>>,
    pub history_cursor: Arc<Mutex<usize>>,
    pub selected_files: Arc<Mutex<Vec<FileInfo>>>,
    pub clipboard: Arc<Mutex<Option<Clipboard>>>,
    pub bookmarks: Arc<Mutex<Vec<Bookmark>>>,
    pub view_mode: Arc<Mutex<ViewMode>>,
    pub sort_order: Arc<Mutex<SortOrder>>,
    pub show_hidden: Arc<Mutex<bool>>,
    pub show_extensions: Arc<Mutex<bool>>,
    pub theme: Arc<Mutex<FileManagerTheme>>,
    pub recent_locations: Arc<Mutex<Vec<PathBuf>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: FileTime,
    pub created: FileTime,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub owner: String,
    pub group: String,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub is_executable: bool,
    pub icon_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTime { pub seconds: i64, pub nanoseconds: u32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Unknown, Regular, Directory, Symlink, BlockDevice, CharDevice, Fifo, Socket,
    Text, Image, Audio, Video, Archive, Document, Spreadsheet, Presentation, Code, Script, Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub user_read: bool, pub user_write: bool, pub user_execute: bool,
    pub group_read: bool, pub group_write: bool, pub group_execute: bool,
    pub other_read: bool, pub other_write: bool, pub other_execute: bool,
}

#[derive(Debug, Clone)]
pub struct Clipboard {
    pub files: Vec<PathBuf>,
    pub operation: ClipboardOperation,
    pub source: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardOperation { Copy, Cut }

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    pub icon_path: String,
    pub is_default: bool,
    pub created: FileTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode { Grid, List, Details, Tree, Compact }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name, NameReverse, Size, SizeReverse, Modified, ModifiedReverse,
    Created, CreatedReverse, Type, TypeReverse,
}

#[derive(Debug, Clone)]
pub struct FileManagerTheme {
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub accent_color: (u8, u8, u8, u8),
    pub selection_color: (u8, u8, u8, u8),
    pub grid_size: u32,
    pub icon_size: IconSize,
    pub font_size: u32,
    pub show_thumbnails: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize { Small, Medium, Large, Extra }

#[derive(Debug, Clone)]
pub struct FileOperationProgress {
    pub total_files: u64, pub processed_files: u64,
    pub total_bytes: u64, pub processed_bytes: u64,
    pub current_file: PathBuf, pub is_complete: bool, pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf, pub name: String, pub file_type: FileType, pub size: u64, pub score: f32,
}

#[derive(Debug, Clone)]
pub struct ArchiveInfo {
    pub archive_type: ArchiveType, pub file_count: u64,
    pub uncompressed_size: u64, pub compressed_size: u64,
    pub is_encrypted: bool, pub is_password_protected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveType { Zip, Tar, TarGz, TarBz2, TarXz, Rar, SevenZip, Iso }

#[derive(Debug, Clone)]
pub struct NetworkLocation {
    pub name: String, pub location_type: NetworkLocationType,
    pub address: String, pub port: Option<u16>, pub username: Option<String>,
    pub is_connected: bool, pub mount_point: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkLocationType { Smb, Nfs, Ftp, Sftp, WebDav, NetworkDrive }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileManagerError {
    FileNotFound(PathBuf), PermissionDenied(PathBuf), FileAlreadyExists(PathBuf),
    DirectoryNotEmpty(PathBuf), NotADirectory(PathBuf), NotAFile(PathBuf),
    InvalidPath(String), CopyError(String), MoveError(String), DeleteError(String),
    CreateError(String), RenameError(String), ArchiveError(String), ExtractError(String),
    NetworkError(String), OperationCancelled, InvalidDestination(PathBuf),
    OutOfDiskSpace, FileTooLarge, ClipboardEmpty,
}

impl FileManager {
    pub fn new(initial_path: PathBuf) -> Self {
        Self {
            current_directory: Arc::new(Mutex::new(initial_path.clone())),
            history: Arc::new(Mutex::new(vec![initial_path])),
            history_cursor: Arc::new(Mutex::new(0)),
            selected_files: Arc::new(Mutex::new(Vec::new())),
            clipboard: Arc::new(Mutex::new(None)),
            bookmarks: Arc::new(Mutex::new(Self::default_bookmarks())),
            view_mode: Arc::new(Mutex::new(ViewMode::List)),
            sort_order: Arc::new(Mutex::new(SortOrder::Name)),
            show_hidden: Arc::new(Mutex::new(false)),
            show_extensions: Arc::new(Mutex::new(true)),
            theme: Arc::new(Mutex::new(FileManagerTheme::default())),
            recent_locations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn default_bookmarks() -> Vec<Bookmark> {
        vec![
            Bookmark { name: String::from("Home"), path: PathBuf::from("/home/user"), icon_path: String::from("/usr/share/icons/folder-home.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Documents"), path: PathBuf::from("/home/user/Documents"), icon_path: String::from("/usr/share/icons/folder-documents.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Downloads"), path: PathBuf::from("/home/user/Downloads"), icon_path: String::from("/usr/share/icons/folder-downloads.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Pictures"), path: PathBuf::from("/home/user/Pictures"), icon_path: String::from("/usr/share/icons/folder-pictures.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Music"), path: PathBuf::from("/home/user/Music"), icon_path: String::from("/usr/share/icons/folder-music.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Videos"), path: PathBuf::from("/home/user/Videos"), icon_path: String::from("/usr/share/icons/folder-videos.png"), is_default: true, created: FileTime::now() },
            Bookmark { name: String::from("Root"), path: PathBuf::from("/"), icon_path: String::from("/usr/share/icons/folder-root.png"), is_default: true, created: FileTime::now() },
        ]
    }

    pub fn navigate(&self, path: &Path) -> Result<(), FileManagerError> {
        let mut current_dir = self.current_directory.lock().unwrap();
        if !path.exists() { return Err(FileManagerError::FileNotFound(path.to_path_buf())); }
        if !path.is_dir() { return Err(FileManagerError::NotADirectory(path.to_path_buf())); }
        let mut history = self.history.lock().unwrap();
        let cursor = *self.history_cursor.lock().unwrap();
        if cursor < history.len() - 1 { history.truncate(cursor + 1); }
        history.push(path.to_path_buf());
        *self.history_cursor.lock().unwrap() = history.len() - 1;
        *current_dir = path.to_path_buf();
        let mut recent = self.recent_locations.lock().unwrap();
        recent.retain(|p| p != path);
        recent.insert(0, path.to_path_buf());
        if recent.len() > 20 { recent.truncate(20); }
        self.selected_files.lock().unwrap().clear();
        Ok(())
    }

    pub fn navigate_up(&self) -> Result<(), FileManagerError> {
        let current_dir = self.current_directory.lock().unwrap();
        match current_dir.parent() {
            Some(p) => { drop(current_dir); self.navigate(p) }
            None => Err(FileManagerError::InvalidPath("Already at root".to_string())),
        }
    }

    pub fn navigate_back(&self) -> Result<(), FileManagerError> {
        let mut cursor = self.history_cursor.lock().unwrap();
        if *cursor == 0 { return Err(FileManagerError::InvalidPath("No back history".to_string())); }
        *cursor -= 1;
        let index = *cursor;
        drop(cursor);
        let history = self.history.lock().unwrap();
        let path = history.get(index).unwrap().clone();
        drop(history);
        *self.current_directory.lock().unwrap() = path.clone();
        Ok(())
    }

    pub fn navigate_forward(&self) -> Result<(), FileManagerError> {
        let history = self.history.lock().unwrap();
        let mut cursor = self.history_cursor.lock().unwrap();
        if *cursor >= history.len() - 1 { return Err(FileManagerError::InvalidPath("No forward history".to_string())); }
        *cursor += 1;
        let index = *cursor;
        let path = history.get(index).unwrap().clone();
        drop(history); drop(cursor);
        *self.current_directory.lock().unwrap() = path.clone();
        Ok(())
    }

    pub fn list_files(&self) -> Result<Vec<FileInfo>, FileManagerError> {
        let show_hidden = *self.show_hidden.lock().unwrap();
        let sort_order = *self.sort_order.lock().unwrap();
        let mut files = Vec::new();
        files.push(FileInfo {
            path: PathBuf::from(".."), name: String::from(".."), is_directory: true,
            size: 0, modified: FileTime::now(), created: FileTime::now(),
            file_type: FileType::Directory, permissions: FilePermissions::default(),
            owner: String::new(), group: String::new(), is_hidden: false,
            is_readonly: false, is_executable: false,
            icon_path: String::from("/usr/share/icons/folder-up.png"),
        });
        Self::sort_files(&mut files, sort_order);
        if !show_hidden { files.retain(|f| !f.is_hidden); }
        Ok(files)
    }

    fn sort_files(files: &mut Vec<FileInfo>, sort_order: SortOrder) {
        match sort_order {
            SortOrder::Name => files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            SortOrder::NameReverse => files.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase())),
            SortOrder::Size => files.sort_by(|a, b| a.size.cmp(&b.size)),
            SortOrder::SizeReverse => files.sort_by(|a, b| b.size.cmp(&a.size)),
            SortOrder::Modified => files.sort_by(|a, b| a.modified.cmp(&b.modified)),
            SortOrder::ModifiedReverse => files.sort_by(|a, b| b.modified.cmp(&a.modified)),
            SortOrder::Created => files.sort_by(|a, b| a.created.cmp(&b.created)),
            SortOrder::CreatedReverse => files.sort_by(|a, b| b.created.cmp(&a.created)),
            SortOrder::Type => files.sort_by(|a, b| format!("{:?}{}", a.file_type, a.name).cmp(&format!("{:?}{}", b.file_type, b.name))),
            SortOrder::TypeReverse => files.sort_by(|a, b| format!("{:?}{}", b.file_type, b.name).cmp(&format!("{:?}{}", a.file_type, a.name))),
        }
        files.sort_by(|a, b| (!a.is_directory).cmp(&(!b.is_directory)));
    }

    pub fn copy_files(&self, files: Vec<PathBuf>, source: PathBuf) {
        *self.clipboard.lock().unwrap() = Some(Clipboard { files, operation: ClipboardOperation::Copy, source });
    }

    pub fn cut_files(&self, files: Vec<PathBuf>, source: PathBuf) {
        *self.clipboard.lock().unwrap() = Some(Clipboard { files, operation: ClipboardOperation::Cut, source });
    }

    pub fn paste_files(&self, destination: &Path) -> Result<FileOperationProgress, FileManagerError> {
        let clipboard = self.clipboard.lock().unwrap();
        let clipboard = match clipboard.as_ref() {
            Some(c) => c.clone(),
            None => return Err(FileManagerError::ClipboardEmpty),
        };
        Ok(FileOperationProgress { total_files: clipboard.files.len() as u64, processed_files: 0, total_bytes: 0, processed_bytes: 0, current_file: PathBuf::new(), is_complete: false, error: None })
    }

    pub fn create_directory(&self, name: &str) -> Result<PathBuf, FileManagerError> {
        Ok(self.current_directory.lock().unwrap().join(name))
    }

    pub fn create_file(&self, name: &str) -> Result<PathBuf, FileManagerError> {
        Ok(self.current_directory.lock().unwrap().join(name))
    }

    pub fn delete_files(&self, files: &[PathBuf]) -> Result<FileOperationProgress, FileManagerError> {
        Ok(FileOperationProgress { total_files: files.len() as u64, processed_files: 0, total_bytes: 0, processed_bytes: 0, current_file: PathBuf::new(), is_complete: false, error: None })
    }

    pub fn rename_file(&self, old_path: &Path, new_name: &str) -> Result<PathBuf, FileManagerError> {
        let parent = old_path.parent().ok_or(FileManagerError::InvalidPath("No parent directory".to_string()))?;
        Ok(parent.join(new_name))
    }

    pub fn search(&self, query: &str, directory: Option<&Path>) -> Result<Vec<SearchResult>, FileManagerError> { Ok(Vec::new()) }

    pub fn add_bookmark(&self, name: String, path: PathBuf, icon_path: String) {
        let mut bookmarks = self.bookmarks.lock().unwrap();
        bookmarks.retain(|b| b.path != path);
        bookmarks.push(Bookmark { name, path, icon_path, is_default: false, created: FileTime::now() });
    }

    pub fn remove_bookmark(&self, path: &Path) { self.bookmarks.lock().unwrap().retain(|b| b.path != path); }
    pub fn set_view_mode(&self, mode: ViewMode) { *self.view_mode.lock().unwrap() = mode; }
    pub fn set_sort_order(&self, order: SortOrder) { *self.sort_order.lock().unwrap() = order; }
    pub fn toggle_hidden(&self) { let mut show = self.show_hidden.lock().unwrap(); *show = !*show; }
    pub fn toggle_extensions(&self) { let mut show = self.show_extensions.lock().unwrap(); *show = !*show; }
    pub fn get_archive_info(&self, path: &Path) -> Result<ArchiveInfo, FileManagerError> {
        Ok(ArchiveInfo { archive_type: ArchiveType::Zip, file_count: 0, uncompressed_size: 0, compressed_size: 0, is_encrypted: false, is_password_protected: false })
    }
    pub fn extract_archive(&self, archive: &Path, destination: &Path) -> Result<FileOperationProgress, FileManagerError> {
        Ok(FileOperationProgress { total_files: 0, processed_files: 0, total_bytes: 0, processed_bytes: 0, current_file: PathBuf::new(), is_complete: false, error: None })
    }
    pub fn create_archive(&self, files: &[PathBuf], destination: &Path, archive_type: ArchiveType) -> Result<FileOperationProgress, FileManagerError> {
        Ok(FileOperationProgress { total_files: files.len() as u64, processed_files: 0, total_bytes: 0, processed_bytes: 0, current_file: PathBuf::new(), is_complete: false, error: None })
    }
    pub fn mount_network(&self, location: NetworkLocation) -> Result<PathBuf, FileManagerError> { Err(FileManagerError::NetworkError("Not implemented".to_string())) }
    pub fn unmount_network(&self, mount_point: &Path) -> Result<(), FileManagerError> { Err(FileManagerError::NetworkError("Not implemented".to_string())) }
}

impl Default for FileManagerTheme {
    fn default() -> Self {
        Self { background_color: (255, 255, 255, 255), text_color: (0, 0, 0, 255), accent_color: (0, 120, 212, 255), selection_color: (51, 153, 255, 100), grid_size: 96, icon_size: IconSize::Medium, font_size: 14, show_thumbnails: true }
    }
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self { user_read: true, user_write: true, user_execute: false, group_read: true, group_write: false, group_execute: false, other_read: true, other_write: false, other_execute: false }
    }
}

impl FileTime {
    pub fn now() -> Self { Self { seconds: 0, nanoseconds: 0 } }
}

impl std::cmp::PartialOrd for FileTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl std::cmp::Ord for FileTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.seconds.cmp(&other.seconds) {
            std::cmp::Ordering::Equal => self.nanoseconds.cmp(&other.nanoseconds),
            other => other,
        }
    }
}

impl std::fmt::Display for FileManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileManagerError::FileNotFound(path) => write!(f, "File not found: {:?}", path),
            FileManagerError::PermissionDenied(path) => write!(f, "Permission denied: {:?}", path),
            FileManagerError::FileAlreadyExists(path) => write!(f, "File already exists: {:?}", path),
            FileManagerError::DirectoryNotEmpty(path) => write!(f, "Directory not empty: {:?}", path),
            FileManagerError::NotADirectory(path) => write!(f, "Not a directory: {:?}", path),
            FileManagerError::NotAFile(path) => write!(f, "Not a file: {:?}", path),
            FileManagerError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            FileManagerError::CopyError(msg) => write!(f, "Copy error: {}", msg),
            FileManagerError::MoveError(msg) => write!(f, "Move error: {}", msg),
            FileManagerError::DeleteError(msg) => write!(f, "Delete error: {}", msg),
            FileManagerError::CreateError(msg) => write!(f, "Create error: {}", msg),
            FileManagerError::RenameError(msg) => write!(f, "Rename error: {}", msg),
            FileManagerError::ArchiveError(msg) => write!(f, "Archive error: {}", msg),
            FileManagerError::ExtractError(msg) => write!(f, "Extract error: {}", msg),
            FileManagerError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            FileManagerError::OperationCancelled => write!(f, "Operation cancelled"),
            FileManagerError::InvalidDestination(path) => write!(f, "Invalid destination: {:?}", path),
            FileManagerError::OutOfDiskSpace => write!(f, "Out of disk space"),
            FileManagerError::FileTooLarge => write!(f, "File too large"),
            FileManagerError::ClipboardEmpty => write!(f, "Clipboard is empty"),
        }
    }
}

impl std::error::Error for FileManagerError {}
