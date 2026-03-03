//! VantisOS Text Editor
//! 
//! A code-focused text editor with syntax highlighting and customization support.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Text editor configuration
#[derive(Clone, Debug)]
pub struct TextEditorConfig {
    /// Font configuration
    pub font: EditorFont,
    /// Theme
    pub theme: EditorTheme,
    /// Editor behavior
    pub behavior: EditorBehavior,
    /// Keybindings
    pub keybindings: Keybindings,
    /// Auto-save settings
    pub autosave: AutosaveConfig,
}

/// Editor font configuration
#[derive(Clone, Debug)]
pub struct EditorFont {
    /// Font family
    pub family: String,
    /// Font size
    pub size: u32,
    /// Line height
    pub line_height: f32,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Use tabs instead of spaces
    pub use_tabs: bool,
    /// Font weight (100-900)
    pub weight: u32,
}

impl Default for EditorFont {
    fn default() -> Self {
        Self {
            family: "monospace".to_string(),
            size: 14,
            line_height: 1.5,
            tab_size: 4,
            use_tabs: false,
            weight: 400,
        }
    }
}

/// Editor theme
#[derive(Clone, Debug)]
pub struct EditorTheme {
    /// Theme name
    pub name: String,
    /// Background color
    pub background: Color,
    /// Foreground color
    pub foreground: Color,
    /// Selection color
    pub selection: Color,
    /// Line highlight color
    pub line_highlight: Color,
    /// Cursor color
    pub cursor: Color,
    /// Syntax colors
    pub syntax: SyntaxColors,
    /// Gutter colors
    pub gutter: GutterColors,
    /// Minimap colors
    pub minimap: MinimapColors,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self {
            name: "Dark+".to_string(),
            background: Color::rgb(0x1e1e1e),
            foreground: Color::rgb(0xd4d4d4),
            selection: Color::rgba(0x264f78, 128),
            line_highlight: Color::rgba(0xffffff, 10),
            cursor: Color::rgb(0xaeafad),
            syntax: SyntaxColors::default(),
            gutter: GutterColors::default(),
            minimap: MinimapColors::default(),
        }
    }
}

/// Syntax highlighting colors
#[derive(Clone, Debug)]
pub struct SyntaxColors {
    pub comment: Color,
    pub keyword: Color,
    pub string: Color,
    pub number: Color,
    pub operator: Color,
    pub function: Color,
    pub variable: Color,
    pub r#type: Color,
    pub constant: Color,
    pub punctuation: Color,
    pub tag: Color,
    pub attribute: Color,
}

impl Default for SyntaxColors {
    fn default() -> Self {
        Self {
            comment: Color::rgb(0x6a9955),
            keyword: Color::rgb(0x569cd6),
            string: Color::rgb(0xce9178),
            number: Color::rgb(0xb5cea8),
            operator: Color::rgb(0xd4d4d4),
            function: Color::rgb(0xdcdcaa),
            variable: Color::rgb(0x9cdcfe),
            r#type: Color::rgb(0x4ec9b0),
            constant: Color::rgb(0x4fc1ff),
            punctuation: Color::rgb(0xd4d4d4),
            tag: Color::rgb(0x569cd6),
            attribute: Color::rgb(0x9cdcfe),
        }
    }
}

/// Gutter colors
#[derive(Clone, Debug)]
pub struct GutterColors {
    pub background: Color,
    pub foreground: Color,
    pub active_line: Color,
}

impl Default for GutterColors {
    fn default() -> Self {
        Self {
            background: Color::rgb(0x1e1e1e),
            foreground: Color::rgb(0x858585),
            active_line: Color::rgb(0xc6c6c6),
        }
    }
}

/// Minimap colors
#[derive(Clone, Debug)]
pub struct MinimapColors {
    pub background: Color,
    pub foreground: Color,
    pub selection: Color,
}

impl Default for MinimapColors {
    fn default() -> Self {
        Self {
            background: Color::rgb(0x1e1e1e),
            foreground: Color::rgba(0xd4d4d4, 100),
            selection: Color::rgba(0x264f78, 128),
        }
    }
}

