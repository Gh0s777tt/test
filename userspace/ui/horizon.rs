use crate::ui::shells::{Shell, classic::ClassicShell};

pub fn start() {
    let shell = ClassicShell::new();
    shell.run();
}
