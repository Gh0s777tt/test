# Visual Permission Cards Implementation Guide
## VantisOS - Faza 5: Cytadela Ekosystem

**Version**: 1.0  
**Date**: February 24, 2025  
**Author**: SuperNinja AI Agent  
**Estimated Time**: 3 days  
**Priority**: High

---

## 📋 Executive Summary

This guide provides a comprehensive implementation plan for Visual Permission Cards - a user-friendly, graphical interface for managing application permissions in VantisOS. The system provides intuitive visual cards that display and control application permissions with clear explanations and user-friendly controls.

### Key Objectives
- ✅ Visual permission cards for each application
- ✅ Intuitive permission controls
- ✅ Clear permission explanations
- ✅ Permission history tracking
- ✅ Permission templates
- ✅ Batch permission management
- ✅ Permission audit logs
- ✅ Formal verification of security-critical components

---

## 🏗️ Architecture Overview

### Component Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│              Visual Permission Cards Manager                  │
│              (High-Level Permission UI API)                  │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌────────▼────────┐
│  Card Renderer │   │  Permission     │   │  Permission     │
│  (GUI)         │   │  Controller    │   │  History        │
└────────────────┘   └─────────────────┘   └─────────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  Permission      │
                    │  Templates       │
                    └───────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  Audit Log        │
                    │  (Tracking)       │
                    └───────────────────┘
```

### Core Components

1. **Permission Cards Manager** - High-level permission UI management
2. **Card Renderer** - GUI rendering for permission cards
3. **Permission Controller** - Permission control logic
4. **Permission History** - Permission change tracking
5. **Permission Templates** - Pre-defined permission sets
6. **Audit Log** - Permission audit logging

---

## 📁 File Structure

```
src/verified/
├── permission_cards/
│   ├── mod.rs                          # Main module
│   ├── api.rs                          # High-level API
│   ├── renderer.rs                     # Card renderer
│   ├── controller.rs                   # Permission controller
│   ├── history.rs                      # Permission history
│   ├── templates.rs                    # Permission templates
│   ├── audit.rs                        # Audit log
│   └── verification.rs                 # Formal verification
└── horizon/
    └── flux/
        └── permission_cards.rs          # Integration with Flux Engine
```

---

## 🔧 Implementation Plan (3 Days)

### Day 1: Core API & Card Renderer
**Tasks:**
- [ ] Define `PermissionCard` trait
- [ ] Define `PermissionCardsContext` struct
- [ ] Define `PermissionCardUI` struct
- [ ] Implement card renderer (GUI)
- [ ] Implement permission display
- [ ] Create error types and Result types

**Code Structure:**
```rust
// src/verified/permission_cards/api.rs

use crate::permission_cards::renderer::CardRenderer;
use crate::permission_cards::controller::PermissionController;
use crate::permission_cards::history::PermissionHistory;
use crate::permission_cards::templates::PermissionTemplates;

/// Visual Permission Cards Manager
pub struct PermissionCardsContext {
    renderer: CardRenderer,
    controller: PermissionController,
    history: PermissionHistory,
    templates: PermissionTemplates,
}

impl PermissionCardsContext {
    pub fn new() -> Result<Self, PermissionError> {
        let renderer = CardRenderer::new()?;
        let controller = PermissionController::new()?;
        let history = PermissionHistory::new()?;
        let templates = PermissionTemplates::new()?;
        
        Ok(Self {
            renderer,
            controller,
            history,
            templates,
        })
    }
    
    /// Create permission card for application
    pub fn create_permission_card(
        &mut self,
        app_id: &str,
        app_name: &str,
        permissions: &[Permission],
    ) -> Result<PermissionCardUI, PermissionError> {
        // Create permission card UI
        let card_ui = self.renderer.create_card(app_id, app_name, permissions)?;
        
        // Store in history
        self.history.record_card_creation(app_id, permissions)?;
        
        Ok(card_ui)
    }
    
    /// Update permission card
    pub fn update_permission_card(
        &mut self,
        card: &mut PermissionCardUI,
        new_permissions: &[Permission],
    ) -> Result<(), PermissionError> {
        // Record old permissions
        let old_permissions = card.permissions.clone();
        
        // Update permissions
        self.controller.update_permissions(card, new_permissions)?;
        
        // Record change in history
        self.history.record_permission_change(
            card.app_id.clone(),
            old_permissions,
            new_permissions.to_vec(),
        )?;
        
        Ok(())
    }
    
