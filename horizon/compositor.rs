use crate::horizon::{framebuffer::Framebuffer, window::Window};

pub struct Compositor {
    pub windows: Vec<Window>,
}

impl Compositor {
    pub fn draw(&self, fb: &mut Framebuffer) {
        fb.clear(0xFF1E222A);
        for w in &self.windows {
            w.draw(fb);
        }
    }
}
