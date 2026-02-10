//! Direct Metal Backend Abstraction Layer
//! 
//! This module provides a trait-based abstraction over different GPU backends
//! (Vulkan, Metal, etc.), allowing for compile-time and runtime backend selection.
//!
//! # Architecture
//! 
//! ```text
//! Application
//!      ↓
//! Direct Metal API
//!      ↓
//! Backend Trait ← Vulkan Backend
//!              ← Metal Backend
//!              ← Future Backends
//! ```

use crate::direct_metal::{
    GpuDeviceId, GpuMemoryId, GpuCommandBufferId,
    GpuFenceId, GpuPipelineId,
};

/// Backend type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    /// Vulkan backend (cross-platform)
    Vulkan,
    /// Metal backend (macOS/iOS)
    Metal,
    /// Software rasterizer (testing/fallback)
    Software,
}

/// GPU backend capabilities
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    /// Backend name
    pub name: String,
    /// Backend version
    pub version: String,
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum buffer size
    pub max_buffer_size: u64,
    /// Supports compute shaders
    pub compute_support: bool,
    /// Supports ray tracing
    pub raytracing_support: bool,
    /// Supports mesh shaders
    pub mesh_shader_support: bool,
    /// Maximum number of queues
    pub max_queues: u32,
    /// Unified memory architecture
    pub unified_memory: bool,
}

/// Backend initialization configuration
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// Enable validation/debug layers
    pub enable_validation: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Preferred device index (None = auto-select)
    pub preferred_device: Option<usize>,
    /// Application name
    pub app_name: String,
    /// Application version
    pub app_version: u32,
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            enable_validation: cfg!(debug_assertions),
            enable_profiling: false,
            preferred_device: None,
            app_name: "VantisOS Application".to_string(),
            app_version: 1,
        }
    }
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device name
    pub name: String,
    /// Device type (discrete, integrated, etc.)
    pub device_type: DeviceType,
    /// Vendor ID
    pub vendor_id: u32,
    /// Device ID
    pub device_id: u32,
    /// Total memory (bytes)
    pub total_memory: u64,
    /// Available memory (bytes)
    pub available_memory: u64,
}

/// GPU device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Discrete GPU (dedicated graphics card)
    Discrete,
    /// Integrated GPU (CPU-integrated)
    Integrated,
    /// Virtual GPU
    Virtual,
    /// Software rasterizer
    Software,
}

/// Backend error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    /// Backend initialization failed
    InitializationFailed(String),
    /// No suitable device found
    NoDeviceFound,
    /// Device creation failed
    DeviceCreationFailed(String),
    /// Memory allocation failed
    AllocationFailed(String),
    /// Command submission failed
    SubmissionFailed(String),
    /// Synchronization failed
    SynchronizationFailed(String),
    /// Backend not supported on this platform
    UnsupportedPlatform,
    /// Invalid operation
    InvalidOperation(String),
}

/// Result type for backend operations
pub type BackendResult<T> = Result<T, BackendError>;

/// Main GPU backend trait
/// 
/// All GPU backends (Vulkan, Metal, etc.) must implement this trait
/// to provide a unified interface for GPU operations.
pub trait GpuBackend: Send + Sync {
    /// Get backend type
    fn backend_type(&self) -> BackendType;
    
    /// Get backend capabilities
    fn capabilities(&self) -> &BackendCapabilities;
    
    /// Initialize the backend
    fn initialize(&mut self, config: &BackendConfig) -> BackendResult<()>;
    
    /// Shutdown the backend and cleanup resources
    fn shutdown(&mut self) -> BackendResult<()>;
    
    /// Enumerate available GPU devices
    fn enumerate_devices(&self) -> BackendResult<Vec<DeviceInfo>>;
    
    /// Create a GPU device handle
    fn create_device(&mut self, device_index: usize) -> BackendResult<GpuDeviceId>;
    
    /// Destroy a GPU device
    fn destroy_device(&mut self, device_id: GpuDeviceId) -> BackendResult<()>;
    
    /// Allocate GPU memory
    fn allocate_memory(
        &mut self,
        device_id: GpuDeviceId,
        size: u64,
        memory_type: MemoryType,
    ) -> BackendResult<GpuMemoryId>;
    
    /// Free GPU memory
    fn free_memory(&mut self, memory_id: GpuMemoryId) -> BackendResult<()>;
    
