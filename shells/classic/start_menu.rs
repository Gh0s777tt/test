pub struct StartMenu {
    pub open: bool,
}

impl StartMenu {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}
