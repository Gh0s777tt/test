use super::classic_shell::ClassicShell;

/// Classic Shell - Traditional desktop environment
/// 
/// This module provides a Windows/macOS-like desktop experience with:
/// - Taskbar with system tray and clock
/// - Start menu with search
/// - Window management with minimize/maximize/close
/// - Desktop icons
/// - Notification system
/// - Workspace management
/// 
/// # Example
/// 
/// ```rust
/// use vantisos_ui_shells::classic::ClassicShell;
/// 
/// let shell = ClassicShell::new();
/// shell.initialize().unwrap();
/// shell.run().unwrap();
/// ```

pub use classic_shell::ClassicShell;