/// Editor behavior settings
#[derive(Clone, Debug)]
pub struct EditorBehavior {
    /// Show line numbers
    pub line_numbers: bool,
    /// Show minimap
    pub minimap: bool,
    /// Word wrap
    pub word_wrap: bool,
    /// Auto indent
    pub auto_indent: bool,
    /// Auto close brackets
    pub auto_close_brackets: bool,
    /// Auto close tags
    pub auto_close_tags: bool,
    /// Highlight matching brackets
    pub highlight_matching_brackets: bool,
    /// Highlight current line
    pub highlight_current_line: bool,
    /// Show indent guides
    pub indent_guides: bool,
    /// Enable code folding
    pub code_folding: bool,
    /// Enable autocomplete
    pub autocomplete: bool,
    /// Autocomplete delay in ms
    pub autocomplete_delay: u64,
    /// Enable snippets
    pub snippets: bool,
    /// Cursor style
    pub cursor_style: CursorStyle,
    /// Cursor blink rate in ms
    pub cursor_blink_rate: u64,
}

impl Default for EditorBehavior {
    fn default() -> Self {
        Self {
            line_numbers: true,
            minimap: true,
            word_wrap: false,
            auto_indent: true,
            auto_close_brackets: true,
            auto_close_tags: true,
            highlight_matching_brackets: true,
            highlight_current_line: true,
            indent_guides: true,
            code_folding: true,
            autocomplete: true,
            autocomplete_delay: 300,
            snippets: true,
            cursor_style: CursorStyle::Bar,
            cursor_blink_rate: 530,
        }
    }
}

/// Cursor style
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Underline,
    Bar,
}

/// Keybindings
#[derive(Clone, Debug)]
pub struct Keybindings {
    pub save: String,
    pub save_as: String,
    pub open: String,
    pub new: String,
    pub close: String,
    pub undo: String,
    pub redo: String,
    pub cut: String,
    pub copy: String,
    pub paste: String,
    pub select_all: String,
    pub find: String,
    pub replace: String,
    pub goto_line: String,
    pub toggle_comment: String,
    pub indent: String,
    pub outdent: String,
    pub duplicate_line: String,
    pub delete_line: String,
    pub move_line_up: String,
    pub move_line_down: String,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            save: "Ctrl+S".to_string(),
            save_as: "Ctrl+Shift+S".to_string(),
            open: "Ctrl+O".to_string(),
            new: "Ctrl+N".to_string(),
            close: "Ctrl+W".to_string(),
            undo: "Ctrl+Z".to_string(),
            redo: "Ctrl+Shift+Z".to_string(),
            cut: "Ctrl+X".to_string(),
            copy: "Ctrl+C".to_string(),
            paste: "Ctrl+V".to_string(),
            select_all: "Ctrl+A".to_string(),
            find: "Ctrl+F".to_string(),
            replace: "Ctrl+H".to_string(),
            goto_line: "Ctrl+G".to_string(),
            toggle_comment: "Ctrl+/".to_string(),
            indent: "Tab".to_string(),
            outdent: "Shift+Tab".to_string(),
            duplicate_line: "Ctrl+Shift+D".to_string(),
            delete_line: "Ctrl+Shift+K".to_string(),
            move_line_up: "Alt+Up".to_string(),
            move_line_down: "Alt+Down".to_string(),
        }
    }
}

/// Auto-save configuration
#[derive(Clone, Debug)]
pub struct AutosaveConfig {
    /// Enable auto-save
    pub enabled: bool,
    /// Auto-save delay in ms
    pub delay: u64,
}

impl Default for AutosaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            delay: 1000,
        }
    }
}

