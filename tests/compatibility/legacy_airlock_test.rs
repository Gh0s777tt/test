// Legacy Airlock Compatibility Tests
// Tests for Windows executable compatibility layer

#[cfg(test)]
mod tests {
    use super::super::super::src::verified::legacy_airlock::*;

    #[test]
    fn test_wine_integration() {
        let wine = WineIntegration::new();
        
        assert!(wine.is_installed());
        
        // Test Wine server initialization
        let result = wine.initialize_server();
        assert!(result.is_ok());
    }

    #[test]
    fn test_exe_loader() {
        let loader = EXELoader::new();
        
        // Test PE file loading
        let result = loader.load_pe_file("/test/app.exe");
        assert!(result.is_ok());
        
        // Test executable validation
        let is_valid = loader.is_valid_executable("/test/app.exe");
        assert!(is_valid);
    }

    #[test]
    fn test_windows_api_translation() {
        let translator = WindowsAPITranslator::new();
        
        // Test API mapping
        assert!(translator.has_translation("CreateFileA"));
        assert!(translator.has_translation("WriteFile"));
        assert!(translator.has_translation("ReadFile"));
    }

    #[test]
    fn test_dll_loading() {
        let loader = DLLLoader::new();
        
        // Test native DLL loading
        let result = loader.load_native_dll("kernel32.dll");
        assert!(result.is_ok());
        
        // Test DLL function resolution
        let result = loader.resolve_function("kernel32.dll", "GetLastError");
        assert!(result.is_ok());
    }

    #[test]
    fn test_registry_emulation() {
        let registry = RegistryEmulation::new();
        
        // Test registry key creation
        let result = registry.create_key("HKEY_CURRENT_USER\\Software\\Test");
        assert!(result.is_ok());
        
        // Test registry value setting
        let result = registry.set_value("HKEY_CURRENT_USER\\Software\\Test", "TestValue", "Test");
        assert!(result.is_ok());
        
        // Test registry value reading
        let value = registry.get_value("HKEY_CURRENT_USER\\Software\\Test", "TestValue");
        assert_eq!(value, Some("Test".to_string()));
    }

    #[test]
    fn test_file_system_redirection() {
        let fs_redirect = FileSystemRedirection::new();
        
        // Test path translation
        let windows_path = "C:\\Users\\Test\\file.txt";
        let vantis_path = fs_redirect.translate_path(windows_path);
        assert!(vantis_path.is_ok());
    }

    #[test]
    fn test_wine_sandbox() {
        let sandbox = WineSandbox::new();
        
        assert!(sandbox.is_isolated());
        
        // Test resource limits
        sandbox.set_memory_limit(1024);
        assert_eq!(sandbox.get_memory_limit(), 1024);
        
        // Test network isolation
        assert!(sandbox.is_network_isolated());
    }

    #[test]
    fn test_malware_scanning() {
        let scanner = MalwareScanner::new();
        
        // Test executable scanning
        let result = scanner.scan_executable("/test/app.exe");
        assert!(result.is_ok());
        
        // Test signature verification
        let result = scanner.verify_signature("/test/app.exe");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compatibility_mode() {
        let mode = CompatibilityMode::new();
        
        // Test Windows version emulation
        mode.set_windows_version(WindowsVersion::Windows10);
        assert_eq!(mode.get_windows_version(), WindowsVersion::Windows10);
        
        // Test DPI awareness
        mode.set_dpi_awareness(true);
        assert!(mode.is_dpi_aware());
    }
}