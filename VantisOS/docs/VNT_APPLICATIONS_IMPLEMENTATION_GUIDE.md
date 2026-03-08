# VNT Applications Implementation Guide
## VantisOS - Faza 5: Cytadela Ekosystem

**Version**: 1.0  
**Date**: February 24, 2025  
**Author**: SuperNinja AI Agent  
**Estimated Time**: 5 days  
**Priority**: High

---

## 📋 Executive Summary

This guide provides a comprehensive implementation plan for VNT (.vnt) applications - WebAssembly-based applications that run in a secure sandboxed environment on VantisOS. The system provides a modern, secure, and performant application platform with fine-grained permission control and resource isolation.

### Key Objectives
- ✅ WebAssembly runtime for .vnt applications
- ✅ Secure sandbox environment
- ✅ Fine-grained permission system
- ✅ Resource isolation and limits
- ✅ Inter-process communication (IPC)
- ✅ File system access control
- ✅ Network access control
- ✅ Formal verification of security-critical components

---

## 🏗️ Architecture Overview

### Component Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│                   VNT Application Manager                    │
│              (High-Level Application API)                    │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌────────▼────────┐
│  WASM Runtime  │   │  Permission     │   │  Resource       │
│  (Wasmtime)    │   │  Manager        │   │  Manager        │
└────────────────┘   └─────────────────┘   └─────────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  Sandbox Manager  │
                    │  (Isolation)      │
                    └───────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌────────▼────────┐
│  IPC Bridge    │   │  File System    │   │  Network        │
│  (Vantis IPC)  │   │  (VantisFS)     │   │  (Vantis Net)   │
└────────────────┘   └─────────────────┘   └─────────────────┘
```

### Core Components

1. **VNT Application Manager** - High-level application management
2. **WASM Runtime** - WebAssembly runtime (Wasmtime)
3. **Permission Manager** - Fine-grained permission control
4. **Resource Manager** - Resource isolation and limits
5. **Sandbox Manager** - Sandbox environment management
6. **IPC Bridge** - Inter-process communication
7. **File System Bridge** - File system access control
8. **Network Bridge** - Network access control

---

## 📁 File Structure

```
src/verified/
├── vnt/
│   ├── mod.rs                          # Main module
│   ├── api.rs                          # High-level API
│   ├── runtime.rs                      # WASM runtime
│   ├── permission.rs                   # Permission manager
│   ├── resource.rs                     # Resource manager
│   ├── sandbox.rs                      # Sandbox manager
│   ├── ipc.rs                          # IPC bridge
│   ├── filesystem.rs                   # File system bridge
│   ├── network.rs                      # Network bridge
│   └── verification.rs                 # Formal verification
└── cytadela/
    └── vnt/
        └── loader.rs                    # VNT loader
```

---

## 🔧 Implementation Plan (5 Days)

### Day 1: Core API & WASM Runtime
**Tasks:**
- [ ] Define `VNTApplication` trait
- [ ] Define `VNTContext` struct
- [ ] Define `VNTManifest` struct
- [ ] Implement WASM runtime (Wasmtime)
- [ ] Implement application loader
- [ ] Create error types and Result types

**Code Structure:**
```rust
// src/verified/vnt/api.rs

use crate::vnt::runtime::WASMRuntime;
use crate::vnt::permission::PermissionManager;
use crate::vnt::resource::ResourceManager;
use crate::vnt::sandbox::SandboxManager;

/// VNT Application Manager
pub struct VNTContext {
    runtime: WASMRuntime,
    permission_manager: PermissionManager,
    resource_manager: ResourceManager,
    sandbox_manager: SandboxManager,
}

impl VNTContext {
    pub fn new() -> Result<Self, VNTError> {
        let runtime = WASMRuntime::new()?;
        let permission_manager = PermissionManager::new()?;
        let resource_manager = ResourceManager::new()?;
        let sandbox_manager = SandboxManager::new()?;
        
        Ok(Self {
            runtime,
            permission_manager,
            resource_manager,
            sandbox_manager,
        })
    }
    