/// Editor document
#[derive(Clone, Debug)]
pub struct Document {
    /// File path
    pub path: Option<PathBuf>,
    /// File name
    pub name: String,
    /// Content
    pub content: String,
    /// Lines
    pub lines: Vec<String>,
    /// Is modified
    pub modified: bool,
    /// Language
    pub language: Language,
    /// Encoding
    pub encoding: String,
    /// Line ending
    pub line_ending: LineEnding,
    /// Undo stack
    pub undo_stack: VecDeque<EditAction>,
    /// Redo stack
    pub redo_stack: VecDeque<EditAction>,
    /// Last saved content hash
    pub saved_hash: Option<u64>,
}

impl Document {
    pub fn new(name: &str) -> Self {
        Self {
            path: None,
            name: name.to_string(),
            content: String::new(),
            lines: vec![String::new()],
            modified: false,
            language: Language::PlainText,
            encoding: "UTF-8".to_string(),
            line_ending: LineEnding::Lf,
            undo_stack: VecDeque::with_capacity(100),
            redo_stack: VecDeque::with_capacity(100),
            saved_hash: None,
        }
    }
    
    pub fn with_content(name: &str, content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let lines = if lines.is_empty() { vec![String::new()] } else { lines };
        
        Self {
            path: None,
            name: name.to_string(),
            content: content.to_string(),
            lines,
            modified: false,
            language: Language::PlainText,
            encoding: "UTF-8".to_string(),
            line_ending: LineEnding::Lf,
            undo_stack: VecDeque::with_capacity(100),
            redo_stack: VecDeque::with_capacity(100),
            saved_hash: None,
        }
    }
    
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
    
    pub fn line_length(&self, index: usize) -> usize {
        self.lines.get(index).map(|l| l.len()).unwrap_or(0)
    }
    
    pub fn insert_char(&mut self, position: Position, c: char) {
        if position.line < self.lines.len() {
            self.lines[position.line].insert(position.column, c);
            self.update_content();
            self.modified = true;
        }
    }
    
    pub fn insert_text(&mut self, position: Position, text: &str) {
        if position.line >= self.lines.len() {
            return;
        }
        
        let mut current_line = position.line;
        let mut current_col = position.column;
        
        for c in text.chars() {
            if c == '\n' {
                let rest: String = self.lines[current_line].drain(current_col..).collect();
                self.lines[current_line] = rest.clone();
                self.lines.insert(current_line, rest);
                current_line += 1;
                current_col = 0;
            } else {
                self.lines[current_line].insert(current_col, c);
                current_col += 1;
            }
        }
        
        self.update_content();
        self.modified = true;
    }
    
    pub fn delete_range(&mut self, start: Position, end: Position) {
        if start.line >= self.lines.len() || end.line >= self.lines.len() {
            return;
        }
        
        if start.line == end.line {
            self.lines[start.line].drain(start.column..end.column);
        } else {
            // Remove lines in between
            for _ in (start.line + 1)..end.line {
                self.lines.remove(start.line + 1);
            }
            
            // Merge first and last line
            let end_line = self.lines.remove(start.line + 1);
            self.lines[start.line].truncate(start.column);
            if let Some(end) = end_line {
                self.lines[start.line].push_str(&end[end.column..]);
            }
        }
        
        self.update_content();
        self.modified = true;
    }
    
    pub fn delete_line(&mut self, line: usize) {
        if line < self.lines.len() {
            self.lines.remove(line);
            if self.lines.is_empty() {
                self.lines.push(String::new());
            }
            self.update_content();
            self.modified = true;
        }
    }
    
    fn update_content(&mut self) {
        self.content = self.lines.join("\n");
    }
    
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    pub fn undo(&mut self) -> Option<EditAction> {
        self.undo_stack.pop_back()
    }
    
    pub fn redo(&mut self) -> Option<EditAction> {
        self.redo_stack.pop_back()
    }
}

/// Editor position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Selection
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub start: Position,
    pub end: Position,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    
    pub fn is_multiline(&self) -> bool {
        self.start.line != self.end.line
    }
    
    pub fn normalized(&self) -> (Position, Position) {
        if self.start.line < self.end.line 
           || (self.start.line == self.end.line && self.start.column <= self.end.column) {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }
}

