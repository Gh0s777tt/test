pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: &'static mut [u32],
}

impl Framebuffer {
    pub fn clear(&mut self, color: u32) {
        for px in self.buffer.iter_mut() {
            *px = color;
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        let idx = y * self.width + x;
        if idx < self.buffer.len() {
            self.buffer[idx] = color;
        }
    }
}
