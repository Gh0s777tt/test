// VNT Apps Compatibility Tests
// Tests for WebAssembly application system

#[cfg(test)]
mod tests {
    use super::super::super::src::verified::vnt_apps::*;

    #[test]
    fn test_vnt_manifest_creation() {
        let manifest = VntManifest {
            name: "com.example.app".to_string(),
            version: "1.0.0".to_string(),
            display_name: "Example App".to_string(),
            description: "Test application".to_string(),
            author: "Test Author".to_string(),
            license: "MIT".to_string(),
            min_vantis_os: "0.4.1".to_string(),
            capabilities: vec![],
            permissions: VntPermissions::default(),
            resources: VntResources::default(),
            ui: VntUiConfig::default(),
            dependencies: vec![],
        };

        assert_eq!(manifest.name, "com.example.app");
        assert_eq!(manifest.version, "1.0.0");
    }

    #[test]
    fn test_vnt_permissions_default() {
        let permissions = VntPermissions::default();
        
        assert!(!permissions.network_access);
        assert!(!permissions.file_system_access);
        assert!(!permissions.hardware_access);
    }

    #[test]
    fn test_vnt_resources_default() {
        let resources = VntResources::default();
        
        assert_eq!(resources.min_memory_mb, 64);
        assert_eq!(resources.min_storage_mb, 10);
        assert_eq!(resources.min_cpu_cores, 1);
    }

    #[test]
    fn test_vnt_package_manager() {
        let mut manager = VntPackageManager::new();
        
        // Test package installation
        let result = manager.install_package("/test/path/app.vnt");
        assert!(result.is_ok());
        
        // Test package listing
        let packages = manager.list_packages();
        assert!(!packages.is_empty());
    }

    #[test]
    fn test_wasm_runtime() {
        let runtime = WasmRuntime::new();
        
        assert!(runtime.is_initialized());
        
        // Test WASM module loading
        let result = runtime.load_module("/test/module.wasm");
        assert!(result.is_ok());
    }

    #[test]
    fn test_capability_system() {
        let mut system = CapabilitySystem::new();
        
        // Test capability granting
        system.grant_capability("network", "com.example.app");
        assert!(system.has_capability("network", "com.example.app"));
        
        // Test capability revocation
        system.revoke_capability("network", "com.example.app");
        assert!(!system.has_capability("network", "com.example.app"));
    }

    #[test]
    fn test_sandbox_isolation() {
        let sandbox = Sandbox::new();
        
        assert!(sandbox.is_isolated());
        
        // Test resource limits
        sandbox.set_memory_limit(128);
        assert_eq!(sandbox.get_memory_limit(), 128);
    }
}