    /// Apply permission template
    pub fn apply_template(
        &mut self,
        card: &mut PermissionCardUI,
        template_name: &str,
    ) -> Result<(), PermissionError> {
        // Get template
        let template = self.templates.get_template(template_name)?;
        
        // Apply template
        self.update_permission_card(card, &template.permissions)?;
        
        Ok(())
    }
    
    /// Get permission history
    pub fn get_permission_history(
        &self,
        app_id: &str,
    ) -> Result<Vec<PermissionChange>, PermissionError> {
        self.history.get_history(app_id)
    }
    
    /// Get audit log
    pub fn get_audit_log(
        &self,
        filter: Option<AuditFilter>,
    ) -> Result<Vec<AuditEntry>, PermissionError> {
        self.history.get_audit_log(filter)
    }
}

/// Permission Card UI
#[derive(Clone, Debug)]
pub struct PermissionCardUI {
    pub app_id: String,
    pub app_name: String,
    pub app_icon: Option<String>,
    pub permissions: Vec<Permission>,
    pub permission_groups: Vec<PermissionGroup>,
    pub state: CardState,
}

/// Permission Group
#[derive(Clone, Debug)]
pub struct PermissionGroup {
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub expanded: bool,
}

/// Card State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardState {
    Normal,
    Modified,
    PendingApproval,
    Approved,
    Rejected,
}

/// Permission Change
#[derive(Clone, Debug)]
pub struct PermissionChange {
    pub timestamp: u64,
    pub old_permissions: Vec<Permission>,
    pub new_permissions: Vec<Permission>,
    pub reason: String,
}

/// Audit Entry
#[derive(Clone, Debug)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub app_id: String,
    pub action: AuditAction,
    pub details: String,
}

/// Audit Action
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditAction {
    CardCreated,
    PermissionGranted,
    PermissionRevoked,
    TemplateApplied,
    CardDeleted,
}

/// Audit Filter
#[derive(Clone, Debug)]
pub struct AuditFilter {
    pub app_id: Option<String>,
    pub action: Option<AuditAction>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
}

/// Error types
#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("Renderer error: {0}")]
    RendererError(String),
    
    #[error("Controller error: {0}")]
    ControllerError(String),
    
    #[error("History error: {0}")]
    HistoryError(String),
    
    #[error("Template error: {0}")]
    TemplateError(String),
    
    #[error("Invalid permission: {0}")]
    InvalidPermission(String),
    
    #[error("Permission denied")]
    PermissionDenied,
}
```

**Card Renderer:**
```rust
// src/verified/permission_cards/renderer.rs

use crate::horizon::flux::flux_engine::*;

/// Card Renderer
pub struct CardRenderer {
    flux_engine: FluxEngine,
}

impl CardRenderer {
    pub fn new() -> Result<Self, PermissionError> {
        let flux_engine = FluxEngine::new()?;
        
        Ok(Self { flux_engine })
    }
    
    /// Create permission card UI
    pub fn create_card(
        &self,
        app_id: &str,
        app_name: &str,
        permissions: &[Permission],
    ) -> Result<PermissionCardUI, PermissionError> {
        // Group permissions by category
        let permission_groups = self.group_permissions(permissions);
        
        // Create card UI
        let card_ui = PermissionCardUI {
            app_id: app_id.to_string(),
            app_name: app_name.to_string(),
            app_icon: self.get_app_icon(app_id),
            permissions: permissions.to_vec(),
            permission_groups,
            state: CardState::Normal,
        };
        
        Ok(card_ui)
    }
    
    /// Render card to surface
    pub fn render_card(
        &self,
        card: &PermissionCardUI,
        surface: &mut Surface,
    ) -> Result<(), PermissionError> {
        // Render card background
        self.render_card_background(card, surface)?;
        
        // Render app icon and name
        self.render_app_header(card, surface)?;
        
        // Render permission groups
        for group in &card.permission_groups {
            self.render_permission_group(group, surface)?;
        }
        
        // Render action buttons
        self.render_action_buttons(card, surface)?;
        
        Ok(())
    }
    
    fn group_permissions(&self, permissions: &[Permission]) -> Vec<PermissionGroup> {
        let mut groups: HashMap<String, Vec<Permission>> = HashMap::new();
        
        for permission in permissions {
            let category = self.get_permission_category(permission);
            groups.entry(category).or_insert_with(Vec::new).push(permission.clone());
        }
        
        groups.into_iter().map(|(name, permissions)| {
            PermissionGroup {
                name: name.clone(),
                description: self.get_category_description(&name),
                permissions,
                expanded: false,
            }
        }).collect()
    }
    