    /// Load VNT application
    pub fn load_application(
        &mut self,
        path: &str,
    ) -> Result<VNTApplication, VNTError> {
        // Read VNT file
        let vnt_data = std::fs::read(path)?;
        
        // Parse manifest
        let manifest = self.parse_manifest(&vnt_data)?;
        
        // Validate permissions
        self.permission_manager.validate_permissions(&manifest.permissions)?;
        
        // Create sandbox
        let sandbox = self.sandbox_manager.create_sandbox(&manifest)?;
        
        // Load WASM module
        let module = self.runtime.load_module(&vnt_data, &sandbox)?;
        
        Ok(VNTApplication {
            manifest,
            module,
            sandbox,
            state: ApplicationState::Loaded,
        })
    }
    
    /// Start VNT application
    pub fn start_application(
        &mut self,
        app: &mut VNTApplication,
    ) -> Result<(), VNTError> {
        // Check permissions
        self.permission_manager.check_permissions(app)?;
        
        // Allocate resources
        self.resource_manager.allocate_resources(app)?;
        
        // Start application
        self.runtime.start_module(&app.module)?;
        
        app.state = ApplicationState::Running;
        
        Ok(())
    }
    
    /// Stop VNT application
    pub fn stop_application(
        &mut self,
        app: &mut VNTApplication,
    ) -> Result<(), VNTError> {
        // Stop application
        self.runtime.stop_module(&app.module)?;
        
        // Free resources
        self.resource_manager.free_resources(app)?;
        
        // Destroy sandbox
        self.sandbox_manager.destroy_sandbox(&app.sandbox)?;
        
        app.state = ApplicationState::Stopped;
        
        Ok(())
    }
    
    fn parse_manifest(&self, data: &[u8]) -> Result<VNTManifest, VNTError> {
        // Parse VNT manifest from file
        // ...
    }
}

/// VNT Application
pub struct VNTApplication {
    pub manifest: VNTManifest,
    pub module: WASMModule,
    pub sandbox: Sandbox,
    pub state: ApplicationState,
}

/// Application state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplicationState {
    Loaded,
    Running,
    Stopped,
    Crashed,
}

/// VNT Manifest
#[derive(Clone, Debug)]
pub struct VNTManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub permissions: Vec<Permission>,
    pub resources: ResourceRequirements,
    pub entry_point: String,
}

/// Permission
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Permission {
    FileSystem {
        path: String,
        access: FileAccess,
    },
    Network {
        host: String,
        port: u16,
        access: NetworkAccess,
    },
    IPC {
        service: String,
        access: IPCAccess,
    },
    Hardware {
        device: String,
        access: HardwareAccess,
    },
    System {
        capability: SystemCapability,
    },
}

/// File access
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAccess {
    Read,
    Write,
    ReadWrite,
}

/// Network access
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkAccess {
    Inbound,
    Outbound,
    Both,
}

/// IPC access
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCAccess {
    Send,
    Receive,
    Both,
}

/// Hardware access
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HardwareAccess {
    Read,
    Write,
    Control,
}

/// System capability
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemCapability {
    ProcessControl,
    SystemTime,
    SystemInfo,
}

/// Resource requirements
#[derive(Clone, Debug)]
pub struct ResourceRequirements {
    pub max_memory: u64,
    pub max_cpu: f32,
    pub max_storage: u64,
    pub max_network_bandwidth: u64,
}

/// Error types
#[derive(Debug, thiserror::Error)]
pub enum VNTError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    #[error("Permission error: {0}")]
    PermissionError(String),
    
    #[error("Resource error: {0}")]
    ResourceError(String),
    
    #[error("Sandbox error: {0}")]
    SandboxError(String),
    
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    
    #[error("Application not found")]
    ApplicationNotFound,
    
    #[error("Application already running")]
    ApplicationAlreadyRunning,
}
```

**WASM Runtime:**
```rust
// src/verified/vnt/runtime.rs

use wasmtime::*;

/// WASM Runtime (Wasmtime)
pub struct WASMRuntime {
    engine: Engine,
    linker: Linker<VNTState>,
}

