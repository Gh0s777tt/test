//! Legacy API Support Module
//! 
//! This module provides legacy API support for VantisOS including
//! API shims, compatibility wrappers, and translation layers.

use alloc::string::String;

/// API type
#[derive(Debug, Clone, Copy)]
pub enum LegacyApiType {
    WindowsApi,
    LinuxApi,
    PosixApi,
    UnixApi,
    Custom,
}

/// API function signature
#[derive(Debug, Clone)]
pub struct ApiFunctionSignature {
    pub function_name: String,
    pub return_type: String,
    pub parameter_types: Vec<String>,
}

/// API shim
#[derive(Debug, Clone)]
pub struct ApiShim {
    pub api_type: LegacyApiType,
    pub original_function_name: String,
    pub shim_function_name: String,
    pub signature: ApiFunctionSignature,
    pub translation_layer: Option<String>,
}

/// Compatibility wrapper
#[derive(Debug, Clone)]
pub struct CompatibilityWrapper {
    pub wrapper_name: String,
    pub api_type: LegacyApiType,
    pub wrapped_functions: Vec<String>,
    pub initialization_code: Option<String>,
}

/// Legacy API manager
pub struct LegacyApiManager {
    api_shims: alloc::vec::Vec<ApiShim>,
    compatibility_wrappers: alloc::vec::Vec<CompatibilityWrapper>,
}

impl LegacyApiManager {
    /// Create a new legacy API manager
    pub fn new() -> Self {
        Self {
            api_shims: Vec::new(),
            compatibility_wrappers: Vec::new(),
        }
    }

    /// Add an API shim
    pub fn add_shim(&mut self, shim: ApiShim) {
        self.api_shims.push(shim);
    }

    /// Add a compatibility wrapper
    pub fn add_wrapper(&mut self, wrapper: CompatibilityWrapper) {
        self.compatibility_wrappers.push(wrapper);
    }

    /// Get shims by API type
    pub fn shims_by_api_type(&self, api_type: LegacyApiType) -> Vec<ApiShim> {
        self.api_shims
            .iter()
            .filter(|s| s.api_type == api_type)
            .cloned()
            .collect()
    }

    /// Get wrappers by API type
    pub fn wrappers_by_api_type(&self, api_type: LegacyApiType) -> Vec<CompatibilityWrapper> {
        self.compatibility_wrappers
            .iter()
            .filter(|w| w.api_type == api_type)
            .cloned()
            .collect()
    }

    /// Create a new API shim
    pub fn create_shim(
        &mut self,
        api_type: LegacyApiType,
        original_name: impl Into<String>,
        shim_name: impl Into<String>,
        signature: ApiFunctionSignature,
    ) {
        let shim = ApiShim {
            api_type,
            original_function_name: original_name.into(),
            shim_function_name: shim_name.into(),
            signature,
            translation_layer: None,
        };
        self.add_shim(shim);
    }

    /// Create a new compatibility wrapper
    pub fn create_wrapper(
        &mut self,
        wrapper_name: impl Into<String>,
        api_type: LegacyApiType,
        wrapped_functions: Vec<String>,
    ) {
        let wrapper = CompatibilityWrapper {
            wrapper_name: wrapper_name.into(),
            api_type,
            wrapped_functions,
            initialization_code: None,
        };
        self.add_wrapper(wrapper);
    }

    /// Get total shims count
    pub fn shims_count(&self) -> usize {
        self.api_shims.len()
    }

    /// Get total wrappers count
    pub fn wrappers_count(&self) -> usize {
        self.compatibility_wrappers.len()
    }
}

impl Default for LegacyApiManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global legacy API manager
static LEGACY_API_MANAGER: spin::Once<LegacyApiManager> = spin::Once::new();

/// Get the global legacy API manager
pub fn legacy_api_manager() -> &'static LegacyApiManager {
    LEGACY_API_MANAGER.call_once(|| LegacyApiManager::new())
}

/// Add an API shim
pub fn add_legacy_api_shim(shim: ApiShim) {
    legacy_api_manager().add_shim(shim);
}

/// Add a compatibility wrapper
pub fn add_compatibility_wrapper(wrapper: CompatibilityWrapper) {
    legacy_api_manager().add_wrapper(wrapper);
}

/// Get shims by API type
pub fn get_shims_by_api_type(api_type: LegacyApiType) -> Vec<ApiShim> {
    legacy_api_manager().shims_by_api_type(api_type)
}

/// Get wrappers by API type
pub fn get_wrappers_by_api_type(api_type: LegacyApiType) -> Vec<CompatibilityWrapper> {
    legacy_api_manager().wrappers_by_api_type(api_type)
}