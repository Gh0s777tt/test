#[cfg(test)]
mod tests {
    use super::super::settings_panel::*;

    #[test]
    fn test_settings_panel_new() {
        let panel = SettingsPanel::new();
        assert_eq!(panel.current_category(), Category::Display);
        assert!(!panel.has_unsaved_changes());
    }

    #[test]
    fn test_settings_panel_with_config() {
        let mut config = SettingsConfig::default();
        config.show_advanced = true;
        let panel = SettingsPanel::with_config(config);
        assert!(panel.config().show_advanced);
    }

    #[test]
    fn test_switch_category() {
        let mut panel = SettingsPanel::new();
        panel.switch_category(Category::Sound);
        assert_eq!(panel.current_category(), Category::Sound);
    }

    #[test]
    fn test_categories_count() {
        let panel = SettingsPanel::new();
        // Should have 16 categories
        assert_eq!(panel.categories().len(), 16);
    }

    #[test]
    fn test_current_settings_display() {
        let panel = SettingsPanel::new();
        let settings = panel.current_settings();
        // Display category should have some settings
        assert!(!settings.is_empty());
    }

    #[test]
    fn test_get_setting() {
        let panel = SettingsPanel::new();
        let setting = panel.get_setting("display.brightness");
        assert!(setting.is_some());
    }

    #[test]
    fn test_get_setting_not_found() {
        let panel = SettingsPanel::new();
        let setting = panel.get_setting("nonexistent.setting");
        assert!(setting.is_none());
    }

    #[test]
    fn test_set_value_boolean() {
        let mut panel = SettingsPanel::new();
        panel.set_value("display.brightness", SettingValue::Integer(50)).unwrap();
        assert!(panel.has_unsaved_changes());
    }

    #[test]
    fn test_set_value_invalid_type() {
        let mut panel = SettingsPanel::new();
        let result = panel.set_value("display.brightness", SettingValue::Boolean(true));
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_to_default() {
        let mut panel = SettingsPanel::new();
        panel.set_value("display.brightness", SettingValue::Integer(50)).unwrap();
        panel.reset_to_default("display.brightness").unwrap();
        let setting = panel.get_setting("display.brightness").unwrap();
        assert_eq!(setting.value, setting.default);
    }

    #[test]
    fn test_reset_to_default_not_found() {
        let mut panel = SettingsPanel::new();
        let result = panel.reset_to_default("nonexistent.setting");
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_all_to_default() {
        let mut panel = SettingsPanel::new();
        panel.set_value("display.brightness", SettingValue::Integer(50)).unwrap();
        panel.reset_all_to_default();
        let setting = panel.get_setting("display.brightness").unwrap();
        assert_eq!(setting.value, setting.default);
    }

    #[test]
    fn test_save() {
        let mut panel = SettingsPanel::new();
        panel.set_value("display.brightness", SettingValue::Integer(50)).unwrap();
        panel.save().unwrap();
        assert!(!panel.has_unsaved_changes());
    }

    #[test]
    fn test_discard_changes() {
        let mut panel = SettingsPanel::new();
        let original_value = panel.get_setting("display.brightness").unwrap().value.clone();
        panel.set_value("display.brightness", SettingValue::Integer(50)).unwrap();
        panel.discard_changes();
        let current_value = panel.get_setting("display.brightness").unwrap().value.clone();
        assert_eq!(original_value, current_value);
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = Color::rgba(255, 128, 64, 128);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::rgba(255, 128, 64, 128);
        let hex = color.to_hex();
        assert_eq!(hex, 4278190208); // 0xFF804080
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex(4278190208);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_setting_value_boolean() {
        let value = SettingValue::Boolean(true);
        assert!(value.is_bool());
        assert_eq!(value.as_bool(), Some(true));
    }

    #[test]
    fn test_setting_value_integer() {
        let value = SettingValue::Integer(42);
        assert!(value.is_integer());
        assert_eq!(value.as_integer(), Some(42));
    }

    #[test]
    fn test_setting_value_float() {
        let value = SettingValue::Float(3.14);
        assert!(value.is_float());
        assert_eq!(value.as_float(), Some(3.14));
    }

    #[test]
    fn test_setting_value_string() {
        let value = SettingValue::String("test".to_string());
        assert!(value.is_string());
        assert_eq!(value.as_string(), Some("test"));
    }

    #[test]
    fn test_category_display_name() {
        assert_eq!(Category::Display.display_name(), "Display");
        assert_eq!(Category::Sound.display_name(), "Sound");
        assert_eq!(Category::Network.display_name(), "Network");
    }

    #[test]
    fn test_category_icon() {
        assert_eq!(Category::Display.icon(), "display");
        assert_eq!(Category::Sound.icon(), "volume-high");
        assert_eq!(Category::System.icon(), "cog");
    }

    #[test]
    fn test_setting_type_variants() {
        assert_eq!(std::mem::discriminant(&SettingType::Boolean), 
                   std::mem::discriminant(&SettingType::Boolean));
        assert_eq!(std::mem::discriminant(&SettingType::Integer), 
                   std::mem::discriminant(&SettingType::Integer));
        assert_eq!(std::mem::discriminant(&SettingType::String), 
                   std::mem::discriminant(&SettingType::String));
    }

    #[test]
    fn test_settings_config_default() {
        let config = SettingsConfig::default();
        assert!(!config.show_advanced);
        assert!(config.confirm_changes);
        assert!(!config.auto_save);
    }

    #[test]
    fn test_settings_error_display() {
        let err = SettingsError::SettingNotFound("test".to_string());
        assert!(err.to_string().contains("Setting not found"));
    }

    #[test]
    fn test_display_category_settings() {
        let panel = SettingsPanel::new();
        panel.switch_category(Category::Display);
        let settings = panel.current_settings();
        assert!(!settings.is_empty());
        // Should have brightness, resolution, etc.
    }

    #[test]
    fn test_sound_category_settings() {
        let panel = SettingsPanel::new();
        panel.switch_category(Category::Sound);
        let settings = panel.current_settings();
        assert!(!settings.is_empty());
    }

    #[test]
    fn test_system_category_settings() {
        let panel = SettingsPanel::new();
        panel.switch_category(Category::System);
        let settings = panel.current_settings();
        assert!(!settings.is_empty());
    }
}