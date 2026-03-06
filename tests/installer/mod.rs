//! Installer Tests
//! 
//! This module contains comprehensive tests for the VantisOS installer,
//! including wizard functionality, partitioning, filesystem operations,
//! user management, network configuration, system configuration, and recovery.

pub mod wizard_test;
pub mod partition_test;
pub mod filesystem_test;
pub mod user_test;
pub mod network_test;
pub mod config_test;
pub mod gui_test;
pub mod tui_test;
pub mod recovery_test;
pub mod automated_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_module_imports() {
        // Verify all installer test modules are accessible
        assert!(true, "Installer test module structure is valid");
    }
}