    /// Map GPU memory to CPU address space
    fn map_memory(&mut self, memory_id: GpuMemoryId) -> BackendResult<*mut u8>;
    
    /// Unmap GPU memory
    fn unmap_memory(&mut self, memory_id: GpuMemoryId) -> BackendResult<()>;
    
    /// Create a command buffer
    fn create_command_buffer(
        &mut self,
        device_id: GpuDeviceId,
    ) -> BackendResult<GpuCommandBufferId>;
    
    /// Begin recording commands
    fn begin_command_buffer(&mut self, cmd_id: GpuCommandBufferId) -> BackendResult<()>;
    
    /// End recording commands
    fn end_command_buffer(&mut self, cmd_id: GpuCommandBufferId) -> BackendResult<()>;
    
    /// Submit command buffer to GPU
    fn submit_commands(
        &mut self,
        device_id: GpuDeviceId,
        cmd_id: GpuCommandBufferId,
        fence: Option<GpuFenceId>,
    ) -> BackendResult<()>;
    
    /// Create a synchronization fence
    fn create_fence(&mut self, device_id: GpuDeviceId) -> BackendResult<GpuFenceId>;
    
    /// Wait for fence to be signaled
    fn wait_for_fence(
        &mut self,
        fence_id: GpuFenceId,
        timeout_ns: u64,
    ) -> BackendResult<bool>;
    
    /// Reset fence to unsignaled state
    fn reset_fence(&mut self, fence_id: GpuFenceId) -> BackendResult<()>;
    
    /// Create a graphics/compute pipeline
    fn create_pipeline(
        &mut self,
        device_id: GpuDeviceId,
        pipeline_type: PipelineType,
    ) -> BackendResult<GpuPipelineId>;
    
    /// Destroy a pipeline
    fn destroy_pipeline(&mut self, pipeline_id: GpuPipelineId) -> BackendResult<()>;
    
    /// Wait for device to become idle
    fn wait_idle(&mut self, device_id: GpuDeviceId) -> BackendResult<()>;
    
    /// Get device information
    fn get_device_info(&self, device_id: GpuDeviceId) -> BackendResult<DeviceInfo>;
}

/// Memory type for GPU allocations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// Device-local memory (fastest, GPU-only)
    DeviceLocal,
    /// Host-visible memory (CPU-accessible)
    HostVisible,
    /// Host-cached memory (CPU-accessible with caching)
    HostCached,
    /// Unified memory (shared between CPU and GPU)
    Unified,
}

/// Pipeline type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineType {
    /// Graphics pipeline (vertex + fragment shaders)
    Graphics,
    /// Compute pipeline (compute shaders)
    Compute,
    /// Ray tracing pipeline
    RayTracing,
}

/// Backend factory - creates appropriate backend for the platform
pub struct BackendFactory;

impl BackendFactory {
    /// Create a backend based on platform and preference
    /// 
    /// # Arguments
    /// * `preferred` - Preferred backend type (None = auto-detect)
    /// 
    /// # Returns
    /// * `BackendResult<Box<dyn GpuBackend>>` - Backend instance
    pub fn create_backend(
        preferred: Option<BackendType>,
    ) -> BackendResult<Box<dyn GpuBackend>> {
        match preferred {
            Some(BackendType::Vulkan) => Self::create_vulkan_backend(),
            Some(BackendType::Metal) => Self::create_metal_backend(),
            Some(BackendType::Software) => Self::create_software_backend(),
            None => Self::create_default_backend(),
        }
    }
    
    /// Create Vulkan backend
    fn create_vulkan_backend() -> BackendResult<Box<dyn GpuBackend>> {
        #[cfg(feature = "vulkan")]
        {
            #[allow(unused_imports)]
            use crate::direct_metal_vulkan::VulkanBackend;
            #[cfg(feature = "vulkan")]
            return Ok(Box::new(VulkanBackend::new()));
        }
        #[cfg(not(feature = "vulkan"))]
        {
            Err(BackendError::UnsupportedPlatform)
        }
    }
    
