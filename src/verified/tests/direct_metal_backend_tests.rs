//! Integration tests for Direct Metal backend system
//!
//! These tests verify the backend abstraction layer and both
//! Vulkan and Metal backend implementations.

#[cfg(test)]
mod backend_integration_tests {
    use crate::direct_metal_backend::*;
    
    #[test]
    fn test_backend_factory_available_backends() {
        let backends = BackendFactory::available_backends();
        
        // Should have at least one backend on supported platforms
        #[cfg(any(feature = "vulkan", all(target_os = "macos", feature = "metal")))]
        assert!(!backends.is_empty());
        
        // Vulkan should be available if feature is enabled
        #[cfg(feature = "vulkan")]
        assert!(backends.contains(&BackendType::Vulkan));
        
        // Metal should be available on macOS if feature is enabled
        #[cfg(all(target_os = "macos", feature = "metal"))]
        assert!(backends.contains(&BackendType::Metal));
    }
    
    #[test]
    fn test_backend_config_default() {
        let config = BackendConfig::default();
        assert_eq!(config.app_name, "VantisOS Application");
        assert_eq!(config.app_version, 1);
        assert_eq!(config.enable_validation, cfg!(debug_assertions));
        assert!(!config.enable_profiling);
        assert!(config.preferred_device.is_none());
    }
    
    #[test]
    fn test_backend_config_custom() {
        let config = BackendConfig {
            enable_validation: true,
            enable_profiling: true,
            preferred_device: Some(0),
            app_name: "Test App".to_string(),
            app_version: 2,
        };
        
        assert!(config.enable_validation);
        assert!(config.enable_profiling);
        assert_eq!(config.preferred_device, Some(0));
        assert_eq!(config.app_name, "Test App");
        assert_eq!(config.app_version, 2);
    }
    
    #[test]
    fn test_memory_type_variants() {
        let types = vec![
            MemoryType::DeviceLocal,
            MemoryType::HostVisible,
            MemoryType::HostCached,
            MemoryType::Unified,
        ];
        
        // Ensure all variants are distinct
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }
    
    #[test]
    fn test_pipeline_type_variants() {
        let types = vec![
            PipelineType::Graphics,
            PipelineType::Compute,
            PipelineType::RayTracing,
        ];
        
        // Ensure all variants are distinct
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }
    
    #[test]
    fn test_device_type_variants() {
        let types = vec![
            DeviceType::Discrete,
            DeviceType::Integrated,
            DeviceType::Virtual,
            DeviceType::Software,
        ];
        
        // Ensure all variants are distinct
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }
    
    #[test]
    fn test_backend_error_variants() {
        let errors = vec![
            BackendError::NoDeviceFound,
            BackendError::UnsupportedPlatform,
            BackendError::InitializationFailed("test".to_string()),
            BackendError::DeviceCreationFailed("test".to_string()),
            BackendError::AllocationFailed("test".to_string()),
            BackendError::SubmissionFailed("test".to_string()),
            BackendError::SynchronizationFailed("test".to_string()),
            BackendError::InvalidOperation("test".to_string()),
        ];
        
        // Ensure errors can be compared
        assert_eq!(BackendError::NoDeviceFound, BackendError::NoDeviceFound);
        assert_ne!(BackendError::NoDeviceFound, BackendError::UnsupportedPlatform);
    }
    