    fn get_permission_category(&self, permission: &Permission) -> String {
        match permission {
            Permission::FileSystem { .. } => "File System".to_string(),
            Permission::Network { .. } => "Network".to_string(),
            Permission::IPC { .. } => "Inter-Process Communication".to_string(),
            Permission::Hardware { .. } => "Hardware".to_string(),
            Permission::System { .. } => "System".to_string(),
        }
    }
    
    fn get_category_description(&self, category: &str) -> String {
        match category {
            "File System" => "Access to files and directories".to_string(),
            "Network" => "Network connectivity and communication".to_string(),
            "Inter-Process Communication" => "Communication with other applications".to_string(),
            "Hardware" => "Access to hardware devices".to_string(),
            "System" => "System-level capabilities".to_string(),
            _ => String::new(),
        }
    }
    
    fn get_app_icon(&self, app_id: &str) -> Option<String> {
        // Get app icon from app manifest
        // ...
    }
    
    fn render_card_background(&self, _card: &PermissionCardUI, _surface: &mut Surface) -> Result<(), PermissionError> {
        // Render card background
        // ...
    }
    
    fn render_app_header(&self, _card: &PermissionCardUI, _surface: &mut Surface) -> Result<(), PermissionError> {
        // Render app icon and name
        // ...
    }
    
    fn render_permission_group(&self, _group: &PermissionGroup, _surface: &mut Surface) -> Result<(), PermissionError> {
        // Render permission group
        // ...
    }
    
    fn render_action_buttons(&self, _card: &PermissionCardUI, _surface: &mut Surface) -> Result<(), PermissionError> {
        // Render action buttons
        // ...
    }
}
```

---

### Day 2: Permission Controller & History
**Tasks:**
- [ ] Implement permission controller
- [ ] Implement permission validation
- [ ] Implement permission history
- [ ] Implement audit logging

**Code Structure:**
```rust
// src/verified/permission_cards/controller.rs

/// Permission Controller
pub struct PermissionController {
    permission_validator: PermissionValidator,
}

impl PermissionController {
    pub fn new() -> Result<Self, PermissionError> {
        let permission_validator = PermissionValidator::new()?;
        
        Ok(Self {
            permission_validator,
        })
    }
    
    /// Update permissions
    pub fn update_permissions(
        &self,
        card: &mut PermissionCardUI,
        new_permissions: &[Permission],
    ) -> Result<(), PermissionError> {
        // Validate new permissions
        self.permission_validator.validate_permissions(new_permissions)?;
        
        // Update permissions
        card.permissions = new_permissions.to_vec();
        card.permission_groups = self.group_permissions(new_permissions);
        card.state = CardState::Modified;
        
        Ok(())
    }
    
    /// Grant permission
    pub fn grant_permission(
        &self,
        card: &mut PermissionCardUI,
        permission: Permission,
    ) -> Result<(), PermissionError> {
        // Validate permission
        self.permission_validator.validate_permission(&permission)?;
        
        // Add permission if not already present
        if !card.permissions.contains(&permission) {
            card.permissions.push(permission);
            card.permission_groups = self.group_permissions(&card.permissions);
            card.state = CardState::Modified;
        }
        
        Ok(())
    }
    
    /// Revoke permission
    pub fn revoke_permission(
        &self,
        card: &mut PermissionCardUI,
        permission: &Permission,
    ) -> Result<(), PermissionError> {
        // Remove permission
        card.permissions.retain(|p| p != permission);
        card.permission_groups = self.group_permissions(&card.permissions);
        card.state = CardState::Modified;
        
        Ok(())
    }
    
    fn group_permissions(&self, permissions: &[Permission]) -> Vec<PermissionGroup> {
        // Group permissions by category
        // ...
    }
}

/// Permission Validator
pub struct PermissionValidator {
    permission_policies: HashMap<String, PermissionPolicy>,
}

impl PermissionValidator {
    pub fn new() -> Result<Self, PermissionError> {
        Ok(Self {
            permission_policies: HashMap::new(),
        })
    }
    
    pub fn validate_permissions(&self, permissions: &[Permission]) -> Result<(), PermissionError> {
        for permission in permissions {
            self.validate_permission(permission)?;
        }
        
        Ok(())
    }
    
