# VantisOS Compatibility Guide

## Overview

VantisOS provides comprehensive compatibility with multiple application ecosystems through three main compatibility layers:

1. **VNT Apps** - WebAssembly application system
2. **Android Subsystem** - Android compatibility layer
3. **Legacy Airlock** - Windows executable compatibility

---

## VNT Apps (WebAssembly Applications)

### Overview

VNT Apps is a native WebAssembly application system that allows developers to build and distribute cross-platform applications for VantisOS.

### Features

- **WebAssembly Runtime**: Full WASM support with Wasmtime
- **Package Management**: VNT Package Manager for installation and updates
- **Capability-Based Security**: Fine-grained permission system
- **Sandbox Isolation**: Resource-limited execution environment
- **VNT App Store**: Centralized application distribution

### Package Structure

```
app.vnt/
├── manifest.vnt      # Package manifest
├── app.wasm          # WebAssembly binary
├── assets/           # Application assets
└── resources/        # Additional resources
```

### Manifest Format

```toml
[package]
name = "com.example.app"
version = "1.0.0"
display_name = "Example App"
description = "Test application"
author = "Test Author"
license = "MIT"
min_vantis_os = "0.4.1"

[capabilities]
network = false
filesystem = true
hardware = false

[permissions]
network_access = false
file_system_access = true
hardware_access = false

[resources]
min_memory_mb = 64
min_storage_mb = 10
min_cpu_cores = 1

[ui]
width = 800
height = 600
resizable = true
```

### Installation

```bash
# Install VNT package
vntpm install app.vnt

# List installed packages
vntpm list

# Remove package
vntpm remove com.example.app

# Update package
vntpm update com.example.app
```

### Development

```rust
use vantisos::vnt_apps::*;

fn main() {
    let mut manager = VntPackageManager::new();
    
    // Install package
    manager.install_package("app.vnt")?;
    
    // Launch application
    manager.launch_app("com.example.app")?;
    
    Ok(())
}
```

---

## Android Subsystem

### Overview

The Android Subsystem provides full Android compatibility, allowing Android applications to run natively on VantisOS.

### Features

- **Android Runtime (ART)**: Full Android runtime with JIT compiler
- **Binder IPC**: Inter-process communication system
- **Hardware Abstraction Layer (HAL)**: 8 HAL modules for hardware access
- **Google Play Services**: Full Google Play Services integration
- **Android Sandbox**: SELinux-enforced sandbox isolation

### Supported Android Versions

- **Minimum SDK**: API 21 (Android 5.0 Lollipop)
- **Target SDK**: API 34 (Android 14)
- **Recommended**: API 28+ (Android 9+)

### APK Installation

```bash
# Install APK
android install app.apk

# List installed apps
android list

# Launch app
android launch com.example.app

# Uninstall app
android uninstall com.example.app
```

### Permissions

Android permissions are mapped to VantisOS capabilities:

| Android Permission | VantisOS Capability |
|-------------------|---------------------|
| INTERNET | network |
| READ_EXTERNAL_STORAGE | filesystem.read |
| WRITE_EXTERNAL_STORAGE | filesystem.write |
| CAMERA | hardware.camera |
| RECORD_AUDIO | hardware.microphone |

### Development

```rust
use vantisos::android_subsystem::*;

fn main() {
    let runtime = AndroidRuntime::new();
    
    // Load APK
    runtime.load_apk("app.apk")?;
    
    // Launch app
    runtime.launch_app("com.example.app.MainActivity")?;
    
    Ok(())
}
```

---

## Legacy Airlock (Windows Compatibility)

### Overview

Legacy Airlock provides Windows executable compatibility through Wine integration, allowing Windows applications to run on VantisOS.

### Features

- **Wine Integration**: Full Wine support with Wine Server
- **Windows API Translation**: Complete Windows API mapping
- **DLL Loading**: Native DLL support
- **Registry Emulation**: Windows registry emulation
- **Malware Scanning**: Automatic executable scanning

### Supported Windows Versions

- **Windows XP**: Full support
- **Windows 7**: Full support
- **Windows 10**: Full support
- **Windows 11**: Experimental support

### EXE Installation

```bash
# Install EXE
wine install app.exe

# Run EXE
wine app.exe

# Configure Wine
winecfg

```

### Compatibility Modes

```bash
# Set Windows version
winecfg --winver win10

# Set DPI awareness
winecfg --dpiaware

# Set virtual desktop
winecfg --virtual-desktop 1920x1080
```

### Development

```rust
use vantisos::legacy_airlock::*;

fn main() {
    let wine = WineIntegration::new();
    
    // Initialize Wine server
    wine.initialize_server()?;
    
    // Load EXE
    let loader = EXELoader::new();
    loader.load_pe_file("app.exe")?;
    
    // Run executable
    wine.run_executable("app.exe")?;
    
    Ok(())
}
```

---

## Security Considerations

### VNT Apps Security