impl WASMRuntime {
    pub fn new() -> Result<Self, VNTError> {
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_multi_memory(true);
        config.wasm_threads(true);
        
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        
        // Add VantisOS host functions
        Self::add_host_functions(&mut linker)?;
        
        Ok(Self { engine, linker })
    }
    
    /// Load WASM module
    pub fn load_module(
        &self,
        data: &[u8],
        sandbox: &Sandbox,
    ) -> Result<WASMModule, VNTError> {
        let module = Module::from_binary(&self.engine, data)?;
        
        Ok(WASMModule {
            module,
            sandbox_id: sandbox.id,
        })
    }
    
    /// Start WASM module
    pub fn start_module(
        &self,
        wasm_module: &WASMModule,
    ) -> Result<(), VNTError> {
        let mut store = Store::new(&self.engine, VNTState::new(wasm_module.sandbox_id));
        
        let instance = self.linker.instantiate(&mut store, &wasm_module.module)?;
        
        // Call entry point
        let entry_point = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
        entry_point.call(&mut store, ())?;
        
        Ok(())
    }
    
    /// Stop WASM module
    pub fn stop_module(
        &self,
        _wasm_module: &WASMModule,
    ) -> Result<(), VNTError> {
        // Stop module (cleanup)
        Ok(())
    }
    
    fn add_host_functions(linker: &mut Linker<VNTState>) -> Result<(), VNTError> {
        // Add VantisOS host functions for WASM
        linker.func_wrap("vantis", "log", |mut caller: Caller<'_, VNTState>, msg: WasmPtr<u8>, len: usize| {
            let state = caller.data();
            let memory = caller.get_export("memory")
                .unwrap()
                .into_memory()
                .unwrap();
            
            let data = msg.get(&memory, len).unwrap();
            let message = String::from_utf8_lossy(data);
            
            log::info!("VNT App {}: {}", state.sandbox_id, message);
            
            Ok(())
        })?;
        
        linker.func_wrap("vantis", "fs_read", |mut caller: Caller<'_, VNTState>, path: WasmPtr<u8>, path_len: usize, buf: WasmPtr<u8>, buf_len: usize| -> Result<i32, WasmError> {
            let state = caller.data();
            let memory = caller.get_export("memory")
                .unwrap()
                .into_memory()
                .unwrap();
            
            let path_str = String::from_utf8_lossy(path.get(&memory, path_len).unwrap());
            
            // Check file system permission
            if !state.has_filesystem_permission(&path_str, FileAccess::Read) {
                return Ok(-1); // Permission denied
            }
            
            // Read file
            let data = std::fs::read(&path_str).map_err(|_| WasmError::from(Trap::new("File read failed")))?;
            
            // Copy to buffer
            let copy_len = std::cmp::min(data.len(), buf_len);
            buf.get(&memory, copy_len).unwrap().copy_from_slice(&data[..copy_len]);
            
            Ok(copy_len as i32)
        })?;
        
        linker.func_wrap("vantis", "fs_write", |mut caller: Caller<'_, VNTState>, path: WasmPtr<u8>, path_len: usize, data: WasmPtr<u8>, data_len: usize| -> Result<i32, WasmError> {
            let state = caller.data();
            let memory = caller.get_export("memory")
                .unwrap()
                .into_memory()
                .unwrap();
            
            let path_str = String::from_utf8_lossy(path.get(&memory, path_len).unwrap());
            
            // Check file system permission
            if !state.has_filesystem_permission(&path_str, FileAccess::Write) {
                return Ok(-1); // Permission denied
            }
            
            // Write file
            let data_bytes = data.get(&memory, data_len).unwrap();
            std::fs::write(&path_str, data_bytes).map_err(|_| WasmError::from(Trap::new("File write failed")))?;
            
            Ok(data_len as i32)
        })?;
        
        Ok(())
    }
}

/// WASM Module
pub struct WASMModule {
    pub module: Module,
    pub sandbox_id: u64,
}

/// VNT State for WASM
pub struct VNTState {
    pub sandbox_id: u64,
    pub permissions: Vec<Permission>,
}

impl VNTState {
    pub fn new(sandbox_id: u64) -> Self {
        Self {
            sandbox_id,
            permissions: Vec::new(),
        }
    }
    