    #[test]
    fn test_backend_capabilities_clone() {
        let caps = BackendCapabilities {
            name: "Test Backend".to_string(),
            version: "1.0.0".to_string(),
            max_texture_size: 8192,
            max_buffer_size: 2 * 1024 * 1024 * 1024,
            compute_support: true,
            raytracing_support: false,
            mesh_shader_support: false,
            max_queues: 4,
            unified_memory: false,
        };
        
        let caps_clone = caps.clone();
        assert_eq!(caps.name, caps_clone.name);
        assert_eq!(caps.version, caps_clone.version);
        assert_eq!(caps.max_texture_size, caps_clone.max_texture_size);
        assert_eq!(caps.max_buffer_size, caps_clone.max_buffer_size);
        assert_eq!(caps.compute_support, caps_clone.compute_support);
        assert_eq!(caps.raytracing_support, caps_clone.raytracing_support);
        assert_eq!(caps.mesh_shader_support, caps_clone.mesh_shader_support);
        assert_eq!(caps.max_queues, caps_clone.max_queues);
        assert_eq!(caps.unified_memory, caps_clone.unified_memory);
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
        assert_eq!(info.vendor_id, info_clone.vendor_id);
        assert_eq!(info.device_id, info_clone.device_id);
        assert_eq!(info.total_memory, info_clone.total_memory);
        assert_eq!(info.available_memory, info_clone.available_memory);
    }
}

#[cfg(all(test, feature = "vulkan"))]
mod vulkan_backend_tests {
    use crate::direct_metal_backend::*;
    use crate::direct_metal_vulkan::VulkanBackend;
    
    #[test]
    fn test_vulkan_backend_lifecycle() {
        let mut backend = VulkanBackend::new();
        assert_eq!(backend.backend_type(), BackendType::Vulkan);
        
        let config = BackendConfig::default();
        assert!(backend.initialize(&config).is_ok());
        
        let devices = backend.enumerate_devices().unwrap();
        assert!(!devices.is_empty());
        
        assert!(backend.shutdown().is_ok());
    }
    
    #[test]
    fn test_vulkan_device_creation() {
        let mut backend = VulkanBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let device_info = backend.get_device_info(device_id).unwrap();
        
        assert!(!device_info.name.is_empty());
        assert!(device_info.total_memory > 0);
        
        assert!(backend.destroy_device(device_id).is_ok());
    }
    
    #[test]
    fn test_vulkan_memory_operations() {
        let mut backend = VulkanBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let memory_id = backend
            .allocate_memory(device_id, 1024 * 1024, MemoryType::HostVisible)
            .unwrap();
        
        let ptr = backend.map_memory(memory_id).unwrap();
        assert!(!ptr.is_null());
        
        assert!(backend.unmap_memory(memory_id).is_ok());
        assert!(backend.free_memory(memory_id).is_ok());
    }
    
    #[test]
    fn test_vulkan_command_buffer_workflow() {
        let mut backend = VulkanBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let cmd_id = backend.create_command_buffer(device_id).unwrap();
        
        assert!(backend.begin_command_buffer(cmd_id).is_ok());
        assert!(backend.end_command_buffer(cmd_id).is_ok());
        
        let fence_id = backend.create_fence(device_id).unwrap();
        assert!(backend.submit_commands(device_id, cmd_id, Some(fence_id)).is_ok());
        
        let signaled = backend.wait_for_fence(fence_id, 1000000).unwrap();
        assert!(signaled);
    }
    
    #[test]
    fn test_vulkan_pipeline_creation() {
        let mut backend = VulkanBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        
        let graphics_pipeline = backend
            .create_pipeline(device_id, PipelineType::Graphics)
            .unwrap();
        let compute_pipeline = backend
            .create_pipeline(device_id, PipelineType::Compute)
            .unwrap();
        
        assert!(backend.destroy_pipeline(graphics_pipeline).is_ok());
        assert!(backend.destroy_pipeline(compute_pipeline).is_ok());
    }
}

#[cfg(all(test, target_os = "macos", feature = "metal"))]
mod metal_backend_tests {
    use crate::direct_metal_backend::*;
    use crate::direct_metal_metal::MetalBackend;
    
    #[test]
    fn test_metal_backend_lifecycle() {
        let mut backend = MetalBackend::new();
        assert_eq!(backend.backend_type(), BackendType::Metal);
        
        let config = BackendConfig::default();
        assert!(backend.initialize(&config).is_ok());
        
        let devices = backend.enumerate_devices().unwrap();
        assert!(!devices.is_empty());
        
        assert!(backend.shutdown().is_ok());
    }
    