- **Capability-Based Security**: Fine-grained permission control
- **Sandbox Isolation**: Resource-limited execution
- **Code Signing**: Package verification
- **Resource Limits**: Memory, CPU, and storage limits

### Android Subsystem Security

- **SELinux Enforcement**: Mandatory access control
- **Permission System**: Android permission model
- **Sandbox Isolation**: Per-app sandbox
- **HAL Isolation**: Hardware abstraction layer

### Legacy Airlock Security

- **Wine Sandbox**: Isolated Wine environment
- **Malware Scanning**: Automatic executable scanning
- **Signature Verification**: Executable signature checking
- **Resource Limits**: Memory and CPU limits

---

## Performance Optimization

### VNT Apps

- **AOT Compilation**: Ahead-of-time compilation for faster startup
- **Lazy Loading**: On-demand resource loading
- **Memory Pooling**: Efficient memory management
- **Caching**: Application and resource caching

### Android Subsystem

- **JIT Compilation**: Just-in-time compilation
- **ART Optimization**: Android Runtime optimizations
- **Binder IPC Optimization**: Efficient inter-process communication
- **HAL Caching**: Hardware abstraction layer caching

### Legacy Airlock

- **Wine Optimization**: Wine performance tuning
- **DLL Caching**: Native DLL caching
- **Registry Caching**: Registry emulation caching
- **API Translation Caching**: Windows API translation caching

---

## Troubleshooting

### VNT Apps Issues

**Problem**: App won't install
- **Solution**: Check manifest format and package structure

**Problem**: App crashes on startup
- **Solution**: Check resource limits and capabilities

**Problem**: Performance issues
- **Solution**: Enable AOT compilation and caching

### Android Subsystem Issues

**Problem**: APK won't install
- **Solution**: Check minimum SDK version and permissions

**Problem**: App crashes
- **Solution**: Check SELinux policies and sandbox settings

**Problem**: Google Play Services issues
- **Solution**: Ensure Play Services are properly initialized

### Legacy Airlock Issues

**Problem**: EXE won't run
- **Solution**: Check Wine version and compatibility mode

**Problem**: Graphics issues
- **Solution**: Configure virtual desktop and DPI awareness

**Problem**: Performance issues
- **Solution**: Enable Wine optimizations and caching

---

## Best Practices

### VNT Apps Development

1. Use minimal capabilities
2. Optimize WASM binary size
3. Implement proper error handling
4. Test resource limits
5. Use proper packaging

### Android App Development

1. Target API 28+ for best compatibility
2. Use proper permissions
3. Test on multiple Android versions
4. Optimize for performance
5. Follow Android design guidelines

### Windows App Development

1. Test on multiple Windows versions
2. Use proper error handling
3. Avoid deprecated APIs
4. Optimize for Wine compatibility
5. Test with malware scanner

---

## API Reference

### VNT Apps API

```rust
// Package Manager
VntPackageManager::new()
    .install_package(path: &str) -> Result<()>
    .remove_package(name: &str) -> Result<()>
    .launch_app(name: &str) -> Result<()>
    .list_packages() -> Vec<VntPackage>

// WASM Runtime
WasmRuntime::new()
    .load_module(path: &str) -> Result<WasmModule>
    .execute(module: &WasmModule) -> Result<()>

// Capability System
CapabilitySystem::new()
    .grant_capability(capability: &str, app: &str)
    .revoke_capability(capability: &str, app: &str)
    .has_capability(capability: &str, app: &str) -> bool
```

### Android Subsystem API

```rust
// Android Runtime
AndroidRuntime::new()
    .load_apk(path: &str) -> Result<()>
    .launch_app(activity: &str) -> Result<()>
    .uninstall_app(package: &str) -> Result<()>

// Binder IPC
BinderIPC::new()
    .register_service(name: &str) -> Result<()>
    .get_service(name: &str) -> Result<Service>

// HAL Manager
HALManager::new()
    .load_hal_module(name: &str) -> Result<()>
    .is_hal_available(name: &str) -> bool
```

### Legacy Airlock API

```rust
// Wine Integration
WineIntegration::new()
    .initialize_server() -> Result<()>
    .run_executable(path: &str) -> Result<()>

// EXE Loader
EXELoader::new()
    .load_pe_file(path: &str) -> Result<PEFile>
    .is_valid_executable(path: &str) -> bool

// Windows API Translator
WindowsAPITranslator::new()
    .translate_call(api: &str, args: &[Arg]) -> Result<ReturnValue>
```

---

## Version Information

- **VantisOS Version**: 0.4.1 "Cytadela Complete"
- **VNT Apps Version**: 1.0.0
- **Android Subsystem Version**: 1.0.0
- **Legacy Airlock Version**: 1.0.0
- **Wine Version**: 8.0+
- **Android Runtime**: API 34

---

## Support

For issues and questions:
- GitHub Issues: https://github.com/vantisCorp/VantisOS/issues
- Documentation: https://docs.vantisos.ai/compatibility
- Community Forum: https://community.vantisos.ai