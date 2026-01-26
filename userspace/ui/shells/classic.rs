use super::Shell;

pub struct ClassicShell;

impl ClassicShell {
    pub fn new() -> Self {
        Self
    }
}

impl Shell for ClassicShell {
    fn run(&self) {
        // taskbar, start menu, windows
    }
}