    #[test]
    fn test_metal_device_creation() {
        let mut backend = MetalBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let device_info = backend.get_device_info(device_id).unwrap();
        
        assert!(!device_info.name.is_empty());
        assert!(device_info.total_memory > 0);
        assert_eq!(device_info.vendor_id, 0x106B); // Apple
        
        assert!(backend.destroy_device(device_id).is_ok());
    }
    
    #[test]
    fn test_metal_unified_memory() {
        let mut backend = MetalBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let memory_id = backend
            .allocate_memory(device_id, 1024 * 1024, MemoryType::Unified)
            .unwrap();
        
        // Metal memory is always accessible
        let ptr = backend.map_memory(memory_id).unwrap();
        assert!(!ptr.is_null());
        
        // Unmapping is a no-op in Metal
        assert!(backend.unmap_memory(memory_id).is_ok());
        assert!(backend.free_memory(memory_id).is_ok());
    }
    
    #[test]
    fn test_metal_command_buffer_workflow() {
        let mut backend = MetalBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        let cmd_id = backend.create_command_buffer(device_id).unwrap();
        
        assert!(backend.begin_command_buffer(cmd_id).is_ok());
        assert!(backend.end_command_buffer(cmd_id).is_ok());
        
        let fence_id = backend.create_fence(device_id).unwrap();
        assert!(backend.submit_commands(device_id, cmd_id, Some(fence_id)).is_ok());
        
        let signaled = backend.wait_for_fence(fence_id, 1000000).unwrap();
        assert!(signaled);
    }
    
    #[test]
    fn test_metal_pipeline_creation() {
        let mut backend = MetalBackend::new();
        let config = BackendConfig::default();
        backend.initialize(&config).unwrap();
        
        let device_id = backend.create_device(0).unwrap();
        
        let compute_pipeline = backend
            .create_pipeline(device_id, PipelineType::Compute)
            .unwrap();
        let graphics_pipeline = backend
            .create_pipeline(device_id, PipelineType::Graphics)
            .unwrap();
        
        assert!(backend.destroy_pipeline(compute_pipeline).is_ok());
        assert!(backend.destroy_pipeline(graphics_pipeline).is_ok());
    }
}

#[cfg(test)]
mod cross_backend_tests {
    use crate::direct_metal_backend::*;
    
    /// Test that both backends implement the same interface
    #[test]
    fn test_backend_trait_consistency() {
        // This test ensures both backends implement GpuBackend trait
        // and have consistent behavior
        
        #[cfg(feature = "vulkan")]
        {
            use crate::direct_metal_vulkan::VulkanBackend;
            let backend: Box<dyn GpuBackend> = Box::new(VulkanBackend::new());
            assert_eq!(backend.backend_type(), BackendType::Vulkan);
        }
        
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            use crate::direct_metal_metal::MetalBackend;
            let backend: Box<dyn GpuBackend> = Box::new(MetalBackend::new());
            assert_eq!(backend.backend_type(), BackendType::Metal);
        }
    }
    
    /// Test backend factory with different preferences
    #[test]
    fn test_backend_factory_preferences() {
        // Test Vulkan preference
        #[cfg(feature = "vulkan")]
        {
            let result = BackendFactory::create_backend(Some(BackendType::Vulkan));
            assert!(result.is_ok());
            let backend = result.unwrap();
            assert_eq!(backend.backend_type(), BackendType::Vulkan);
        }
        
        // Test Metal preference
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            let result = BackendFactory::create_backend(Some(BackendType::Metal));
            assert!(result.is_ok());
            let backend = result.unwrap();
            assert_eq!(backend.backend_type(), BackendType::Metal);
        }
        
        // Test auto-detection
        #[cfg(any(feature = "vulkan", all(target_os = "macos", feature = "metal")))]
        {
            let result = BackendFactory::create_backend(None);
            assert!(result.is_ok());
        }
    }
}