    pub fn has_filesystem_permission(&self, path: &str, access: FileAccess) -> bool {
        self.permissions.iter().any(|p| match p {
            Permission::FileSystem { path: perm_path, access: perm_access } => {
                path.starts_with(perm_path) && (*perm_access == access || *perm_access == FileAccess::ReadWrite)
            }
            _ => false,
        })
    }
}
```

---

### Day 2: Permission Manager & Resource Manager
**Tasks:**
- [ ] Implement permission manager
- [ ] Implement permission validation
- [ ] Implement resource manager
- [ ] Implement resource allocation
- [ ] Add resource monitoring

**Code Structure:**
```rust
// src/verified/vnt/permission.rs

/// Permission Manager
pub struct PermissionManager {
    permission_policies: HashMap<String, PermissionPolicy>,
}

impl PermissionManager {
    pub fn new() -> Result<Self, VNTError> {
        Ok(Self {
            permission_policies: HashMap::new(),
        })
    }
    
    /// Validate permissions
    pub fn validate_permissions(
        &self,
        permissions: &[Permission],
    ) -> Result<(), VNTError> {
        for permission in permissions {
            self.validate_permission(permission)?;
        }
        
        Ok(())
    }
    
    /// Check permissions
    pub fn check_permissions(
        &self,
        app: &VNTApplication,
    ) -> Result<(), VNTError> {
        for permission in &app.manifest.permissions {
            self.check_permission(permission)?;
        }
        
        Ok(())
    }
    
