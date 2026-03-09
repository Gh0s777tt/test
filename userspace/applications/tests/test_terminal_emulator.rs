#[cfg(test)]
mod tests {
    use super::super::terminal_emulator::*;
    use std::path::PathBuf;

    #[test]
    fn test_terminal_emulator_new() {
        let emulator = TerminalEmulator::new();
        assert_eq!(emulator.tabs().len(), 0);
        assert_eq!(emulator.active_profile(), "Default");
    }

    #[test]
    fn test_new_tab() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        assert_eq!(emulator.tabs().len(), 1);
        assert_eq!(emulator.active_tab().unwrap().title, "Tab 1");
    }

    #[test]
    fn test_multiple_tabs() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.new_tab().unwrap();
        emulator.new_tab().unwrap();
        assert_eq!(emulator.tabs().len(), 3);
    }

    #[test]
    fn test_switch_tab() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.new_tab().unwrap();
        emulator.switch_tab(0).unwrap();
        assert_eq!(emulator.active_tab().unwrap().title, "Tab 1");
    }

    #[test]
    fn test_switch_tab_invalid() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        let result = emulator.switch_tab(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_close_tab() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.new_tab().unwrap();
        emulator.close_tab(0).unwrap();
        assert_eq!(emulator.tabs().len(), 1);
    }

    #[test]
    fn test_write_text() {
        let emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.write("Hello, World!").unwrap();
        let tab = emulator.active_tab().unwrap();
        let session = tab.session.lock().unwrap();
        assert_eq!(session.buffer.current_line.segments.len(), 1);
    }

    #[test]
    fn test_newline() {
        let emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.write("First line").unwrap();
        emulator.newline().unwrap();
        emulator.write("Second line").unwrap();
        let tab = emulator.active_tab().unwrap();
        let session = tab.session.lock().unwrap();
        assert_eq!(session.buffer.lines.len(), 1);
    }

    #[test]
    fn test_clear_buffer() {
        let emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.write("Some text").unwrap();
        emulator.newline().unwrap();
        emulator.write("More text").unwrap();
        emulator.clear().unwrap();
        let tab = emulator.active_tab().unwrap();
        let session = tab.session.lock().unwrap();
        assert_eq!(session.buffer.lines.len(), 0);
    }

    #[test]
    fn test_search() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.write("Hello World").unwrap();
        emulator.newline().unwrap();
        emulator.write("Hello Again").unwrap();
        emulator.search("Hello");
        assert_eq!(emulator.search_results().len(), 2);
    }

    #[test]
    fn test_clear_search() {
        let mut emulator = TerminalEmulator::new();
        emulator.new_tab().unwrap();
        emulator.write("Hello").unwrap();
        emulator.search("Hello");
        assert!(!emulator.search_results().is_empty());
        emulator.clear_search();
        assert!(emulator.search_results().is_empty());
    }

    #[test]
    fn test_add_profile() {
        let mut emulator = TerminalEmulator::new();
        let config = TerminalConfig::default();
        emulator.add_profile("Custom".to_string(), config);
        assert!(emulator.profiles().contains_key("Custom"));
    }

    #[test]
    fn test_switch_profile() {
        let mut emulator = TerminalEmulator::new();
        let mut config = TerminalConfig::default();
        config.behavior.scrollback_size = 5000;
        emulator.add_profile("Custom".to_string(), config);
        emulator.switch_profile("Custom").unwrap();
        assert_eq!(emulator.config().behavior.scrollback_size, 5000);
    }

    #[test]
    fn test_switch_profile_not_found() {
        let mut emulator = TerminalEmulator::new();
        let result = emulator.switch_profile("NonExistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_cursor_style() {
        let mut config = TerminalConfig::default();
        config.behavior.cursor_style = CursorStyle::Bar;
        let emulator = TerminalEmulator::with_config(config);
        assert_eq!(emulator.config().behavior.cursor_style, CursorStyle::Bar);
    }

    #[test]
    fn test_scrollback_size() {
        let mut config = TerminalConfig::default();
        config.behavior.scrollback_size = 20000;
        let mut emulator = TerminalEmulator::with_config(config.clone());
        emulator.new_tab().unwrap();
        let tab = emulator.active_tab().unwrap();
        let session = tab.session.lock().unwrap();
        assert_eq!(session.buffer.scrollback_size, 20000);
    }

    #[test]
    fn test_shell_config() {
        let mut config = TerminalConfig::default();
        config.shell.executable = "/bin/zsh".to_string();
        config.shell.args = vec!["--interactive".to_string()];
        let emulator = TerminalEmulator::with_config(config);
        assert_eq!(emulator.config().shell.executable, "/bin/zsh");
        assert_eq!(emulator.config().shell.args[0], "--interactive");
    }

    #[test]
    fn test_color_scheme() {
        let config = TerminalConfig::default();
        assert_eq!(config.colors.background.r, 30);
        assert_eq!(config.colors.foreground.r, 212);
    }

    #[test]
    fn test_keybinding_defaults() {
        let config = TerminalConfig::default();
        assert_eq!(config.shortcuts.copy, "Ctrl+Shift+C");
        assert_eq!(config.shortcuts.paste, "Ctrl+Shift+V");
        assert_eq!(config.shortcuts.new_tab, "Ctrl+Shift+T");
    }

    #[test]
    fn test_bell_behavior() {
        let config = TerminalConfig::default();
        assert_eq!(config.behavior.bell, BellBehavior::Both);
    }

    #[test]
    fn test_confirm_close_enabled() {
        let config = TerminalConfig::default();
        assert!(config.behavior.confirm_close);
    }
}