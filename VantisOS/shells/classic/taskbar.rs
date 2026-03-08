use crate::horizon::framebuffer::Framebuffer;

pub fn draw(fb: &mut Framebuffer) {
    let h = fb.height;
    for y in h-40..h {
        for x in 0..fb.width {
            fb.put_pixel(x, y, 0xFF2B2F36);
        }
    }
}
