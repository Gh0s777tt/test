// Android Subsystem Compatibility Tests
// Tests for Android compatibility layer

#[cfg(test)]
mod tests {
    use super::super::super::src::verified::android_subsystem::*;

    #[test]
    fn test_android_manifest_creation() {
        let manifest = AndroidManifest {
            package: "com.example.androidapp".to_string(),
            version_code: 1,
            version_name: "1.0.0".to_string(),
            app_name: "Android Test App".to_string(),
            min_sdk_version: 21,
            target_sdk_version: 34,
            permissions: vec![],
            activities: vec![],
            services: vec![],
            receivers: vec![],
        };

        assert_eq!(manifest.package, "com.example.androidapp");
        assert_eq!(manifest.version_code, 1);
        assert_eq!(manifest.min_sdk_version, 21);
    }

    #[test]
    fn test_activity_creation() {
        let activity = Activity {
            name: "MainActivity".to_string(),
            exported: true,
            intent_filters: vec![],
        };

        assert_eq!(activity.name, "MainActivity");
        assert!(activity.exported);
    }

    #[test]
    fn test_service_creation() {
        let service = Service {
            name: "BackgroundService".to_string(),
            exported: false,
            process: ":remote".to_string(),
        };

        assert_eq!(service.name, "BackgroundService");
        assert!(!service.exported);
    }

    #[test]
    fn test_android_runtime() {
        let runtime = AndroidRuntime::new();
        
        assert!(runtime.is_initialized());
        
        // Test APK loading
        let result = runtime.load_apk("/test/app.apk");
        assert!(result.is_ok());
    }

    #[test]
    fn test_binder_ipc() {
        let ipc = BinderIPC::new();
        
        assert!(ipc.is_connected());
        
        // Test service registration
        let result = ipc.register_service("test.service");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hal_manager() {
        let manager = HALManager::new();
        
        // Test HAL module loading
        let result = manager.load_hal_module("camera");
        assert!(result.is_ok());
        
        // Test HAL availability
        assert!(manager.is_hal_available("camera"));
    }

    #[test]
    fn test_permission_system() {
        let mut system = PermissionSystem::new();
        
        // Test permission granting
        system.grant_permission("INTERNET", "com.example.app");
        assert!(system.has_permission("INTERNET", "com.example.app"));
        
        // Test permission revocation
        system.revoke_permission("INTERNET", "com.example.app");
        assert!(!system.has_permission("INTERNET", "com.example.app"));
    }

    #[test]
    fn test_android_sandbox() {
        let sandbox = AndroidSandbox::new();
        
        assert!(sandbox.is_isolated());
        
        // Test SELinux enforcement
        assert!(sandbox.is_selinux_enforced());
        
        // Test resource limits
        sandbox.set_memory_limit(512);
        assert_eq!(sandbox.get_memory_limit(), 512);
    }

    #[test]
    fn test_play_services() {
        let play_services = PlayServices::new();
        
        assert!(play_services.is_available());
        
        // Test Google Play Services integration
        let result = play_services.initialize();
        assert!(result.is_ok());
    }
}