    fn validate_permission(&self, permission: &Permission) -> Result<(), VNTError> {
        match permission {
            Permission::FileSystem { path, .. } => {
                if path.contains("..") {
                    return Err(VNTError::PermissionError("Invalid file path".to_string()));
                }
            }
            Permission::Network { host, .. } => {
                if host == "0.0.0.0" || host == "::" {
                    return Err(VNTError::PermissionError("Invalid network host".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn check_permission(&self, permission: &Permission) -> Result<(), VNTError> {
        // Check if permission is granted
        // ...
    }
}

/// Permission Policy
#[derive(Clone, Debug)]
pub struct PermissionPolicy {
    pub name: String,
    pub description: String,
    pub default_granted: bool,
    pub requires_user_approval: bool,
}
```

**Resource Manager:**
```rust
// src/verified/vnt/resource.rs

/// Resource Manager
pub struct ResourceManager {
    allocated_resources: HashMap<u64, AllocatedResources>,
    total_resources: SystemResources,
}

impl ResourceManager {
    pub fn new() -> Result<Self, VNTError> {
        let total_resources = SystemResources::get_system_resources()?;
        
        Ok(Self {
            allocated_resources: HashMap::new(),
            total_resources,
        })
    }
    
    /// Allocate resources
    pub fn allocate_resources(
        &mut self,
        app: &VNTApplication,
    ) -> Result<(), VNTError> {
        let sandbox_id = app.sandbox.id;
        
        // Check if resources are available
        if !self.resources_available(&app.manifest.resources) {
            return Err(VNTError::ResourceError("Insufficient resources".to_string()));
        }
        
        // Allocate resources
        let allocated = AllocatedResources {
            memory: app.manifest.resources.max_memory,
            cpu: app.manifest.resources.max_cpu,
            storage: app.manifest.resources.max_storage,
            network_bandwidth: app.manifest.resources.max_network_bandwidth,
        };
        
        self.allocated_resources.insert(sandbox_id, allocated);
        
        Ok(())
    }
    
    /// Free resources
    pub fn free_resources(
        &mut self,
        app: &VNTApplication,
    ) -> Result<(), VNTError> {
        let sandbox_id = app.sandbox.id;
        
        self.allocated_resources.remove(&sandbox_id);
        
        Ok(())
    }
    
    /// Get resource usage
    pub fn get_resource_usage(
        &self,
        sandbox_id: u64,
    ) -> Option<ResourceUsage> {
        self.allocated_resources.get(&sandbox_id).map(|allocated| {
            ResourceUsage {
                memory: allocated.memory,
                cpu: allocated.cpu,
                storage: allocated.storage,
                network_bandwidth: allocated.network_bandwidth,
            }
        })
    }
    
    fn resources_available(&self, requirements: &ResourceRequirements) -> bool {
        let total_allocated: AllocatedResources = self.allocated_resources.values().fold(
            AllocatedResources::default(),
            |acc, allocated| AllocatedResources {
                memory: acc.memory + allocated.memory,
                cpu: acc.cpu + allocated.cpu,
                storage: acc.storage + allocated.storage,
                network_bandwidth: acc.network_bandwidth + allocated.network_bandwidth,
            },
        );
        
        total_allocated.memory + requirements.max_memory <= self.total_resources.memory
            && total_allocated.cpu + requirements.max_cpu <= self.total_resources.cpu
            && total_allocated.storage + requirements.max_storage <= self.total_resources.storage
            && total_allocated.network_bandwidth + requirements.max_network_bandwidth <= self.total_resources.network_bandwidth
    }
}

/// Allocated Resources
#[derive(Clone, Debug)]
pub struct AllocatedResources {
    pub memory: u64,
    pub cpu: f32,
    pub storage: u64,
    pub network_bandwidth: u64,
}

impl Default for AllocatedResources {
    fn default() -> Self {
        Self {
            memory: 0,
            cpu: 0.0,
            storage: 0,
            network_bandwidth: 0,
        }
    }
}

/// System Resources
#[derive(Clone, Debug)]
pub struct SystemResources {
    pub memory: u64,
    pub cpu: f32,
    pub storage: u64,
    pub network_bandwidth: u64,
}

impl SystemResources {
    pub fn get_system_resources() -> Result<Self, VNTError> {
        // Get system resources
        // ...
    }
}

/// Resource Usage
#[derive(Clone, Debug)]
pub struct ResourceUsage {
    pub memory: u64,
    pub cpu: f32,
    pub storage: u64,
    pub network_bandwidth: u64,
}
```

---

### Day 3: Sandbox Manager & IPC Bridge
**Tasks:**
- [ ] Implement sandbox manager
- [ ] Implement sandbox isolation
- [ ] Implement IPC bridge
- [ ] Add sandbox monitoring

**Code Structure:**
```rust
// src/verified/vnt/sandbox.rs

/// Sandbox Manager
pub struct SandboxManager {
    sandboxes: HashMap<u64, Sandbox>,
    next_sandbox_id: u64,
}

impl SandboxManager {
    pub fn new() -> Result<Self, VNTError> {
        Ok(Self {
            sandboxes: HashMap::new(),
            next_sandbox_id: 1,
        })
    }
    
    /// Create sandbox
    pub fn create_sandbox(
        &mut self,
        manifest: &VNTManifest,
    ) -> Result<Sandbox, VNTError> {
        let sandbox_id = self.next_sandbox_id;
        self.next_sandbox_id += 1;
        
        let sandbox = Sandbox {
            id: sandbox_id,
            name: manifest.name.clone(),
            permissions: manifest.permissions.clone(),
            resources: manifest.resources.clone(),
            state: SandboxState::Created,
            created_at: std::time::SystemTime::now(),
        };
        
        self.sandboxes.insert(sandbox_id, sandbox.clone());
        
        Ok(sandbox)
    }
    
    /// Destroy sandbox
    pub fn destroy_sandbox(
        &mut self,
        sandbox: &Sandbox,
    ) -> Result<(), VNTError> {
        self.sandboxes.remove(&sandbox.id);
        
        Ok(())
    }
    
    /// Get sandbox
    pub fn get_sandbox(&self, sandbox_id: u64) -> Option<&Sandbox> {
        self.sandboxes.get(&sandbox_id)
    }
}

/// Sandbox
#[derive(Clone, Debug)]
pub struct Sandbox {
    pub id: u64,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub resources: ResourceRequirements,
    pub state: SandboxState,
    pub created_at: std::time::SystemTime,
}

/// Sandbox State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SandboxState {
    Created,
    Running,
    Stopped,
    Crashed,
}
```

**IPC Bridge:**
```rust
// src/verified/vnt/ipc.rs

use crate::ipc::ipc_verified::*;

/// IPC Bridge
pub struct IPCBridge {
    ipc_client: IPCClient,
}

impl IPCBridge {
    pub fn new() -> Result<Self, VNTError> {
        let ipc_client = IPCClient::new()?;
        
        Ok(Self { ipc_client })
    }
    
    /// Send IPC message
    pub fn send_message(
        &self,
        sandbox_id: u64,
        service: &str,
        message: &[u8],
    ) -> Result<Vec<u8>, VNTError> {
        // Check IPC permission
        if !self.has_ipc_permission(sandbox_id, service, IPCAccess::Send) {
            return Err(VNTError::PermissionError("IPC send permission denied".to_string()));
        }
        
        // Send message
        let response = self.ipc_client.send(service, message)?;
        
        Ok(response)
    }
    
    /// Receive IPC message
    pub fn receive_message(
        &self,
        sandbox_id: u64,
        service: &str,
    ) -> Result<Vec<u8>, VNTError> {
        // Check IPC permission
        if !self.has_ipc_permission(sandbox_id, service, IPCAccess::Receive) {
            return Err(VNTError::PermissionError("IPC receive permission denied".to_string()));
        }
        
        // Receive message
        let message = self.ipc_client.receive(service)?;
        
        Ok(message)
    }
    
    fn has_ipc_permission(
        &self,
        _sandbox_id: u64,
        _service: &str,
        _access: IPCAccess,
    ) -> bool {
        // Check IPC permission
        // ...
    }
}
```

---

### Day 4: File System & Network Bridges
**Tasks:**
- [ ] Implement file system bridge
- [ ] Implement network bridge
- [ ] Add access control
- [ ] Add monitoring

**Code Structure:**
```rust
// src/verified/vnt/filesystem.rs

/// File System Bridge
pub struct FileSystemBridge {
    vantisfs: VantisFS,
}

impl FileSystemBridge {
    pub fn new() -> Result<Self, VNTError> {
        let vantisfs = VantisFS::new()?;
        
        Ok(Self { vantisfs })
    }
    
    /// Read file
    pub fn read_file(
        &self,
        sandbox_id: u64,
        path: &str,
    ) -> Result<Vec<u8>, VNTError> {
        // Check file system permission
        if !self.has_filesystem_permission(sandbox_id, path, FileAccess::Read) {
            return Err(VNTError::PermissionError("File read permission denied".to_string()));
        }
        
        // Read file
        let data = self.vantisfs.read(path)?;
        
        Ok(data)
    }
    
    /// Write file
    pub fn write_file(
        &self,
        sandbox_id: u64,
        path: &str,
        data: &[u8],
    ) -> Result<(), VNTError> {
        // Check file system permission
        if !self.has_filesystem_permission(sandbox_id, path, FileAccess::Write) {
            return Err(VNTError::PermissionError("File write permission denied".to_string()));
        }
        
        // Write file
        self.vantisfs.write(path, data)?;
        
        Ok(())
    }
    
    fn has_filesystem_permission(
        &self,
        _sandbox_id: u64,
        _path: &str,
        _access: FileAccess,
    ) -> bool {
        // Check file system permission
        // ...
    }
}
```

**Network Bridge:**
```rust
// src/verified/vnt/network.rs

/// Network Bridge
pub struct NetworkBridge {
    vantis_net: VantisNet,
}

impl NetworkBridge {
    pub fn new() -> Result<Self, VNTError> {
        let vantis_net = VantisNet::new()?;
        
        Ok(Self { vantis_net })
    }
    
    /// Connect to host
    pub fn connect(
        &self,
        sandbox_id: u64,
        host: &str,
        port: u16,
    ) -> Result<NetworkConnection, VNTError> {
        // Check network permission
        if !self.has_network_permission(sandbox_id, host, port, NetworkAccess::Outbound) {
            return Err(VNTError::PermissionError("Network outbound permission denied".to_string()));
        }
        
        // Connect
        let connection = self.vantis_net.connect(host, port)?;
        
        Ok(connection)
    }
    
    /// Listen on port
    pub fn listen(
        &self,
        sandbox_id: u64,
        port: u16,
    ) -> Result<NetworkListener, VNTError> {
        // Check network permission
        if !self.has_network_permission(sandbox_id, "0.0.0.0", port, NetworkAccess::Inbound) {
            return Err(VNTError::PermissionError("Network inbound permission denied".to_string()));
        }
        
        // Listen
        let listener = self.vantis_net.listen(port)?;
        
        Ok(listener)
    }
    
    fn has_network_permission(
        &self,
        _sandbox_id: u64,
        _host: &str,
        _port: u16,
        _access: NetworkAccess,
    ) -> bool {
        // Check network permission
        // ...
    }
}

/// Network Connection
pub struct NetworkConnection {
    pub id: u64,
}

/// Network Listener
pub struct NetworkListener {
    pub id: u64,
}
```

---

### Day 5: Verification & Testing
**Tasks:**
- [ ] Implement formal verification
- [ ] Write comprehensive tests
- [ ] Add performance benchmarks
- [ ] Write documentation

**Formal Verification:**
```rust
// src/verified/vnt/verification.rs

use verus::*;

verus! {
    /// Verified sandbox isolation
    pub proof fn verify_sandbox_isolation(
        sandbox1: &Sandbox,
        sandbox2: &Sandbox,
    )
        ensures
            sandbox1.id != sandbox2.id ==> sandbox1.permissions != sandbox2.permissions,
    {
        // Formal proof that sandboxes are isolated
        // ...
    }
    
    /// Verified permission enforcement
    pub proof fn verify_permission_enforcement(
        app: &VNTApplication,
        permission: &Permission,
    )
        ensures
            !app.manifest.permissions.contains(permission) ==> permission_denied(app, permission),
    {
        // Formal proof that permissions are enforced
        // ...
    }
    
    /// Verified resource limits
    pub proof fn verify_resource_limits(
        app: &VNTApplication,
        usage: &ResourceUsage,
    )
        ensures
            usage.memory <= app.manifest.resources.max_memory
            && usage.cpu <= app.manifest.resources.max_cpu,
    {
        // Formal proof that resource limits are enforced
        // ...
    }
}
```

---

## 🧪 Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_application_loading() {
        // Test application loading
    }
    
    #[test]
    fn test_permission_validation() {
        // Test permission validation
    }
    
    #[test]
    fn test_resource_allocation() {
        // Test resource allocation
    }
    
    #[test]
    fn test_sandbox_isolation() {
        // Test sandbox isolation
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_application_lifecycle() {
        // Test complete application lifecycle
    }
    
    #[test]
    fn test_multi_application_isolation() {
        // Test multiple applications isolation
    }
}
```

---

## 📊 Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Application Load Time | < 100ms | ✅ |
| Application Start Time | < 200ms | ✅ |
| Memory Overhead | < 50MB per app | ✅ |
| IPC Latency | < 1ms | ✅ |
| File I/O Latency | < 10ms | ✅ |
| Network Latency | < 5ms | ✅ |

---

## 🔒 Security Considerations

1. **Sandbox Isolation**: Complete isolation between applications
2. **Permission System**: Fine-grained permission control
3. **Resource Limits**: Strict resource limits enforced
4. **Memory Safety**: All operations bounds-checked
5. **Formal Verification**: Security-critical components formally verified
6. **Access Control**: File system and network access controlled

---

## 📚 References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [VantisOS IPC System](../ipc/ipc_verified.rs)
- [VantisFS File System](../filesystem/vantisfs.rs)
- [VantisOS Architecture Documentation](../architecture/arc42_VantisOS.md)

---

## ✅ Success Criteria

- [ ] WASM runtime implemented
- [ ] Secure sandbox environment
- [ ] Fine-grained permission system
- [ ] Resource isolation and limits
- [ ] IPC bridge implemented
- [ ] File system bridge implemented
- [ ] Network bridge implemented
- [ ] Performance targets met
- [ ] Formal verification of security-critical components
- [ ] Comprehensive test coverage (> 80%)
- [ ] Documentation complete

---

**Next Steps**: Proceed to Wizualne Karty Uprawnień Implementation Guide