/// Edit action
#[derive(Clone, Debug)]
pub struct EditAction {
    /// Action type
    pub action_type: EditActionType,
    /// Position
    pub position: Position,
    /// Text inserted
    pub text: String,
    /// Text deleted
    pub deleted: String,
}

/// Edit action type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditActionType {
    Insert,
    Delete,
    Replace,
}

/// Programming language
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    PlainText,
    Rust,
    JavaScript,
    TypeScript,
    Python,
    C,
    Cpp,
    Java,
    Go,
    Ruby,
    PHP,
    Swift,
    Kotlin,
    Html,
    Css,
    Json,
    Yaml,
    Toml,
    Markdown,
    Shell,
    Sql,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "js" => Self::JavaScript,
            "ts" => Self::TypeScript,
            "py" => Self::Python,
            "c" => Self::C,
            "cpp" | "cc" | "cxx" => Self::Cpp,
            "java" => Self::Java,
            "go" => Self::Go,
            "rb" => Self::Ruby,
            "php" => Self::PHP,
            "swift" => Self::Swift,
            "kt" => Self::Kotlin,
            "html" | "htm" => Self::Html,
            "css" => Self::Css,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "md" => Self::Markdown,
            "sh" | "bash" => Self::Shell,
            "sql" => Self::Sql,
            _ => Self::PlainText,
        }
    }
}

/// Line ending type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineEnding {
    Lf,       // Unix
    Crlf,     // Windows
    Cr,       // Classic Mac
}

/// Text editor
pub struct TextEditor {
    /// Configuration
    config: TextEditorConfig,
    /// Open documents
    documents: Vec<Document>,
    /// Active document index
    active_document: usize,
    /// Cursor position
    cursor: Position,
    /// Selection
    selection: Selection,
    /// Scroll position
    scroll: ScrollPosition,
    /// Find state
    find_state: Option<FindState>,
    /// Auto-save timer
    last_edit: Option<Instant>,
}

/// Scroll position
#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollPosition {
    pub line: usize,
    pub column: usize,
    pub x_offset: f32,
    pub y_offset: f32,
}

/// Find state
#[derive(Clone, Debug)]
pub struct FindState {
    /// Search query
    pub query: String,
    /// Replace text
    pub replace: Option<String>,
    /// Case sensitive
    pub case_sensitive: bool,
    /// Whole word
    pub whole_word: bool,
    /// Use regex
    pub use_regex: bool,
    /// Search results
    pub results: Vec<SearchResult>,
    /// Current result index
    pub current_index: usize,
}

/// Search result
#[derive(Clone, Copy, Debug)]
pub struct SearchResult {
    pub start: Position,
    pub end: Position,
}

/// Text editor errors
#[derive(Debug, Clone)]
pub enum TextEditorError {
    /// Document not found
    DocumentNotFound,
    /// File read error
    FileReadError(String),
    /// File write error
    FileWriteError(String),
    /// Invalid position
    InvalidPosition,
    /// Encoding error
    EncodingError(String),
    /// Undo stack empty
    UndoStackEmpty,
    /// Redo stack empty
    RedoStackEmpty,
    /// Search pattern error
    SearchPatternError(String),
}

impl std::fmt::Display for TextEditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TextEditorError::DocumentNotFound => write!(f, "Document not found"),
            TextEditorError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            TextEditorError::FileWriteError(msg) => write!(f, "File write error: {}", msg),
            TextEditorError::InvalidPosition => write!(f, "Invalid position"),
            TextEditorError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            TextEditorError::UndoStackEmpty => write!(f, "Undo stack empty"),
            TextEditorError::RedoStackEmpty => write!(f, "Redo stack empty"),
            TextEditorError::SearchPatternError(msg) => write!(f, "Search pattern error: {}", msg),
        }
    }
}

impl std::error::Error for TextEditorError {}