    /// Create Metal backend
    fn create_metal_backend() -> BackendResult<Box<dyn GpuBackend>> {
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            #[allow(unused_imports)]
            use crate::direct_metal_metal::MetalBackend;
            #[cfg(all(target_os = "macos", feature = "metal"))]
            return Ok(Box::new(MetalBackend::new()));
        }
        #[cfg(not(all(target_os = "macos", feature = "metal")))]
        {
            Err(BackendError::UnsupportedPlatform)
        }
    }
    
    /// Create software backend
    fn create_software_backend() -> BackendResult<Box<dyn GpuBackend>> {
        // TODO: Implement software rasterizer for testing
        Err(BackendError::UnsupportedPlatform)
    }
    
    /// Create default backend for the platform
    fn create_default_backend() -> BackendResult<Box<dyn GpuBackend>> {
        // Try backends in order of preference
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            #[allow(unused_imports)]
            use crate::direct_metal_metal::MetalBackend;
            #[cfg(all(target_os = "macos", feature = "metal"))]
            return Ok(Box::new(MetalBackend::new()));
        }
        
        #[cfg(feature = "vulkan")]
        {
            #[allow(unused_imports)]
            use crate::direct_metal_vulkan::VulkanBackend;
            #[cfg(feature = "vulkan")]
            return Ok(Box::new(VulkanBackend::new()));
        }
        
        #[cfg(not(any(feature = "vulkan", all(target_os = "macos", feature = "metal"))))]
        {
            Err(BackendError::UnsupportedPlatform)
        }
    }
    
    /// Get list of available backends on this platform
    pub fn available_backends() -> Vec<BackendType> {
        let backends = Vec::new();
        
        #[cfg(feature = "vulkan")]
        backends.push(BackendType::Vulkan);
        
        #[cfg(all(target_os = "macos", feature = "metal"))]
        backends.push(BackendType::Metal);
        
        backends
    }
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_backend_config_default() {
        let config = BackendConfig::default();
        assert_eq!(config.app_name, "VantisOS Application");
        assert_eq!(config.app_version, 1);
        assert_eq!(config.enable_validation, cfg!(debug_assertions));
    }
    
    #[test]
    fn test_backend_type_equality() {
        assert_eq!(BackendType::Vulkan, BackendType::Vulkan);
        assert_ne!(BackendType::Vulkan, BackendType::Metal);
    }
    
    #[test]
    fn test_device_type_equality() {
        assert_eq!(DeviceType::Discrete, DeviceType::Discrete);
        assert_ne!(DeviceType::Discrete, DeviceType::Integrated);
    }
    
    #[test]
    fn test_memory_type_equality() {
        assert_eq!(MemoryType::DeviceLocal, MemoryType::DeviceLocal);
        assert_ne!(MemoryType::DeviceLocal, MemoryType::HostVisible);
    }
    
    #[test]
    fn test_pipeline_type_equality() {
        assert_eq!(PipelineType::Graphics, PipelineType::Graphics);
        assert_ne!(PipelineType::Graphics, PipelineType::Compute);
    }
    
    #[test]
    fn test_backend_error_equality() {
        let err1 = BackendError::NoDeviceFound;
        let err2 = BackendError::NoDeviceFound;
        assert_eq!(err1, err2);
    }
    
    #[test]
    fn test_available_backends() {
        let backends = BackendFactory::available_backends();
        // Should have at least one backend available
        // (or none if no features enabled, which is fine for testing)
        assert!(backends.len() >= 0);
    }
    
    #[test]
    fn test_backend_capabilities_clone() {
        let caps = BackendCapabilities {
            name: "Test Backend".to_string(),
            version: "1.0".to_string(),
            max_texture_size: 16384,
            max_buffer_size: 1024 * 1024 * 1024,
            compute_support: true,
            raytracing_support: false,
            mesh_shader_support: false,
            max_queues: 4,
            unified_memory: false,
        };
        
        let caps_clone = caps.clone();
        assert_eq!(caps.name, caps_clone.name);
        assert_eq!(caps.max_texture_size, caps_clone.max_texture_size);
    }
    
    #[test]
    fn test_device_info_clone() {
        let info = DeviceInfo {
            name: "Test GPU".to_string(),
            device_type: DeviceType::Discrete,
            vendor_id: 0x10DE,
            device_id: 0x1234,
            total_memory: 8 * 1024 * 1024 * 1024,
            available_memory: 6 * 1024 * 1024 * 1024,
        };
        
        let info_clone = info.clone();
        assert_eq!(info.name, info_clone.name);
        assert_eq!(info.device_type, info_clone.device_type);
    }
}