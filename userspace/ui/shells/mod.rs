pub mod classic;
pub mod classic_shell;
pub mod radial;
pub mod spatial;

pub use classic_shell::{ClassicShell, Shell, ShellError};

/// Re-export commonly used types
pub mod types {
    pub use super::classic_shell::{
        Taskbar, TaskbarPosition, TaskbarTheme, PinnedApp, RunningApp,
        StartMenu, StartMenuApp, SearchResult, AppCategory, PowerOptions, UserProfile,
        WindowManager, Window, WindowState, WindowType, WindowDecorations,
        DesktopIcons, DesktopIcon, IconType, IconGrid, SortOrder, IconSize,
        NotificationSystem, Notification, NotificationUrgency, NotificationPosition,
        WorkspaceManager, Workspace, SystemTray, TrayIcon, Clock, ShellState,
    };
}