impl TextEditor {
    /// Create a new text editor
    pub fn new() -> Self {
        Self {
            config: TextEditorConfig::default(),
            documents: vec![Document::new("Untitled")],
            active_document: 0,
            cursor: Position::default(),
            selection: Selection::default(),
            scroll: ScrollPosition::default(),
            find_state: None,
            last_edit: None,
        }
    }
    
    /// Create editor with configuration
    pub fn with_config(config: TextEditorConfig) -> Self {
        Self {
            config,
            documents: vec![Document::new("Untitled")],
            active_document: 0,
            cursor: Position::default(),
            selection: Selection::default(),
            scroll: ScrollPosition::default(),
            find_state: None,
            last_edit: None,
        }
    }
    
    /// Get active document
    pub fn active_document(&self) -> Option<&Document> {
        self.documents.get(self.active_document)
    }
    
    /// Get active document mut
    pub fn active_document_mut(&mut self) -> Option<&mut Document> {
        self.documents.get_mut(self.active_document)
    }
    
    /// Get all documents
    pub fn documents(&self) -> &[Document] {
        &self.documents
    }
    
    /// New document
    pub fn new_document(&mut self) {
        let doc = Document::new(&format!("Untitled {}", self.documents.len() + 1));
        self.documents.push(doc);
        self.active_document = self.documents.len() - 1;
        self.cursor = Position::default();
        self.selection = Selection::default();
    }
    
    /// Open document
    pub fn open_document(&mut self, path: PathBuf) -> Result<(), TextEditorError> {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let content = std::fs::read_to_string(&path)
            .map_err(|e| TextEditorError::FileReadError(e.to_string()))?;
        
        let mut doc = Document::with_content(&name, &content);
        doc.path = Some(path);
        doc.language = Language::from_extension(extension);
        
        self.documents.push(doc);
        self.active_document = self.documents.len() - 1;
        self.cursor = Position::default();
        self.selection = Selection::default();
        
        Ok(())
    }
    
    /// Save document
    pub fn save_document(&mut self, index: usize) -> Result<(), TextEditorError> {
        let doc = self.documents.get_mut(index).ok_or(TextEditorError::DocumentNotFound)?;
        
        if let Some(path) = &doc.path {
            std::fs::write(path, &doc.content)
                .map_err(|e| TextEditorError::FileWriteError(e.to_string()))?;
            doc.modified = false;
            Ok(())
        } else {
            Err(TextEditorError::FileWriteError("No file path set".to_string()))
        }
    }
    
    /// Save document as
    pub fn save_document_as(&mut self, index: usize, path: PathBuf) -> Result<(), TextEditorError> {
        let doc = self.documents.get_mut(index).ok_or(TextEditorError::DocumentNotFound)?;
        
        std::fs::write(&path, &doc.content)
            .map_err(|e| TextEditorError::FileWriteError(e.to_string()))?;
        
        doc.path = Some(path.clone());
        doc.name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        doc.modified = false;
        
        Ok(())
    }
    
    /// Close document
    pub fn close_document(&mut self, index: usize) -> Result<bool, TextEditorError> {
        if index >= self.documents.len() {
            return Err(TextEditorError::DocumentNotFound);
        }
        
        let doc = &self.documents[index];
        let modified = doc.modified;
        
        self.documents.remove(index);
        
        if self.documents.is_empty() {
            self.documents.push(Document::new("Untitled"));
        }
        
        if self.active_document >= self.documents.len() {
            self.active_document = self.documents.len() - 1;
        }
        
        Ok(modified)
    }
    
    /// Switch to document
    pub fn switch_document(&mut self, index: usize) -> Result<(), TextEditorError> {
        if index >= self.documents.len() {
            return Err(TextEditorError::DocumentNotFound);
        }
        
        self.active_document = index;
        self.cursor = Position::default();
        self.selection = Selection::default();
        
        Ok(())
    }
    
    /// Insert character
    pub fn insert_char(&mut self, c: char) {
        if let Some(doc) = self.active_document_mut() {
            // Delete selection if any
            if !self.selection.is_empty() {
                let (start, end) = self.selection.normalized();
                doc.delete_range(start, end);
                self.cursor = start;
                self.selection = Selection::default();
            }
            
            doc.insert_char(self.cursor, c);
            self.cursor.column += 1;
            self.last_edit = Some(Instant::now());
        }
    }
    
