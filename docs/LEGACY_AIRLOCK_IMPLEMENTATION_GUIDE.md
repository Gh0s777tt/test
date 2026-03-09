# Legacy Airlock (.exe) Implementation Guide

## Executive Summary

This guide provides a comprehensive implementation plan for the Legacy Airlock subsystem in VantisOS, enabling Windows executable (.exe) files to run in a secure, isolated environment with full compatibility.

**Implementation Timeline**: 5 days  
**Complexity**: Very High  
**Dependencies**: Vantis Core, VantisFS, Vantis Vault  
**Security Level**: Critical (EAL 7+)

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Technical Requirements](#technical-requirements)
3. [Implementation Plan](#implementation-plan)
4. [Security Considerations](#security-considerations)
5. [Performance Targets](#performance-targets)
6. [Testing Strategy](#testing-strategy)
7. [Code Examples](#code-examples)
8. [Troubleshooting](#troubleshooting)

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     VantisOS Kernel                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   IPC Core   │  │  Scheduler   │  │ Memory Mgmt  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Legacy Airlock Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  PE Loader   │  │  Win32 API   │  │  Syscall     │      │
│  │              │  │  Emulation   │  │  Translation │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Windows Compatibility Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Kernel32    │  │  User32      │  │  GDI32       │      │
│  │  DLLs        │  │  DLLs        │  │  DLLs        │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Windows Executables                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   .exe Files │  │   .dll Files │  │  Resources   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Key Components

1. **PE Loader**: Portable Executable file format loader
2. **Win32 API Emulation**: Windows API compatibility layer
3. **Syscall Translation**: Windows syscalls to VantisOS syscalls
4. **DLL Loader**: Dynamic Link Library loading
5. **Registry Emulation**: Windows Registry compatibility

---

## Technical Requirements

### Windows Version Support

- **Target**: Windows 10/11 compatibility
- **Minimum**: Windows 7 SP1
- **PE Format**: PE32, PE32+
- **Architecture**: x86, x86_64

### Supported APIs

**Core APIs:**
- Kernel32: Process management, memory, file I/O
- User32: Windows, messages, input
- GDI32: Graphics, fonts, printing
- Advapi32: Registry, security, services

**Extended APIs:**
- Shell32: Shell operations
- Comctl32: Common controls
- Winmm: Multimedia
- Winsock: Networking

### Software Dependencies

```toml
[dependencies]
# PE Format
pe-parser = { version = "0.5.0", features = ["full"] }

# Windows API
winapi = { version = "0.3.9", features = ["full"] }
windows-sys = { version = "0.52.0", features = ["full"] }

# Emulation
unicorn-engine = "2.0"
capstone = "0.11"

# Graphics
vulkan = "1.3"
opengl = "0.18"
```

---

## Implementation Plan

### Day 1: PE Loader Implementation

**Tasks:**
1. Implement PE file parser
2. Handle PE32 and PE32+ formats
3. Load sections into memory
4. Resolve imports and exports

**Code Structure:**
```rust
// src/legacy/pe_loader.rs
use vantis_core::memory::MemoryManager;

pub struct PeLoader {
    memory_manager: Arc<MemoryManager>,
    loaded_modules: HashMap<String, LoadedModule>,
}

#[derive(Clone)]
pub struct LoadedModule {
    pub base_address: u64,
    pub entry_point: u64,
    pub sections: Vec<Section>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

impl PeLoader {
    pub fn new(memory_manager: Arc<MemoryManager>) -> Result<Self, LoaderError> {
        Ok(PeLoader {
            memory_manager,
            loaded_modules: HashMap::new(),
        })
    }

    pub fn load_exe(&mut self, exe_path: &Path) -> Result<LoadedModule, LoaderError> {
        // Read PE file
        let pe_data = fs::read(exe_path)?;
        let pe = PeFile::parse(&pe_data)?;
        
        // Validate PE format
        self.validate_pe(&pe)?;
        
        // Allocate memory
        let base_address = self.allocate_memory(&pe)?;
        
        // Load sections
        let sections = self.load_sections(&pe, base_address)?;
        
        // Resolve imports
        let imports = self.resolve_imports(&pe)?;
        
        // Apply relocations
        self.apply_relocations(&pe, base_address)?;
        
        // Get entry point
        let entry_point = base_address + pe.entry_point();
        
        let module = LoadedModule {
            base_address,
            entry_point,
            sections,
            imports,
            exports: self.extract_exports(&pe)?,
        };
        
        self.loaded_modules.insert(
            exe_path.to_string_lossy().to_string(),
            module.clone()
        );
        
        Ok(module)
    }

    pub fn load_dll(&mut self, dll_path: &Path) -> Result<LoadedModule, LoaderError> {
        // Similar to load_exe but for DLLs
        let dll_data = fs::read(dll_path)?;
        let pe = PeFile::parse(&dll_data)?;
        
        // Check if already loaded
        let dll_name = dll_path.file_name()
            .ok_or(LoaderError::InvalidPath)?
            .to_string_lossy()
            .to_string();
        
        if let Some(module) = self.loaded_modules.get(&dll_name) {
            return Ok(module.clone());
        }
        
        // Load DLL
        let base_address = self.allocate_memory(&pe)?;
        let sections = self.load_sections(&pe, base_address)?;
        let imports = self.resolve_imports(&pe)?;
        self.apply_relocations(&pe, base_address)?;
        
        let module = LoadedModule {
            base_address,
            entry_point: 0, // DLLs don't have entry points
            sections,
            imports,
            exports: self.extract_exports(&pe)?,
        };
        
        self.loaded_modules.insert(dll_name, module.clone());
        
        Ok(module)
    }

    fn allocate_memory(&self, pe: &PeFile) -> Result<u64, LoaderError> {
        // Calculate required memory
        let image_size = pe.image_size();
        
        // Allocate memory with proper permissions
        let address = self.memory_manager.allocate(
            image_size as usize,
            MemoryFlags::READ | MemoryFlags::WRITE | MemoryFlags::EXECUTE
        )?;
        
        Ok(address as u64)
    }

    fn load_sections(&self, pe: &PeFile, base_address: u64) -> Result<Vec<Section>, LoaderError> {
        let mut sections = Vec::new();
        
        for section_header in pe.section_headers() {
            let section_data = pe.section_data(section_header)?;
            
            // Map section to memory
            let section_address = base_address + section_header.virtual_address;
            self.memory_manager.write(
                section_address as usize,
                &section_data
            )?;
            
            // Set section permissions
            let mut flags = MemoryFlags::empty();
            if section_header.characteristics.contains(Characteristics::EXECUTABLE) {
                flags |= MemoryFlags::EXECUTE;
            }
            if section_header.characteristics.contains(Characteristics::READABLE) {
                flags |= MemoryFlags::READ;
            }
            if section_header.characteristics.contains(Characteristics::WRITABLE) {
                flags |= MemoryFlags::WRITE;
            }
            
            self.memory_manager.set_permissions(
                section_address as usize,
                section_header.virtual_size as usize,
                flags
            )?;
            
            sections.push(Section {
                name: section_header.name.to_string(),
                address: section_address,
                size: section_header.virtual_size,
                flags,
            });
        }
        
        Ok(sections)
    }

    fn resolve_imports(&mut self, pe: &PeFile) -> Result<Vec<Import>, LoaderError> {
        let mut imports = Vec::new();
        
        for import_desc in pe.import_descriptors() {
            let dll_name = import_desc.dll_name(pe)?;
            
            // Load DLL
            let dll_module = self.load_dll(&PathBuf::from(dll_name))?;
            
            // Resolve imports
            for import in import_desc.imports(pe)? {
                let function_name = import.name(pe)?;
                
                // Find function in DLL exports
                let function_address = dll_module.exports
                    .iter()
                    .find(|e| e.name == function_name)
                    .map(|e| dll_module.base_address + e.address)
                    .ok_or(LoaderError::FunctionNotFound(function_name.clone()))?;
                
                // Patch import table
                self.memory_manager.write_u64(
                    (pe.image_base() + import.address()) as usize,
                    function_address
                )?;
                
                imports.push(Import {
                    dll_name: dll_name.to_string(),
                    function_name: function_name.to_string(),
                    address: function_address,
                });
            }
        }
        
        Ok(imports)
    }

    fn apply_relocations(&self, pe: &PeFile, base_address: u64) -> Result<(), LoaderError> {
        let delta = base_address.wrapping_sub(pe.image_base());
        
        for reloc in pe.relocations() {
            for reloc_entry in reloc.entries() {
                let target_address = base_address + reloc_entry.offset;
                let current_value = self.memory_manager.read_u64(target_address as usize)?;
                
                match reloc_entry.type_ {
                    RelocType::DIR64 => {
                        let new_value = current_value.wrapping_add(delta);
                        self.memory_manager.write_u64(target_address as usize, new_value)?;
                    }
                    RelocType::HIGHLOW => {
                        let new_value = (current_value as u32).wrapping_add(delta as u32);
                        self.memory_manager.write_u32(target_address as usize, new_value)?;
                    }
                    _ => {
                        return Err(LoaderError::UnsupportedRelocation(reloc_entry.type_));
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### Day 2: Win32 API Emulation

**Tasks:**
1. Implement core Win32 APIs
2. Handle window management
3. Implement message handling
4. Add graphics support

**Code Structure:**
```rust
// src/legacy/win32_api.rs
use vantis_core::graphics::GraphicsBackend;

pub struct Win32Api {
    graphics_backend: Arc<GraphicsBackend>,
    windows: HashMap<HWND, Window>,
    message_queue: Vec<Message>,
}

impl Win32Api {
    pub fn new(graphics_backend: Arc<GraphicsBackend>) -> Result<Self, ApiError> {
        Ok(Win32Api {
            graphics_backend,
            windows: HashMap::new(),
            message_queue: Vec::new(),
        })
    }

    // Window Management
    pub fn create_window_ex(
        &mut self,
        ex_style: DWORD,
        class_name: &str,
        window_name: &str,
        style: DWORD,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: HWND,
        menu: HMENU,
        instance: HINSTANCE,
        param: LPVOID,
    ) -> Result<HWND, ApiError> {
        // Create window
        let window = Window::new(
            class_name,
            window_name,
            x, y, width, height,
            style,
            ex_style,
        )?;
        
        // Create graphics surface
        let surface = self.graphics_backend.create_surface(
            width as u32,
            height as u32,
            SurfaceFormat::BGRA8
        )?;
        
        window.set_surface(surface);
        
        let hwnd = window.handle();
        self.windows.insert(hwnd, window);
        
        Ok(hwnd)
    }

    pub fn show_window(&mut self, hwnd: HWND, cmd_show: i32) -> Result<(), ApiError> {
        let window = self.windows.get_mut(&hwnd)
            .ok_or(ApiError::InvalidWindow)?;
        
        window.show(cmd_show)?;
        
        Ok(())
    }

    pub fn update_window(&mut self, hwnd: HWND) -> Result<(), ApiError> {
        let window = self.windows.get_mut(&hwnd)
            .ok_or(ApiError::InvalidWindow)?;
        
        window.update()?;
        
        Ok(())
    }

    // Message Handling
    pub fn get_message(&mut self, msg: &mut MSG, hwnd: HWND, filter_min: UINT, filter_max: UINT) -> Result<BOOL, ApiError> {
        if let Some(message) = self.message_queue.pop() {
            *msg = message;
            Ok(TRUE)
        } else {
            Ok(FALSE)
        }
    }

    pub fn translate_message(&self, msg: &MSG) -> Result<BOOL, ApiError> {
        // Translate keyboard messages
        Ok(TRUE)
    }

    pub fn dispatch_message(&mut self, msg: &MSG) -> Result<LRESULT, ApiError> {
        let window = self.windows.get_mut(&msg.hwnd)
            .ok_or(ApiError::InvalidWindow)?;
        
        let result = window.window_proc(msg.message, msg.wParam, msg.lParam)?;
        
        Ok(result)
    }

    pub fn post_message(&mut self, hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Result<(), ApiError> {
        self.message_queue.push(MSG {
            hwnd,
            message: msg,
            wParam: wparam,
            lParam: lparam,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        });
        
        Ok(())
    }

    // Graphics API
    pub fn get_dc(&self, hwnd: HWND) -> Result<HDC, ApiError> {
        let window = self.windows.get(&hwnd)
            .ok_or(ApiError::InvalidWindow)?;
        
        Ok(window.device_context())
    }

    pub fn release_dc(&self, hwnd: HWND, hdc: HDC) -> Result<i32, ApiError> {
        Ok(1)
    }

    pub fn text_out(&self, hdc: HDC, x: i32, y: i32, text: &str) -> Result<BOOL, ApiError> {
        // Render text to surface
        // Implementation details
        Ok(TRUE)
    }
}

// Window structure
pub struct Window {
    handle: HWND,
    class_name: String,
    title: String,
    rect: RECT,
    style: DWORD,
    ex_style: DWORD,
    surface: Option<Surface>,
    device_context: HDC,
}

impl Window {
    pub fn new(
        class_name: &str,
        title: &str,
        x: i32, y: i32,
        width: i32, height: i32,
        style: DWORD,
        ex_style: DWORD,
    ) -> Result<Self, ApiError> {
        let handle = generate_hwnd();
        
        Ok(Window {
            handle,
            class_name: class_name.to_string(),
            title: title.to_string(),
            rect: RECT {
                left: x,
                top: y,
                right: x + width,
                bottom: y + height,
            },
            style,
            ex_style,
            surface: None,
            device_context: generate_hdc(),
        })
    }

    pub fn show(&mut self, cmd_show: i32) -> Result<(), ApiError> {
        // Show window
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), ApiError> {
        // Update window
        Ok(())
    }

    pub fn window_proc(&mut self, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, ApiError> {
        match msg {
            WM_CREATE => Ok(0),
            WM_DESTROY => {
                // Post quit message
                Ok(0)
            }
            WM_PAINT => {
                // Handle paint
                Ok(0)
            }
            _ => {
                // Default window procedure
                Ok(DefWindowProc(self.handle, msg, wparam, lparam))
            }
        }
    }

    pub fn handle(&self) -> HWND {
        self.handle
    }

    pub fn set_surface(&mut self, surface: Surface) {
        self.surface = Some(surface);
    }

    pub fn device_context(&self) -> HDC {
        self.device_context
    }
}
```

### Day 3: Syscall Translation

**Tasks:**
1. Implement Windows syscall translation
2. Handle file operations
3. Implement memory management
4. Add process management

**Code Structure:**
```rust
// src/legacy/syscall_translation.rs
use vantis_core::syscall::SyscallHandler;

pub struct SyscallTranslator {
    vantis_syscalls: Arc<SyscallHandler>,
    file_handles: HashMap<HANDLE, FileHandle>,
    process_handles: HashMap<HANDLE, ProcessHandle>,
}

impl SyscallTranslator {
    pub fn new(vantis_syscalls: Arc<SyscallHandler>) -> Result<Self, TranslationError> {
        Ok(SyscallTranslator {
            vantis_syscalls,
            file_handles: HashMap::new(),
            process_handles: HashMap::new(),
        })
    }

    // File Operations
    pub fn create_file(
        &mut self,
        filename: &str,
        desired_access: DWORD,
        share_mode: DWORD,
        security_attributes: Option<&SECURITY_ATTRIBUTES>,
        creation_disposition: DWORD,
        flags_and_attributes: DWORD,
        template_file: HANDLE,
    ) -> Result<HANDLE, TranslationError> {
        // Translate to VantisOS file open
        let vantis_flags = self.translate_file_flags(desired_access, creation_disposition)?;
        
        let file_handle = self.vantis_syscalls.open_file(
            filename,
            vantis_flags
        )?;
        
        let handle = generate_handle();
        self.file_handles.insert(handle, FileHandle::new(file_handle));
        
        Ok(handle)
    }

    pub fn read_file(
        &mut self,
        handle: HANDLE,
        buffer: &mut [u8],
        bytes_to_read: DWORD,
        bytes_read: &mut DWORD,
        overlapped: Option<&mut OVERLAPPED>,
    ) -> Result<BOOL, TranslationError> {
        let file_handle = self.file_handles.get(&handle)
            .ok_or(TranslationError::InvalidHandle)?;
        
        let result = self.vantis_syscalls.read_file(
            file_handle.vantis_handle(),
            buffer
        )?;
        
        *bytes_read = result as DWORD;
        
        Ok(TRUE)
    }

    pub fn write_file(
        &mut self,
        handle: HANDLE,
        buffer: &[u8],
        bytes_to_write: DWORD,
        bytes_written: &mut DWORD,
        overlapped: Option<&mut OVERLAPPED>,
    ) -> Result<BOOL, TranslationError> {
        let file_handle = self.file_handles.get(&handle)
            .ok_or(TranslationError::InvalidHandle)?;
        
        let result = self.vantis_syscalls.write_file(
            file_handle.vantis_handle(),
            buffer
        )?;
        
        *bytes_written = result as DWORD;
        
        Ok(TRUE)
    }

    pub fn close_handle(&mut self, handle: HANDLE) -> Result<BOOL, TranslationError> {
        if let Some(file_handle) = self.file_handles.remove(&handle) {
            self.vantis_syscalls.close_file(file_handle.vantis_handle())?;
        } else if let Some(process_handle) = self.process_handles.remove(&handle) {
            self.vantis_syscalls.close_process(process_handle.vantis_handle())?;
        }
        
        Ok(TRUE)
    }

    // Memory Operations
    pub fn virtual_alloc(
        &mut self,
        address: LPVOID,
        size: SIZE_T,
        allocation_type: DWORD,
        protect: DWORD,
    ) -> Result<LPVOID, TranslationError> {
        // Translate to VantisOS memory allocation
        let vantis_flags = self.translate_memory_flags(protect)?;
        
        let vantis_address = self.vantis_syscalls.allocate_memory(
            address as usize,
            size as usize,
            vantis_flags
        )?;
        
        Ok(vantis_address as LPVOID)
    }

    pub fn virtual_free(
        &mut self,
        address: LPVOID,
        size: SIZE_T,
        free_type: DWORD,
    ) -> Result<BOOL, TranslationError> {
        self.vantis_syscalls.free_memory(address as usize)?;
        Ok(TRUE)
    }

    // Process Operations
    pub fn create_process(
        &mut self,
        application_name: &str,
        command_line: &str,
        process_attributes: Option<&SECURITY_ATTRIBUTES>,
        thread_attributes: Option<&SECURITY_ATTRIBUTES>,
        inherit_handles: BOOL,
        creation_flags: DWORD,
        environment: Option<&[u8]>,
        current_directory: Option<&str>,
        startup_info: &STARTUPINFO,
        process_info: &mut PROCESS_INFORMATION,
    ) -> Result<BOOL, TranslationError> {
        // Create process
        let vantis_process = self.vantis_syscalls.create_process(
            application_name,
            command_line
        )?;
        
        let process_handle = generate_handle();
        let thread_handle = generate_handle();
        
        self.process_handles.insert(
            process_handle,
            ProcessHandle::new(vantis_process)
        );
        
        process_info.hProcess = process_handle;
        process_info.hThread = thread_handle;
        process_info.dwProcessId = vantis_process.pid();
        process_info.dwThreadId = vantis_process.tid();
        
        Ok(TRUE)
    }

    pub fn terminate_process(&mut self, handle: HANDLE, exit_code: UINT) -> Result<BOOL, TranslationError> {
        let process_handle = self.process_handles.get(&handle)
            .ok_or(TranslationError::InvalidHandle)?;
        
        self.vantis_syscalls.terminate_process(
            process_handle.vantis_handle(),
            exit_code
        )?;
        
        Ok(TRUE)
    }

    // Helper functions
    fn translate_file_flags(&self, desired_access: DWORD, creation_disposition: DWORD) -> Result<FileFlags, TranslationError> {
        let mut flags = FileFlags::empty();
        
        if desired_access & GENERIC_READ != 0 {
            flags |= FileFlags::READ;
        }
        if desired_access & GENERIC_WRITE != 0 {
            flags |= FileFlags::WRITE;
        }
        
        match creation_disposition {
            CREATE_NEW => flags |= FileFlags::CREATE_NEW,
            CREATE_ALWAYS => flags |= FileFlags::CREATE_ALWAYS,
            OPEN_EXISTING => flags |= FileFlags::OPEN_EXISTING,
            OPEN_ALWAYS => flags |= FileFlags::OPEN_ALWAYS,
            TRUNCATE_EXISTING => flags |= FileFlags::TRUNCATE_EXISTING,
            _ => return Err(TranslationError::InvalidCreationDisposition),
        }
        
        Ok(flags)
    }

    fn translate_memory_flags(&self, protect: DWORD) -> Result<MemoryFlags, TranslationError> {
        let mut flags = MemoryFlags::empty();
        
        match protect {
            PAGE_EXECUTE_READWRITE => {
                flags |= MemoryFlags::READ | MemoryFlags::WRITE | MemoryFlags::EXECUTE;
            }
            PAGE_EXECUTE_READ => {
                flags |= MemoryFlags::READ | MemoryFlags::EXECUTE;
            }
            PAGE_READWRITE => {
                flags |= MemoryFlags::READ | MemoryFlags::WRITE;
            }
            PAGE_READONLY => {
                flags |= MemoryFlags::READ;
            }
            _ => return Err(TranslationError::InvalidMemoryProtection),
        }
        
        Ok(flags)
    }
}
```

### Day 4: Registry Emulation

**Tasks:**
1. Implement registry storage
2. Handle registry operations
3. Add persistence
4. Implement registry virtualization

**Code Structure:**
```rust
// src/legacy/registry.rs
use vantis_core::fs::FileSystem;

pub struct Registry {
    root_key: HKEY,
    keys: HashMap<String, RegistryKey>,
    fs: Arc<FileSystem>,
}

#[derive(Clone)]
pub struct RegistryKey {
    name: String,
    subkeys: HashMap<String, HKEY>,
    values: HashMap<String, RegistryValue>,
}

#[derive(Clone)]
pub enum RegistryValue {
    DWord(u32),
    QWord(u64),
    String(String),
    MultiString(Vec<String>),
    Binary(Vec<u8>),
}

impl Registry {
    pub fn new(fs: Arc<FileSystem>) -> Result<Self, RegistryError> {
        let mut registry = Registry {
            root_key: HKEY_CLASSES_ROOT,
            keys: HashMap::new(),
            fs,
        };
        
        // Load registry from disk
        registry.load_registry()?;
        
        // Create default keys
        registry.create_default_keys()?;
        
        Ok(registry)
    }

    pub fn create_key(
        &mut self,
        hkey: HKEY,
        sub_key: &str,
        class: Option<&str>,
        options: DWORD,
        sam_desired: DWORD,
        security_attributes: Option<&SECURITY_ATTRIBUTES>,
        phk_result: &mut HKEY,
        disposition: &mut DWORD,
    ) -> Result<(), RegistryError> {
        // Create or open key
        let key_path = self.get_key_path(hkey, sub_key)?;
        
        if self.keys.contains_key(&key_path) {
            *disposition = REG_OPENED_EXISTING_KEY;
        } else {
            let new_key = RegistryKey {
                name: sub_key.to_string(),
                subkeys: HashMap::new(),
                values: HashMap::new(),
            };
            
            self.keys.insert(key_path.clone(), new_key);
            *disposition = REG_CREATED_NEW_KEY;
        }
        
        *phk_result = self.get_key_handle(&key_path)?;
        
        Ok(())
    }

    pub fn open_key(
        &mut self,
        hkey: HKEY,
        sub_key: &str,
        options: DWORD,
        sam_desired: DWORD,
        phk_result: &mut HKEY,
    ) -> Result<(), RegistryError> {
        let key_path = self.get_key_path(hkey, sub_key)?;
        
        if !self.keys.contains_key(&key_path) {
            return Err(RegistryError::KeyNotFound);
        }
        
        *phk_result = self.get_key_handle(&key_path)?;
        
        Ok(())
    }

    pub fn close_key(&mut self, hkey: HKEY) -> Result<(), RegistryError> {
        // Close key handle
        Ok(())
    }

    pub fn query_value(
        &self,
        hkey: HKEY,
        value_name: &str,
        reserved: Option<LPVOID>,
        value_type: &mut DWORD,
        data: &mut [u8],
        data_size: &mut DWORD,
    ) -> Result<(), RegistryError> {
        let key = self.get_key_from_handle(hkey)?;
        
        let value = key.values.get(value_name)
            .ok_or(RegistryError::ValueNotFound)?;
        
        match value {
            RegistryValue::DWord(v) => {
                *value_type = REG_DWORD;
                let bytes = v.to_le_bytes();
                data[..4].copy_from_slice(&bytes);
                *data_size = 4;
            }
            RegistryValue::String(v) => {
                *value_type = REG_SZ;
                let bytes = v.as_bytes();
                let len = bytes.len().min(data.len() - 1);
                data[..len].copy_from_slice(&bytes[..len]);
                data[len] = 0;
                *data_size = (len + 1) as DWORD;
            }
            RegistryValue::Binary(v) => {
                *value_type = REG_BINARY;
                let len = v.len().min(data.len());
                data[..len].copy_from_slice(&v[..len]);
                *data_size = len as DWORD;
            }
            _ => {
                return Err(RegistryError::UnsupportedValueType);
            }
        }
        
        Ok(())
    }

    pub fn set_value(
        &mut self,
        hkey: HKEY,
        value_name: &str,
        reserved: Option<LPVOID>,
        value_type: DWORD,
        data: &[u8],
        data_size: DWORD,
    ) -> Result<(), RegistryError> {
        let key = self.get_key_from_handle_mut(hkey)?;
        
        let value = match value_type {
            REG_DWORD => {
                let v = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                RegistryValue::DWord(v)
            }
            REG_SZ => {
                let v = String::from_utf8_lossy(&data[..data_size as usize]).to_string();
                RegistryValue::String(v)
            }
            REG_BINARY => {
                RegistryValue::Binary(data[..data_size as usize].to_vec())
            }
            _ => {
                return Err(RegistryError::UnsupportedValueType);
            }
        };
        
        key.values.insert(value_name.to_string(), value);
        
        Ok(())
    }

    pub fn delete_key(&mut self, hkey: HKEY, sub_key: &str) -> Result<(), RegistryError> {
        let key_path = self.get_key_path(hkey, sub_key)?;
        
        if !self.keys.contains_key(&key_path) {
            return Err(RegistryError::KeyNotFound);
        }
        
        self.keys.remove(&key_path);
        
        Ok(())
    }

    pub fn delete_value(&mut self, hkey: HKEY, value_name: &str) -> Result<(), RegistryError> {
        let key = self.get_key_from_handle_mut(hkey)?;
        
        if !key.values.contains_key(value_name) {
            return Err(RegistryError::ValueNotFound);
        }
        
        key.values.remove(value_name);
        
        Ok(())
    }

    fn create_default_keys(&mut self) -> Result<(), RegistryError> {
        // Create HKEY_CLASSES_ROOT
        self.keys.insert(
            "HKEY_CLASSES_ROOT".to_string(),
            RegistryKey::new("HKEY_CLASSES_ROOT")
        );
        
        // Create HKEY_CURRENT_USER
        self.keys.insert(
            "HKEY_CURRENT_USER".to_string(),
            RegistryKey::new("HKEY_CURRENT_USER")
        );
        
        // Create HKEY_LOCAL_MACHINE
        self.keys.insert(
            "HKEY_LOCAL_MACHINE".to_string(),
            RegistryKey::new("HKEY_LOCAL_MACHINE")
        );
        
        // Create HKEY_USERS
        self.keys.insert(
            "HKEY_USERS".to_string(),
            RegistryKey::new("HKEY_USERS")
        );
        
        Ok(())
    }

    fn load_registry(&mut self) -> Result<(), RegistryError> {
        // Load registry from VantisFS
        let registry_path = PathBuf::from("/system/registry.dat");
        
        if self.fs.exists(&registry_path) {
            let data = self.fs.read_file(&registry_path)?;
            // Deserialize registry
            // Implementation details
        }
        
        Ok(())
    }

    fn save_registry(&self) -> Result<(), RegistryError> {
        // Save registry to VantisFS
        let registry_path = PathBuf::from("/system/registry.dat");
        
        // Serialize registry
        let data = self.serialize_registry()?;
        
        self.fs.write_file(&registry_path, &data)?;
        
        Ok(())
    }

    fn get_key_path(&self, hkey: HKEY, sub_key: &str) -> Result<String, RegistryError> {
        let root = match hkey {
            HKEY_CLASSES_ROOT => "HKEY_CLASSES_ROOT",
            HKEY_CURRENT_USER => "HKEY_CURRENT_USER",
            HKEY_LOCAL_MACHINE => "HKEY_LOCAL_MACHINE",
            HKEY_USERS => "HKEY_USERS",
            _ => return Err(RegistryError::InvalidRootKey),
        };
        
        Ok(format!("{}/{}", root, sub_key))
    }

    fn get_key_handle(&self, key_path: &str) -> Result<HKEY, RegistryError> {
        // Generate handle from key path
        Ok(generate_hkey(key_path))
    }

    fn get_key_from_handle(&self, hkey: HKEY) -> Result<&RegistryKey, RegistryError> {
        let key_path = self.get_key_path_from_handle(hkey)?;
        self.keys.get(&key_path)
            .ok_or(RegistryError::KeyNotFound)
    }

    fn get_key_from_handle_mut(&mut self, hkey: HKEY) -> Result<&mut RegistryKey, RegistryError> {
        let key_path = self.get_key_path_from_handle(hkey)?;
        self.keys.get_mut(&key_path)
            .ok_or(RegistryError::KeyNotFound)
    }

    fn get_key_path_from_handle(&self, hkey: HKEY) -> Result<String, RegistryError> {
        // Convert handle to key path
        // Implementation details
        Ok(String::new())
    }

    fn serialize_registry(&self) -> Result<Vec<u8>, RegistryError> {
        // Serialize registry to bytes
        // Implementation details
        Ok(Vec::new())
    }
}

impl RegistryKey {
    pub fn new(name: &str) -> Self {
        RegistryKey {
            name: name.to_string(),
            subkeys: HashMap::new(),
            values: HashMap::new(),
        }
    }
}
```

### Day 5: Integration and Testing

**Tasks:**
1. Integrate all components
2. Implement sandboxing
3. Add security measures
4. Comprehensive testing

**Code Structure:**
```rust
// src/legacy/airlock.rs
use crate::pe_loader::PeLoader;
use crate::win32_api::Win32Api;
use crate::syscall_translation::SyscallTranslator;
use crate::registry::Registry;

pub struct LegacyAirlock {
    pe_loader: Arc<PeLoader>,
    win32_api: Arc<Win32Api>,
    syscall_translator: Arc<SyscallTranslator>,
    registry: Arc<Registry>,
    sandbox: Sandbox,
}

impl LegacyAirlock {
    pub fn new(
        memory_manager: Arc<MemoryManager>,
        scheduler: Arc<Scheduler>,
        graphics_backend: Arc<GraphicsBackend>,
        syscall_handler: Arc<SyscallHandler>,
        fs: Arc<FileSystem>,
    ) -> Result<Self, AirlockError> {
        // Create components
        let pe_loader = Arc::new(PeLoader::new(memory_manager.clone())?);
        let win32_api = Arc::new(Win32Api::new(graphics_backend)?);
        let syscall_translator = Arc::new(SyscallTranslator::new(syscall_handler)?);
        let registry = Arc::new(Registry::new(fs)?);
        let sandbox = Sandbox::new()?;
        
        Ok(LegacyAirlock {
            pe_loader,
            win32_api,
            syscall_translator,
            registry,
            sandbox,
        })
    }

    pub fn run_exe(&mut self, exe_path: &Path) -> Result<(), AirlockError> {
        // Enter sandbox
        self.sandbox.enter()?;
        
        // Load executable
        let module = self.pe_loader.load_exe(exe_path)?;
        
        // Create process
        let process = self.create_process(&module)?;
        
        // Execute
        process.execute()?;
        
        // Exit sandbox
        self.sandbox.exit()?;
        
        Ok(())
    }

    fn create_process(&self, module: &LoadedModule) -> Result<Process, AirlockError> {
        // Create process with Win32 environment
        let process = Process::new(
            module.base_address,
            module.entry_point,
            self.win32_api.clone(),
            self.syscall_translator.clone(),
            self.registry.clone(),
        )?;
        
        Ok(process)
    }
}

pub struct Process {
    base_address: u64,
    entry_point: u64,
    win32_api: Arc<Win32Api>,
    syscall_translator: Arc<SyscallTranslator>,
    registry: Arc<Registry>,
}

impl Process {
    pub fn new(
        base_address: u64,
        entry_point: u64,
        win32_api: Arc<Win32Api>,
        syscall_translator: Arc<SyscallTranslator>,
        registry: Arc<Registry>,
    ) -> Result<Self, AirlockError> {
        Ok(Process {
            base_address,
            entry_point,
            win32_api,
            syscall_translator,
            registry,
        })
    }

    pub fn execute(&mut self) -> Result<(), AirlockError> {
        // Jump to entry point
        // Implementation details
        Ok(())
    }
}
```

---

## Security Considerations

### Sandbox Isolation

```rust
// src/legacy/security/sandbox.rs
pub struct Sandbox {
    namespace: Namespace,
    seccomp_filter: SeccompFilter,
    capabilities: Capabilities,
    network_isolated: bool,
}

impl Sandbox {
    pub fn new() -> Result<Self, SandboxError> {
        Ok(Sandbox {
            namespace: Namespace::new(
                NamespaceFlags::PID | 
                NamespaceFlags::NET | 
                NamespaceFlags::MOUNT | 
                NamespaceFlags::UTS
            )?,
            seccomp_filter: SeccompFilter::from_rules(WINDOWS_SECCOMP_RULES)?,
            capabilities: Capabilities::none(),
            network_isolated: true,
        })
    }

    pub fn enter(&self) -> Result<(), SandboxError> {
        // Enter namespace
        self.namespace.enter()?;
        
        // Apply seccomp filter
        self.seccomp_filter.apply()?;
        
        // Drop all capabilities
        self.capabilities.drop_all()?;
        
        // Isolate network
        if self.network_isolated {
            self.isolate_network()?;
        }
        
        Ok(())
    }

    pub fn exit(&self) -> Result<(), SandboxError> {
        // Exit sandbox
        Ok(())
    }

    fn isolate_network(&self) -> Result<(), SandboxError> {
        // Create isolated network namespace
        // Implementation details
        Ok(())
    }
}

const WINDOWS_SECCOMP_RULES: &[SeccompRule] = &[
    SeccompRule::allow(syscall::read),
    SeccompRule::allow(syscall::write),
    SeccompRule::allow(syscall::mmap),
    SeccompRule::allow(syscall::munmap),
    // Windows-specific syscalls
    SeccompRule::allow(syscall::socket),
    SeccompRule::allow(syscall::connect),
];
```

---

## Performance Targets

### Startup Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| EXE Loading | <500ms | Time from load to ready |
| DLL Loading | <100ms per DLL | Time to load each DLL |
| Process Creation | <200ms | Time to create process |
| First Window | <1s | Time to first window visible |

### Runtime Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| API Call Overhead | <10μs | Win32 API call latency |
| Syscall Translation | <5μs | Syscall translation overhead |
| Memory Overhead | <100MB | Additional memory usage |
| CPU Overhead | <5% | Additional CPU usage |

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_loader_creation() {
        let memory_manager = Arc::new(MemoryManager::new());
        let loader = PeLoader::new(memory_manager);
        assert!(loader.is_ok());
    }

    #[test]
    fn test_win32_api_creation() {
        let graphics_backend = Arc::new(GraphicsBackend::new());
        let api = Win32Api::new(graphics_backend);
        assert!(api.is_ok());
    }

    #[test]
    fn test_registry_operations() {
        let fs = Arc::new(FileSystem::new());
        let mut registry = Registry::new(fs).unwrap();
        
        let mut hkey = HKEY(0);
        let mut disposition = 0;
        
        let result = registry.create_key(
            HKEY_CURRENT_USER,
            "Software\\Test",
            None,
            0,
            KEY_ALL_ACCESS,
            None,
            &mut hkey,
            &mut disposition,
        );
        
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_exe_execution() {
        let airlock = create_test_airlock();
        let exe_path = PathBuf::from("test_data/test.exe");
        
        let result = airlock.run_exe(&exe_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_window_creation() {
        let mut api = create_test_win32_api();
        
        let hwnd = api.create_window_ex(
            0,
            "TestClass",
            "Test Window",
            WS_OVERLAPPEDWINDOW,
            100, 100, 800, 600,
            HWND(0),
            HMENU(0),
            HINSTANCE(0),
            ptr::null_mut(),
        );
        
        assert!(hwnd.is_ok());
    }
}
```

---

## Code Examples

### Running a Windows Executable

```rust
use vantis_legacy::LegacyAirlock;

fn main() -> Result<(), Box<dyn Error>> {
    // Create airlock
    let airlock = LegacyAirlock::new(
        memory_manager,
        scheduler,
        graphics_backend,
        syscall_handler,
        fs,
    )?;

    // Run executable
    airlock.run_exe(&PathBuf::from("app.exe"))?;

    Ok(())
}
```

### Creating a Window

```rust
use vantis_legacy::Win32Api;

fn create_window(api: &mut Win32Api) -> Result<HWND, Box<dyn Error>> {
    let hwnd = api.create_window_ex(
        0,
        "MyWindowClass",
        "My Window",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        800,
        600,
        HWND(0),
        HMENU(0),
        HINSTANCE(0),
        ptr::null_mut(),
    )?;

    api.show_window(hwnd, SW_SHOW)?;
    api.update_window(hwnd)?;

    Ok(hwnd)
}
```

---

## Troubleshooting

### Common Issues

**Issue: EXE fails to load with "Invalid PE format"**
- **Solution**: Verify PE file format and architecture
- **Command**: `file app.exe`

**Issue: DLL not found**
- **Solution**: Check DLL search path and dependencies
- **Command**: `ldd app.exe`

**Issue: Window creation fails**
- **Solution**: Verify graphics backend initialization
- **Command**: `vantis-legacy check-gpu`

**Issue: Registry access denied**
- **Solution**: Check registry permissions and virtualization
- **Command**: `vantis-legacy registry list`

---

## Conclusion

This implementation guide provides a comprehensive plan for integrating Windows executable support into VantisOS through the Legacy Airlock subsystem. The 5-day timeline covers all critical components including PE loading, Win32 API emulation, syscall translation, and registry emulation.

**Key Success Metrics:**
- ✅ Windows 10/11 compatibility
- ✅ Secure sandboxing
- ✅ <1s first window time
- ✅ <10μs API call overhead
- ✅ <5% CPU overhead

**Next Steps:**
1. Begin implementation following the 5-day plan
2. Set up testing environment with sample executables
3. Integrate with VantisOS build system
4. Conduct security audit
5. Performance optimization

---

**Document Version**: 1.0  
**Last Updated**: February 24, 2025  
**Author**: SuperNinja AI Agent  
**Status**: Implementation Guide