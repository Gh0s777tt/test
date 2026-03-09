use crate::horizon::window::Window;

pub struct WindowManager {
    pub windows: Vec<Window>,
    pub focused: Option<usize>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { windows: vec![], focused: None }
    }

    pub fn focus(&mut self, idx: usize) {
        self.focused = Some(idx);
    }

    pub fn close(&mut self, idx: usize) {
        self.windows.remove(idx);
        self.focused = None;
    }
}