    /// Insert newline
    pub fn insert_newline(&mut self) {
        if let Some(doc) = self.active_document_mut() {
            if !self.selection.is_empty() {
                let (start, _) = self.selection.normalized();
                doc.delete_range(start, start);
                self.cursor = start;
                self.selection = Selection::default();
            }
            
            let current_line = doc.lines.get(self.cursor.line).cloned().unwrap_or_default();
            let indent: String = current_line.chars().take_while(|c| c.is_whitespace()).collect();
            
            // Split current line
            let rest: String = current_line.chars().skip(self.cursor.column).collect();
            let mut new_line = indent.clone();
            new_line.push_str(&rest);
            
            // Update lines
            doc.lines[self.cursor.line].truncate(self.cursor.column);
            doc.lines.insert(self.cursor.line + 1, new_line);
            
            // Auto-indent
            if self.config.behavior.auto_indent {
                self.cursor.column = indent.len();
            } else {
                self.cursor.column = 0;
            }
            self.cursor.line += 1;
            
            doc.update_content();
            doc.modified = true;
            self.last_edit = Some(Instant::now());
        }
    }
    
    /// Delete character before cursor
    pub fn backspace(&mut self) {
        if let Some(doc) = self.active_document_mut() {
            if !self.selection.is_empty() {
                let (start, end) = self.selection.normalized();
                doc.delete_range(start, end);
                self.cursor = start;
                self.selection = Selection::default();
            } else if self.cursor.column > 0 {
                self.cursor.column -= 1;
                doc.lines[self.cursor.line].remove(self.cursor.column);
                doc.update_content();
                doc.modified = true;
            } else if self.cursor.line > 0 {
                // Join with previous line
                let prev_len = doc.lines[self.cursor.line - 1].len();
                let current_line = doc.lines.remove(self.cursor.line);
                doc.lines[self.cursor.line - 1].push_str(&current_line);
                self.cursor.line -= 1;
                self.cursor.column = prev_len;
                doc.update_content();
                doc.modified = true;
            }
            self.last_edit = Some(Instant::now());
        }
    }
    
    /// Delete character after cursor
    pub fn delete(&mut self) {
        if let Some(doc) = self.active_document_mut() {
            if !self.selection.is_empty() {
                let (start, end) = self.selection.normalized();
                doc.delete_range(start, end);
                self.cursor = start;
                self.selection = Selection::default();
            } else if self.cursor.column < doc.line_length(self.cursor.line) {
                doc.lines[self.cursor.line].remove(self.cursor.column);
                doc.update_content();
                doc.modified = true;
            } else if self.cursor.line < doc.lines.len() - 1 {
                // Join with next line
                let next_line = doc.lines.remove(self.cursor.line + 1);
                doc.lines[self.cursor.line].push_str(&next_line);
                doc.update_content();
                doc.modified = true;
            }
            self.last_edit = Some(Instant::now());
        }
    }
    
