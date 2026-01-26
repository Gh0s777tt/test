#[derive(Clone, Copy)]
pub enum InputEvent {
    Key(char),
    Mouse { x: i32, y: i32, pressed: bool },
}

pub fn poll() -> Option<InputEvent> {
    None // stub MVP
}
