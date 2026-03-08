# VantisOS Development Todo

## Phase 3: System Updates & UI (In Progress)

### Update System Module
- [x] Research available repositories for update mechanisms
- [x] Design update system architecture
- [x] Implement update manager core (update/mod.rs - ~600 lines)
- [x] Add update sources configuration
- [x] Implement driver update detection
- [x] Add application update tracking
- [ ] Create update notification system

### Archive Support Module
- [x] Implement ZIP support (archive/zip.rs - ~500 lines)
- [x] Implement TAR support (archive/tar.rs - ~400 lines)
- [x] Create compression module (archive/compression.rs - deflate, LZMA, bzip2, zstd)
- [x] Create encryption module (archive/encryption.rs - AES-256, CBC, PBKDF2)
- [ ] Implement RAR support (placeholder in mod.rs)
- [ ] Implement 7z support (placeholder in mod.rs)
- [ ] Create SFX archive support

### UI/Shell System
- [x] Shell module core (shell/mod.rs - types, events, surface trait)
- [x] Desktop environment (shell/desktop.rs - icons, context menus)
- [x] Context menu (shell/menu.rs - context menu implementation)
- [x] Taskbar (shell/taskbar.rs - start button, apps, system tray)
- [x] Start menu (shell/menu.rs - pinned apps, search, power options)
- [x] File explorer (shell/explorer.rs - navigation, file listing)
- [x] Drive management (shell/explorer.rs - drives, quick access)
- [x] Window manager (shell/window.rs - window creation, decorations, events)

### Integration Tasks
- [x] Update kernel lib.rs to include new modules
- [ ] Fix pre-existing kernel compilation issues (missing serial_println, volatile, etc.)
- [ ] Test build with new modules

### Files Created
- iso_build/kernel/src/update/mod.rs - Update manager with driver/app tracking
- iso_build/kernel/src/archive/mod.rs - Archive format detection and handling
- iso_build/kernel/src/archive/zip.rs - ZIP reader/writer implementation
- iso_build/kernel/src/archive/tar.rs - TAR reader/writer implementation
- iso_build/kernel/src/archive/compression.rs - Compression algorithms
- iso_build/kernel/src/archive/encryption.rs - AES-256 encryption
- iso_build/kernel/src/shell/mod.rs - Shell core types and traits
- iso_build/kernel/src/shell/desktop.rs - Desktop environment
- iso_build/kernel/src/shell/taskbar.rs - Taskbar implementation
- iso_build/kernel/src/shell/menu.rs - Start menu and context menus
- iso_build/kernel/src/shell/explorer.rs - File explorer
- iso_build/kernel/src/shell/window.rs - Window manager