    /// Move cursor
    pub fn move_cursor(&mut self, direction: CursorDirection, extend_selection: bool) {
        if let Some(doc) = self.active_document() {
            let old_cursor = self.cursor;
            
            match direction {
                CursorDirection::Left => {
                    if self.cursor.column > 0 {
                        self.cursor.column -= 1;
                    } else if self.cursor.line > 0 {
                        self.cursor.line -= 1;
                        self.cursor.column = doc.line_length(self.cursor.line);
                    }
                }
                CursorDirection::Right => {
                    if self.cursor.column < doc.line_length(self.cursor.line) {
                        self.cursor.column += 1;
                    } else if self.cursor.line < doc.lines.len() - 1 {
                        self.cursor.line += 1;
                        self.cursor.column = 0;
                    }
                }
                CursorDirection::Up => {
                    if self.cursor.line > 0 {
                        self.cursor.line -= 1;
                        self.cursor.column = self.cursor.column.min(doc.line_length(self.cursor.line));
                    }
                }
                CursorDirection::Down => {
                    if self.cursor.line < doc.lines.len() - 1 {
                        self.cursor.line += 1;
                        self.cursor.column = self.cursor.column.min(doc.line_length(self.cursor.line));
                    }
                }
                CursorDirection::Home => {
                    self.cursor.column = 0;
                }
                CursorDirection::End => {
                    self.cursor.column = doc.line_length(self.cursor.line);
                }
                CursorDirection::PageUp => {
                    let page_size = 20; // TODO: Calculate based on view height
                    self.cursor.line = self.cursor.line.saturating_sub(page_size);
                    self.cursor.column = self.cursor.column.min(doc.line_length(self.cursor.line));
                }
                CursorDirection::PageDown => {
                    let page_size = 20;
                    self.cursor.line = (self.cursor.line + page_size).min(doc.lines.len() - 1);
                    self.cursor.column = self.cursor.column.min(doc.line_length(self.cursor.line));
                }
                CursorDirection::DocumentStart => {
                    self.cursor.line = 0;
                    self.cursor.column = 0;
                }
                CursorDirection::DocumentEnd => {
                    self.cursor.line = doc.lines.len() - 1;
                    self.cursor.column = doc.line_length(self.cursor.line);
                }
            }
            
            if extend_selection {
                if self.selection.is_empty() {
                    self.selection.start = old_cursor;
                }
                self.selection.end = self.cursor;
            } else {
                self.selection = Selection::default();
            }
        }
    }
    
    /// Select all
    pub fn select_all(&mut self) {
        if let Some(doc) = self.active_document() {
            self.selection.start = Position { line: 0, column: 0 };
            self.selection.end = Position {
                line: doc.lines.len() - 1,
                column: doc.line_length(doc.lines.len() - 1),
            };
            self.cursor = self.selection.end;
        }
    }
    
    /// Copy selection
    pub fn copy(&self) -> Option<String> {
        if self.selection.is_empty() {
            return None;
        }
        
        let doc = self.active_document()?;
        let (start, end) = self.selection.normalized();
        
        if start.line == end.line {
            doc.lines.get(start.line)
                .map(|line| line[start.column..end.column].to_string())
        } else {
            let mut result = String::new();
            for i in start.line..=end.line {
                if i == start.line {
                    result.push_str(&doc.lines[i][start.column..]);
                } else if i == end.line {
                    result.push('\n');
                    result.push_str(&doc.lines[i][..end.column]);
                } else {
                    result.push('\n');
                    result.push_str(&doc.lines[i]);
                }
            }
            Some(result)
        }
    }
    
    /// Cut selection
    pub fn cut(&mut self) -> Option<String> {
        let text = self.copy()?;
        if !self.selection.is_empty() {
            let (start, _) = self.selection.normalized();
            if let Some(doc) = self.active_document_mut() {
                doc.delete_range(start, start);
                self.cursor = start;
                self.selection = Selection::default();
            }
        }
        Some(text)
    }
    
    /// Paste text
    pub fn paste(&mut self, text: &str) {
        if let Some(doc) = self.active_document_mut() {
            if !self.selection.is_empty() {
                let (start, end) = self.selection.normalized();
                doc.delete_range(start, end);
                self.cursor = start;
                self.selection = Selection::default();
            }
            doc.insert_text(self.cursor, text);
            
            // Update cursor position
            for c in text.chars() {
                if c == '\n' {
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                } else {
                    self.cursor.column += 1;
                }
            }
            
            self.last_edit = Some(Instant::now());
        }
    }
    
