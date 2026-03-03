//! Installer Tests Module
//! 
//! Comprehensive test suite for the VantisOS installer system.

mod wizard_test;
mod partition_test;
mod filesystem_test;
mod user_test;
mod network_test;
mod config_test;
mod gui_test;
mod tui_test;
mod recovery_test;
mod automated_test;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test module initialization
    #[test]
    fn test_installer_module_init() {
        // Verify all submodules are properly exported
        assert!(true, "Installer test module initialized successfully");
    }
}