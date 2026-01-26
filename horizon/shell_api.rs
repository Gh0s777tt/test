pub enum ShellType {
    Classic,
    Radial,
    Spatial,
}

pub trait Shell {
    fn render(&self);
    fn on_input(&self, input: &str);
}

pub struct ClassicShell;

impl Shell for ClassicShell {
    fn render(&self) {
        println!("Classic Shell render");
    }
    fn on_input(&self, input: &str) {
        println!("Classic input: {}", input);
    }
}