    /// Undo
    pub fn undo(&mut self) -> Result<(), TextEditorError> {
        if let Some(doc) = self.active_document_mut() {
            if let Some(action) = doc.undo() {
                // Apply inverse action
                match action.action_type {
                    EditActionType::Insert => {
                        // Delete the inserted text
                        doc.delete_range(action.position, Position {
                            line: action.position.line,
                            column: action.position.column + action.text.len(),
                        });
                    }
                    EditActionType::Delete => {
                        // Re-insert the deleted text
                        doc.insert_text(action.position, &action.deleted);
                    }
                    EditActionType::Replace => {
                        doc.insert_text(action.position, &action.deleted);
                    }
                }
                self.cursor = action.position;
                return Ok(());
            }
        }
        Err(TextEditorError::UndoStackEmpty)
    }
    
    /// Redo
    pub fn redo(&mut self) -> Result<(), TextEditorError> {
        if let Some(doc) = self.active_document_mut() {
            if let Some(action) = doc.redo() {
                // Re-apply the action
                match action.action_type {
                    EditActionType::Insert => {
                        doc.insert_text(action.position, &action.text);
                    }
                    EditActionType::Delete => {
                        doc.delete_range(action.position, Position {
                            line: action.position.line,
                            column: action.position.column + action.text.len(),
                        });
                    }
                    EditActionType::Replace => {
                        doc.insert_text(action.position, &action.text);
                    }
                }
                self.cursor = action.position;
                return Ok(());
            }
        }
        Err(TextEditorError::RedoStackEmpty)
    }
    
    /// Find text
    pub fn find(&mut self, query: &str, case_sensitive: bool, whole_word: bool) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        if let Some(doc) = self.active_document() {
            for (line_idx, line) in doc.lines.iter().enumerate() {
                let search_line = if case_sensitive {
                    line.clone()
                } else {
                    line.to_lowercase()
                };
                let search_query = if case_sensitive {
                    query.to_string()
                } else {
                    query.to_lowercase()
                };
                
                let mut col = 0;
                while let Some(pos) = search_line[col..].find(&search_query) {
                    let start_col = col + pos;
                    let end_col = start_col + query.len();
                    
                    // Check whole word
                    let valid = if whole_word {
                        let before_ok = start_col == 0 || 
                            !line.chars().nth(start_col - 1).map(|c| c.is_alphanumeric()).unwrap_or(false);
                        let after_ok = end_col >= line.len() ||
                            !line.chars().nth(end_col).map(|c| c.is_alphanumeric()).unwrap_or(false);
                        before_ok && after_ok
                    } else {
                        true
                    };
                    
                    if valid {
                        results.push(SearchResult {
                            start: Position { line: line_idx, column: start_col },
                            end: Position { line: line_idx, column: end_col },
                        });
                    }
                    col = end_col;
                }
            }
        }
        
        self.find_state = Some(FindState {
            query: query.to_string(),
            replace: None,
            case_sensitive,
            whole_word,
            use_regex: false,
            results: results.clone(),
            current_index: 0,
        });
        
        results
    }
    
    /// Find next
    pub fn find_next(&mut self) -> Option<SearchResult> {
        if let Some(state) = &self.find_state {
            if !state.results.is_empty() {
                let index = state.current_index;
                let result = state.results.get(index).copied();
                
                if let Some(result) = result {
                    self.cursor = result.start;
                    self.selection.start = result.start;
                    self.selection.end = result.end;
                }
                
                // Advance to next result
                if let Some(state) = &mut self.find_state {
                    state.current_index = (state.current_index + 1) % state.results.len();
                }
                
                return result;
            }
        }
        None
    }
    
    /// Get cursor position
    pub fn cursor(&self) -> Position {
        self.cursor
    }
    
    /// Get selection
    pub fn selection(&self) -> Selection {
        self.selection
    }
    
    /// Get scroll position
    pub fn scroll(&self) -> ScrollPosition {
        self.scroll
    }
    
    /// Set scroll position
    pub fn set_scroll(&mut self, scroll: ScrollPosition) {
        self.scroll = scroll;
    }
    
    /// Get configuration
    pub fn config(&self) -> &TextEditorConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: TextEditorConfig) {
        self.config = config;
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Cursor movement direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    DocumentStart,
    DocumentEnd,
}

/// Color representation
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