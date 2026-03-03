#[cfg(test)]
mod tests {
    use super::super::text_editor::*;
    use std::path::PathBuf;

    #[test]
    fn test_text_editor_new() {
        let editor = TextEditor::new();
        assert_eq!(editor.documents().len(), 1);
        assert_eq!(editor.active_document().unwrap().name, "Untitled");
    }

    #[test]
    fn test_new_document() {
        let mut editor = TextEditor::new();
        editor.new_document();
        assert_eq!(editor.documents().len(), 2);
    }

    #[test]
    fn test_insert_char() {
        let mut editor = TextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines[0], "Hi");
    }

    #[test]
    fn test_insert_newline() {
        let mut editor = TextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        editor.insert_newline();
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines.len(), 2);
    }

    #[test]
    fn test_backspace() {
        let mut editor = TextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        editor.backspace();
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines[0], "H");
    }

    #[test]
    fn test_delete() {
        let mut editor = TextEditor::new();
        editor.insert_char('H');
        editor.insert_char('i');
        editor.move_cursor(CursorDirection::Left, false);
        editor.delete();
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines[0], "H");
    }

    #[test]
    fn test_move_cursor_left() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.insert_char('C');
        editor.move_cursor(CursorDirection::Left, false);
        assert_eq!(editor.cursor().column, 1);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.move_cursor(CursorDirection::Left, false);
        editor.move_cursor(CursorDirection::Right, false);
        assert_eq!(editor.cursor().column, 1);
    }

    #[test]
    fn test_move_cursor_up_down() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_newline();
        editor.insert_char('B');
        editor.move_cursor(CursorDirection::Up, false);
        assert_eq!(editor.cursor().line, 0);
        editor.move_cursor(CursorDirection::Down, false);
        assert_eq!(editor.cursor().line, 1);
    }

    #[test]
    fn test_move_cursor_home_end() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.insert_char('C');
        editor.move_cursor(CursorDirection::Home, false);
        assert_eq!(editor.cursor().column, 0);
        editor.move_cursor(CursorDirection::End, false);
        assert_eq!(editor.cursor().column, 3);
    }

    #[test]
    fn test_select_all() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.select_all();
        let sel = editor.selection();
        assert_eq!(sel.start.line, 0);
        assert_eq!(sel.start.column, 0);
        assert_eq!(sel.end.column, 2);
    }

    #[test]
    fn test_selection_extend() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.insert_char('C');
        editor.move_cursor(CursorDirection::Left, true);
        editor.move_cursor(CursorDirection::Left, true);
        let sel = editor.selection();
        assert!(!sel.is_empty());
    }

    #[test]
    fn test_copy_selection() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.insert_char('C');
        editor.select_all();
        let text = editor.copy();
        assert_eq!(text, Some("ABC".to_string()));
    }

    #[test]
    fn test_cut_selection() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_char('B');
        editor.insert_char('C');
        editor.select_all();
        let text = editor.cut();
        assert_eq!(text, Some("ABC".to_string()));
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines[0], "");
    }

    #[test]
    fn test_paste() {
        let mut editor = TextEditor::new();
        editor.paste("Hello");
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines[0], "Hello");
    }

    #[test]
    fn test_paste_multiline() {
        let mut editor = TextEditor::new();
        editor.paste("Line1\nLine2");
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.lines.len(), 2);
        assert_eq!(doc.lines[0], "Line1");
        assert_eq!(doc.lines[1], "Line2");
    }

    #[test]
    fn test_document_modified() {
        let mut editor = TextEditor::new();
        editor.insert_char('X');
        let doc = editor.active_document().unwrap();
        assert!(doc.modified);
    }

    #[test]
    fn test_document_line_count() {
        let mut editor = TextEditor::new();
        editor.insert_char('A');
        editor.insert_newline();
        editor.insert_char('B');
        editor.insert_newline();
        editor.insert_char('C');
        let doc = editor.active_document().unwrap();
        assert_eq!(doc.line_count(), 3);
    }

    #[test]
    fn test_close_document() {
        let mut editor = TextEditor::new();
        editor.new_document();
        assert_eq!(editor.documents().len(), 2);
        editor.close_document(1).unwrap();
        assert_eq!(editor.documents().len(), 1);
    }

    #[test]
    fn test_switch_document() {
        let mut editor = TextEditor::new();
        editor.new_document();
        editor.switch_document(0).unwrap();
        assert_eq!(editor.active_document().unwrap().name, "Untitled");
    }

    #[test]
    fn test_find_text() {
        let mut editor = TextEditor::new();
        editor.paste("Hello World\nHello Again");
        let results = editor.find("Hello", false, false);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_case_sensitive() {
        let mut editor = TextEditor::new();
        editor.paste("hello Hello HELLO");
        let results = editor.find("Hello", true, false);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_find_whole_word() {
        let mut editor = TextEditor::new();
        editor.paste("Hello HelloWorld Hello");
        let results = editor.find("Hello", false, true);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_editor_config_default() {
        let config = TextEditorConfig::default();
        assert_eq!(config.font.size, 14);
        assert!(config.behavior.line_numbers);
        assert!(config.behavior.minimap);
        assert!(config.behavior.auto_indent);
    }

    #[test]
    fn test_editor_config_custom() {
        let mut config = TextEditorConfig::default();
        config.font.size = 18;
        config.behavior.word_wrap = true;
        let editor = TextEditor::with_config(config);
        assert_eq!(editor.config().font.size, 18);
        assert!(editor.config().behavior.word_wrap);
    }

    #[test]
    fn test_theme_default() {
        let theme = EditorTheme::default();
        assert_eq!(theme.name, "Dark+");
        assert_eq!(theme.background.r, 30);
    }

    #[test]
    fn test_keybindings_default() {
        let keybindings = Keybindings::default();
        assert_eq!(keybindings.save, "Ctrl+S");
        assert_eq!(keybindings.open, "Ctrl+O");
        assert_eq!(keybindings.find, "Ctrl+F");
    }

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("ts"), Language::TypeScript);
        assert_eq!(Language::from_extension("unknown"), Language::PlainText);
    }

    #[test]
    fn test_cursor_style() {
        let config = TextEditorConfig::default();
        assert_eq!(config.behavior.cursor_style, CursorStyle::Bar);
    }

    #[test]
    fn test_autosave_config() {
        let config = TextEditorConfig::default();
        assert!(config.autosave.enabled);
        assert_eq!(config.autosave.delay, 1000);
    }

    #[test]
    fn test_position_default() {
        let pos = Position::default();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_selection_default() {
        let sel = Selection::default();
        assert!(sel.is_empty());
    }

    #[test]
    fn test_selection_multiline() {
        let mut sel = Selection::default();
        sel.start = Position { line: 0, column: 0 };
        sel.end = Position { line: 2, column: 5 };
        assert!(sel.is_multiline());
    }
}