pub enum Shortcut {
    OpenMenu,
    CloseWindow,
}

pub fn handle(key: char) -> Option<Shortcut> {
    match key {
        'm' => Some(Shortcut::OpenMenu),
        'q' => Some(Shortcut::CloseWindow),
        _ => None,
    }
}
