pub mod classic;
pub mod classic_shell;
pub mod radial_shell;
pub mod radial;
pub mod spatial_shell;
pub mod spatial;

pub use classic_shell::{ClassicShell, Shell, ShellError};
pub use radial_shell::{RadialShell, RadialConfig, RadialShellError};
pub use spatial_shell::{SpatialShell, SpatialConfig, SpatialShellError};

/// Re-export commonly used types
pub mod types {
    pub use super::classic_shell::{
        Taskbar, TaskbarPosition, TaskbarTheme, PinnedApp, RunningApp,
        StartMenu, StartMenuApp, SearchResult, AppCategory, PowerOptions, UserProfile,
        WindowManager, Window, WindowState, WindowType, WindowDecorations,
        DesktopIcons, DesktopIcon, IconType, IconGrid, SortOrder, IconSize,
        NotificationSystem, Notification, NotificationUrgency, NotificationPosition,
        WorkspaceManager, Workspace, SystemTray, TrayIcon, Clock, ShellState as ClassicShellState,
    };
    
    pub use super::radial_shell::{
        RadialMenu, MenuItem, MenuAction, SystemAction, QuickAction, QuickActionPosition,
        NotificationCenter, Notification, NotificationPriority, RecentItem, RecentItemType,
        Gesture, SwipeDirection, RadialTheme, GestureConfig, ShellState as RadialShellState,
    };
    
    pub use super::spatial_shell::{
        Room, RoomType, RoomDimensions, SpatialObject, ObjectType, ObjectAction,
        RoomConnection, ConnectionType, Camera, HandTracking, Hand, HandGesture,
        InputState, SpatialTheme, ShadowQuality, ShellState as SpatialShellState,
    };
}
