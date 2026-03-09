//! # Web Browser Application
//!
//! A lightweight web browser application for VantisOS with support for basic
//! web browsing, bookmarks, history, and tab management.

use serde::{Deserialize, Serialize};

/// Browser tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub url: Option<String>,
    pub title: String,
    pub loading: bool,
    pub history: Vec<String>,
    pub history_index: usize,
}

impl Tab {
    /// Create a new tab
    pub fn new() -> Self {
        Tab {
            id: Self::generate_id(),
            url: None,
            title: "New Tab".to_string(),
            loading: false,
            history: Vec::new(),
            history_index: 0,
        }
    }

    /// Navigate to a URL
    pub fn navigate(&mut self, url: String) {
        // Add current URL to history
        if let Some(current_url) = &self.url {
            self.history.truncate(self.history_index + 1);
            self.history.push(current_url.clone());
        }
        
        self.url = Some(url.clone());
        self.history_index = self.history.len() - 1;
        self.loading = true;
        self.title = "Loading...".to_string();
    }

    /// Go back in history
    pub fn go_back(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(url) = self.history.get(self.history_index) {
                self.url = Some(url.clone());
                return true;
            }
        }
        false
    }

    /// Go forward in history
    pub fn go_forward(&mut self) -> bool {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            if let Some(url) = self.history.get(self.history_index) {
                self.url = Some(url.clone());
                return true;
            }
        }
        false
    }

    /// Reload current page
    pub fn reload(&mut self) {
        if let Some(url) = &self.url {
            self.navigate(url.clone());
        }
    }

    /// Stop loading
    pub fn stop(&mut self) {
        self.loading = false;
    }

    /// Generate unique tab ID
    fn generate_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("tab-{}", timestamp)
    }
}

/// Bookmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
    pub folder: Option<String>,
    pub created_at: String,
}

impl Bookmark {
    /// Create a new bookmark
    pub fn new(url: String, title: String, folder: Option<String>) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Bookmark {
            id: format!("bmk-{}", timestamp),
            url,
            title,
            folder,
            created_at: timestamp.to_string(),
        }
    }
}

/// Browser history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub visited_at: String,
    pub visit_count: u32,
}

/// Browser application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Browser {
    pub tabs: Vec<Tab>,
    pub active_tab_index: usize,
    pub bookmarks: Vec<Bookmark>,
    pub history: Vec<HistoryEntry>,
    pub home_page: String,
    pub search_engine: String,
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}

impl Browser {
    /// Create a new browser instance
    pub fn new() -> Self {
        let mut browser = Browser {
            tabs: Vec::new(),
            active_tab_index: 0,
            bookmarks: Vec::new(),
            history: Vec::new(),
            home_page: "https://www.example.com".to_string(),
            search_engine: "https://www.search.com?q=".to_string(),
        };
        
        // Create initial tab
        browser.new_tab();
        browser
    }

    /// Create a new tab
    pub fn new_tab(&mut self) {
        let tab = Tab::new();
        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
    }

    /// Close current tab
    pub fn close_tab(&mut self) -> bool {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.active_tab_index);
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    /// Switch to a specific tab
    pub fn switch_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            true
        } else {
            false
        }
    }

    /// Get current tab
    pub fn current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    /// Get mutable current tab
    pub fn current_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }

    /// Navigate current tab to URL
    pub fn navigate(&mut self, url: String) {
        if let Some(tab) = self.current_tab_mut() {
            tab.navigate(url.clone());
            self.add_to_history(&url, "Page".to_string());
        }
    }

    /// Go back in current tab
    pub fn go_back(&mut self) -> bool {
        if let Some(tab) = self.current_tab_mut() {
            tab.go_back()
        } else {
            false
        }
    }

    /// Go forward in current tab
    pub fn go_forward(&mut self) -> bool {
        if let Some(tab) = self.current_tab_mut() {
            tab.go_forward()
        } else {
            false
        }
    }

    /// Reload current tab
    pub fn reload(&mut self) {
        if let Some(tab) = self.current_tab_mut() {
            tab.reload();
        }
    }

    /// Stop loading current tab
    pub fn stop(&mut self) {
        if let Some(tab) = self.current_tab_mut() {
            tab.stop();
        }
    }

    /// Navigate to home page
    pub fn go_home(&mut self) {
        self.navigate(self.home_page.clone());
    }

    /// Search query
    pub fn search(&mut self, query: String) {
        let url = format!("{}{}", self.search_engine, urlencoding::encode(&query));
        self.navigate(url);
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, url: String, title: String, folder: Option<String>) {
        let bookmark = Bookmark::new(url.clone(), title, folder);
        self.bookmarks.push(bookmark);
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, id: &str) -> bool {
        if let Some(index) = self.bookmarks.iter().position(|b| b.id == id) {
            self.bookmarks.remove(index);
            true
        } else {
            false
        }
    }

    /// Get bookmarks for a folder
    pub fn get_bookmarks(&self, folder: Option<&str>) -> Vec<&Bookmark> {
        self.bookmarks
            .iter()
            .filter(|b| b.folder.as_deref() == folder)
            .collect()
    }

    /// Add to history
    fn add_to_history(&mut self, url: &str, title: String) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        
        // Check if already in history
        if let Some(entry) = self.history.iter_mut().find(|e| e.url == url) {
            entry.title = title;
            entry.visited_at = timestamp;
            entry.visit_count += 1;
            return;
        }
        
        let entry = HistoryEntry {
            url: url.to_string(),
            title,
            visited_at: timestamp,
            visit_count: 1,
        };
        
        self.history.push(entry);
        
        // Keep only last 1000 entries
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    /// Get history
    pub fn get_history(&self) -> &[HistoryEntry] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Search history
    pub fn search_history(&self, query: &str) -> Vec<&HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.history
            .iter()
            .filter(|e| {
                e.url.to_lowercase().contains(&query_lower)
                    || e.title.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get bookmark folders
    pub fn get_bookmark_folders(&self) -> Vec<String> {
        let mut folders: Vec<String> = self.bookmarks
            .iter()
            .filter_map(|b| b.folder.clone())
            .collect();
        folders.sort();
        folders.dedup();
        folders
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_creation() {
        let browser = Browser::new();
        assert_eq!(browser.tabs.len(), 1);
        assert_eq!(browser.active_tab_index, 0);
    }

    #[test]
    fn test_new_tab() {
        let mut browser = Browser::new();
        browser.new_tab();
        assert_eq!(browser.tabs.len(), 2);
    }

    #[test]
    fn test_close_tab() {
        let mut browser = Browser::new();
        browser.new_tab();
        assert!(browser.close_tab());
        assert_eq!(browser.tabs.len(), 1);
    }

    #[test]
    fn test_navigate() {
        let mut browser = Browser::new();
        browser.navigate("https://www.example.com".to_string());
        let tab = browser.current_tab().unwrap();
        assert_eq!(tab.url, Some("https://www.example.com".to_string()));
    }

    #[test]
    fn test_add_bookmark() {
        let mut browser = Browser::new();
        browser.add_bookmark(
            "https://www.example.com".to_string(),
            "Example".to_string(),
            Some("Favorites".to_string()),
        );
        assert_eq!(browser.bookmarks.len(), 1);
    }

    #[test]
    fn test_remove_bookmark() {
        let mut browser = Browser::new();
        browser.add_bookmark(
            "https://www.example.com".to_string(),
            "Example".to_string(),
            None,
        );
        let bookmark = &browser.bookmarks[0];
        assert!(browser.remove_bookmark(&bookmark.id));
        assert_eq!(browser.bookmarks.len(), 0);
    }
}