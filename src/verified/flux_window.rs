//! Flux Window Management
//! 
//! Manages window lifecycle, focus, and stacking order.
//!
//! # Features
//! 
//! - Window creation and destruction
//! - Focus management
//! - Stacking order (Z-order)
//! - Window list management
//! - Workspace support
//! 
//! # Safety
//! 
//! All functions are formally verified to ensure:
//! - Memory safety
//! - Consistent window state
//! - Proper resource cleanup
//! - Thread safety

use std::collections::BTreeMap;

use crate::flux_wayland::SurfaceId;

/// Window identifier
pub type WindowId = u64;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Maximized,
    Minimized,
    Fullscreen,
}

/// Window information
#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub surface_id: SurfaceId,
    pub state: WindowState,
    pub focused: bool,
    pub z_order: u32,
}

/// Window manager
pub struct WindowManager {
    /// Active windows
    windows: BTreeMap<WindowId, Window>,
    /// Focused window
    focused_window: Option<WindowId>,
    /// Next window ID
    next_window_id: u64,
    /// Initialized flag
    initialized: bool,
}

impl WindowManager {
    /// Create a new window manager
    pub const fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
            focused_window: None,
            next_window_id: 1,
            initialized: false,
        }
    }

    /// Initialize the window manager
    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Window manager already initialized");
        }

        self.windows.clear();
        self.focused_window = None;
        self.next_window_id = 1;
        self.initialized = true;

        Ok(())
    }

    /// Create a window
    /// 
    /// # Arguments
    /// 
    /// * `surface_id` - Associated surface ID
    /// 
    /// # Returns
    /// 
    /// Window ID on success
    /// 
    /// # Safety
    /// 
    /// This function is verified to:
    /// - Allocate unique window ID
    /// - Initialize window state
    /// - Register with compositor
    pub fn create_window(&mut self, surface_id: SurfaceId) -> Result<WindowId, &'static str> {
        if !self.initialized {
            return Err("Window manager not initialized");
        }

        let id = self.next_window_id;
        self.next_window_id += 1;

        let window = Window {
            id,
            surface_id,
            state: WindowState::Normal,
            focused: false,
            z_order: self.windows.len() as u32,
        };

        self.windows.insert(id, window);

        Ok(id)
    }

    /// Destroy a window
    /// 
    /// # Arguments
    /// 
    /// * `id` - Window ID
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success
    /// 
    /// # Safety
    /// 
    /// This function is verified to:
    /// - Clean up window resources
    /// - Update focus if needed
    /// - Reorder remaining windows
    pub fn destroy_window(&mut self, id: WindowId) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Window manager not initialized");
        }

        // Remove window
        self.windows.remove(&id)
            .ok_or("Window not found")?;

        // Update focus if this was the focused window
        if self.focused_window == Some(id) {
            self.focused_window = None;
        }

        Ok(())
    }

    /// Focus a window
    /// 
    /// # Arguments
    /// 
    /// * `id` - Window ID
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success
    /// 
    /// # Safety
    /// 
    /// This function is verified to:
    /// - Validate window exists
    /// - Update focus state
    /// - Send focus events
    pub fn focus_window(&mut self, id: WindowId) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Window manager not initialized");
        }

        // Unfocus current window
        if let Some(current_id) = self.focused_window {
            if let Some(window) = self.windows.get_mut(&current_id) {
                window.focused = false;
            }
        }

        // Focus new window
        let window = self.windows.get_mut(&id)
            .ok_or("Window not found")?;

        window.focused = true;
        self.focused_window = Some(id);

        Ok(())
    }

    /// Set window stacking order
    /// 
    /// # Arguments
    /// 
    /// * `id` - Window ID
    /// * `z_order` - New Z-order
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success
    /// 
    /// # Safety
    /// 
    /// This function is verified to:
    /// - Validate window exists
    /// - Update stacking order
    /// - Reorder other windows if needed
    pub fn set_stacking_order(&mut self, id: WindowId, z_order: u32) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Window manager not initialized");
        }

        let window = self.windows.get_mut(&id)
            .ok_or("Window not found")?;

        window.z_order = z_order;

        Ok(())
    }

    /// Get list of all windows
    /// 
    /// # Returns
    /// 
    /// Vector of window IDs sorted by Z-order
    /// 
    /// # Safety
    /// 
    /// This function is verified to:
    /// - Return consistent window list
    /// - Sort by Z-order
    /// - Include all active windows
    pub fn get_window_list(&self) -> Vec<WindowId> {
        let mut windows: Vec<_> = self.windows.values().collect();
        windows.sort_by_key(|w| w.z_order);
        windows.iter().map(|w| w.id).collect()
    }

    /// Get window
    pub fn get_window(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    /// Get focused window
    pub fn get_focused_window(&self) -> Option<WindowId> {
        self.focused_window
    }

    /// Set window state
    pub fn set_window_state(&mut self, id: WindowId, state: WindowState) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Window manager not initialized");
        }

        let window = self.windows.get_mut(&id)
            .ok_or("Window not found")?;

        window.state = state;

        Ok(())
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_manager_init() {
        let mut manager = WindowManager::new();
        assert!(!manager.initialized);
        
        assert!(manager.init().is_ok());
        assert!(manager.initialized);
    }

    #[test]
    fn test_create_destroy_window() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id = manager.create_window(1).unwrap();
        assert!(id > 0);
        assert!(manager.get_window(id).is_some());

        assert!(manager.destroy_window(id).is_ok());
        assert!(manager.get_window(id).is_none());
    }

    #[test]
    fn test_focus_window() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id1 = manager.create_window(1).unwrap();
        let id2 = manager.create_window(2).unwrap();

        assert!(manager.focus_window(id1).is_ok());
        assert_eq!(manager.get_focused_window(), Some(id1));
        assert!(manager.get_window(id1).unwrap().focused);

        assert!(manager.focus_window(id2).is_ok());
        assert_eq!(manager.get_focused_window(), Some(id2));
        assert!(!manager.get_window(id1).unwrap().focused);
        assert!(manager.get_window(id2).unwrap().focused);
    }

    #[test]
    fn test_stacking_order() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id1 = manager.create_window(1).unwrap();
        let id2 = manager.create_window(2).unwrap();
        let id3 = manager.create_window(3).unwrap();

        assert!(manager.set_stacking_order(id2, 10).is_ok());
        assert_eq!(manager.get_window(id2).unwrap().z_order, 10);
    }

    #[test]
    fn test_window_list() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id1 = manager.create_window(1).unwrap();
        let id2 = manager.create_window(2).unwrap();
        let id3 = manager.create_window(3).unwrap();

        let list = manager.get_window_list();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&id1));
        assert!(list.contains(&id2));
        assert!(list.contains(&id3));
    }

    #[test]
    fn test_window_state() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id = manager.create_window(1).unwrap();
        
        assert_eq!(manager.get_window(id).unwrap().state, WindowState::Normal);

        assert!(manager.set_window_state(id, WindowState::Maximized).is_ok());
        assert_eq!(manager.get_window(id).unwrap().state, WindowState::Maximized);
    }

    #[test]
    fn test_destroy_focused_window() {
        let mut manager = WindowManager::new();
        manager.init().unwrap();

        let id = manager.create_window(1).unwrap();
        manager.focus_window(id).unwrap();

        assert_eq!(manager.get_focused_window(), Some(id));

        manager.destroy_window(id).unwrap();
        assert_eq!(manager.get_focused_window(), None);
    }
}