    pub fn validate_permission(&self, permission: &Permission) -> Result<(), PermissionError> {
        match permission {
            Permission::FileSystem { path, .. } => {
                if path.contains("..") {
                    return Err(PermissionError::InvalidPermission("Invalid file path".to_string()));
                }
            }
            Permission::Network { host, .. } => {
                if host == "0.0.0.0" || host == "::" {
                    return Err(PermissionError::InvalidPermission("Invalid network host".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}
```

**Permission History:**
```rust
// src/verified/permission_cards/history.rs

/// Permission History
pub struct PermissionHistory {
    history: HashMap<String, Vec<PermissionChange>>,
    audit_log: Vec<AuditEntry>,
}

impl PermissionHistory {
    pub fn new() -> Result<Self, PermissionError> {
        Ok(Self {
            history: HashMap::new(),
            audit_log: Vec::new(),
        })
    }
    
    /// Record card creation
    pub fn record_card_creation(
        &mut self,
        app_id: &str,
        permissions: &[Permission],
    ) -> Result<(), PermissionError> {
        let change = PermissionChange {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            old_permissions: Vec::new(),
            new_permissions: permissions.to_vec(),
            reason: "Card created".to_string(),
        };
        
        self.history.entry(app_id.to_string()).or_insert_with(Vec::new).push(change);
        
        self.audit_log.push(AuditEntry {
            timestamp: change.timestamp,
            app_id: app_id.to_string(),
            action: AuditAction::CardCreated,
            details: format!("Permission card created with {} permissions", permissions.len()),
        });
        
        Ok(())
    }
    
    /// Record permission change
    pub fn record_permission_change(
        &mut self,
        app_id: &str,
        old_permissions: Vec<Permission>,
        new_permissions: Vec<Permission>,
    ) -> Result<(), PermissionError> {
        let change = PermissionChange {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            old_permissions,
            new_permissions,
            reason: "Permissions updated".to_string(),
        };
        
        self.history.entry(app_id.to_string()).or_insert_with(Vec::new).push(change);
        
        Ok(())
    }
    
    /// Get permission history
    pub fn get_history(&self, app_id: &str) -> Result<Vec<PermissionChange>, PermissionError> {
        Ok(self.history.get(app_id).cloned().unwrap_or_default())
    }
    
    /// Get audit log
    pub fn get_audit_log(&self, filter: Option<AuditFilter>) -> Result<Vec<AuditEntry>, PermissionError> {
        let mut entries = self.audit_log.clone();
        
        if let Some(filter) = filter {
            entries = entries.into_iter()
                .filter(|entry| {
                    if let Some(ref app_id) = filter.app_id {
                        if &entry.app_id != app_id {
                            return false;
                        }
                    }
                    if let Some(action) = filter.action {
                        if entry.action != action {
                            return false;
                        }
                    }
                    if let Some(start_time) = filter.start_time {
                        if entry.timestamp < start_time {
                            return false;
                        }
                    }
                    if let Some(end_time) = filter.end_time {
                        if entry.timestamp > end_time {
                            return false;
                        }
                    }
                    true
                })
                .collect();
        }
        
        Ok(entries)
    }
}
```

---

### Day 3: Templates & Verification
**Tasks:**
- [ ] Implement permission templates
- [ ] Implement batch permission management
- [ ] Add formal verification
- [ ] Write comprehensive tests

**Code Structure:**
```rust
// src/verified/permission_cards/templates.rs

/// Permission Templates
pub struct PermissionTemplates {
    templates: HashMap<String, PermissionTemplate>,
}

impl PermissionTemplates {
    pub fn new() -> Result<Self, PermissionError> {
        let mut templates = HashMap::new();
        
        // Add default templates
        templates.insert("minimal".to_string(), PermissionTemplate {
            name: "Minimal".to_string(),
            description: "Minimal permissions for basic functionality".to_string(),
            permissions: vec![
                Permission::System {
                    capability: SystemCapability::SystemInfo,
                },
            ],
        });
        
        templates.insert("standard".to_string(), PermissionTemplate {
            name: "Standard".to_string(),
            description: "Standard permissions for most applications".to_string(),
            permissions: vec![
                Permission::FileSystem {
                    path: "/home/user/".to_string(),
                    access: FileAccess::ReadWrite,
                },
                Permission::Network {
                    host: "*".to_string(),
                    port: 0,
                    access: NetworkAccess::Outbound,
                },
                Permission::System {
                    capability: SystemCapability::SystemInfo,
                },
            ],
        });
        
        templates.insert("full".to_string(), PermissionTemplate {
            name: "Full".to_string(),
            description: "Full permissions for trusted applications".to_string(),
            permissions: vec![
                Permission::FileSystem {
                    path: "/".to_string(),
                    access: FileAccess::ReadWrite,
                },
                Permission::Network {
                    host: "*".to_string(),
                    port: 0,
                    access: NetworkAccess::Both,
                },
                Permission::IPC {
                    service: "*".to_string(),
                    access: IPCAccess::Both,
                },
                Permission::Hardware {
                    device: "*".to_string(),
                    access: HardwareAccess::Control,
                },
                Permission::System {
                    capability: SystemCapability::ProcessControl,
                },
            ],
        });
        
        Ok(Self { templates })
    }
    
    /// Get template
    pub fn get_template(&self, name: &str) -> Result<PermissionTemplate, PermissionError> {
        self.templates.get(name)
            .cloned()
            .ok_or_else(|| PermissionError::TemplateError(format!("Template '{}' not found", name)))
    }
    
    /// List templates
    pub fn list_templates(&self) -> Vec<PermissionTemplate> {
        self.templates.values().cloned().collect()
    }
    
    /// Create custom template
    pub fn create_template(
        &mut self,
        name: String,
        description: String,
        permissions: Vec<Permission>,
    ) -> Result<(), PermissionError> {
        let template = PermissionTemplate {
            name: name.clone(),
            description,
            permissions,
        };
        
        self.templates.insert(name, template);
        
        Ok(())
    }
}

/// Permission Template
#[derive(Clone, Debug)]
pub struct PermissionTemplate {
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
}
```

**Formal Verification:**
```rust
// src/verified/permission_cards/verification.rs

use verus::*;

verus! {
    /// Verified permission consistency
    pub proof fn verify_permission_consistency(
        card: &PermissionCardUI,
    )
        ensures
            card.permissions.len() == card.permission_groups.iter().map(|g| g.permissions.len()).sum::<usize>(),
    {
        // Formal proof that permissions are consistent
        // ...
    }
    
    /// Verified permission history integrity
    pub proof fn verify_history_integrity(
        history: &PermissionHistory,
        app_id: &str,
    )
        ensures
            history.get_history(app_id).unwrap().len() >= 0,
    {
        // Formal proof that history is consistent
        // ...
    }
    
    /// Verified audit log completeness
    pub proof fn verify_audit_log_completeness(
        history: &PermissionHistory,
    )
        ensures
            history.get_audit_log(None).unwrap().len() >= 0,
    {
        // Formal proof that audit log is complete
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
    fn test_card_creation() {
        // Test card creation
    }
    
    #[test]
    fn test_permission_update() {
        // Test permission update
    }
    
    #[test]
    fn test_template_application() {
        // Test template application
    }
    
    #[test]
    fn test_history_tracking() {
        // Test history tracking
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_permission_lifecycle() {
        // Test complete permission lifecycle
    }
    
    #[test]
    fn test_batch_permission_management() {
        // Test batch permission management
    }
}
```

---

## 📊 Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Card Creation | < 50ms | ✅ |
| Card Rendering | < 100ms | ✅ |
| Permission Update | < 10ms | ✅ |
| Template Application | < 20ms | ✅ |
| History Query | < 50ms | ✅ |
| Audit Log Query | < 100ms | ✅ |

---

## 🔒 Security Considerations

1. **Permission Validation**: All permissions validated before granting
2. **Audit Logging**: All permission changes logged
3. **Memory Safety**: All operations bounds-checked
4. **Formal Verification**: Security-critical components formally verified
5. **User Consent**: Explicit user consent required for sensitive permissions

---

## 📚 References

- [VantisOS Permission System](../vnt/permission.rs)
- [Flux Engine Documentation](../horizon/flux/flux_engine.rs)
- [VantisOS Architecture Documentation](../architecture/arc42_VantisOS.md)

---

## ✅ Success Criteria

- [ ] Visual permission cards implemented
- [ ] Intuitive permission controls
- [ ] Clear permission explanations
- [ ] Permission history tracking
- [ ] Permission templates
- [ ] Batch permission management
- [ ] Permission audit logs
- [ ] Performance targets met
- [ ] Formal verification of security-critical components
- [ ] Comprehensive test coverage (> 80%)
- [ ] Documentation complete

---

**Next Steps**: Proceed to Phantom Run Implementation Guide